//! Tab system for managing multiple workspaces
//!
//! Provides tab management functionality allowing users to work with multiple
//! independent workspaces in a single session.
//!
//! # Examples
//!
//! ```
//! use toad::tabs::{Tab, TabManager};
//!
//! // Create tab manager (starts with default "Main" tab)
//! let mut tabs = TabManager::new();
//! assert_eq!(tabs.tab_count(), 1);
//!
//! // Add more tabs
//! tabs.add_tab("Debug".to_string());
//! tabs.add_tab("Test".to_string());
//!
//! assert_eq!(tabs.tab_count(), 3);
//! assert_eq!(tabs.active_index(), 0);
//! ```
//!
//! # Tab Navigation
//!
//! - Number keys (1-9): Jump to specific tab
//! - Tab: Cycle to next tab
//! - Shift+Tab: Cycle to previous tab
//! - gt: Vim-style next tab (future)
//! - gT: Vim-style previous tab (future)

use serde::{Deserialize, Serialize};

/// A single tab in the tab system
///
/// Each tab represents an independent workspace with its own state.
///
/// # Examples
///
/// ```
/// use toad::tabs::Tab;
///
/// let tab = Tab::new("Project 1".to_string());
/// assert_eq!(tab.name(), "Project 1");
/// assert!(!tab.is_modified());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Tab {
    /// Unique identifier for the tab
    id: usize,

    /// Display name of the tab
    name: String,

    /// Whether the tab has unsaved changes
    modified: bool,

    /// Optional icon for the tab (for future use)
    icon: Option<String>,
}

impl Tab {
    /// Create a new tab with the given name
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::tabs::Tab;
    ///
    /// let tab = Tab::new("Home".to_string());
    /// assert_eq!(tab.name(), "Home");
    /// ```
    pub fn new(name: String) -> Self {
        Self {
            id: 0, // Will be set by TabManager
            name,
            modified: false,
            icon: None,
        }
    }

    /// Create a tab with a specific ID
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::tabs::Tab;
    ///
    /// let tab = Tab::with_id(5, "Tab 5".to_string());
    /// assert_eq!(tab.id(), 5);
    /// ```
    pub fn with_id(id: usize, name: String) -> Self {
        Self {
            id,
            name,
            modified: false,
            icon: None,
        }
    }

    /// Get the tab ID
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::tabs::Tab;
    ///
    /// let tab = Tab::with_id(3, "Tab".to_string());
    /// assert_eq!(tab.id(), 3);
    /// ```
    pub fn id(&self) -> usize {
        self.id
    }

    /// Set the tab ID
    pub(crate) fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    /// Get the tab name
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::tabs::Tab;
    ///
    /// let tab = Tab::new("My Tab".to_string());
    /// assert_eq!(tab.name(), "My Tab");
    /// ```
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set the tab name
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::tabs::Tab;
    ///
    /// let mut tab = Tab::new("Old Name".to_string());
    /// tab.set_name("New Name".to_string());
    /// assert_eq!(tab.name(), "New Name");
    /// ```
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Check if tab has unsaved changes
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::tabs::Tab;
    ///
    /// let mut tab = Tab::new("Tab".to_string());
    /// assert!(!tab.is_modified());
    ///
    /// tab.set_modified(true);
    /// assert!(tab.is_modified());
    /// ```
    pub fn is_modified(&self) -> bool {
        self.modified
    }

    /// Set the modified status
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::tabs::Tab;
    ///
    /// let mut tab = Tab::new("Tab".to_string());
    /// tab.set_modified(true);
    /// assert!(tab.is_modified());
    /// ```
    pub fn set_modified(&mut self, modified: bool) {
        self.modified = modified;
    }

    /// Get the tab icon
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::tabs::Tab;
    ///
    /// let tab = Tab::new("Tab".to_string());
    /// assert_eq!(tab.icon(), None);
    /// ```
    pub fn icon(&self) -> Option<&String> {
        self.icon.as_ref()
    }

