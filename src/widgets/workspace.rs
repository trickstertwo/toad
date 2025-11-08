//! Workspace management for multiple project contexts
//!
//! Provides functionality to manage and switch between multiple workspaces.
//!
//! # Examples
//!
//! ```
//! use toad::widgets::WorkspaceManager;
//!
//! let mut manager = WorkspaceManager::new();
//! manager.create_workspace("project1", "/path/to/project1");
//!
//! assert_eq!(manager.workspace_count(), 1);
//! ```

use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

/// Workspace configuration
///
/// Represents a workspace with its settings and state.
///
/// # Examples
///
/// ```
/// use toad::widgets::Workspace;
///
/// let workspace = Workspace::new("my-project", "/path/to/project");
/// assert_eq!(workspace.name(), "my-project");
/// assert_eq!(workspace.path().to_str(), Some("/path/to/project"));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Workspace {
    /// Workspace name
    name: String,
    /// Workspace root path
    path: PathBuf,
    /// Workspace settings
    settings: HashMap<String, String>,
    /// Workspace state
    state: HashMap<String, String>,
    /// Last accessed timestamp
    last_accessed: u64,
}

impl Workspace {
    /// Create a new workspace
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Workspace;
    ///
    /// let workspace = Workspace::new("my-project", "/path/to/project");
    /// assert_eq!(workspace.name(), "my-project");
    /// ```
    pub fn new(name: impl Into<String>, path: impl Into<PathBuf>) -> Self {
        Self {
            name: name.into(),
            path: path.into(),
            settings: HashMap::new(),
            state: HashMap::new(),
            last_accessed: Self::current_timestamp(),
        }
    }

    /// Get workspace name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get workspace path
    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    /// Get workspace settings
    pub fn settings(&self) -> &HashMap<String, String> {
        &self.settings
    }

    /// Get workspace state
    pub fn state(&self) -> &HashMap<String, String> {
        &self.state
    }

    /// Get last accessed timestamp
    pub fn last_accessed(&self) -> u64 {
        self.last_accessed
    }

    /// Set a workspace setting
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Workspace;
    ///
    /// let mut workspace = Workspace::new("project", "/path");
    /// workspace.set_setting("theme", "dark");
    /// assert_eq!(workspace.get_setting("theme"), Some("dark"));
    /// ```
    pub fn set_setting(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.settings.insert(key.into(), value.into());
    }

    /// Get a workspace setting
    pub fn get_setting(&self, key: &str) -> Option<&str> {
        self.settings.get(key).map(|s| s.as_str())
    }

    /// Remove a workspace setting
    pub fn remove_setting(&mut self, key: &str) -> Option<String> {
        self.settings.remove(key)
    }

    /// Set workspace state
    pub fn set_state(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.state.insert(key.into(), value.into());
    }

    /// Get workspace state
    pub fn get_state(&self, key: &str) -> Option<&str> {
        self.state.get(key).map(|s| s.as_str())
    }

    /// Remove workspace state
    pub fn remove_state(&mut self, key: &str) -> Option<String> {
        self.state.remove(key)
    }

    /// Update last accessed timestamp
    pub fn touch(&mut self) {
        self.last_accessed = Self::current_timestamp();
    }

    /// Set workspace path
    pub fn set_path(&mut self, path: impl Into<PathBuf>) {
        self.path = path.into();
    }

    /// Get current Unix timestamp
    fn current_timestamp() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }
}

/// Workspace manager
///
/// Manages multiple workspaces with switching and persistence.
///
/// # Examples
///
/// ```
/// use toad::widgets::WorkspaceManager;
///
/// let mut manager = WorkspaceManager::new();
/// manager.create_workspace("project1", "/path/to/project1");
/// manager.create_workspace("project2", "/path/to/project2");
///
/// manager.switch_workspace("project2");
/// assert_eq!(manager.active_workspace_name(), Some("project2"));
/// ```
#[derive(Debug, Clone, Default)]
pub struct WorkspaceManager {
    /// All workspaces
    workspaces: HashMap<String, Workspace>,
    /// Active workspace name
    active_workspace: Option<String>,
    /// Recently used workspace names
    recent: Vec<String>,
    /// Max recent workspaces to track
    max_recent: usize,
}

