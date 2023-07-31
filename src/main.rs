use crate::{
    connect::AddrConnectInfo,
    routes::{full, ip, raw},
    state::AppState,
};

use anyhow::Result;
use axum::{error_handling::HandleErrorLayer, http::StatusCode, routing::get, Router};
use std::{net::SocketAddr, time::Duration};
use tower::{buffer::BufferLayer, limit::RateLimitLayer, ServiceBuilder};
use tower_http::services::ServeDir;

mod connect;
mod http;
mod ip;
mod routes;
mod state;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let state = AppState::new()?;

    let app = Router::new()
        .route("/", get(full))
        .route("/raw", get(raw))
        .route("/:ip", get(ip))
        .nest_service("/static", ServeDir::new("public"))
        .with_state(state)
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|err| async move {
                    tracing::error!("Unhandled error: {}", err);

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
    tracing::debug!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<AddrConnectInfo>())
        .await
        .unwrap();

    Ok(())
}