    /// Set the tab icon
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::tabs::Tab;
    ///
    /// let mut tab = Tab::new("Tab".to_string());
    /// tab.set_icon(Some("📁".to_string()));
    /// assert_eq!(tab.icon(), Some(&"📁".to_string()));
    /// ```
    pub fn set_icon(&mut self, icon: Option<String>) {
        self.icon = icon;
    }

    /// Get a display string for the tab (with modified indicator)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::tabs::Tab;
    ///
    /// let mut tab = Tab::new("File".to_string());
    /// assert_eq!(tab.display_name(), "File");
    ///
    /// tab.set_modified(true);
    /// assert_eq!(tab.display_name(), "File *");
    /// ```
    pub fn display_name(&self) -> String {
        if self.modified {
            format!("{} *", self.name)
        } else {
            self.name.clone()
        }
    }
}

/// Manages a collection of tabs with navigation
///
/// # Examples
///
/// ```
/// use toad::tabs::TabManager;
///
/// let mut tabs = TabManager::new();
/// tabs.add_tab("Tab 2".to_string());
///
/// assert_eq!(tabs.tab_count(), 2);
/// assert_eq!(tabs.active_tab().map(|t| t.name()), Some("Main"));
///
/// tabs.next_tab();
/// assert_eq!(tabs.active_tab().map(|t| t.name()), Some("Tab 2"));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TabManager {
    /// List of tabs
    tabs: Vec<Tab>,

    /// Index of the currently active tab
    active_index: usize,

    /// Next ID to assign to a new tab
    next_id: usize,

    /// Maximum number of tabs allowed
    max_tabs: usize,
}

impl Default for TabManager {
    fn default() -> Self {
        Self::new()
    }
}

impl TabManager {
    /// Create a new tab manager with a default tab
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::tabs::TabManager;
    ///
    /// let tabs = TabManager::new();
    /// assert_eq!(tabs.tab_count(), 1);
    /// assert_eq!(tabs.active_tab().map(|t| t.name()), Some("Main"));
    /// ```
    pub fn new() -> Self {
        let mut manager = Self {
            tabs: Vec::new(),
            active_index: 0,
            next_id: 0,
            max_tabs: 9, // Limit to 9 for number key navigation
        };

        // Add default tab
        manager.add_tab("Main".to_string());
        manager
    }

    /// Create an empty tab manager (no default tab)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::tabs::TabManager;
    ///
    /// let tabs = TabManager::empty();
    /// assert_eq!(tabs.tab_count(), 0);
    /// ```
    pub fn empty() -> Self {
        Self {
            tabs: Vec::new(),
            active_index: 0,
            next_id: 0,
            max_tabs: 9,
        }
    }

    /// Add a new tab with the given name
    ///
    /// Returns the ID of the new tab, or None if max tabs reached.
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::tabs::TabManager;
    ///
    /// let mut tabs = TabManager::empty();
    /// let id = tabs.add_tab("New Tab".to_string());
    /// assert_eq!(id, Some(0));
    /// assert_eq!(tabs.tab_count(), 1);
    /// ```
    pub fn add_tab(&mut self, name: String) -> Option<usize> {
        if self.tabs.len() >= self.max_tabs {
            return None;
        }

        let id = self.next_id;
        self.next_id += 1;

        let mut tab = Tab::new(name);
        tab.set_id(id);
        self.tabs.push(tab);

        Some(id)
    }

    /// Remove the tab at the given index
    ///
    /// Returns the removed tab, or None if index is invalid.
    /// Cannot remove the last tab.
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::tabs::TabManager;
    ///
    /// let mut tabs = TabManager::new();
    /// tabs.add_tab("Tab 2".to_string());
    ///
    /// let removed = tabs.remove_tab(1);
    /// assert!(removed.is_some());
    /// assert_eq!(tabs.tab_count(), 1);
    /// ```
    pub fn remove_tab(&mut self, index: usize) -> Option<Tab> {
        // Don't allow removing the last tab
        if self.tabs.len() <= 1 || index >= self.tabs.len() {
            return None;
        }

        let tab = self.tabs.remove(index);

        // Adjust active index if needed
        if self.active_index >= self.tabs.len() {
            self.active_index = self.tabs.len().saturating_sub(1);
        } else if self.active_index > index {
            self.active_index -= 1;
        }

        Some(tab)
    }

