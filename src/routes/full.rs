use std::net::IpAddr;

use axum::{
    body::Body,
    extract::{ConnectInfo, State},
    http::header::USER_AGENT,
    http::{Request, StatusCode},
    response::{Html, IntoResponse, Response},
    Json,
};
use serde::Serialize;

use crate::{
    connect::AddrConnectInfo,
    http::{is_user_agent_automated, Http},
    ip::{get_reverse, Geo, AS, UNKNOWN},
    state::AppState,
};

pub const X_REAL_IP: &str = "X-Real-Ip";
const X_REAL_PORT: &str = "X-Real-Port";
const X_REAL_PROTO: &str = "X-Real-Proto";
const X_TLS_VERSION: &str = "X-Tls-Version";

#[derive(Debug, Serialize)]
pub struct Full {
    ip: String,
    port: u16,
    reverse: String,
    r#as: AS,
    geo: Geo,
    http: Http,
}

pub async fn full(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<AddrConnectInfo>,
    request: Request<Body>,
) -> Response {
    let user_agent_header = request.headers().get(USER_AGENT);
    // TODO: handle error
    let user_agent = user_agent_header.map(|user_agent| user_agent.to_str().unwrap().to_string());

    // TODO: refactor this shit
    let ip = request
        .headers()
        .get(X_REAL_IP)
        .map_or(addr.ip().to_string(), |x_real_ip| {
            x_real_ip.to_str().unwrap().to_string()
        });

    let port = request
        .headers()
        .get(X_REAL_PORT)
        .map_or(addr.port(), |x_real_port| {
            x_real_port.to_str().unwrap().parse().unwrap()
        });

    let addr = match ip.parse::<IpAddr>() {
        Ok(addr) => addr,
        Err(err) => {
            tracing::error!("Invalid IP address format ({}): {}", ip, err);

            return (StatusCode::BAD_REQUEST, "Invalid IP address format.").into_response();
        }
    };

    let is_automated = is_user_agent_automated(&user_agent);
    let reverse = get_reverse(&addr);
    let r#as = AS::new(&state.maxmind, addr.clone());
    let geo = Geo::new(&state.maxmind, addr);

    let http_version = request.headers().get(X_REAL_PROTO).map_or_else(
        || format!("{:?}", request.version()),
        |proto| proto.to_str().unwrap().to_string(),
    );

    let mut tls = request.headers().get(X_TLS_VERSION).map_or_else(
        || UNKNOWN.to_string(),
        |tls| tls.to_str().unwrap().to_string(),
    );

    if tls == "{http.request.tls.version}" {
        tls = UNKNOWN.to_string();
    }

    let http = Http::new(http_version, tls, user_agent);

    let full = Full {
        ip,
        port,
        reverse,
        r#as,
        geo,
        http,
    };

    if is_automated {
        return Json(full).into_response();
    }

    // TODO: handle error
    let html = state.handlebars.render("full", &full).unwrap();
    return Html(html).into_response();
}
