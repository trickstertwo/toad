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
    /// Whether this tab has unsaved changes
    pub modified: bool,
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

    /// Get display name (with modification indicator)
    pub fn display_name(&self) -> String {
        if self.modified {
            format!("{}*", self.title)
        } else {
            self.title.clone()
        }
    }
}

impl fmt::Display for Tab {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

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
        manager.add_tab(title);
        manager
    }

    /// Add a new tab
    pub fn add_tab(&mut self, title: impl Into<String>) -> TabId {
        let id = self.next_id;
        self.next_id += 1;

        let tab = Tab::new(id, title);
        self.tabs.push(tab);

        // Auto-activate if this is the first tab
        if self.tabs.len() == 1 {
            self.active = Some(0);
        }

        id
    }

    /// Add a tab with custom configuration
    pub fn add_tab_with(&mut self, tab: Tab) -> TabId {
        let id = tab.id;
        self.tabs.push(tab);

        if self.tabs.len() == 1 {
            self.active = Some(0);
        }

        // Update next_id to avoid conflicts
        if id >= self.next_id {
            self.next_id = id + 1;
        }

        id
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

        let id1 = manager.add_tab("Tab 1");
        assert_eq!(manager.count(), 1);
        assert_eq!(manager.active_tab_id(), Some(id1));

        let _id2 = manager.add_tab("Tab 2");
        assert_eq!(manager.count(), 2);
        assert_eq!(manager.active_tab_id(), Some(id1)); // Active doesn't change
    }

    #[test]
    fn test_tab_manager_close() {
        let mut manager = TabManager::new();
        let _id1 = manager.add_tab("Tab 1");
        let id2 = manager.add_tab("Tab 2");
        let _id3 = manager.add_tab("Tab 3");

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
        manager.add_tab("Tab 1");
        manager.add_tab("Tab 2");
        manager.add_tab("Tab 3");

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
        manager.add_tab("Tab 1");
        manager.add_tab("Tab 2");
        manager.add_tab("Tab 3");

        assert!(manager.switch_to_index(2));
        assert_eq!(manager.active_index(), Some(2));

        assert!(!manager.switch_to_index(10));
        assert_eq!(manager.active_index(), Some(2)); // Unchanged
    }

    #[test]
    fn test_tab_manager_get_tab() {
        let mut manager = TabManager::new();
        let id = manager.add_tab("Test");

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
}
