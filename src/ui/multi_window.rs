//! Multi-window system for managing multiple TOAD instances
//!
//! Provides window management, switching, and overview capabilities.
//! Inspired by modern terminal multiplexers and IDE window systems.
//!
//! # Examples
//!
//! ```
//! use toad::ui::multi_window::{Window, WindowManager};
//!
//! let mut manager = WindowManager::new();
//! let window_id = manager.create_window("Main Workspace");
//! manager.switch_to(window_id);
//! ```

use std::collections::HashMap;
use std::time::{Duration, SystemTime};

/// Unique window identifier
pub type WindowId = usize;

/// Window state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowState {
    /// Window is active and visible
    Active,
    /// Window is in background
    Background,
    /// Window is minimized
    Minimized,
    /// Window is being closed
    Closing,
}

/// Window priority for task switching
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WindowPriority {
    /// Low priority background task
    Low,
    /// Normal priority
    Normal,
    /// High priority (e.g., active development)
    High,
    /// Urgent (e.g., build errors, critical task)
    Urgent,
}

/// Represents a single window instance
#[derive(Debug, Clone)]
pub struct Window {
    /// Unique identifier
    id: WindowId,
    /// Window title
    title: String,
    /// Window state
    state: WindowState,
    /// Priority level
    priority: WindowPriority,
    /// Workspace/context path
    workspace: Option<String>,
    /// Creation timestamp
    created_at: SystemTime,
    /// Last accessed timestamp
    last_accessed: SystemTime,
    /// Whether window has unsaved changes
    has_unsaved_changes: bool,
    /// Preview text (for window switcher)
    preview_text: Option<String>,
    /// Window-specific metadata
    metadata: HashMap<String, String>,
}

impl Window {
    /// Create a new window
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::multi_window::Window;
    ///
    /// let window = Window::new(1, "Main");
    /// assert_eq!(window.id(), 1);
    /// assert_eq!(window.title(), "Main");
    /// ```
    pub fn new(id: WindowId, title: impl Into<String>) -> Self {
        let now = SystemTime::now();
        Self {
            id,
            title: title.into(),
            state: WindowState::Active,
            priority: WindowPriority::Normal,
            workspace: None,
            created_at: now,
            last_accessed: now,
            has_unsaved_changes: false,
            preview_text: None,
            metadata: HashMap::new(),
        }
    }

    /// Get window ID
    pub fn id(&self) -> WindowId {
        self.id
    }

    /// Get window title
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Set window title
    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
        self.touch();
    }

    /// Get window state
    pub fn state(&self) -> &WindowState {
        &self.state
    }

    /// Set window state
    pub fn set_state(&mut self, state: WindowState) {
        self.state = state;
        if state == WindowState::Active {
            self.touch();
        }
    }

    /// Get priority
    pub fn priority(&self) -> WindowPriority {
        self.priority
    }

    /// Set priority
    pub fn set_priority(&mut self, priority: WindowPriority) {
        self.priority = priority;
    }

    /// Get workspace path
    pub fn workspace(&self) -> Option<&str> {
        self.workspace.as_deref()
    }

    /// Set workspace
    pub fn set_workspace(&mut self, workspace: impl Into<String>) {
        self.workspace = Some(workspace.into());
        self.touch();
    }

    /// Check if window has unsaved changes
    pub fn has_unsaved_changes(&self) -> bool {
        self.has_unsaved_changes
    }

    /// Set unsaved changes flag
    pub fn set_unsaved_changes(&mut self, has_changes: bool) {
        self.has_unsaved_changes = has_changes;
    }

    /// Get preview text
    pub fn preview_text(&self) -> Option<&str> {
        self.preview_text.as_deref()
    }

    /// Set preview text
    pub fn set_preview_text(&mut self, text: impl Into<String>) {
        self.preview_text = Some(text.into());
    }

    /// Get metadata value
    pub fn get_metadata(&self, key: &str) -> Option<&str> {
        self.metadata.get(key).map(|s| s.as_str())
    }

    /// Set metadata value
    pub fn set_metadata(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.metadata.insert(key.into(), value.into());
    }

    /// Get age of window
    pub fn age(&self) -> Duration {
        self.created_at
            .elapsed()
            .unwrap_or_else(|_| Duration::from_secs(0))
    }

    /// Get idle time since last access
    pub fn idle_time(&self) -> Duration {
        self.last_accessed
            .elapsed()
            .unwrap_or_else(|_| Duration::from_secs(0))
    }

    /// Update last accessed timestamp
    pub fn touch(&mut self) {
        self.last_accessed = SystemTime::now();
    }

    /// Check if window is active
    pub fn is_active(&self) -> bool {
        self.state == WindowState::Active
    }

    /// Check if window is minimized
    pub fn is_minimized(&self) -> bool {
        self.state == WindowState::Minimized
    }
}

