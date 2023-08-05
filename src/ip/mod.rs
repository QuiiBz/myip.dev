use std::net::IpAddr;

use dns_lookup::lookup_addr;

mod r#as;
mod geo;

pub use geo::Geo;
pub use r#as::AS;

pub const UNKNOWN: &str = "unknown";

#[derive(Debug, Clone)]
pub enum Ip {
    V4(String),
    V6(String),
}

impl From<IpAddr> for Ip {
    fn from(ip: IpAddr) -> Self {
        match ip {
            IpAddr::V4(ip) => Ip::V4(ip.to_string()),
            IpAddr::V6(ip) => Ip::V6(ip.to_string()),
        }
    }
}

impl ToString for Ip {
    fn to_string(&self) -> String {
        match self {
            Ip::V4(ip) => ip.to_string(),
            Ip::V6(ip) => ip.to_string(),
        }
    }
}

pub fn get_reverse(addr: &IpAddr) -> String {
    lookup_addr(&addr).unwrap_or_else(|err| {
        tracing::warn!("Failed to get reverse for {}: {}", addr, err);

        return UNKNOWN.into();
    })
}
