use std::{
    net::IpAddr,
    num::NonZeroUsize,
    sync::{Arc, Mutex, MutexGuard},
};

use anyhow::Result;
use lru::LruCache;

use super::Whois;

#[derive(Debug, Clone)]
pub struct WhoisCache(Arc<Mutex<LruCache<IpAddr, Whois>>>);

/// A cache for whois lookups, since they are expensive and can take a long time.
impl WhoisCache {
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(LruCache::new(
            NonZeroUsize::new(10_000).unwrap(),
        ))))
    }

    fn lock(&self) -> Result<MutexGuard<LruCache<IpAddr, Whois>>> {
        self.0
            .lock()
            .map_err(|err| anyhow::anyhow!("Failed to lock LRU cache: {}", err))
    }

    fn inner_get(&self, addr: IpAddr) -> Result<Whois> {
        let mut lru = self.lock()?;

        match lru.get(&addr) {
            Some(whois) => {
                tracing::debug!("Found whois in cache: {}", whois);

                Ok(whois.clone())
            }
            None => {
                tracing::info!("No whois cache, resolving...");

                // We don't want to hold the lock for the entire duration of the function
                // because whois lookups can take a long time.
                drop(lru);

                let whois = Whois::new(addr)?;
                self.lock()?.put(addr, whois.clone());

                tracing::debug!("Resolved whois: {}", whois);

                Ok(whois)
            }
        }
    }

    pub fn get(&self, addr: IpAddr) -> Whois {
        match self.inner_get(addr) {
            Ok(whois) => whois,
            Err(err) => {
                tracing::warn!("Failed to get whois: {}", err);

                Whois::default()
            }
        }
    }
}
