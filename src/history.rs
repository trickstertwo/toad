//! Command and action history tracking
//!
//! Provides persistent history tracking for commands and actions with search capabilities.
//!
//! # Examples
//!
//! ```
//! use toad::history::History;
//!
//! let mut history = History::new(100);
//! history.add("cargo build".to_string());
//! history.add("cargo test".to_string());
//!
//! assert_eq!(history.len(), 2);
//! assert_eq!(history.get(0), Some(&"cargo test".to_string()));
//! ```

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use anyhow::{Context, Result};

/// Command/action history with persistence and search
///
/// Maintains a bounded list of historical entries with optional
/// persistence to disk. Supports navigation, search, and deduplication.
///
/// # Examples
///
/// ```
/// use toad::history::History;
///
/// let mut history = History::new(10);
/// history.add("ls -la".to_string());
/// history.add("cd src".to_string());
///
/// assert_eq!(history.len(), 2);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct History {
    /// Maximum number of entries to keep
    max_size: usize,
    /// Historical entries (newest first)
    entries: Vec<String>,
    /// Current navigation position
    #[serde(skip)]
    position: usize,
}

impl History {
    /// Create a new history with maximum size
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::history::History;
    ///
    /// let history = History::new(100);
    /// assert_eq!(history.max_size(), 100);
    /// assert_eq!(history.len(), 0);
    /// ```
    pub fn new(max_size: usize) -> Self {
        Self {
            max_size,
            entries: Vec::new(),
            position: 0,
        }
    }

    /// Get the maximum size
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::history::History;
    ///
    /// let history = History::new(50);
    /// assert_eq!(history.max_size(), 50);
    /// ```
    pub fn max_size(&self) -> usize {
        self.max_size
    }

    /// Add an entry to history
    ///
    /// Adds to the front of the list, removing duplicates if present.
    /// Enforces maximum size by removing oldest entries.
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::history::History;
    ///
    /// let mut history = History::new(3);
    /// history.add("first".to_string());
    /// history.add("second".to_string());
    /// history.add("third".to_string());
    ///
    /// assert_eq!(history.len(), 3);
    /// assert_eq!(history.get(0), Some(&"third".to_string()));
    /// ```
    pub fn add(&mut self, entry: String) {
        // Don't add empty entries
        if entry.trim().is_empty() {
            return;
        }

        // Remove duplicate if it exists
        if let Some(pos) = self.entries.iter().position(|e| e == &entry) {
            self.entries.remove(pos);
        }

        // Add to front
        self.entries.insert(0, entry);

        // Enforce max size
        if self.entries.len() > self.max_size {
            self.entries.truncate(self.max_size);
        }

        // Reset position
        self.position = 0;
    }

    /// Get entry at index (0 = most recent)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::history::History;
    ///
    /// let mut history = History::new(10);
    /// history.add("first".to_string());
    /// history.add("second".to_string());
    ///
    /// assert_eq!(history.get(0), Some(&"second".to_string()));
    /// assert_eq!(history.get(1), Some(&"first".to_string()));
    /// assert_eq!(history.get(2), None);
    /// ```
    pub fn get(&self, index: usize) -> Option<&String> {
        self.entries.get(index)
    }

    /// Get current number of entries
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::history::History;
    ///
    /// let mut history = History::new(10);
    /// assert_eq!(history.len(), 0);
    ///
    /// history.add("test".to_string());
    /// assert_eq!(history.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if history is empty
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::history::History;
    ///
    /// let mut history = History::new(10);
    /// assert!(history.is_empty());
    ///
    /// history.add("test".to_string());
    /// assert!(!history.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Navigate to older entry (backward in history)
    ///
    /// Returns the entry at the current position and advances to next older entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::history::History;
    ///
    /// let mut history = History::new(10);
    /// history.add("first".to_string());
    /// history.add("second".to_string());
    ///
    /// assert_eq!(history.older(), Some(&"second".to_string()));
    /// assert_eq!(history.older(), Some(&"first".to_string()));
    /// assert_eq!(history.older(), None);
    /// ```
    pub fn older(&mut self) -> Option<&String> {
        if self.position < self.entries.len() {
            let entry = self.entries.get(self.position);
            if entry.is_some() {
                self.position += 1;
            }
            entry
        } else {
            None
        }
    }