/// Multi-window manager
///
/// Manages multiple window instances with switching, overview, and persistence.
#[derive(Debug)]
pub struct WindowManager {
    /// All windows
    windows: HashMap<WindowId, Window>,
    /// Active window ID
    active_window: Option<WindowId>,
    /// Window ID counter
    next_id: WindowId,
    /// Recently used windows (MRU order)
    mru_order: Vec<WindowId>,
    /// Maximum number of windows
    max_windows: usize,
}

impl WindowManager {
    /// Create a new window manager
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::multi_window::WindowManager;
    ///
    /// let manager = WindowManager::new();
    /// ```
    pub fn new() -> Self {
        Self {
            windows: HashMap::new(),
            active_window: None,
            next_id: 1,
            mru_order: Vec::new(),
            max_windows: 10,
        }
    }

    /// Create a new window
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::multi_window::WindowManager;
    ///
    /// let mut manager = WindowManager::new();
    /// let id = manager.create_window("Main");
    /// assert!(manager.get_window(id).is_some());
    /// ```
    pub fn create_window(&mut self, title: impl Into<String>) -> WindowId {
        let id = self.next_id;
        self.next_id += 1;

        let window = Window::new(id, title);
        self.windows.insert(id, window);
        self.mru_order.insert(0, id);

        // Auto-switch to first window
        if self.active_window.is_none() {
            self.active_window = Some(id);
        }

        // Enforce max windows limit
        if self.windows.len() > self.max_windows {
            self.close_oldest_inactive();
        }

        id
    }

    /// Get window by ID
    pub fn get_window(&self, id: WindowId) -> Option<&Window> {
        self.windows.get(&id)
    }

    /// Get mutable window by ID
    pub fn get_window_mut(&mut self, id: WindowId) -> Option<&mut Window> {
        self.windows.get_mut(&id)
    }

    /// Get active window
    pub fn active_window(&self) -> Option<&Window> {
        self.active_window.and_then(|id| self.windows.get(&id))
    }

    /// Get active window (mutable)
    pub fn active_window_mut(&mut self) -> Option<&mut Window> {
        self.active_window.and_then(|id| self.windows.get_mut(&id))
    }

    /// Switch to window
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::multi_window::WindowManager;
    ///
    /// let mut manager = WindowManager::new();
    /// let id1 = manager.create_window("Window 1");
    /// let id2 = manager.create_window("Window 2");
    ///
    /// assert!(manager.switch_to(id2));
    /// assert_eq!(manager.active_window().unwrap().id(), id2);
    /// ```
    pub fn switch_to(&mut self, id: WindowId) -> bool {
        if self.windows.contains_key(&id) {
            // Update previous active window state
            if let Some(prev_id) = self.active_window {
                if let Some(window) = self.windows.get_mut(&prev_id) {
                    window.set_state(WindowState::Background);
                }
            }

            // Update MRU order
            self.mru_order.retain(|&x| x != id);
            self.mru_order.insert(0, id);

            // Activate new window
            if let Some(window) = self.windows.get_mut(&id) {
                window.set_state(WindowState::Active);
            }

            self.active_window = Some(id);
            true
        } else {
            false
        }
    }

    /// Switch to next window
    pub fn next_window(&mut self) -> Option<WindowId> {
        if self.mru_order.is_empty() {
            return None;
        }

        let current_idx = self
            .active_window
            .and_then(|id| self.mru_order.iter().position(|&x| x == id))
            .unwrap_or(0);

        let next_idx = (current_idx + 1) % self.mru_order.len();
        let next_id = self.mru_order[next_idx];

        self.switch_to(next_id);
        Some(next_id)
    }

