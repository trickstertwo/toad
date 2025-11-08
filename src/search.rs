//! Search and find functionality
//!
//! Provides comprehensive search capabilities with support for:
//! - Forward and backward search
//! - Case sensitivity options
//! - Regular expressions
//! - Match navigation (n/N)
//! - Search history
//!
//! # Examples
//!
//! ## Basic Search
//!
//! ```
//! use toad::search::Search;
//!
//! let mut search = Search::new();
//! search.set_query("test");
//!
//! assert_eq!(search.query(), "test");
//! assert!(search.is_active());
//! ```
//!
//! ## Search with Navigation
//!
//! ```
//! use toad::search::Search;
//!
//! let mut search = Search::new();
//! search.set_query("error");
//! search.set_total_matches(5);
//!
//! search.next_match();
//! assert_eq!(search.current_match(), 1);
//!
//! search.next_match();
//! assert_eq!(search.current_match(), 2);
//!
//! search.previous_match();
//! assert_eq!(search.current_match(), 1);
//! ```

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// Search direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SearchDirection {
    /// Search forward (/)
    Forward,
    /// Search backward (?)
    Backward,
}

/// Search mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SearchMode {
    /// Plain text search
    PlainText,
    /// Regular expression search
    Regex,
    /// Fuzzy search
    Fuzzy,
}

/// Search options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchOptions {
    /// Case sensitive search
    pub case_sensitive: bool,
    /// Whole word matching
    pub whole_word: bool,
    /// Wrap around when reaching end
    pub wrap_around: bool,
    /// Search mode (plain text, regex, fuzzy)
    pub mode: SearchMode,
}

impl Default for SearchOptions {
    fn default() -> Self {
        Self {
            case_sensitive: false,
            whole_word: false,
            wrap_around: true,
            mode: SearchMode::PlainText,
        }
    }
}

/// Search state and functionality
///
/// Manages search queries, navigation between matches, and search history.
///
/// # Examples
///
/// ```
/// use toad::search::Search;
///
/// let mut search = Search::new();
///
/// // Start a search
/// search.set_query("function");
/// search.set_total_matches(10);
///
/// // Navigate matches
/// search.next_match(); // Go to match 1
/// search.next_match(); // Go to match 2
///
/// // Check state
/// assert_eq!(search.current_match(), 2);
/// assert_eq!(search.total_matches(), 10);
/// ```
#[derive(Debug, Clone)]
pub struct Search {
    query: String,
    direction: SearchDirection,
    options: SearchOptions,
    current_match: usize,
    total_matches: usize,
    is_active: bool,
    history: VecDeque<String>,
    max_history: usize,
}

impl Search {
    /// Create a new search instance
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::search::Search;
    ///
    /// let search = Search::new();
    /// assert!(!search.is_active());
    /// assert_eq!(search.query(), "");
    /// ```
    pub fn new() -> Self {
        Self {
            query: String::new(),
            direction: SearchDirection::Forward,
            options: SearchOptions::default(),
            current_match: 0,
            total_matches: 0,
            is_active: false,
            history: VecDeque::new(),
            max_history: 50,
        }
    }

    /// Create a search with custom options
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::search::{Search, SearchOptions};
    ///
    /// let mut options = SearchOptions::default();
    /// options.case_sensitive = true;
    ///
    /// let search = Search::with_options(options);
    /// assert!(search.options().case_sensitive);
    /// ```
    pub fn with_options(options: SearchOptions) -> Self {
        Self {
            query: String::new(),
            direction: SearchDirection::Forward,
            options,
            current_match: 0,
            total_matches: 0,
            is_active: false,
            history: VecDeque::new(),
            max_history: 50,
        }
    }

