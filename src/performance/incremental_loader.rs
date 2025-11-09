//! Incremental data loader for streaming large datasets
//!
//! Provides efficient loading of large datasets by streaming data in chunks
//! rather than loading everything at once. Useful for:
//! - Large file lists
//! - Database query results
//! - Log files
//! - Search results
//!
//! # Examples
//!
//! ```
//! use toad::performance::IncrementalLoader;
//!
//! let items = vec!["item1", "item2", "item3"];
//! let mut loader = IncrementalLoader::new(items, 10);
//!
//! assert_eq!(loader.total_count(), 3);
//! assert_eq!(loader.loaded_count(), 0);
//!
//! loader.load_next_chunk();
//! assert_eq!(loader.loaded_count(), 3); // All loaded since batch size > total
//! ```

use std::cmp::min;

/// Incremental data loader for large datasets
///
/// Loads data in chunks to avoid blocking the UI and reduce memory usage.
///
/// # Type Parameters
///
/// * `T` - The type of items being loaded
#[derive(Debug, Clone)]
pub struct IncrementalLoader<T> {
    /// All items (may be lazily loaded)
    items: Vec<T>,
    /// Number of items loaded so far
    loaded_count: usize,
    /// Items to load per chunk
    chunk_size: usize,
    /// Whether all items are loaded
    fully_loaded: bool,
}

impl<T> IncrementalLoader<T> {
    /// Create a new incremental loader
    ///
    /// # Arguments
    ///
    /// * `items` - The full dataset
    /// * `chunk_size` - Number of items to load per chunk
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::performance::IncrementalLoader;
    ///
    /// let items = vec![1, 2, 3, 4, 5];
    /// let loader = IncrementalLoader::new(items, 2);
    /// assert_eq!(loader.chunk_size(), 2);
    /// ```
    pub fn new(items: Vec<T>, chunk_size: usize) -> Self {
        let chunk_size = chunk_size.max(1); // Ensure at least 1
        let fully_loaded = items.is_empty(); // Empty is fully loaded
        Self {
            items,
            loaded_count: 0,
            chunk_size,
            fully_loaded,
        }
    }

    /// Create an empty incremental loader
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::performance::IncrementalLoader;
    ///
    /// let loader: IncrementalLoader<String> = IncrementalLoader::empty(10);
    /// assert_eq!(loader.total_count(), 0);
    /// ```
    pub fn empty(chunk_size: usize) -> Self {
        Self::new(Vec::new(), chunk_size)
    }

    /// Load the next chunk of items
    ///
    /// Returns the number of items newly loaded.
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::performance::IncrementalLoader;
    ///
    /// let items = vec![1, 2, 3, 4, 5];
    /// let mut loader = IncrementalLoader::new(items, 2);
    ///
    /// let loaded = loader.load_next_chunk();
    /// assert_eq!(loaded, 2);
    /// assert_eq!(loader.loaded_count(), 2);
    /// ```
    pub fn load_next_chunk(&mut self) -> usize {
        if self.fully_loaded {
            return 0;
        }

        let remaining = self.items.len() - self.loaded_count;
        let to_load = min(self.chunk_size, remaining);

        self.loaded_count += to_load;

        if self.loaded_count >= self.items.len() {
            self.fully_loaded = true;
        }

        to_load
    }

    /// Load all remaining items
    ///
    /// Returns the number of items newly loaded.
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::performance::IncrementalLoader;
    ///
    /// let items = vec![1, 2, 3, 4, 5];
    /// let mut loader = IncrementalLoader::new(items, 2);
    ///
    /// loader.load_next_chunk(); // Load 2
    /// let loaded = loader.load_all();
    /// assert_eq!(loaded, 3); // Remaining 3
    /// assert!(loader.is_fully_loaded());
    /// ```
    pub fn load_all(&mut self) -> usize {
        if self.fully_loaded {
            return 0;
        }

        let remaining = self.items.len() - self.loaded_count;
        self.loaded_count = self.items.len();
        self.fully_loaded = true;

        remaining
    }

    /// Get the currently loaded items
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::performance::IncrementalLoader;
    ///
    /// let items = vec![1, 2, 3, 4, 5];
    /// let mut loader = IncrementalLoader::new(items, 2);
    ///
    /// loader.load_next_chunk();
    /// assert_eq!(loader.loaded_items(), &[1, 2]);
    /// ```
    pub fn loaded_items(&self) -> &[T] {
        &self.items[..self.loaded_count]
    }

    /// Get mutable access to loaded items
    pub fn loaded_items_mut(&mut self) -> &mut [T] {
        &mut self.items[..self.loaded_count]
    }

    /// Get the total number of items
    pub fn total_count(&self) -> usize {
        self.items.len()
    }

    /// Get the number of items currently loaded
    pub fn loaded_count(&self) -> usize {
        self.loaded_count
    }

