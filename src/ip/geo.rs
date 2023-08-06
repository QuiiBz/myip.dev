use std::net::IpAddr;

use maxminddb::geoip2::City;
use serde::Serialize;

use super::UNKNOWN;
use crate::state::Maxmind;

#[derive(Debug, Serialize)]
pub struct Geo {
    city: String,
    country: String,
    latitude: f64,
    longitude: f64,
}

impl Default for Geo {
    fn default() -> Self {
        Self {
            city: UNKNOWN.into(),
            country: UNKNOWN.into(),
            latitude: 0.0,
            longitude: 0.0,
        }
    }
}

impl Geo {
    pub fn new<'a>(maxmind: &'a Maxmind, addr: IpAddr) -> Self {
        match maxmind.city.lookup::<City<'a>>(addr) {
            Ok(city) => Self {
                // TODO: clean this shit
                city: city.city.as_ref().map_or_else(
                    || UNKNOWN.into(),
                    |city| {
                        city.names.as_ref().map_or_else(
                            || UNKNOWN.into(),
                            |names| {
                                names
                                    .get("en")
                                    .map_or_else(|| UNKNOWN.into(), |name| name.to_string())
                            },
                        )
                    },
                ),
                country: city.country.as_ref().map_or_else(
                    || UNKNOWN.into(),
                    |country| {
                        country
                            .iso_code
                            .map_or_else(|| UNKNOWN.into(), |code| code.to_string())
                    },
                ),
                latitude: city.location.as_ref().map_or(0.0, |location| {
                    location.latitude.map_or(0.0, |latitude| latitude)
                }),
                longitude: city.location.as_ref().map_or(0.0, |location| {
                    location.longitude.map_or(0.0, |longitude| longitude)
                }),
            },
            Err(err) => {
                tracing::warn!("Failed to lookup Geo: {}", err);

                Self::default()
            }
        }
    }
}
