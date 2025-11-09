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

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

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
        let contents = serde_json::to_string_pretty(self).context("Failed to serialize history")?;

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

    // ========================================
    // MEDIUM TIER EDGE CASE TESTS
    // ========================================

    // Max Size Edge Cases
    #[test]
    fn test_history_max_size_zero() {
        let mut history = History::new(0);
        history.add("test".to_string());
        assert_eq!(history.len(), 0); // Can't store anything
    }

    #[test]
    fn test_history_max_size_one() {
        let mut history = History::new(1);
        history.add("first".to_string());
        assert_eq!(history.len(), 1);

        history.add("second".to_string());
        assert_eq!(history.len(), 1);
        assert_eq!(history.get(0), Some(&"second".to_string()));
    }

    #[test]
    fn test_history_very_large_max_size() {
        let history = History::new(1_000_000);
        assert_eq!(history.max_size(), 1_000_000);
    }

    #[test]
    fn test_history_fill_to_max() {
        let mut history = History::new(10);

        for i in 0..10 {
            history.add(format!("entry {}", i));
        }

        assert_eq!(history.len(), 10);

        // Add one more - should evict oldest
        history.add("new entry".to_string());
        assert_eq!(history.len(), 10);
        assert_eq!(history.get(9), Some(&"entry 1".to_string())); // entry 0 evicted
    }

    // Unicode/Emoji Edge Cases
    #[test]
    fn test_history_unicode_entries() {
        let mut history = History::new(10);

        history.add("日本語コマンド".to_string());
        history.add("中文命令".to_string());
        history.add("العربية الأمر".to_string());

        assert_eq!(history.len(), 3);
        assert_eq!(history.get(0), Some(&"العربية الأمر".to_string()));
    }

    #[test]
    fn test_history_emoji_entries() {
        let mut history = History::new(10);

        history.add("🐸 frog command".to_string());
        history.add("🎉 party time".to_string());
        history.add("👨‍💻 coding".to_string());

        assert_eq!(history.len(), 3);
    }

    #[test]
    fn test_history_very_long_entry() {
        let mut history = History::new(10);
        let long_entry = "command ".repeat(1000);

        history.add(long_entry.clone());
        assert_eq!(history.len(), 1);
        assert_eq!(history.get(0), Some(&long_entry));
    }

    // Rapid Operations
    #[test]
    fn test_history_rapid_addition() {
        let mut history = History::new(1000);

        for i in 0..500 {
            history.add(format!("cmd {}", i));
        }

        assert_eq!(history.len(), 500);
    }

    #[test]
    fn test_history_rapid_addition_with_overflow() {
        let mut history = History::new(50);

        for i in 0..200 {
            history.add(format!("cmd {}", i));
        }

        assert_eq!(history.len(), 50);
        // Most recent should be cmd 199
        assert_eq!(history.get(0), Some(&"cmd 199".to_string()));
    }

    // Navigation Edge Cases
    #[test]
    fn test_navigation_on_empty_history() {
        let mut history = History::new(10);

        assert_eq!(history.older(), None);
        assert_eq!(history.newer(), None);
        assert_eq!(history.position(), 0);
    }

    #[test]
    fn test_navigation_single_entry() {
        let mut history = History::new(10);
        history.add("only one".to_string());

        assert_eq!(history.older(), Some(&"only one".to_string()));
        assert_eq!(history.older(), None);

        assert_eq!(history.newer(), None); // Already at newest
    }

    #[test]
    fn test_navigation_boundaries() {
        let mut history = History::new(10);
        history.add("first".to_string());
        history.add("second".to_string());
        history.add("third".to_string());

        // Navigate to end
        history.older();
        history.older();
        history.older();
        assert_eq!(history.older(), None);
        assert_eq!(history.older(), None); // Still None

        // Navigate back to start
        history.newer();
        history.newer();
        history.newer();
        assert_eq!(history.newer(), None);
        assert_eq!(history.newer(), None); // Still None
    }

    #[test]
    fn test_position_resets_on_add() {
        let mut history = History::new(10);
        history.add("first".to_string());
        history.add("second".to_string());

        history.older();
        history.older();
        assert_eq!(history.position(), 2);

        history.add("third".to_string());
        assert_eq!(history.position(), 0); // Reset
    }

    #[test]
    fn test_multiple_navigation_cycles() {
        let mut history = History::new(5);
        for i in 0..5 {
            history.add(format!("entry {}", i));
        }

        // Cycle back and forth multiple times
        for _ in 0..3 {
            for _ in 0..5 {
                history.older();
            }
            for _ in 0..4 {
                history.newer();
            }
        }

        // Should be in consistent state
        assert!(history.position() <= history.len());
    }

    // Search Edge Cases
    #[test]
    fn test_search_empty_history() {
        let history = History::new(10);
        let matches = history.search("test");
        assert_eq!(matches.len(), 0);
    }

    #[test]
    fn test_search_no_matches() {
        let mut history = History::new(10);
        history.add("cargo build".to_string());
        history.add("cargo test".to_string());

        let matches = history.search("git");
        assert_eq!(matches.len(), 0);
    }

    #[test]
    fn test_search_empty_query() {
        let mut history = History::new(10);
        history.add("test".to_string());
        history.add("other".to_string());

        let matches = history.search("");
        assert_eq!(matches.len(), 2); // Empty string matches everything
    }

    #[test]
    fn test_search_case_sensitive() {
        let mut history = History::new(10);
        history.add("Cargo Build".to_string());
        history.add("cargo test".to_string());

        let matches = history.search("Cargo");
        assert_eq!(matches.len(), 1);

        let matches = history.search("cargo");
        assert_eq!(matches.len(), 1);
    }

    #[test]
    fn test_search_with_unicode() {
        let mut history = History::new(10);
        history.add("日本語 test".to_string());
        history.add("test 中文".to_string());
        history.add("other".to_string());

        let matches = history.search("test");
        assert_eq!(matches.len(), 2);

        let matches = history.search("日本");
        assert_eq!(matches.len(), 1);
    }

    #[test]
    fn test_search_special_characters() {
        let mut history = History::new(10);
        history.add("git commit -m \"test\"".to_string());
        history.add("echo $HOME".to_string());
        history.add("ls -la".to_string());

        let matches = history.search("\"");
        assert_eq!(matches.len(), 1);

        let matches = history.search("$");
        assert_eq!(matches.len(), 1);
    }

    // Deduplication Edge Cases
    #[test]
    fn test_deduplication_exact_match() {
        let mut history = History::new(10);
        history.add("test".to_string());
        history.add("other".to_string());
        history.add("test".to_string());

        assert_eq!(history.len(), 2);
        assert_eq!(history.get(0), Some(&"test".to_string()));
        assert_eq!(history.get(1), Some(&"other".to_string()));
    }

    #[test]
    fn test_deduplication_moves_to_front() {
        let mut history = History::new(10);
        history.add("first".to_string());
        history.add("second".to_string());
        history.add("third".to_string());
        history.add("first".to_string()); // Move to front

        assert_eq!(history.len(), 3);
        assert_eq!(history.get(0), Some(&"first".to_string()));
        assert_eq!(history.get(1), Some(&"third".to_string()));
        assert_eq!(history.get(2), Some(&"second".to_string()));
    }

    #[test]
    fn test_deduplication_with_whitespace() {
        let mut history = History::new(10);
        history.add("test".to_string());
        history.add("test ".to_string()); // Different (trailing space)

        assert_eq!(history.len(), 2);
    }

    // Empty/Whitespace Edge Cases
    #[test]
    fn test_add_only_whitespace() {
        let mut history = History::new(10);
        history.add("   ".to_string());
        history.add("\t".to_string());
        history.add("\n".to_string());

        assert_eq!(history.len(), 0);
    }

    #[test]
    fn test_add_entry_with_leading_trailing_whitespace() {
        let mut history = History::new(10);
        history.add("  test  ".to_string());

        // It stores the entry as-is (doesn't trim)
        assert_eq!(history.len(), 1);
        assert_eq!(history.get(0), Some(&"  test  ".to_string()));
    }

    // Load/Save Edge Cases
    #[test]
    fn test_save_load_cycle() {
        let mut history = History::new(10);
        history.add("first".to_string());
        history.add("second".to_string());

        let temp_file = std::env::temp_dir().join("test_history.json");
        history.save(&temp_file).unwrap();

        let loaded = History::load(&temp_file).unwrap();
        assert_eq!(loaded.len(), 2);
        assert_eq!(loaded.max_size(), 10);
        assert_eq!(loaded.get(0), Some(&"second".to_string()));

        let _ = std::fs::remove_file(&temp_file);
    }

    #[test]
    fn test_load_nonexistent_file() {
        let result = History::load(Path::new("/nonexistent/path/history.json"));
        assert!(result.is_err());
    }

    #[test]
    fn test_save_creates_directory() {
        let temp_dir = std::env::temp_dir()
            .join("toad_test_nested")
            .join("deep")
            .join("path");
        let temp_file = temp_dir.join("history.json");

        let _ = std::fs::remove_dir_all(&temp_dir.parent().unwrap().parent().unwrap());

        let mut history = History::new(5);
        history.add("test".to_string());
        history.save(&temp_file).unwrap();

        assert!(temp_file.exists());

        let _ = std::fs::remove_dir_all(&temp_dir.parent().unwrap().parent().unwrap());
    }

    #[test]
    fn test_load_or_new_fallback() {
        let history = History::load_or_new(100);
        assert_eq!(history.max_size(), 100);
    }

    // Trait Tests
    #[test]
    fn test_history_clone() {
        let mut history1 = History::new(10);
        history1.add("test".to_string());

        let history2 = history1.clone();

        assert_eq!(history1.len(), history2.len());
        assert_eq!(history1.max_size(), history2.max_size());
        assert_eq!(history1.get(0), history2.get(0));
    }

    #[test]
    fn test_history_debug() {
        let history = History::new(10);
        let debug_str = format!("{:?}", history);
        assert!(debug_str.contains("History"));
    }

    #[test]
    fn test_history_serialization() {
        let mut history = History::new(10);
        history.add("test1".to_string());
        history.add("test2".to_string());

        let json = serde_json::to_string(&history).unwrap();
        let deserialized: History = serde_json::from_str(&json).unwrap();

        assert_eq!(history.len(), deserialized.len());
        assert_eq!(history.max_size(), deserialized.max_size());
    }

    // Complex Scenarios
    #[test]
    fn test_interleaved_add_navigate_search() {
        let mut history = History::new(10);

        history.add("cmd1".to_string());
        history.older();
        history.add("cmd2".to_string());
        let matches = history.search("cmd");
        history.older();
        history.add("cmd3".to_string());

        assert_eq!(matches.len(), 2);
        assert_eq!(history.len(), 3);
        assert_eq!(history.position(), 0); // Reset by last add
    }

    #[test]
    fn test_clear_resets_position() {
        let mut history = History::new(10);
        history.add("test".to_string());
        history.older();

        assert_eq!(history.position(), 1);

        history.clear();
        assert_eq!(history.position(), 0);
    }

    #[test]
    fn test_entries_returns_correct_slice() {
        let mut history = History::new(10);
        history.add("first".to_string());
        history.add("second".to_string());

        let entries = history.entries();
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0], "second");
        assert_eq!(entries[1], "first");
    }

    #[test]
    fn test_multiple_deduplication_cycles() {
        let mut history = History::new(5);

        for _ in 0..10 {
            history.add("repeat".to_string());
            history.add("other".to_string());
        }

        assert_eq!(history.len(), 2);
    }

    #[test]
    fn test_navigation_after_clear() {
        let mut history = History::new(10);
        history.add("test".to_string());
        history.older();

        history.clear();

        assert_eq!(history.older(), None);
        assert_eq!(history.newer(), None);
    }
}
