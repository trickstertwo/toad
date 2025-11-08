//! Undo/Redo system for action management
//!
//! Provides a history-based undo/redo system for tracking and reverting actions.
//!
//! # Examples
//!
//! ```
//! use toad::widgets::{UndoRedoManager, Action};
//!
//! let mut manager = UndoRedoManager::new();
//!
//! // Execute actions
//! manager.execute(Action::new("delete_line", "line content"));
//! manager.execute(Action::new("insert_text", "hello"));
//!
//! // Undo
//! if let Some(action) = manager.undo() {
//!     println!("Undid: {}", action.name());
//! }
//!
//! // Redo
//! if let Some(action) = manager.redo() {
//!     println!("Redid: {}", action.name());
//! }
//! ```

use std::collections::VecDeque;

/// An action that can be undone/redone
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Action {
    /// Action name
    name: String,
    /// Action data (before state)
    before: Option<String>,
    /// Action data (after state)
    after: Option<String>,
    /// Optional metadata
    metadata: Option<String>,
}

impl Action {
    /// Create a new action
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Action;
    ///
    /// let action = Action::new("delete", "content");
    /// assert_eq!(action.name(), "delete");
    /// ```
    pub fn new(name: impl Into<String>, data: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            before: Some(data.into()),
            after: None,
            metadata: None,
        }
    }

    /// Create an action with before/after states
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Action;
    ///
    /// let action = Action::with_states("replace", "old", "new");
    /// assert_eq!(action.before(), Some("old"));
    /// assert_eq!(action.after(), Some("new"));
    /// ```
    pub fn with_states(
        name: impl Into<String>,
        before: impl Into<String>,
        after: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            before: Some(before.into()),
            after: Some(after.into()),
            metadata: None,
        }
    }

    /// Add metadata to the action
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Action;
    ///
    /// let action = Action::new("delete", "content")
    ///     .with_metadata("line 42");
    /// ```
    pub fn with_metadata(mut self, metadata: impl Into<String>) -> Self {
        self.metadata = Some(metadata.into());
        self
    }

    /// Get action name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get before state
    pub fn before(&self) -> Option<&str> {
        self.before.as_deref()
    }

    /// Get after state
    pub fn after(&self) -> Option<&str> {
        self.after.as_deref()
    }

    /// Get metadata
    pub fn metadata(&self) -> Option<&str> {
        self.metadata.as_deref()
    }
}

/// Undo/Redo manager
///
/// Manages a history of actions with undo/redo capability.
///
/// # Examples
///
/// ```
/// use toad::widgets::{UndoRedoManager, Action};
///
/// let mut manager = UndoRedoManager::new();
///
/// manager.execute(Action::new("action1", "data1"));
/// manager.execute(Action::new("action2", "data2"));
///
/// assert_eq!(manager.can_undo(), true);
/// assert_eq!(manager.can_redo(), false);
///
/// manager.undo();
/// assert_eq!(manager.can_redo(), true);
/// ```
#[derive(Debug, Clone)]
pub struct UndoRedoManager {
    /// Action history
    history: VecDeque<Action>,
    /// Current position in history
    position: usize,
    /// Maximum history size
    max_history: usize,
}

impl Default for UndoRedoManager {
    fn default() -> Self {
        Self::new()
    }
}

