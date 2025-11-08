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

use crate::theme::ToadTheme;
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
}
