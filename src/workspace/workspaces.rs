/// Multiple workspace/project context management
///
/// Allows users to manage multiple project contexts with separate
/// state, recent files, and settings
///
/// # Examples
///
/// ```
/// use toad::workspaces::{Workspace, WorkspaceManager};
///
/// let mut manager = WorkspaceManager::new();
/// manager.create_workspace("my-project", "/path/to/project");
/// ```
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

/// A single workspace/project context
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Workspace {
    /// Unique workspace ID
    pub id: String,
    /// Display name
    pub name: String,
    /// Root directory path
    pub root_path: PathBuf,
    /// Recent files in this workspace
    pub recent_files: Vec<PathBuf>,
    /// Workspace-specific settings
    pub settings: HashMap<String, String>,
    /// Last accessed timestamp
    pub last_accessed: u64,
    /// Created timestamp
    pub created_at: u64,
}

impl Workspace {
    /// Create a new workspace
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::workspaces::Workspace;
    /// use std::path::PathBuf;
    ///
    /// let ws = Workspace::new("my-proj", "My Project", PathBuf::from("/path"));
    /// assert_eq!(ws.id, "my-proj");
    /// ```
    pub fn new(id: impl Into<String>, name: impl Into<String>, root_path: PathBuf) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            id: id.into(),
            name: name.into(),
            root_path,
            recent_files: Vec::new(),
            settings: HashMap::new(),
            last_accessed: now,
            created_at: now,
        }
    }

    /// Add a recent file
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::workspaces::Workspace;
    /// use std::path::PathBuf;
    ///
    /// let mut ws = Workspace::new("id", "name", PathBuf::from("/path"));
    /// ws.add_recent_file(PathBuf::from("/path/file.rs"));
    /// assert_eq!(ws.recent_files.len(), 1);
    /// ```
    pub fn add_recent_file(&mut self, path: PathBuf) {
        // Remove if already exists
        self.recent_files.retain(|p| p != &path);
        // Add to front
        self.recent_files.insert(0, path);
        // Limit to 20 recent files
        if self.recent_files.len() > 20 {
            self.recent_files.truncate(20);
        }
    }

    /// Set a workspace-specific setting
    pub fn set_setting(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.settings.insert(key.into(), value.into());
    }

    /// Get a workspace-specific setting
    pub fn get_setting(&self, key: &str) -> Option<&String> {
        self.settings.get(key)
    }

    /// Update last accessed time
    pub fn touch(&mut self) {
        self.last_accessed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
    }

    /// Check if path is within workspace
    pub fn contains_path(&self, path: &Path) -> bool {
        path.starts_with(&self.root_path)
    }

    /// Get age in seconds
    pub fn age_seconds(&self) -> u64 {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        now.saturating_sub(self.created_at)
    }

    /// Get time since last access in seconds
    pub fn idle_seconds(&self) -> u64 {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        now.saturating_sub(self.last_accessed)
    }
}

/// Manager for multiple workspaces
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkspaceManager {
    /// All workspaces by ID
    workspaces: HashMap<String, Workspace>,
    /// Currently active workspace ID
    active_workspace: Option<String>,
}

