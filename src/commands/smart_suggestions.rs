/// Smart context-aware suggestions system
///
/// Provides intelligent hints and suggestions based on context, user behavior,
/// and current state
///
/// # Examples
///
/// ```
/// use toad::smart_suggestions::{SmartSuggestions, SuggestionContext};
///
/// let mut suggestions = SmartSuggestions::new();
/// let ctx = SuggestionContext::default();
/// let hints = suggestions.get_suggestions(&ctx);
/// ```
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Context information for generating suggestions
#[derive(Debug, Clone, Default)]
pub struct SuggestionContext {
    /// Current file path
    pub current_file: Option<String>,
    /// File extension
    pub file_extension: Option<String>,
    /// Current mode (Normal, Insert, etc.)
    pub current_mode: Option<String>,
    /// Recent commands
    pub recent_commands: Vec<String>,
    /// Cursor position (line, column)
    pub cursor_position: Option<(usize, usize)>,
    /// Is file modified
    pub is_modified: bool,
    /// Is in git repository
    pub is_git_repo: bool,
    /// Has uncommitted changes
    pub has_git_changes: bool,
    /// Current search query
    pub search_query: Option<String>,
}

/// A single suggestion
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Suggestion {
    /// Suggestion text
    pub text: String,
    /// Suggestion type
    pub suggestion_type: SuggestionType,
    /// Relevance score (0.0 to 1.0)
    pub relevance: f64,
    /// Optional icon
    pub icon: Option<String>,
    /// Optional action to execute
    pub action: Option<String>,
}

/// Type of suggestion
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SuggestionType {
    /// Command suggestion
    Command,
    /// Shortcut hint
    Shortcut,
    /// Workflow tip
    Tip,
    /// Warning or caution
    Warning,
    /// Information
    Info,
    /// Next action recommendation
    NextAction,
}

impl Suggestion {
    /// Create a new suggestion
    pub fn new(text: impl Into<String>, suggestion_type: SuggestionType, relevance: f64) -> Self {
        Self {
            text: text.into(),
            suggestion_type,
            relevance: relevance.clamp(0.0, 1.0),
            icon: None,
            action: None,
        }
    }

    /// Set icon
    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set action
    pub fn with_action(mut self, action: impl Into<String>) -> Self {
        self.action = Some(action.into());
        self
    }
}

/// Rule for generating suggestions
#[derive(Debug, Clone)]
pub struct SuggestionRule {
    /// Rule ID
    pub id: String,
    /// Rule priority (higher = more important)
    pub priority: i32,
    /// Condition checker
    pub condition: fn(&SuggestionContext) -> bool,
    /// Suggestion generator
    pub generator: fn(&SuggestionContext) -> Vec<Suggestion>,
}

/// Smart suggestions engine
#[derive(Debug, Clone, Default)]
pub struct SmartSuggestions {
    /// Registered rules
    rules: Vec<SuggestionRule>,
    /// Custom hints
    custom_hints: HashMap<String, String>,
    /// Maximum suggestions to return
    max_suggestions: usize,
}

