//! Vim-style macros and marks system
//!
//! Provides macro recording/playback and bookmark management for
//! vim-style navigation and automation.
//!
//! # Examples
//!
//! ```
//! use toad::widgets::{MacroRecorder, MarkRegistry};
//!
//! // Macros
//! let mut recorder = MacroRecorder::new();
//! recorder.start_recording('q');
//! recorder.record_action("delete_line");
//! recorder.record_action("move_down");
//! recorder.stop_recording();
//!
//! // Marks
//! let mut marks = MarkRegistry::new();
//! marks.set_mark('a', (10, 5));
//! assert_eq!(marks.get_mark('a'), Some(&(10, 5)));
//! ```

use std::collections::HashMap;

/// Action that can be recorded in a macro
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MacroAction {
    /// Action name (e.g., "delete_line", "move_down")
    pub name: String,
    /// Optional action parameter
    pub param: Option<String>,
}

impl MacroAction {
    /// Create a new action
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::MacroAction;
    ///
    /// let action = MacroAction::new("delete_line");
    /// assert_eq!(action.name, "delete_line");
    /// ```
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            param: None,
        }
    }

    /// Create an action with parameter
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::MacroAction;
    ///
    /// let action = MacroAction::with_param("move", "5j");
    /// assert_eq!(action.param, Some("5j".to_string()));
    /// ```
    pub fn with_param(name: impl Into<String>, param: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            param: Some(param.into()),
        }
    }
}

/// Macro recorder for recording and playing back sequences of actions
///
/// # Examples
///
/// ```
/// use toad::widgets::MacroRecorder;
///
/// let mut recorder = MacroRecorder::new();
///
/// // Record a macro
/// recorder.start_recording('q');
/// recorder.record_action("delete_line");
/// recorder.record_action("move_down");
/// recorder.stop_recording();
///
/// // Play it back
/// let actions = recorder.playback_macro('q');
/// assert_eq!(actions.len(), 2);
/// ```
#[derive(Debug, Clone)]
pub struct MacroRecorder {
    /// Stored macros (register -> actions)
    macros: HashMap<char, Vec<MacroAction>>,
    /// Currently recording register
    recording_register: Option<char>,
    /// Actions being recorded
    recording_buffer: Vec<MacroAction>,
}

impl Default for MacroRecorder {
    fn default() -> Self {
        Self::new()
    }
}

impl MacroRecorder {
    /// Create a new macro recorder
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::MacroRecorder;
    ///
    /// let recorder = MacroRecorder::new();
    /// assert!(!recorder.is_recording());
    /// ```
    pub fn new() -> Self {
        Self {
            macros: HashMap::new(),
            recording_register: None,
            recording_buffer: Vec::new(),
        }
    }

    /// Start recording a macro to a register
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::MacroRecorder;
    ///
    /// let mut recorder = MacroRecorder::new();
    /// recorder.start_recording('q');
    /// assert!(recorder.is_recording());
    /// ```
    pub fn start_recording(&mut self, register: char) {
        self.recording_register = Some(register);
        self.recording_buffer.clear();
    }

    /// Stop recording the current macro
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::MacroRecorder;
    ///
    /// let mut recorder = MacroRecorder::new();
    /// recorder.start_recording('q');
    /// recorder.record_action("test");
    /// recorder.stop_recording();
    /// assert!(!recorder.is_recording());
    /// ```
    pub fn stop_recording(&mut self) {
        if let Some(register) = self.recording_register {
            self.macros.insert(register, self.recording_buffer.clone());
            self.recording_register = None;
            self.recording_buffer.clear();
        }
    }

    /// Check if currently recording
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::MacroRecorder;
    ///
    /// let mut recorder = MacroRecorder::new();
    /// assert!(!recorder.is_recording());
    /// recorder.start_recording('a');
    /// assert!(recorder.is_recording());
    /// ```
    pub fn is_recording(&self) -> bool {
        self.recording_register.is_some()
    }

    /// Get the current recording register
    pub fn recording_register(&self) -> Option<char> {
        self.recording_register
    }

    /// Record an action
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::MacroRecorder;
    ///
    /// let mut recorder = MacroRecorder::new();
    /// recorder.start_recording('q');
    /// recorder.record_action("delete");
    /// recorder.record_action("paste");
    /// recorder.stop_recording();
    /// ```
    pub fn record_action(&mut self, action: impl Into<String>) {
        if self.is_recording() {
            self.recording_buffer.push(MacroAction::new(action));
        }
    }

