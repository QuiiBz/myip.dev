use axum::extract::connect_info::Connected;
use hyper::server::conn::AddrStream;

use crate::ip::Ip;

#[derive(Debug, Clone)]
pub struct AddrConnectInfo(Ip);

impl AddrConnectInfo {
    pub fn ip(&self) -> &Ip {
        &self.0
    }
}

impl Connected<&AddrStream> for AddrConnectInfo {
    fn connect_info(target: &AddrStream) -> Self {
        let addr = target.remote_addr();

        AddrConnectInfo(addr.ip().into())
    }
}