impl SmartSuggestions {
    /// Create a new smart suggestions engine
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            custom_hints: HashMap::new(),
            max_suggestions: 5,
        }
    }

    /// Create with default rules
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::smart_suggestions::SmartSuggestions;
    ///
    /// let suggestions = SmartSuggestions::with_defaults();
    /// ```
    pub fn with_defaults() -> Self {
        let mut engine = Self::new();

        // Unsaved changes reminder
        engine.add_rule(SuggestionRule {
            id: "unsaved_changes".to_string(),
            priority: 100,
            condition: |ctx| ctx.is_modified,
            generator: |_| {
                vec![
                    Suggestion::new(
                        "You have unsaved changes. Press Ctrl+S to save",
                        SuggestionType::Warning,
                        0.9,
                    )
                    .with_icon("")
                    .with_action("save"),
                ]
            },
        });

        // Git changes reminder
        engine.add_rule(SuggestionRule {
            id: "git_changes".to_string(),
            priority: 80,
            condition: |ctx| ctx.is_git_repo && ctx.has_git_changes,
            generator: |_| {
                vec![
                    Suggestion::new(
                        "You have uncommitted changes. Consider committing with :git commit",
                        SuggestionType::Info,
                        0.7,
                    )
                    .with_icon("")
                    .with_action("git_status"),
                ]
            },
        });

        // Rust file tips
        engine.add_rule(SuggestionRule {
            id: "rust_tips".to_string(),
            priority: 50,
            condition: |ctx| ctx.file_extension.as_deref() == Some("rs"),
            generator: |_| {
                vec![
                    Suggestion::new(
                        "Tip: Use cargo check for fast feedback",
                        SuggestionType::Tip,
                        0.5,
                    )
                    .with_icon(""),
                ]
            },
        });

        // Search continuation
        engine.add_rule(SuggestionRule {
            id: "search_next".to_string(),
            priority: 90,
            condition: |ctx| ctx.search_query.is_some(),
            generator: |_| {
                vec![
                    Suggestion::new(
                        "Press 'n' for next match, 'N' for previous",
                        SuggestionType::Shortcut,
                        0.8,
                    )
                    .with_icon(""),
                ]
            },
        });

        // Mode hints
        engine.add_rule(SuggestionRule {
            id: "insert_mode_hint".to_string(),
            priority: 60,
            condition: |ctx| ctx.current_mode.as_deref() == Some("Insert"),
            generator: |_| {
                vec![
                    Suggestion::new(
                        "Press Esc to return to Normal mode",
                        SuggestionType::Shortcut,
                        0.6,
                    )
                    .with_icon(""),
                ]
            },
        });

        engine
    }

    /// Add a suggestion rule
    pub fn add_rule(&mut self, rule: SuggestionRule) {
        self.rules.push(rule);
        // Keep sorted by priority (descending)
        self.rules.sort_by(|a, b| b.priority.cmp(&a.priority));
    }

    /// Add a custom hint for a file pattern
    pub fn add_custom_hint(&mut self, pattern: impl Into<String>, hint: impl Into<String>) {
        self.custom_hints.insert(pattern.into(), hint.into());
    }

    /// Get suggestions for the current context
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::smart_suggestions::{SmartSuggestions, SuggestionContext};
    ///
    /// let suggestions = SmartSuggestions::with_defaults();
    /// let ctx = SuggestionContext {
    ///     is_modified: true,
    ///     ..Default::default()
    /// };
    /// let hints = suggestions.get_suggestions(&ctx);
    /// assert!(!hints.is_empty());
    /// ```
    pub fn get_suggestions(&self, context: &SuggestionContext) -> Vec<Suggestion> {
        let mut suggestions = Vec::new();

        // Apply all matching rules
        for rule in &self.rules {
            if (rule.condition)(context) {
                suggestions.extend((rule.generator)(context));
            }
        }

        // Add custom hints
        if let Some(file) = &context.current_file {
            for (pattern, hint) in &self.custom_hints {
                if file.contains(pattern) {
                    suggestions.push(Suggestion::new(hint.clone(), SuggestionType::Tip, 0.5));
                }
            }
        }

        // Sort by relevance (descending)
        suggestions.sort_by(|a, b| {
            b.relevance
                .partial_cmp(&a.relevance)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // Limit to max suggestions
        suggestions.into_iter().take(self.max_suggestions).collect()
    }

    /// Get suggestions by type
    pub fn get_by_type(
        &self,
        context: &SuggestionContext,
        suggestion_type: SuggestionType,
    ) -> Vec<Suggestion> {
        self.get_suggestions(context)
            .into_iter()
            .filter(|s| s.suggestion_type == suggestion_type)
            .collect()
    }

    /// Set maximum suggestions
    pub fn set_max_suggestions(&mut self, max: usize) {
        self.max_suggestions = max;
    }

    /// Clear all rules
    pub fn clear_rules(&mut self) {
        self.rules.clear();
    }

    /// Clear custom hints
    pub fn clear_hints(&mut self) {
        self.custom_hints.clear();
    }

    /// Get number of rules
    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }
}