    /// Record an action with parameter
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::MacroRecorder;
    ///
    /// let mut recorder = MacroRecorder::new();
    /// recorder.start_recording('q');
    /// recorder.record_action_with_param("move", "5j");
    /// recorder.stop_recording();
    /// ```
    pub fn record_action_with_param(
        &mut self,
        action: impl Into<String>,
        param: impl Into<String>,
    ) {
        if self.is_recording() {
            self.recording_buffer
                .push(MacroAction::with_param(action, param));
        }
    }

    /// Play back a macro
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::MacroRecorder;
    ///
    /// let mut recorder = MacroRecorder::new();
    /// recorder.start_recording('q');
    /// recorder.record_action("test");
    /// recorder.stop_recording();
    ///
    /// let actions = recorder.playback_macro('q');
    /// assert_eq!(actions.len(), 1);
    /// ```
    pub fn playback_macro(&self, register: char) -> Vec<MacroAction> {
        self.macros.get(&register).cloned().unwrap_or_default()
    }

    /// Check if a macro exists in a register
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::MacroRecorder;
    ///
    /// let mut recorder = MacroRecorder::new();
    /// assert!(!recorder.has_macro('q'));
    ///
    /// recorder.start_recording('q');
    /// recorder.stop_recording();
    /// assert!(recorder.has_macro('q'));
    /// ```
    pub fn has_macro(&self, register: char) -> bool {
        self.macros.contains_key(&register)
    }

    /// Clear a macro from a register
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::MacroRecorder;
    ///
    /// let mut recorder = MacroRecorder::new();
    /// recorder.start_recording('q');
    /// recorder.stop_recording();
    /// assert!(recorder.has_macro('q'));
    ///
    /// recorder.clear_macro('q');
    /// assert!(!recorder.has_macro('q'));
    /// ```
    pub fn clear_macro(&mut self, register: char) {
        self.macros.remove(&register);
    }

    /// Clear all macros
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::MacroRecorder;
    ///
    /// let mut recorder = MacroRecorder::new();
    /// recorder.start_recording('q');
    /// recorder.stop_recording();
    /// recorder.clear_all();
    /// assert_eq!(recorder.macro_count(), 0);
    /// ```
    pub fn clear_all(&mut self) {
        self.macros.clear();
    }

    /// Get number of stored macros
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::MacroRecorder;
    ///
    /// let mut recorder = MacroRecorder::new();
    /// assert_eq!(recorder.macro_count(), 0);
    ///
    /// recorder.start_recording('q');
    /// recorder.stop_recording();
    /// assert_eq!(recorder.macro_count(), 1);
    /// ```
    pub fn macro_count(&self) -> usize {
        self.macros.len()
    }
}

/// Mark registry for bookmarks
///
/// # Examples
///
/// ```
/// use toad::widgets::MarkRegistry;
///
/// let mut marks = MarkRegistry::new();
///
/// // Set a mark
/// marks.set_mark('a', (10, 5));
///
/// // Jump to a mark
/// let pos = marks.get_mark('a');
/// assert_eq!(pos, Some(&(10, 5)));
/// ```
#[derive(Debug, Clone)]
pub struct MarkRegistry {
    /// Local marks (a-z)
    local_marks: HashMap<char, (usize, usize)>,
    /// Global marks (A-Z)
    global_marks: HashMap<char, (String, usize, usize)>,
}

