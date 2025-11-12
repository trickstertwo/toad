/// Tab system for managing multiple workspaces
///
/// Provides a tab-based interface for organizing different views or workspaces
use serde::{Deserialize, Serialize};
use std::fmt;

/// Unique tab identifier
pub type TabId = usize;

/// A single tab in the tab system
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tab {
    /// Unique ID
    pub id: TabId,
    /// Display title
    pub title: String,
    /// Optional icon
    pub icon: Option<String>,
    /// Whether this tab can be closed
    pub closable: bool,
    /// Whether this tab has unsaved changes (indicator: ●)
    pub modified: bool,
    /// Whether this tab has an active operation (indicator: *)
    #[serde(default)]
    pub has_operation: bool,
    /// Whether this tab has an error (indicator: !)
    #[serde(default)]
    pub has_error: bool,
}

impl Tab {
    /// Create a new tab
    pub fn new(id: TabId, title: impl Into<String>) -> Self {
        Self {
            id,
            title: title.into(),
            icon: None,
            closable: true,
            modified: false,
            has_operation: false,
            has_error: false,
        }
    }

    /// Set icon
    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set closable
    pub fn with_closable(mut self, closable: bool) -> Self {
        self.closable = closable;
        self
    }

    /// Set modified state
    pub fn set_modified(&mut self, modified: bool) {
        self.modified = modified;
    }

    /// Set operation state
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::workspace::Tab;
    ///
    /// let mut tab = Tab::new(0, "Active Tab");
    /// tab.set_operation(true);
    /// assert!(tab.has_operation);
    /// ```
    pub fn set_operation(&mut self, has_operation: bool) {
        self.has_operation = has_operation;
    }

    /// Set error state
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::workspace::Tab;
    ///
    /// let mut tab = Tab::new(0, "Error Tab");
    /// tab.set_error(true);
    /// assert!(tab.has_error);
    /// ```
    pub fn set_error(&mut self, has_error: bool) {
        self.has_error = has_error;
    }

    /// Get display name (with modification indicator)
    pub fn display_name(&self) -> String {
        if self.modified {
            format!("{}*", self.title)
        } else {
            self.title.clone()
        }
    }

    /// Get full display name with all indicators (●, *, !)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::workspace::Tab;
    ///
    /// let mut tab = Tab::new(0, "My Tab");
    /// tab.set_modified(true);
    /// tab.set_operation(true);
    /// tab.set_error(true);
    /// assert_eq!(tab.display_name_with_indicators(), "My Tab●*!");
    /// ```
    pub fn display_name_with_indicators(&self) -> String {
        let mut name = self.title.clone();
        if self.modified {
            name.push('●');
        }
        if self.has_operation {
            name.push('*');
        }
        if self.has_error {
            name.push('!');
        }
        name
    }
}

impl fmt::Display for Tab {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

/// Maximum number of tabs allowed
pub const MAX_TABS: usize = 10;

/// Tab manager for organizing multiple tabs
#[derive(Debug, Clone)]
pub struct TabManager {
    /// All tabs
    tabs: Vec<Tab>,
    /// Currently active tab index
    active: Option<usize>,
    /// Next tab ID to assign
    next_id: TabId,
}

impl TabManager {
    /// Create a new tab manager
    pub fn new() -> Self {
        Self {
            tabs: Vec::new(),
            active: None,
            next_id: 0,
        }
    }

    /// Create with an initial tab
    pub fn with_tab(title: impl Into<String>) -> Self {
        let mut manager = Self::new();
        manager.add_tab(title).unwrap();
        manager
    }

    /// Add a new tab
    ///
    /// Returns `None` if the maximum number of tabs (10) is reached.
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::workspace::TabManager;
    ///
    /// let mut manager = TabManager::new();
    /// let id = manager.add_tab("Main");
    /// assert!(id.is_some());
    /// ```
    pub fn add_tab(&mut self, title: impl Into<String>) -> Option<TabId> {
        // Check max tabs limit
        if self.tabs.len() >= MAX_TABS {
            return None;
        }

        let id = self.next_id;
        self.next_id += 1;

        let tab = Tab::new(id, title);
        self.tabs.push(tab);

        // Auto-activate if this is the first tab
        if self.tabs.len() == 1 {
            self.active = Some(0);
        }

        Some(id)
    }

    /// Add a tab with custom configuration
    ///
    /// Returns `None` if the maximum number of tabs (10) is reached.
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::workspace::{TabManager, Tab};
    ///
    /// let mut manager = TabManager::new();
    /// let tab = Tab::new(0, "Custom").with_icon("🔧");
    /// let id = manager.add_tab_with(tab);
    /// assert!(id.is_some());
    /// ```
    pub fn add_tab_with(&mut self, tab: Tab) -> Option<TabId> {
        // Check max tabs limit
        if self.tabs.len() >= MAX_TABS {
            return None;
        }

        let id = tab.id;
        self.tabs.push(tab);

        if self.tabs.len() == 1 {
            self.active = Some(0);
        }

        // Update next_id to avoid conflicts
        if id >= self.next_id {
            self.next_id = id + 1;
        }

        Some(id)
    }

