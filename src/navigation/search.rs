/// Search functionality for text content
///
/// Provides search capabilities with highlighting and navigation
use serde::{Deserialize, Serialize};

/// Search result containing matched text and position
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchMatch {
    /// Line number (0-indexed)
    pub line: usize,
    /// Column start position
    pub start: usize,
    /// Column end position
    pub end: usize,
    /// The matched text
    pub text: String,
}

/// Search state for tracking current search
#[derive(Debug, Clone)]
pub struct SearchState {
    /// Current search query
    query: String,
    /// All matches found
    matches: Vec<SearchMatch>,
    /// Currently selected match index
    current: Option<usize>,
    /// Whether search is case-sensitive
    case_sensitive: bool,
    /// Whether to use regex
    use_regex: bool,
}

impl SearchState {
    /// Create a new empty search state
    pub fn new() -> Self {
        Self {
            query: String::new(),
            matches: Vec::new(),
            current: None,
            case_sensitive: false,
            use_regex: false,
        }
    }

    /// Set the search query
    pub fn set_query(&mut self, query: String) {
        self.query = query;
        self.current = None;
    }

    /// Get the current query
    pub fn query(&self) -> &str {
        &self.query
    }

    /// Set case sensitivity
    pub fn set_case_sensitive(&mut self, sensitive: bool) {
        self.case_sensitive = sensitive;
    }

    /// Check if case sensitive
    pub fn is_case_sensitive(&self) -> bool {
        self.case_sensitive
    }

    /// Set regex mode
    pub fn set_use_regex(&mut self, use_regex: bool) {
        self.use_regex = use_regex;
    }

    /// Check if using regex
    pub fn is_using_regex(&self) -> bool {
        self.use_regex
    }

    /// Perform search on content lines
    pub fn search(&mut self, lines: &[String]) {
        self.matches.clear();

        if self.query.is_empty() {
            return;
        }

        let query = if self.case_sensitive {
            self.query.clone()
        } else {
            self.query.to_lowercase()
        };

        for (line_idx, line) in lines.iter().enumerate() {
            let search_line = if self.case_sensitive {
                line.clone()
            } else {
                line.to_lowercase()
            };

            // Simple substring search (TODO: add regex support)
            let mut start = 0;
            while let Some(pos) = search_line[start..].find(&query) {
                let absolute_pos = start + pos;
                self.matches.push(SearchMatch {
                    line: line_idx,
                    start: absolute_pos,
                    end: absolute_pos + query.len(),
                    text: line[absolute_pos..absolute_pos + query.len()].to_string(),
                });
                start = absolute_pos + 1;
            }
        }

        // Select first match if any
        if !self.matches.is_empty() {
            self.current = Some(0);
        }
    }

    /// Get all matches
    pub fn matches(&self) -> &[SearchMatch] {
        &self.matches
    }

    /// Get current match index
    pub fn current_index(&self) -> Option<usize> {
        self.current
    }

    /// Get current match
    pub fn current_match(&self) -> Option<&SearchMatch> {
        self.current.and_then(|idx| self.matches.get(idx))
    }

    /// Move to next match
    pub fn next(&mut self) -> Option<&SearchMatch> {
        if self.matches.is_empty() {
            return None;
        }

        self.current = match self.current {
            Some(idx) if idx + 1 < self.matches.len() => Some(idx + 1),
            Some(_) => Some(0), // Wrap to beginning
            None => Some(0),
        };

        self.current_match()
    }

    /// Move to previous match
    pub fn previous(&mut self) -> Option<&SearchMatch> {
        if self.matches.is_empty() {
            return None;
        }

        self.current = match self.current {
            Some(0) => Some(self.matches.len() - 1), // Wrap to end
            Some(idx) => Some(idx - 1),
            None => Some(self.matches.len() - 1),
        };

        self.current_match()
    }

    /// Get total number of matches
    pub fn match_count(&self) -> usize {
        self.matches.len()
    }

    /// Clear all search data
    pub fn clear(&mut self) {
        self.query.clear();
        self.matches.clear();
        self.current = None;
    }

    /// Check if search is active
    pub fn is_active(&self) -> bool {
        !self.query.is_empty()
    }
}

impl Default for SearchState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_basic() {
        let mut search = SearchState::new();
        let lines = vec![
            "Hello world".to_string(),
            "hello again".to_string(),
            "World of hello".to_string(),
        ];

        search.set_query("hello".to_string());
        search.search(&lines);

        assert_eq!(search.match_count(), 3); // Case-insensitive by default
        assert_eq!(search.current_index(), Some(0));
    }

    #[test]
    fn test_search_case_sensitive() {
        let mut search = SearchState::new();
        search.set_case_sensitive(true);

        let lines = vec!["Hello world".to_string(), "hello again".to_string()];

        search.set_query("hello".to_string());
        search.search(&lines);

        assert_eq!(search.match_count(), 1);
    }

    #[test]
    fn test_search_navigation() {
        let mut search = SearchState::new();
        let lines = vec!["test".to_string(), "test".to_string(), "test".to_string()];

        search.set_query("test".to_string());
        search.search(&lines);

        assert_eq!(search.current_index(), Some(0));

        search.next();
        assert_eq!(search.current_index(), Some(1));

        search.next();
        assert_eq!(search.current_index(), Some(2));

        search.next(); // Should wrap
        assert_eq!(search.current_index(), Some(0));

        search.previous(); // Should wrap to end
        assert_eq!(search.current_index(), Some(2));
    }

    #[test]
    fn test_search_clear() {
        let mut search = SearchState::new();
        search.set_query("test".to_string());

        let lines = vec!["test".to_string()];
        search.search(&lines);

        assert!(search.is_active());
        assert_eq!(search.match_count(), 1);

        search.clear();
        assert!(!search.is_active());
        assert_eq!(search.match_count(), 0);
    }
}