    /// Navigate to newer entry (forward in history)
    ///
    /// Returns the entry at the new position after moving forward.
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::history::History;
    ///
    /// let mut history = History::new(10);
    /// history.add("first".to_string());
    /// history.add("second".to_string());
    ///
    /// // Go back twice
    /// history.older();
    /// history.older();
    ///
    /// // Come forward (skip the entry we just saw)
    /// assert_eq!(history.newer(), Some(&"second".to_string()));
    /// assert_eq!(history.newer(), None);
    /// ```
    pub fn newer(&mut self) -> Option<&String> {
        if self.position > 1 {
            self.position -= 1;
            self.entries.get(self.position - 1)
        } else if self.position == 1 {
            self.position = 0;
            None
        } else {
            None
        }
    }

    /// Reset navigation position to start
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::history::History;
    ///
    /// let mut history = History::new(10);
    /// history.add("test".to_string());
    /// history.older();
    ///
    /// history.reset_position();
    /// assert_eq!(history.position(), 0);
    /// ```
    pub fn reset_position(&mut self) {
        self.position = 0;
    }

    /// Get current navigation position
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::history::History;
    ///
    /// let mut history = History::new(10);
    /// history.add("test".to_string());
    ///
    /// assert_eq!(history.position(), 0);
    /// history.older();
    /// assert_eq!(history.position(), 1);
    /// ```
    pub fn position(&self) -> usize {
        self.position
    }

    /// Search history for entries containing the query
    ///
    /// Returns indices of matching entries.
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::history::History;
    ///
    /// let mut history = History::new(10);
    /// history.add("cargo build".to_string());
    /// history.add("cargo test".to_string());
    /// history.add("git status".to_string());
    ///
    /// let matches = history.search("cargo");
    /// assert_eq!(matches.len(), 2);
    /// ```
    pub fn search(&self, query: &str) -> Vec<usize> {
        self.entries
            .iter()
            .enumerate()
            .filter(|(_, entry)| entry.contains(query))
            .map(|(idx, _)| idx)
            .collect()
    }

    /// Clear all history
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::history::History;
    ///
    /// let mut history = History::new(10);
    /// history.add("test".to_string());
    /// assert_eq!(history.len(), 1);
    ///
    /// history.clear();
    /// assert_eq!(history.len(), 0);
    /// ```
    pub fn clear(&mut self) {
        self.entries.clear();
        self.position = 0;
    }

    /// Get all entries as a slice (newest first)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::history::History;
    ///
    /// let mut history = History::new(10);
    /// history.add("first".to_string());
    /// history.add("second".to_string());
    ///
    /// let entries = history.entries();
    /// assert_eq!(entries[0], "second");
    /// assert_eq!(entries[1], "first");
    /// ```
    pub fn entries(&self) -> &[String] {
        &self.entries
    }

    /// Load history from file
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use toad::history::History;
    /// use std::path::Path;
    ///
    /// let history = History::load(Path::new("history.json")).unwrap();
    /// ```
    pub fn load(path: &Path) -> Result<Self> {
        let contents = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read history file: {}", path.display()))?;

        let history: History = serde_json::from_str(&contents)
            .with_context(|| format!("Failed to parse history file: {}", path.display()))?;

        Ok(history)
    }

    /// Save history to file
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use toad::history::History;
    /// use std::path::Path;
    ///
    /// let mut history = History::new(100);
    /// history.add("test".to_string());
    /// history.save(Path::new("history.json")).unwrap();
    /// ```
    pub fn save(&self, path: &Path) -> Result<()> {
        let contents = serde_json::to_string_pretty(self)
            .context("Failed to serialize history")?;

        // Create parent directory if needed
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
        }

        std::fs::write(path, contents)
            .with_context(|| format!("Failed to write history file: {}", path.display()))?;

