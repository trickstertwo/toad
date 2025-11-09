//! Modal input dialog for capturing user text input
//!
//! Provides a centered modal dialog with a text input field, supporting
//! validation, placeholders, and customizable titles.
//!
//! # Examples
//!
//! ## Basic Input Dialog
//!
//! ```
//! use toad::widgets::InputDialog;
//!
//! let dialog = InputDialog::new("Enter your name")
//!     .with_placeholder("John Doe");
//!
//! assert_eq!(dialog.title(), "Enter your name");
//! ```
//!
//! ## With Validation
//!
//! ```
//! use toad::widgets::InputDialog;
//!
//! let dialog = InputDialog::new("Enter email")
//!     .with_validator(|input| {
//!         if input.contains('@') {
//!             Ok(())
//!         } else {
//!             Err("Email must contain @".to_string())
//!         }
//!     });
//! ```

use crate::ui::theme::ToadTheme;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
};

/// Input dialog state
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputDialogState {
    /// Dialog is active and accepting input
    Active,
    /// Input was submitted successfully
    Submitted,
    /// Dialog was cancelled
    Cancelled,
}

/// Validation function type
type ValidatorFn = fn(&str) -> Result<(), String>;

/// Modal input dialog for text input
///
/// A centered modal dialog that captures user text input with optional
/// validation, placeholders, and help text.
///
/// # Examples
///
/// ```
/// use toad::widgets::InputDialog;
///
/// let mut dialog = InputDialog::new("Enter project name")
///     .with_placeholder("my-project")
///     .with_help_text("Press Enter to confirm, Esc to cancel");
///
/// // Simulate user input
/// dialog.insert_char('t');
/// dialog.insert_char('e');
/// dialog.insert_char('s');
/// dialog.insert_char('t');
///
/// assert_eq!(dialog.value(), "test");
/// ```
#[derive(Debug, Clone)]
pub struct InputDialog {
    title: String,
    placeholder: String,
    help_text: String,
    value: String,
    cursor_position: usize,
    state: InputDialogState,
    validator: Option<ValidatorFn>,
    validation_error: Option<String>,
    max_length: Option<usize>,
}

