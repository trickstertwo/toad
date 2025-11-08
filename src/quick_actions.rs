/// Quick actions panel for frequently used commands
///
/// Provides a quick access panel for common operations with keyboard shortcuts,
/// usage tracking, and context-aware suggestions
///
/// # Examples
///
/// ```
/// use toad::quick_actions::{QuickAction, QuickActionManager};
///
/// let mut manager = QuickActionManager::new();
/// manager.add_action(QuickAction::new("save", "Save file", "Ctrl+S"));
/// ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// A single quick action
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuickAction {
    /// Unique ID for the action
    pub id: String,
    /// Display label
    pub label: String,
    /// Optional keyboard shortcut
    pub shortcut: Option<String>,
    /// Action category
    pub category: ActionCategory,
    /// Optional icon (Nerd Font character)
    pub icon: Option<String>,
    /// Whether this action is currently enabled
    pub enabled: bool,
    /// Usage count for prioritization
    #[serde(default)]
    pub usage_count: usize,
    /// Last used timestamp
    #[serde(default)]
    pub last_used: u64,
}

/// Category for grouping actions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActionCategory {
    /// File operations
    File,
    /// Edit operations
    Edit,
    /// Search/navigation
    Search,
    /// Git operations
    Git,
    /// View/UI operations
    View,
    /// Custom/other
    Custom,
}

impl QuickAction {
    /// Create a new quick action
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::quick_actions::QuickAction;
    ///
    /// let action = QuickAction::new("save", "Save file", "Ctrl+S");
    /// assert_eq!(action.id, "save");
    /// ```
    pub fn new(id: impl Into<String>, label: impl Into<String>, shortcut: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            shortcut: Some(shortcut.into()),
            category: ActionCategory::Custom,
            icon: None,
            enabled: true,
            usage_count: 0,
            last_used: 0,
        }
    }

    /// Set category
    pub fn with_category(mut self, category: ActionCategory) -> Self {
        self.category = category;
        self
    }

    /// Set icon
    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set enabled state
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Record usage
    pub fn record_usage(&mut self) {
        self.usage_count += 1;
        self.last_used = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }

    /// Get recency score (0.0 to 1.0, higher is more recent)
    pub fn recency_score(&self) -> f64 {
        if self.last_used == 0 {
            return 0.0;
        }

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let age_seconds = now.saturating_sub(self.last_used);

        // Decay over 7 days
        let decay_seconds = 7 * 24 * 60 * 60;
        1.0 - (age_seconds as f64 / decay_seconds as f64).min(1.0)
    }

    /// Get frequency score (0.0 to 1.0, higher is more frequent)
    pub fn frequency_score(&self, max_usage: usize) -> f64 {
        if max_usage == 0 {
            return 0.0;
        }
        (self.usage_count as f64 / max_usage as f64).min(1.0)
    }

    /// Get combined priority score
    pub fn priority_score(&self, max_usage: usize) -> f64 {
        // Weight: 60% frequency, 40% recency
        self.frequency_score(max_usage) * 0.6 + self.recency_score() * 0.4
    }
}

/// Manager for quick actions
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QuickActionManager {
    /// Actions by ID
    actions: HashMap<String, QuickAction>,
    /// Maximum actions to show
    max_visible: usize,
}