    /// Get the number of tabs
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::tabs::TabManager;
    ///
    /// let tabs = TabManager::new();
    /// assert_eq!(tabs.tab_count(), 1);
    /// ```
    pub fn tab_count(&self) -> usize {
        self.tabs.len()
    }

    /// Get the active tab index
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::tabs::TabManager;
    ///
    /// let tabs = TabManager::new();
    /// assert_eq!(tabs.active_index(), 0);
    /// ```
    pub fn active_index(&self) -> usize {
        self.active_index
    }

    /// Get reference to the active tab
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::tabs::TabManager;
    ///
    /// let tabs = TabManager::new();
    /// assert_eq!(tabs.active_tab().map(|t| t.name()), Some("Main"));
    /// ```
    pub fn active_tab(&self) -> Option<&Tab> {
        self.tabs.get(self.active_index)
    }

    /// Get mutable reference to the active tab
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::tabs::TabManager;
    ///
    /// let mut tabs = TabManager::new();
    /// if let Some(tab) = tabs.active_tab_mut() {
    ///     tab.set_modified(true);
    /// }
    /// assert!(tabs.active_tab().unwrap().is_modified());
    /// ```
    pub fn active_tab_mut(&mut self) -> Option<&mut Tab> {
        self.tabs.get_mut(self.active_index)
    }

    /// Get reference to a tab by index
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::tabs::TabManager;
    ///
    /// let tabs = TabManager::new();
    /// assert_eq!(tabs.tab(0).map(|t| t.name()), Some("Main"));
    /// assert_eq!(tabs.tab(1), None);
    /// ```
    pub fn tab(&self, index: usize) -> Option<&Tab> {
        self.tabs.get(index)
    }

    /// Get mutable reference to a tab by index
    pub fn tab_mut(&mut self, index: usize) -> Option<&mut Tab> {
        self.tabs.get_mut(index)
    }

    /// Get all tabs as a slice
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::tabs::TabManager;
    ///
    /// let tabs = TabManager::new();
    /// assert_eq!(tabs.tabs().len(), 1);
    /// ```
    pub fn tabs(&self) -> &[Tab] {
        &self.tabs
    }

    /// Switch to the next tab (wraps around)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::tabs::TabManager;
    ///
    /// let mut tabs = TabManager::new();
    /// tabs.add_tab("Tab 2".to_string());
    ///
    /// assert_eq!(tabs.active_index(), 0);
    /// tabs.next_tab();
    /// assert_eq!(tabs.active_index(), 1);
    /// tabs.next_tab();
    /// assert_eq!(tabs.active_index(), 0); // Wrapped around
    /// ```
    pub fn next_tab(&mut self) {
        if self.tabs.is_empty() {
            return;
        }

        self.active_index = (self.active_index + 1) % self.tabs.len();
    }

    /// Switch to the previous tab (wraps around)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::tabs::TabManager;
    ///
    /// let mut tabs = TabManager::new();
    /// tabs.add_tab("Tab 2".to_string());
    ///
    /// assert_eq!(tabs.active_index(), 0);
    /// tabs.prev_tab();
    /// assert_eq!(tabs.active_index(), 1); // Wrapped to last
    /// tabs.prev_tab();
    /// assert_eq!(tabs.active_index(), 0);
    /// ```
    pub fn prev_tab(&mut self) {
        if self.tabs.is_empty() {
            return;
        }

        if self.active_index == 0 {
            self.active_index = self.tabs.len() - 1;
        } else {
            self.active_index -= 1;
        }
    }

