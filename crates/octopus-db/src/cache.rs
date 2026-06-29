use dashmap::DashMap;
use std::hash::Hash;
use std::sync::Arc;

/// Generic in-memory cache backed by DashMap
pub struct Cache<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    inner: DashMap<K, V>,
}

impl<K, V> Cache<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    pub fn new() -> Self {
        Self {
            inner: DashMap::new(),
        }
    }

    pub fn get(&self, key: &K) -> Option<V> {
        self.inner.get(key).map(|r| r.value().clone())
    }

    pub fn insert(&self, key: K, value: V) {
        self.inner.insert(key, value);
    }

    pub fn remove(&self, key: &K) -> Option<V> {
        self.inner.remove(key).map(|(_, v)| v)
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn clear(&self) {
        self.inner.clear();
    }

    pub fn contains(&self, key: &K) -> bool {
        self.inner.contains_key(key)
    }
}

impl<K, V> Default for Cache<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

/// Thread-safe cache alias
pub type SharedCache<K, V> = Arc<Cache<K, V>>;

pub fn new_shared_cache<K, V>() -> SharedCache<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    Arc::new(Cache::new())
}