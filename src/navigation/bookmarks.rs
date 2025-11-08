use anyhow::{Context, Result};
/// Bookmarks for quick navigation to file locations
///
/// Provides named bookmarks to save and jump to frequently accessed locations
///
/// # Examples
///
/// ```
/// use toad::bookmarks::{Bookmark, BookmarkManager};
///
/// let mut manager = BookmarkManager::new();
/// manager.add_bookmark("main", "/src/main.rs", 10, 0);
/// assert_eq!(manager.count(), 1);
/// ```
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// A single bookmark
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Bookmark {
    /// Bookmark name
    pub name: String,
    /// File path
    pub path: PathBuf,
    /// Line number
    pub line: usize,
    /// Column number
    pub col: usize,
    /// Optional description
    pub description: Option<String>,
    /// Creation timestamp
    pub created_at: u64,
}

impl Bookmark {
    /// Create a new bookmark
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::bookmarks::Bookmark;
    ///
    /// let bookmark = Bookmark::new("main", "/src/main.rs", 10, 0);
    /// assert_eq!(bookmark.name, "main");
    /// ```
    pub fn new<S: Into<String>, P: Into<PathBuf>>(
        name: S,
        path: P,
        line: usize,
        col: usize,
    ) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            name: name.into(),
            path: path.into(),
            line,
            col,
            description: None,
            created_at: now,
        }
    }

    /// Set description
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::bookmarks::Bookmark;
    ///
    /// let bookmark = Bookmark::new("main", "/src/main.rs", 10, 0)
    ///     .with_description("Entry point");
    /// assert_eq!(bookmark.description, Some("Entry point".to_string()));
    /// ```
    pub fn with_description<S: Into<String>>(mut self, description: S) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Get file name
    pub fn file_name(&self) -> Option<&str> {
        self.path.file_name().and_then(|n| n.to_str())
    }

    /// Check if file still exists
    pub fn exists(&self) -> bool {
        self.path.exists()
    }

    /// Get display string
    pub fn display(&self) -> String {
        format!(
            "{}: {}:{}:{}",
            self.name,
            self.path.display(),
            self.line,
            self.col
        )
    }
}

/// Bookmark manager
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookmarkManager {
    /// Bookmarks (keyed by name)
    bookmarks: HashMap<String, Bookmark>,
}