    /// Switch to a specific tab by index
    ///
    /// Returns true if successful, false if index is invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::tabs::TabManager;
    ///
    /// let mut tabs = TabManager::new();
    /// tabs.add_tab("Tab 2".to_string());
    ///
    /// assert!(tabs.switch_to(1));
    /// assert_eq!(tabs.active_index(), 1);
    ///
    /// assert!(!tabs.switch_to(5)); // Invalid index
    /// ```
    pub fn switch_to(&mut self, index: usize) -> bool {
        if index < self.tabs.len() {
            self.active_index = index;
            true
        } else {
            false
        }
    }

    /// Switch to tab by number key (1-9)
    ///
    /// Returns true if successful, false if number is invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::tabs::TabManager;
    ///
    /// let mut tabs = TabManager::new();
    /// tabs.add_tab("Tab 2".to_string());
    ///
    /// assert!(tabs.switch_to_number(2)); // Switch to second tab
    /// assert_eq!(tabs.active_index(), 1);
    ///
    /// assert!(!tabs.switch_to_number(5)); // No 5th tab
    /// ```
    pub fn switch_to_number(&mut self, number: usize) -> bool {
        if number > 0 && number <= self.tabs.len() {
            self.active_index = number - 1;
            true
        } else {
            false
        }
    }

    /// Get maximum number of tabs allowed
    pub fn max_tabs(&self) -> usize {
        self.max_tabs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tab_creation() {
        let tab = Tab::new("Test".to_string());
        assert_eq!(tab.name(), "Test");
        assert_eq!(tab.id(), 0);
        assert!(!tab.is_modified());
        assert_eq!(tab.icon(), None);
    }

    #[test]
    fn test_tab_with_id() {
        let tab = Tab::with_id(5, "Tab 5".to_string());
        assert_eq!(tab.id(), 5);
        assert_eq!(tab.name(), "Tab 5");
    }

    #[test]
    fn test_tab_setters() {
        let mut tab = Tab::new("Original".to_string());

        tab.set_name("Changed".to_string());
        assert_eq!(tab.name(), "Changed");

        tab.set_modified(true);
        assert!(tab.is_modified());

        tab.set_icon(Some("🔧".to_string()));
        assert_eq!(tab.icon(), Some(&"🔧".to_string()));
    }

    #[test]
    fn test_tab_display_name() {
        let mut tab = Tab::new("File".to_string());
        assert_eq!(tab.display_name(), "File");

        tab.set_modified(true);
        assert_eq!(tab.display_name(), "File *");
    }

    #[test]
    fn test_tab_manager_creation() {
        let tabs = TabManager::new();
        assert_eq!(tabs.tab_count(), 1);
        assert_eq!(tabs.active_index(), 0);
        assert_eq!(tabs.active_tab().map(|t| t.name()), Some("Main"));
    }

    #[test]
    fn test_tab_manager_empty() {
        let tabs = TabManager::empty();
        assert_eq!(tabs.tab_count(), 0);
        assert_eq!(tabs.active_tab(), None);
    }

    #[test]
    fn test_add_tab() {
        let mut tabs = TabManager::empty();

        let id1 = tabs.add_tab("Tab 1".to_string());
        assert_eq!(id1, Some(0));
        assert_eq!(tabs.tab_count(), 1);

        let id2 = tabs.add_tab("Tab 2".to_string());
        assert_eq!(id2, Some(1));
        assert_eq!(tabs.tab_count(), 2);
    }

    #[test]
    fn test_max_tabs() {
        let mut tabs = TabManager::empty();

        // Add 9 tabs (max)
        for i in 1..=9 {
            let id = tabs.add_tab(format!("Tab {}", i));
            assert!(id.is_some());
        }

        assert_eq!(tabs.tab_count(), 9);

        // Try to add 10th tab
        let id = tabs.add_tab("Tab 10".to_string());
        assert_eq!(id, None);
        assert_eq!(tabs.tab_count(), 9);
    }

    #[test]
    fn test_remove_tab() {
        let mut tabs = TabManager::new();
        tabs.add_tab("Tab 2".to_string());
        tabs.add_tab("Tab 3".to_string());

        assert_eq!(tabs.tab_count(), 3);

        let removed = tabs.remove_tab(1);
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().name(), "Tab 2");
        assert_eq!(tabs.tab_count(), 2);
    }