impl UndoRedoManager {
    /// Create a new undo/redo manager
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::UndoRedoManager;
    ///
    /// let manager = UndoRedoManager::new();
    /// assert_eq!(manager.history_size(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            history: VecDeque::new(),
            position: 0,
            max_history: 100,
        }
    }

    /// Create manager with custom max history
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::UndoRedoManager;
    ///
    /// let manager = UndoRedoManager::with_max_history(50);
    /// ```
    pub fn with_max_history(max: usize) -> Self {
        Self {
            history: VecDeque::new(),
            position: 0,
            max_history: max,
        }
    }

    /// Execute an action and add to history
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{UndoRedoManager, Action};
    ///
    /// let mut manager = UndoRedoManager::new();
    /// manager.execute(Action::new("delete", "text"));
    /// assert_eq!(manager.history_size(), 1);
    /// ```
    pub fn execute(&mut self, action: Action) {
        // Remove any redo history when executing new action
        self.history.truncate(self.position);

        // Add new action
        self.history.push_back(action);
        self.position = self.history.len();

        // Trim history if needed
        while self.history.len() > self.max_history {
            self.history.pop_front();
            self.position = self.position.saturating_sub(1);
        }
    }

    /// Undo the last action
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{UndoRedoManager, Action};
    ///
    /// let mut manager = UndoRedoManager::new();
    /// manager.execute(Action::new("delete", "text"));
    ///
    /// let undone = manager.undo();
    /// assert!(undone.is_some());
    /// assert_eq!(undone.unwrap().name(), "delete");
    /// ```
    pub fn undo(&mut self) -> Option<Action> {
        if self.can_undo() {
            self.position -= 1;
            self.history.get(self.position).cloned()
        } else {
            None
        }
    }

    /// Redo the next action
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{UndoRedoManager, Action};
    ///
    /// let mut manager = UndoRedoManager::new();
    /// manager.execute(Action::new("delete", "text"));
    /// manager.undo();
    ///
    /// let redone = manager.redo();
    /// assert!(redone.is_some());
    /// ```
    pub fn redo(&mut self) -> Option<Action> {
        if self.can_redo() {
            let action = self.history.get(self.position).cloned();
            self.position += 1;
            action
        } else {
            None
        }
    }

    /// Check if undo is available
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{UndoRedoManager, Action};
    ///
    /// let mut manager = UndoRedoManager::new();
    /// assert!(!manager.can_undo());
    ///
    /// manager.execute(Action::new("test", "data"));
    /// assert!(manager.can_undo());
    /// ```
    pub fn can_undo(&self) -> bool {
        self.position > 0
    }

    /// Check if redo is available
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{UndoRedoManager, Action};
    ///
    /// let mut manager = UndoRedoManager::new();
    /// manager.execute(Action::new("test", "data"));
    /// assert!(!manager.can_redo());
    ///
    /// manager.undo();
    /// assert!(manager.can_redo());
    /// ```
    pub fn can_redo(&self) -> bool {
        self.position < self.history.len()
    }

    /// Get current position in history
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{UndoRedoManager, Action};
    ///
    /// let mut manager = UndoRedoManager::new();
    /// assert_eq!(manager.position(), 0);
    ///
    /// manager.execute(Action::new("test", "data"));
    /// assert_eq!(manager.position(), 1);
    /// ```
    pub fn position(&self) -> usize {
        self.position
    }

    /// Get total history size
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{UndoRedoManager, Action};
    ///
    /// let mut manager = UndoRedoManager::new();
    /// manager.execute(Action::new("a", "1"));
    /// manager.execute(Action::new("b", "2"));
    /// assert_eq!(manager.history_size(), 2);
    /// ```
    pub fn history_size(&self) -> usize {
        self.history.len()
    }

    /// Clear all history
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{UndoRedoManager, Action};
    ///
    /// let mut manager = UndoRedoManager::new();
    /// manager.execute(Action::new("test", "data"));
    /// manager.clear();
    /// assert_eq!(manager.history_size(), 0);
    /// ```
    pub fn clear(&mut self) {
        self.history.clear();
        self.position = 0;
    }

    /// Get action at current position - 1 (last executed)
    pub fn current_action(&self) -> Option<&Action> {
        if self.position > 0 {
            self.history.get(self.position - 1)
        } else {
            None
        }
    }

    /// Get all actions in history
    pub fn actions(&self) -> impl Iterator<Item = &Action> {
        self.history.iter()
    }

    /// Get undo-able actions
    pub fn undoable_actions(&self) -> impl Iterator<Item = &Action> {
        self.history.iter().take(self.position)
    }

    /// Get redo-able actions
    pub fn redoable_actions(&self) -> impl Iterator<Item = &Action> {
        self.history.iter().skip(self.position)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_new() {
        let action = Action::new("delete", "content");
        assert_eq!(action.name(), "delete");
        assert_eq!(action.before(), Some("content"));
        assert_eq!(action.after(), None);
    }

    #[test]
    fn test_action_with_states() {
        let action = Action::with_states("replace", "old", "new");
        assert_eq!(action.name(), "replace");
        assert_eq!(action.before(), Some("old"));
        assert_eq!(action.after(), Some("new"));
    }

    #[test]
    fn test_action_with_metadata() {
        let action = Action::new("delete", "content").with_metadata("line 10");
        assert_eq!(action.metadata(), Some("line 10"));
    }

    #[test]
    fn test_manager_new() {
        let manager = UndoRedoManager::new();
        assert_eq!(manager.history_size(), 0);
        assert_eq!(manager.position(), 0);
        assert!(!manager.can_undo());
        assert!(!manager.can_redo());
    }

    #[test]
    fn test_manager_default() {
        let manager = UndoRedoManager::default();
        assert_eq!(manager.history_size(), 0);
    }

    #[test]
    fn test_manager_with_max_history() {
        let manager = UndoRedoManager::with_max_history(50);
        assert_eq!(manager.max_history, 50);
    }

    #[test]
    fn test_execute() {
        let mut manager = UndoRedoManager::new();
        manager.execute(Action::new("action1", "data1"));
        manager.execute(Action::new("action2", "data2"));

        assert_eq!(manager.history_size(), 2);
        assert_eq!(manager.position(), 2);
    }

    #[test]
    fn test_undo() {
        let mut manager = UndoRedoManager::new();
        manager.execute(Action::new("action1", "data1"));
        manager.execute(Action::new("action2", "data2"));

        let undone = manager.undo();
        assert!(undone.is_some());
        assert_eq!(undone.unwrap().name(), "action2");
        assert_eq!(manager.position(), 1);
    }

    #[test]
    fn test_redo() {
        let mut manager = UndoRedoManager::new();
        manager.execute(Action::new("action1", "data1"));
        manager.undo();

        let redone = manager.redo();
        assert!(redone.is_some());
        assert_eq!(redone.unwrap().name(), "action1");
        assert_eq!(manager.position(), 1);
    }

    #[test]
    fn test_can_undo() {
        let mut manager = UndoRedoManager::new();
        assert!(!manager.can_undo());

        manager.execute(Action::new("test", "data"));
        assert!(manager.can_undo());

        manager.undo();
        assert!(!manager.can_undo());
    }

    #[test]
    fn test_can_redo() {
        let mut manager = UndoRedoManager::new();
        manager.execute(Action::new("test", "data"));
        assert!(!manager.can_redo());

        manager.undo();
        assert!(manager.can_redo());

        manager.redo();
        assert!(!manager.can_redo());
    }

    #[test]
    fn test_execute_clears_redo() {
        let mut manager = UndoRedoManager::new();
        manager.execute(Action::new("action1", "data1"));
        manager.execute(Action::new("action2", "data2"));
        manager.undo();

        assert!(manager.can_redo());

        manager.execute(Action::new("action3", "data3"));
        assert!(!manager.can_redo());
        assert_eq!(manager.history_size(), 2);
    }

    #[test]
    fn test_max_history() {
        let mut manager = UndoRedoManager::with_max_history(3);

        for i in 0..5 {
            manager.execute(Action::new(format!("action{}", i), format!("data{}", i)));
        }

        assert_eq!(manager.history_size(), 3);
    }

    #[test]
    fn test_clear() {
        let mut manager = UndoRedoManager::new();
        manager.execute(Action::new("action1", "data1"));
        manager.execute(Action::new("action2", "data2"));

        manager.clear();
        assert_eq!(manager.history_size(), 0);
        assert_eq!(manager.position(), 0);
    }

    #[test]
    fn test_current_action() {
        let mut manager = UndoRedoManager::new();
        assert!(manager.current_action().is_none());

        manager.execute(Action::new("action1", "data1"));
        assert_eq!(manager.current_action().unwrap().name(), "action1");

        manager.execute(Action::new("action2", "data2"));
        assert_eq!(manager.current_action().unwrap().name(), "action2");

        manager.undo();
        assert_eq!(manager.current_action().unwrap().name(), "action1");
    }

    #[test]
    fn test_actions_iterators() {
        let mut manager = UndoRedoManager::new();
        manager.execute(Action::new("a", "1"));
        manager.execute(Action::new("b", "2"));
        manager.execute(Action::new("c", "3"));
        manager.undo();

        let all: Vec<_> = manager.actions().map(|a| a.name()).collect();
        assert_eq!(all, vec!["a", "b", "c"]);

        let undoable: Vec<_> = manager.undoable_actions().map(|a| a.name()).collect();
        assert_eq!(undoable, vec!["a", "b"]);

        let redoable: Vec<_> = manager.redoable_actions().map(|a| a.name()).collect();
        assert_eq!(redoable, vec!["c"]);
    }
}