    /// Close a tab by ID
    pub fn close_tab(&mut self, id: TabId) -> Option<Tab> {
        if let Some(idx) = self.tabs.iter().position(|t| t.id == id) {
            let tab = self.tabs.remove(idx);

            // Adjust active index
            if let Some(active_idx) = self.active {
                if active_idx == idx {
                    // Closing active tab
                    if self.tabs.is_empty() {
                        self.active = None;
                    } else if active_idx >= self.tabs.len() {
                        self.active = Some(self.tabs.len() - 1);
                    }
                } else if active_idx > idx {
                    self.active = Some(active_idx - 1);
                }
            }

            Some(tab)
        } else {
            None
        }
    }

    /// Get tab by ID
    pub fn get_tab(&self, id: TabId) -> Option<&Tab> {
        self.tabs.iter().find(|t| t.id == id)
    }

    /// Get mutable tab by ID
    pub fn get_tab_mut(&mut self, id: TabId) -> Option<&mut Tab> {
        self.tabs.iter_mut().find(|t| t.id == id)
    }

    /// Get all tabs
    pub fn tabs(&self) -> &[Tab] {
        &self.tabs
    }

    /// Get active tab
    pub fn active_tab(&self) -> Option<&Tab> {
        self.active.and_then(|idx| self.tabs.get(idx))
    }

    /// Get active tab ID
    pub fn active_tab_id(&self) -> Option<TabId> {
        self.active_tab().map(|t| t.id)
    }

    /// Set active tab by ID
    pub fn set_active(&mut self, id: TabId) -> bool {
        if let Some(idx) = self.tabs.iter().position(|t| t.id == id) {
            self.active = Some(idx);
            true
        } else {
            false
        }
    }

    /// Switch to next tab
    pub fn next_tab(&mut self) {
        if self.tabs.is_empty() {
            return;
        }

        self.active = Some(match self.active {
            Some(idx) if idx + 1 < self.tabs.len() => idx + 1,
            _ => 0,
        });
    }

    /// Switch to previous tab
    pub fn previous_tab(&mut self) {
        if self.tabs.is_empty() {
            return;
        }

        self.active = Some(match self.active {
            Some(0) | None => self.tabs.len() - 1,
            Some(idx) => idx - 1,
        });
    }

    /// Switch to tab by index (0-based)
    pub fn switch_to_index(&mut self, index: usize) -> bool {
        if index < self.tabs.len() {
            self.active = Some(index);
            true
        } else {
            false
        }
    }

    /// Get tab count
    pub fn count(&self) -> usize {
        self.tabs.len()
    }

    /// Check if manager is empty
    pub fn is_empty(&self) -> bool {
        self.tabs.is_empty()
    }

    /// Get active tab index
    pub fn active_index(&self) -> Option<usize> {
        self.active
    }

    /// Check if at maximum tab capacity
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::workspace::TabManager;
    ///
    /// let mut manager = TabManager::new();
    /// assert!(!manager.at_max_tabs());
    /// ```
    pub fn at_max_tabs(&self) -> bool {
        self.tabs.len() >= MAX_TABS
    }

    /// Get remaining tab slots
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::workspace::TabManager;
    ///
    /// let manager = TabManager::new();
    /// assert_eq!(manager.remaining_slots(), 10);
    /// ```
    pub fn remaining_slots(&self) -> usize {
        MAX_TABS.saturating_sub(self.tabs.len())
    }
}

impl Default for TabManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tab_creation() {
        let tab = Tab::new(0, "Test Tab");
        assert_eq!(tab.id, 0);
        assert_eq!(tab.title, "Test Tab");
        assert!(tab.closable);
        assert!(!tab.modified);
    }

    #[test]
    fn test_tab_display_name() {
        let mut tab = Tab::new(0, "Document");
        assert_eq!(tab.display_name(), "Document");

        tab.set_modified(true);
        assert_eq!(tab.display_name(), "Document*");
    }

    #[test]
    fn test_tab_manager_add() {
        let mut manager = TabManager::new();
        assert_eq!(manager.count(), 0);

        let id1 = manager.add_tab("Tab 1").unwrap();
        assert_eq!(manager.count(), 1);
        assert_eq!(manager.active_tab_id(), Some(id1));

        let _id2 = manager.add_tab("Tab 2").unwrap();
        assert_eq!(manager.count(), 2);
        assert_eq!(manager.active_tab_id(), Some(id1)); // Active doesn't change
    }

    #[test]
    fn test_tab_manager_close() {
        let mut manager = TabManager::new();
        let _id1 = manager.add_tab("Tab 1").unwrap();
        let id2 = manager.add_tab("Tab 2").unwrap();
        let _id3 = manager.add_tab("Tab 3").unwrap();

        manager.set_active(id2);
        assert_eq!(manager.count(), 3);

        // Close active tab
        let closed = manager.close_tab(id2);
        assert!(closed.is_some());
        assert_eq!(manager.count(), 2);
        assert_ne!(manager.active_tab_id(), Some(id2));

        // Close non-existent tab
        let closed = manager.close_tab(999);
        assert!(closed.is_none());
    }

