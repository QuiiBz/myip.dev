use std::sync::Arc;

use anyhow::Result;
use handlebars::Handlebars;
use maxminddb::Reader;

use crate::whois::WhoisCache;

type MaxmindDB = Reader<Vec<u8>>;

pub struct Maxmind {
    pub asn: MaxmindDB,
    pub city: MaxmindDB,
}

#[derive(Clone)]
pub struct AppState {
    pub handlebars: Handlebars<'static>,
    pub maxmind: Arc<Maxmind>,
    pub whois_cache: WhoisCache,
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

        let maxmind = match Self::load_maxmind() {
            Ok(maxmind) => maxmind,
            Err(err) => {
                tracing::error!("Failed to load maxmind: {}", err);
                std::process::exit(1);
            }
        };

        let whois_cache = WhoisCache::new();

        Ok(Self {
            handlebars,
            maxmind,
            whois_cache,
        })
    }

    fn load_handlebars<'a>() -> Result<Handlebars<'a>> {
        let mut handlebars = Handlebars::new();

        handlebars.register_template_file("layout", "./src/templates/layout.html")?;

        handlebars.register_template_file("full", "./src/templates/full.html")?;
        handlebars.register_template_file("ip", "./src/templates/ip.html")?;

        Ok(handlebars)
    }

    fn load_maxmind() -> Result<Arc<Maxmind>> {
        let asn = Reader::open_readfile("./GeoLite2-ASN.mmdb")?;
        let city = Reader::open_readfile("./GeoLite2-City.mmdb")?;

        Ok(Arc::new(Maxmind { asn, city }))
    }
}
