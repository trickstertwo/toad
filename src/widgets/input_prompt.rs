//! Input prompt modal
//!
//! Modal dialog with text input field for collecting user input.
//!
//! # Examples
//!
//! ```
//! use toad::widgets::InputPrompt;
//!
//! let mut prompt = InputPrompt::new("Enter Name", "Please enter your name:");
//! prompt.insert_char('A');
//! prompt.insert_char('l');
//! prompt.insert_char('i');
//! prompt.insert_char('c');
//! prompt.insert_char('e');
//!
//! assert_eq!(prompt.value(), "Alice");
//! ```

use crate::theme::ToadTheme;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

/// Input prompt modal for collecting user text input.
///
/// This widget combines a modal dialog with a text input field,
/// providing a focused way to collect single-line text input from the user.
///
/// # Examples
///
/// ```
/// use toad::widgets::InputPrompt;
///
/// // Create a new input prompt
/// let prompt = InputPrompt::new("Title", "Enter your email:");
///
/// assert_eq!(prompt.value(), "");
/// ```
///
/// # Cursor navigation
///
/// ```
/// use toad::widgets::InputPrompt;
///
/// let mut prompt = InputPrompt::new("Title", "Message");
/// prompt.set_value("Hello".to_string());
///
/// // Cursor starts at end
/// assert_eq!(prompt.cursor_position(), 5);
///
/// // Move cursor
/// prompt.move_cursor_start();
/// assert_eq!(prompt.cursor_position(), 0);
/// ```
pub struct InputPrompt {
    title: String,
    message: String,
    input_value: String,
    cursor_position: usize,
    placeholder: String,
    is_focused: bool,
}