    #[test]
    fn test_cannot_remove_last_tab() {
        let mut tabs = TabManager::new();

        let removed = tabs.remove_tab(0);
        assert_eq!(removed, None);
        assert_eq!(tabs.tab_count(), 1);
    }

    #[test]
    fn test_remove_tab_adjusts_active_index() {
        let mut tabs = TabManager::new();
        tabs.add_tab("Tab 2".to_string());
        tabs.add_tab("Tab 3".to_string());

        tabs.switch_to(2); // Active on last tab
        assert_eq!(tabs.active_index(), 2);

        tabs.remove_tab(2);
        assert_eq!(tabs.active_index(), 1); // Adjusted
    }

    #[test]
    fn test_next_tab() {
        let mut tabs = TabManager::new();
        tabs.add_tab("Tab 2".to_string());
        tabs.add_tab("Tab 3".to_string());

        assert_eq!(tabs.active_index(), 0);

        tabs.next_tab();
        assert_eq!(tabs.active_index(), 1);

        tabs.next_tab();
        assert_eq!(tabs.active_index(), 2);

        tabs.next_tab(); // Wrap around
        assert_eq!(tabs.active_index(), 0);
    }

    #[test]
    fn test_prev_tab() {
        let mut tabs = TabManager::new();
        tabs.add_tab("Tab 2".to_string());
        tabs.add_tab("Tab 3".to_string());

        assert_eq!(tabs.active_index(), 0);

        tabs.prev_tab(); // Wrap to last
        assert_eq!(tabs.active_index(), 2);

        tabs.prev_tab();
        assert_eq!(tabs.active_index(), 1);

        tabs.prev_tab();
        assert_eq!(tabs.active_index(), 0);
    }

    #[test]
    fn test_switch_to() {
        let mut tabs = TabManager::new();
        tabs.add_tab("Tab 2".to_string());

        assert!(tabs.switch_to(1));
        assert_eq!(tabs.active_index(), 1);

        assert!(!tabs.switch_to(5)); // Invalid
        assert_eq!(tabs.active_index(), 1); // Unchanged
    }

    #[test]
    fn test_switch_to_number() {
        let mut tabs = TabManager::new();
        tabs.add_tab("Tab 2".to_string());
        tabs.add_tab("Tab 3".to_string());

        assert!(tabs.switch_to_number(2));
        assert_eq!(tabs.active_index(), 1);

        assert!(tabs.switch_to_number(3));
        assert_eq!(tabs.active_index(), 2);

        assert!(!tabs.switch_to_number(0)); // Invalid (numbers are 1-based)
        assert!(!tabs.switch_to_number(5)); // Out of range
    }

    #[test]
    fn test_tab_access() {
        let mut tabs = TabManager::new();
        tabs.add_tab("Tab 2".to_string());

        assert_eq!(tabs.tab(0).map(|t| t.name()), Some("Main"));
        assert_eq!(tabs.tab(1).map(|t| t.name()), Some("Tab 2"));
        assert_eq!(tabs.tab(5), None);

        if let Some(tab) = tabs.tab_mut(0) {
            tab.set_modified(true);
        }

        assert!(tabs.tab(0).unwrap().is_modified());
    }

    #[test]
    fn test_tabs_slice() {
        let tabs = TabManager::new();
        let all_tabs = tabs.tabs();

        assert_eq!(all_tabs.len(), 1);
        assert_eq!(all_tabs[0].name(), "Main");
    }
}
