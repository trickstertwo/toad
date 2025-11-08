/// Undo/Redo system for action history
///
/// Generic undo/redo stack supporting any action type
///
/// # Examples
///
/// ```
/// use toad::undo::{UndoStack, Action};
///
/// #[derive(Clone)]
/// struct TextAction { text: String };
///
/// impl Action for TextAction {
///     fn execute(&self) -> Result<(), String> { Ok(()) }
///     fn undo(&self) -> Result<(), String> { Ok(()) }
/// }
///
/// let mut stack = UndoStack::new();
/// stack.execute(TextAction { text: "hello".to_string() });
/// assert!(stack.can_undo());
/// ```
use serde::{Deserialize, Serialize};
use std::fmt;

/// Trait for undoable actions
pub trait Action: Clone {
    /// Execute the action
    fn execute(&self) -> Result<(), String>;

    /// Undo the action
    fn undo(&self) -> Result<(), String>;

    /// Optional: Get a description of this action
    fn description(&self) -> String {
        "Action".to_string()
    }
}

/// Undo/Redo stack
#[derive(Debug, Clone)]
pub struct UndoStack<A: Action> {
    /// Undo history (most recent at the end)
    undo_stack: Vec<A>,
    /// Redo history (most recent at the end)
    redo_stack: Vec<A>,
    /// Maximum number of actions to keep in history
    max_history: usize,
    /// Whether the stack has been modified since last save
    dirty: bool,
}

impl<A: Action> UndoStack<A> {
    /// Create a new undo stack
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::undo::UndoStack;
    ///
    /// #[derive(Clone)]
    /// struct MyAction;
    /// impl toad::undo::Action for MyAction {
    ///     fn execute(&self) -> Result<(), String> { Ok(()) }
    ///     fn undo(&self) -> Result<(), String> { Ok(()) }
    /// }
    ///
    /// let stack: UndoStack<MyAction> = UndoStack::new();
    /// assert_eq!(stack.undo_count(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            max_history: 1000,
            dirty: false,
        }
    }

    /// Create a new undo stack with custom history size
    pub fn with_max_history(max_history: usize) -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            max_history,
            dirty: false,
        }
    }

    /// Execute an action and add it to undo history
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::undo::{UndoStack, Action};
    ///
    /// #[derive(Clone)]
    /// struct Inc { value: i32 };
    /// impl Action for Inc {
    ///     fn execute(&self) -> Result<(), String> { Ok(()) }
    ///     fn undo(&self) -> Result<(), String> { Ok(()) }
    /// }
    ///
    /// let mut stack = UndoStack::new();
    /// stack.execute(Inc { value: 1 }).unwrap();
    /// assert_eq!(stack.undo_count(), 1);
    /// ```
    pub fn execute(&mut self, action: A) -> Result<(), String> {
        action.execute()?;

        // Executing a new action clears the redo stack
        self.redo_stack.clear();

        // Add to undo stack
        self.undo_stack.push(action);

        // Trim history if needed
        if self.undo_stack.len() > self.max_history {
            self.undo_stack.remove(0);
        }

        self.dirty = true;
        Ok(())
    }

    /// Undo the last action
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::undo::{UndoStack, Action};
    ///
    /// #[derive(Clone)]
    /// struct MyAction;
    /// impl Action for MyAction {
    ///     fn execute(&self) -> Result<(), String> { Ok(()) }
    ///     fn undo(&self) -> Result<(), String> { Ok(()) }
    /// }
    ///
    /// let mut stack = UndoStack::new();
    /// stack.execute(MyAction).unwrap();
    /// assert!(stack.can_undo());
    /// stack.undo().unwrap();
    /// assert!(!stack.can_undo());
    /// ```
    pub fn undo(&mut self) -> Result<(), String> {
        if let Some(action) = self.undo_stack.pop() {
            action.undo()?;
            self.redo_stack.push(action);
            self.dirty = true;
            Ok(())
        } else {
            Err("Nothing to undo".to_string())
        }
    }

    /// Redo the last undone action
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::undo::{UndoStack, Action};
    ///
    /// #[derive(Clone)]
    /// struct MyAction;
    /// impl Action for MyAction {
    ///     fn execute(&self) -> Result<(), String> { Ok(()) }
    ///     fn undo(&self) -> Result<(), String> { Ok(()) }
    /// }
    ///
    /// let mut stack = UndoStack::new();
    /// stack.execute(MyAction).unwrap();
    /// stack.undo().unwrap();
    /// assert!(stack.can_redo());
    /// stack.redo().unwrap();
    /// assert!(!stack.can_redo());
    /// ```
    pub fn redo(&mut self) -> Result<(), String> {
        if let Some(action) = self.redo_stack.pop() {
            action.execute()?;
            self.undo_stack.push(action);
            self.dirty = true;
            Ok(())
        } else {
            Err("Nothing to redo".to_string())
        }
    }

    /// Check if undo is available
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    /// Check if redo is available
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// Get the number of actions in undo history
    pub fn undo_count(&self) -> usize {
        self.undo_stack.len()
    }

    /// Get the number of actions in redo history
    pub fn redo_count(&self) -> usize {
        self.redo_stack.len()
    }

    /// Clear all history
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
        self.dirty = false;
    }

    /// Mark as saved (clears dirty flag)
    pub fn mark_saved(&mut self) {
        self.dirty = false;
    }

    /// Check if modified since last save
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// Get the last action description (for status display)
    pub fn last_action_description(&self) -> Option<String> {
        self.undo_stack.last().map(|a| a.description())
    }

    /// Get all undo action descriptions
    pub fn undo_descriptions(&self) -> Vec<String> {
        self.undo_stack.iter().map(|a| a.description()).collect()
    }

    /// Get all redo action descriptions
    pub fn redo_descriptions(&self) -> Vec<String> {
        self.redo_stack.iter().map(|a| a.description()).collect()
    }

    /// Peek at the last undoable action without removing it
    pub fn peek_undo(&self) -> Option<&A> {
        self.undo_stack.last()
    }

    /// Peek at the last redoable action without removing it
    pub fn peek_redo(&self) -> Option<&A> {
        self.redo_stack.last()
    }

    /// Begin a group of actions (all will be undone together)
    pub fn begin_group(&mut self) -> GroupGuard<A> {
        GroupGuard {
            stack: self,
            actions: Vec::new(),
        }
    }
}

