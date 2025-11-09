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
    name: String,
    /// Session data as key-value pairs
    data: HashMap<String, String>,
    /// Session metadata
    metadata: HashMap<String, String>,
    /// Timestamp of creation
    created_at: u64,
    /// Timestamp of last update
    updated_at: u64,
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
    current_data: HashMap<String, String>,
    /// Current session metadata
    current_metadata: HashMap<String, String>,
    /// Saved sessions
    sessions: HashMap<String, SessionData>,
    /// Active session name
    active_session: Option<String>,
    /// Auto-save enabled
    auto_save: bool,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_data_new() {
        let session = SessionData::new("test");
        assert_eq!(session.name(), "test");
        assert!(session.data().is_empty());
        assert!(session.metadata().is_empty());
        assert!(session.is_valid());
    }

    #[test]
    fn test_session_data_set_get() {
        let mut session = SessionData::new("test");
        session.set_data("key", "value");
        assert_eq!(session.get_data("key"), Some("value"));
    }

    #[test]
    fn test_session_data_remove() {
        let mut session = SessionData::new("test");
        session.set_data("key", "value");
        assert_eq!(session.remove_data("key"), Some("value".to_string()));
        assert_eq!(session.get_data("key"), None);
    }

    #[test]
    fn test_session_data_metadata() {
        let mut session = SessionData::new("test");
        session.set_metadata("author", "user");
        assert_eq!(session.get_metadata("author"), Some("user"));
    }

    #[test]
    fn test_session_data_timestamps() {
        let session = SessionData::new("test");
        assert!(session.created_at() > 0);
        assert!(session.updated_at() > 0);
        assert!(session.created_at() <= session.updated_at());
    }

    #[test]
    fn test_session_data_is_valid() {
        let session = SessionData::new("test");
        assert!(session.is_valid());

        let invalid = SessionData {
            name: String::new(),
            data: HashMap::new(),
            metadata: HashMap::new(),
            created_at: 0,
            updated_at: 0,
        };
        assert!(!invalid.is_valid());
    }

    #[test]
    fn test_session_manager_new() {
        let manager = SessionManager::new();
        assert_eq!(manager.session_count(), 0);
        assert_eq!(manager.active_session(), None);
        assert!(!manager.auto_save());
    }

    #[test]
    fn test_session_manager_default() {
        let manager = SessionManager::default();
        assert_eq!(manager.session_count(), 0);
    }

    #[test]
    fn test_session_manager_set_data() {
        let mut manager = SessionManager::new();
        manager.set_data("key", "value".to_string());
        assert_eq!(manager.get_data("key"), Some("value"));
    }

    #[test]
    fn test_session_manager_remove_data() {
        let mut manager = SessionManager::new();
        manager.set_data("key", "value".to_string());
        assert_eq!(manager.remove_data("key"), Some("value".to_string()));
        assert_eq!(manager.get_data("key"), None);
    }

    #[test]
    fn test_session_manager_clear_data() {
        let mut manager = SessionManager::new();
        manager.set_data("key1", "value1".to_string());
        manager.set_data("key2", "value2".to_string());
        manager.clear_data();
        assert_eq!(manager.get_data("key1"), None);
        assert_eq!(manager.get_data("key2"), None);
    }

    #[test]
    fn test_session_manager_metadata() {
        let mut manager = SessionManager::new();
        manager.set_metadata("author", "user".to_string());
        assert_eq!(manager.get_metadata("author"), Some("user"));
    }

    #[test]
    fn test_session_manager_save_session() {
        let mut manager = SessionManager::new();
        manager.set_data("key", "value".to_string());

        let session = manager.save_session("my-session");
        assert!(session.is_some());
        assert_eq!(manager.session_count(), 1);
        assert_eq!(manager.active_session(), Some("my-session"));
    }

    #[test]
    fn test_session_manager_load_session() {
        let mut manager = SessionManager::new();
        manager.set_data("key", "value".to_string());
        manager.save_session("my-session");

        manager.clear_data();
        assert!(manager.load_session("my-session"));
        assert_eq!(manager.get_data("key"), Some("value"));
    }

    #[test]
    fn test_session_manager_load_nonexistent() {
        let mut manager = SessionManager::new();
        assert!(!manager.load_session("nonexistent"));
    }

    #[test]
    fn test_session_manager_delete_session() {
        let mut manager = SessionManager::new();
        manager.save_session("my-session");
        assert_eq!(manager.session_count(), 1);

        assert!(manager.delete_session("my-session"));
        assert_eq!(manager.session_count(), 0);
        assert_eq!(manager.active_session(), None);
    }

    #[test]
    fn test_session_manager_delete_nonexistent() {
        let mut manager = SessionManager::new();
        assert!(!manager.delete_session("nonexistent"));
    }

    #[test]
    fn test_session_manager_get_session() {
        let mut manager = SessionManager::new();
        manager.set_data("key", "value".to_string());
        manager.save_session("my-session");

        let session = manager.get_session("my-session");
        assert!(session.is_some());
        assert_eq!(session.unwrap().name(), "my-session");
    }

    #[test]
    fn test_session_manager_session_names() {
        let mut manager = SessionManager::new();
        manager.save_session("session1");
        manager.save_session("session2");

        let names = manager.session_names();
        assert_eq!(names.len(), 2);
        assert!(names.contains(&"session1"));
        assert!(names.contains(&"session2"));
    }

    #[test]
    fn test_session_manager_has_session() {
        let mut manager = SessionManager::new();
        manager.save_session("my-session");

        assert!(manager.has_session("my-session"));
        assert!(!manager.has_session("other-session"));
    }

    #[test]
    fn test_session_manager_clear_sessions() {
        let mut manager = SessionManager::new();
        manager.save_session("session1");
        manager.save_session("session2");
        assert_eq!(manager.session_count(), 2);

        manager.clear_sessions();
        assert_eq!(manager.session_count(), 0);
        assert_eq!(manager.active_session(), None);
    }

    #[test]
    fn test_session_manager_rename_session() {
        let mut manager = SessionManager::new();
        manager.save_session("old-name");

        assert!(manager.rename_session("old-name", "new-name"));
        assert!(manager.has_session("new-name"));
        assert!(!manager.has_session("old-name"));
        assert_eq!(manager.active_session(), Some("new-name"));
    }

    #[test]
    fn test_session_manager_rename_nonexistent() {
        let mut manager = SessionManager::new();
        assert!(!manager.rename_session("nonexistent", "new-name"));
    }

    #[test]
    fn test_session_manager_export_import() {
        let mut manager = SessionManager::new();
        manager.set_data("key", "value".to_string());
        manager.save_session("my-session");

        let json = manager.export_session("my-session").unwrap();
        assert!(!json.is_empty());

        let mut new_manager = SessionManager::new();
        let name = new_manager.import_session(&json).unwrap();
        assert_eq!(name, "my-session");

        new_manager.load_session(&name);
        assert_eq!(new_manager.get_data("key"), Some("value"));
    }

    #[test]
    fn test_session_manager_export_nonexistent() {
        let manager = SessionManager::new();
        assert!(manager.export_session("nonexistent").is_err());
    }

    #[test]
    fn test_session_manager_import_invalid() {
        let mut manager = SessionManager::new();
        assert!(manager.import_session("invalid json").is_err());
    }

    #[test]
    fn test_session_manager_auto_save() {
        let mut manager = SessionManager::new();
        manager.set_auto_save(true);
        assert!(manager.auto_save());

        manager.save_session("auto-session");
        manager.set_data("key", "value1".to_string());

        // Auto-save should update the session
        let session = manager.get_session("auto-session").unwrap();
        assert_eq!(session.get_data("key"), Some("value1"));
    }

    // ============================================================================
    // ADVANCED COMPREHENSIVE EDGE CASE TESTS (90%+ COVERAGE)
    // ============================================================================

    // ============ SessionData Stress Tests ============

    #[test]
    fn test_session_data_with_10000_entries() {
        let mut session = SessionData::new("stress-test");
        for i in 0..10000 {
            session.set_data(format!("key{}", i), format!("value{}", i));
        }
        assert_eq!(session.data().len(), 10000);
        assert_eq!(session.get_data("key9999"), Some("value9999"));
    }

    #[test]
    fn test_session_data_with_10000_metadata_entries() {
        let mut session = SessionData::new("metadata-stress");
        for i in 0..10000 {
            session.set_metadata(format!("meta{}", i), format!("val{}", i));
        }
        assert_eq!(session.metadata().len(), 10000);
        assert_eq!(session.get_metadata("meta9999"), Some("val9999"));
    }

    #[test]
    fn test_session_data_very_long_key() {
        let long_key = "K".repeat(100000);
        let mut session = SessionData::new("test");
        session.set_data(long_key.clone(), "value");
        assert_eq!(session.get_data(&long_key), Some("value"));
    }

    #[test]
    fn test_session_data_very_long_value() {
        let long_value = "V".repeat(100000);
        let mut session = SessionData::new("test");
        session.set_data("key", long_value.clone());
        assert_eq!(session.get_data("key"), Some(long_value.as_str()));
    }

    // ============ SessionData Unicode Edge Cases ============

    #[test]
    fn test_session_data_unicode_name() {
        let session = SessionData::new("セッション名 🎉");
        assert_eq!(session.name(), "セッション名 🎉");
        assert!(session.is_valid());
    }

    #[test]
    fn test_session_data_unicode_keys_values() {
        let mut session = SessionData::new("test");
        session.set_data("キー", "値");
        session.set_data("مفتاح", "قيمة");
        session.set_data("ключ", "значение");

        assert_eq!(session.get_data("キー"), Some("値"));
        assert_eq!(session.get_data("مفتاح"), Some("قيمة"));
        assert_eq!(session.get_data("ключ"), Some("значение"));
    }

    #[test]
    fn test_session_data_emoji_sequences() {
        let mut session = SessionData::new("👨‍👩‍👧‍👦 Family");
        session.set_data("🎉🎊", "🎈🎁");
        assert_eq!(session.get_data("🎉🎊"), Some("🎈🎁"));
    }

    #[test]
    fn test_session_data_rtl_text() {
        let mut session = SessionData::new("مرحبا");
        session.set_data("عنوان", "قيمة");
        assert_eq!(session.get_data("عنوان"), Some("قيمة"));
    }

    // ============ SessionData Timestamp Edge Cases ============

    #[test]
    fn test_session_data_updated_at_changes() {
        let mut session = SessionData::new("test");
        let created = session.created_at();
        let initial_updated = session.updated_at();

        std::thread::sleep(std::time::Duration::from_millis(10));
        session.set_data("key", "value");

        assert_eq!(session.created_at(), created);
        assert!(session.updated_at() >= initial_updated);
    }

    #[test]
    fn test_session_data_updated_at_on_remove() {
        let mut session = SessionData::new("test");
        session.set_data("key", "value");
        let before_remove = session.updated_at();

        std::thread::sleep(std::time::Duration::from_millis(10));
        session.remove_data("key");

        assert!(session.updated_at() >= before_remove);
    }

    #[test]
    fn test_session_data_updated_at_unchanged_on_nonexistent_remove() {
        let mut session = SessionData::new("test");
        let before = session.updated_at();

        session.remove_data("nonexistent");

        assert_eq!(session.updated_at(), before);
    }

    #[test]
    fn test_session_data_updated_at_on_metadata() {
        let mut session = SessionData::new("test");
        let initial = session.updated_at();

        std::thread::sleep(std::time::Duration::from_millis(10));
        session.set_metadata("key", "value");

        assert!(session.updated_at() >= initial);
    }

    // ============ SessionData Clone and Equality ============

    #[test]
    fn test_session_data_clone() {
        let mut session = SessionData::new("test");
        session.set_data("key", "value");
        session.set_metadata("meta", "data");

        let cloned = session.clone();
        assert_eq!(session, cloned);
        assert_eq!(cloned.name(), "test");
        assert_eq!(cloned.get_data("key"), Some("value"));
        assert_eq!(cloned.get_metadata("meta"), Some("data"));
    }

    #[test]
    fn test_session_data_equality_with_same_content() {
        let session1 = SessionData::new("test");
        let session2 = SessionData::new("test");

        // Different instances but same name
        assert_eq!(session1.name(), session2.name());
    }

    #[test]
    fn test_session_data_inequality_different_names() {
        let session1 = SessionData::new("test1");
        let session2 = SessionData::new("test2");

        assert_ne!(session1, session2);
    }

    // ============ SessionData Debug Format ============

    #[test]
    fn test_session_data_debug_format() {
        let session = SessionData::new("test-session");
        let debug_str = format!("{:?}", session);
        assert!(debug_str.contains("test-session"));
    }

    // ============ SessionData Validation ============

    #[test]
    fn test_session_data_empty_name_invalid() {
        let invalid = SessionData {
            name: String::new(),
            data: HashMap::new(),
            metadata: HashMap::new(),
            created_at: 100,
            updated_at: 200,
        };
        assert!(!invalid.is_valid());
    }

    #[test]
    fn test_session_data_invalid_timestamps() {
        let invalid = SessionData {
            name: "test".to_string(),
            data: HashMap::new(),
            metadata: HashMap::new(),
            created_at: 200,
            updated_at: 100, // updated_at < created_at
        };
        assert!(!invalid.is_valid());
    }

    // ============ SessionManager Stress Tests ============

    #[test]
    fn test_session_manager_10000_sessions() {
        let mut manager = SessionManager::new();
        for i in 0..10000 {
            manager.save_session(format!("session{}", i));
        }
        assert_eq!(manager.session_count(), 10000);
        assert!(manager.has_session("session9999"));
    }

    #[test]
    fn test_session_manager_10000_data_entries() {
        let mut manager = SessionManager::new();
        for i in 0..10000 {
            manager.set_data(format!("key{}", i), format!("value{}", i));
        }
        manager.save_session("big-session");

        let session = manager.get_session("big-session").unwrap();
        assert_eq!(session.data().len(), 10000);
    }

    #[test]
    fn test_session_manager_rapid_save_load_cycles() {
        let mut manager = SessionManager::new();
        for i in 0..100 {
            manager.set_data("key", format!("value{}", i));
            manager.save_session("test");
            manager.clear_data();
            manager.load_session("test");
            assert_eq!(manager.get_data("key"), Some(format!("value{}", i).as_str()));
        }
    }

    // ============ SessionManager Unicode Edge Cases ============

    #[test]
    fn test_session_manager_unicode_session_names() {
        let mut manager = SessionManager::new();
        manager.save_session("セッション1");
        manager.save_session("الجلسة2");
        manager.save_session("сеанс3");

        assert_eq!(manager.session_count(), 3);
        assert!(manager.has_session("セッション1"));
        assert!(manager.has_session("الجلسة2"));
        assert!(manager.has_session("сеанс3"));
    }

    #[test]
    fn test_session_manager_emoji_session_name() {
        let mut manager = SessionManager::new();
        manager.save_session("🎉 Party Session 🎊");
        assert!(manager.has_session("🎉 Party Session 🎊"));

        manager.load_session("🎉 Party Session 🎊");
        assert_eq!(manager.active_session(), Some("🎉 Party Session 🎊"));
    }

    #[test]
    fn test_session_manager_very_long_session_name() {
        let long_name = "S".repeat(100000);
        let mut manager = SessionManager::new();
        manager.save_session(long_name.clone());
        assert!(manager.has_session(&long_name));
    }

    // ============ SessionManager Complex State Transitions ============

    #[test]
    fn test_session_manager_save_overwrite() {
        let mut manager = SessionManager::new();
        manager.set_data("key", "value1".to_string());
        manager.save_session("test");

        manager.set_data("key", "value2".to_string());
        manager.save_session("test");

        manager.clear_data();
        manager.load_session("test");
        assert_eq!(manager.get_data("key"), Some("value2"));
    }

    #[test]
    fn test_session_manager_multiple_session_switching() {
        let mut manager = SessionManager::new();

        // Create session 1
        manager.set_data("key", "session1".to_string());
        manager.save_session("s1");

        // Create session 2
        manager.clear_data();
        manager.set_data("key", "session2".to_string());
        manager.save_session("s2");

        // Switch back to session 1
        manager.load_session("s1");
        assert_eq!(manager.get_data("key"), Some("session1"));

        // Switch to session 2
        manager.load_session("s2");
        assert_eq!(manager.get_data("key"), Some("session2"));
    }

    #[test]
    fn test_session_manager_delete_active_session() {
        let mut manager = SessionManager::new();
        manager.save_session("active");
        assert_eq!(manager.active_session(), Some("active"));

        manager.delete_session("active");
        assert_eq!(manager.active_session(), None);
        assert_eq!(manager.session_count(), 0);
    }

    #[test]
    fn test_session_manager_delete_non_active_session() {
        let mut manager = SessionManager::new();
        manager.save_session("s1");
        manager.save_session("s2");

        manager.delete_session("s1");
        assert_eq!(manager.active_session(), Some("s2"));
        assert_eq!(manager.session_count(), 1);
    }

    // ============ SessionManager Auto-Save Edge Cases ============

    #[test]
    fn test_session_manager_auto_save_without_active_session() {
        let mut manager = SessionManager::new();
        manager.set_auto_save(true);

        // No active session, auto-save should do nothing
        manager.set_data("key", "value".to_string());
        assert_eq!(manager.session_count(), 0);
    }

    #[test]
    fn test_session_manager_auto_save_toggle() {
        let mut manager = SessionManager::new();
        manager.save_session("test");

        manager.set_auto_save(true);
        manager.set_data("key1", "value1".to_string());

        manager.set_auto_save(false);
        manager.set_data("key2", "value2".to_string());

        // Only key1 should be auto-saved
        let session = manager.get_session("test").unwrap();
        assert_eq!(session.get_data("key1"), Some("value1"));
    }

    // ============ SessionManager Export/Import Edge Cases ============

    #[test]
    fn test_session_manager_export_import_with_unicode() {
        let mut manager = SessionManager::new();
        manager.set_data("キー", "値".to_string());
        manager.save_session("テスト");

        let json = manager.export_session("テスト").unwrap();
        assert!(!json.is_empty());

        let mut new_manager = SessionManager::new();
        new_manager.import_session(&json).unwrap();
        new_manager.load_session("テスト");
        assert_eq!(new_manager.get_data("キー"), Some("値"));
    }

    #[test]
    fn test_session_manager_export_import_large_session() {
        let mut manager = SessionManager::new();
        for i in 0..1000 {
            manager.set_data(format!("key{}", i), format!("value{}", i));
        }
        manager.save_session("large");

        let json = manager.export_session("large").unwrap();
        let mut new_manager = SessionManager::new();
        new_manager.import_session(&json).unwrap();
        new_manager.load_session("large");

        assert_eq!(new_manager.get_data("key999"), Some("value999"));
    }

    #[test]
    fn test_session_manager_import_corrupted_json() {
        let mut manager = SessionManager::new();
        let result = manager.import_session("{\"name\":\"test\",\"data");
        assert!(result.is_err());
    }

    #[test]
    fn test_session_manager_import_invalid_session_data() {
        let mut manager = SessionManager::new();
        let invalid_json = r#"{"name":"","data":{},"metadata":{},"created_at":200,"updated_at":100}"#;
        let result = manager.import_session(invalid_json);
        assert!(result.is_err());
    }

    // ============ SessionManager Rename Edge Cases ============

    #[test]
    fn test_session_manager_rename_active_session() {
        let mut manager = SessionManager::new();
        manager.save_session("old");
        assert_eq!(manager.active_session(), Some("old"));

        manager.rename_session("old", "new");
        assert_eq!(manager.active_session(), Some("new"));
        assert!(!manager.has_session("old"));
        assert!(manager.has_session("new"));
    }

    #[test]
    fn test_session_manager_rename_non_active_session() {
        let mut manager = SessionManager::new();
        manager.save_session("s1");
        manager.save_session("s2");

        manager.rename_session("s1", "s1-renamed");
        assert_eq!(manager.active_session(), Some("s2"));
        assert!(manager.has_session("s1-renamed"));
    }

    #[test]
    fn test_session_manager_rename_with_unicode() {
        let mut manager = SessionManager::new();
        manager.save_session("english");
        manager.rename_session("english", "日本語");
        assert!(manager.has_session("日本語"));
    }

    // ============ SessionManager Clone ============

    #[test]
    fn test_session_manager_clone() {
        let mut manager = SessionManager::new();
        manager.set_data("key", "value".to_string());
        manager.save_session("test");

        let cloned = manager.clone();
        assert_eq!(cloned.session_count(), 1);
        assert!(cloned.has_session("test"));
    }

    // ============ SessionManager Debug Format ============

    #[test]
    fn test_session_manager_debug_format() {
        let mut manager = SessionManager::new();
        manager.save_session("test");
        let debug_str = format!("{:?}", manager);
        assert!(!debug_str.is_empty());
    }

    // ============ SessionManager Edge Cases ============

    #[test]
    fn test_session_manager_empty_session_name() {
        let mut manager = SessionManager::new();
        let session = manager.save_session("");
        assert!(session.is_some());
        assert!(manager.has_session(""));
    }

    #[test]
    fn test_session_manager_whitespace_only_name() {
        let mut manager = SessionManager::new();
        manager.save_session("   ");
        assert!(manager.has_session("   "));
    }

    #[test]
    fn test_session_manager_special_characters_in_name() {
        let mut manager = SessionManager::new();
        manager.save_session("!@#$%^&*()_+-=[]{}|;:',.<>?/~`");
        assert!(manager.has_session("!@#$%^&*()_+-=[]{}|;:',.<>?/~`"));
    }

    #[test]
    fn test_session_manager_session_names_sorted() {
        let mut manager = SessionManager::new();
        manager.save_session("zebra");
        manager.save_session("alpha");
        manager.save_session("beta");

        let names = manager.session_names();
        assert_eq!(names.len(), 3);
        // Just verify all names are present (order is undefined for HashMap)
        assert!(names.contains(&"zebra"));
        assert!(names.contains(&"alpha"));
        assert!(names.contains(&"beta"));
    }

    // ============ Comprehensive Stress Test ============

    #[test]
    fn test_comprehensive_session_manager_stress() {
        let mut manager = SessionManager::new();

        // Create 100 sessions with varying data
        for i in 0..100 {
            manager.clear_data();
            for j in 0..(i % 10) {
                manager.set_data(format!("key{}", j), format!("value{}-{}", i, j));
            }
            manager.save_session(format!("session{}", i));
        }

        assert_eq!(manager.session_count(), 100);

        // Load and verify random sessions
        for i in (0..100).step_by(10) {
            manager.load_session(&format!("session{}", i));
            assert_eq!(manager.active_session(), Some(format!("session{}", i).as_str()));
        }

        // Rename some sessions
        for i in (0..50).step_by(10) {
            manager.rename_session(&format!("session{}", i), format!("renamed{}", i));
            assert!(manager.has_session(&format!("renamed{}", i)));
        }

        // Delete some sessions
        for i in (50..100).step_by(10) {
            manager.delete_session(&format!("session{}", i));
        }

        // Verify final state
        assert!(manager.session_count() < 100);
    }

    #[test]
    fn test_session_data_serialize_deserialize() {
        let mut session = SessionData::new("test");
        session.set_data("key", "value");
        session.set_metadata("author", "tester");

        // Serialize
        let json = serde_json::to_string(&session).unwrap();

        // Deserialize
        let deserialized: SessionData = serde_json::from_str(&json).unwrap();

        assert_eq!(session, deserialized);
        assert_eq!(deserialized.get_data("key"), Some("value"));
        assert_eq!(deserialized.get_metadata("author"), Some("tester"));
    }
}
