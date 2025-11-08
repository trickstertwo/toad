/// Fuzzy finding system (skim/fzf-style)
///
/// Provides fuzzy matching, smart case, ranking, and incremental search

use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

/// A fuzzy match result with score and positions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FuzzyMatch {
    /// The matched item
    pub item: String,
    /// Match score (higher is better)
    pub score: i32,
    /// Positions of matched characters
    pub positions: Vec<usize>,
}

impl FuzzyMatch {
    /// Create a new fuzzy match
    pub fn new(item: String, score: i32, positions: Vec<usize>) -> Self {
        Self {
            item,
            score,
            positions,
        }
    }
}

impl PartialOrd for FuzzyMatch {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FuzzyMatch {
    fn cmp(&self, other: &Self) -> Ordering {
        // Sort by score (descending), then by item (ascending)
        other
            .score
            .cmp(&self.score)
            .then_with(|| self.item.cmp(&other.item))
    }
}

/// Fuzzy matching strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MatchStrategy {
    /// Simple substring matching
    Substring,
    /// Fuzzy matching (characters in order)
    Fuzzy,
    /// Exact matching
    Exact,
}

/// Case sensitivity mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaseMode {
    /// Always case-sensitive
    Sensitive,
    /// Always case-insensitive
    Insensitive,
    /// Smart: case-insensitive unless query has uppercase
    Smart,
}

impl CaseMode {
    /// Determine if search should be case-sensitive
    pub fn is_sensitive(&self, query: &str) -> bool {
        match self {
            CaseMode::Sensitive => true,
            CaseMode::Insensitive => false,
            CaseMode::Smart => query.chars().any(|c| c.is_uppercase()),
        }
    }
}

/// Fuzzy matcher for scoring items
pub struct FuzzyMatcher {
    strategy: MatchStrategy,
    case_mode: CaseMode,
}

impl FuzzyMatcher {
    /// Create a new fuzzy matcher
    pub fn new(strategy: MatchStrategy, case_mode: CaseMode) -> Self {
        Self {
            strategy,
            case_mode,
        }
    }

    /// Create with default settings (Fuzzy + Smart case)
    pub fn default() -> Self {
        Self::new(MatchStrategy::Fuzzy, CaseMode::Smart)
    }

    /// Match an item against a query
    pub fn match_item(&self, query: &str, item: &str) -> Option<FuzzyMatch> {
        if query.is_empty() {
            return Some(FuzzyMatch::new(item.to_string(), 0, vec![]));
        }

        let case_sensitive = self.case_mode.is_sensitive(query);

        match self.strategy {
            MatchStrategy::Exact => self.exact_match(query, item, case_sensitive),
            MatchStrategy::Substring => self.substring_match(query, item, case_sensitive),
            MatchStrategy::Fuzzy => self.fuzzy_match(query, item, case_sensitive),
        }
    }

    /// Exact match
    fn exact_match(&self, query: &str, item: &str, case_sensitive: bool) -> Option<FuzzyMatch> {
        let matches = if case_sensitive {
            query == item
        } else {
            query.to_lowercase() == item.to_lowercase()
        };

        if matches {
            let positions: Vec<usize> = (0..query.len()).collect();
            Some(FuzzyMatch::new(item.to_string(), 1000, positions))
        } else {
            None
        }
    }

    /// Substring match
    fn substring_match(
        &self,
        query: &str,
        item: &str,
        case_sensitive: bool,
    ) -> Option<FuzzyMatch> {
        let (query_str, item_str) = if case_sensitive {
            (query.to_string(), item.to_string())
        } else {
            (query.to_lowercase(), item.to_lowercase())
        };

        if let Some(start) = item_str.find(&query_str) {
            let positions: Vec<usize> = (start..start + query.len()).collect();
            // Score based on position (earlier is better) and length
            let score = 500 - (start as i32) + (100 / (item.len() as i32 + 1));
            Some(FuzzyMatch::new(item.to_string(), score, positions))
        } else {
            None
        }
    }

    /// Fuzzy match (characters in order)
    fn fuzzy_match(&self, query: &str, item: &str, case_sensitive: bool) -> Option<FuzzyMatch> {
        let query_chars: Vec<char> = if case_sensitive {
            query.chars().collect()
        } else {
            query.to_lowercase().chars().collect()
        };

        let item_chars: Vec<char> = if case_sensitive {
            item.chars().collect()
        } else {
            item.to_lowercase().chars().collect()
        };

        let mut positions = Vec::new();
        let mut item_idx = 0;
        let mut consecutive = 0;
        let mut gap_penalty = 0;

        for query_char in &query_chars {
            // Find next occurrence of query_char in item
            let mut found = false;
            while item_idx < item_chars.len() {
                if item_chars[item_idx] == *query_char {
                    positions.push(item_idx);
                    item_idx += 1;
                    found = true;

                    // Bonus for consecutive matches
                    if positions.len() > 1
                        && positions[positions.len() - 1] == positions[positions.len() - 2] + 1
                    {
                        consecutive += 1;
                    } else {
                        consecutive = 0;
                    }
                    break;
                } else {
                    item_idx += 1;
                    gap_penalty += 1;
                }
            }

            if !found {
                return None;
            }
        }

        // Calculate score based on:
        // - Match length (longer queries = higher score)
        // - Start position (earlier = better)
        // - Consecutive matches (more = better)
        // - Gap penalty (fewer gaps = better)
        let start_pos = positions.first().copied().unwrap_or(0);
        let score = (query_chars.len() as i32 * 50)
            + (consecutive * 20)
            - (start_pos as i32)
            - (gap_penalty / 2);

        Some(FuzzyMatch::new(item.to_string(), score, positions))
    }