        Ok(())
    }

    /// Get default history file path
    ///
    /// Returns `~/.config/toad/history.json` on Unix-like systems,
    /// or `%APPDATA%\toad\history.json` on Windows.
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::history::History;
    ///
    /// let path = History::default_path();
    /// assert!(path.ends_with("toad/history.json") || path.ends_with("toad\\history.json"));
    /// ```
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

        config_dir.join("toad").join("history.json")
    }

    /// Load history from default path, or create new if file doesn't exist
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::history::History;
    ///
    /// let history = History::load_or_new(100);
    /// assert_eq!(history.max_size(), 100);
    /// ```
    pub fn load_or_new(max_size: usize) -> Self {
        let path = Self::default_path();

        if path.exists() {
            Self::load(&path).unwrap_or_else(|_| Self::new(max_size))
        } else {
            Self::new(max_size)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_history_creation() {
        let history = History::new(100);
        assert_eq!(history.max_size(), 100);
        assert_eq!(history.len(), 0);
        assert!(history.is_empty());
    }

    #[test]
    fn test_history_add() {
        let mut history = History::new(3);
        history.add("first".to_string());
        history.add("second".to_string());
        history.add("third".to_string());

        assert_eq!(history.len(), 3);
        assert_eq!(history.get(0), Some(&"third".to_string()));
        assert_eq!(history.get(1), Some(&"second".to_string()));
        assert_eq!(history.get(2), Some(&"first".to_string()));
    }

    #[test]
    fn test_history_max_size() {
        let mut history = History::new(2);
        history.add("first".to_string());
        history.add("second".to_string());
        history.add("third".to_string());

        assert_eq!(history.len(), 2);
        assert_eq!(history.get(0), Some(&"third".to_string()));
        assert_eq!(history.get(1), Some(&"second".to_string()));
    }

    #[test]
    fn test_history_deduplication() {
        let mut history = History::new(10);
        history.add("test".to_string());
        history.add("other".to_string());
        history.add("test".to_string());

        assert_eq!(history.len(), 2);
        assert_eq!(history.get(0), Some(&"test".to_string()));
        assert_eq!(history.get(1), Some(&"other".to_string()));
    }

    #[test]
    fn test_history_navigation() {
        let mut history = History::new(10);
        history.add("first".to_string());
        history.add("second".to_string());
        history.add("third".to_string());

        // Navigate backward through history (newest to oldest)
        assert_eq!(history.older(), Some(&"third".to_string()));
        assert_eq!(history.older(), Some(&"second".to_string()));
        assert_eq!(history.older(), Some(&"first".to_string()));
        assert_eq!(history.older(), None);

        // Navigate forward through history (going back toward newest)
        assert_eq!(history.newer(), Some(&"second".to_string()));
        assert_eq!(history.newer(), Some(&"third".to_string()));
        assert_eq!(history.newer(), None); // At most recent, nowhere to go
    }

    #[test]
    fn test_history_search() {
        let mut history = History::new(10);
        history.add("cargo build".to_string());
        history.add("cargo test".to_string());
        history.add("git status".to_string());
        history.add("cargo clippy".to_string());

        let matches = history.search("cargo");
        assert_eq!(matches.len(), 3);

        let matches = history.search("git");
        assert_eq!(matches.len(), 1);
    }

    #[test]
    fn test_history_clear() {
        let mut history = History::new(10);
        history.add("test".to_string());
        history.add("other".to_string());

        assert_eq!(history.len(), 2);

        history.clear();
        assert_eq!(history.len(), 0);
        assert!(history.is_empty());
    }

    #[test]
    fn test_history_empty_entries() {
        let mut history = History::new(10);
        history.add("".to_string());
        history.add("  ".to_string());

        assert_eq!(history.len(), 0);
    }

    #[test]
    fn test_history_position_reset() {
        let mut history = History::new(10);
        history.add("test".to_string());
        history.older();

        assert_eq!(history.position(), 1);

        history.reset_position();
        assert_eq!(history.position(), 0);
    }

    #[test]
    fn test_default_path() {
        let path = History::default_path();
        let path_str = path.to_string_lossy();

        assert!(path_str.contains("toad"));
        assert!(path_str.contains("history.json"));
    }
}