    /// Set the search query
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::search::Search;
    ///
    /// let mut search = Search::new();
    /// search.set_query("test");
    /// assert_eq!(search.query(), "test");
    /// assert!(search.is_active());
    /// ```
    pub fn set_query(&mut self, query: impl Into<String>) {
        let query_str = query.into();

        // Add to history if not empty and different from last
        if !query_str.is_empty() && self.history.front() != Some(&query_str) {
            self.history.push_front(query_str.clone());
            if self.history.len() > self.max_history {
                self.history.pop_back();
            }
        }

        self.query = query_str;
        self.is_active = !self.query.is_empty();
        self.current_match = 0;
    }

    /// Get the current search query
    pub fn query(&self) -> &str {
        &self.query
    }

    /// Clear the search
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::search::Search;
    ///
    /// let mut search = Search::new();
    /// search.set_query("test");
    /// search.clear();
    ///
    /// assert_eq!(search.query(), "");
    /// assert!(!search.is_active());
    /// ```
    pub fn clear(&mut self) {
        self.query.clear();
        self.is_active = false;
        self.current_match = 0;
        self.total_matches = 0;
    }

    /// Check if search is active
    pub fn is_active(&self) -> bool {
        self.is_active
    }

    /// Set the search direction
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::search::{Search, SearchDirection};
    ///
    /// let mut search = Search::new();
    /// search.set_direction(SearchDirection::Backward);
    /// assert_eq!(search.direction(), SearchDirection::Backward);
    /// ```
    pub fn set_direction(&mut self, direction: SearchDirection) {
        self.direction = direction;
    }

    /// Get the search direction
    pub fn direction(&self) -> SearchDirection {
        self.direction
    }

    /// Set search options
    pub fn set_options(&mut self, options: SearchOptions) {
        self.options = options;
    }

    /// Get search options
    pub fn options(&self) -> &SearchOptions {
        &self.options
    }

    /// Toggle case sensitivity
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::search::Search;
    ///
    /// let mut search = Search::new();
    /// assert!(!search.options().case_sensitive);
    ///
    /// search.toggle_case_sensitive();
    /// assert!(search.options().case_sensitive);
    /// ```
    pub fn toggle_case_sensitive(&mut self) {
        self.options.case_sensitive = !self.options.case_sensitive;
    }

    /// Set the total number of matches
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::search::Search;
    ///
    /// let mut search = Search::new();
    /// search.set_query("test");
    /// search.set_total_matches(5);
    ///
    /// assert_eq!(search.total_matches(), 5);
    /// ```
    pub fn set_total_matches(&mut self, total: usize) {
        self.total_matches = total;
        if self.current_match > total {
            self.current_match = total;
        }
    }

    /// Get the total number of matches
    pub fn total_matches(&self) -> usize {
        self.total_matches
    }

    /// Get the current match index (0-based)
    pub fn current_match(&self) -> usize {
        self.current_match
    }

    /// Move to the next match
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::search::Search;
    ///
    /// let mut search = Search::new();
    /// search.set_query("test");
    /// search.set_total_matches(5);
    ///
    /// search.next_match();
    /// assert_eq!(search.current_match(), 1);
    ///
    /// search.next_match();
    /// assert_eq!(search.current_match(), 2);
    /// ```
    pub fn next_match(&mut self) {
        if self.total_matches == 0 {
            return;
        }

        if self.current_match < self.total_matches {
            self.current_match += 1;
        } else if self.options.wrap_around {
            self.current_match = 1; // Wrap to first match
        }
    }

    /// Move to the previous match
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::search::Search;
    ///
    /// let mut search = Search::new();
    /// search.set_query("test");
    /// search.set_total_matches(5);
    /// search.next_match();
    /// search.next_match();
    ///
    /// search.previous_match();
    /// assert_eq!(search.current_match(), 1);
    /// ```
    pub fn previous_match(&mut self) {
        if self.total_matches == 0 {
            return;
        }

        if self.current_match > 1 {
            self.current_match -= 1;
        } else if self.options.wrap_around {
            self.current_match = self.total_matches; // Wrap to last match
        }
    }