    /// Set matching strategy
    pub fn set_strategy(&mut self, strategy: MatchStrategy) {
        self.strategy = strategy;
    }

    /// Set case mode
    pub fn set_case_mode(&mut self, case_mode: CaseMode) {
        self.case_mode = case_mode;
    }
}

/// Fuzzy finder for managing search state
pub struct FuzzyFinder {
    /// Current search query
    query: String,
    /// All available items
    items: Vec<String>,
    /// Matched and ranked results
    matches: Vec<FuzzyMatch>,
    /// Matcher instance
    matcher: FuzzyMatcher,
    /// Selected match index
    selected: Option<usize>,
    /// Maximum results to keep
    max_results: usize,
}

impl FuzzyFinder {
    /// Create a new fuzzy finder
    pub fn new(items: Vec<String>) -> Self {
        Self {
            query: String::new(),
            items,
            matches: Vec::new(),
            matcher: FuzzyMatcher::default(),
            selected: None,
            max_results: 100,
        }
    }

    /// Set the search query and update matches (incremental search)
    pub fn set_query(&mut self, query: impl Into<String>) {
        self.query = query.into();
        self.update_matches();
    }

    /// Get current query
    pub fn query(&self) -> &str {
        &self.query
    }

    /// Update items list
    pub fn set_items(&mut self, items: Vec<String>) {
        self.items = items;
        self.update_matches();
    }

    /// Get all items
    pub fn items(&self) -> &[String] {
        &self.items
    }

    /// Get current matches
    pub fn matches(&self) -> &[FuzzyMatch] {
        &self.matches
    }

    /// Get selected match
    pub fn selected(&self) -> Option<&FuzzyMatch> {
        self.selected.and_then(|idx| self.matches.get(idx))
    }

    /// Get selected index
    pub fn selected_index(&self) -> Option<usize> {
        self.selected
    }

    /// Select next match
    pub fn select_next(&mut self) {
        if self.matches.is_empty() {
            return;
        }

        self.selected = Some(match self.selected {
            Some(idx) if idx + 1 < self.matches.len() => idx + 1,
            _ => 0,
        });
    }

    /// Select previous match
    pub fn select_previous(&mut self) {
        if self.matches.is_empty() {
            return;
        }

        self.selected = Some(match self.selected {
            Some(0) | None => self.matches.len() - 1,
            Some(idx) => idx - 1,
        });
    }

    /// Set max results
    pub fn set_max_results(&mut self, max: usize) {
        self.max_results = max;
        if self.matches.len() > max {
            self.matches.truncate(max);
        }
    }

    /// Set matching strategy
    pub fn set_strategy(&mut self, strategy: MatchStrategy) {
        self.matcher.set_strategy(strategy);
        self.update_matches();
    }

    /// Set case mode
    pub fn set_case_mode(&mut self, case_mode: CaseMode) {
        self.matcher.set_case_mode(case_mode);
        self.update_matches();
    }

    /// Clear query and matches
    pub fn clear(&mut self) {
        self.query.clear();
        self.matches.clear();
        self.selected = None;
    }

    /// Update matches based on current query
    fn update_matches(&mut self) {
        self.matches.clear();
        self.selected = None;

        for item in &self.items {
            if let Some(fuzzy_match) = self.matcher.match_item(&self.query, item) {
                self.matches.push(fuzzy_match);
            }
        }

        // Sort by score (descending)
        self.matches.sort();

        // Limit results
        if self.matches.len() > self.max_results {
            self.matches.truncate(self.max_results);
        }

        // Auto-select first match
        if !self.matches.is_empty() {
            self.selected = Some(0);
        }
    }

    /// Get match count
    pub fn match_count(&self) -> usize {
        self.matches.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.matches.is_empty()
    }
}

