use axum::extract::connect_info::Connected;
use hyper::server::conn::AddrStream;

use crate::ip::Ip;

#[derive(Debug, Clone)]
pub struct AddrConnectInfo {
    ip: Ip,
    port: u16,
}

impl AddrConnectInfo {
    pub fn ip(&self) -> &Ip {
        &self.ip
    }

    pub fn port(&self) -> u16 {
        self.port
    }
}

impl Connected<&AddrStream> for AddrConnectInfo {
    fn connect_info(target: &AddrStream) -> Self {
        let addr = target.remote_addr();

        AddrConnectInfo {
            ip: addr.ip().into(),
            port: addr.port(),
        }
    }
}
