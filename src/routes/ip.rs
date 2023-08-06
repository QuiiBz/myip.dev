use std::net::IpAddr;

use axum::{
    body::Body,
    extract::{Path, State},
    http::header::USER_AGENT,
    http::{Request, StatusCode},
    response::{Html, IntoResponse, Response},
    Json,
};
use serde::Serialize;

use crate::{
    http::is_user_agent_automated,
    ip::{get_reverse, Geo, AS},
    state::AppState,
    whois::Whois,
};

#[derive(Debug, Serialize)]
pub struct Ip {
    ip: String,
    reverse: String,
    r#as: AS,
    whois: Whois,
    geo: Geo,
}

pub async fn ip(
    State(state): State<AppState>,
    Path(ip): Path<String>,
    request: Request<Body>,
) -> Response {
    let user_agent = request
        .headers()
        .get(USER_AGENT)
        .map(|user_agent| user_agent.to_str().unwrap_or_default().to_string());

    let addr = match ip.parse::<IpAddr>() {
        Ok(addr) => addr,
        Err(err) => {
            tracing::error!("Invalid IP address format ({}): {}", ip, err);

            return (StatusCode::BAD_REQUEST, "Invalid IP address format.").into_response();
        }
    };

    let _guard = tracing::info_span!("resolve", ip = %addr).entered();

    let is_automated = is_user_agent_automated(&user_agent);
    let reverse = get_reverse(&addr);
    let r#as = AS::new(&state.maxmind, addr.clone());
    let whois = state.whois_cache.get(addr.clone());
    let geo = Geo::new(&state.maxmind, addr);

    let ip = Ip {
        ip,
        reverse,
        r#as,
        whois,
        geo,
    };

    if is_automated {
        return Json(ip).into_response();
    }

    match state.handlebars.render("ip", &ip) {
        Ok(html) => Html(html).into_response(),
        Err(err) => {
            tracing::error!("Failed to render ip template: {}", err);

            (StatusCode::INTERNAL_SERVER_ERROR, "Please try again later.").into_response()
        }
    }
}