    /// Get the number of remaining items to load
    pub fn remaining_count(&self) -> usize {
        self.items.len() - self.loaded_count
    }

    /// Check if all items are loaded
    pub fn is_fully_loaded(&self) -> bool {
        self.fully_loaded
    }

    /// Get the chunk size
    pub fn chunk_size(&self) -> usize {
        self.chunk_size
    }

    /// Set a new chunk size
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::performance::IncrementalLoader;
    ///
    /// let items = vec![1, 2, 3, 4, 5];
    /// let mut loader = IncrementalLoader::new(items, 2);
    ///
    /// loader.set_chunk_size(10);
    /// assert_eq!(loader.chunk_size(), 10);
    /// ```
    pub fn set_chunk_size(&mut self, size: usize) {
        self.chunk_size = size.max(1);
    }

    /// Reset the loader to initial state
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::performance::IncrementalLoader;
    ///
    /// let items = vec![1, 2, 3, 4, 5];
    /// let mut loader = IncrementalLoader::new(items, 2);
    ///
    /// loader.load_all();
    /// assert_eq!(loader.loaded_count(), 5);
    ///
    /// loader.reset();
    /// assert_eq!(loader.loaded_count(), 0);
    /// ```
    pub fn reset(&mut self) {
        self.loaded_count = 0;
        self.fully_loaded = false;
    }

    /// Get loading progress as a percentage (0.0 to 1.0)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::performance::IncrementalLoader;
    ///
    /// let items = vec![1, 2, 3, 4];
    /// let mut loader = IncrementalLoader::new(items, 2);
    ///
    /// loader.load_next_chunk();
    /// assert_eq!(loader.progress(), 0.5); // 2 out of 4
    /// ```
    pub fn progress(&self) -> f64 {
        if self.items.is_empty() {
            1.0
        } else {
            self.loaded_count as f64 / self.items.len() as f64
        }
    }

    /// Replace all items and reset loader
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::performance::IncrementalLoader;
    ///
    /// let mut loader = IncrementalLoader::new(vec![1, 2], 10);
    /// loader.set_items(vec![3, 4, 5]);
    /// assert_eq!(loader.total_count(), 3);
    /// assert_eq!(loader.loaded_count(), 0);
    /// ```
    pub fn set_items(&mut self, items: Vec<T>) {
        self.items = items;
        self.reset();
    }

    /// Add items to the end of the dataset
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::performance::IncrementalLoader;
    ///
    /// let mut loader = IncrementalLoader::new(vec![1, 2], 10);
    /// loader.add_items(vec![3, 4]);
    /// assert_eq!(loader.total_count(), 4);
    /// ```
    pub fn add_items(&mut self, mut items: Vec<T>) {
        self.items.append(&mut items);
        self.fully_loaded = false;
    }

    /// Consume the loader and return all items
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::performance::IncrementalLoader;
    ///
    /// let loader = IncrementalLoader::new(vec![1, 2, 3], 10);
    /// let items = loader.into_items();
    /// assert_eq!(items, vec![1, 2, 3]);
    /// ```
    pub fn into_items(self) -> Vec<T> {
        self.items
    }
}

impl<T> Default for IncrementalLoader<T> {
    fn default() -> Self {
        Self::empty(100)
    }
}

/// Async incremental loader for streaming data from async sources
///
/// Useful for loading data from databases, APIs, or files.
#[derive(Debug)]
pub struct AsyncIncrementalLoader<T> {
    /// Loaded items
    items: Vec<T>,
    /// Chunk size
    chunk_size: usize,
    /// Whether source is exhausted
    exhausted: bool,
}

impl<T> AsyncIncrementalLoader<T> {
    /// Create a new async incremental loader
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::performance::AsyncIncrementalLoader;
    ///
    /// let loader: AsyncIncrementalLoader<String> = AsyncIncrementalLoader::new(10);
    /// assert_eq!(loader.chunk_size(), 10);
    /// ```
    pub fn new(chunk_size: usize) -> Self {
        Self {
            items: Vec::new(),
            chunk_size: chunk_size.max(1),
            exhausted: false,
        }
    }

    /// Add a chunk of loaded items
    ///
    /// # Arguments
    ///
    /// * `items` - New items to add
    /// * `exhausted` - Whether the data source is exhausted
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::performance::AsyncIncrementalLoader;
    ///
    /// let mut loader = AsyncIncrementalLoader::new(10);
    /// loader.add_chunk(vec![1, 2, 3], false);
    /// assert_eq!(loader.loaded_count(), 3);
    /// assert!(!loader.is_exhausted());
    /// ```
    pub fn add_chunk(&mut self, mut items: Vec<T>, exhausted: bool) {
        self.items.append(&mut items);
        self.exhausted = exhausted;
    }

    /// Get all loaded items
    pub fn items(&self) -> &[T] {
        &self.items
    }

