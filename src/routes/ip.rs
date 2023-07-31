use std::net::IpAddr;

use axum::{
    body::Body,
    extract::{Path, State},
    http::header::USER_AGENT,
    http::Request,
    response::{Html, IntoResponse, Response},
    Json,
};
use serde::Serialize;

use crate::{
    http::is_user_agent_automated,
    ip::{get_reverse, Geo, AS},
    state::AppState,
};

#[derive(Debug, Serialize)]
pub struct Ip {
    ip: String,
    reverse: String,
    r#as: AS,
    geo: Geo,
}

pub async fn ip(
    State(state): State<AppState>,
    Path(ip): Path<String>,
    request: Request<Body>,
) -> Response {
    let user_agent_header = request.headers().get(USER_AGENT);
    let user_agent = user_agent_header.map(|user_agent| user_agent.to_str().unwrap().to_string());

    // TODO: handle error
    let addr = ip.parse::<IpAddr>().unwrap();

    let is_automated = is_user_agent_automated(&user_agent);
    let reverse = get_reverse(&addr);
    let r#as = AS::from(&state.maxmind_asn, addr);
    let geo = Geo::from(&state.maxmind_city, addr);

    let ip = Ip {
        ip,
        reverse,
        r#as,
        geo,
    };

    if is_automated {
        return Json(ip).into_response();
    }

    // TODO: handle error
    let html = state.handlebars.render("ip", &ip).unwrap();
    return Html(html).into_response();
}
