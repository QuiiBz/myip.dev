use std::sync::Arc;

use anyhow::Result;
use handlebars::Handlebars;
use maxminddb::Reader;

use crate::ip::MaxmindDB;

#[derive(Clone)]
pub struct AppState {
    pub handlebars: Handlebars<'static>,
    // TODO: use a sub-struct
    pub maxmind_asn: Arc<MaxmindDB>,
    pub maxmind_city: Arc<MaxmindDB>,
}

impl AppState {
    pub fn new() -> Result<Self> {
        let handlebars = match Self::load_handlebars() {
            Ok(handlebars) => handlebars,
            Err(err) => {
                tracing::error!("Failed to load handlebars: {}", err);
                std::process::exit(1);
            }
        };

        let (maxmind_asn, maxmind_city) = match Self::load_maxmind() {
            Ok(maxmind) => maxmind,
            Err(err) => {
                tracing::error!("Failed to load maxmind: {}", err);
                std::process::exit(1);
            }
        };

        Ok(Self {
            handlebars,
            maxmind_asn,
            maxmind_city,
        })
    }

    fn load_handlebars<'a>() -> Result<Handlebars<'a>> {
        let mut handlebars = Handlebars::new();

        handlebars.register_template_file("full", "./src/templates/full.html")?;
        handlebars.register_template_file("ip", "./src/templates/ip.html")?;

        Ok(handlebars)
    }

    fn load_maxmind() -> Result<(Arc<MaxmindDB>, Arc<MaxmindDB>)> {
        let maxmind_asn = Arc::new(Reader::open_readfile("./GeoLite2-ASN.mmdb")?);
        let maxmind_city = Arc::new(Reader::open_readfile("./GeoLite2-City.mmdb")?);

        Ok((maxmind_asn, maxmind_city))
    }
}
