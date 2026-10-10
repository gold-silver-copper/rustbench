/// A least-recently-used cache from `String` keys to `i64` values that
/// holds at most `capacity` entries.
///
/// An entry is *used* when it is inserted, when `put` replaces its value,
/// and when `get` reads it. `peek`, `contains` and `len` never change which
/// entry is least recently used.
///
/// ```
/// let mut c = task::LruCache::new(2);
/// c.put("a", 1);
/// c.put("b", 2);
/// assert_eq!(c.get("a"), Some(1));
/// assert_eq!(c.put("c", 3), Some(("b".to_string(), 2)));
/// assert_eq!(c.len(), 2);
/// ```
pub struct LruCache {
    // fields are up to you
}

impl LruCache {
    /// An empty cache that holds at most `capacity` entries.
    pub fn new(capacity: usize) -> Self {
        todo!()
    }

    /// Returns the value for `key` and marks that entry as the most recently
    /// used. Returns `None` if `key` is absent.
    pub fn get(&mut self, key: &str) -> Option<i64> {
        todo!()
    }

    /// Returns the value for `key` without marking it as used.
    pub fn peek(&self, key: &str) -> Option<i64> {
        todo!()
    }

    /// Whether `key` is in the cache, without marking it as used.
    pub fn contains(&self, key: &str) -> bool {
        todo!()
    }

    /// Sets the value for `key` and makes it the most recently used entry.
    ///
    /// If `key` was already present its value is replaced, nothing is
    /// evicted, and `None` is returned. If `key` is new and the cache is
    /// full, the least recently used entry is removed and returned as
    /// `Some((key, value))`. With capacity 0 nothing is ever stored: `put`
    /// returns the given key and value as the evicted entry.
    pub fn put(&mut self, key: &str, value: i64) -> Option<(String, i64)> {
        todo!()
    }

    /// The number of entries in the cache.
    pub fn len(&self) -> usize {
        todo!()
    }

    /// Whether the cache has no entries.
    pub fn is_empty(&self) -> bool {
        todo!()
    }
}