    #[test]
    fn test_tab_manager_navigation() {
        let mut manager = TabManager::new();
        manager.add_tab("Tab 1").unwrap();
        manager.add_tab("Tab 2").unwrap();
        manager.add_tab("Tab 3").unwrap();

        assert_eq!(manager.active_index(), Some(0));

        manager.next_tab();
        assert_eq!(manager.active_index(), Some(1));

        manager.next_tab();
        assert_eq!(manager.active_index(), Some(2));

        manager.next_tab(); // Wrap around
        assert_eq!(manager.active_index(), Some(0));

        manager.previous_tab(); // Wrap to end
        assert_eq!(manager.active_index(), Some(2));
    }

    #[test]
    fn test_tab_manager_switch_by_index() {
        let mut manager = TabManager::new();
        manager.add_tab("Tab 1").unwrap();
        manager.add_tab("Tab 2").unwrap();
        manager.add_tab("Tab 3").unwrap();

        assert!(manager.switch_to_index(2));
        assert_eq!(manager.active_index(), Some(2));

        assert!(!manager.switch_to_index(10));
        assert_eq!(manager.active_index(), Some(2)); // Unchanged
    }

    #[test]
    fn test_tab_manager_get_tab() {
        let mut manager = TabManager::new();
        let id = manager.add_tab("Test").unwrap();

        let tab = manager.get_tab(id);
        assert!(tab.is_some());
        assert_eq!(tab.unwrap().title, "Test");

        let tab_mut = manager.get_tab_mut(id);
        assert!(tab_mut.is_some());
        tab_mut.unwrap().set_modified(true);

        assert!(manager.get_tab(id).unwrap().modified);
    }

    #[test]
    fn test_tab_with_icon() {
        let tab = Tab::new(0, "File").with_icon("📄");
        assert_eq!(tab.icon, Some("📄".to_string()));
    }

    // ========================================
    // MEDIUM TIER EDGE CASE TESTS
    // ========================================

    // Boundary Conditions
    #[test]
    fn test_empty_tab_manager() {
        let manager = TabManager::new();
        assert_eq!(manager.count(), 0);
        assert!(manager.is_empty());
        assert!(manager.active_tab().is_none());
        assert!(manager.active_tab_id().is_none());
        assert!(manager.active_index().is_none());
    }

    #[test]
    fn test_empty_tab_manager_navigation() {
        let mut manager = TabManager::new();

        // Should not crash on empty manager
        manager.next_tab();
        assert!(manager.active_index().is_none());

        manager.previous_tab();
        assert!(manager.active_index().is_none());
    }

    #[test]
    fn test_single_tab_navigation() {
        let mut manager = TabManager::new();
        manager.add_tab("Only Tab").unwrap();

        assert_eq!(manager.active_index(), Some(0));

        // Next wraps to same tab
        manager.next_tab();
        assert_eq!(manager.active_index(), Some(0));

        // Previous wraps to same tab
        manager.previous_tab();
        assert_eq!(manager.active_index(), Some(0));
    }

    #[test]
    fn test_close_last_tab() {
        let mut manager = TabManager::new();
        let id = manager.add_tab("Last Tab").unwrap();

        assert_eq!(manager.count(), 1);
        assert!(manager.active_tab().is_some());

        manager.close_tab(id);
        assert_eq!(manager.count(), 0);
        assert!(manager.is_empty());
        assert!(manager.active_tab().is_none());
    }

    #[test]
    fn test_close_active_tab_with_multiple() {
        let mut manager = TabManager::new();
        let id1 = manager.add_tab("Tab 1").unwrap();
        let id2 = manager.add_tab("Tab 2").unwrap();
        let id3 = manager.add_tab("Tab 3").unwrap();

        manager.set_active(id2);
        assert_eq!(manager.active_tab_id(), Some(id2));

        manager.close_tab(id2);
        assert_eq!(manager.count(), 2);
        // Should move to tab 3 (next available)
        assert_ne!(manager.active_tab_id(), Some(id2));
        assert!(manager.active_tab().is_some());
    }

    #[test]
    fn test_close_last_tab_when_active() {
        let mut manager = TabManager::new();
        manager.add_tab("Tab 1").unwrap();
        manager.add_tab("Tab 2").unwrap();
        let id3 = manager.add_tab("Tab 3").unwrap();

        manager.set_active(id3);
        assert_eq!(manager.active_index(), Some(2));

        manager.close_tab(id3);
        assert_eq!(manager.count(), 2);
        // Should move to previous tab
        assert_eq!(manager.active_index(), Some(1));
    }

    // Unicode/Emoji Edge Cases
    #[test]
    fn test_tab_with_emoji_title() {
        let tab = Tab::new(0, "🐸 Frog Tab 🎉");
        assert_eq!(tab.title, "🐸 Frog Tab 🎉");
        assert_eq!(tab.display_name(), "🐸 Frog Tab 🎉");

        let tab2 = Tab::new(1, "👨‍💻 Coding 🌫");
        assert_eq!(tab2.title, "👨‍💻 Coding 🌫");
    }