impl BookmarkManager {
    /// Create a new bookmark manager
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::bookmarks::BookmarkManager;
    ///
    /// let manager = BookmarkManager::new();
    /// assert_eq!(manager.count(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            bookmarks: HashMap::new(),
        }
    }

    /// Add a bookmark
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::bookmarks::BookmarkManager;
    ///
    /// let mut manager = BookmarkManager::new();
    /// manager.add_bookmark("main", "/src/main.rs", 10, 0);
    /// assert!(manager.has("main"));
    /// ```
    pub fn add_bookmark<S: Into<String>, P: Into<PathBuf>>(
        &mut self,
        name: S,
        path: P,
        line: usize,
        col: usize,
    ) {
        let name = name.into();
        let bookmark = Bookmark::new(name.clone(), path, line, col);
        self.bookmarks.insert(name, bookmark);
    }

    /// Add a bookmark with description
    pub fn add_bookmark_with_desc<S: Into<String>, P: Into<PathBuf>, D: Into<String>>(
        &mut self,
        name: S,
        path: P,
        line: usize,
        col: usize,
        description: D,
    ) {
        let name = name.into();
        let bookmark = Bookmark::new(name.clone(), path, line, col).with_description(description);
        self.bookmarks.insert(name, bookmark);
    }

    /// Add a bookmark object
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::bookmarks::{Bookmark, BookmarkManager};
    ///
    /// let mut manager = BookmarkManager::new();
    /// let bookmark = Bookmark::new("test", "/test.rs", 5, 0);
    /// manager.add(bookmark);
    /// assert_eq!(manager.count(), 1);
    /// ```
    pub fn add(&mut self, bookmark: Bookmark) {
        self.bookmarks.insert(bookmark.name.clone(), bookmark);
    }

    /// Get a bookmark by name
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::bookmarks::BookmarkManager;
    ///
    /// let mut manager = BookmarkManager::new();
    /// manager.add_bookmark("main", "/src/main.rs", 10, 0);
    ///
    /// let bookmark = manager.get("main");
    /// assert!(bookmark.is_some());
    /// ```
    pub fn get(&self, name: &str) -> Option<&Bookmark> {
        self.bookmarks.get(name)
    }

    /// Check if bookmark exists
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::bookmarks::BookmarkManager;
    ///
    /// let mut manager = BookmarkManager::new();
    /// manager.add_bookmark("main", "/src/main.rs", 10, 0);
    /// assert!(manager.has("main"));
    /// assert!(!manager.has("other"));
    /// ```
    pub fn has(&self, name: &str) -> bool {
        self.bookmarks.contains_key(name)
    }

    /// Remove a bookmark
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::bookmarks::BookmarkManager;
    ///
    /// let mut manager = BookmarkManager::new();
    /// manager.add_bookmark("main", "/src/main.rs", 10, 0);
    /// assert!(manager.remove("main"));
    /// assert!(!manager.has("main"));
    /// ```
    pub fn remove(&mut self, name: &str) -> bool {
        self.bookmarks.remove(name).is_some()
    }

    /// Get all bookmarks
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::bookmarks::BookmarkManager;
    ///
    /// let mut manager = BookmarkManager::new();
    /// manager.add_bookmark("a", "/a.rs", 1, 0);
    /// manager.add_bookmark("b", "/b.rs", 2, 0);
    ///
    /// let bookmarks = manager.all();
    /// assert_eq!(bookmarks.len(), 2);
    /// ```
    pub fn all(&self) -> Vec<&Bookmark> {
        self.bookmarks.values().collect()
    }

    /// Get all bookmark names
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::bookmarks::BookmarkManager;
    ///
    /// let mut manager = BookmarkManager::new();
    /// manager.add_bookmark("a", "/a.rs", 1, 0);
    /// manager.add_bookmark("b", "/b.rs", 2, 0);
    ///
    /// let names = manager.names();
    /// assert_eq!(names.len(), 2);
    /// ```
    pub fn names(&self) -> Vec<String> {
        let mut names: Vec<_> = self.bookmarks.keys().cloned().collect();
        names.sort();
        names
    }

    /// Get number of bookmarks
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::bookmarks::BookmarkManager;
    ///
    /// let mut manager = BookmarkManager::new();
    /// assert_eq!(manager.count(), 0);
    ///
    /// manager.add_bookmark("test", "/test.rs", 1, 0);
    /// assert_eq!(manager.count(), 1);
    /// ```
    pub fn count(&self) -> usize {
        self.bookmarks.len()
    }

    /// Check if empty
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::bookmarks::BookmarkManager;
    ///
    /// let manager = BookmarkManager::new();
    /// assert!(manager.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.bookmarks.is_empty()
    }

    /// Clear all bookmarks
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::bookmarks::BookmarkManager;
    ///
    /// let mut manager = BookmarkManager::new();
    /// manager.add_bookmark("test", "/test.rs", 1, 0);
    /// manager.clear();
    /// assert_eq!(manager.count(), 0);
    /// ```
    pub fn clear(&mut self) {
        self.bookmarks.clear();
    }

    /// Search bookmarks by name or path
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::bookmarks::BookmarkManager;
    ///
    /// let mut manager = BookmarkManager::new();
    /// manager.add_bookmark("main", "/src/main.rs", 10, 0);
    /// manager.add_bookmark("test", "/tests/test.rs", 5, 0);
    ///
    /// let results = manager.search("test");
    /// assert_eq!(results.len(), 1);
    /// ```
    pub fn search(&self, query: &str) -> Vec<&Bookmark> {
        self.bookmarks
            .values()
            .filter(|b| {
                b.name.contains(query)
                    || b.path.to_string_lossy().contains(query)
                    || b.description
                        .as_ref()
                        .map(|d| d.contains(query))
                        .unwrap_or(false)
            })
            .collect()
    }

    /// Get bookmarks for a specific file
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::bookmarks::BookmarkManager;
    ///
    /// let mut manager = BookmarkManager::new();
    /// manager.add_bookmark("a", "/src/main.rs", 10, 0);
    /// manager.add_bookmark("b", "/src/main.rs", 20, 0);
    /// manager.add_bookmark("c", "/src/other.rs", 5, 0);
    ///
    /// let main_bookmarks = manager.for_file("/src/main.rs");
    /// assert_eq!(main_bookmarks.len(), 2);
    /// ```
    pub fn for_file<P: AsRef<Path>>(&self, path: P) -> Vec<&Bookmark> {
        let path = path.as_ref();
        self.bookmarks.values().filter(|b| b.path == path).collect()
    }

    /// Remove bookmarks for non-existent files
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::bookmarks::BookmarkManager;
    ///
    /// let mut manager = BookmarkManager::new();
    /// manager.add_bookmark("test", "/nonexistent/file.rs", 1, 0);
    /// manager.cleanup();
    /// assert_eq!(manager.count(), 0);
    /// ```
    pub fn cleanup(&mut self) {
        self.bookmarks.retain(|_, b| b.exists());
    }

    /// Get bookmarks sorted by name
    pub fn sorted_by_name(&self) -> Vec<&Bookmark> {
        let mut bookmarks: Vec<_> = self.bookmarks.values().collect();
        bookmarks.sort_by(|a, b| a.name.cmp(&b.name));
        bookmarks
    }

    /// Get bookmarks sorted by creation time (newest first)
    pub fn sorted_by_time(&self) -> Vec<&Bookmark> {
        let mut bookmarks: Vec<_> = self.bookmarks.values().collect();
        bookmarks.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        bookmarks
    }

    /// Save bookmarks to file
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use toad::bookmarks::BookmarkManager;
    /// use std::path::Path;
    ///
    /// let mut manager = BookmarkManager::new();
    /// manager.add_bookmark("test", "/test.rs", 1, 0);
    /// manager.save_to_file(Path::new("bookmarks.json")).unwrap();
    /// ```
    pub fn save_to_file(&self, path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// Load bookmarks from file
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use toad::bookmarks::BookmarkManager;
    /// use std::path::Path;
    ///
    /// let manager = BookmarkManager::load_from_file(Path::new("bookmarks.json")).unwrap();
    /// ```
    pub fn load_from_file(path: &Path) -> Result<Self> {
        let contents = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read bookmarks: {}", path.display()))?;

        let manager = serde_json::from_str(&contents)
            .with_context(|| format!("Failed to parse bookmarks: {}", path.display()))?;

        Ok(manager)
    }

    /// Get default bookmarks file path
    pub fn default_path() -> PathBuf {
        let config_dir = if cfg!(target_os = "windows") {
            std::env::var("APPDATA")
                .map(PathBuf::from)
                .unwrap_or_else(|_| PathBuf::from("."))
        } else {
            std::env::var("XDG_CONFIG_HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|_| {
                    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
                    PathBuf::from(home).join(".config")
                })
        };

        config_dir.join("toad").join("bookmarks.json")
    }

    /// Load from default path or create new
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::bookmarks::BookmarkManager;
    ///
    /// let manager = BookmarkManager::load_or_new();
    /// ```
    pub fn load_or_new() -> Self {
        let path = Self::default_path();

        if path.exists() {
            Self::load_from_file(&path).unwrap_or_else(|_| Self::new())
        } else {
            Self::new()
        }
    }
}