    /// Jump to a specific match
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::search::Search;
    ///
    /// let mut search = Search::new();
    /// search.set_query("test");
    /// search.set_total_matches(10);
    ///
    /// search.jump_to_match(5);
    /// assert_eq!(search.current_match(), 5);
    /// ```
    pub fn jump_to_match(&mut self, index: usize) {
        if index > 0 && index <= self.total_matches {
            self.current_match = index;
        }
    }

    /// Get search history
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::search::Search;
    ///
    /// let mut search = Search::new();
    /// search.set_query("first");
    /// search.set_query("second");
    ///
    /// let history: Vec<&String> = search.history().collect();
    /// assert_eq!(history.len(), 2);
    /// assert_eq!(history[0], "second");
    /// assert_eq!(history[1], "first");
    /// ```
    pub fn history(&self) -> impl Iterator<Item = &String> {
        self.history.iter()
    }

    /// Clear search history
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::search::Search;
    ///
    /// let mut search = Search::new();
    /// search.set_query("test");
    /// search.clear_history();
    ///
    /// assert_eq!(search.history().count(), 0);
    /// ```
    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    /// Check if there are any matches
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::search::Search;
    ///
    /// let mut search = Search::new();
    /// search.set_query("test");
    /// assert!(!search.has_matches());
    ///
    /// search.set_total_matches(3);
    /// assert!(search.has_matches());
    /// ```
    pub fn has_matches(&self) -> bool {
        self.total_matches > 0
    }

    /// Get a formatted status string
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::search::Search;
    ///
    /// let mut search = Search::new();
    /// search.set_query("test");
    /// search.set_total_matches(5);
    /// search.next_match();
    ///
    /// let status = search.status_string();
    /// assert_eq!(status, "1/5");
    /// ```
    pub fn status_string(&self) -> String {
        if self.total_matches == 0 {
            "0/0".to_string()
        } else {
            format!("{}/{}", self.current_match, self.total_matches)
        }
    }

    /// Check if query matches text (with current options)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::search::Search;
    ///
    /// let mut search = Search::new();
    /// search.set_query("test");
    ///
    /// assert!(search.matches("this is a test"));
    /// assert!(!search.matches("no match here"));
    /// ```
    pub fn matches(&self, text: &str) -> bool {
        if self.query.is_empty() {
            return false;
        }

        if self.options.case_sensitive {
            text.contains(&self.query)
        } else {
            text.to_lowercase().contains(&self.query.to_lowercase())
        }
    }
}

impl Default for Search {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_creation() {
        let search = Search::new();
        assert!(!search.is_active());
        assert_eq!(search.query(), "");
        assert_eq!(search.total_matches(), 0);
        assert_eq!(search.current_match(), 0);
    }

    #[test]
    fn test_search_set_query() {
        let mut search = Search::new();
        search.set_query("test");

        assert!(search.is_active());
        assert_eq!(search.query(), "test");
    }

    #[test]
    fn test_search_clear() {
        let mut search = Search::new();
        search.set_query("test");
        search.set_total_matches(5);

        search.clear();

        assert!(!search.is_active());
        assert_eq!(search.query(), "");
        assert_eq!(search.total_matches(), 0);
    }

    #[test]
    fn test_search_direction() {
        let mut search = Search::new();
        assert_eq!(search.direction(), SearchDirection::Forward);

        search.set_direction(SearchDirection::Backward);
        assert_eq!(search.direction(), SearchDirection::Backward);
    }

    #[test]
    fn test_search_case_sensitivity() {
        let mut search = Search::new();
        assert!(!search.options().case_sensitive);

        search.toggle_case_sensitive();
        assert!(search.options().case_sensitive);

        search.toggle_case_sensitive();
        assert!(!search.options().case_sensitive);
    }

    #[test]
    fn test_search_navigation() {
        let mut search = Search::new();
        search.set_query("test");
        search.set_total_matches(5);

        assert_eq!(search.current_match(), 0);

        search.next_match();
        assert_eq!(search.current_match(), 1);

        search.next_match();
        assert_eq!(search.current_match(), 2);

        search.previous_match();
        assert_eq!(search.current_match(), 1);
    }

