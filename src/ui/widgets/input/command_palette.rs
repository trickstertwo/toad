//! Command mode widget for ex-style command input
//!
//! Provides vim-style command mode with `:` prefix for executing commands.
//!
//! # Examples
//!
//! ```
//! use toad::widgets::CommandMode;
//!
//! let mut cmd_mode = CommandMode::new();
//! cmd_mode.activate();
//!
//! cmd_mode.insert_char('w');
//! cmd_mode.insert_char('q');
//! assert_eq!(cmd_mode.input(), "wq");
//! ```

use std::collections::VecDeque;

/// Command mode widget
///
/// Provides ex-style command input with history and autocomplete support.
///
/// # Examples
///
/// ```
/// use toad::widgets::CommandMode;
///
/// let mut cmd = CommandMode::new();
/// cmd.activate();
///
/// cmd.insert_char('w');
/// assert_eq!(cmd.input(), "w");
/// assert!(cmd.is_active());
/// ```
#[derive(Debug, Clone)]
pub struct CommandMode {
    /// Current input buffer
    input: String,
    /// Cursor position in input
    cursor: usize,
    /// Whether command mode is active
    active: bool,
    /// Command history
    history: VecDeque<String>,
    /// Current position in history
    history_pos: Option<usize>,
    /// Maximum history size
    max_history: usize,
    /// Last executed command
    last_command: Option<String>,
    /// Temporary buffer for history navigation
    temp_input: Option<String>,
}

impl Default for CommandMode {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandMode {
    /// Create a new command mode
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::CommandMode;
    ///
    /// let cmd = CommandMode::new();
    /// assert!(!cmd.is_active());
    /// assert_eq!(cmd.input(), "");
    /// ```
    pub fn new() -> Self {
        Self {
            input: String::new(),
            cursor: 0,
            active: false,
            history: VecDeque::new(),
            history_pos: None,
            max_history: 100,
            last_command: None,
            temp_input: None,
        }
    }

    /// Create command mode with custom max history
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::CommandMode;
    ///
    /// let cmd = CommandMode::with_max_history(50);
    /// ```
    pub fn with_max_history(max: usize) -> Self {
        Self {
            input: String::new(),
            cursor: 0,
            active: false,
            history: VecDeque::new(),
            history_pos: None,
            max_history: max,
            last_command: None,
            temp_input: None,
        }
    }

    /// Activate command mode
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::CommandMode;
    ///
    /// let mut cmd = CommandMode::new();
    /// cmd.activate();
    /// assert!(cmd.is_active());
    /// ```
    pub fn activate(&mut self) {
        self.active = true;
        self.input.clear();
        self.cursor = 0;
        self.history_pos = None;
        self.temp_input = None;
    }

    /// Deactivate command mode
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::CommandMode;
    ///
    /// let mut cmd = CommandMode::new();
    /// cmd.activate();
    /// cmd.deactivate();
    /// assert!(!cmd.is_active());
    /// ```
    pub fn deactivate(&mut self) {
        self.active = false;
        self.history_pos = None;
        self.temp_input = None;
    }

    /// Toggle command mode
    pub fn toggle(&mut self) {
        if self.active {
            self.deactivate();
        } else {
            self.activate();
        }
    }

    /// Check if command mode is active
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Get current input
    pub fn input(&self) -> &str {
        &self.input
    }

    /// Get cursor position
    pub fn cursor(&self) -> usize {
        self.cursor
    }

    /// Insert character at cursor
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::CommandMode;
    ///
    /// let mut cmd = CommandMode::new();
    /// cmd.insert_char('w');
    /// cmd.insert_char('q');
    /// assert_eq!(cmd.input(), "wq");
    /// ```
    pub fn insert_char(&mut self, c: char) {
        self.input.insert(self.cursor, c);
        self.cursor += 1;
        self.history_pos = None;
        self.temp_input = None;
    }

    /// Delete character before cursor
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::CommandMode;
    ///
    /// let mut cmd = CommandMode::new();
    /// cmd.insert_char('w');
    /// cmd.insert_char('q');
    /// cmd.backspace();
    /// assert_eq!(cmd.input(), "w");
    /// ```
    pub fn backspace(&mut self) -> bool {
        if self.cursor > 0 {
            self.cursor -= 1;
            self.input.remove(self.cursor);
            self.history_pos = None;
            self.temp_input = None;
            true
        } else {
            false
        }
    }