/// Helper for building suggestion contexts
pub struct ContextBuilder {
    context: SuggestionContext,
}

impl ContextBuilder {
    /// Create a new context builder
    pub fn new() -> Self {
        Self {
            context: SuggestionContext::default(),
        }
    }

    /// Set current file
    pub fn with_file(mut self, file: impl Into<String>) -> Self {
        let file_str = file.into();
        // Extract extension
        if let Some(ext) = file_str.split('.').last() {
            if ext != &file_str {
                self.context.file_extension = Some(ext.to_string());
            }
        }
        self.context.current_file = Some(file_str);
        self
    }

    /// Set current mode
    pub fn with_mode(mut self, mode: impl Into<String>) -> Self {
        self.context.current_mode = Some(mode.into());
        self
    }

    /// Add recent command
    pub fn with_command(mut self, cmd: impl Into<String>) -> Self {
        self.context.recent_commands.push(cmd.into());
        self
    }

    /// Set modified state
    pub fn modified(mut self, is_modified: bool) -> Self {
        self.context.is_modified = is_modified;
        self
    }

    /// Set git state
    pub fn git_repo(mut self, is_repo: bool, has_changes: bool) -> Self {
        self.context.is_git_repo = is_repo;
        self.context.has_git_changes = has_changes;
        self
    }

    /// Set search query
    pub fn searching(mut self, query: impl Into<String>) -> Self {
        self.context.search_query = Some(query.into());
        self
    }

    /// Build the context
    pub fn build(self) -> SuggestionContext {
        self.context
    }
}

