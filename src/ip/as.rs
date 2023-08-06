use std::net::IpAddr;

use maxminddb::geoip2::Asn;
use serde::Serialize;

use super::UNKNOWN;
use crate::state::Maxmind;

#[derive(Debug, Serialize)]
pub struct AS {
    asn: String,
    org: String,
}

impl Default for AS {
    fn default() -> Self {
        Self {
            asn: UNKNOWN.into(),
            org: UNKNOWN.into(),
        }
    }
}

impl AS {
    pub fn new(maxmind: &Maxmind, addr: IpAddr) -> Self {
        match maxmind.asn.lookup::<Asn>(addr) {
            Ok(autonomous_system) => {
                let asn = autonomous_system
                    .autonomous_system_number
                    .map_or_else(|| UNKNOWN.into(), |asn| format!("AS{}", asn));

                let org = autonomous_system
                    .autonomous_system_organization
                    .map_or_else(|| UNKNOWN.into(), |org| org.to_string());

                Self { asn, org }
            }
            Err(err) => {
                tracing::warn!("Failed to lookup AS: {}", err);

                Self::default()
            }
        }
    }
}