    /// Delete character at cursor
    pub fn delete(&mut self) -> bool {
        if self.cursor < self.input.len() {
            self.input.remove(self.cursor);
            self.history_pos = None;
            self.temp_input = None;
            true
        } else {
            false
        }
    }

    /// Move cursor left
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::CommandMode;
    ///
    /// let mut cmd = CommandMode::new();
    /// cmd.insert_char('w');
    /// cmd.insert_char('q');
    /// cmd.move_left();
    /// assert_eq!(cmd.cursor(), 1);
    /// ```
    pub fn move_left(&mut self) -> bool {
        if self.cursor > 0 {
            self.cursor -= 1;
            true
        } else {
            false
        }
    }

    /// Move cursor right
    pub fn move_right(&mut self) -> bool {
        if self.cursor < self.input.len() {
            self.cursor += 1;
            true
        } else {
            false
        }
    }

    /// Move cursor to start
    pub fn move_to_start(&mut self) {
        self.cursor = 0;
    }

    /// Move cursor to end
    pub fn move_to_end(&mut self) {
        self.cursor = self.input.len();
    }

    /// Clear input
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::CommandMode;
    ///
    /// let mut cmd = CommandMode::new();
    /// cmd.insert_char('w');
    /// cmd.clear();
    /// assert_eq!(cmd.input(), "");
    /// ```
    pub fn clear(&mut self) {
        self.input.clear();
        self.cursor = 0;
        self.history_pos = None;
        self.temp_input = None;
    }

    /// Execute current command and add to history
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::CommandMode;
    ///
    /// let mut cmd = CommandMode::new();
    /// cmd.insert_char('w');
    /// cmd.insert_char('q');
    ///
    /// let command = cmd.execute();
    /// assert_eq!(command, Some("wq".to_string()));
    /// assert_eq!(cmd.history_count(), 1);
    /// ```
    pub fn execute(&mut self) -> Option<String> {
        if self.input.is_empty() {
            return None;
        }

        let command = self.input.clone();
        self.add_to_history(command.clone());
        self.last_command = Some(command.clone());
        self.clear();
        self.deactivate();

        Some(command)
    }

    /// Add command to history
    fn add_to_history(&mut self, command: String) {
        // Don't add duplicate of last command
        if self.history.back() == Some(&command) {
            return;
        }

        self.history.push_back(command);

        // Trim history if needed
        while self.history.len() > self.max_history {
            self.history.pop_front();
        }
    }

    /// Navigate to previous command in history
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::CommandMode;
    ///
    /// let mut cmd = CommandMode::new();
    /// cmd.insert_char('w');
    /// cmd.execute();
    /// cmd.activate();
    ///
    /// cmd.history_previous();
    /// assert_eq!(cmd.input(), "w");
    /// ```
    pub fn history_previous(&mut self) -> bool {
        if self.history.is_empty() {
            return false;
        }

        // Save current input on first history navigation
        if self.history_pos.is_none() {
            self.temp_input = Some(self.input.clone());
        }

        let new_pos = match self.history_pos {
            None => self.history.len() - 1,
            Some(pos) if pos > 0 => pos - 1,
            Some(_) => return false,
        };

        self.history_pos = Some(new_pos);
        if let Some(cmd) = self.history.get(new_pos) {
            self.input = cmd.clone();
            self.cursor = self.input.len();
            true
        } else {
            false
        }
    }

    /// Navigate to next command in history
    pub fn history_next(&mut self) -> bool {
        let Some(pos) = self.history_pos else {
            return false;
        };

        if pos + 1 < self.history.len() {
            self.history_pos = Some(pos + 1);
            if let Some(cmd) = self.history.get(pos + 1) {
                self.input = cmd.clone();
                self.cursor = self.input.len();
                return true;
            }
        } else {
            // Restore temporary input
            if let Some(temp) = self.temp_input.take() {
                self.input = temp;
                self.cursor = self.input.len();
                self.history_pos = None;
                return true;
            }
        }

        false
    }

    /// Get command history
    pub fn history(&self) -> &VecDeque<String> {
        &self.history
    }