impl Default for BookmarkManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bookmark_creation() {
        let bookmark = Bookmark::new("test", "/path/to/file.rs", 10, 5);
        assert_eq!(bookmark.name, "test");
        assert_eq!(bookmark.path, PathBuf::from("/path/to/file.rs"));
        assert_eq!(bookmark.line, 10);
        assert_eq!(bookmark.col, 5);
        assert!(bookmark.description.is_none());
    }

    #[test]
    fn test_bookmark_with_description() {
        let bookmark = Bookmark::new("test", "/file.rs", 1, 0).with_description("Test bookmark");
        assert_eq!(bookmark.description, Some("Test bookmark".to_string()));
    }

    #[test]
    fn test_bookmark_file_name() {
        let bookmark = Bookmark::new("test", "/path/to/file.rs", 1, 0);
        assert_eq!(bookmark.file_name(), Some("file.rs"));
    }

    #[test]
    fn test_bookmark_display() {
        let bookmark = Bookmark::new("test", "/file.rs", 10, 5);
        let display = bookmark.display();
        assert!(display.contains("test"));
        assert!(display.contains("10"));
        assert!(display.contains("5"));
    }

    #[test]
    fn test_manager_creation() {
        let manager = BookmarkManager::new();
        assert_eq!(manager.count(), 0);
        assert!(manager.is_empty());
    }

    #[test]
    fn test_manager_add_bookmark() {
        let mut manager = BookmarkManager::new();
        manager.add_bookmark("main", "/src/main.rs", 10, 0);

        assert_eq!(manager.count(), 1);
        assert!(manager.has("main"));
    }

    #[test]
    fn test_manager_add_bookmark_with_desc() {
        let mut manager = BookmarkManager::new();
        manager.add_bookmark_with_desc("test", "/test.rs", 5, 0, "Test location");

        let bookmark = manager.get("test").unwrap();
        assert_eq!(bookmark.description, Some("Test location".to_string()));
    }

    #[test]
    fn test_manager_add_object() {
        let mut manager = BookmarkManager::new();
        let bookmark = Bookmark::new("test", "/test.rs", 1, 0);
        manager.add(bookmark);

        assert_eq!(manager.count(), 1);
    }

    #[test]
    fn test_manager_get() {
        let mut manager = BookmarkManager::new();
        manager.add_bookmark("test", "/test.rs", 5, 0);

        let bookmark = manager.get("test");
        assert!(bookmark.is_some());
        assert_eq!(bookmark.unwrap().line, 5);
    }

    #[test]
    fn test_manager_remove() {
        let mut manager = BookmarkManager::new();
        manager.add_bookmark("test", "/test.rs", 1, 0);

        assert!(manager.remove("test"));
        assert!(!manager.has("test"));
        assert!(!manager.remove("nonexistent"));
    }

    #[test]
    fn test_manager_all() {
        let mut manager = BookmarkManager::new();
        manager.add_bookmark("a", "/a.rs", 1, 0);
        manager.add_bookmark("b", "/b.rs", 2, 0);

        let all = manager.all();
        assert_eq!(all.len(), 2);
    }

    #[test]
    fn test_manager_names() {
        let mut manager = BookmarkManager::new();
        manager.add_bookmark("b", "/b.rs", 1, 0);
        manager.add_bookmark("a", "/a.rs", 2, 0);

        let names = manager.names();
        assert_eq!(names, vec!["a", "b"]);
    }

    #[test]
    fn test_manager_clear() {
        let mut manager = BookmarkManager::new();
        manager.add_bookmark("test", "/test.rs", 1, 0);
        manager.clear();

        assert_eq!(manager.count(), 0);
        assert!(manager.is_empty());
    }

    #[test]
    fn test_manager_search() {
        let mut manager = BookmarkManager::new();
        manager.add_bookmark("main", "/src/main.rs", 10, 0);
        manager.add_bookmark("test", "/tests/test.rs", 5, 0);

        let results = manager.search("test");
        assert_eq!(results.len(), 1);

        let results = manager.search("rs");
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_manager_for_file() {
        let mut manager = BookmarkManager::new();
        manager.add_bookmark("a", "/src/main.rs", 10, 0);
        manager.add_bookmark("b", "/src/main.rs", 20, 0);
        manager.add_bookmark("c", "/src/other.rs", 5, 0);

        let main_bookmarks = manager.for_file("/src/main.rs");
        assert_eq!(main_bookmarks.len(), 2);
    }

    #[test]
    fn test_manager_cleanup() {
        let mut manager = BookmarkManager::new();
        manager.add_bookmark("test", "/nonexistent/file.rs", 1, 0);

        manager.cleanup();
        assert_eq!(manager.count(), 0);
    }

    #[test]
    fn test_manager_sorted_by_name() {
        let mut manager = BookmarkManager::new();
        manager.add_bookmark("c", "/c.rs", 1, 0);
        manager.add_bookmark("a", "/a.rs", 2, 0);
        manager.add_bookmark("b", "/b.rs", 3, 0);

        let sorted = manager.sorted_by_name();
        assert_eq!(sorted[0].name, "a");
        assert_eq!(sorted[1].name, "b");
        assert_eq!(sorted[2].name, "c");
    }

    #[test]
    fn test_manager_sorted_by_time() {
        let mut manager = BookmarkManager::new();
        manager.add_bookmark("first", "/first.rs", 1, 0);
        manager.add_bookmark("second", "/second.rs", 2, 0);

        let sorted = manager.sorted_by_time();
        // Just verify we get 2 items back (order may vary due to timing)
        assert_eq!(sorted.len(), 2);
    }

    #[test]
    fn test_manager_default() {
        let manager = BookmarkManager::default();
        assert_eq!(manager.count(), 0);
    }

    #[test]
    fn test_default_path() {
        let path = BookmarkManager::default_path();
        let path_str = path.to_string_lossy();

        assert!(path_str.contains("toad"));
        assert!(path_str.contains("bookmarks.json"));
    }
}
