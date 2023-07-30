use std::{net::IpAddr, sync::Arc};

use dns_lookup::lookup_addr;
use maxminddb::{geoip2::Asn, Reader};
use serde::Serialize;

pub type MaxmindDB = Reader<Vec<u8>>;

const UNKNOWN: &str = "unknown";

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

impl From<Ip> for IpAddr {
    fn from(ip: Ip) -> Self {
        match ip {
            Ip::V4(ip) => ip.parse().unwrap(),
            Ip::V6(ip) => ip.parse().unwrap(),
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

#[derive(Debug, Serialize)]
pub struct AS {
    pub asn: String,
    pub org: String,
}

impl AS {
    pub fn from(maxmind: &Arc<MaxmindDB>, addr: IpAddr) -> Self {
        let asn = maxmind.lookup::<Asn>(addr);

        let asn = asn.as_ref().map(|asn| {
            (
                asn.autonomous_system_number
                    .map_or_else(|| UNKNOWN.into(), |asn| format!("AS{}", asn)),
                asn.autonomous_system_organization
                    .map_or_else(|| UNKNOWN.into(), |org| org.to_string()),
            )
        });

        asn.map(|(asn, org)| AS { asn, org })
            .unwrap_or_else(|_| AS {
                asn: UNKNOWN.into(),
                org: UNKNOWN.into(),
            })
    }
}

pub fn get_reverse(addr: &IpAddr) -> Option<String> {
    lookup_addr(&addr).ok()
}