    #[test]
    fn test_tab_with_unicode_title() {
        let tab = Tab::new(0, "日本語のタブ");
        assert_eq!(tab.title, "日本語のタブ");

        let tab2 = Tab::new(1, "Тест 中文 العربية");
        assert_eq!(tab2.title, "Тест 中文 العربية");
    }

    #[test]
    fn test_tab_emoji_icon() {
        let tab = Tab::new(0, "File").with_icon("📄🎨🔧");
        assert_eq!(tab.icon, Some("📄🎨🔧".to_string()));
    }

    #[test]
    fn test_tab_with_modified_emoji() {
        let mut tab = Tab::new(0, "🎉 Party");
        tab.set_modified(true);
        assert_eq!(tab.display_name(), "🎉 Party*");
    }

    // Extreme Values
    #[test]
    fn test_very_long_tab_title() {
        let long_title = "A".repeat(1000);
        let tab = Tab::new(0, &long_title);
        assert_eq!(tab.title.len(), 1000);
        assert_eq!(tab.display_name(), long_title);
    }

    #[test]
    fn test_very_long_tab_title_with_modified() {
        let long_title = "B".repeat(500);
        let mut tab = Tab::new(0, &long_title);
        tab.set_modified(true);
        let expected = format!("{}*", long_title);
        assert_eq!(tab.display_name(), expected);
        assert_eq!(tab.display_name().len(), 501);
    }

    #[test]
    fn test_many_tabs() {
        let mut manager = TabManager::new();

        // Add 100 tabs
        for i in 0..100 {
            manager.add_tab(format!("Tab {}", i));
        }

        assert_eq!(manager.count(), 100);
        assert!(manager.active_tab().is_some());

        // Navigate through all tabs
        for i in 0..100 {
            assert!(manager.switch_to_index(i));
            assert_eq!(manager.active_index(), Some(i));
        }
    }

    #[test]
    fn test_many_tabs_navigation_wrap() {
        let mut manager = TabManager::new();

        for i in 0..50 {
            manager.add_tab(format!("Tab {}", i));
        }

        // Start at first
        assert_eq!(manager.active_index(), Some(0));

        // Navigate to last using previous (should wrap)
        manager.previous_tab();
        assert_eq!(manager.active_index(), Some(49));

        // Navigate to first using next (should wrap)
        manager.next_tab();
        assert_eq!(manager.active_index(), Some(0));
    }

    #[test]
    fn test_rapid_tab_operations() {
        let mut manager = TabManager::new();

        // Rapidly add and close tabs
        for _ in 0..100 {
            let id = manager.add_tab("Temp").unwrap();
            manager.close_tab(id);
        }

        assert_eq!(manager.count(), 0);
    }

    #[test]
    fn test_rapid_navigation() {
        let mut manager = TabManager::new();

        for i in 0..10 {
            manager.add_tab(format!("Tab {}", i));
        }

        // Rapidly navigate next 100 times
        for _ in 0..100 {
            manager.next_tab();
        }

        // Should end at index 0 (100 % 10 = 0)
        assert_eq!(manager.active_index(), Some(0));

        // Rapidly navigate previous 100 times
        for _ in 0..100 {
            manager.previous_tab();
        }

        // Should end at index 0 again (wraps perfectly)
        assert_eq!(manager.active_index(), Some(0));
    }

    // Tab ID Management
    #[test]
    fn test_tab_id_increment() {
        let mut manager = TabManager::new();

        let id1 = manager.add_tab("Tab 1").unwrap();
        let id2 = manager.add_tab("Tab 2").unwrap();
        let id3 = manager.add_tab("Tab 3").unwrap();

        assert_eq!(id1, 0);
        assert_eq!(id2, 1);
        assert_eq!(id3, 2);
    }

    #[test]
    fn test_tab_id_after_close() {
        let mut manager = TabManager::new();

        let id1 = manager.add_tab("Tab 1").unwrap();
        let _id2 = manager.add_tab("Tab 2").unwrap();

        manager.close_tab(id1);

        // Next ID should still be 2 (IDs not reused)
        let id3 = manager.add_tab("Tab 3").unwrap();
        assert_eq!(id3, 2);
    }

    #[test]
    fn test_add_tab_with_custom_id() {
        let mut manager = TabManager::new();

        let custom_tab = Tab::new(100, "Custom ID Tab");
        manager.add_tab_with(custom_tab);

        assert!(manager.get_tab(100).is_some());

        // Next auto-generated ID should be > 100
        let next_id = manager.add_tab("Next Tab").unwrap();
        assert!(next_id > 100);
    }

    #[test]
    fn test_add_tab_with_conflicting_id() {
        let mut manager = TabManager::new();

        let tab1 = Tab::new(5, "Tab 5");
        manager.add_tab_with(tab1);

        let tab2 = Tab::new(5, "Another Tab 5");
        manager.add_tab_with(tab2);

        // Both should exist with same ID (allowed, but unusual)
        assert_eq!(manager.count(), 2);
    }

