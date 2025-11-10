//! Session management for saving and restoring application state
//!
//! Provides functionality to persist and restore complete application sessions.
//!
//! # Examples
//!
//! ```
//! use toad::widgets::SessionManager;
//!
//! let mut manager = SessionManager::new();
//! manager.set_data("key", "value".to_string());
//!
//! let session = manager.save_session("my-session");
//! assert!(session.is_some());
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// Session data
///
/// Represents a saved application session with metadata.
///
/// # Examples
///
/// ```
/// use toad::widgets::SessionData;
///
/// let session = SessionData::new("my-session");
/// assert_eq!(session.name(), "my-session");
/// assert!(session.is_valid());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SessionData {
    /// Session name
    pub(super) name: String,
    /// Session data as key-value pairs
    pub(super) data: HashMap<String, String>,
    /// Session metadata
    pub(super) metadata: HashMap<String, String>,
    /// Timestamp of creation
    pub(super) created_at: u64,
    /// Timestamp of last update
    pub(super) updated_at: u64,
}

impl SessionData {
    /// Create a new session
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::SessionData;
    ///
    /// let session = SessionData::new("my-session");
    /// assert_eq!(session.name(), "my-session");
    /// ```
    pub fn new(name: impl Into<String>) -> Self {
        let now = Self::current_timestamp();
        Self {
            name: name.into(),
            data: HashMap::new(),
            metadata: HashMap::new(),
            created_at: now,
            updated_at: now,
        }
    }

    /// Get session name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get session data
    pub fn data(&self) -> &HashMap<String, String> {
        &self.data
    }

    /// Get session metadata
    pub fn metadata(&self) -> &HashMap<String, String> {
        &self.metadata
    }

    /// Get creation timestamp
    pub fn created_at(&self) -> u64 {
        self.created_at
    }

    /// Get last update timestamp
    pub fn updated_at(&self) -> u64 {
        self.updated_at
    }

    /// Set data value
    pub fn set_data(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.data.insert(key.into(), value.into());
        self.updated_at = Self::current_timestamp();
    }

    /// Get data value
    pub fn get_data(&self, key: &str) -> Option<&str> {
        self.data.get(key).map(|s| s.as_str())
    }

    /// Remove data value
    pub fn remove_data(&mut self, key: &str) -> Option<String> {
        let result = self.data.remove(key);
        if result.is_some() {
            self.updated_at = Self::current_timestamp();
        }
        result
    }

    /// Set metadata value
    pub fn set_metadata(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.metadata.insert(key.into(), value.into());
        self.updated_at = Self::current_timestamp();
    }

    /// Get metadata value
    pub fn get_metadata(&self, key: &str) -> Option<&str> {
        self.metadata.get(key).map(|s| s.as_str())
    }

    /// Check if session is valid (not corrupted)
    pub fn is_valid(&self) -> bool {
        !self.name.is_empty() && self.created_at <= self.updated_at
    }

    /// Get current Unix timestamp
    fn current_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }
}

/// Session manager
///
/// Manages multiple sessions with save/load/switch functionality.
///
/// # Examples
///
/// ```
/// use toad::widgets::SessionManager;
///
/// let mut manager = SessionManager::new();
/// manager.set_data("key", "value".to_string());
///
/// let session = manager.save_session("my-session");
/// assert!(session.is_some());
///
/// manager.load_session("my-session");
/// assert_eq!(manager.get_data("key"), Some("value"));
/// ```
#[derive(Debug, Clone, Default)]
pub struct SessionManager {
    /// Current session data
    pub(super) current_data: HashMap<String, String>,
    /// Current session metadata
    pub(super) current_metadata: HashMap<String, String>,
    /// Saved sessions
    pub(super) sessions: HashMap<String, SessionData>,
    /// Active session name
    pub(super) active_session: Option<String>,
    /// Auto-save enabled
    pub(super) auto_save: bool,
}

