/// Macro recording and replay system
///
/// Record and replay sequences of actions (Vim-style q/@ macros)
///
/// # Examples
///
/// ```
/// use toad::macros::{MacroManager, MacroAction};
///
/// let mut manager = MacroManager::new();
/// manager.start_recording('a');
/// manager.record_action(MacroAction::InsertText("hello".to_string()));
/// manager.stop_recording();
/// assert!(manager.has_macro('a'));
/// ```
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A single macro action
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MacroAction {
    /// Insert text
    InsertText(String),
    /// Delete text
    DeleteText(usize),
    /// Move cursor
    MoveCursor { line: isize, col: isize },
    /// Enter mode
    EnterMode(String),
    /// Execute command
    Command(String),
    /// Custom action with name and data
    Custom { name: String, data: String },
}

impl MacroAction {
    /// Get a description of this action
    pub fn description(&self) -> String {
        match self {
            MacroAction::InsertText(text) => format!("Insert: {}", text),
            MacroAction::DeleteText(count) => format!("Delete: {} chars", count),
            MacroAction::MoveCursor { line, col } => format!("Move: ({}, {})", line, col),
            MacroAction::EnterMode(mode) => format!("Mode: {}", mode),
            MacroAction::Command(cmd) => format!("Command: {}", cmd),
            MacroAction::Custom { name, .. } => format!("Custom: {}", name),
        }
    }
}

/// A recorded macro
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Macro {
    /// Macro register (a-z, A-Z, 0-9)
    pub register: char,
    /// Recorded actions
    pub actions: Vec<MacroAction>,
}

impl Macro {
    /// Create a new empty macro
    pub fn new(register: char) -> Self {
        Self {
            register,
            actions: Vec::new(),
        }
    }

    /// Add an action to the macro
    pub fn add_action(&mut self, action: MacroAction) {
        self.actions.push(action);
    }

    /// Get the number of actions
    pub fn len(&self) -> usize {
        self.actions.len()
    }

    /// Check if macro is empty
    pub fn is_empty(&self) -> bool {
        self.actions.is_empty()
    }

    /// Clear all actions
    pub fn clear(&mut self) {
        self.actions.clear();
    }

    /// Get actions
    pub fn actions(&self) -> &[MacroAction] {
        &self.actions
    }
}

/// Macro manager
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacroManager {
    /// Stored macros (keyed by register)
    macros: HashMap<char, Macro>,
    /// Currently recording macro
    recording: Option<char>,
    /// Current macro being recorded
    current_macro: Option<Macro>,
    /// Last executed macro register
    last_macro: Option<char>,
}

impl MacroManager {
    /// Create a new macro manager
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::macros::MacroManager;
    ///
    /// let manager = MacroManager::new();
    /// assert!(!manager.is_recording());
    /// ```
    pub fn new() -> Self {
        Self {
            macros: HashMap::new(),
            recording: None,
            current_macro: None,
            last_macro: None,
        }
    }

    /// Start recording a macro
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::macros::MacroManager;
    ///
    /// let mut manager = MacroManager::new();
    /// assert!(manager.start_recording('a'));
    /// assert!(manager.is_recording());
    /// ```
    pub fn start_recording(&mut self, register: char) -> bool {
        if !Self::is_valid_register(register) {
            return false;
        }

        if self.is_recording() {
            return false;
        }

        self.recording = Some(register);
        self.current_macro = Some(Macro::new(register));
        true
    }

    /// Stop recording the current macro
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::macros::{MacroManager, MacroAction};
    ///
    /// let mut manager = MacroManager::new();
    /// manager.start_recording('a');
    /// manager.record_action(MacroAction::InsertText("test".to_string()));
    /// manager.stop_recording();
    /// assert!(!manager.is_recording());
    /// assert!(manager.has_macro('a'));
    /// ```
    pub fn stop_recording(&mut self) -> bool {
        if let Some(register) = self.recording.take()
            && let Some(macro_) = self.current_macro.take()
                && !macro_.is_empty() {
                    self.macros.insert(register, macro_);
                    return true;
                }
        false
    }

    /// Record an action to the current macro
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::macros::{MacroManager, MacroAction};
    ///
    /// let mut manager = MacroManager::new();
    /// manager.start_recording('a');
    /// assert!(manager.record_action(MacroAction::InsertText("hello".to_string())));
    /// ```
    pub fn record_action(&mut self, action: MacroAction) -> bool {
        if let Some(ref mut macro_) = self.current_macro {
            macro_.add_action(action);
            true
        } else {
            false
        }
    }

    /// Check if currently recording
    pub fn is_recording(&self) -> bool {
        self.recording.is_some()
    }

    /// Get the current recording register
    pub fn recording_register(&self) -> Option<char> {
        self.recording
    }