impl<A: Action> Default for UndoStack<A> {
    fn default() -> Self {
        Self::new()
    }
}

/// Guard for grouped actions
pub struct GroupGuard<'a, A: Action> {
    stack: &'a mut UndoStack<A>,
    actions: Vec<A>,
}

impl<'a, A: Action> GroupGuard<'a, A> {
    /// Execute an action as part of the group
    pub fn execute(&mut self, action: A) -> Result<(), String> {
        action.execute()?;
        self.actions.push(action);
        Ok(())
    }

    /// Commit the group as a single undoable action
    pub fn commit(self) {
        if !self.actions.is_empty() {
            // In a real implementation, you'd wrap these in a GroupAction
            // For now, we'll just add them individually
            for action in self.actions {
                let _ = self.stack.execute(action);
            }
        }
    }

    /// Cancel the group without committing
    pub fn cancel(self) {
        // Undo all executed actions
        for _ in 0..self.actions.len() {
            let _ = self.actions.last().map(|a| a.undo());
        }
    }
}

/// A simple text edit action for examples/testing
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextInsert {
    /// Text to insert
    pub text: String,
    /// Position to insert at
    pub position: usize,
}

impl Action for TextInsert {
    fn execute(&self) -> Result<(), String> {
        // In a real implementation, this would modify the document
        Ok(())
    }

    fn undo(&self) -> Result<(), String> {
        // In a real implementation, this would remove the text
        Ok(())
    }

    fn description(&self) -> String {
        format!("Insert \"{}\" at {}", self.text, self.position)
    }
}

/// A simple text delete action for examples/testing
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextDelete {
    /// Text that was deleted
    pub text: String,
    /// Position where text was deleted from
    pub position: usize,
}

impl Action for TextDelete {
    fn execute(&self) -> Result<(), String> {
        // In a real implementation, this would delete the text
        Ok(())
    }

    fn undo(&self) -> Result<(), String> {
        // In a real implementation, this would restore the text
        Ok(())
    }

    fn description(&self) -> String {
        format!("Delete \"{}\" from {}", self.text, self.position)
    }
}

/// History navigator for browsing undo/redo history
#[derive(Debug, Clone)]
pub struct HistoryNavigator<A: Action> {
    /// Reference to the undo stack
    stack: UndoStack<A>,
    /// Current position in history
    position: usize,
}

impl<A: Action> HistoryNavigator<A> {
    /// Create a new history navigator
    pub fn new(stack: UndoStack<A>) -> Self {
        let position = stack.undo_count();
        Self { stack, position }
    }

    /// Move to previous state in history
    pub fn prev(&mut self) -> Result<(), String> {
        if self.position > 0 {
            self.position -= 1;
            self.stack.undo()?;
            Ok(())
        } else {
            Err("At beginning of history".to_string())
        }
    }

    /// Move to next state in history
    pub fn next(&mut self) -> Result<(), String> {
        if self.position < self.stack.undo_count() + self.stack.redo_count() {
            self.position += 1;
            self.stack.redo()?;
            Ok(())
        } else {
            Err("At end of history".to_string())
        }
    }

