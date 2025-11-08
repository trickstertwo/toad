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
}