impl InputPrompt {
    /// Create a new input prompt.
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::InputPrompt;
    ///
    /// let prompt = InputPrompt::new("Enter Name", "What is your name?");
    /// assert_eq!(prompt.value(), "");
    /// ```
    pub fn new(title: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            message: message.into(),
            input_value: String::new(),
            cursor_position: 0,
            placeholder: "Enter text...".to_string(),
            is_focused: true,
        }
    }

    /// Set placeholder text shown when input is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::InputPrompt;
    ///
    /// let prompt = InputPrompt::new("Title", "Message")
    ///     .with_placeholder("example@email.com");
    /// ```
    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Get the current input value.
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::InputPrompt;
    ///
    /// let mut prompt = InputPrompt::new("Title", "Message");
    /// prompt.set_value("test".to_string());
    /// assert_eq!(prompt.value(), "test");
    /// ```
    pub fn value(&self) -> &str {
        &self.input_value
    }

    /// Set the input value and move cursor to end.
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::InputPrompt;
    ///
    /// let mut prompt = InputPrompt::new("Title", "Message");
    /// prompt.set_value("Initial value".to_string());
    /// assert_eq!(prompt.value(), "Initial value");
    /// assert_eq!(prompt.cursor_position(), 13);
    /// ```
    pub fn set_value(&mut self, value: String) {
        self.input_value = value;
        self.cursor_position = self.input_value.len();
    }

    /// Get the cursor position (byte index).
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::InputPrompt;
    ///
    /// let mut prompt = InputPrompt::new("Title", "Message");
    /// prompt.insert_char('A');
    /// assert_eq!(prompt.cursor_position(), 1);
    /// ```
    pub fn cursor_position(&self) -> usize {
        self.cursor_position
    }

    /// Set focused state.
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::InputPrompt;
    ///
    /// let mut prompt = InputPrompt::new("Title", "Message");
    /// prompt.set_focused(false);
    /// assert!(!prompt.is_focused());
    /// ```
    pub fn set_focused(&mut self, focused: bool) {
        self.is_focused = focused;
    }

    /// Check if the input prompt is focused.
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::InputPrompt;
    ///
    /// let prompt = InputPrompt::new("Title", "Message");
    /// assert!(prompt.is_focused());
    /// ```
    pub fn is_focused(&self) -> bool {
        self.is_focused
    }

    /// Insert a character at the cursor position.
    ///
    /// The cursor advances by the character's UTF-8 byte length.
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::InputPrompt;
    ///
    /// let mut prompt = InputPrompt::new("Title", "Message");
    /// prompt.insert_char('H');
    /// prompt.insert_char('i');
    /// assert_eq!(prompt.value(), "Hi");
    /// ```
    pub fn insert_char(&mut self, c: char) {
        self.input_value.insert(self.cursor_position, c);
        self.cursor_position += c.len_utf8();
    }

    /// Delete the character before the cursor (backspace).
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::InputPrompt;
    ///
    /// let mut prompt = InputPrompt::new("Title", "Message");
    /// prompt.set_value("Hello".to_string());
    /// prompt.delete_char();
    /// assert_eq!(prompt.value(), "Hell");
    /// ```
    pub fn delete_char(&mut self) {
        if self.cursor_position > 0 {
            let chars: Vec<char> = self.input_value.chars().collect();
            let char_pos = self.char_position();
            if char_pos > 0 {
                let mut new_chars = chars;
                new_chars.remove(char_pos - 1);
                self.input_value = new_chars.into_iter().collect();
                self.cursor_position = self.char_to_byte_idx(char_pos - 1);
            }
        }
    }

    /// Move cursor left by one character.
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::InputPrompt;
    ///
    /// let mut prompt = InputPrompt::new("Title", "Message");
    /// prompt.set_value("Test".to_string());
    /// prompt.move_cursor_left();
    /// assert_eq!(prompt.cursor_position(), 3);
    /// ```
    pub fn move_cursor_left(&mut self) {
        if self.cursor_position > 0 {
            let char_pos = self.char_position();
            if char_pos > 0 {
                self.cursor_position = self.char_to_byte_idx(char_pos - 1);
            }
        }
    }

    /// Move cursor right by one character.
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::InputPrompt;
    ///
    /// let mut prompt = InputPrompt::new("Title", "Message");
    /// prompt.set_value("Test".to_string());
    /// prompt.move_cursor_start();
    /// prompt.move_cursor_right();
    /// assert_eq!(prompt.cursor_position(), 1);
    /// ```
    pub fn move_cursor_right(&mut self) {
        let char_pos = self.char_position();
        let char_count = self.input_value.chars().count();
        if char_pos < char_count {
            self.cursor_position = self.char_to_byte_idx(char_pos + 1);
        }
    }

    /// Move cursor to start of input.
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::InputPrompt;
    ///
    /// let mut prompt = InputPrompt::new("Title", "Message");
    /// prompt.set_value("Test".to_string());
    /// prompt.move_cursor_start();
    /// assert_eq!(prompt.cursor_position(), 0);
    /// ```
    pub fn move_cursor_start(&mut self) {
        self.cursor_position = 0;
    }

    /// Move cursor to end of input.
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::InputPrompt;
    ///
    /// let mut prompt = InputPrompt::new("Title", "Message");
    /// prompt.set_value("Test".to_string());
    /// prompt.move_cursor_start();
    /// prompt.move_cursor_end();
    /// assert_eq!(prompt.cursor_position(), 4);
    /// ```
    pub fn move_cursor_end(&mut self) {
        self.cursor_position = self.input_value.len();
    }

    /// Clear the input and reset cursor to start.
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::InputPrompt;
    ///
    /// let mut prompt = InputPrompt::new("Title", "Message");
    /// prompt.set_value("Test".to_string());
    /// prompt.clear();
    /// assert_eq!(prompt.value(), "");
    /// assert_eq!(prompt.cursor_position(), 0);
    /// ```
    pub fn clear(&mut self) {
        self.input_value.clear();
        self.cursor_position = 0;
    }

    /// Get character position from byte position
    fn char_position(&self) -> usize {
        self.input_value[..self.cursor_position].chars().count()
    }

    /// Convert character index to byte index
    fn char_to_byte_idx(&self, char_idx: usize) -> usize {
        self.input_value
            .char_indices()
            .nth(char_idx)
            .map(|(idx, _)| idx)
            .unwrap_or(self.input_value.len())
    }

    /// Render the input prompt modal
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        // Create centered modal
        let vertical = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(25),
                Constraint::Percentage(50),
                Constraint::Percentage(25),
            ])
            .split(area);

        let horizontal = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(15),
                Constraint::Percentage(70),
                Constraint::Percentage(15),
            ])
            .split(vertical[1]);

        let modal_area = horizontal[1];

        // Render modal block
        let block = Block::default()
            .title(format!(" {} ", self.title))
            .title_style(
                Style::default()
                    .fg(ToadTheme::TOAD_GREEN)
                    .add_modifier(Modifier::BOLD),
            )
            .borders(Borders::ALL)
            .border_style(Style::default().fg(ToadTheme::TOAD_GREEN))
            .style(Style::default().bg(ToadTheme::BLACK));

        let inner = block.inner(modal_area);
        frame.render_widget(block, modal_area);

        // Split inner area
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Message
                Constraint::Length(1), // Spacing
                Constraint::Length(3), // Input field
                Constraint::Length(1), // Spacing
                Constraint::Length(1), // Help text
            ])
            .split(inner);

        // Render message
        let message_lines = vec![
            Line::from(""),
            Line::from(Span::styled(
                &self.message,
                Style::default()
                    .fg(ToadTheme::FOREGROUND)
                    .add_modifier(Modifier::BOLD),
            )),
        ];

        let message_paragraph = Paragraph::new(message_lines)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        frame.render_widget(message_paragraph, chunks[0]);

        // Render input field
        let input_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(if self.is_focused {
                ToadTheme::TOAD_GREEN
            } else {
                ToadTheme::DARK_GRAY
            }));

        let input_inner = input_block.inner(chunks[2]);
        frame.render_widget(input_block, chunks[2]);

        // Render input text with cursor
        let input_line = if self.input_value.is_empty() {
            Line::from(Span::styled(
                &self.placeholder,
                Style::default()
                    .fg(ToadTheme::DARK_GRAY)
                    .add_modifier(Modifier::ITALIC),
            ))
        } else {
            let before_cursor = &self.input_value[..self.cursor_position];
            let after_cursor = &self.input_value[self.cursor_position..];

            let cursor_char = if after_cursor.is_empty() {
                " "
            } else {
                &after_cursor[..after_cursor.chars().next().unwrap().len_utf8()]
            };

            let rest = if after_cursor.is_empty() {
                ""
            } else {
                &after_cursor[cursor_char.len()..]
            };

            let mut spans = vec![Span::styled(
                before_cursor,
                Style::default().fg(ToadTheme::FOREGROUND),
            )];

            if self.is_focused {
                spans.push(Span::styled(
                    cursor_char,
                    Style::default()
                        .fg(ToadTheme::BLACK)
                        .bg(ToadTheme::TOAD_GREEN),
                ));
            } else {
                spans.push(Span::styled(
                    cursor_char,
                    Style::default().fg(ToadTheme::FOREGROUND),
                ));
            }

            spans.push(Span::styled(rest, Style::default().fg(ToadTheme::FOREGROUND)));

            Line::from(spans)
        };

        let input_paragraph = Paragraph::new(input_line);
        frame.render_widget(input_paragraph, input_inner);

        // Render help text
        let help_text = Line::from(vec![
            Span::styled("Enter", Style::default().fg(ToadTheme::TOAD_GREEN)),
            Span::styled(" to confirm, ", Style::default().fg(ToadTheme::GRAY)),
            Span::styled("Esc", Style::default().fg(ToadTheme::TOAD_GREEN)),
            Span::styled(" to cancel", Style::default().fg(ToadTheme::GRAY)),
        ]);

        let help_paragraph = Paragraph::new(help_text).alignment(Alignment::Center);
        frame.render_widget(help_paragraph, chunks[4]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_prompt_creation() {
        let prompt = InputPrompt::new("Test Title", "Enter something");
        assert_eq!(prompt.value(), "");
        assert_eq!(prompt.is_focused, true);
    }

    #[test]
    fn test_input_prompt_placeholder() {
        let prompt = InputPrompt::new("Title", "Message").with_placeholder("Custom placeholder");
        assert_eq!(prompt.placeholder, "Custom placeholder");
    }

    #[test]
    fn test_input_prompt_insert_char() {
        let mut prompt = InputPrompt::new("Title", "Message");
        prompt.insert_char('H');
        prompt.insert_char('i');
        assert_eq!(prompt.value(), "Hi");
    }

    #[test]
    fn test_input_prompt_delete_char() {
        let mut prompt = InputPrompt::new("Title", "Message");
        prompt.set_value("Hello".to_string());
        prompt.delete_char();
        assert_eq!(prompt.value(), "Hell");
    }

    #[test]
    fn test_input_prompt_cursor_movement() {
        let mut prompt = InputPrompt::new("Title", "Message");
        prompt.set_value("Test".to_string());

        assert_eq!(prompt.cursor_position, 4);

        prompt.move_cursor_left();
        assert_eq!(prompt.cursor_position, 3);

        prompt.move_cursor_start();
        assert_eq!(prompt.cursor_position, 0);

        prompt.move_cursor_end();
        assert_eq!(prompt.cursor_position, 4);

        prompt.move_cursor_right();
        assert_eq!(prompt.cursor_position, 4); // Can't move past end
    }

    #[test]
    fn test_input_prompt_clear() {
        let mut prompt = InputPrompt::new("Title", "Message");
        prompt.set_value("Test".to_string());
        assert_eq!(prompt.value(), "Test");

        prompt.clear();
        assert_eq!(prompt.value(), "");
        assert_eq!(prompt.cursor_position, 0);
    }

    #[test]
    fn test_input_prompt_set_focused() {
        let mut prompt = InputPrompt::new("Title", "Message");
        assert_eq!(prompt.is_focused, true);

        prompt.set_focused(false);
        assert_eq!(prompt.is_focused, false);
    }
}