    // Modified State
    #[test]
    fn test_tab_modified_state_transitions() {
        let mut tab = Tab::new(0, "Document");

        assert!(!tab.modified);
        assert_eq!(tab.display_name(), "Document");

        tab.set_modified(true);
        assert!(tab.modified);
        assert_eq!(tab.display_name(), "Document*");

        tab.set_modified(false);
        assert!(!tab.modified);
        assert_eq!(tab.display_name(), "Document");

        // Rapid toggles
        for i in 0..100 {
            tab.set_modified(i % 2 == 0);
            assert_eq!(tab.modified, i % 2 == 0);
        }
    }

    #[test]
    fn test_modified_state_persists_through_operations() {
        let mut manager = TabManager::new();
        let id = manager.add_tab("Doc").unwrap();

        manager.get_tab_mut(id).unwrap().set_modified(true);

        // Navigate away and back
        manager.add_tab("Other").unwrap();
        manager.set_active(id);

        // Modified state should persist
        assert!(manager.get_tab(id).unwrap().modified);
    }

    // Closable State
    #[test]
    fn test_tab_closable_state() {
        let tab = Tab::new(0, "Normal").with_closable(true);
        assert!(tab.closable);

        let permanent = Tab::new(1, "Permanent").with_closable(false);
        assert!(!permanent.closable);
    }

    #[test]
    fn test_close_non_closable_tab() {
        let mut manager = TabManager::new();
        let permanent_tab = Tab::new(0, "Permanent").with_closable(false);
        manager.add_tab_with(permanent_tab);

        // Manager can still close it (enforcement is UI responsibility)
        let closed = manager.close_tab(0);
        assert!(closed.is_some());
        assert_eq!(manager.count(), 0);
    }

    // Edge Cases in Navigation
    #[test]
    fn test_set_active_nonexistent_tab() {
        let mut manager = TabManager::new();
        manager.add_tab("Tab 1").unwrap();

        assert!(!manager.set_active(999));
        assert_eq!(manager.active_tab_id(), Some(0)); // Unchanged
    }

    #[test]
    fn test_switch_to_invalid_index() {
        let mut manager = TabManager::new();
        manager.add_tab("Tab 1").unwrap();
        manager.add_tab("Tab 2").unwrap();

        assert!(!manager.switch_to_index(10));
        assert_eq!(manager.active_index(), Some(0)); // Unchanged

        assert!(!manager.switch_to_index(usize::MAX));
        assert_eq!(manager.active_index(), Some(0)); // Unchanged
    }

    #[test]
    fn test_get_nonexistent_tab() {
        let manager = TabManager::new();
        assert!(manager.get_tab(999).is_none());
    }

    #[test]
    fn test_close_nonexistent_tab() {
        let mut manager = TabManager::new();
        manager.add_tab("Tab 1").unwrap();

        let closed = manager.close_tab(999);
        assert!(closed.is_none());
        assert_eq!(manager.count(), 1); // Unchanged
    }

    // Builder Pattern
    #[test]
    fn test_tab_builder_pattern() {
        let tab = Tab::new(0, "Complete").with_icon("🎯").with_closable(false);

        assert_eq!(tab.id, 0);
        assert_eq!(tab.title, "Complete");
        assert_eq!(tab.icon, Some("🎯".to_string()));
        assert!(!tab.closable);
        assert!(!tab.modified);
    }

    #[test]
    fn test_tab_manager_with_tab() {
        let manager = TabManager::with_tab("Initial");
        assert_eq!(manager.count(), 1);
        assert!(manager.active_tab().is_some());
        assert_eq!(manager.active_tab().unwrap().title, "Initial");
    }

    // Trait Tests
    #[test]
    fn test_tab_clone() {
        let tab1 = Tab::new(0, "Original").with_icon("📄");
        let tab2 = tab1.clone();

        assert_eq!(tab1, tab2);
        assert_eq!(tab1.id, tab2.id);
        assert_eq!(tab1.title, tab2.title);
        assert_eq!(tab1.icon, tab2.icon);
    }

    #[test]
    fn test_tab_debug() {
        let tab = Tab::new(0, "Debug Test");
        let debug_str = format!("{:?}", tab);
        assert!(debug_str.contains("Tab"));
        assert!(debug_str.contains("Debug Test"));
    }

    #[test]
    fn test_tab_display() {
        let tab = Tab::new(0, "Display Test");
        let display_str = format!("{}", tab);
        assert_eq!(display_str, "Display Test");

        let mut modified_tab = Tab::new(1, "Modified");
        modified_tab.set_modified(true);
        let display_str2 = format!("{}", modified_tab);
        assert_eq!(display_str2, "Modified*");
    }

    #[test]
    fn test_tab_partial_eq() {
        let tab1 = Tab::new(0, "Same");
        let tab2 = Tab::new(0, "Same");
        let tab3 = Tab::new(1, "Different");

        assert_eq!(tab1, tab2);
        assert_ne!(tab1, tab3);
    }

