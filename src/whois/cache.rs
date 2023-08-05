use std::{
    net::IpAddr,
    num::NonZeroUsize,
    sync::{Arc, Mutex},
};

use lru::LruCache;

use super::Whois;

#[derive(Debug, Clone)]
pub struct WhoisCache(Arc<Mutex<LruCache<IpAddr, Whois>>>);

impl WhoisCache {
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(LruCache::new(
            NonZeroUsize::new(10_000).unwrap(),
        ))))
    }

    /// A cache for whois lookups, since they are expensive and can take a long time.
    pub fn get(&self, addr: IpAddr) -> Whois {
        let mut lru = match self.0.lock() {
            Ok(lru) => lru,
            Err(err) => {
                tracing::error!("Failed to lock LRU cache: {}", err);

                return Whois::default();
            }
        };

        match lru.get(&addr) {
            Some(whois) => whois.clone(),
            None => {
                tracing::info!("No whois cache for IP {}, resolving...", addr);

                let whois = match Whois::new(addr) {
                    Ok(whois) => whois,
                    Err(err) => {
                        tracing::warn!("Failed to get whois for IP {}: {}", addr, err);

                        return Whois::default();
                    }
                };

                lru.put(addr, whois.clone());
                whois
            }
        }
    }
}