    /// Execute a macro
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::macros::{MacroManager, MacroAction};
    ///
    /// let mut manager = MacroManager::new();
    /// manager.start_recording('a');
    /// manager.record_action(MacroAction::InsertText("test".to_string()));
    /// manager.stop_recording();
    ///
    /// let actions = manager.execute('a');
    /// assert_eq!(actions.len(), 1);
    /// ```
    pub fn execute(&mut self, register: char) -> Vec<MacroAction> {
        if let Some(macro_) = self.macros.get(&register) {
            self.last_macro = Some(register);
            macro_.actions.clone()
        } else {
            Vec::new()
        }
    }

    /// Repeat the last executed macro
    pub fn repeat_last(&mut self) -> Option<Vec<MacroAction>> {
        self.last_macro.map(|register| self.execute(register))
    }

    /// Check if a macro exists
    pub fn has_macro(&self, register: char) -> bool {
        self.macros.contains_key(&register)
    }

    /// Get a macro
    pub fn get_macro(&self, register: char) -> Option<&Macro> {
        self.macros.get(&register)
    }

    /// Delete a macro
    pub fn delete_macro(&mut self, register: char) -> bool {
        self.macros.remove(&register).is_some()
    }

    /// Clear all macros
    pub fn clear_all(&mut self) {
        self.macros.clear();
        self.last_macro = None;
    }

    /// Get all macro registers
    pub fn registers(&self) -> Vec<char> {
        let mut regs: Vec<char> = self.macros.keys().copied().collect();
        regs.sort();
        regs
    }

    /// Get the number of stored macros
    pub fn count(&self) -> usize {
        self.macros.len()
    }

    /// Check if a register is valid for macros (a-z, A-Z, 0-9)
    pub fn is_valid_register(ch: char) -> bool {
        ch.is_ascii_alphanumeric()
    }

    /// Save macros to file
    pub fn save_to_file(&self, path: &std::path::Path) -> anyhow::Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// Load macros from file
    pub fn load_from_file(path: &std::path::Path) -> anyhow::Result<Self> {
        let contents = std::fs::read_to_string(path)?;
        let manager = serde_json::from_str(&contents)?;
        Ok(manager)
    }
}

impl Default for MacroManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macro_action_description() {
        let action = MacroAction::InsertText("hello".to_string());
        assert!(action.description().contains("Insert"));

        let action = MacroAction::DeleteText(5);
        assert!(action.description().contains("Delete"));