    /// Get the number of loaded items
    pub fn loaded_count(&self) -> usize {
        self.items.len()
    }

    /// Check if the data source is exhausted
    pub fn is_exhausted(&self) -> bool {
        self.exhausted
    }

    /// Get the chunk size
    pub fn chunk_size(&self) -> usize {
        self.chunk_size
    }

    /// Reset the loader
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::performance::AsyncIncrementalLoader;
    ///
    /// let mut loader = AsyncIncrementalLoader::new(10);
    /// loader.add_chunk(vec![1, 2, 3], true);
    ///
    /// loader.reset();
    /// assert_eq!(loader.loaded_count(), 0);
    /// assert!(!loader.is_exhausted());
    /// ```
    pub fn reset(&mut self) {
        self.items.clear();
        self.exhausted = false;
    }
}

impl<T> Default for AsyncIncrementalLoader<T> {
    fn default() -> Self {
        Self::new(100)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_incremental_loader_new() {
        let items = vec![1, 2, 3, 4, 5];
        let loader = IncrementalLoader::new(items, 2);

        assert_eq!(loader.total_count(), 5);
        assert_eq!(loader.loaded_count(), 0);
        assert_eq!(loader.chunk_size(), 2);
        assert!(!loader.is_fully_loaded());
    }

    #[test]
    fn test_incremental_loader_load_next_chunk() {
        let items = vec![1, 2, 3, 4, 5];
        let mut loader = IncrementalLoader::new(items, 2);

        let loaded = loader.load_next_chunk();
        assert_eq!(loaded, 2);
        assert_eq!(loader.loaded_count(), 2);
        assert_eq!(loader.loaded_items(), &[1, 2]);

        let loaded = loader.load_next_chunk();
        assert_eq!(loaded, 2);
        assert_eq!(loader.loaded_count(), 4);
        assert_eq!(loader.loaded_items(), &[1, 2, 3, 4]);

        let loaded = loader.load_next_chunk();
        assert_eq!(loaded, 1);
        assert_eq!(loader.loaded_count(), 5);
        assert!(loader.is_fully_loaded());
    }

    #[test]
    fn test_incremental_loader_load_all() {
        let items = vec![1, 2, 3, 4, 5];
        let mut loader = IncrementalLoader::new(items, 2);

        loader.load_next_chunk(); // Load 2
        let remaining = loader.load_all();

        assert_eq!(remaining, 3);
        assert_eq!(loader.loaded_count(), 5);
        assert!(loader.is_fully_loaded());
    }

    #[test]
    fn test_incremental_loader_progress() {
        let items = vec![1, 2, 3, 4];
        let mut loader = IncrementalLoader::new(items, 2);

        assert_eq!(loader.progress(), 0.0);

        loader.load_next_chunk();
        assert_eq!(loader.progress(), 0.5);

        loader.load_all();
        assert_eq!(loader.progress(), 1.0);
    }

    #[test]
    fn test_incremental_loader_reset() {
        let items = vec![1, 2, 3];
        let mut loader = IncrementalLoader::new(items, 2);

        loader.load_all();
        assert_eq!(loader.loaded_count(), 3);

        loader.reset();
        assert_eq!(loader.loaded_count(), 0);
        assert!(!loader.is_fully_loaded());
    }

    #[test]
    fn test_incremental_loader_empty() {
        let loader: IncrementalLoader<i32> = IncrementalLoader::empty(10);

        assert_eq!(loader.total_count(), 0);
        assert_eq!(loader.loaded_count(), 0);
        assert!(loader.is_fully_loaded()); // Empty is considered fully loaded
    }

    #[test]
    fn test_incremental_loader_set_items() {
        let mut loader = IncrementalLoader::new(vec![1, 2], 10);
        loader.load_all();

        loader.set_items(vec![3, 4, 5]);

        assert_eq!(loader.total_count(), 3);
        assert_eq!(loader.loaded_count(), 0);
    }

    #[test]
    fn test_incremental_loader_add_items() {
        let mut loader = IncrementalLoader::new(vec![1, 2], 10);
        loader.load_all();

        loader.add_items(vec![3, 4]);

        assert_eq!(loader.total_count(), 4);
        assert!(!loader.is_fully_loaded());
    }

    #[test]
    fn test_async_incremental_loader() {
        let mut loader: AsyncIncrementalLoader<i32> = AsyncIncrementalLoader::new(10);

        loader.add_chunk(vec![1, 2, 3], false);
        assert_eq!(loader.loaded_count(), 3);
        assert!(!loader.is_exhausted());

        loader.add_chunk(vec![4, 5], true);
        assert_eq!(loader.loaded_count(), 5);
        assert!(loader.is_exhausted());

        loader.reset();
        assert_eq!(loader.loaded_count(), 0);
        assert!(!loader.is_exhausted());
    }
}