    /// Get number of commands in history
    pub fn history_count(&self) -> usize {
        self.history.len()
    }

    /// Get last executed command
    pub fn last_command(&self) -> Option<&str> {
        self.last_command.as_deref()
    }

    /// Clear history
    pub fn clear_history(&mut self) {
        self.history.clear();
        self.history_pos = None;
        self.last_command = None;
    }

    /// Set input (for testing or autocomplete)
    pub fn set_input(&mut self, input: String) {
        self.input = input;
        self.cursor = self.input.len();
        self.history_pos = None;
        self.temp_input = None;
    }

    /// Get prompt string for display
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::CommandMode;
    ///
    /// let cmd = CommandMode::new();
    /// assert_eq!(cmd.prompt(), ":");
    /// ```
    pub fn prompt(&self) -> &str {
        ":"
    }

    /// Get full display string (prompt + input)
    pub fn display_string(&self) -> String {
        format!("{}{}", self.prompt(), self.input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_mode_new() {
        let cmd = CommandMode::new();
        assert!(!cmd.is_active());
        assert_eq!(cmd.input(), "");
        assert_eq!(cmd.cursor(), 0);
        assert_eq!(cmd.history_count(), 0);
    }

    #[test]
    fn test_command_mode_default() {
        let cmd = CommandMode::default();
        assert!(!cmd.is_active());
    }

    #[test]
    fn test_command_mode_with_max_history() {
        let cmd = CommandMode::with_max_history(50);
        assert_eq!(cmd.max_history, 50);
    }

    #[test]
    fn test_activate_deactivate() {
        let mut cmd = CommandMode::new();
        assert!(!cmd.is_active());

        cmd.activate();
        assert!(cmd.is_active());

        cmd.deactivate();
        assert!(!cmd.is_active());
    }

    #[test]
    fn test_toggle() {
        let mut cmd = CommandMode::new();
        cmd.toggle();
        assert!(cmd.is_active());
        cmd.toggle();
        assert!(!cmd.is_active());
    }

    #[test]
    fn test_insert_char() {
        let mut cmd = CommandMode::new();
        cmd.insert_char('w');
        cmd.insert_char('q');
        assert_eq!(cmd.input(), "wq");
        assert_eq!(cmd.cursor(), 2);
    }

    #[test]
    fn test_backspace() {
        let mut cmd = CommandMode::new();
        cmd.insert_char('w');
        cmd.insert_char('q');

        assert!(cmd.backspace());
        assert_eq!(cmd.input(), "w");
        assert_eq!(cmd.cursor(), 1);

        assert!(cmd.backspace());
        assert_eq!(cmd.input(), "");
        assert_eq!(cmd.cursor(), 0);

        assert!(!cmd.backspace());
    }

    #[test]
    fn test_delete() {
        let mut cmd = CommandMode::new();
        cmd.insert_char('w');
        cmd.insert_char('q');
        cmd.move_to_start();

        assert!(cmd.delete());
        assert_eq!(cmd.input(), "q");

        assert!(cmd.delete());
        assert_eq!(cmd.input(), "");

        assert!(!cmd.delete());
    }

    #[test]
    fn test_move_cursor() {
        let mut cmd = CommandMode::new();
        cmd.insert_char('w');
        cmd.insert_char('q');

        assert!(cmd.move_left());
        assert_eq!(cmd.cursor(), 1);

        assert!(cmd.move_left());
        assert_eq!(cmd.cursor(), 0);

        assert!(!cmd.move_left());

        assert!(cmd.move_right());
        assert_eq!(cmd.cursor(), 1);

        assert!(cmd.move_right());
        assert_eq!(cmd.cursor(), 2);

        assert!(!cmd.move_right());
    }

    #[test]
    fn test_move_to_start_end() {
        let mut cmd = CommandMode::new();
        cmd.insert_char('w');
        cmd.insert_char('q');

        cmd.move_to_start();
        assert_eq!(cmd.cursor(), 0);

        cmd.move_to_end();
        assert_eq!(cmd.cursor(), 2);
    }

    #[test]
    fn test_clear() {
        let mut cmd = CommandMode::new();
        cmd.insert_char('w');
        cmd.insert_char('q');
        cmd.clear();
        assert_eq!(cmd.input(), "");
        assert_eq!(cmd.cursor(), 0);
    }

    #[test]
    fn test_execute() {
        let mut cmd = CommandMode::new();
        cmd.activate();
        cmd.insert_char('w');
        cmd.insert_char('q');

        let command = cmd.execute();
        assert_eq!(command, Some("wq".to_string()));
        assert_eq!(cmd.input(), "");
        assert!(!cmd.is_active());
        assert_eq!(cmd.history_count(), 1);
        assert_eq!(cmd.last_command(), Some("wq"));
    }

    #[test]
    fn test_execute_empty() {
        let mut cmd = CommandMode::new();
        let command = cmd.execute();
        assert_eq!(command, None);
        assert_eq!(cmd.history_count(), 0);
    }

    #[test]
    fn test_history_navigation() {
        let mut cmd = CommandMode::new();

        // Add commands to history
        cmd.insert_char('w');
        cmd.execute();

        cmd.activate();
        cmd.insert_char('q');
        cmd.execute();

        cmd.activate();

        // Navigate back
        assert!(cmd.history_previous());
        assert_eq!(cmd.input(), "q");

        assert!(cmd.history_previous());
        assert_eq!(cmd.input(), "w");

        assert!(!cmd.history_previous());

        // Navigate forward
        assert!(cmd.history_next());
        assert_eq!(cmd.input(), "q");

        assert!(cmd.history_next());
        assert_eq!(cmd.input(), "");
    }

    #[test]
    fn test_history_temp_input() {
        let mut cmd = CommandMode::new();
        cmd.insert_char('w');
        cmd.execute();

        cmd.activate();
        cmd.insert_char('t');
        cmd.insert_char('e');
        cmd.insert_char('s');
        cmd.insert_char('t');

        // Navigate to history
        cmd.history_previous();
        assert_eq!(cmd.input(), "w");

        // Navigate back to temp input
        cmd.history_next();
        assert_eq!(cmd.input(), "test");
    }

    #[test]
    fn test_history_max_size() {
        let mut cmd = CommandMode::with_max_history(3);

        for i in 0..5 {
            cmd.insert_char(char::from_digit(i, 10).unwrap());
            cmd.execute();
            cmd.activate();
        }

        assert_eq!(cmd.history_count(), 3);
        assert_eq!(cmd.history().front(), Some(&"2".to_string()));
        assert_eq!(cmd.history().back(), Some(&"4".to_string()));
    }

    #[test]
    fn test_history_no_duplicates() {
        let mut cmd = CommandMode::new();
        cmd.insert_char('w');
        cmd.execute();

        cmd.activate();
        cmd.insert_char('w');
        cmd.execute();

        assert_eq!(cmd.history_count(), 1);
    }

    #[test]
    fn test_clear_history() {
        let mut cmd = CommandMode::new();
        cmd.insert_char('w');
        cmd.execute();

        assert_eq!(cmd.history_count(), 1);
        cmd.clear_history();
        assert_eq!(cmd.history_count(), 0);
        assert_eq!(cmd.last_command(), None);
    }

    #[test]
    fn test_set_input() {
        let mut cmd = CommandMode::new();
        cmd.set_input("test".to_string());
        assert_eq!(cmd.input(), "test");
        assert_eq!(cmd.cursor(), 4);
    }

    #[test]
    fn test_prompt() {
        let cmd = CommandMode::new();
        assert_eq!(cmd.prompt(), ":");
    }

    #[test]
    fn test_display_string() {
        let mut cmd = CommandMode::new();
        cmd.insert_char('w');
        cmd.insert_char('q');
        assert_eq!(cmd.display_string(), ":wq");
    }

    #[test]
    fn test_activate_clears_input() {
        let mut cmd = CommandMode::new();
        cmd.insert_char('w');
        cmd.activate();
        assert_eq!(cmd.input(), "");
        assert_eq!(cmd.cursor(), 0);
    }

    #[test]
    fn test_history_position_reset_on_edit() {
        let mut cmd = CommandMode::new();
        cmd.insert_char('w');
        cmd.execute();

        cmd.activate();
        cmd.history_previous();
        assert_eq!(cmd.input(), "w");

        cmd.insert_char('q');
        assert!(cmd.history_pos.is_none());
    }
}