    /// Switch to previous window
    pub fn prev_window(&mut self) -> Option<WindowId> {
        if self.mru_order.is_empty() {
            return None;
        }

        let current_idx = self
            .active_window
            .and_then(|id| self.mru_order.iter().position(|&x| x == id))
            .unwrap_or(0);

        let prev_idx = if current_idx == 0 {
            self.mru_order.len() - 1
        } else {
            current_idx - 1
        };

        let prev_id = self.mru_order[prev_idx];

        self.switch_to(prev_id);
        Some(prev_id)
    }

    /// Close window
    pub fn close_window(&mut self, id: WindowId) -> bool {
        if let Some(window) = self.windows.remove(&id) {
            self.mru_order.retain(|&x| x != id);

            // If closing active window, switch to next
            if self.active_window == Some(id) {
                self.active_window = self.mru_order.first().copied();
                if let Some(new_active) = self.active_window {
                    if let Some(win) = self.windows.get_mut(&new_active) {
                        win.set_state(WindowState::Active);
                    }
                }
            }

            true
        } else {
            false
        }
    }

    /// Close oldest inactive window
    fn close_oldest_inactive(&mut self) {
        let oldest = self
            .windows
            .iter()
            .filter(|(id, w)| Some(**id) != self.active_window && !w.has_unsaved_changes())
            .max_by_key(|(_, w)| w.idle_time())
            .map(|(id, _)| *id);

        if let Some(id) = oldest {
            self.close_window(id);
        }
    }

    /// Get all windows in MRU order
    pub fn windows_mru(&self) -> Vec<&Window> {
        self.mru_order
            .iter()
            .filter_map(|id| self.windows.get(id))
            .collect()
    }

    /// Get window count
    pub fn window_count(&self) -> usize {
        self.windows.len()
    }

    /// Check if manager has windows
    pub fn is_empty(&self) -> bool {
        self.windows.is_empty()
    }

    /// Set max windows
    pub fn set_max_windows(&mut self, max: usize) {
        self.max_windows = max;
    }

    /// Get windows by priority
    pub fn windows_by_priority(&self) -> Vec<&Window> {
        let mut windows: Vec<&Window> = self.windows.values().collect();
        windows.sort_by(|a, b| b.priority().cmp(&a.priority()));
        windows
    }

    /// Get windows with unsaved changes
    pub fn unsaved_windows(&self) -> Vec<&Window> {
        self.windows
            .values()
            .filter(|w| w.has_unsaved_changes())
            .collect()
    }
}