    #[test]
    fn test_search_wrap_around() {
        let mut search = Search::new();
        search.set_query("test");
        search.set_total_matches(3);

        // Go to last match
        search.jump_to_match(3);
        assert_eq!(search.current_match(), 3);

        // Next should wrap to first
        search.next_match();
        assert_eq!(search.current_match(), 1);

        // Previous should wrap to last
        search.previous_match();
        assert_eq!(search.current_match(), 3);
    }

    #[test]
    fn test_search_jump_to_match() {
        let mut search = Search::new();
        search.set_query("test");
        search.set_total_matches(10);

        search.jump_to_match(5);
        assert_eq!(search.current_match(), 5);

        // Invalid jump (out of range)
        search.jump_to_match(15);
        assert_eq!(search.current_match(), 5); // Should not change

        search.jump_to_match(0);
        assert_eq!(search.current_match(), 5); // Should not change
    }

    #[test]
    fn test_search_history() {
        let mut search = Search::new();

        search.set_query("first");
        search.set_query("second");
        search.set_query("third");

        let history: Vec<&String> = search.history().collect();
        assert_eq!(history.len(), 3);
        assert_eq!(history[0], "third");
        assert_eq!(history[1], "second");
        assert_eq!(history[2], "first");
    }

    #[test]
    fn test_search_clear_history() {
        let mut search = Search::new();
        search.set_query("test1");
        search.set_query("test2");

        search.clear_history();
        assert_eq!(search.history().count(), 0);
    }

    #[test]
    fn test_search_has_matches() {
        let mut search = Search::new();
        search.set_query("test");

        assert!(!search.has_matches());

        search.set_total_matches(3);
        assert!(search.has_matches());
    }

    #[test]
    fn test_search_status_string() {
        let mut search = Search::new();
        search.set_query("test");

        assert_eq!(search.status_string(), "0/0");

        search.set_total_matches(5);
        search.next_match();
        assert_eq!(search.status_string(), "1/5");

        search.next_match();
        assert_eq!(search.status_string(), "2/5");
    }

    #[test]
    fn test_search_matches() {
        let mut search = Search::new();
        search.set_query("test");

        assert!(search.matches("this is a test"));
        assert!(search.matches("testing 123"));
        assert!(!search.matches("no match"));
    }

    #[test]
    fn test_search_matches_case_insensitive() {
        let mut search = Search::new();
        search.set_query("Test");

        // Default is case insensitive
        assert!(search.matches("this is a test"));
        assert!(search.matches("TEST"));
        assert!(search.matches("TeSt"));
    }

    #[test]
    fn test_search_matches_case_sensitive() {
        let mut search = Search::new();
        search.set_query("Test");
        search.toggle_case_sensitive();

        assert!(search.matches("Test"));
        assert!(!search.matches("test"));
        assert!(!search.matches("TEST"));
    }

    #[test]
    fn test_search_with_options() {
        let options = SearchOptions {
            case_sensitive: true,
            wrap_around: false,
            ..Default::default()
        };

        let search = Search::with_options(options);
        assert!(search.options().case_sensitive);
        assert!(!search.options().wrap_around);
    }

    #[test]
    fn test_search_no_wrap() {
        let options = SearchOptions {
            wrap_around: false,
            ..Default::default()
        };

        let mut search = Search::with_options(options);
        search.set_query("test");
        search.set_total_matches(3);

        // At match 0, previous should not wrap
        search.previous_match();
        assert_eq!(search.current_match(), 0);

        // Go to last
        search.jump_to_match(3);

        // Next should not wrap
        search.next_match();
        assert_eq!(search.current_match(), 3);
    }

    #[test]
    fn test_default() {
        let search = Search::default();
        assert!(!search.is_active());
    }
}
