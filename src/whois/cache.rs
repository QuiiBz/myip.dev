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

    pub fn get(&self, addr: IpAddr) -> Whois {
        let mut lru = self.0.lock().unwrap();

        match lru.get(&addr) {
            Some(whois) => whois.clone(),
            None => {
                let whois = Whois::new(addr).unwrap_or_default();
                lru.put(addr, whois.clone());

                whois
            }
        }
    }
}
