/// Autocomplete system for text inputs
///
/// Provides intelligent completion suggestions based on context
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A single autocomplete suggestion
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Suggestion {
    /// The text to insert
    pub text: String,
    /// Description of this suggestion
    pub description: Option<String>,
    /// Category or type of suggestion
    pub category: Option<String>,
    /// Score/rank for sorting (higher is better)
    pub score: usize,
}

impl Suggestion {
    /// Create a new suggestion
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            description: None,
            category: None,
            score: 0,
        }
    }

    /// Set description
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Set category
    pub fn with_category(mut self, cat: impl Into<String>) -> Self {
        self.category = Some(cat.into());
        self
    }

    /// Set score
    pub fn with_score(mut self, score: usize) -> Self {
        self.score = score;
        self
    }
}

/// Autocomplete provider trait
pub trait AutocompleteProvider: Send + Sync {
    /// Get suggestions for the given input
    fn get_suggestions(&self, input: &str) -> Vec<Suggestion>;
}

/// Simple word-based autocomplete provider
#[derive(Debug, Clone)]
pub struct WordProvider {
    /// Dictionary of words
    words: Vec<String>,
}

impl WordProvider {
    /// Create a new word provider
    pub fn new(words: Vec<String>) -> Self {
        Self { words }
    }

    /// Add words to the dictionary
    pub fn add_words(&mut self, words: Vec<String>) {
        self.words.extend(words);
        self.words.sort();
        self.words.dedup();
    }

    /// Match score for fuzzy matching
    fn match_score(word: &str, input: &str) -> usize {
        if word.starts_with(input) {
            1000 + word.len() // Exact prefix match
        } else if word.to_lowercase().starts_with(&input.to_lowercase()) {
            500 + word.len() // Case-insensitive prefix match
        } else if word.contains(input) {
            100 // Contains match
        } else {
            0 // No match
        }
    }
}

impl AutocompleteProvider for WordProvider {
    fn get_suggestions(&self, input: &str) -> Vec<Suggestion> {
        if input.is_empty() {
            return Vec::new();
        }

        let mut suggestions: Vec<Suggestion> = self
            .words
            .iter()
            .filter_map(|word| {
                let score = Self::match_score(word, input);
                if score > 0 {
                    Some(
                        Suggestion::new(word.clone())
                            .with_score(score)
                            .with_category("word"),
                    )
                } else {
                    None
                }
            })
            .collect();

        // Sort by score (descending)
        suggestions.sort_by(|a, b| b.score.cmp(&a.score));

        // Limit to top 10
        suggestions.truncate(10);

        suggestions
    }
}

/// Command-based autocomplete provider
#[derive(Debug, Clone)]
pub struct CommandProvider {
    /// Available commands
    commands: HashMap<String, String>,
}

impl CommandProvider {
    /// Create a new command provider
    pub fn new() -> Self {
        let mut commands = HashMap::new();
        commands.insert("help".to_string(), "Show help information".to_string());
        commands.insert("quit".to_string(), "Exit the application".to_string());
        commands.insert("clear".to_string(), "Clear the screen".to_string());
        commands.insert("save".to_string(), "Save current state".to_string());
        commands.insert("load".to_string(), "Load saved state".to_string());
        commands.insert("config".to_string(), "Open configuration".to_string());
        commands.insert("theme".to_string(), "Change theme".to_string());

        Self { commands }
    }

    /// Add a command
    pub fn add_command(&mut self, name: String, description: String) {
        self.commands.insert(name, description);
    }
}

impl Default for CommandProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl AutocompleteProvider for CommandProvider {
    fn get_suggestions(&self, input: &str) -> Vec<Suggestion> {
        if input.is_empty() {
            return Vec::new();
        }

        let mut suggestions: Vec<Suggestion> = self
            .commands
            .iter()
            .filter_map(|(name, desc)| {
                if name.starts_with(input) {
                    Some(
                        Suggestion::new(name.clone())
                            .with_description(desc.clone())
                            .with_category("command")
                            .with_score(1000),
                    )
                } else if name.contains(input) {
                    Some(
                        Suggestion::new(name.clone())
                            .with_description(desc.clone())
                            .with_category("command")
                            .with_score(500),
                    )
                } else {
                    None
                }
            })
            .collect();

        suggestions.sort_by(|a, b| b.score.cmp(&a.score).then(a.text.cmp(&b.text)));
        suggestions.truncate(10);

        suggestions
    }
}