impl Default for WindowManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window_creation() {
        let window = Window::new(1, "Test Window");
        assert_eq!(window.id(), 1);
        assert_eq!(window.title(), "Test Window");
        assert_eq!(window.state(), &WindowState::Active);
        assert_eq!(window.priority(), WindowPriority::Normal);
    }

    #[test]
    fn test_window_state_changes() {
        let mut window = Window::new(1, "Test");
        window.set_state(WindowState::Minimized);
        assert_eq!(window.state(), &WindowState::Minimized);
        assert!(window.is_minimized());
    }

    #[test]
    fn test_window_metadata() {
        let mut window = Window::new(1, "Test");
        window.set_metadata("project", "toad");
        assert_eq!(window.get_metadata("project"), Some("toad"));
    }

    #[test]
    fn test_window_manager_creation() {
        let manager = WindowManager::new();
        assert!(manager.is_empty());
        assert_eq!(manager.window_count(), 0);
    }

    #[test]
    fn test_create_window() {
        let mut manager = WindowManager::new();
        let id = manager.create_window("Window 1");
        assert_eq!(manager.window_count(), 1);
        assert!(manager.get_window(id).is_some());
    }

    #[test]
    fn test_switch_to_window() {
        let mut manager = WindowManager::new();
        let id1 = manager.create_window("Window 1");
        let id2 = manager.create_window("Window 2");

        assert!(manager.switch_to(id2));
        assert_eq!(manager.active_window().unwrap().id(), id2);

        assert!(manager.switch_to(id1));
        assert_eq!(manager.active_window().unwrap().id(), id1);
    }

    #[test]
    fn test_next_prev_window() {
        let mut manager = WindowManager::new();
        let id1 = manager.create_window("Window 1");
        let id2 = manager.create_window("Window 2");
        let id3 = manager.create_window("Window 3");

        // After creation: MRU = [id3, id2, id1], active = id1
        manager.switch_to(id1);
        // After switch to id1: MRU = [id1, id3, id2], active = id1

        // Next from id1 should cycle through MRU: id1 → id3
        let next = manager.next_window();
        assert_eq!(next, Some(id3));
        // MRU is now [id3, id1, id2]

        // Next from id3: id3 → id1
        let next = manager.next_window();
        assert_eq!(next, Some(id1));
        // MRU is now [id1, id3, id2]

        // Previous from id1: wraps to last in MRU, which is id2
        let prev = manager.prev_window();
        assert_eq!(prev, Some(id2));
    }

    #[test]
    fn test_close_window() {
        let mut manager = WindowManager::new();
        let id1 = manager.create_window("Window 1");
        let id2 = manager.create_window("Window 2");

        assert!(manager.close_window(id1));
        assert_eq!(manager.window_count(), 1);
        assert!(manager.get_window(id1).is_none());
        assert!(manager.get_window(id2).is_some());
    }

    #[test]
    fn test_close_active_window_switches() {
        let mut manager = WindowManager::new();
        let id1 = manager.create_window("Window 1");
        let id2 = manager.create_window("Window 2");

        manager.switch_to(id1);
        assert_eq!(manager.active_window().unwrap().id(), id1);

        manager.close_window(id1);
        assert_eq!(manager.active_window().unwrap().id(), id2);
    }

    #[test]
    fn test_mru_order() {
        let mut manager = WindowManager::new();
        let id1 = manager.create_window("Window 1");
        let id2 = manager.create_window("Window 2");
        let id3 = manager.create_window("Window 3");

        manager.switch_to(id1);
        manager.switch_to(id3);
        manager.switch_to(id2);

        let mru = manager.windows_mru();
        assert_eq!(mru[0].id(), id2);
        assert_eq!(mru[1].id(), id3);
        assert_eq!(mru[2].id(), id1);
    }

    #[test]
    fn test_windows_by_priority() {
        let mut manager = WindowManager::new();
        let id1 = manager.create_window("Low");
        let id2 = manager.create_window("Urgent");
        let id3 = manager.create_window("High");

        manager.get_window_mut(id1).unwrap().set_priority(WindowPriority::Low);
        manager.get_window_mut(id2).unwrap().set_priority(WindowPriority::Urgent);
        manager.get_window_mut(id3).unwrap().set_priority(WindowPriority::High);

        let by_priority = manager.windows_by_priority();
        assert_eq!(by_priority[0].id(), id2); // Urgent first
        assert_eq!(by_priority[1].id(), id3); // High second
        assert_eq!(by_priority[2].id(), id1); // Low last
    }

    #[test]
    fn test_unsaved_windows() {
        let mut manager = WindowManager::new();
        let id1 = manager.create_window("Saved");
        let id2 = manager.create_window("Unsaved");

        manager.get_window_mut(id2).unwrap().set_unsaved_changes(true);

        let unsaved = manager.unsaved_windows();
        assert_eq!(unsaved.len(), 1);
        assert_eq!(unsaved[0].id(), id2);
    }

    #[test]
    fn test_max_windows_limit() {
        let mut manager = WindowManager::new();
        manager.set_max_windows(3);

        let id1 = manager.create_window("Window 1");
        let id2 = manager.create_window("Window 2");
        let _id3 = manager.create_window("Window 3");

        // Switch to id2 to make id1 least recently used
        manager.switch_to(id2);

        // Creating 4th window should close least recently used inactive window
        let id4 = manager.create_window("Window 4");

        assert_eq!(manager.window_count(), 3);
        // One of the old windows should be closed
        assert!(manager.get_window(id4).is_some());
        // Active window (id2) should still be present
        assert!(manager.get_window(id2).is_some());
    }

    #[test]
    fn test_window_with_workspace() {
        let mut window = Window::new(1, "Test");
        window.set_workspace("/home/user/project");
        assert_eq!(window.workspace(), Some("/home/user/project"));
    }

    #[test]
    fn test_window_preview_text() {
        let mut window = Window::new(1, "Test");
        window.set_preview_text("Preview content");
        assert_eq!(window.preview_text(), Some("Preview content"));
    }
}