impl WorkspaceManager {
    /// Create a new workspace manager
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::WorkspaceManager;
    ///
    /// let manager = WorkspaceManager::new();
    /// assert_eq!(manager.workspace_count(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            workspaces: HashMap::new(),
            active_workspace: None,
            recent: Vec::new(),
            max_recent: 10,
        }
    }

    /// Create a new workspace
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::WorkspaceManager;
    ///
    /// let mut manager = WorkspaceManager::new();
    /// manager.create_workspace("my-project", "/path/to/project");
    /// assert_eq!(manager.workspace_count(), 1);
    /// ```
    pub fn create_workspace(
        &mut self,
        name: impl Into<String>,
        path: impl Into<PathBuf>,
    ) -> bool {
        let name = name.into();
        if self.workspaces.contains_key(&name) {
            return false;
        }

        let workspace = Workspace::new(name.clone(), path);
        self.workspaces.insert(name.clone(), workspace);

        // Auto-switch to first workspace
        if self.active_workspace.is_none() {
            self.active_workspace = Some(name);
        }

        true
    }

    /// Switch to a workspace
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::WorkspaceManager;
    ///
    /// let mut manager = WorkspaceManager::new();
    /// manager.create_workspace("project1", "/path1");
    /// manager.create_workspace("project2", "/path2");
    ///
    /// assert!(manager.switch_workspace("project2"));
    /// assert_eq!(manager.active_workspace_name(), Some("project2"));
    /// ```
    pub fn switch_workspace(&mut self, name: &str) -> bool {
        if let Some(workspace) = self.workspaces.get_mut(name) {
            workspace.touch();
            self.active_workspace = Some(name.to_string());
            self.add_to_recent(name);
            true
        } else {
            false
        }
    }

    /// Get active workspace
    pub fn active_workspace(&self) -> Option<&Workspace> {
        self.active_workspace
            .as_ref()
            .and_then(|name| self.workspaces.get(name))
    }

    /// Get active workspace (mutable)
    pub fn active_workspace_mut(&mut self) -> Option<&mut Workspace> {
        self.active_workspace
            .as_ref()
            .and_then(|name| self.workspaces.get_mut(name))
    }

    /// Get active workspace name
    pub fn active_workspace_name(&self) -> Option<&str> {
        self.active_workspace.as_deref()
    }

    /// Get workspace by name
    pub fn get_workspace(&self, name: &str) -> Option<&Workspace> {
        self.workspaces.get(name)
    }

    /// Get workspace by name (mutable)
    pub fn get_workspace_mut(&mut self, name: &str) -> Option<&mut Workspace> {
        self.workspaces.get_mut(name)
    }

    /// Delete a workspace
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::WorkspaceManager;
    ///
    /// let mut manager = WorkspaceManager::new();
    /// manager.create_workspace("project", "/path");
    /// assert!(manager.delete_workspace("project"));
    /// assert_eq!(manager.workspace_count(), 0);
    /// ```
    pub fn delete_workspace(&mut self, name: &str) -> bool {
        let removed = self.workspaces.remove(name).is_some();

        if removed {
            // Update active workspace if deleted
            if self.active_workspace.as_deref() == Some(name) {
                self.active_workspace = self.workspaces.keys().next().cloned();
            }

            // Remove from recent
            self.recent.retain(|n| n != name);
        }

        removed
    }

    /// Rename a workspace
    pub fn rename_workspace(&mut self, old_name: &str, new_name: impl Into<String>) -> bool {
        if let Some(mut workspace) = self.workspaces.remove(old_name) {
            let new_name = new_name.into();
            workspace.name = new_name.clone();
            self.workspaces.insert(new_name.clone(), workspace);

            // Update active workspace
            if self.active_workspace.as_deref() == Some(old_name) {
                self.active_workspace = Some(new_name.clone());
            }

            // Update recent
            for recent_name in &mut self.recent {
                if recent_name == old_name {
                    *recent_name = new_name.clone();
                }
            }

            true
        } else {
            false
        }
    }

    /// Get all workspace names
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::WorkspaceManager;
    ///
    /// let mut manager = WorkspaceManager::new();
    /// manager.create_workspace("project1", "/path1");
    /// manager.create_workspace("project2", "/path2");
    ///
    /// let names = manager.workspace_names();
    /// assert_eq!(names.len(), 2);
    /// ```
    pub fn workspace_names(&self) -> Vec<&str> {
        self.workspaces.keys().map(|s| s.as_str()).collect()
    }

    /// Get workspace count
    pub fn workspace_count(&self) -> usize {
        self.workspaces.len()
    }

    /// Check if workspace exists
    pub fn has_workspace(&self, name: &str) -> bool {
        self.workspaces.contains_key(name)
    }

    /// Get recent workspace names
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::WorkspaceManager;
    ///
    /// let mut manager = WorkspaceManager::new();
    /// manager.create_workspace("p1", "/path1");
    /// manager.create_workspace("p2", "/path2");
    ///
    /// manager.switch_workspace("p1");
    /// manager.switch_workspace("p2");
    ///
    /// let recent = manager.recent_workspaces();
    /// assert_eq!(recent[0], "p2");
    /// assert_eq!(recent[1], "p1");
    /// ```
    pub fn recent_workspaces(&self) -> &[String] {
        &self.recent
    }

    /// Add workspace to recent list
    fn add_to_recent(&mut self, name: &str) {
        // Remove if already in list
        self.recent.retain(|n| n != name);

        // Add to front
        self.recent.insert(0, name.to_string());

        // Trim to max size
        if self.recent.len() > self.max_recent {
            self.recent.truncate(self.max_recent);
        }
    }

    /// Clear all workspaces
    pub fn clear(&mut self) {
        self.workspaces.clear();
        self.active_workspace = None;
        self.recent.clear();
    }

    /// Set max recent workspaces
    pub fn set_max_recent(&mut self, max: usize) {
        self.max_recent = max;
        if self.recent.len() > max {
            self.recent.truncate(max);
        }
    }

    /// Get workspaces sorted by last accessed
    pub fn workspaces_by_recent(&self) -> Vec<&Workspace> {
        let mut workspaces: Vec<&Workspace> = self.workspaces.values().collect();
        workspaces.sort_by(|a, b| b.last_accessed.cmp(&a.last_accessed));
        workspaces
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workspace_new() {
        let workspace = Workspace::new("test", "/path/to/test");
        assert_eq!(workspace.name(), "test");
        assert_eq!(workspace.path().to_str(), Some("/path/to/test"));
        assert!(workspace.settings().is_empty());
        assert!(workspace.state().is_empty());
        assert!(workspace.last_accessed() > 0);
    }

    #[test]
    fn test_workspace_settings() {
        let mut workspace = Workspace::new("test", "/path");
        workspace.set_setting("theme", "dark");
        assert_eq!(workspace.get_setting("theme"), Some("dark"));

        workspace.remove_setting("theme");
        assert_eq!(workspace.get_setting("theme"), None);
    }

    #[test]
    fn test_workspace_state() {
        let mut workspace = Workspace::new("test", "/path");
        workspace.set_state("cursor", "10");
        assert_eq!(workspace.get_state("cursor"), Some("10"));

        workspace.remove_state("cursor");
        assert_eq!(workspace.get_state("cursor"), None);
    }

    #[test]
    fn test_workspace_touch() {
        let mut workspace = Workspace::new("test", "/path");
        let initial = workspace.last_accessed();

        std::thread::sleep(std::time::Duration::from_millis(10));
        workspace.touch();

        assert!(workspace.last_accessed() >= initial);
    }

    #[test]
    fn test_workspace_set_path() {
        let mut workspace = Workspace::new("test", "/old/path");
        workspace.set_path("/new/path");
        assert_eq!(workspace.path().to_str(), Some("/new/path"));
    }

    #[test]
    fn test_workspace_manager_new() {
        let manager = WorkspaceManager::new();
        assert_eq!(manager.workspace_count(), 0);
        assert_eq!(manager.active_workspace_name(), None);
    }

    #[test]
    fn test_workspace_manager_default() {
        let manager = WorkspaceManager::default();
        assert_eq!(manager.workspace_count(), 0);
    }

    #[test]
    fn test_workspace_manager_create() {
        let mut manager = WorkspaceManager::new();
        assert!(manager.create_workspace("project1", "/path1"));
        assert_eq!(manager.workspace_count(), 1);

        // Cannot create duplicate
        assert!(!manager.create_workspace("project1", "/path1"));
        assert_eq!(manager.workspace_count(), 1);
    }

    #[test]
    fn test_workspace_manager_auto_switch_first() {
        let mut manager = WorkspaceManager::new();
        manager.create_workspace("project1", "/path1");
        assert_eq!(manager.active_workspace_name(), Some("project1"));
    }

    #[test]
    fn test_workspace_manager_switch() {
        let mut manager = WorkspaceManager::new();
        manager.create_workspace("project1", "/path1");
        manager.create_workspace("project2", "/path2");

        assert!(manager.switch_workspace("project2"));
        assert_eq!(manager.active_workspace_name(), Some("project2"));

        assert!(!manager.switch_workspace("nonexistent"));
    }

    #[test]
    fn test_workspace_manager_active_workspace() {
        let mut manager = WorkspaceManager::new();
        manager.create_workspace("project1", "/path1");

        let workspace = manager.active_workspace();
        assert!(workspace.is_some());
        assert_eq!(workspace.unwrap().name(), "project1");
    }

    #[test]
    fn test_workspace_manager_active_workspace_mut() {
        let mut manager = WorkspaceManager::new();
        manager.create_workspace("project1", "/path1");

        if let Some(workspace) = manager.active_workspace_mut() {
            workspace.set_setting("key", "value");
        }

        assert_eq!(
            manager
                .active_workspace()
                .unwrap()
                .get_setting("key"),
            Some("value")
        );
    }

    #[test]
    fn test_workspace_manager_get_workspace() {
        let mut manager = WorkspaceManager::new();
        manager.create_workspace("project1", "/path1");

        let workspace = manager.get_workspace("project1");
        assert!(workspace.is_some());
        assert_eq!(workspace.unwrap().name(), "project1");

        assert!(manager.get_workspace("nonexistent").is_none());
    }

    #[test]
    fn test_workspace_manager_get_workspace_mut() {
        let mut manager = WorkspaceManager::new();
        manager.create_workspace("project1", "/path1");

        if let Some(workspace) = manager.get_workspace_mut("project1") {
            workspace.set_setting("key", "value");
        }

        assert_eq!(
            manager.get_workspace("project1").unwrap().get_setting("key"),
            Some("value")
        );
    }

    #[test]
    fn test_workspace_manager_delete() {
        let mut manager = WorkspaceManager::new();
        manager.create_workspace("project1", "/path1");
        manager.create_workspace("project2", "/path2");

        assert!(manager.delete_workspace("project1"));
        assert_eq!(manager.workspace_count(), 1);

        assert!(!manager.delete_workspace("nonexistent"));
    }

    #[test]
    fn test_workspace_manager_delete_active() {
        let mut manager = WorkspaceManager::new();
        manager.create_workspace("project1", "/path1");
        manager.create_workspace("project2", "/path2");
        manager.switch_workspace("project1");

        manager.delete_workspace("project1");
        assert_eq!(manager.active_workspace_name(), Some("project2"));
    }

    #[test]
    fn test_workspace_manager_rename() {
        let mut manager = WorkspaceManager::new();
        manager.create_workspace("old-name", "/path");

        assert!(manager.rename_workspace("old-name", "new-name"));
        assert!(manager.has_workspace("new-name"));
        assert!(!manager.has_workspace("old-name"));
        assert_eq!(manager.active_workspace_name(), Some("new-name"));
    }

    #[test]
    fn test_workspace_manager_workspace_names() {
        let mut manager = WorkspaceManager::new();
        manager.create_workspace("project1", "/path1");
        manager.create_workspace("project2", "/path2");

        let names = manager.workspace_names();
        assert_eq!(names.len(), 2);
        assert!(names.contains(&"project1"));
        assert!(names.contains(&"project2"));
    }

    #[test]
    fn test_workspace_manager_has_workspace() {
        let mut manager = WorkspaceManager::new();
        manager.create_workspace("project1", "/path1");

        assert!(manager.has_workspace("project1"));
        assert!(!manager.has_workspace("project2"));
    }

    #[test]
    fn test_workspace_manager_recent() {
        let mut manager = WorkspaceManager::new();
        manager.create_workspace("p1", "/path1");
        manager.create_workspace("p2", "/path2");
        manager.create_workspace("p3", "/path3");

        manager.switch_workspace("p1");
        manager.switch_workspace("p2");
        manager.switch_workspace("p3");

        let recent = manager.recent_workspaces();
        assert_eq!(recent[0], "p3");
        assert_eq!(recent[1], "p2");
        assert_eq!(recent[2], "p1");
    }

    #[test]
    fn test_workspace_manager_recent_dedupe() {
        let mut manager = WorkspaceManager::new();
        manager.create_workspace("p1", "/path1");
        manager.create_workspace("p2", "/path2");

        manager.switch_workspace("p1");
        manager.switch_workspace("p2");
        manager.switch_workspace("p1");

        let recent = manager.recent_workspaces();
        assert_eq!(recent.len(), 2);
        assert_eq!(recent[0], "p1");
        assert_eq!(recent[1], "p2");
    }

    #[test]
    fn test_workspace_manager_clear() {
        let mut manager = WorkspaceManager::new();
        manager.create_workspace("p1", "/path1");
        manager.create_workspace("p2", "/path2");

        manager.clear();
        assert_eq!(manager.workspace_count(), 0);
        assert_eq!(manager.active_workspace_name(), None);
        assert_eq!(manager.recent_workspaces().len(), 0);
    }

    #[test]
    fn test_workspace_manager_set_max_recent() {
        let mut manager = WorkspaceManager::new();
        manager.set_max_recent(2);

        manager.create_workspace("p1", "/path1");
        manager.create_workspace("p2", "/path2");
        manager.create_workspace("p3", "/path3");

        manager.switch_workspace("p1");
        manager.switch_workspace("p2");
        manager.switch_workspace("p3");

        let recent = manager.recent_workspaces();
        assert_eq!(recent.len(), 2);
    }

    #[test]
    fn test_workspace_manager_workspaces_by_recent() {
        let mut manager = WorkspaceManager::new();
        manager.create_workspace("p1", "/path1");
        manager.create_workspace("p2", "/path2");

        std::thread::sleep(std::time::Duration::from_millis(10));
        manager.switch_workspace("p2");

        let workspaces = manager.workspaces_by_recent();
        assert_eq!(workspaces[0].name(), "p2");
        assert_eq!(workspaces[1].name(), "p1");
    }
}