    /// Get current position
    pub fn position(&self) -> usize {
        self.position
    }

    /// Get total history length
    pub fn length(&self) -> usize {
        self.stack.undo_count() + self.stack.redo_count()
    }

    /// Check if can go back
    pub fn can_go_back(&self) -> bool {
        self.position > 0
    }

    /// Check if can go forward
    pub fn can_go_forward(&self) -> bool {
        self.position < self.length()
    }

    /// Get the underlying stack
    pub fn stack(&self) -> &UndoStack<A> {
        &self.stack
    }

    /// Get mutable access to the underlying stack
    pub fn stack_mut(&mut self) -> &mut UndoStack<A> {
        &mut self.stack
    }
}

impl<A: Action> fmt::Display for UndoStack<A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "UndoStack(undo: {}, redo: {}{})",
            self.undo_count(),
            self.redo_count(),
            if self.dirty { ", modified" } else { "" }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone)]
    struct CounterAction {
        value: i32,
        counter: std::rc::Rc<std::cell::RefCell<i32>>,
    }

    impl Action for CounterAction {
        fn execute(&self) -> Result<(), String> {
            *self.counter.borrow_mut() += self.value;
            Ok(())
        }

        fn undo(&self) -> Result<(), String> {
            *self.counter.borrow_mut() -= self.value;
            Ok(())
        }

        fn description(&self) -> String {
            format!("Add {}", self.value)
        }
    }

    #[test]
    fn test_undo_stack_creation() {
        let stack: UndoStack<TextInsert> = UndoStack::new();
        assert_eq!(stack.undo_count(), 0);
        assert_eq!(stack.redo_count(), 0);
        assert!(!stack.can_undo());
        assert!(!stack.can_redo());
    }

    #[test]
    fn test_execute_action() {
        let counter = std::rc::Rc::new(std::cell::RefCell::new(0));
        let mut stack = UndoStack::new();

        let action = CounterAction {
            value: 5,
            counter: counter.clone(),
        };
        stack.execute(action).unwrap();

        assert_eq!(*counter.borrow(), 5);
        assert_eq!(stack.undo_count(), 1);
        assert!(stack.can_undo());
    }

    #[test]
    fn test_undo_action() {
        let counter = std::rc::Rc::new(std::cell::RefCell::new(0));
        let mut stack = UndoStack::new();

        let action = CounterAction {
            value: 5,
            counter: counter.clone(),
        };
        stack.execute(action).unwrap();
        stack.undo().unwrap();

        assert_eq!(*counter.borrow(), 0);
        assert_eq!(stack.undo_count(), 0);
        assert_eq!(stack.redo_count(), 1);
        assert!(stack.can_redo());
    }

    #[test]
    fn test_redo_action() {
        let counter = std::rc::Rc::new(std::cell::RefCell::new(0));
        let mut stack = UndoStack::new();

        let action = CounterAction {
            value: 5,
            counter: counter.clone(),
        };
        stack.execute(action).unwrap();
        stack.undo().unwrap();
        stack.redo().unwrap();

        assert_eq!(*counter.borrow(), 5);
        assert_eq!(stack.undo_count(), 1);
        assert_eq!(stack.redo_count(), 0);
    }

    #[test]
    fn test_multiple_actions() {
        let counter = std::rc::Rc::new(std::cell::RefCell::new(0));
        let mut stack = UndoStack::new();

        stack
            .execute(CounterAction {
                value: 1,
                counter: counter.clone(),
            })
            .unwrap();
        stack
            .execute(CounterAction {
                value: 2,
                counter: counter.clone(),
            })
            .unwrap();
        stack
            .execute(CounterAction {
                value: 3,
                counter: counter.clone(),
            })
            .unwrap();

        assert_eq!(*counter.borrow(), 6);
        assert_eq!(stack.undo_count(), 3);
    }

    #[test]
    fn test_undo_multiple() {
        let counter = std::rc::Rc::new(std::cell::RefCell::new(0));
        let mut stack = UndoStack::new();

        stack
            .execute(CounterAction {
                value: 1,
                counter: counter.clone(),
            })
            .unwrap();
        stack
            .execute(CounterAction {
                value: 2,
                counter: counter.clone(),
            })
            .unwrap();
        stack
            .execute(CounterAction {
                value: 3,
                counter: counter.clone(),
            })
            .unwrap();

        stack.undo().unwrap();
        stack.undo().unwrap();

        assert_eq!(*counter.borrow(), 1);
        assert_eq!(stack.undo_count(), 1);
        assert_eq!(stack.redo_count(), 2);
    }

    #[test]
    fn test_new_action_clears_redo() {
        let counter = std::rc::Rc::new(std::cell::RefCell::new(0));
        let mut stack = UndoStack::new();

        stack
            .execute(CounterAction {
                value: 1,
                counter: counter.clone(),
            })
            .unwrap();
        stack.undo().unwrap();

        assert_eq!(stack.redo_count(), 1);

        stack
            .execute(CounterAction {
                value: 2,
                counter: counter.clone(),
            })
            .unwrap();

        assert_eq!(stack.redo_count(), 0);
    }

    #[test]
    fn test_max_history() {
        let counter = std::rc::Rc::new(std::cell::RefCell::new(0));
        let mut stack = UndoStack::with_max_history(3);

        for i in 1..=5 {
            stack
                .execute(CounterAction {
                    value: i,
                    counter: counter.clone(),
                })
                .unwrap();
        }

        assert_eq!(stack.undo_count(), 3);
    }

    #[test]
    fn test_clear() {
        let counter = std::rc::Rc::new(std::cell::RefCell::new(0));
        let mut stack = UndoStack::new();

        stack
            .execute(CounterAction {
                value: 1,
                counter: counter.clone(),
            })
            .unwrap();
        stack.clear();

        assert_eq!(stack.undo_count(), 0);
        assert_eq!(stack.redo_count(), 0);
        assert!(!stack.is_dirty());
    }

    #[test]
    fn test_dirty_flag() {
        let counter = std::rc::Rc::new(std::cell::RefCell::new(0));
        let mut stack = UndoStack::new();

        assert!(!stack.is_dirty());

        stack
            .execute(CounterAction {
                value: 1,
                counter: counter.clone(),
            })
            .unwrap();
        assert!(stack.is_dirty());

        stack.mark_saved();
        assert!(!stack.is_dirty());

        stack.undo().unwrap();
        assert!(stack.is_dirty());
    }

    #[test]
    fn test_descriptions() {
        let counter = std::rc::Rc::new(std::cell::RefCell::new(0));
        let mut stack = UndoStack::new();

        stack
            .execute(CounterAction {
                value: 5,
                counter: counter.clone(),
            })
            .unwrap();

        assert_eq!(stack.last_action_description(), Some("Add 5".to_string()));
        assert_eq!(stack.undo_descriptions(), vec!["Add 5"]);
    }

    #[test]
    fn test_peek() {
        let counter = std::rc::Rc::new(std::cell::RefCell::new(0));
        let mut stack = UndoStack::new();

        stack
            .execute(CounterAction {
                value: 5,
                counter: counter.clone(),
            })
            .unwrap();

        assert!(stack.peek_undo().is_some());
        assert!(stack.peek_redo().is_none());

        stack.undo().unwrap();

        assert!(stack.peek_undo().is_none());
        assert!(stack.peek_redo().is_some());
    }

    #[test]
    fn test_text_insert_action() {
        let action = TextInsert {
            text: "hello".to_string(),
            position: 0,
        };

        assert!(action.execute().is_ok());
        assert!(action.undo().is_ok());
        assert_eq!(action.description(), "Insert \"hello\" at 0");
    }

    #[test]
    fn test_text_delete_action() {
        let action = TextDelete {
            text: "world".to_string(),
            position: 5,
        };

        assert!(action.execute().is_ok());
        assert!(action.undo().is_ok());
        assert_eq!(action.description(), "Delete \"world\" from 5");
    }

    #[test]
    fn test_history_navigator() {
        let counter = std::rc::Rc::new(std::cell::RefCell::new(0));
        let mut stack = UndoStack::new();

        stack
            .execute(CounterAction {
                value: 1,
                counter: counter.clone(),
            })
            .unwrap();
        stack
            .execute(CounterAction {
                value: 2,
                counter: counter.clone(),
            })
            .unwrap();

        let mut navigator = HistoryNavigator::new(stack);

        assert_eq!(navigator.position(), 2);
        assert_eq!(navigator.length(), 2);
        assert!(navigator.can_go_back());
        assert!(!navigator.can_go_forward());

        navigator.prev().unwrap();
        assert_eq!(navigator.position(), 1);
        assert!(navigator.can_go_forward());
    }

    #[test]
    fn test_undo_stack_display() {
        let stack: UndoStack<TextInsert> = UndoStack::new();
        let display = format!("{}", stack);
        assert!(display.contains("undo: 0"));
        assert!(display.contains("redo: 0"));
    }

    #[test]
    fn test_cannot_undo_empty() {
        let mut stack: UndoStack<TextInsert> = UndoStack::new();
        assert!(stack.undo().is_err());
    }

    #[test]
    fn test_cannot_redo_empty() {
        let mut stack: UndoStack<TextInsert> = UndoStack::new();
        assert!(stack.redo().is_err());
    }
}