impl WorkspaceManager {
    /// Create a new workspace manager
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::workspaces::WorkspaceManager;
    ///
    /// let manager = WorkspaceManager::new();
    /// assert_eq!(manager.count(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            workspaces: HashMap::new(),
            active_workspace: None,
        }
    }

    /// Create a new workspace
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::workspaces::WorkspaceManager;
    ///
    /// let mut manager = WorkspaceManager::new();
    /// manager.create_workspace("proj", "My Project", "/path/to/project");
    /// assert_eq!(manager.count(), 1);
    /// ```
    pub fn create_workspace(
        &mut self,
        id: impl Into<String>,
        name: impl Into<String>,
        root_path: impl Into<PathBuf>,
    ) -> &Workspace {
        let id_str = id.into();
        let workspace = Workspace::new(id_str.clone(), name, root_path.into());
        self.workspaces.insert(id_str.clone(), workspace);
        // Safe: we just inserted this value above
        self.workspaces.get(&id_str).expect("workspace was just inserted")
    }

    /// Get workspace by ID
    pub fn get_workspace(&self, id: &str) -> Option<&Workspace> {
        self.workspaces.get(id)
    }

    /// Get mutable workspace by ID
    pub fn get_workspace_mut(&mut self, id: &str) -> Option<&mut Workspace> {
        self.workspaces.get_mut(id)
    }

    /// Get active workspace
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::workspaces::WorkspaceManager;
    ///
    /// let mut manager = WorkspaceManager::new();
    /// manager.create_workspace("proj", "My Project", "/path");
    /// manager.set_active("proj");
    /// assert!(manager.active_workspace().is_some());
    /// ```
    pub fn active_workspace(&self) -> Option<&Workspace> {
        self.active_workspace
            .as_ref()
            .and_then(|id| self.workspaces.get(id))
    }

    /// Get mutable active workspace
    pub fn active_workspace_mut(&mut self) -> Option<&mut Workspace> {
        let id = self.active_workspace.clone()?;
        self.workspaces.get_mut(&id)
    }

    /// Set active workspace
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::workspaces::WorkspaceManager;
    ///
    /// let mut manager = WorkspaceManager::new();
    /// manager.create_workspace("proj", "My Project", "/path");
    /// assert!(manager.set_active("proj"));
    /// ```
    pub fn set_active(&mut self, id: &str) -> bool {
        if self.workspaces.contains_key(id) {
            self.active_workspace = Some(id.to_string());
            if let Some(ws) = self.workspaces.get_mut(id) {
                ws.touch();
            }
            true
        } else {
            false
        }
    }

    /// Switch to next workspace
    pub fn next_workspace(&mut self) -> Option<&Workspace> {
        let mut ids: Vec<_> = self.workspaces.keys().cloned().collect();
        ids.sort();

        if ids.is_empty() {
            return None;
        }

        let next_id = match &self.active_workspace {
            None => ids[0].clone(),
            Some(current) => {
                let pos = ids.iter().position(|id| id == current)?;
                ids[(pos + 1) % ids.len()].clone()
            }
        };

        self.set_active(&next_id);
        self.active_workspace()
    }

    /// Switch to previous workspace
    pub fn previous_workspace(&mut self) -> Option<&Workspace> {
        let mut ids: Vec<_> = self.workspaces.keys().cloned().collect();
        ids.sort();

        if ids.is_empty() {
            return None;
        }

        let prev_id = match &self.active_workspace {
            None => ids[ids.len() - 1].clone(),
            Some(current) => {
                let pos = ids.iter().position(|id| id == current)?;
                ids[(pos + ids.len() - 1) % ids.len()].clone()
            }
        };

        self.set_active(&prev_id);
        self.active_workspace()
    }

    /// Remove a workspace
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::workspaces::WorkspaceManager;
    ///
    /// let mut manager = WorkspaceManager::new();
    /// manager.create_workspace("proj", "My Project", "/path");
    /// manager.remove_workspace("proj");
    /// assert_eq!(manager.count(), 0);
    /// ```
    pub fn remove_workspace(&mut self, id: &str) -> Option<Workspace> {
        let removed = self.workspaces.remove(id);

        // Clear active if it was the removed workspace
        if self.active_workspace.as_deref() == Some(id) {
            self.active_workspace = None;
        }

        removed
    }

    /// Get all workspaces
    pub fn all_workspaces(&self) -> Vec<&Workspace> {
        self.workspaces.values().collect()
    }

    /// Get workspaces sorted by last accessed (most recent first)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::workspaces::WorkspaceManager;
    ///
    /// let mut manager = WorkspaceManager::new();
    /// manager.create_workspace("a", "A", "/a");
    /// manager.create_workspace("b", "B", "/b");
    /// manager.set_active("b");
    ///
    /// let recent = manager.recent_workspaces();
    /// assert_eq!(recent[0].id, "b");
    /// ```
    pub fn recent_workspaces(&self) -> Vec<&Workspace> {
        let mut workspaces: Vec<_> = self.workspaces.values().collect();
        workspaces.sort_by(|a, b| b.last_accessed.cmp(&a.last_accessed));
        workspaces
    }

    /// Find workspace containing path
    pub fn find_by_path(&self, path: &Path) -> Option<&Workspace> {
        self.workspaces.values().find(|ws| ws.contains_path(path))
    }

    /// Get count of workspaces
    pub fn count(&self) -> usize {
        self.workspaces.len()
    }

    /// Clear all workspaces
    pub fn clear(&mut self) {
        self.workspaces.clear();
        self.active_workspace = None;
    }

    /// Load workspaces from JSON file
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use toad::workspaces::WorkspaceManager;
    ///
    /// let manager = WorkspaceManager::load_from_file("workspaces.json").unwrap();
    /// ```
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content =
            fs::read_to_string(path.as_ref()).context("Failed to read workspaces file")?;
        let manager: Self =
            serde_json::from_str(&content).context("Failed to parse workspaces JSON")?;
        Ok(manager)
    }

    /// Save workspaces to JSON file
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use toad::workspaces::WorkspaceManager;
    ///
    /// let manager = WorkspaceManager::new();
    /// manager.save_to_file("workspaces.json").unwrap();
    /// ```
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let json = serde_json::to_string_pretty(self).context("Failed to serialize workspaces")?;
        fs::write(path.as_ref(), json).context("Failed to write workspaces file")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workspace_creation() {
        let ws = Workspace::new("id", "My Project", PathBuf::from("/path"));
        assert_eq!(ws.id, "id");
        assert_eq!(ws.name, "My Project");
        assert_eq!(ws.root_path, PathBuf::from("/path"));
    }

    #[test]
    fn test_workspace_add_recent_file() {
        let mut ws = Workspace::new("id", "name", PathBuf::from("/path"));
        ws.add_recent_file(PathBuf::from("/path/file.rs"));
        assert_eq!(ws.recent_files.len(), 1);
    }

    #[test]
    fn test_workspace_recent_files_limit() {
        let mut ws = Workspace::new("id", "name", PathBuf::from("/path"));
        for i in 0..25 {
            ws.add_recent_file(PathBuf::from(format!("/path/file{}.rs", i)));
        }
        assert_eq!(ws.recent_files.len(), 20);
    }

    #[test]
    fn test_workspace_settings() {
        let mut ws = Workspace::new("id", "name", PathBuf::from("/path"));
        ws.set_setting("theme", "dark");
        assert_eq!(ws.get_setting("theme"), Some(&"dark".to_string()));
    }

    #[test]
    fn test_workspace_contains_path() {
        let ws = Workspace::new("id", "name", PathBuf::from("/project"));
        assert!(ws.contains_path(Path::new("/project/src/main.rs")));
        assert!(!ws.contains_path(Path::new("/other/file.rs")));
    }

    #[test]
    fn test_manager_creation() {
        let manager = WorkspaceManager::new();
        assert_eq!(manager.count(), 0);
    }

    #[test]
    fn test_manager_create_workspace() {
        let mut manager = WorkspaceManager::new();
        manager.create_workspace("proj", "My Project", "/path");
        assert_eq!(manager.count(), 1);
    }

    #[test]
    fn test_manager_get_workspace() {
        let mut manager = WorkspaceManager::new();
        manager.create_workspace("proj", "My Project", "/path");
        assert!(manager.get_workspace("proj").is_some());
        assert!(manager.get_workspace("nonexistent").is_none());
    }

    #[test]
    fn test_manager_set_active() {
        let mut manager = WorkspaceManager::new();
        manager.create_workspace("proj", "My Project", "/path");
        assert!(manager.set_active("proj"));
        assert!(manager.active_workspace().is_some());
    }

    #[test]
    fn test_manager_set_active_invalid() {
        let mut manager = WorkspaceManager::new();
        assert!(!manager.set_active("nonexistent"));
    }

    #[test]
    fn test_manager_next_workspace() {
        let mut manager = WorkspaceManager::new();
        manager.create_workspace("a", "A", "/a");
        manager.create_workspace("b", "B", "/b");
        manager.create_workspace("c", "C", "/c");

        manager.set_active("a");
        manager.next_workspace();
        assert_eq!(manager.active_workspace().unwrap().id, "b");

        manager.next_workspace();
        assert_eq!(manager.active_workspace().unwrap().id, "c");

        manager.next_workspace();
        assert_eq!(manager.active_workspace().unwrap().id, "a");
    }

    #[test]
    fn test_manager_previous_workspace() {
        let mut manager = WorkspaceManager::new();
        manager.create_workspace("a", "A", "/a");
        manager.create_workspace("b", "B", "/b");
        manager.create_workspace("c", "C", "/c");

        manager.set_active("a");
        manager.previous_workspace();
        assert_eq!(manager.active_workspace().unwrap().id, "c");
    }

    #[test]
    fn test_manager_remove_workspace() {
        let mut manager = WorkspaceManager::new();
        manager.create_workspace("proj", "My Project", "/path");
        manager.remove_workspace("proj");
        assert_eq!(manager.count(), 0);
    }

    #[test]
    fn test_manager_recent_workspaces() {
        let mut manager = WorkspaceManager::new();
        manager.create_workspace("a", "A", "/a");
        manager.create_workspace("b", "B", "/b");

        manager.set_active("b");
        std::thread::sleep(std::time::Duration::from_millis(10));
        manager.set_active("a");

        let recent = manager.recent_workspaces();
        assert_eq!(recent[0].id, "a");
        assert_eq!(recent[1].id, "b");
    }

    #[test]
    fn test_manager_find_by_path() {
        let mut manager = WorkspaceManager::new();
        manager.create_workspace("proj", "Project", "/project");

        let found = manager.find_by_path(Path::new("/project/src/main.rs"));
        assert!(found.is_some());
        assert_eq!(found.unwrap().id, "proj");
    }

    #[test]
    fn test_manager_clear() {
        let mut manager = WorkspaceManager::new();
        manager.create_workspace("proj", "Project", "/path");
        manager.clear();
        assert_eq!(manager.count(), 0);
    }

    #[test]
    fn test_workspace_touch() {
        let mut ws = Workspace::new("id", "name", PathBuf::from("/path"));
        let old_time = ws.last_accessed;
        std::thread::sleep(std::time::Duration::from_secs(1));
        ws.touch();
        assert!(ws.last_accessed > old_time);
    }

    #[test]
    fn test_workspace_age() {
        let ws = Workspace::new("id", "name", PathBuf::from("/path"));
        assert!(ws.age_seconds() >= 0);
    }

    #[test]
    fn test_workspace_idle() {
        let ws = Workspace::new("id", "name", PathBuf::from("/path"));
        assert!(ws.idle_seconds() >= 0);
    }

    #[test]
    fn test_serialization() {
        let manager = WorkspaceManager::new();
        let json = serde_json::to_string(&manager).unwrap();
        let deserialized: WorkspaceManager = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.count(), 0);
    }
}