        let action = MacroAction::MoveCursor { line: 1, col: 2 };
        assert!(action.description().contains("Move"));
    }

    #[test]
    fn test_macro_creation() {
        let macro_ = Macro::new('a');
        assert_eq!(macro_.register, 'a');
        assert!(macro_.is_empty());
    }

    #[test]
    fn test_macro_add_action() {
        let mut macro_ = Macro::new('a');
        macro_.add_action(MacroAction::InsertText("test".to_string()));

        assert_eq!(macro_.len(), 1);
        assert!(!macro_.is_empty());
    }

    #[test]
    fn test_macro_clear() {
        let mut macro_ = Macro::new('a');
        macro_.add_action(MacroAction::InsertText("test".to_string()));
        assert!(!macro_.is_empty());

        macro_.clear();
        assert!(macro_.is_empty());
    }

    #[test]
    fn test_manager_creation() {
        let manager = MacroManager::new();
        assert!(!manager.is_recording());
        assert_eq!(manager.count(), 0);
    }

    #[test]
    fn test_start_recording() {
        let mut manager = MacroManager::new();
        assert!(manager.start_recording('a'));
        assert!(manager.is_recording());
        assert_eq!(manager.recording_register(), Some('a'));
    }

    #[test]
    fn test_start_recording_invalid() {
        let mut manager = MacroManager::new();
        assert!(!manager.start_recording('!'));
        assert!(!manager.is_recording());
    }

    #[test]
    fn test_start_recording_while_recording() {
        let mut manager = MacroManager::new();
        manager.start_recording('a');
        assert!(!manager.start_recording('b'));
        assert_eq!(manager.recording_register(), Some('a'));
    }

    #[test]
    fn test_record_action() {
        let mut manager = MacroManager::new();
        manager.start_recording('a');

        assert!(manager.record_action(MacroAction::InsertText("hello".to_string())));
    }

    #[test]
    fn test_record_action_not_recording() {
        let mut manager = MacroManager::new();
        assert!(!manager.record_action(MacroAction::InsertText("hello".to_string())));
    }

    #[test]
    fn test_stop_recording() {
        let mut manager = MacroManager::new();
        manager.start_recording('a');
        manager.record_action(MacroAction::InsertText("test".to_string()));

        assert!(manager.stop_recording());
        assert!(!manager.is_recording());
        assert!(manager.has_macro('a'));
    }

    #[test]
    fn test_stop_recording_empty_macro() {
        let mut manager = MacroManager::new();
        manager.start_recording('a');

        assert!(!manager.stop_recording());
        assert!(!manager.has_macro('a'));
    }

    #[test]
    fn test_execute_macro() {
        let mut manager = MacroManager::new();
        manager.start_recording('a');
        manager.record_action(MacroAction::InsertText("hello".to_string()));
        manager.record_action(MacroAction::MoveCursor { line: 0, col: 1 });
        manager.stop_recording();

        let actions = manager.execute('a');
        assert_eq!(actions.len(), 2);
    }

    #[test]
    fn test_execute_nonexistent_macro() {
        let mut manager = MacroManager::new();
        let actions = manager.execute('z');
        assert_eq!(actions.len(), 0);
    }

    #[test]
    fn test_repeat_last() {
        let mut manager = MacroManager::new();
        manager.start_recording('a');
        manager.record_action(MacroAction::InsertText("test".to_string()));
        manager.stop_recording();

        manager.execute('a');

        let actions = manager.repeat_last();
        assert!(actions.is_some());
        assert_eq!(actions.unwrap().len(), 1);
    }

    #[test]
    fn test_repeat_last_no_execution() {
        let mut manager = MacroManager::new();
        assert!(manager.repeat_last().is_none());
    }

    #[test]
    fn test_delete_macro() {
        let mut manager = MacroManager::new();
        manager.start_recording('a');
        manager.record_action(MacroAction::InsertText("test".to_string()));
        manager.stop_recording();

        assert!(manager.has_macro('a'));
        assert!(manager.delete_macro('a'));
        assert!(!manager.has_macro('a'));
    }

    #[test]
    fn test_clear_all() {
        let mut manager = MacroManager::new();
        manager.start_recording('a');
        manager.record_action(MacroAction::InsertText("test1".to_string()));
        manager.stop_recording();

        manager.start_recording('b');
        manager.record_action(MacroAction::InsertText("test2".to_string()));
        manager.stop_recording();

        assert_eq!(manager.count(), 2);

        manager.clear_all();
        assert_eq!(manager.count(), 0);
    }

    #[test]
    fn test_registers() {
        let mut manager = MacroManager::new();
        manager.start_recording('c');
        manager.record_action(MacroAction::InsertText("test1".to_string()));
        manager.stop_recording();

        manager.start_recording('a');
        manager.record_action(MacroAction::InsertText("test2".to_string()));
        manager.stop_recording();

        let regs = manager.registers();
        assert_eq!(regs, vec!['a', 'c']); // Sorted
    }

    #[test]
    fn test_is_valid_register() {
        assert!(MacroManager::is_valid_register('a'));
        assert!(MacroManager::is_valid_register('Z'));
        assert!(MacroManager::is_valid_register('0'));
        assert!(!MacroManager::is_valid_register('!'));
        assert!(!MacroManager::is_valid_register('@'));
    }

    #[test]
    fn test_get_macro() {
        let mut manager = MacroManager::new();
        manager.start_recording('a');
        manager.record_action(MacroAction::InsertText("test".to_string()));
        manager.stop_recording();

        let macro_ = manager.get_macro('a');
        assert!(macro_.is_some());
        assert_eq!(macro_.unwrap().actions().len(), 1);
    }

    #[test]
    fn test_default() {
        let manager = MacroManager::default();
        assert!(!manager.is_recording());
    }

    #[test]
    fn test_save_and_load() {
        let mut manager = MacroManager::new();
        manager.start_recording('a');
        manager.record_action(MacroAction::InsertText("test".to_string()));
        manager.stop_recording();

        let temp_dir = std::env::temp_dir();
        let path = temp_dir.join("test_macros.json");

        // Save
        manager.save_to_file(&path).unwrap();

        // Load
        let loaded = MacroManager::load_from_file(&path).unwrap();
        assert_eq!(loaded.count(), 1);
        assert!(loaded.has_macro('a'));

        // Cleanup
        std::fs::remove_file(&path).ok();
    }

    #[test]
    fn test_macro_actions() {
        let macro_ = Macro::new('a');
        assert_eq!(macro_.actions().len(), 0);

        let mut macro_ = Macro::new('b');
        macro_.add_action(MacroAction::Command("save".to_string()));
        assert_eq!(macro_.actions().len(), 1);
    }

    #[test]
    fn test_complex_macro() {
        let mut manager = MacroManager::new();
        manager.start_recording('q');

        manager.record_action(MacroAction::EnterMode("insert".to_string()));
        manager.record_action(MacroAction::InsertText("hello ".to_string()));
        manager.record_action(MacroAction::EnterMode("normal".to_string()));
        manager.record_action(MacroAction::MoveCursor { line: 1, col: 0 });

        manager.stop_recording();

        let actions = manager.execute('q');
        assert_eq!(actions.len(), 4);
    }

    #[test]
    fn test_custom_action() {
        let action = MacroAction::Custom {
            name: "custom_op".to_string(),
            data: "data".to_string(),
        };

        assert!(action.description().contains("Custom"));
        assert!(action.description().contains("custom_op"));
    }
}