    #[test]
    fn test_tab_manager_clone() {
        let mut manager1 = TabManager::new();
        manager1.add_tab("Tab 1");
        manager1.add_tab("Tab 2");

        let manager2 = manager1.clone();

        assert_eq!(manager1.count(), manager2.count());
        assert_eq!(manager1.active_index(), manager2.active_index());
    }

    #[test]
    fn test_tab_manager_debug() {
        let manager = TabManager::new();
        let debug_str = format!("{:?}", manager);
        assert!(debug_str.contains("TabManager"));
    }

    #[test]
    fn test_tab_manager_default() {
        let manager = TabManager::default();
        assert_eq!(manager.count(), 0);
        assert!(manager.is_empty());
    }

    // Serialization
    #[test]
    fn test_tab_serialization() {
        let tab = Tab::new(5, "Serialize Test").with_icon("🔥");

        let json = serde_json::to_string(&tab).unwrap();
        let deserialized: Tab = serde_json::from_str(&json).unwrap();

        assert_eq!(tab, deserialized);
    }

    #[test]
    fn test_tab_serialization_with_modified() {
        let mut tab = Tab::new(10, "Modified Doc");
        tab.set_modified(true);

        let json = serde_json::to_string(&tab).unwrap();
        let deserialized: Tab = serde_json::from_str(&json).unwrap();

        assert_eq!(tab.modified, deserialized.modified);
        assert_eq!(tab.id, deserialized.id);
    }

    // Empty/Null Cases
    #[test]
    fn test_tab_empty_title() {
        let tab = Tab::new(0, "");
        assert_eq!(tab.title, "");
        assert_eq!(tab.display_name(), "");

        let mut modified = Tab::new(1, "");
        modified.set_modified(true);
        assert_eq!(modified.display_name(), "*");
    }

    #[test]
    fn test_tab_empty_icon() {
        let tab = Tab::new(0, "No Icon").with_icon("");
        assert_eq!(tab.icon, Some("".to_string()));
    }

    // Complex Scenarios
    #[test]
    fn test_close_all_tabs_sequentially() {
        let mut manager = TabManager::new();
        let mut ids = Vec::new();

        for i in 0..10 {
            let id = manager.add_tab(format!("Tab {}", i));
            ids.push(id);
        }

        // Close all tabs from first to last
        for id in ids {
            manager.close_tab(id);
        }

        assert_eq!(manager.count(), 0);
        assert!(manager.is_empty());
        assert!(manager.active_tab().is_none());
    }

    #[test]
    fn test_close_tabs_in_reverse() {
        let mut manager = TabManager::new();
        let mut ids = Vec::new();

        for i in 0..10 {
            let id = manager.add_tab(format!("Tab {}", i));
            ids.push(id);
        }

        // Close all tabs from last to first
        for id in ids.iter().rev() {
            manager.close_tab(*id);
        }

        assert_eq!(manager.count(), 0);
    }

    #[test]
    fn test_interleaved_add_close_navigation() {
        let mut manager = TabManager::new();

        let id1 = manager.add_tab("Tab 1").unwrap();
        manager.next_tab();

        let _id2 = manager.add_tab("Tab 2").unwrap();
        manager.previous_tab();

        manager.close_tab(id1);
        manager.next_tab();

        let _id3 = manager.add_tab("Tab 3").unwrap();

        // Manager should be in consistent state
        assert_eq!(manager.count(), 2);
        assert!(manager.active_tab().is_some());
    }

    // ============================================================================
    // TAB INDICATORS TESTS (Layer 6.6)
    // ============================================================================

    #[test]
    fn test_tab_operation_indicator() {
        let mut tab = Tab::new(0, "Tab");
        assert!(!tab.has_operation);

        tab.set_operation(true);
        assert!(tab.has_operation);

        tab.set_operation(false);
        assert!(!tab.has_operation);
    }

    #[test]
    fn test_tab_error_indicator() {
        let mut tab = Tab::new(0, "Tab");
        assert!(!tab.has_error);

        tab.set_error(true);
        assert!(tab.has_error);

        tab.set_error(false);
        assert!(!tab.has_error);
    }

    #[test]
    fn test_tab_display_name_with_indicators_none() {
        let tab = Tab::new(0, "Clean Tab");
        assert_eq!(tab.display_name_with_indicators(), "Clean Tab");
    }

    #[test]
    fn test_tab_display_name_with_indicators_modified_only() {
        let mut tab = Tab::new(0, "Modified Tab");
        tab.set_modified(true);
        assert_eq!(tab.display_name_with_indicators(), "Modified Tab●");
    }

    #[test]
    fn test_tab_display_name_with_indicators_operation_only() {
        let mut tab = Tab::new(0, "Active Tab");
        tab.set_operation(true);
        assert_eq!(tab.display_name_with_indicators(), "Active Tab*");
    }

