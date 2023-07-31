use axum::{body::Body, extract::ConnectInfo, http::Request};

use super::full::X_REAL_IP;
use crate::connect::AddrConnectInfo;

pub async fn raw(
    ConnectInfo(addr): ConnectInfo<AddrConnectInfo>,
    request: Request<Body>,
) -> String {
    // TODO: refactor this shit
    request
        .headers()
        .get(X_REAL_IP)
        .map_or(addr.ip().to_string(), |x_real_ip| {
            x_real_ip.to_str().unwrap().to_string()
        })
}
