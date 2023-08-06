use std::net::{IpAddr, SocketAddr};

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
    http::{extract_ip, is_user_agent_automated, Http, X_REAL_IP, X_REAL_PROTO, X_TLS_VERSION},
    ip::{get_reverse, Geo, AS, UNKNOWN},
    state::AppState,
    whois::Whois,
};

#[derive(Debug, Serialize)]
pub struct Full {
    ip: String,
    reverse: String,
    r#as: AS,
    whois: Whois,
    geo: Geo,
    http: Http,
}

pub async fn full(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    request: Request<Body>,
) -> Response {
    let user_agent = request
        .headers()
        .get(USER_AGENT)
        .map(|user_agent| user_agent.to_str().unwrap_or_default().to_string());

    let ip = extract_ip(request.headers().get(X_REAL_IP), addr.ip());

    let addr = match ip.parse::<IpAddr>() {
        Ok(addr) => addr,
        Err(err) => {
            tracing::warn!("Could not parse IP {}: {}", ip, err);

            return (StatusCode::BAD_REQUEST, "Invalid IP address format.").into_response();
        }
    };

    let is_automated = is_user_agent_automated(&user_agent);
    let reverse = get_reverse(&addr);
    let r#as = AS::new(&state.maxmind, addr.clone());
    let whois = state.whois_cache.get(addr.clone());
    let geo = Geo::new(&state.maxmind, addr);

    let http_version = request.headers().get(X_REAL_PROTO).map_or_else(
        || format!("{:?}", request.version()),
        |proto| proto.to_str().unwrap_or_default().to_string(),
    );

    let mut tls = request.headers().get(X_TLS_VERSION).map_or_else(
        || UNKNOWN.to_string(),
        |tls| tls.to_str().unwrap_or_default().to_string(),
    );

    if tls == "{http.request.tls.version}" {
        tls = UNKNOWN.to_string();
    }

    let http = Http::new(http_version, tls, user_agent);

    let full = Full {
        ip,
        reverse,
        r#as,
        whois,
        geo,
        http,
    };

    if is_automated {
        return Json(full).into_response();
    }

    match state.handlebars.render("full", &full) {
        Ok(html) => Html(html).into_response(),
        Err(err) => {
            tracing::error!("Failed to render full template: {}", err);

            (StatusCode::INTERNAL_SERVER_ERROR, "Please try again later.").into_response()
        }
    }
}
