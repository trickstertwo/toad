//! Session state persistence
//!
//! Handles saving and restoring application session state across runs.
//! Includes working directory, history, and UI state.
//!
//! # Examples
//!
//! ```
//! use toad::session::SessionState;
//!
//! // Create new session
//! let session = SessionState::new();
//! assert!(!session.welcome_shown());
//! ```
//!
//! # Session File Format
//!
//! Session state is persisted in JSON format at `~/.config/toad/session.json`
//! or `$XDG_CONFIG_HOME/toad/session.json` on Unix-like systems,
//! and `%APPDATA%\toad\session.json` on Windows.

use crate::history::History;
use color_eyre::{eyre::Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Application session state that persists across runs
///
/// Stores session-specific state like the current directory, command history,
/// and UI preferences that should be restored when the application restarts.
///
/// # Examples
///
/// ```
/// use toad::session::SessionState;
/// use std::path::PathBuf;
///
/// let mut session = SessionState::new();
/// session.set_working_directory(PathBuf::from("/home/user/project"));
/// session.set_welcome_shown(true);
///
/// assert!(session.welcome_shown());
/// assert_eq!(session.working_directory(), &PathBuf::from("/home/user/project"));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionState {
    /// Whether the welcome screen has been shown
    welcome_shown: bool,

    /// Last working directory
    working_directory: PathBuf,

    /// Last active screen before exit
    last_screen: String,

    /// Number of installed plugins
    plugin_count: usize,

    /// Command history
    history: History,

    /// Version of the session format (for migration)
    #[serde(default = "default_version")]
    version: u32,
}

fn default_version() -> u32 {
    1
}

impl Default for SessionState {
    fn default() -> Self {
        Self::new()
    }
}