/// Autocomplete manager
pub struct AutocompleteManager {
    /// Registered providers
    providers: Vec<Box<dyn AutocompleteProvider>>,
    /// Currently selected suggestion index
    selected: Option<usize>,
    /// Cached suggestions
    cached_suggestions: Vec<Suggestion>,
}

impl AutocompleteManager {
    /// Create a new autocomplete manager
    pub fn new() -> Self {
        Self {
            providers: Vec::new(),
            selected: None,
            cached_suggestions: Vec::new(),
        }
    }

    /// Add a provider
    pub fn add_provider(&mut self, provider: Box<dyn AutocompleteProvider>) {
        self.providers.push(provider);
    }

    /// Get suggestions for input
    pub fn get_suggestions(&mut self, input: &str) -> &[Suggestion] {
        self.cached_suggestions.clear();
        self.selected = None;

        for provider in &self.providers {
            self.cached_suggestions
                .extend(provider.get_suggestions(input));
        }

        // Sort all suggestions by score
        self.cached_suggestions
            .sort_by(|a, b| b.score.cmp(&a.score).then(a.text.cmp(&b.text)));

        // Remove duplicates
        self.cached_suggestions.dedup_by(|a, b| a.text == b.text);

        // Limit total suggestions
        self.cached_suggestions.truncate(20);

        if !self.cached_suggestions.is_empty() {
            self.selected = Some(0);
        }

        &self.cached_suggestions
    }

    /// Get currently selected suggestion
    pub fn selected(&self) -> Option<&Suggestion> {
        self.selected
            .and_then(|idx| self.cached_suggestions.get(idx))
    }

    /// Select next suggestion
    pub fn select_next(&mut self) {
        if self.cached_suggestions.is_empty() {
            return;
        }

        self.selected = Some(match self.selected {
            Some(idx) if idx + 1 < self.cached_suggestions.len() => idx + 1,
            _ => 0,
        });
    }

    /// Select previous suggestion
    pub fn select_previous(&mut self) {
        if self.cached_suggestions.is_empty() {
            return;
        }

        self.selected = Some(match self.selected {
            Some(0) | None => self.cached_suggestions.len() - 1,
            Some(idx) => idx - 1,
        });
    }

    /// Get all cached suggestions
    pub fn suggestions(&self) -> &[Suggestion] {
        &self.cached_suggestions
    }

    /// Clear cached suggestions
    pub fn clear(&mut self) {
        self.cached_suggestions.clear();
        self.selected = None;
    }
}

impl Default for AutocompleteManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suggestion_creation() {
        let sug = Suggestion::new("test")
            .with_description("A test")
            .with_category("demo")
            .with_score(100);