impl Default for MarkRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl MarkRegistry {
    /// Create a new mark registry
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::MarkRegistry;
    ///
    /// let marks = MarkRegistry::new();
    /// assert_eq!(marks.mark_count(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            local_marks: HashMap::new(),
            global_marks: HashMap::new(),
        }
    }

    /// Set a local mark (a-z)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::MarkRegistry;
    ///
    /// let mut marks = MarkRegistry::new();
    /// marks.set_mark('a', (10, 5));
    /// assert!(marks.has_mark('a'));
    /// ```
    pub fn set_mark(&mut self, mark: char, position: (usize, usize)) {
        if mark.is_ascii_lowercase() {
            self.local_marks.insert(mark, position);
        }
    }

    /// Set a global mark (A-Z) with file path
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::MarkRegistry;
    ///
    /// let mut marks = MarkRegistry::new();
    /// marks.set_global_mark('A', "file.txt", (10, 5));
    /// assert!(marks.has_mark('A'));
    /// ```
    pub fn set_global_mark(
        &mut self,
        mark: char,
        file: impl Into<String>,
        position: (usize, usize),
    ) {
        if mark.is_ascii_uppercase() {
            self.global_marks
                .insert(mark, (file.into(), position.0, position.1));
        }
    }

    /// Get a local mark position
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::MarkRegistry;
    ///
    /// let mut marks = MarkRegistry::new();
    /// marks.set_mark('a', (10, 5));
    /// assert_eq!(marks.get_mark('a'), Some(&(10, 5)));
    /// assert_eq!(marks.get_mark('b'), None);
    /// ```
    pub fn get_mark(&self, mark: char) -> Option<&(usize, usize)> {
        self.local_marks.get(&mark)
    }

    /// Get a global mark (file, row, col)
    pub fn get_global_mark(&self, mark: char) -> Option<&(String, usize, usize)> {
        self.global_marks.get(&mark)
    }

    /// Check if a mark exists
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::MarkRegistry;
    ///
    /// let mut marks = MarkRegistry::new();
    /// assert!(!marks.has_mark('a'));
    ///
    /// marks.set_mark('a', (0, 0));
    /// assert!(marks.has_mark('a'));
    /// ```
    pub fn has_mark(&self, mark: char) -> bool {
        if mark.is_ascii_lowercase() {
            self.local_marks.contains_key(&mark)
        } else {
            self.global_marks.contains_key(&mark)
        }
    }

    /// Clear a mark
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::MarkRegistry;
    ///
    /// let mut marks = MarkRegistry::new();
    /// marks.set_mark('a', (10, 5));
    /// assert!(marks.has_mark('a'));
    ///
    /// marks.clear_mark('a');
    /// assert!(!marks.has_mark('a'));
    /// ```
    pub fn clear_mark(&mut self, mark: char) {
        if mark.is_ascii_lowercase() {
            self.local_marks.remove(&mark);
        } else {
            self.global_marks.remove(&mark);
        }
    }

    /// Clear all marks
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::MarkRegistry;
    ///
    /// let mut marks = MarkRegistry::new();
    /// marks.set_mark('a', (10, 5));
    /// marks.set_mark('b', (20, 10));
    /// marks.clear_all();
    /// assert_eq!(marks.mark_count(), 0);
    /// ```
    pub fn clear_all(&mut self) {
        self.local_marks.clear();
        self.global_marks.clear();
    }

    /// Get total number of marks
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::MarkRegistry;
    ///
    /// let mut marks = MarkRegistry::new();
    /// assert_eq!(marks.mark_count(), 0);
    ///
    /// marks.set_mark('a', (10, 5));
    /// marks.set_global_mark('A', "file.txt", (20, 10));
    /// assert_eq!(marks.mark_count(), 2);
    /// ```
    pub fn mark_count(&self) -> usize {
        self.local_marks.len() + self.global_marks.len()
    }

    /// Get all local marks
    pub fn local_marks(&self) -> Vec<(char, (usize, usize))> {
        let mut marks: Vec<_> = self.local_marks.iter().map(|(k, v)| (*k, *v)).collect();
        marks.sort_by_key(|(k, _)| *k);
        marks
    }

    /// Get all global marks
    pub fn global_marks(&self) -> Vec<(char, String, (usize, usize))> {
        let mut marks: Vec<_> = self
            .global_marks
            .iter()
            .map(|(k, (f, r, c))| (*k, f.clone(), (*r, *c)))
            .collect();
        marks.sort_by_key(|(k, _, _)| *k);
        marks
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macro_action_new() {
        let action = MacroAction::new("delete");
        assert_eq!(action.name, "delete");
        assert_eq!(action.param, None);
    }

    #[test]
    fn test_macro_action_with_param() {
        let action = MacroAction::with_param("move", "5j");
        assert_eq!(action.name, "move");
        assert_eq!(action.param, Some("5j".to_string()));
    }

    #[test]
    fn test_macro_recorder_new() {
        let recorder = MacroRecorder::new();
        assert!(!recorder.is_recording());
        assert_eq!(recorder.macro_count(), 0);
    }

    #[test]
    fn test_macro_recorder_default() {
        let recorder = MacroRecorder::default();
        assert!(!recorder.is_recording());
    }

    #[test]
    fn test_macro_recorder_start_stop() {
        let mut recorder = MacroRecorder::new();

        recorder.start_recording('q');
        assert!(recorder.is_recording());
        assert_eq!(recorder.recording_register(), Some('q'));

        recorder.stop_recording();
        assert!(!recorder.is_recording());
        assert_eq!(recorder.recording_register(), None);
    }

    #[test]
    fn test_macro_recorder_record_action() {
        let mut recorder = MacroRecorder::new();

        recorder.start_recording('q');
        recorder.record_action("delete");
        recorder.record_action("paste");
        recorder.stop_recording();

        let actions = recorder.playback_macro('q');
        assert_eq!(actions.len(), 2);
        assert_eq!(actions[0].name, "delete");
        assert_eq!(actions[1].name, "paste");
    }

    #[test]
    fn test_macro_recorder_record_with_param() {
        let mut recorder = MacroRecorder::new();

        recorder.start_recording('q');
        recorder.record_action_with_param("move", "5j");
        recorder.stop_recording();

        let actions = recorder.playback_macro('q');
        assert_eq!(actions.len(), 1);
        assert_eq!(actions[0].param, Some("5j".to_string()));
    }

    #[test]
    fn test_macro_recorder_has_macro() {
        let mut recorder = MacroRecorder::new();
        assert!(!recorder.has_macro('q'));

        recorder.start_recording('q');
        recorder.stop_recording();
        assert!(recorder.has_macro('q'));
    }

    #[test]
    fn test_macro_recorder_clear_macro() {
        let mut recorder = MacroRecorder::new();

        recorder.start_recording('q');
        recorder.stop_recording();
        assert!(recorder.has_macro('q'));

        recorder.clear_macro('q');
        assert!(!recorder.has_macro('q'));
    }

    #[test]
    fn test_macro_recorder_clear_all() {
        let mut recorder = MacroRecorder::new();

        recorder.start_recording('q');
        recorder.stop_recording();
        recorder.start_recording('w');
        recorder.stop_recording();

        assert_eq!(recorder.macro_count(), 2);
        recorder.clear_all();
        assert_eq!(recorder.macro_count(), 0);
    }

    #[test]
    fn test_macro_recorder_playback_empty() {
        let recorder = MacroRecorder::new();
        let actions = recorder.playback_macro('q');
        assert_eq!(actions.len(), 0);
    }

    #[test]
    fn test_mark_registry_new() {
        let marks = MarkRegistry::new();
        assert_eq!(marks.mark_count(), 0);
    }

    #[test]
    fn test_mark_registry_default() {
        let marks = MarkRegistry::default();
        assert_eq!(marks.mark_count(), 0);
    }

    #[test]
    fn test_mark_registry_set_local() {
        let mut marks = MarkRegistry::new();

        marks.set_mark('a', (10, 5));
        assert!(marks.has_mark('a'));
        assert_eq!(marks.get_mark('a'), Some(&(10, 5)));
    }

    #[test]
    fn test_mark_registry_set_global() {
        let mut marks = MarkRegistry::new();

        marks.set_global_mark('A', "file.txt", (10, 5));
        assert!(marks.has_mark('A'));

        let global = marks.get_global_mark('A');
        assert_eq!(global, Some(&("file.txt".to_string(), 10, 5)));
    }

    #[test]
    fn test_mark_registry_clear_mark() {
        let mut marks = MarkRegistry::new();

        marks.set_mark('a', (10, 5));
        assert!(marks.has_mark('a'));

        marks.clear_mark('a');
        assert!(!marks.has_mark('a'));
    }

    #[test]
    fn test_mark_registry_clear_all() {
        let mut marks = MarkRegistry::new();

        marks.set_mark('a', (10, 5));
        marks.set_mark('b', (20, 10));
        marks.set_global_mark('A', "file.txt", (30, 15));

        assert_eq!(marks.mark_count(), 3);
        marks.clear_all();
        assert_eq!(marks.mark_count(), 0);
    }

    #[test]
    fn test_mark_registry_local_marks() {
        let mut marks = MarkRegistry::new();

        marks.set_mark('a', (10, 5));
        marks.set_mark('b', (20, 10));

        let local = marks.local_marks();
        assert_eq!(local.len(), 2);
        assert_eq!(local[0], ('a', (10, 5)));
        assert_eq!(local[1], ('b', (20, 10)));
    }

    #[test]
    fn test_mark_registry_global_marks() {
        let mut marks = MarkRegistry::new();

        marks.set_global_mark('A', "file1.txt", (10, 5));
        marks.set_global_mark('B', "file2.txt", (20, 10));

        let global = marks.global_marks();
        assert_eq!(global.len(), 2);
        assert_eq!(global[0].0, 'A');
        assert_eq!(global[1].0, 'B');
    }

    #[test]
    fn test_mark_registry_ignores_invalid_marks() {
        let mut marks = MarkRegistry::new();

        // Local marks only accept lowercase
        marks.set_mark('A', (10, 5));
        assert!(!marks.has_mark('A'));

        // Global marks only accept uppercase
        marks.set_global_mark('a', "file.txt", (10, 5));
        assert!(!marks.has_mark('a'));
    }
}