impl SessionState {
    /// Create a new empty session state
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::session::SessionState;
    ///
    /// let session = SessionState::new();
    /// assert!(!session.welcome_shown());
    /// assert_eq!(session.plugin_count(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            welcome_shown: false,
            working_directory: std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/")),
            last_screen: "Welcome".to_string(),
            plugin_count: 0,
            history: History::new(1000),
            version: 1,
        }
    }

    /// Check if welcome screen has been shown
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::session::SessionState;
    ///
    /// let mut session = SessionState::new();
    /// assert!(!session.welcome_shown());
    ///
    /// session.set_welcome_shown(true);
    /// assert!(session.welcome_shown());
    /// ```
    pub fn welcome_shown(&self) -> bool {
        self.welcome_shown
    }

    /// Set whether welcome screen has been shown
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::session::SessionState;
    ///
    /// let mut session = SessionState::new();
    /// session.set_welcome_shown(true);
    /// assert!(session.welcome_shown());
    /// ```
    pub fn set_welcome_shown(&mut self, shown: bool) {
        self.welcome_shown = shown;
    }

    /// Get the working directory
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::session::SessionState;
    /// use std::path::PathBuf;
    ///
    /// let mut session = SessionState::new();
    /// session.set_working_directory(PathBuf::from("/tmp"));
    /// assert_eq!(session.working_directory(), &PathBuf::from("/tmp"));
    /// ```
    pub fn working_directory(&self) -> &PathBuf {
        &self.working_directory
    }

    /// Set the working directory
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::session::SessionState;
    /// use std::path::PathBuf;
    ///
    /// let mut session = SessionState::new();
    /// session.set_working_directory(PathBuf::from("/home/user"));
    /// assert_eq!(session.working_directory(), &PathBuf::from("/home/user"));
    /// ```
    pub fn set_working_directory(&mut self, dir: PathBuf) {
        self.working_directory = dir;
    }

    /// Get the last active screen
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::session::SessionState;
    ///
    /// let session = SessionState::new();
    /// assert_eq!(session.last_screen(), "Welcome");
    /// ```
    pub fn last_screen(&self) -> &str {
        &self.last_screen
    }

    /// Set the last active screen
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::session::SessionState;
    ///
    /// let mut session = SessionState::new();
    /// session.set_last_screen("Main".to_string());
    /// assert_eq!(session.last_screen(), "Main");
    /// ```
    pub fn set_last_screen(&mut self, screen: String) {
        self.last_screen = screen;
    }

    /// Get the plugin count
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::session::SessionState;
    ///
    /// let mut session = SessionState::new();
    /// assert_eq!(session.plugin_count(), 0);
    ///
    /// session.set_plugin_count(5);
    /// assert_eq!(session.plugin_count(), 5);
    /// ```
    pub fn plugin_count(&self) -> usize {
        self.plugin_count
    }

    /// Set the plugin count
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::session::SessionState;
    ///
    /// let mut session = SessionState::new();
    /// session.set_plugin_count(3);
    /// assert_eq!(session.plugin_count(), 3);
    /// ```
    pub fn set_plugin_count(&mut self, count: usize) {
        self.plugin_count = count;
    }

    /// Get the command history
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::session::SessionState;
    ///
    /// let session = SessionState::new();
    /// assert_eq!(session.history().len(), 0);
    /// ```
    pub fn history(&self) -> &History {
        &self.history
    }

    /// Get mutable command history
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::session::SessionState;
    ///
    /// let mut session = SessionState::new();
    /// session.history_mut().add("test command".to_string());
    /// assert_eq!(session.history().len(), 1);
    /// ```
    pub fn history_mut(&mut self) -> &mut History {
        &mut self.history
    }

    /// Get the session format version
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::session::SessionState;
    ///
    /// let session = SessionState::new();
    /// assert_eq!(session.version(), 1);
    /// ```
    pub fn version(&self) -> u32 {
        self.version
    }

    /// Load session state from file
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use toad::session::SessionState;
    /// use std::path::Path;
    ///
    /// let session = SessionState::load(Path::new("session.json")).unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - File cannot be read
    /// - JSON parsing fails
    /// - Session version is incompatible
    pub fn load(path: &Path) -> Result<Self> {
        let contents = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read session file: {}", path.display()))?;

        let session: SessionState = serde_json::from_str(&contents)
            .with_context(|| format!("Failed to parse session file: {}", path.display()))?;

        // Check version compatibility
        if session.version > 1 {
            return Err(color_eyre::eyre::eyre!(
                "Session file version {} is not supported (max version: 1)",
                session.version
            ));
        }

        Ok(session)
    }

    /// Save session state to file
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use toad::session::SessionState;
    /// use std::path::Path;
    ///
    /// let session = SessionState::new();
    /// session.save(Path::new("session.json")).unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Parent directory cannot be created
    /// - File cannot be written
    /// - JSON serialization fails
    pub fn save(&self, path: &Path) -> Result<()> {
        let contents =
            serde_json::to_string_pretty(self).context("Failed to serialize session state")?;

        // Create parent directory if needed
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
        }

        std::fs::write(path, contents)
            .with_context(|| format!("Failed to write session file: {}", path.display()))?;

        Ok(())
    }

    /// Get the default session file path
    ///
    /// Returns `~/.config/toad/session.json` on Unix-like systems,
    /// or `%APPDATA%\toad\session.json` on Windows.
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::session::SessionState;
    ///
    /// let path = SessionState::default_path();
    /// assert!(path.ends_with("toad/session.json") || path.ends_with("toad\\session.json"));
    /// ```
    pub fn default_path() -> PathBuf {
        let config_dir = if cfg!(target_os = "windows") {
            std::env::var("APPDATA")
                .map(PathBuf::from)
                .unwrap_or_else(|_| PathBuf::from("."))
        } else {
            std::env::var("XDG_CONFIG_HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|_| {
                    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
                    PathBuf::from(home).join(".config")
                })
        };

        config_dir.join("toad").join("session.json")
    }

    /// Load session from default path, or create new if file doesn't exist
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::session::SessionState;
    ///
    /// let session = SessionState::load_or_new();
    /// assert_eq!(session.version(), 1);
    /// ```
    pub fn load_or_new() -> Self {
        let path = Self::default_path();

        if path.exists() {
            Self::load(&path).unwrap_or_else(|_| Self::new())
        } else {
            Self::new()
        }
    }

    /// Auto-save session to default path
    ///
    /// Convenience method for saving to the default session file location.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use toad::session::SessionState;
    ///
    /// let session = SessionState::new();
    /// session.auto_save().unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if saving fails (see [`SessionState::save`])
    pub fn auto_save(&self) -> Result<()> {
        let path = Self::default_path();
        self.save(&path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_session_creation() {
        let session = SessionState::new();
        assert!(!session.welcome_shown());
        assert_eq!(session.plugin_count(), 0);
        assert_eq!(session.version(), 1);
        assert_eq!(session.last_screen(), "Welcome");
    }

    #[test]
    fn test_welcome_shown() {
        let mut session = SessionState::new();
        assert!(!session.welcome_shown());

        session.set_welcome_shown(true);
        assert!(session.welcome_shown());

        session.set_welcome_shown(false);
        assert!(!session.welcome_shown());
    }

    #[test]
    fn test_working_directory() {
        let mut session = SessionState::new();
        let dir = PathBuf::from("/home/user/project");

        session.set_working_directory(dir.clone());
        assert_eq!(session.working_directory(), &dir);
    }

    #[test]
    fn test_last_screen() {
        let mut session = SessionState::new();
        assert_eq!(session.last_screen(), "Welcome");

        session.set_last_screen("Main".to_string());
        assert_eq!(session.last_screen(), "Main");

        session.set_last_screen("TrustDialog".to_string());
        assert_eq!(session.last_screen(), "TrustDialog");
    }

    #[test]
    fn test_plugin_count() {
        let mut session = SessionState::new();
        assert_eq!(session.plugin_count(), 0);

        session.set_plugin_count(5);
        assert_eq!(session.plugin_count(), 5);

        session.set_plugin_count(10);
        assert_eq!(session.plugin_count(), 10);
    }

    #[test]
    fn test_history() {
        let mut session = SessionState::new();
        assert_eq!(session.history().len(), 0);

        session.history_mut().add("test command".to_string());
        assert_eq!(session.history().len(), 1);

        session.history_mut().add("another command".to_string());
        assert_eq!(session.history().len(), 2);
    }

    #[test]
    fn test_serialization() {
        let mut session = SessionState::new();
        session.set_welcome_shown(true);
        session.set_working_directory(PathBuf::from("/tmp"));
        session.set_last_screen("Main".to_string());
        session.set_plugin_count(3);
        session.history_mut().add("test".to_string());

        let json = serde_json::to_string(&session).unwrap();
        assert!(json.contains("\"welcome_shown\":true"));
        assert!(json.contains("\"working_directory\":\"/tmp\""));
        assert!(json.contains("\"last_screen\":\"Main\""));
        assert!(json.contains("\"plugin_count\":3"));
    }

    #[test]
    fn test_deserialization() {
        let json = r#"{
            "welcome_shown": true,
            "working_directory": "/home/user",
            "last_screen": "Main",
            "plugin_count": 5,
            "history": {
                "max_size": 1000,
                "entries": ["cmd1", "cmd2"]
            },
            "version": 1
        }"#;

        let session: SessionState = serde_json::from_str(json).unwrap();
        assert!(session.welcome_shown());
        assert_eq!(session.working_directory(), &PathBuf::from("/home/user"));
        assert_eq!(session.last_screen(), "Main");
        assert_eq!(session.plugin_count(), 5);
        assert_eq!(session.history().len(), 2);
        assert_eq!(session.version(), 1);
    }

    #[test]
    fn test_default_path() {
        let path = SessionState::default_path();
        let path_str = path.to_string_lossy();

        assert!(path_str.contains("toad"));
        assert!(path_str.contains("session.json"));
    }

    #[test]
    fn test_load_or_new() {
        // Should return new session when file doesn't exist
        let session = SessionState::load_or_new();
        assert_eq!(session.version(), 1);
    }

    #[test]
    fn test_version_compatibility() {
        let json = r#"{
            "welcome_shown": false,
            "working_directory": "/",
            "last_screen": "Welcome",
            "plugin_count": 0,
            "history": {
                "max_size": 1000,
                "entries": []
            },
            "version": 999
        }"#;

        let _session: SessionState = serde_json::from_str(json).unwrap();

        // Try to load it (simulating file read)
        let temp_file = std::env::temp_dir().join("test_session_version.json");
        std::fs::write(&temp_file, json).unwrap();

        let result = SessionState::load(&temp_file);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("version 999 is not supported"));

        // Clean up
        let _ = std::fs::remove_file(&temp_file);
    }
}
