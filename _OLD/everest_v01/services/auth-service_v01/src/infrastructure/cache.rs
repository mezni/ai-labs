use dashmap::DashMap;
use jsonwebtoken::DecodingKey;
use std::time::{Duration, Instant};

pub struct JwtKeyCache {
    keys: DashMap<String, CachedKey>,
}

struct CachedKey {
    key: DecodingKey,
    expires_at: Instant,
}

impl JwtKeyCache {
    pub fn new() -> Self {
        Self {
            keys: DashMap::new(),
        }
    }

    pub fn get(&self, kid: &str) -> Option<DecodingKey> {
        if let Some(entry) = self.keys.get(kid) {
            if entry.expires_at > Instant::now() {
                return Some(entry.key.clone());
            }
            drop(entry);
            self.keys.remove(kid);
        }
        None
    }

    pub fn set(&self, kid: String, key: DecodingKey, ttl: Duration) {
        self.keys.insert(
            kid,
            CachedKey {
                key,
                expires_at: Instant::now() + ttl,
            },
        );
    }

    pub fn clear(&self) {
        self.keys.clear();
    }
}

impl Default for JwtKeyCache {
    fn default() -> Self {
        Self::new()
    }
}