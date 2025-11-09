/// LRU cache for parsed AST contexts
///
/// Caches FileContext by (path, modification_time) to avoid re-parsing unchanged files.
use super::FileContext;
use lru::LruCache;
use std::num::NonZeroUsize;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

/// Cache key: (file path, modification time)
type CacheKey = (PathBuf, SystemTime);

/// Thread-safe LRU cache for AST contexts
#[derive(Clone)]
pub struct AstCache {
    cache: Arc<Mutex<LruCache<CacheKey, FileContext>>>,
}

impl AstCache {
    /// Create a new AST cache with default capacity (100 entries)
    pub fn new() -> Self {
        Self::with_capacity(100)
    }

    /// Create a new AST cache with specified capacity
    pub fn with_capacity(capacity: usize) -> Self {
        let capacity = NonZeroUsize::new(capacity).expect("Capacity must be > 0");
        Self {
            cache: Arc::new(Mutex::new(LruCache::new(capacity))),
        }
    }

    /// Get a cached file context if it exists and is up-to-date
    ///
    /// # Arguments
    /// * `path` - File path
    /// * `mtime` - Current modification time
    ///
    /// # Returns
    /// * `Some(FileContext)` if cached and mtime matches
    /// * `None` if not cached or mtime changed (stale)
    pub fn get(&self, path: &PathBuf, mtime: SystemTime) -> Option<FileContext> {
        let mut cache = self.cache.lock().unwrap();
        cache.get(&(path.clone(), mtime)).cloned()
    }

    /// Insert a file context into the cache
    ///
    /// # Arguments
    /// * `path` - File path
    /// * `mtime` - Modification time
    /// * `context` - Parsed file context
    pub fn insert(&self, path: PathBuf, mtime: SystemTime, context: FileContext) {
        let mut cache = self.cache.lock().unwrap();
        cache.put((path, mtime), context);
    }

    /// Clear all cached entries
    pub fn clear(&self) {
        let mut cache = self.cache.lock().unwrap();
        cache.clear();
    }

    /// Get the number of cached entries
    pub fn size(&self) -> usize {
        let cache = self.cache.lock().unwrap();
        cache.len()
    }

    /// Get cache capacity
    pub fn capacity(&self) -> usize {
        let cache = self.cache.lock().unwrap();
        cache.cap().get()
    }
}

impl Default for AstCache {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai::context::{Language, Symbol, SymbolKind};
    use std::time::Duration;

    fn create_test_context(path: &str) -> FileContext {
        let mut ctx = FileContext::new(PathBuf::from(path), Language::Python);
        ctx.add_symbol(Symbol::new("test", SymbolKind::Function, (1, 10)));
        ctx
    }

    #[test]
    fn test_cache_hit() {
        let cache = AstCache::new();
        let path = PathBuf::from("test.py");
        let mtime = SystemTime::now();
        let context = create_test_context("test.py");

        // Insert into cache
        cache.insert(path.clone(), mtime, context.clone());

        // Should retrieve
        let retrieved = cache.get(&path, mtime);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().symbols.len(), 1);
    }

    #[test]
    fn test_cache_miss_different_mtime() {
        let cache = AstCache::new();
        let path = PathBuf::from("test.py");
        let old_mtime = SystemTime::now();
        let context = create_test_context("test.py");

        // Insert with old mtime
        cache.insert(path.clone(), old_mtime, context);

        // Try to get with new mtime
        let new_mtime = old_mtime + Duration::from_secs(10);
        let retrieved = cache.get(&path, new_mtime);
        assert!(retrieved.is_none()); // Cache miss due to mtime change
    }

    #[test]
    fn test_cache_eviction() {
        let cache = AstCache::with_capacity(2);
        let mtime = SystemTime::now();

        // Insert 3 items (capacity is 2, so LRU will evict)
        cache.insert(
            PathBuf::from("file1.py"),
            mtime,
            create_test_context("file1.py"),
        );
        cache.insert(
            PathBuf::from("file2.py"),
            mtime,
            create_test_context("file2.py"),
        );
        cache.insert(
            PathBuf::from("file3.py"),
            mtime,
            create_test_context("file3.py"),
        );

        // Cache should have exactly 2 entries (LRU evicted file1)
        assert_eq!(cache.size(), 2);
    }

    #[test]
    fn test_cache_clear() {
        let cache = AstCache::new();
        let path = PathBuf::from("test.py");
        let mtime = SystemTime::now();

        cache.insert(path.clone(), mtime, create_test_context("test.py"));
        assert_eq!(cache.size(), 1);

        cache.clear();
        assert_eq!(cache.size(), 0);
    }

    #[test]
    fn test_cache_capacity() {
        let cache = AstCache::with_capacity(50);
        assert_eq!(cache.capacity(), 50);
    }

    #[test]
    fn test_cache_thread_safety() {
        use std::thread;

        let cache = AstCache::new();
        let cache_clone = cache.clone();

        // Spawn thread that inserts
        let handle = thread::spawn(move || {
            let path = PathBuf::from("thread_test.py");
            let mtime = SystemTime::now();
            cache_clone.insert(path, mtime, create_test_context("thread_test.py"));
        });

        handle.join().unwrap();

        // Should be accessible from main thread
        assert_eq!(cache.size(), 1);
    }
}
