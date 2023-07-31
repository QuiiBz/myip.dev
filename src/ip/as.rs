use std::{net::IpAddr, sync::Arc};

use maxminddb::geoip2::Asn;
use serde::Serialize;

use super::{MaxmindDB, UNKNOWN};

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
    pub fn from(maxmind: &Arc<MaxmindDB>, addr: IpAddr) -> Self {
        let autonomous_system = maxmind.lookup::<Asn>(addr);

        match autonomous_system {
            Ok(autonomous_system) => {
                let asn = autonomous_system
                    .autonomous_system_number
                    .map_or_else(|| UNKNOWN.into(), |asn| asn.to_string());

                let org = autonomous_system
                    .autonomous_system_organization
                    .map_or_else(|| UNKNOWN.into(), |org| org.to_string());

                Self { asn, org }
            }
            // TODO: log error
            Err(_) => Self::default(),
        }
    }
}