impl Default for FuzzyFinder {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuzzy_match_creation() {
        let m = FuzzyMatch::new("test".to_string(), 100, vec![0, 1, 2, 3]);
        assert_eq!(m.item, "test");
        assert_eq!(m.score, 100);
        assert_eq!(m.positions, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_fuzzy_match_ordering() {
        let m1 = FuzzyMatch::new("a".to_string(), 100, vec![]);
        let m2 = FuzzyMatch::new("b".to_string(), 200, vec![]);
        assert!(m2 < m1); // Higher score comes first
    }

    #[test]
    fn test_case_mode_smart() {
        let mode = CaseMode::Smart;
        assert!(!mode.is_sensitive("hello")); // All lowercase = insensitive
        assert!(mode.is_sensitive("Hello")); // Has uppercase = sensitive
    }

    #[test]
    fn test_exact_match() {
        let matcher = FuzzyMatcher::new(MatchStrategy::Exact, CaseMode::Sensitive);
        assert!(matcher.match_item("test", "test").is_some());
        assert!(matcher.match_item("test", "Test").is_none());
    }

    #[test]
    fn test_exact_match_case_insensitive() {
        let matcher = FuzzyMatcher::new(MatchStrategy::Exact, CaseMode::Insensitive);
        assert!(matcher.match_item("test", "TEST").is_some());
    }

    #[test]
    fn test_substring_match() {
        let matcher = FuzzyMatcher::new(MatchStrategy::Substring, CaseMode::Insensitive);
        let result = matcher.match_item("ell", "hello");
        assert!(result.is_some());
        let m = result.unwrap();
        assert_eq!(m.positions, vec![1, 2, 3]);
    }

    #[test]
    fn test_fuzzy_match() {
        let matcher = FuzzyMatcher::new(MatchStrategy::Fuzzy, CaseMode::Insensitive);
        let result = matcher.match_item("hlo", "hello");
        assert!(result.is_some());
        let m = result.unwrap();
        assert!(m.positions.contains(&0)); // 'h'
        assert!(m.positions.contains(&2)); // 'l'
        assert!(m.positions.contains(&4)); // 'o'
    }

    #[test]
    fn test_fuzzy_match_no_match() {
        let matcher = FuzzyMatcher::new(MatchStrategy::Fuzzy, CaseMode::Insensitive);
        assert!(matcher.match_item("xyz", "hello").is_none());
    }

    #[test]
    fn test_fuzzy_finder_creation() {
        let finder = FuzzyFinder::new(vec!["hello".to_string(), "world".to_string()]);
        assert_eq!(finder.items().len(), 2);
        assert_eq!(finder.match_count(), 0);
    }

    #[test]
    fn test_fuzzy_finder_search() {
        let mut finder = FuzzyFinder::new(vec![
            "hello".to_string(),
            "help".to_string(),
            "world".to_string(),
        ]);

        finder.set_query("hel");
        assert!(finder.match_count() > 0);
        assert!(finder
            .matches()
            .iter()
            .all(|m| m.item.starts_with("hel") || m.item.contains("hel")));
    }

    #[test]
    fn test_fuzzy_finder_incremental() {
        let mut finder = FuzzyFinder::new(vec![
            "test".to_string(),
            "testing".to_string(),
            "tester".to_string(),
        ]);

        finder.set_query("t");
        let count1 = finder.match_count();
        assert_eq!(count1, 3);

        finder.set_query("te");
        let count2 = finder.match_count();
        assert_eq!(count2, 3);

        finder.set_query("test");
        let count3 = finder.match_count();
        assert_eq!(count3, 3);
    }

    #[test]
    fn test_fuzzy_finder_navigation() {
        let mut finder = FuzzyFinder::new(vec![
            "apple".to_string(),
            "application".to_string(),
            "apply".to_string(),
        ]);

        finder.set_query("app");
        assert!(!finder.is_empty());

        let first = finder.selected().map(|m| m.item.clone());
        assert!(first.is_some());

        finder.select_next();
        let second = finder.selected().map(|m| m.item.clone());
        assert!(second.is_some());
        assert_ne!(first, second);

        finder.select_previous();
        assert_eq!(finder.selected().map(|m| m.item.clone()), first);
    }

    #[test]
    fn test_fuzzy_finder_max_results() {
        let items: Vec<String> = (0..200).map(|i| format!("item{}", i)).collect();
        let mut finder = FuzzyFinder::new(items);
        finder.set_max_results(50);

        finder.set_query("item");
        assert!(finder.match_count() <= 50);
    }

    #[test]
    fn test_fuzzy_finder_clear() {
        let mut finder = FuzzyFinder::new(vec!["test".to_string()]);
        finder.set_query("test");
        assert!(!finder.is_empty());

        finder.clear();
        assert_eq!(finder.query(), "");
        assert!(finder.is_empty());
    }

    #[test]
    fn test_fuzzy_match_consecutive_bonus() {
        let matcher = FuzzyMatcher::new(MatchStrategy::Fuzzy, CaseMode::Insensitive);

        // "test" in "test" should score higher than "test" in "t e s t"
        let m1 = matcher.match_item("test", "test").unwrap();
        let m2 = matcher.match_item("test", "t_e_s_t").unwrap();

        assert!(m1.score > m2.score);
    }

    #[test]
    fn test_strategy_switching() {
        let mut finder = FuzzyFinder::new(vec!["hello".to_string(), "help".to_string()]);

        finder.set_strategy(MatchStrategy::Exact);
        finder.set_query("hello");
        assert_eq!(finder.match_count(), 1);

        finder.set_strategy(MatchStrategy::Substring);
        finder.set_query("hel");
        assert_eq!(finder.match_count(), 2);
    }
}