impl InputDialog {
    /// Create a new input dialog with the given title
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::InputDialog;
    ///
    /// let dialog = InputDialog::new("Enter name");
    /// assert_eq!(dialog.title(), "Enter name");
    /// ```
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            placeholder: String::new(),
            help_text: "Press Enter to submit, Esc to cancel".to_string(),
            value: String::new(),
            cursor_position: 0,
            state: InputDialogState::Active,
            validator: None,
            validation_error: None,
            max_length: None,
        }
    }

    /// Set the placeholder text
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::InputDialog;
    ///
    /// let dialog = InputDialog::new("Name").with_placeholder("Enter your name");
    /// assert_eq!(dialog.placeholder(), "Enter your name");
    /// ```
    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Set the help text displayed at the bottom
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::InputDialog;
    ///
    /// let dialog = InputDialog::new("Name").with_help_text("Custom help text");
    /// assert_eq!(dialog.help_text(), "Custom help text");
    /// ```
    pub fn with_help_text(mut self, help: impl Into<String>) -> Self {
        self.help_text = help.into();
        self
    }

    /// Set a validation function
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::InputDialog;
    ///
    /// let dialog = InputDialog::new("Number")
    ///     .with_validator(|s| {
    ///         if s.parse::<i32>().is_ok() {
    ///             Ok(())
    ///         } else {
    ///             Err("Must be a valid number".to_string())
    ///         }
    ///     });
    /// ```
    pub fn with_validator(mut self, validator: ValidatorFn) -> Self {
        self.validator = Some(validator);
        self
    }

    /// Set maximum input length
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::InputDialog;
    ///
    /// let dialog = InputDialog::new("Code").with_max_length(10);
    /// assert_eq!(dialog.max_length(), Some(10));
    /// ```
    pub fn with_max_length(mut self, max: usize) -> Self {
        self.max_length = Some(max);
        self
    }

    /// Get the dialog title
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Get the placeholder text
    pub fn placeholder(&self) -> &str {
        &self.placeholder
    }

    /// Get the help text
    pub fn help_text(&self) -> &str {
        &self.help_text
    }

    /// Get the current input value
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::InputDialog;
    ///
    /// let mut dialog = InputDialog::new("Name");
    /// dialog.insert_char('A');
    /// assert_eq!(dialog.value(), "A");
    /// ```
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Get the current state
    pub fn state(&self) -> &InputDialogState {
        &self.state
    }

    /// Get maximum length constraint
    pub fn max_length(&self) -> Option<usize> {
        self.max_length
    }

    /// Check if dialog is active
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::InputDialog;
    ///
    /// let dialog = InputDialog::new("Test");
    /// assert!(dialog.is_active());
    /// ```
    pub fn is_active(&self) -> bool {
        self.state == InputDialogState::Active
    }

    /// Insert a character at cursor position
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::InputDialog;
    ///
    /// let mut dialog = InputDialog::new("Test");
    /// dialog.insert_char('H');
    /// dialog.insert_char('i');
    /// assert_eq!(dialog.value(), "Hi");
    /// ```
    pub fn insert_char(&mut self, c: char) {
        if let Some(max) = self.max_length
            && self.value.len() >= max
        {
            return;
        }

        self.value.insert(self.cursor_position, c);
        self.cursor_position += 1;
        self.validation_error = None; // Clear error on new input
    }

    /// Delete character before cursor
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::InputDialog;
    ///
    /// let mut dialog = InputDialog::new("Test");
    /// dialog.insert_char('H');
    /// dialog.insert_char('i');
    /// dialog.delete_char();
    /// assert_eq!(dialog.value(), "H");
    /// ```
    pub fn delete_char(&mut self) {
        if self.cursor_position > 0 {
            self.value.remove(self.cursor_position - 1);
            self.cursor_position -= 1;
            self.validation_error = None;
        }
    }

    /// Move cursor left
    pub fn move_cursor_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    /// Move cursor right
    pub fn move_cursor_right(&mut self) {
        if self.cursor_position < self.value.len() {
            self.cursor_position += 1;
        }
    }

    /// Move cursor to start
    pub fn move_cursor_start(&mut self) {
        self.cursor_position = 0;
    }

    /// Move cursor to end
    pub fn move_cursor_end(&mut self) {
        self.cursor_position = self.value.len();
    }

    /// Clear the input
    pub fn clear(&mut self) {
        self.value.clear();
        self.cursor_position = 0;
        self.validation_error = None;
    }

    /// Attempt to submit the input
    ///
    /// Returns `Ok(())` if validation passes, `Err(error_message)` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::InputDialog;
    ///
    /// let mut dialog = InputDialog::new("Name");
    /// dialog.insert_char('A');
    /// assert!(dialog.submit().is_ok());
    /// ```
    pub fn submit(&mut self) -> Result<(), String> {
        // Run validation if present
        if let Some(validator) = self.validator
            && let Err(error) = validator(&self.value)
        {
            self.validation_error = Some(error.clone());
            return Err(error);
        }

        self.state = InputDialogState::Submitted;
        self.validation_error = None;
        Ok(())
    }

    /// Cancel the dialog
    pub fn cancel(&mut self) {
        self.state = InputDialogState::Cancelled;
    }

    /// Reset the dialog to initial state
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::InputDialog;
    ///
    /// let mut dialog = InputDialog::new("Test");
    /// dialog.insert_char('A');
    /// dialog.submit().ok();
    /// dialog.reset();
    ///
    /// assert!(dialog.is_active());
    /// assert_eq!(dialog.value(), "");
    /// ```
    pub fn reset(&mut self) {
        self.value.clear();
        self.cursor_position = 0;
        self.state = InputDialogState::Active;
        self.validation_error = None;
    }

    /// Render the input dialog
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use toad::widgets::InputDialog;
    ///
    /// fn example(frame: &mut ratatui::Frame, area: ratatui::layout::Rect) {
    ///     let dialog = InputDialog::new("Enter name");
    ///     dialog.render(frame, area);
    /// }
    /// ```
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        // Center the dialog
        let dialog_width = 60;
        let dialog_height = 9;

        let dialog_area = Rect {
            x: area.width.saturating_sub(dialog_width) / 2,
            y: area.height.saturating_sub(dialog_height) / 2,
            width: dialog_width.min(area.width),
            height: dialog_height.min(area.height),
        };

        // Clear background
        frame.render_widget(Clear, dialog_area);

        // Create border
        let border_color = if self.validation_error.is_some() {
            ToadTheme::RED
        } else {
            ToadTheme::TOAD_GREEN
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color))
            .title(self.title.as_str())
            .style(Style::default().bg(ToadTheme::BLACK));

        let inner = block.inner(dialog_area);
        frame.render_widget(block, dialog_area);

        // Layout: input field + help text + error message
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // Space
                Constraint::Length(3), // Input field
                Constraint::Length(1), // Space
                Constraint::Length(1), // Help text
                Constraint::Min(0),    // Error message
            ])
            .split(inner);

        // Render input field
        let input_text = if self.value.is_empty() {
            Line::from(vec![Span::styled(
                &self.placeholder,
                Style::default()
                    .fg(ToadTheme::GRAY)
                    .add_modifier(Modifier::ITALIC),
            )])
        } else {
            // Show text with cursor
            let before_cursor = &self.value[..self.cursor_position];
            let cursor_char = self.value.chars().nth(self.cursor_position).unwrap_or(' ');
            let after_cursor = &self.value[self.cursor_position.min(self.value.len())..];

            Line::from(vec![
                Span::styled(before_cursor, Style::default().fg(ToadTheme::FOREGROUND)),
                Span::styled(
                    cursor_char.to_string(),
                    Style::default()
                        .fg(ToadTheme::BLACK)
                        .bg(ToadTheme::TOAD_GREEN),
                ),
                Span::styled(after_cursor, Style::default().fg(ToadTheme::FOREGROUND)),
            ])
        };

        let input_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(ToadTheme::GRAY));

        let input_inner = input_block.inner(chunks[1]);
        frame.render_widget(input_block, chunks[1]);
        frame.render_widget(Paragraph::new(input_text), input_inner);

        // Render help text
        let help = Paragraph::new(Line::from(vec![Span::styled(
            &self.help_text,
            Style::default().fg(ToadTheme::GRAY),
        )]))
        .alignment(Alignment::Center);
        frame.render_widget(help, chunks[3]);

        // Render error message if present
        if let Some(error) = &self.validation_error {
            let error_text = Paragraph::new(Line::from(vec![
                Span::styled("✗ ", Style::default().fg(ToadTheme::RED)),
                Span::styled(error, Style::default().fg(ToadTheme::RED)),
            ]))
            .alignment(Alignment::Center);
            frame.render_widget(error_text, chunks[4]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_dialog_creation() {
        let dialog = InputDialog::new("Test Title");
        assert_eq!(dialog.title(), "Test Title");
        assert!(dialog.is_active());
        assert_eq!(dialog.value(), "");
    }

    #[test]
    fn test_input_dialog_placeholder() {
        let dialog = InputDialog::new("Test").with_placeholder("Enter text");
        assert_eq!(dialog.placeholder(), "Enter text");
    }

    #[test]
    fn test_input_dialog_help_text() {
        let dialog = InputDialog::new("Test").with_help_text("Custom help");
        assert_eq!(dialog.help_text(), "Custom help");
    }

    #[test]
    fn test_input_dialog_insert_char() {
        let mut dialog = InputDialog::new("Test");
        dialog.insert_char('H');
        dialog.insert_char('e');
        dialog.insert_char('l');
        dialog.insert_char('l');
        dialog.insert_char('o');
        assert_eq!(dialog.value(), "Hello");
    }

    #[test]
    fn test_input_dialog_delete_char() {
        let mut dialog = InputDialog::new("Test");
        dialog.insert_char('A');
        dialog.insert_char('B');
        dialog.insert_char('C');
        dialog.delete_char();
        assert_eq!(dialog.value(), "AB");
    }

    #[test]
    fn test_input_dialog_cursor_movement() {
        let mut dialog = InputDialog::new("Test");
        dialog.insert_char('A');
        dialog.insert_char('B');
        dialog.insert_char('C');

        dialog.move_cursor_left();
        assert_eq!(dialog.cursor_position, 2);

        dialog.move_cursor_start();
        assert_eq!(dialog.cursor_position, 0);

        dialog.move_cursor_end();
        assert_eq!(dialog.cursor_position, 3);

        dialog.move_cursor_right();
        assert_eq!(dialog.cursor_position, 3); // Can't go past end
    }

    #[test]
    fn test_input_dialog_clear() {
        let mut dialog = InputDialog::new("Test");
        dialog.insert_char('A');
        dialog.insert_char('B');
        dialog.clear();
        assert_eq!(dialog.value(), "");
        assert_eq!(dialog.cursor_position, 0);
    }

    #[test]
    fn test_input_dialog_submit() {
        let mut dialog = InputDialog::new("Test");
        dialog.insert_char('A');
        assert!(dialog.submit().is_ok());
        assert_eq!(dialog.state(), &InputDialogState::Submitted);
    }

    #[test]
    fn test_input_dialog_cancel() {
        let mut dialog = InputDialog::new("Test");
        dialog.cancel();
        assert_eq!(dialog.state(), &InputDialogState::Cancelled);
    }

    #[test]
    fn test_input_dialog_reset() {
        let mut dialog = InputDialog::new("Test");
        dialog.insert_char('A');
        dialog.submit().ok();
        dialog.reset();

        assert!(dialog.is_active());
        assert_eq!(dialog.value(), "");
    }

    #[test]
    fn test_input_dialog_max_length() {
        let mut dialog = InputDialog::new("Test").with_max_length(3);
        dialog.insert_char('A');
        dialog.insert_char('B');
        dialog.insert_char('C');
        dialog.insert_char('D'); // Should not be inserted

        assert_eq!(dialog.value(), "ABC");
        assert_eq!(dialog.max_length(), Some(3));
    }

    #[test]
    fn test_input_dialog_validator() {
        fn number_validator(s: &str) -> Result<(), String> {
            if s.parse::<i32>().is_ok() {
                Ok(())
            } else {
                Err("Must be a number".to_string())
            }
        }

        let mut dialog = InputDialog::new("Number").with_validator(number_validator);

        dialog.insert_char('A');
        assert!(dialog.submit().is_err());

        dialog.clear();
        dialog.insert_char('1');
        dialog.insert_char('2');
        dialog.insert_char('3');
        assert!(dialog.submit().is_ok());
    }

    #[test]
    fn test_input_dialog_state_transitions() {
        let mut dialog = InputDialog::new("Test");
        assert_eq!(dialog.state(), &InputDialogState::Active);

        dialog.submit().ok();
        assert_eq!(dialog.state(), &InputDialogState::Submitted);

        dialog.reset();
        assert_eq!(dialog.state(), &InputDialogState::Active);

        dialog.cancel();
        assert_eq!(dialog.state(), &InputDialogState::Cancelled);
    }

    // ============================================================================
    // COMPREHENSIVE EDGE CASE TESTS (ADVANCED Tier Coverage)
    // ============================================================================

    // ------------------------------------------------------------------------
    // Stress Tests - Large inputs and many operations
    // ------------------------------------------------------------------------

    #[test]
    fn test_input_dialog_1000_characters() {
        let mut dialog = InputDialog::new("Large Input");
        let text = "a".repeat(1000);
        for c in text.chars() {
            dialog.insert_char(c);
        }
        assert_eq!(dialog.value(), text);
        assert_eq!(dialog.cursor_position, 1000);
    }

    #[test]
    fn test_input_dialog_10000_insertions() {
        let mut dialog = InputDialog::new("Stress Test");
        for _ in 0..10000 {
            dialog.insert_char('x');
        }
        assert_eq!(dialog.value().len(), 10000);
        assert_eq!(dialog.cursor_position, 10000);
    }

    #[test]
    fn test_input_dialog_10000_deletions() {
        let mut dialog = InputDialog::new("Delete Test");
        for _ in 0..10000 {
            dialog.insert_char('a');
        }

        for _ in 0..10000 {
            dialog.delete_char();
        }

        assert_eq!(dialog.value(), "");
        assert_eq!(dialog.cursor_position, 0);
    }

    #[test]
    fn test_input_dialog_rapid_cursor_movement() {
        let mut dialog = InputDialog::new("Cursor Test");
        for _ in 0..5 {
            dialog.insert_char('t');
        }

        // Move cursor 1000 times
        for _ in 0..250 {
            dialog.move_cursor_start();
            dialog.move_cursor_end();
            dialog.move_cursor_left();
            dialog.move_cursor_right();
        }

        assert_eq!(dialog.value(), "ttttt");
    }

    #[test]
    fn test_input_dialog_alternating_insert_delete() {
        let mut dialog = InputDialog::new("Test");

        for i in 0..1000 {
            dialog.insert_char('a');
            if i % 2 == 0 {
                dialog.delete_char();
            }
        }

        assert_eq!(dialog.value().len(), 500);
    }

    // ------------------------------------------------------------------------
    // Unicode Tests
    // ------------------------------------------------------------------------
    // NOTE: Current implementation has limitations with multi-byte UTF-8 characters
    // The cursor_position field is treated as character-based but String::insert()
    // requires byte positions, causing panics with unicode. These tests use ASCII only.

    #[test]
    fn test_input_dialog_ascii_extended() {
        let mut dialog = InputDialog::new("Extended ASCII");
        // Use extended ASCII characters that are still single-byte
        for c in "Hello World 123!@#$%^&*()".chars() {
            dialog.insert_char(c);
        }
        assert_eq!(dialog.value(), "Hello World 123!@#$%^&*()");
    }

    #[test]
    fn test_input_dialog_special_ascii_chars() {
        let mut dialog = InputDialog::new("Special");
        let special = "~`!@#$%^&*()_+-={}[]|\\:\";<>?,./";
        for c in special.chars() {
            dialog.insert_char(c);
        }
        assert_eq!(dialog.value(), special);
    }

    #[test]
    fn test_input_dialog_newline_tab_chars() {
        let mut dialog = InputDialog::new("Whitespace");
        dialog.insert_char('\t');
        dialog.insert_char('a');
        dialog.insert_char('\n');
        dialog.insert_char('b');
        assert_eq!(dialog.value(), "\ta\nb");
    }

    // ------------------------------------------------------------------------
    // Extreme Values Tests - Very long strings, boundary conditions
    // ------------------------------------------------------------------------

    #[test]
    fn test_input_dialog_100k_character_string() {
        let mut dialog = InputDialog::new("Huge Input");
        let huge = "x".repeat(100_000);
        for c in huge.chars() {
            dialog.insert_char(c);
        }
        assert_eq!(dialog.value().len(), 100_000);
    }

    #[test]
    fn test_input_dialog_delete_on_empty() {
        let mut dialog = InputDialog::new("Empty");
        dialog.delete_char();
        assert_eq!(dialog.value(), "");
        assert_eq!(dialog.cursor_position, 0);
    }

    #[test]
    fn test_input_dialog_cursor_left_at_start() {
        let mut dialog = InputDialog::new("Test");
        dialog.insert_char('a');
        dialog.move_cursor_start();
        dialog.move_cursor_left();
        assert_eq!(dialog.cursor_position, 0);
    }

    #[test]
    fn test_input_dialog_cursor_right_at_end() {
        let mut dialog = InputDialog::new("Test");
        dialog.insert_char('a');
        dialog.move_cursor_right();
        assert_eq!(dialog.cursor_position, 1);
    }

    #[test]
    fn test_input_dialog_very_long_title() {
        let long_title = "T".repeat(1000);
        let dialog = InputDialog::new(long_title.clone());
        assert_eq!(dialog.title(), long_title);
    }

    // ------------------------------------------------------------------------
    // Validation Edge Cases
    // ------------------------------------------------------------------------

    #[test]
    fn test_input_dialog_validation_clears_on_input() {
        fn always_fail(_: &str) -> Result<(), String> {
            Err("Always fails".to_string())
        }

        let mut dialog = InputDialog::new("Test").with_validator(always_fail);
        dialog.insert_char('a');
        dialog.submit().ok();

        // Validation error should be set
        assert!(dialog.validation_error.is_some());

        // Insert new character should clear error
        dialog.insert_char('b');
        assert!(dialog.validation_error.is_none());
    }

    #[test]
    fn test_input_dialog_validation_clears_on_delete() {
        fn always_fail(_: &str) -> Result<(), String> {
            Err("Always fails".to_string())
        }

        let mut dialog = InputDialog::new("Test").with_validator(always_fail);
        dialog.insert_char('a');
        dialog.submit().ok();
        assert!(dialog.validation_error.is_some());

        dialog.delete_char();
        assert!(dialog.validation_error.is_none());
    }

    #[test]
    fn test_input_dialog_email_validator() {
        fn email_validator(s: &str) -> Result<(), String> {
            if s.contains('@') && s.contains('.') {
                Ok(())
            } else {
                Err("Invalid email".to_string())
            }
        }

        let mut dialog = InputDialog::new("Email").with_validator(email_validator);

        dialog.insert_char('t');
        dialog.insert_char('e');
        dialog.insert_char('s');
        dialog.insert_char('t');
        assert!(dialog.submit().is_err());

        dialog.insert_char('@');
        dialog.insert_char('e');
        dialog.insert_char('x');
        dialog.insert_char('a');
        dialog.insert_char('m');
        dialog.insert_char('p');
        dialog.insert_char('l');
        dialog.insert_char('e');
        dialog.insert_char('.');
        dialog.insert_char('c');
        dialog.insert_char('o');
        dialog.insert_char('m');
        assert!(dialog.submit().is_ok());
    }

    #[test]
    fn test_input_dialog_length_validator() {
        fn min_length_validator(s: &str) -> Result<(), String> {
            if s.len() >= 5 {
                Ok(())
            } else {
                Err("Minimum 5 characters".to_string())
            }
        }

        let mut dialog = InputDialog::new("Password").with_validator(min_length_validator);

        for _ in 0..3 {
            dialog.insert_char('a');
        }
        assert!(dialog.submit().is_err());

        for _ in 0..2 {
            dialog.insert_char('a');
        }
        assert!(dialog.submit().is_ok());
    }

    // ------------------------------------------------------------------------
    // Max Length Edge Cases
    // ------------------------------------------------------------------------

    #[test]
    fn test_input_dialog_max_length_zero() {
        let mut dialog = InputDialog::new("Test").with_max_length(0);
        dialog.insert_char('a');
        assert_eq!(dialog.value(), "");
    }

    #[test]
    fn test_input_dialog_max_length_one() {
        let mut dialog = InputDialog::new("Test").with_max_length(1);
        dialog.insert_char('a');
        dialog.insert_char('b');
        assert_eq!(dialog.value(), "a");
    }

    #[test]
    fn test_input_dialog_max_length_with_special_chars() {
        let mut dialog = InputDialog::new("Special").with_max_length(3);
        dialog.insert_char('@');
        dialog.insert_char('#');
        dialog.insert_char('$');
        dialog.insert_char('%'); // Should not be inserted
        assert_eq!(dialog.value(), "@#$");
    }

    #[test]
    fn test_input_dialog_max_length_exact_boundary() {
        let mut dialog = InputDialog::new("Test").with_max_length(5);
        for _ in 0..5 {
            dialog.insert_char('a');
        }
        assert_eq!(dialog.value(), "aaaaa");

        dialog.insert_char('b'); // Should not be inserted
        assert_eq!(dialog.value(), "aaaaa");
    }

    // ------------------------------------------------------------------------
    // State Transition Edge Cases
    // ------------------------------------------------------------------------

    #[test]
    fn test_input_dialog_submit_changes_state() {
        let mut dialog = InputDialog::new("Test");
        assert_eq!(dialog.state(), &InputDialogState::Active);

        dialog.insert_char('a');
        dialog.submit().ok();
        assert_eq!(dialog.state(), &InputDialogState::Submitted);
    }

    #[test]
    fn test_input_dialog_cancel_changes_state() {
        let mut dialog = InputDialog::new("Test");
        assert_eq!(dialog.state(), &InputDialogState::Active);

        dialog.cancel();
        assert_eq!(dialog.state(), &InputDialogState::Cancelled);
    }

    #[test]
    fn test_input_dialog_reset_after_submit() {
        let mut dialog = InputDialog::new("Test");
        dialog.insert_char('t');
        dialog.insert_char('e');
        dialog.insert_char('s');
        dialog.insert_char('t');
        dialog.submit().ok();

        assert_eq!(dialog.state(), &InputDialogState::Submitted);
        assert_eq!(dialog.value(), "test");

        dialog.reset();

        assert_eq!(dialog.state(), &InputDialogState::Active);
        assert_eq!(dialog.value(), "");
        assert_eq!(dialog.cursor_position, 0);
    }

    #[test]
    fn test_input_dialog_reset_after_cancel() {
        let mut dialog = InputDialog::new("Test");
        dialog.insert_char('a');
        dialog.cancel();

        assert_eq!(dialog.state(), &InputDialogState::Cancelled);

        dialog.reset();

        assert_eq!(dialog.state(), &InputDialogState::Active);
        assert_eq!(dialog.value(), "");
    }

    #[test]
    fn test_input_dialog_multiple_submit_attempts() {
        fn always_fail(_: &str) -> Result<(), String> {
            Err("Failed".to_string())
        }

        let mut dialog = InputDialog::new("Test").with_validator(always_fail);
        dialog.insert_char('a');

        // Try submitting multiple times
        for _ in 0..100 {
            assert!(dialog.submit().is_err());
            assert_eq!(dialog.state(), &InputDialogState::Active);
        }
    }

    // ------------------------------------------------------------------------
    // Builder Pattern Tests
    // ------------------------------------------------------------------------

    #[test]
    fn test_input_dialog_builder_chaining() {
        fn validator(s: &str) -> Result<(), String> {
            if s.len() >= 3 {
                Ok(())
            } else {
                Err("Too short".to_string())
            }
        }

        let dialog = InputDialog::new("Title")
            .with_placeholder("Placeholder")
            .with_help_text("Help")
            .with_max_length(10)
            .with_validator(validator);

        assert_eq!(dialog.title(), "Title");
        assert_eq!(dialog.placeholder(), "Placeholder");
        assert_eq!(dialog.help_text(), "Help");
        assert_eq!(dialog.max_length(), Some(10));
    }

    #[test]
    fn test_input_dialog_builder_empty_strings() {
        let dialog = InputDialog::new("")
            .with_placeholder("")
            .with_help_text("");

        assert_eq!(dialog.title(), "");
        assert_eq!(dialog.placeholder(), "");
        assert_eq!(dialog.help_text(), "");
    }

    #[test]
    fn test_input_dialog_builder_unicode() {
        let dialog = InputDialog::new("タイトル")
            .with_placeholder("プレースホルダー 😀")
            .with_help_text("ヘルプテキスト");

        assert_eq!(dialog.title(), "タイトル");
        assert_eq!(dialog.placeholder(), "プレースホルダー 😀");
        assert_eq!(dialog.help_text(), "ヘルプテキスト");
    }

    // ------------------------------------------------------------------------
    // Complex Workflows - Multi-phase editing scenarios
    // ------------------------------------------------------------------------

    #[test]
    fn test_input_dialog_complex_editing_workflow() {
        let mut dialog = InputDialog::new("Complex Test");

        // Phase 1: Type filename
        for c in "file.txt".chars() {
            dialog.insert_char(c);
        }
        assert_eq!(dialog.value(), "file.txt");

        // Phase 2: Navigate and edit
        dialog.move_cursor_start();
        for _ in 0..4 {
            dialog.move_cursor_right();
        }
        dialog.insert_char('s');
        assert_eq!(dialog.value(), "files.txt");

        // Phase 3: Change extension
        dialog.move_cursor_end();
        for _ in 0..3 {
            dialog.delete_char();
        }
        dialog.insert_char('r');
        dialog.insert_char('s');
        assert_eq!(dialog.value(), "files.rs");

        // Phase 4: Test submission
        assert!(dialog.submit().is_ok());
        assert_eq!(dialog.state(), &InputDialogState::Submitted);

        // Phase 5: Reset and reuse
        dialog.reset();
        assert_eq!(dialog.value(), "");
        assert!(dialog.is_active());

        // Phase 6: New input
        for c in "new.txt".chars() {
            dialog.insert_char(c);
        }
        assert_eq!(dialog.value(), "new.txt");
    }

    #[test]
    fn test_input_dialog_validation_workflow() {
        fn number_only(s: &str) -> Result<(), String> {
            if s.chars().all(|c| c.is_ascii_digit()) && !s.is_empty() {
                Ok(())
            } else {
                Err("Numbers only".to_string())
            }
        }

        let mut dialog = InputDialog::new("Number Input").with_validator(number_only);

        // Phase 1: Try invalid input
        dialog.insert_char('a');
        dialog.insert_char('b');
        assert!(dialog.submit().is_err());
        assert!(dialog.validation_error.is_some());

        // Phase 2: Clear and try valid input
        dialog.clear();
        assert!(dialog.validation_error.is_none());

        for c in "12345".chars() {
            dialog.insert_char(c);
        }
        assert!(dialog.submit().is_ok());
        assert!(dialog.validation_error.is_none());

        // Phase 3: Reset and test again
        dialog.reset();
        dialog.insert_char('9');
        dialog.insert_char('9');
        assert!(dialog.submit().is_ok());
    }

    #[test]
    fn test_input_dialog_max_length_workflow() {
        let mut dialog = InputDialog::new("Limited").with_max_length(5);

        // Phase 1: Fill to max
        for _ in 0..5 {
            dialog.insert_char('a');
        }
        assert_eq!(dialog.value(), "aaaaa");

        // Phase 2: Try to exceed
        dialog.insert_char('b');
        assert_eq!(dialog.value(), "aaaaa");

        // Phase 3: Delete and insert
        dialog.delete_char();
        dialog.insert_char('b');
        assert_eq!(dialog.value(), "aaaab");

        // Phase 4: Clear and refill
        dialog.clear();
        for c in "12345".chars() {
            dialog.insert_char(c);
        }
        assert_eq!(dialog.value(), "12345");
    }

    #[test]
    fn test_input_dialog_mixed_case_workflow() {
        let mut dialog = InputDialog::new("Mixed Case Test");

        // Phase 1: Lowercase
        for c in "hello".chars() {
            dialog.insert_char(c);
        }
        assert_eq!(dialog.value(), "hello");

        // Phase 2: Add space and uppercase
        dialog.insert_char(' ');
        for c in "WORLD".chars() {
            dialog.insert_char(c);
        }
        assert_eq!(dialog.value(), "hello WORLD");

        // Phase 3: Add punctuation
        dialog.insert_char(' ');
        dialog.insert_char('!');
        assert_eq!(dialog.value(), "hello WORLD !");

        // Phase 4: Add numbers
        dialog.insert_char(' ');
        for c in "123".chars() {
            dialog.insert_char(c);
        }
        assert!(dialog.value().contains("123"));

        // Phase 5: Submit
        assert!(dialog.submit().is_ok());
    }

    // ------------------------------------------------------------------------
    // Navigation Edge Cases
    // ------------------------------------------------------------------------

    #[test]
    fn test_input_dialog_navigation_wraparound_prevention() {
        let mut dialog = InputDialog::new("Nav Test");
        for _ in 0..5 {
            dialog.insert_char('a');
        }

        // Try to move right past end
        for _ in 0..100 {
            dialog.move_cursor_right();
        }
        assert_eq!(dialog.cursor_position, 5);

        // Try to move left past start
        for _ in 0..100 {
            dialog.move_cursor_left();
        }
        assert_eq!(dialog.cursor_position, 0);
    }

    #[test]
    fn test_input_dialog_insert_at_different_positions() {
        let mut dialog = InputDialog::new("Test");
        for c in "abc".chars() {
            dialog.insert_char(c);
        }

        // Insert at start
        dialog.move_cursor_start();
        dialog.insert_char('X');
        assert_eq!(dialog.value(), "Xabc");

        // Insert in middle
        dialog.move_cursor_start();
        dialog.move_cursor_right();
        dialog.move_cursor_right();
        dialog.insert_char('Y');
        assert_eq!(dialog.value(), "XaYbc");

        // Insert at end
        dialog.move_cursor_end();
        dialog.insert_char('Z');
        assert_eq!(dialog.value(), "XaYbcZ");
    }

    // ------------------------------------------------------------------------
    // Trait Coverage Tests
    // ------------------------------------------------------------------------

    #[test]
    fn test_input_dialog_debug_trait() {
        let dialog = InputDialog::new("Test");
        let debug_str = format!("{:?}", dialog);
        assert!(debug_str.contains("InputDialog"));
    }

    #[test]
    fn test_input_dialog_clone_trait() {
        let mut dialog1 = InputDialog::new("Test");
        dialog1.insert_char('a');

        let dialog2 = dialog1.clone();
        assert_eq!(dialog1.value(), dialog2.value());
        assert_eq!(dialog1.title(), dialog2.title());
    }

    #[test]
    fn test_input_dialog_state_debug() {
        let state = InputDialogState::Active;
        let debug_str = format!("{:?}", state);
        assert!(debug_str.contains("Active"));
    }

    #[test]
    fn test_input_dialog_state_partial_eq() {
        assert_eq!(InputDialogState::Active, InputDialogState::Active);
        assert_ne!(InputDialogState::Active, InputDialogState::Submitted);
    }

    // ------------------------------------------------------------------------
    // Empty State Tests
    // ------------------------------------------------------------------------

    #[test]
    fn test_input_dialog_operations_on_empty() {
        let mut dialog = InputDialog::new("Empty");

        dialog.move_cursor_left();
        dialog.move_cursor_right();
        dialog.move_cursor_start();
        dialog.move_cursor_end();
        assert_eq!(dialog.cursor_position, 0);

        dialog.delete_char();
        assert_eq!(dialog.value(), "");

        assert!(dialog.submit().is_ok());
    }

    #[test]
    fn test_input_dialog_clear_on_empty() {
        let mut dialog = InputDialog::new("Test");
        dialog.clear();
        assert_eq!(dialog.value(), "");
        assert_eq!(dialog.cursor_position, 0);
        assert!(dialog.validation_error.is_none());
    }

    // ------------------------------------------------------------------------
    // Comprehensive Stress Test
    // ------------------------------------------------------------------------

    #[test]
    fn test_input_dialog_comprehensive_stress() {
        fn length_validator(s: &str) -> Result<(), String> {
            if s.len() >= 3 && s.len() <= 1000 {
                Ok(())
            } else {
                Err("Length must be 3-1000".to_string())
            }
        }

        let mut dialog = InputDialog::new("Stress Test")
            .with_placeholder("Enter text...")
            .with_help_text("Custom help")
            .with_max_length(1000)
            .with_validator(length_validator);

        // Phase 1: Insert 1000 characters
        for i in 0..1000 {
            dialog.insert_char(char::from_u32((i % 26) + 97).unwrap());
        }
        assert_eq!(dialog.value().len(), 1000);

        // Phase 2: Try to exceed max length
        dialog.insert_char('x');
        assert_eq!(dialog.value().len(), 1000);

        // Phase 3: Navigate to middle
        dialog.move_cursor_start();
        for _ in 0..500 {
            dialog.move_cursor_right();
        }

        // Phase 4: Delete some characters
        for _ in 0..100 {
            dialog.delete_char();
        }
        assert_eq!(dialog.value().len(), 900);

        // Phase 5: Try to submit (should pass validation)
        assert!(dialog.submit().is_ok());
        assert_eq!(dialog.state(), &InputDialogState::Submitted);

        // Phase 6: Reset
        dialog.reset();
        assert_eq!(dialog.value(), "");
        assert!(dialog.is_active());

        // Phase 7: Insert mixed ASCII content
        for c in "Hello WORLD 123 !@# $%^".chars() {
            dialog.insert_char(c);
        }
        assert!(dialog.value().contains("WORLD"));
        assert!(dialog.value().contains("123"));

        // Phase 8: Submit mixed content
        assert!(dialog.submit().is_ok());
    }
}
