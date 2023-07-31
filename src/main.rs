use crate::{
    connect::AddrConnectInfo,
    ip::MaxmindDB,
    routes::{full, ip, raw},
};

use axum::{error_handling::HandleErrorLayer, http::StatusCode, routing::get, BoxError, Router};
use handlebars::Handlebars;
use maxminddb::Reader;
use std::{net::SocketAddr, sync::Arc, time::Duration};
use tower::{buffer::BufferLayer, limit::RateLimitLayer, ServiceBuilder};
use tower_http::services::ServeDir;

mod connect;
mod http;
mod ip;
mod routes;

#[derive(Clone)]
pub struct AppState {
    handlebars: Handlebars<'static>,
    maxmind_asn: Arc<MaxmindDB>,
    maxmind_city: Arc<MaxmindDB>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let mut handlebars = Handlebars::new();
    handlebars.register_template_file("full", "./src/templates/full.html")?;
    handlebars.register_template_file("ip", "./src/templates/ip.html")?;

    let maxmind_asn = Arc::new(Reader::open_readfile("./GeoLite2-ASN.mmdb")?);
    let maxmind_city = Arc::new(Reader::open_readfile("./GeoLite2-City.mmdb")?);

    let state = AppState {
        handlebars,
        maxmind_asn,
        maxmind_city,
    };

    let app = Router::new()
        .route("/", get(full))
        .route("/raw", get(raw))
        .route("/:ip", get(ip))
        .nest_service("/static", ServeDir::new("public"))
        .with_state(state)
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|err: BoxError| async move {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unhandled error: {}", err),
                    )
                }))
                .layer(BufferLayer::new(1024))
                // TODO: use per-ip rate limiter instead of global
                .layer(RateLimitLayer::new(10, Duration::from_secs(1))),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<AddrConnectInfo>())
        .await
        .unwrap();

    Ok(())
}