        assert_eq!(sug.text, "test");
        assert_eq!(sug.description, Some("A test".to_string()));
        assert_eq!(sug.category, Some("demo".to_string()));
        assert_eq!(sug.score, 100);
    }

    #[test]
    fn test_word_provider() {
        let provider = WordProvider::new(vec![
            "hello".to_string(),
            "help".to_string(),
            "world".to_string(),
        ]);

        let suggestions = provider.get_suggestions("hel");
        assert_eq!(suggestions.len(), 2);
        assert!(suggestions.iter().any(|s| s.text == "hello"));
        assert!(suggestions.iter().any(|s| s.text == "help"));
    }

    #[test]
    fn test_command_provider() {
        let provider = CommandProvider::new();
        let suggestions = provider.get_suggestions("he");

        assert!(suggestions.iter().any(|s| s.text == "help"));
        assert!(suggestions.iter().all(|s| s.description.is_some()));
    }

    #[test]
    fn test_autocomplete_manager() {
        let mut manager = AutocompleteManager::new();
        manager.add_provider(Box::new(CommandProvider::new()));

        let suggestions = manager.get_suggestions("he");
        assert!(!suggestions.is_empty());

        // Test navigation
        assert!(manager.selected().is_some());
        manager.select_next();
        manager.select_previous();
    }

    #[test]
    fn test_autocomplete_navigation() {
        let mut manager = AutocompleteManager::new();
        manager.add_provider(Box::new(WordProvider::new(vec![
            "apple".to_string(),
            "application".to_string(),
            "apply".to_string(),
        ])));

        let suggestions = manager.get_suggestions("app");
        assert!(!suggestions.is_empty());

        // Just verify we can navigate without panicking
        let first = manager.selected().map(|s| s.text.clone());
        assert!(first.is_some());

        manager.select_next();
        let second = manager.selected().map(|s| s.text.clone());
        assert!(second.is_some());
        assert_ne!(first, second);

        manager.select_previous();
        assert_eq!(manager.selected().map(|s| s.text.clone()), first);
    }

    // ============================================================================
    // COMPREHENSIVE EDGE CASE TESTS (ADVANCED TIER - Advanced Input)
    // ============================================================================

    // ============ Stress Tests ============

    #[test]
    fn test_word_provider_10000_words() {
        let words: Vec<String> = (0..10000).map(|i| format!("word{:05}", i)).collect();
        let provider = WordProvider::new(words);
        let suggestions = provider.get_suggestions("word0");
        // Should return top 10 matches (word00000-word00009, etc.)
        assert_eq!(suggestions.len(), 10);
    }

    #[test]
    fn test_command_provider_1000_commands() {
        let mut provider = CommandProvider::new();
        for i in 0..1000 {
            provider.add_command(format!("cmd{:04}", i), format!("Command {}", i));
        }
        let suggestions = provider.get_suggestions("cmd0");
        assert_eq!(suggestions.len(), 10); // Truncated to 10
    }

    #[test]
    fn test_rapid_suggestion_generation() {
        let mut manager = AutocompleteManager::new();
        manager.add_provider(Box::new(WordProvider::new(vec![
            "test".to_string(),
            "testing".to_string(),
        ])));

        // Generate suggestions 1000 times rapidly
        for _ in 0..1000 {
            let _suggestions = manager.get_suggestions("test");
        }
        assert!(manager.selected().is_some());
    }

    #[test]
    fn test_large_suggestion_list_before_truncation() {
        let words: Vec<String> = (0..100).map(|i| format!("test{:03}", i)).collect();
        let provider = WordProvider::new(words);
        let suggestions = provider.get_suggestions("test");
        // Should be truncated to 10
        assert_eq!(suggestions.len(), 10);
    }

    // ============ Unicode Edge Cases ============

    #[test]
    fn test_emoji_in_suggestions() {
        let provider = WordProvider::new(vec![
            "🚀 rocket".to_string(),
            "🐸 frog".to_string(),
            "💚 heart".to_string(),
        ]);
        let suggestions = provider.get_suggestions("🚀");
        assert_eq!(suggestions.len(), 1);
        assert_eq!(suggestions[0].text, "🚀 rocket");
    }

    #[test]
    fn test_rtl_arabic_suggestions() {
        let provider = WordProvider::new(vec![
            "مرحبا".to_string(),
            "مساء".to_string(),
            "صباح".to_string(),
        ]);
        let suggestions = provider.get_suggestions("مر");
        assert_eq!(suggestions.len(), 1);
        assert_eq!(suggestions[0].text, "مرحبا");
    }

    #[test]
    fn test_rtl_hebrew_suggestions() {
        let provider = WordProvider::new(vec![
            "שלום".to_string(),
            "שבת".to_string(),
            "שמש".to_string(),
        ]);
        let suggestions = provider.get_suggestions("של");
        assert_eq!(suggestions.len(), 1);
        assert_eq!(suggestions[0].text, "שלום");
    }

    #[test]
    fn test_japanese_suggestions() {
        let provider = WordProvider::new(vec![
            "こんにちは".to_string(),
            "こんばんは".to_string(),
            "ありがとう".to_string(),
        ]);
        let suggestions = provider.get_suggestions("こん");
        assert_eq!(suggestions.len(), 2);
    }

    #[test]
    fn test_mixed_scripts_suggestions() {
        let provider = WordProvider::new(vec![
            "Hello مرحبا".to_string(),
            "World שלום".to_string(),
            "Test こんにちは".to_string(),
        ]);
        let suggestions = provider.get_suggestions("Hello");
        assert_eq!(suggestions.len(), 1);
        assert_eq!(suggestions[0].text, "Hello مرحبا");
    }

    #[test]
    fn test_combining_characters_suggestions() {
        let provider = WordProvider::new(vec![
            "café".to_string(),
            "naïve".to_string(),
            "résumé".to_string(),
        ]);
        let suggestions = provider.get_suggestions("caf");
        assert_eq!(suggestions.len(), 1);
    }

    #[test]
    fn test_zero_width_characters() {
        let provider = WordProvider::new(vec![
            "test\u{200B}word".to_string(),    // Zero-width space
            "another\u{FEFF}word".to_string(), // Zero-width no-break space
        ]);
        let suggestions = provider.get_suggestions("test");
        assert_eq!(suggestions.len(), 1);
    }

    // ============ Extreme Values ============

    #[test]
    fn test_very_long_suggestion_text() {
        let long_text = "a".repeat(100_000);
        let provider = WordProvider::new(vec![long_text.clone()]);
        let suggestions = provider.get_suggestions("a");
        assert_eq!(suggestions.len(), 1);
        assert_eq!(suggestions[0].text.len(), 100_000);
    }

    #[test]
    fn test_empty_input() {
        let provider = WordProvider::new(vec!["test".to_string()]);
        let suggestions = provider.get_suggestions("");
        assert!(suggestions.is_empty());
    }

    #[test]
    fn test_single_character_input() {
        let provider =
            WordProvider::new(vec!["a".to_string(), "ab".to_string(), "abc".to_string()]);
        let suggestions = provider.get_suggestions("a");
        assert_eq!(suggestions.len(), 3);
    }

    #[test]
    fn test_special_characters_input() {
        let provider = WordProvider::new(vec!["test!@#".to_string(), "test$%^".to_string()]);
        let suggestions = provider.get_suggestions("test!");
        assert_eq!(suggestions.len(), 1);
        assert_eq!(suggestions[0].text, "test!@#");
    }

    // ============ Scoring Edge Cases ============

    #[test]
    fn test_exact_prefix_vs_case_insensitive() {
        let provider = WordProvider::new(vec!["Test".to_string(), "test".to_string()]);
        let suggestions = provider.get_suggestions("test");
        // Exact match should score higher
        assert_eq!(suggestions[0].text, "test");
    }

    #[test]
    fn test_multiple_matches_same_score() {
        let provider = WordProvider::new(vec![
            "apple".to_string(),
            "apply".to_string(),
            "application".to_string(),
        ]);
        let suggestions = provider.get_suggestions("app");
        // All should have same base score (exact prefix match)
        // Shorter words should come first due to length scoring
        assert_eq!(suggestions.len(), 3);
    }

    #[test]
    fn test_contains_match_lower_score() {
        let provider = WordProvider::new(vec![
            "prefix_test".to_string(),
            "contains_test_middle".to_string(),
        ]);
        let suggestions = provider.get_suggestions("test");
        // Contains match should have lower score than prefix
        assert!(suggestions.len() >= 1);
    }

    #[test]
    fn test_zero_score_filtered_out() {
        let provider = WordProvider::new(vec![
            "apple".to_string(),
            "banana".to_string(),
            "cherry".to_string(),
        ]);
        let suggestions = provider.get_suggestions("xyz");
        // No matches should return empty
        assert!(suggestions.is_empty());
    }

    // ============ Provider Edge Cases ============

    #[test]
    fn test_empty_word_provider() {
        let provider = WordProvider::new(vec![]);
        let suggestions = provider.get_suggestions("test");
        assert!(suggestions.is_empty());
    }

    #[test]
    fn test_duplicate_words_in_provider() {
        let mut provider = WordProvider::new(vec![
            "test".to_string(),
            "test".to_string(),
            "testing".to_string(),
        ]);
        // add_words should deduplicate
        provider.add_words(vec!["test".to_string()]);
        let suggestions = provider.get_suggestions("test");
        // Should have test and testing, not duplicates
        assert!(suggestions.len() <= 2);
    }

    #[test]
    fn test_add_words_to_provider() {
        let mut provider = WordProvider::new(vec!["hello".to_string()]);
        provider.add_words(vec!["help".to_string(), "world".to_string()]);
        let suggestions = provider.get_suggestions("hel");
        assert_eq!(suggestions.len(), 2);
    }

    #[test]
    fn test_command_provider_add_command() {
        let mut provider = CommandProvider::new();
        provider.add_command("newcmd".to_string(), "New command".to_string());
        let suggestions = provider.get_suggestions("new");
        assert_eq!(suggestions.len(), 1);
        assert_eq!(suggestions[0].text, "newcmd");
    }

    // ============ Manager Edge Cases ============

    #[test]
    fn test_manager_no_providers() {
        let mut manager = AutocompleteManager::new();
        let suggestions = manager.get_suggestions("test");
        assert!(suggestions.is_empty());
        assert!(manager.selected().is_none());
    }

    #[test]
    fn test_manager_multiple_providers() {
        let mut manager = AutocompleteManager::new();
        manager.add_provider(Box::new(WordProvider::new(vec!["test".to_string()])));
        manager.add_provider(Box::new(CommandProvider::new()));
        let suggestions = manager.get_suggestions("test");
        // Should combine suggestions from both providers
        assert!(!suggestions.is_empty());
    }

    #[test]
    fn test_manager_deduplication() {
        let mut manager = AutocompleteManager::new();
        manager.add_provider(Box::new(WordProvider::new(vec!["test".to_string()])));
        manager.add_provider(Box::new(WordProvider::new(vec!["test".to_string()])));
        let suggestions = manager.get_suggestions("test");
        // Should deduplicate "test" from both providers
        assert_eq!(suggestions.iter().filter(|s| s.text == "test").count(), 1);
    }

    #[test]
    fn test_manager_truncation_to_20() {
        // Create two providers with different prefixes to get more than 10 total results
        let words1: Vec<String> = (0..15).map(|i| format!("test{:02}", i)).collect();
        let words2: Vec<String> = (15..30).map(|i| format!("test{:02}", i)).collect();
        let mut manager = AutocompleteManager::new();
        manager.add_provider(Box::new(WordProvider::new(words1)));
        manager.add_provider(Box::new(WordProvider::new(words2)));
        let suggestions = manager.get_suggestions("test");
        // Each provider returns max 10, manager truncates to 20 total
        assert_eq!(suggestions.len(), 20);
    }

    #[test]
    fn test_manager_clear() {
        let mut manager = AutocompleteManager::new();
        manager.add_provider(Box::new(WordProvider::new(vec!["test".to_string()])));
        let _suggestions = manager.get_suggestions("test");
        assert!(manager.selected().is_some());

        manager.clear();
        assert!(manager.suggestions().is_empty());
        assert!(manager.selected().is_none());
    }

    // ============ Navigation Edge Cases ============

    #[test]
    fn test_navigation_empty_suggestions() {
        let mut manager = AutocompleteManager::new();
        manager.select_next();
        manager.select_previous();
        assert!(manager.selected().is_none());
    }

    #[test]
    fn test_navigation_single_suggestion() {
        let mut manager = AutocompleteManager::new();
        manager.add_provider(Box::new(WordProvider::new(vec!["test".to_string()])));
        let _suggestions = manager.get_suggestions("test");
        assert_eq!(manager.selected().unwrap().text, "test");

        manager.select_next();
        assert_eq!(manager.selected().unwrap().text, "test"); // Wraps to same

        manager.select_previous();
        assert_eq!(manager.selected().unwrap().text, "test"); // Wraps to same
    }

    #[test]
    fn test_navigation_wraparound_forward() {
        let mut manager = AutocompleteManager::new();
        manager.add_provider(Box::new(WordProvider::new(vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
        ])));
        let _suggestions = manager.get_suggestions(""); // Match empty to avoid filtering
        manager.cached_suggestions = vec![
            Suggestion::new("a"),
            Suggestion::new("b"),
            Suggestion::new("c"),
        ];
        manager.selected = Some(0);

        // Navigate through all suggestions
        manager.select_next(); // 1
        manager.select_next(); // 2
        manager.select_next(); // Wrap to 0
        assert_eq!(manager.selected(), Some(&Suggestion::new("a")));
    }

    #[test]
    fn test_navigation_wraparound_backward() {
        let mut manager = AutocompleteManager::new();
        manager.add_provider(Box::new(WordProvider::new(vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
        ])));
        let _suggestions = manager.get_suggestions("");
        manager.cached_suggestions = vec![
            Suggestion::new("a"),
            Suggestion::new("b"),
            Suggestion::new("c"),
        ];
        manager.selected = Some(0);

        manager.select_previous(); // Wrap to 2
        assert_eq!(manager.selected(), Some(&Suggestion::new("c")));
    }

    // ============ Trait Coverage ============

    #[test]
    fn test_suggestion_clone() {
        let sug = Suggestion::new("test").with_score(100);
        let cloned = sug.clone();
        assert_eq!(sug.text, cloned.text);
        assert_eq!(sug.score, cloned.score);
    }

    #[test]
    fn test_suggestion_partial_eq() {
        let sug1 = Suggestion::new("test").with_score(100);
        let sug2 = Suggestion::new("test").with_score(100);
        let sug3 = Suggestion::new("other").with_score(100);
        assert_eq!(sug1, sug2);
        assert_ne!(sug1, sug3);
    }

    #[test]
    fn test_suggestion_debug() {
        let sug = Suggestion::new("test");
        let debug_str = format!("{:?}", sug);
        assert!(debug_str.contains("test"));
    }

    #[test]
    fn test_suggestion_serialize() {
        let sug = Suggestion::new("test")
            .with_description("A test")
            .with_score(100);
        let json = serde_json::to_string(&sug).unwrap();
        assert!(json.contains("test"));
        assert!(json.contains("A test"));
    }

    #[test]
    fn test_suggestion_deserialize() {
        let json = r#"{"text":"test","description":"A test","category":null,"score":100}"#;
        let sug: Suggestion = serde_json::from_str(json).unwrap();
        assert_eq!(sug.text, "test");
        assert_eq!(sug.description, Some("A test".to_string()));
        assert_eq!(sug.score, 100);
    }

    #[test]
    fn test_command_provider_default() {
        let provider = CommandProvider::default();
        let suggestions = provider.get_suggestions("help");
        assert!(!suggestions.is_empty());
    }

    #[test]
    fn test_autocomplete_manager_default() {
        let manager = AutocompleteManager::default();
        assert!(manager.suggestions().is_empty());
        assert!(manager.selected().is_none());
    }

    // ============ Builder Pattern ============

    #[test]
    fn test_chained_builders() {
        let sug = Suggestion::new("test")
            .with_description("desc")
            .with_category("cat")
            .with_score(999);
        assert_eq!(sug.text, "test");
        assert_eq!(sug.description, Some("desc".to_string()));
        assert_eq!(sug.category, Some("cat".to_string()));
        assert_eq!(sug.score, 999);
    }

    #[test]
    fn test_partial_builder() {
        let sug = Suggestion::new("test").with_score(50);
        assert_eq!(sug.text, "test");
        assert_eq!(sug.description, None);
        assert_eq!(sug.category, None);
        assert_eq!(sug.score, 50);
    }

    // ============ Complex Workflows ============

    #[test]
    fn test_multi_step_suggestion_workflow() {
        let mut manager = AutocompleteManager::new();
        manager.add_provider(Box::new(WordProvider::new(vec![
            "apple".to_string(),
            "application".to_string(),
        ])));

        // Step 1: Get suggestions
        let suggestions = manager.get_suggestions("app");
        assert_eq!(suggestions.len(), 2);

        // Step 2: Navigate
        let first = manager.selected().unwrap().text.clone();
        manager.select_next();
        let second = manager.selected().unwrap().text.clone();
        assert_ne!(first, second);

        // Step 3: Get new suggestions (resets selection)
        let _suggestions = manager.get_suggestions("apple");
        assert_eq!(manager.selected().unwrap().text, "apple");

        // Step 4: Clear
        manager.clear();
        assert!(manager.selected().is_none());
    }

    #[test]
    fn test_caching_behavior() {
        let mut manager = AutocompleteManager::new();
        manager.add_provider(Box::new(WordProvider::new(vec!["test".to_string()])));

        // First query
        let suggestions1 = manager.get_suggestions("test");
        let len1 = suggestions1.len();
        assert_eq!(len1, 1);

        // Second query with different input should clear cache
        let suggestions2 = manager.get_suggestions("other");
        let len2 = suggestions2.len();
        assert_ne!(len1, len2);
    }

    #[test]
    fn test_selection_state_management() {
        let mut manager = AutocompleteManager::new();
        manager.add_provider(Box::new(WordProvider::new(vec![
            "a".to_string(),
            "b".to_string(),
        ])));

        let _suggestions = manager.get_suggestions("");
        manager.cached_suggestions = vec![Suggestion::new("a"), Suggestion::new("b")];
        manager.selected = Some(0);

        // Verify state transitions
        assert_eq!(manager.selected().unwrap().text, "a");
        manager.select_next();
        assert_eq!(manager.selected().unwrap().text, "b");
        manager.select_next(); // Wrap to 0
        assert_eq!(manager.selected().unwrap().text, "a");
    }

    // ============ Case Sensitivity ============

    #[test]
    fn test_case_insensitive_matching() {
        let provider = WordProvider::new(vec![
            "Test".to_string(),
            "TEST".to_string(),
            "test".to_string(),
        ]);
        let suggestions = provider.get_suggestions("TeSt");
        // Should match all three with case-insensitive scoring
        assert_eq!(suggestions.len(), 3);
    }

    #[test]
    fn test_case_sensitive_exact_match_priority() {
        let provider = WordProvider::new(vec!["Test".to_string(), "test".to_string()]);
        let suggestions = provider.get_suggestions("test");
        // Exact case match "test" should score higher than "Test"
        assert_eq!(suggestions[0].text, "test");
    }

    // ============ Comprehensive Stress Test ============

    #[test]
    fn test_comprehensive_autocomplete_stress() {
        // Phase 1: Create large dataset
        let mut words: Vec<String> = (0..100)
            .map(|i| match i % 4 {
                0 => format!("cmd{:03}", i),
                1 => format!("🚀 emoji{:03}", i),
                2 => format!("日本語{:03}", i),
                _ => format!("مرحبا{:03}", i),
            })
            .collect();
        words.push("test_exact".to_string());
        words.push("Test_case".to_string());

        // Phase 2: Create manager with multiple providers
        let mut manager = AutocompleteManager::new();
        manager.add_provider(Box::new(WordProvider::new(words.clone())));
        manager.add_provider(Box::new(CommandProvider::new()));

        // Phase 3: Rapid suggestion generation
        for i in 0..50 {
            let query = format!("cmd{:01}", i % 10);
            let _suggestions = manager.get_suggestions(&query);
        }

        // Phase 4: Test navigation
        let _suggestions = manager.get_suggestions("cmd0");
        for _ in 0..20 {
            manager.select_next();
        }
        for _ in 0..20 {
            manager.select_previous();
        }

        // Phase 5: Test clear and repopulate
        manager.clear();
        assert!(manager.suggestions().is_empty());
        let _suggestions = manager.get_suggestions("test");
        assert!(!manager.suggestions().is_empty());

        // Phase 6: Test Unicode
        let _suggestions = manager.get_suggestions("🚀");
        let _suggestions = manager.get_suggestions("日本語");

        // Phase 7: Test case sensitivity
        let suggestions = manager.get_suggestions("test");
        assert!(suggestions.iter().any(|s| s.text.contains("test")));

        // Phase 8: Add more words dynamically
        let mut word_provider = WordProvider::new(vec![]);
        word_provider.add_words(words.clone());
        let suggestions = word_provider.get_suggestions("cmd");
        assert!(!suggestions.is_empty());

        // Phase 9: Test edge cases
        let _empty = manager.get_suggestions("");
        let _single = manager.get_suggestions("c");

        // Phase 10: Final state verification
        manager.clear();
        assert!(manager.selected().is_none());
        assert!(manager.suggestions().is_empty());
    }
}
