use std::net::SocketAddr;

use axum::{body::Body, extract::ConnectInfo, http::Request};

use crate::http::{extract_ip, X_REAL_IP};

pub async fn raw(ConnectInfo(addr): ConnectInfo<SocketAddr>, request: Request<Body>) -> String {
    extract_ip(request.headers().get(X_REAL_IP), addr.ip())
}