impl Default for ContextBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suggestion_creation() {
        let suggestion = Suggestion::new("Test hint", SuggestionType::Tip, 0.8);
        assert_eq!(suggestion.text, "Test hint");
        assert_eq!(suggestion.suggestion_type, SuggestionType::Tip);
        assert_eq!(suggestion.relevance, 0.8);
    }

    #[test]
    fn test_suggestion_with_icon() {
        let suggestion = Suggestion::new("Test", SuggestionType::Tip, 0.5).with_icon("");
        assert_eq!(suggestion.icon, Some("".to_string()));
    }

    #[test]
    fn test_suggestion_with_action() {
        let suggestion = Suggestion::new("Test", SuggestionType::Command, 0.5).with_action("save");
        assert_eq!(suggestion.action, Some("save".to_string()));
    }

    #[test]
    fn test_smart_suggestions_creation() {
        let suggestions = SmartSuggestions::new();
        assert_eq!(suggestions.rule_count(), 0);
    }

    #[test]
    fn test_smart_suggestions_with_defaults() {
        let suggestions = SmartSuggestions::with_defaults();
        assert!(suggestions.rule_count() > 0);
    }

    #[test]
    fn test_unsaved_changes_suggestion() {
        let suggestions = SmartSuggestions::with_defaults();
        let ctx = SuggestionContext {
            is_modified: true,
            ..Default::default()
        };

        let hints = suggestions.get_suggestions(&ctx);
        assert!(!hints.is_empty());
        assert!(hints.iter().any(|s| s.text.contains("unsaved")));
    }

    #[test]
    fn test_git_changes_suggestion() {
        let suggestions = SmartSuggestions::with_defaults();
        let ctx = SuggestionContext {
            is_git_repo: true,
            has_git_changes: true,
            ..Default::default()
        };

        let hints = suggestions.get_suggestions(&ctx);
        assert!(hints.iter().any(|s| s.text.contains("uncommitted")));
    }

    #[test]
    fn test_rust_file_suggestion() {
        let suggestions = SmartSuggestions::with_defaults();
        let ctx = SuggestionContext {
            file_extension: Some("rs".to_string()),
            ..Default::default()
        };

        let hints = suggestions.get_suggestions(&ctx);
        assert!(hints.iter().any(|s| s.text.contains("cargo")));
    }

    #[test]
    fn test_search_suggestion() {
        let suggestions = SmartSuggestions::with_defaults();
        let ctx = SuggestionContext {
            search_query: Some("test".to_string()),
            ..Default::default()
        };

        let hints = suggestions.get_suggestions(&ctx);
        assert!(hints.iter().any(|s| s.text.contains("next match")));
    }

    #[test]
    fn test_custom_hint() {
        let mut suggestions = SmartSuggestions::new();
        suggestions.add_custom_hint("README", "Don't forget to update the README!");

        let ctx = SuggestionContext {
            current_file: Some("README.md".to_string()),
            ..Default::default()
        };

        let hints = suggestions.get_suggestions(&ctx);
        assert!(hints.iter().any(|s| s.text.contains("README")));
    }

    #[test]
    fn test_get_by_type() {
        let suggestions = SmartSuggestions::with_defaults();
        let ctx = SuggestionContext {
            is_modified: true,
            ..Default::default()
        };

        let warnings = suggestions.get_by_type(&ctx, SuggestionType::Warning);
        assert!(!warnings.is_empty());
    }

    #[test]
    fn test_context_builder() {
        let ctx = ContextBuilder::new()
            .with_file("test.rs")
            .with_mode("Normal")
            .modified(true)
            .build();

        assert_eq!(ctx.current_file, Some("test.rs".to_string()));
        assert_eq!(ctx.file_extension, Some("rs".to_string()));
        assert_eq!(ctx.current_mode, Some("Normal".to_string()));
        assert!(ctx.is_modified);
    }

    #[test]
    fn test_context_builder_git() {
        let ctx = ContextBuilder::new().git_repo(true, true).build();

        assert!(ctx.is_git_repo);
        assert!(ctx.has_git_changes);
    }

    #[test]
    fn test_max_suggestions() {
        let mut suggestions = SmartSuggestions::with_defaults();
        suggestions.set_max_suggestions(2);

        let ctx = SuggestionContext {
            is_modified: true,
            is_git_repo: true,
            has_git_changes: true,
            ..Default::default()
        };

        let hints = suggestions.get_suggestions(&ctx);
        assert!(hints.len() <= 2);
    }

    #[test]
    fn test_clear_rules() {
        let mut suggestions = SmartSuggestions::with_defaults();
        suggestions.clear_rules();
        assert_eq!(suggestions.rule_count(), 0);
    }

    #[test]
    fn test_suggestion_types() {
        // Test all type variants
        let _types = [
            SuggestionType::Command,
            SuggestionType::Shortcut,
            SuggestionType::Tip,
            SuggestionType::Warning,
            SuggestionType::Info,
            SuggestionType::NextAction,
        ];
    }

    #[test]
    fn test_relevance_clamping() {
        let suggestion1 = Suggestion::new("Test", SuggestionType::Tip, 1.5);
        assert_eq!(suggestion1.relevance, 1.0);

        let suggestion2 = Suggestion::new("Test", SuggestionType::Tip, -0.5);
        assert_eq!(suggestion2.relevance, 0.0);
    }

    #[test]
    fn test_suggestion_sorting() {
        let suggestions = SmartSuggestions::with_defaults();
        let ctx = SuggestionContext {
            is_modified: true,
            is_git_repo: true,
            has_git_changes: true,
            ..Default::default()
        };

        let hints = suggestions.get_suggestions(&ctx);
        // Should be sorted by relevance descending
        for i in 0..hints.len().saturating_sub(1) {
            assert!(hints[i].relevance >= hints[i + 1].relevance);
        }
    }
}
