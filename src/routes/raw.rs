use axum::{body::Body, extract::ConnectInfo, http::Request};

use super::full::X_REAL_IP;
use crate::connect::AddrConnectInfo;

pub async fn raw(
    ConnectInfo(addr): ConnectInfo<AddrConnectInfo>,
    request: Request<Body>,
) -> String {
    request.headers().get(X_REAL_IP).map_or_else(
        || addr.ip().to_string(),
        |x_real_ip| x_real_ip.to_str().unwrap_or_default().to_string(),
    )
}
