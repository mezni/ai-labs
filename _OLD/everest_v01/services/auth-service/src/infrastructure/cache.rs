// src/infrastructure/cache.rs
use dashmap::DashMap;
use std::hash::Hash;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Generic in-memory cache with TTL support
#[derive(Clone)]
pub struct Cache<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    data: Arc<DashMap<K, CachedItem<V>>>,
    default_ttl: Duration,
}

struct CachedItem<V> {
    value: V,
    expires_at: u64,
}

impl<K, V> Cache<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    pub fn new(default_ttl: Duration) -> Self {
        Self {
            data: Arc::new(DashMap::new()),
            default_ttl,
        }
    }

    pub fn with_capacity(capacity: usize, default_ttl: Duration) -> Self {
        Self {
            data: Arc::new(DashMap::with_capacity(capacity)),
            default_ttl,
        }
    }

    /// Get value from cache if not expired
    pub fn get(&self, key: &K) -> Option<V> {
        if let Some(entry) = self.data.get(key) {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();

            if entry.expires_at > now {
                return Some(entry.value.clone());
            } else {
                // Remove expired entry
                drop(entry);
                self.data.remove(key);
            }
        }
        None
    }

    /// Insert value with default TTL
    pub fn insert(&self, key: K, value: V) {
        self.insert_with_ttl(key, value, self.default_ttl);
    }

    /// Insert value with custom TTL
    pub fn insert_with_ttl(&self, key: K, value: V, ttl: Duration) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let expires_at = now + ttl.as_secs();

        self.data.insert(key, CachedItem { value, expires_at });
    }

    /// Remove value from cache
    pub fn remove(&self, key: &K) -> Option<V> {
        self.data.remove(key).map(|(_, item)| item.value)
    }

    /// Clear all entries
    pub fn clear(&self) {
        self.data.clear();
    }

    /// Get number of cached items (including expired)
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check if cache is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Remove all expired entries
    pub fn cleanup_expired(&self) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        self.data.retain(|_, item| item.expires_at > now);
    }

    /// Get or insert value using a factory function
    pub fn get_or_insert_with<F>(&self, key: K, factory: F) -> V
    where
        F: FnOnce() -> V,
    {
        if let Some(value) = self.get(&key) {
            return value;
        }

        let value = factory();
        self.insert(key, value.clone());
        value
    }

    /// Get or insert value with async factory
    pub async fn get_or_insert_with_async<F, Fut>(&self, key: K, factory: F) -> V
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = V>,
    {
        if let Some(value) = self.get(&key) {
            return value;
        }

        let value = factory().await;
        self.insert(key, value.clone());
        value
    }
}

/// User metadata cache
pub type UserCache = Cache<String, CachedUser>;

#[derive(Debug, Clone)]
pub struct CachedUser {
    pub user_id: String,
    pub email: String,
    pub username: String,
    pub role: String,
    pub network_id: String,
    pub station_id: String,
    pub is_active: bool,
}

impl UserCache {
    pub fn new_user_cache() -> Self {
        Cache::new(Duration::from_secs(
            crate::core::constants::USER_CACHE_DURATION_SECS,
        ))
    }
}

/// Token blacklist cache (for logout)
pub type TokenBlacklist = Cache<String, bool>;

impl TokenBlacklist {
    pub fn new_blacklist() -> Self {
        Cache::new(Duration::from_secs(3600)) // 1 hour default
    }

    pub fn is_blacklisted(&self, token: &str) -> bool {
        // Convert &str to String then borrow as &String
        self.get(&token.to_string()).is_some()
    }

    pub fn blacklist_token(&self, token: String, ttl_secs: u64) {
        self.insert_with_ttl(token, true, Duration::from_secs(ttl_secs));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;

    #[test]
    fn test_cache_insert_and_get() {
        let cache: Cache<String, String> = Cache::new(Duration::from_secs(10));

        cache.insert("key1".to_string(), "value1".to_string());

        assert_eq!(cache.get(&"key1".to_string()), Some("value1".to_string()));
        assert_eq!(cache.get(&"key2".to_string()), None);
    }

    #[test]
    fn test_cache_expiration() {
        let cache: Cache<String, String> = Cache::new(Duration::from_secs(1));

        cache.insert("key1".to_string(), "value1".to_string());
        assert_eq!(cache.get(&"key1".to_string()), Some("value1".to_string()));

        sleep(Duration::from_secs(2));
        assert_eq!(cache.get(&"key1".to_string()), None);
    }

    #[test]
    fn test_cache_remove() {
        let cache: Cache<String, String> = Cache::new(Duration::from_secs(10));

        cache.insert("key1".to_string(), "value1".to_string());
        assert_eq!(cache.get(&"key1".to_string()), Some("value1".to_string()));

        cache.remove(&"key1".to_string());
        assert_eq!(cache.get(&"key1".to_string()), None);
    }

    #[test]
    fn test_cache_cleanup() {
        let cache: Cache<String, String> = Cache::new(Duration::from_secs(1));

        cache.insert("key1".to_string(), "value1".to_string());
        cache.insert("key2".to_string(), "value2".to_string());

        assert_eq!(cache.len(), 2);

        sleep(Duration::from_secs(2));
        cache.cleanup_expired();

        assert_eq!(cache.len(), 0);
    }
}
