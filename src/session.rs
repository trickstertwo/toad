/// Session state persistence
///
/// Save and restore application session state including window layout,
/// recent files, and other session-specific data.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Session state that can be saved and restored
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    /// Session name/identifier
    pub name: String,

    /// Working directory when session was saved
    pub working_directory: PathBuf,

    /// Vim mode enabled
    pub vim_mode: bool,

    /// Performance overlay shown
    pub show_performance: bool,

    /// Recent files opened in this session
    pub recent_files: Vec<PathBuf>,

    /// Last command executed
    pub last_command: Option<String>,
}

impl Session {
    /// Create a new session
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            working_directory: std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/")),
            vim_mode: false,
            show_performance: false,
            recent_files: Vec::new(),
            last_command: None,
        }
    }

    /// Get default session file path
    pub fn default_path() -> PathBuf {
        if let Some(data_dir) = dirs::data_dir() {
            data_dir.join("toad").join("session.json")
        } else {
            PathBuf::from(".toad_session.json")
        }
    }

    /// Save session to file
    pub fn save_to_file(&self, path: &PathBuf) -> anyhow::Result<()> {
        // Create parent directory if it doesn't exist
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// Load session from file
    pub fn load_from_file(path: &PathBuf) -> anyhow::Result<Self> {
        let contents = std::fs::read_to_string(path)?;
        let session = serde_json::from_str(&contents)?;
        Ok(session)
    }

    /// Save session to default location
    pub fn save(&self) -> anyhow::Result<()> {
        let path = Self::default_path();
        self.save_to_file(&path)
    }

    /// Load session from default location
    pub fn load() -> anyhow::Result<Self> {
        let path = Self::default_path();
        Self::load_from_file(&path)
    }

    /// Load session or create default
    pub fn load_or_default() -> Self {
        let path = Self::default_path();
        Self::load_from_file(&path).unwrap_or_else(|_| Self::new("default"))
    }

    /// Add a file to recent files list
    pub fn add_recent_file(&mut self, file: PathBuf) {
        // Remove if already exists
        self.recent_files.retain(|f| f != &file);

        // Add to front
        self.recent_files.insert(0, file);

        // Keep only last 20 files
        if self.recent_files.len() > 20 {
            self.recent_files.truncate(20);
        }
    }

    /// Get recent files
    pub fn recent_files(&self) -> &[PathBuf] {
        &self.recent_files
    }

    /// Clear recent files
    pub fn clear_recent_files(&mut self) {
        self.recent_files.clear();
    }
}

impl Default for Session {
    fn default() -> Self {
        Self::new("default")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_session_creation() {
        let session = Session::new("test");
        assert_eq!(session.name, "test");
        assert!(!session.vim_mode);
        assert_eq!(session.recent_files.len(), 0);
    }

    #[test]
    fn test_session_save_load() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("session.json");

        let mut session = Session::new("test");
        session.vim_mode = true;
        session.add_recent_file(PathBuf::from("/tmp/file1.txt"));
        session.add_recent_file(PathBuf::from("/tmp/file2.txt"));

        // Save
        session.save_to_file(&file_path).unwrap();
        assert!(file_path.exists());

        // Load
        let loaded = Session::load_from_file(&file_path).unwrap();
        assert_eq!(loaded.name, "test");
        assert!(loaded.vim_mode);
        assert_eq!(loaded.recent_files.len(), 2);
        assert_eq!(loaded.recent_files[0], PathBuf::from("/tmp/file2.txt")); // Most recent first
    }

    #[test]
    fn test_recent_files_limit() {
        let mut session = Session::new("test");

        // Add 25 files (should keep only 20)
        for i in 0..25 {
            session.add_recent_file(PathBuf::from(format!("/tmp/file{}.txt", i)));
        }

        assert_eq!(session.recent_files.len(), 20);
        // Most recent should be file24
        assert_eq!(session.recent_files[0], PathBuf::from("/tmp/file24.txt"));
    }

    #[test]
    fn test_recent_files_no_duplicates() {
        let mut session = Session::new("test");
        let file = PathBuf::from("/tmp/file.txt");

        session.add_recent_file(file.clone());
        session.add_recent_file(file.clone());
        session.add_recent_file(file.clone());

        assert_eq!(session.recent_files.len(), 1);
    }

    #[test]
    fn test_clear_recent_files() {
        let mut session = Session::new("test");
        session.add_recent_file(PathBuf::from("/tmp/file.txt"));
        assert_eq!(session.recent_files.len(), 1);

        session.clear_recent_files();
        assert_eq!(session.recent_files.len(), 0);
    }
}