impl SessionManager {
    /// Create a new session manager
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::SessionManager;
    ///
    /// let manager = SessionManager::new();
    /// assert_eq!(manager.session_count(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            current_data: HashMap::new(),
            current_metadata: HashMap::new(),
            sessions: HashMap::new(),
            active_session: None,
            auto_save: false,
        }
    }

    /// Enable or disable auto-save
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::SessionManager;
    ///
    /// let mut manager = SessionManager::new();
    /// manager.set_auto_save(true);
    /// ```
    pub fn set_auto_save(&mut self, enabled: bool) {
        self.auto_save = enabled;
    }

    /// Check if auto-save is enabled
    pub fn auto_save(&self) -> bool {
        self.auto_save
    }

    /// Set current data
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::SessionManager;
    ///
    /// let mut manager = SessionManager::new();
    /// manager.set_data("key", "value".to_string());
    /// assert_eq!(manager.get_data("key"), Some("value"));
    /// ```
    pub fn set_data(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.current_data.insert(key.into(), value.into());
        if self.auto_save
            && let Some(name) = &self.active_session.clone()
        {
            self.save_session(name);
        }
    }

    /// Get current data
    pub fn get_data(&self, key: &str) -> Option<&str> {
        self.current_data.get(key).map(|s| s.as_str())
    }

    /// Remove current data
    pub fn remove_data(&mut self, key: &str) -> Option<String> {
        self.current_data.remove(key)
    }

    /// Clear current data
    pub fn clear_data(&mut self) {
        self.current_data.clear();
    }

    /// Set current metadata
    pub fn set_metadata(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.current_metadata.insert(key.into(), value.into());
    }

    /// Get current metadata
    pub fn get_metadata(&self, key: &str) -> Option<&str> {
        self.current_metadata.get(key).map(|s| s.as_str())
    }

    /// Save current state as a session
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::SessionManager;
    ///
    /// let mut manager = SessionManager::new();
    /// manager.set_data("key", "value".to_string());
    ///
    /// let session = manager.save_session("my-session");
    /// assert!(session.is_some());
    /// assert_eq!(manager.session_count(), 1);
    /// ```
    pub fn save_session(&mut self, name: impl Into<String>) -> Option<SessionData> {
        let name = name.into();
        let mut session = SessionData::new(name.clone());

        for (k, v) in &self.current_data {
            session.set_data(k, v);
        }

        for (k, v) in &self.current_metadata {
            session.set_metadata(k, v);
        }

        self.sessions.insert(name.clone(), session.clone());
        self.active_session = Some(name);

        Some(session)
    }

    /// Load a session by name
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::SessionManager;
    ///
    /// let mut manager = SessionManager::new();
    /// manager.set_data("key", "value".to_string());
    /// manager.save_session("my-session");
    ///
    /// manager.clear_data();
    /// assert!(manager.load_session("my-session"));
    /// assert_eq!(manager.get_data("key"), Some("value"));
    /// ```
    pub fn load_session(&mut self, name: &str) -> bool {
        if let Some(session) = self.sessions.get(name) {
            self.current_data = session.data().clone();
            self.current_metadata = session.metadata().clone();
            self.active_session = Some(name.to_string());
            true
        } else {
            false
        }
    }

    /// Delete a session
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::SessionManager;
    ///
    /// let mut manager = SessionManager::new();
    /// manager.save_session("my-session");
    /// assert_eq!(manager.session_count(), 1);
    ///
    /// assert!(manager.delete_session("my-session"));
    /// assert_eq!(manager.session_count(), 0);
    /// ```
    pub fn delete_session(&mut self, name: &str) -> bool {
        let result = self.sessions.remove(name).is_some();
        if result && self.active_session.as_deref() == Some(name) {
            self.active_session = None;
        }
        result
    }

    /// Get session by name
    pub fn get_session(&self, name: &str) -> Option<&SessionData> {
        self.sessions.get(name)
    }

    /// Get all session names
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::SessionManager;
    ///
    /// let mut manager = SessionManager::new();
    /// manager.save_session("session1");
    /// manager.save_session("session2");
    ///
    /// let names = manager.session_names();
    /// assert_eq!(names.len(), 2);
    /// ```
    pub fn session_names(&self) -> Vec<&str> {
        self.sessions.keys().map(|s| s.as_str()).collect()
    }

    /// Get number of saved sessions
    pub fn session_count(&self) -> usize {
        self.sessions.len()
    }

    /// Get active session name
    pub fn active_session(&self) -> Option<&str> {
        self.active_session.as_deref()
    }

    /// Check if a session exists
    pub fn has_session(&self, name: &str) -> bool {
        self.sessions.contains_key(name)
    }

    /// Clear all sessions
    pub fn clear_sessions(&mut self) {
        self.sessions.clear();
        self.active_session = None;
    }

    /// Rename a session
    pub fn rename_session(&mut self, old_name: &str, new_name: impl Into<String>) -> bool {
        if let Some(mut session) = self.sessions.remove(old_name) {
            let new_name = new_name.into();
            session.name = new_name.clone();
            session.updated_at = SessionData::current_timestamp();
            self.sessions.insert(new_name.clone(), session);

            if self.active_session.as_deref() == Some(old_name) {
                self.active_session = Some(new_name);
            }
            true
        } else {
            false
        }
    }

    /// Export session as JSON
    pub fn export_session(&self, name: &str) -> Result<String, String> {
        self.sessions
            .get(name)
            .ok_or_else(|| format!("Session '{}' not found", name))
            .and_then(|session| {
                serde_json::to_string_pretty(session)
                    .map_err(|e| format!("Serialization error: {}", e))
            })
    }

    /// Import session from JSON
    pub fn import_session(&mut self, json: &str) -> Result<String, String> {
        let session: SessionData =
            serde_json::from_str(json).map_err(|e| format!("Deserialization error: {}", e))?;

        if !session.is_valid() {
            return Err("Invalid session data".to_string());
        }

        let name = session.name().to_string();
        self.sessions.insert(name.clone(), session);
        Ok(name)
    }
}
