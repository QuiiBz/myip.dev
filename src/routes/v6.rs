use axum::extract::ConnectInfo;

use crate::connect::AddrConnectInfo;

pub async fn v6(ConnectInfo(addr): ConnectInfo<AddrConnectInfo>) -> String {
    addr.ip().to_string()
}