    #[test]
    fn test_tab_display_name_with_indicators_error_only() {
        let mut tab = Tab::new(0, "Error Tab");
        tab.set_error(true);
        assert_eq!(tab.display_name_with_indicators(), "Error Tab!");
    }

    #[test]
    fn test_tab_display_name_with_indicators_all() {
        let mut tab = Tab::new(0, "Busy Tab");
        tab.set_modified(true);
        tab.set_operation(true);
        tab.set_error(true);
        assert_eq!(tab.display_name_with_indicators(), "Busy Tab●*!");
    }

    #[test]
    fn test_tab_display_name_with_indicators_modified_and_operation() {
        let mut tab = Tab::new(0, "Tab");
        tab.set_modified(true);
        tab.set_operation(true);
        assert_eq!(tab.display_name_with_indicators(), "Tab●*");
    }

    #[test]
    fn test_tab_display_name_with_indicators_modified_and_error() {
        let mut tab = Tab::new(0, "Tab");
        tab.set_modified(true);
        tab.set_error(true);
        assert_eq!(tab.display_name_with_indicators(), "Tab●!");
    }

    #[test]
    fn test_tab_display_name_with_indicators_operation_and_error() {
        let mut tab = Tab::new(0, "Tab");
        tab.set_operation(true);
        tab.set_error(true);
        assert_eq!(tab.display_name_with_indicators(), "Tab*!");
    }

    #[test]
    fn test_tab_indicators_serialization() {
        let mut tab = Tab::new(0, "Tab");
        tab.set_modified(true);
        tab.set_operation(true);
        tab.set_error(true);

        let json = serde_json::to_string(&tab).unwrap();
        assert!(json.contains("\"modified\":true"));
        assert!(json.contains("\"has_operation\":true"));
        assert!(json.contains("\"has_error\":true"));
    }

    #[test]
    fn test_tab_indicators_deserialization() {
        let json = r#"{
            "id": 0,
            "title": "Test",
            "icon": null,
            "closable": true,
            "modified": true,
            "has_operation": true,
            "has_error": true
        }"#;

