use std::collections::HashMap;
use std::sync::Mutex;
use once_cell::sync::Lazy;

static CACHE: Lazy<Mutex<HashMap<String, CacheEntry>>> = Lazy::new(|| Mutex::new(HashMap::new()));

struct CacheEntry {
    response: String,
    expires_at: std::time::Instant,
}

pub fn get(key: &str) -> Option<String> {
    if let Ok(cache) = CACHE.lock() {
        if let Some(entry) = cache.get(key) {
            if entry.expires_at > std::time::Instant::now() {
                return Some(entry.response.clone());
            }
        }
    }
    None
}

pub fn set(key: &str, value: &str, ttl_secs: u64) {
    if let Ok(mut cache) = CACHE.lock() {
        cache.insert(key.to_string(), CacheEntry {
            response: value.to_string(),
            expires_at: std::time::Instant::now() + std::time::Duration::from_secs(ttl_secs),
        });
    }
}

pub fn clear() {
    if let Ok(mut cache) = CACHE.lock() {
        cache.clear();
    }
}

pub fn len() -> usize {
    CACHE.lock().map(|c| c.len()).unwrap_or(0)
}