impl QuickActionManager {
    /// Create a new quick action manager
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::quick_actions::QuickActionManager;
    ///
    /// let manager = QuickActionManager::new();
    /// assert_eq!(manager.count(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            actions: HashMap::new(),
            max_visible: 10,
        }
    }

    /// Create manager with common defaults
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::quick_actions::QuickActionManager;
    ///
    /// let manager = QuickActionManager::with_defaults();
    /// assert!(manager.count() > 0);
    /// ```
    pub fn with_defaults() -> Self {
        let mut manager = Self::new();

        // File actions
        manager.add_action(
            QuickAction::new("save", "Save", "Ctrl+S")
                .with_category(ActionCategory::File)
                .with_icon(""),
        );
        manager.add_action(
            QuickAction::new("open", "Open File", "Ctrl+O")
                .with_category(ActionCategory::File)
                .with_icon(""),
        );
        manager.add_action(
            QuickAction::new("close", "Close", "Ctrl+W")
                .with_category(ActionCategory::File)
                .with_icon(""),
        );

        // Edit actions
        manager.add_action(
            QuickAction::new("undo", "Undo", "Ctrl+Z")
                .with_category(ActionCategory::Edit)
                .with_icon(""),
        );
        manager.add_action(
            QuickAction::new("redo", "Redo", "Ctrl+Y")
                .with_category(ActionCategory::Edit)
                .with_icon(""),
        );

        // Search actions
        manager.add_action(
            QuickAction::new("find", "Find", "Ctrl+F")
                .with_category(ActionCategory::Search)
                .with_icon(""),
        );
        manager.add_action(
            QuickAction::new("goto", "Go to Line", "Ctrl+G")
                .with_category(ActionCategory::Search)
                .with_icon(""),
        );

        // Git actions
        manager.add_action(
            QuickAction::new("git_status", "Git Status", "")
                .with_category(ActionCategory::Git)
                .with_icon(""),
        );
        manager.add_action(
            QuickAction::new("git_commit", "Git Commit", "")
                .with_category(ActionCategory::Git)
                .with_icon(""),
        );

        manager
    }

    /// Add an action
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::quick_actions::{QuickAction, QuickActionManager};
    ///
    /// let mut manager = QuickActionManager::new();
    /// manager.add_action(QuickAction::new("test", "Test", ""));
    /// assert_eq!(manager.count(), 1);
    /// ```
    pub fn add_action(&mut self, action: QuickAction) {
        self.actions.insert(action.id.clone(), action);
    }

    /// Remove an action
    pub fn remove_action(&mut self, id: &str) -> Option<QuickAction> {
        self.actions.remove(id)
    }

    /// Get an action
    pub fn get_action(&self, id: &str) -> Option<&QuickAction> {
        self.actions.get(id)
    }

    /// Get mutable action
    pub fn get_action_mut(&mut self, id: &str) -> Option<&mut QuickAction> {
        self.actions.get_mut(id)
    }

    /// Record usage of an action
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::quick_actions::{QuickAction, QuickActionManager};
    ///
    /// let mut manager = QuickActionManager::new();
    /// manager.add_action(QuickAction::new("save", "Save", "Ctrl+S"));
    /// manager.record_usage("save");
    /// assert_eq!(manager.get_action("save").unwrap().usage_count, 1);
    /// ```
    pub fn record_usage(&mut self, id: &str) {
        if let Some(action) = self.actions.get_mut(id) {
            action.record_usage();
        }
    }

    /// Get all actions
    pub fn all_actions(&self) -> Vec<&QuickAction> {
        self.actions.values().filter(|a| a.enabled).collect()
    }

    /// Get actions by category
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::quick_actions::{ActionCategory, QuickActionManager};
    ///
    /// let manager = QuickActionManager::with_defaults();
    /// let file_actions = manager.by_category(ActionCategory::File);
    /// assert!(!file_actions.is_empty());
    /// ```
    pub fn by_category(&self, category: ActionCategory) -> Vec<&QuickAction> {
        self.actions
            .values()
            .filter(|a| a.enabled && a.category == category)
            .collect()
    }

    /// Get most frequently used actions
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::quick_actions::{QuickAction, QuickActionManager};
    ///
    /// let mut manager = QuickActionManager::new();
    /// manager.add_action(QuickAction::new("a", "A", ""));
    /// manager.record_usage("a");
    /// manager.record_usage("a");
    ///
    /// let frequent = manager.most_frequent(5);
    /// assert!(!frequent.is_empty());
    /// ```
    pub fn most_frequent(&self, limit: usize) -> Vec<&QuickAction> {
        let mut actions: Vec<_> = self.actions.values().filter(|a| a.enabled).collect();
        actions.sort_by(|a, b| b.usage_count.cmp(&a.usage_count));
        actions.into_iter().take(limit).collect()
    }

    /// Get recently used actions
    pub fn recently_used(&self, limit: usize) -> Vec<&QuickAction> {
        let mut actions: Vec<_> = self
            .actions
            .values()
            .filter(|a| a.enabled && a.last_used > 0)
            .collect();
        actions.sort_by(|a, b| b.last_used.cmp(&a.last_used));
        actions.into_iter().take(limit).collect()
    }

    /// Get suggested actions (prioritized by frequency and recency)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::quick_actions::QuickActionManager;
    ///
    /// let manager = QuickActionManager::with_defaults();
    /// let suggested = manager.suggested();
    /// assert!(!suggested.is_empty());
    /// ```
    pub fn suggested(&self) -> Vec<&QuickAction> {
        let max_usage = self.actions.values().map(|a| a.usage_count).max().unwrap_or(1);

        let mut actions: Vec<_> = self.actions.values().filter(|a| a.enabled).collect();
        actions.sort_by(|a, b| {
            b.priority_score(max_usage)
                .partial_cmp(&a.priority_score(max_usage))
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        actions.into_iter().take(self.max_visible).collect()
    }

    /// Search actions by label or ID
    pub fn search(&self, query: &str) -> Vec<&QuickAction> {
        let query_lower = query.to_lowercase();
        self.actions
            .values()
            .filter(|a| {
                a.enabled
                    && (a.id.to_lowercase().contains(&query_lower)
                        || a.label.to_lowercase().contains(&query_lower))
            })
            .collect()
    }

    /// Get count of actions
    pub fn count(&self) -> usize {
        self.actions.len()
    }

    /// Set maximum visible actions
    pub fn set_max_visible(&mut self, max: usize) {
        self.max_visible = max;
    }

    /// Clear all actions
    pub fn clear(&mut self) {
        self.actions.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_creation() {
        let action = QuickAction::new("save", "Save File", "Ctrl+S");
        assert_eq!(action.id, "save");
        assert_eq!(action.label, "Save File");
        assert_eq!(action.shortcut, Some("Ctrl+S".to_string()));
    }

    #[test]
    fn test_action_with_category() {
        let action = QuickAction::new("save", "Save", "Ctrl+S")
            .with_category(ActionCategory::File);
        assert_eq!(action.category, ActionCategory::File);
    }

    #[test]
    fn test_action_with_icon() {
        let action = QuickAction::new("save", "Save", "Ctrl+S")
            .with_icon("");
        assert_eq!(action.icon, Some("".to_string()));
    }

    #[test]
    fn test_action_record_usage() {
        let mut action = QuickAction::new("save", "Save", "Ctrl+S");
        assert_eq!(action.usage_count, 0);

        action.record_usage();
        assert_eq!(action.usage_count, 1);
        assert!(action.last_used > 0);
    }

    #[test]
    fn test_manager_creation() {
        let manager = QuickActionManager::new();
        assert_eq!(manager.count(), 0);
    }

    #[test]
    fn test_manager_with_defaults() {
        let manager = QuickActionManager::with_defaults();
        assert!(manager.count() > 0);
    }

    #[test]
    fn test_manager_add_action() {
        let mut manager = QuickActionManager::new();
        manager.add_action(QuickAction::new("test", "Test", ""));
        assert_eq!(manager.count(), 1);
    }

    #[test]
    fn test_manager_remove_action() {
        let mut manager = QuickActionManager::new();
        manager.add_action(QuickAction::new("test", "Test", ""));
        manager.remove_action("test");
        assert_eq!(manager.count(), 0);
    }

    #[test]
    fn test_manager_get_action() {
        let mut manager = QuickActionManager::new();
        manager.add_action(QuickAction::new("save", "Save", "Ctrl+S"));
        assert!(manager.get_action("save").is_some());
        assert!(manager.get_action("nonexistent").is_none());
    }

    #[test]
    fn test_manager_record_usage() {
        let mut manager = QuickActionManager::new();
        manager.add_action(QuickAction::new("save", "Save", "Ctrl+S"));
        manager.record_usage("save");
        assert_eq!(manager.get_action("save").unwrap().usage_count, 1);
    }

    #[test]
    fn test_manager_by_category() {
        let manager = QuickActionManager::with_defaults();
        let file_actions = manager.by_category(ActionCategory::File);
        assert!(!file_actions.is_empty());
    }

    #[test]
    fn test_manager_most_frequent() {
        let mut manager = QuickActionManager::new();
        manager.add_action(QuickAction::new("a", "A", ""));
        manager.add_action(QuickAction::new("b", "B", ""));

        manager.record_usage("a");
        manager.record_usage("a");
        manager.record_usage("b");

        let frequent = manager.most_frequent(5);
        assert_eq!(frequent[0].id, "a");
    }

    #[test]
    fn test_manager_recently_used() {
        let mut manager = QuickActionManager::new();
        manager.add_action(QuickAction::new("a", "A", ""));
        manager.record_usage("a");

        let recent = manager.recently_used(5);
        assert!(!recent.is_empty());
    }

    #[test]
    fn test_manager_suggested() {
        let manager = QuickActionManager::with_defaults();
        let suggested = manager.suggested();
        assert!(!suggested.is_empty());
    }

    #[test]
    fn test_manager_search() {
        let mut manager = QuickActionManager::new();
        manager.add_action(QuickAction::new("save", "Save File", ""));
        manager.add_action(QuickAction::new("open", "Open File", ""));

        let results = manager.search("save");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "save");
    }

    #[test]
    fn test_manager_clear() {
        let mut manager = QuickActionManager::with_defaults();
        manager.clear();
        assert_eq!(manager.count(), 0);
    }

    #[test]
    fn test_action_scores() {
        let mut action = QuickAction::new("test", "Test", "");
        action.record_usage();

        assert!(action.recency_score() > 0.0);
        assert!(action.frequency_score(1) > 0.0);
        assert!(action.priority_score(1) > 0.0);
    }

    #[test]
    fn test_action_enabled_disabled() {
        let mut manager = QuickActionManager::new();
        manager.add_action(QuickAction::new("test", "Test", "").with_enabled(false));

        let all = manager.all_actions();
        assert_eq!(all.len(), 0); // Disabled actions not included
    }

    #[test]
    fn test_action_categories() {
        // Test all category variants
        let _categories = [
            ActionCategory::File,
            ActionCategory::Edit,
            ActionCategory::Search,
            ActionCategory::Git,
            ActionCategory::View,
            ActionCategory::Custom,
        ];
    }
}