        let tab: Tab = serde_json::from_str(json).unwrap();
        assert!(tab.modified);
        assert!(tab.has_operation);
        assert!(tab.has_error);
    }

    #[test]
    fn test_tab_indicators_backward_compat() {
        // Old JSON without has_operation and has_error fields
        let json = r#"{
            "id": 0,
            "title": "Old Tab",
            "icon": null,
            "closable": true,
            "modified": false
        }"#;

        let tab: Tab = serde_json::from_str(json).unwrap();
        assert!(!tab.has_operation);
        assert!(!tab.has_error);
    }

    #[test]
    fn test_tab_indicators_with_unicode() {
        let mut tab = Tab::new(0, "日本語タブ");
        tab.set_modified(true);
        tab.set_operation(true);
        tab.set_error(true);
        assert_eq!(tab.display_name_with_indicators(), "日本語タブ●*!");
    }

    #[test]
    fn test_tab_indicators_with_emoji() {
        let mut tab = Tab::new(0, "🐸 Frog Tab");
        tab.set_modified(true);
        tab.set_operation(true);
        tab.set_error(true);
        assert_eq!(tab.display_name_with_indicators(), "🐸 Frog Tab●*!");
    }

    #[test]
    fn test_tab_indicators_clone() {
        let mut tab1 = Tab::new(0, "Tab");
        tab1.set_modified(true);
        tab1.set_operation(true);
        tab1.set_error(true);

        let tab2 = tab1.clone();
        assert_eq!(tab1.has_operation, tab2.has_operation);
        assert_eq!(tab1.has_error, tab2.has_error);
    }

    #[test]
    fn test_tab_indicators_state_transitions() {
        let mut tab = Tab::new(0, "Tab");

        // No indicators
        assert_eq!(tab.display_name_with_indicators(), "Tab");

        // Add operation
        tab.set_operation(true);
        assert_eq!(tab.display_name_with_indicators(), "Tab*");

        // Add error
        tab.set_error(true);
        assert_eq!(tab.display_name_with_indicators(), "Tab*!");

        // Add modified
        tab.set_modified(true);
        assert_eq!(tab.display_name_with_indicators(), "Tab●*!");

        // Remove operation
        tab.set_operation(false);
        assert_eq!(tab.display_name_with_indicators(), "Tab●!");

        // Remove all
        tab.set_modified(false);
        tab.set_error(false);
        assert_eq!(tab.display_name_with_indicators(), "Tab");
    }

    // ============================================================================
    // MAX TABS LIMIT TESTS (Layer 6.6)
    // ============================================================================

    #[test]
    fn test_max_tabs_constant() {
        assert_eq!(MAX_TABS, 10);
    }

    #[test]
    fn test_add_tab_at_max_limit() {
        let mut manager = TabManager::new();

        // Add 10 tabs (max)
        for i in 0..10 {
            let result = manager.add_tab(format!("Tab {}", i));
            assert!(result.is_some(), "Should allow adding tab {}", i);
        }

        assert_eq!(manager.count(), 10);

        // Try to add 11th tab - should fail
        let result = manager.add_tab("Tab 11");
        assert!(result.is_none(), "Should not allow adding tab beyond max");

        // Count should still be 10
        assert_eq!(manager.count(), 10);
    }

    #[test]
    fn test_add_tab_with_at_max_limit() {
        let mut manager = TabManager::new();

        // Add 10 tabs
        for i in 0..10 {
            let tab = Tab::new(i, format!("Tab {}", i));
            let result = manager.add_tab_with(tab);
            assert!(result.is_some());
        }

        // Try to add 11th tab with custom config
        let tab11 = Tab::new(10, "Tab 11").with_icon("🔧");
        let result = manager.add_tab_with(tab11);
        assert!(result.is_none());

        assert_eq!(manager.count(), 10);
    }

    #[test]
    fn test_at_max_tabs_method() {
        let mut manager = TabManager::new();

        assert!(!manager.at_max_tabs());

        // Add 9 tabs
        for i in 0..9 {
            manager.add_tab(format!("Tab {}", i));
        }

        assert!(!manager.at_max_tabs());

        // Add 10th tab
        manager.add_tab("Tab 10").unwrap();
        assert!(manager.at_max_tabs());
    }

    #[test]
    fn test_remaining_slots_method() {
        let mut manager = TabManager::new();

        assert_eq!(manager.remaining_slots(), 10);

        manager.add_tab("Tab 1").unwrap();
        assert_eq!(manager.remaining_slots(), 9);

        manager.add_tab("Tab 2").unwrap();
        assert_eq!(manager.remaining_slots(), 8);

        // Add 8 more tabs
        for i in 3..=10 {
            manager.add_tab(format!("Tab {}", i));
        }

        assert_eq!(manager.remaining_slots(), 0);
    }

    #[test]
    fn test_add_after_closing_tab() {
        let mut manager = TabManager::new();

        // Fill to max
        for i in 0..10 {
            manager.add_tab(format!("Tab {}", i));
        }

        assert!(manager.at_max_tabs());

        // Try to add - should fail
        assert!(manager.add_tab("Extra").is_none());

        // Close a tab
        manager.close_tab(5);
        assert_eq!(manager.count(), 9);
        assert!(!manager.at_max_tabs());

        // Now should be able to add again
        let result = manager.add_tab("New Tab");
        assert!(result.is_some());
        assert_eq!(manager.count(), 10);
    }

    #[test]
    fn test_max_tabs_with_navigation() {
        let mut manager = TabManager::new();

        // Add max tabs
        for i in 0..10 {
            manager.add_tab(format!("Tab {}", i));
        }

        // Navigation should still work
        manager.next_tab();
        assert_eq!(manager.active_index(), Some(1));

        manager.previous_tab();
        assert_eq!(manager.active_index(), Some(0));

        // Try to add while navigating - should fail
        assert!(manager.add_tab("Extra").is_none());
    }

    #[test]
    fn test_max_tabs_cycle_add_close() {
        let mut manager = TabManager::new();

        for _ in 0..100 {
            // Fill to max
            for i in 0..10 {
                manager.add_tab(format!("Tab {}", i));
            }

            // Remove all
            let ids: Vec<_> = manager.tabs().iter().map(|t| t.id).collect();
            for id in ids {
                manager.close_tab(id);
            }

            assert_eq!(manager.count(), 0);
        }
    }

    #[test]
    fn test_max_tabs_unicode_titles() {
        let mut manager = TabManager::new();

        // Add 10 tabs with unicode titles
        for i in 0..10 {
            manager.add_tab(format!("🐸 Tab {} 日本語", i));
        }

        assert!(manager.at_max_tabs());

        // Should not allow 11th
        assert!(manager.add_tab("🔧 Extra").is_none());
    }

    #[test]
    fn test_max_tabs_with_icons() {
        let mut manager = TabManager::new();

        for i in 0..10 {
            let tab = Tab::new(i, format!("Tab {}", i)).with_icon("📁");
            manager.add_tab_with(tab);
        }

        assert_eq!(manager.count(), 10);

        let tab11 = Tab::new(10, "Tab 11").with_icon("🔧");
        assert!(manager.add_tab_with(tab11).is_none());
    }

    #[test]
    fn test_remaining_slots_boundary() {
        let mut manager = TabManager::new();

        for i in 0..10 {
            let remaining = manager.remaining_slots();
            assert_eq!(remaining, 10 - i);
            manager.add_tab(format!("Tab {}", i));
        }

        assert_eq!(manager.remaining_slots(), 0);

        // Close 5 tabs
        for id in 0..5 {
            manager.close_tab(id);
        }

        assert_eq!(manager.remaining_slots(), 5);
    }

    #[test]
    fn test_max_tabs_idempotent_check() {
        let mut manager = TabManager::new();

        for i in 0..10 {
            manager.add_tab(format!("Tab {}", i));
        }

        // Multiple checks should be idempotent
        for _ in 0..100 {
            assert!(manager.at_max_tabs());
            assert_eq!(manager.remaining_slots(), 0);
        }
    }
}
