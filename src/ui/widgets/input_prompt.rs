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

use crate::ui::theme::ToadTheme;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
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
#[derive(Debug)]
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

            spans.push(Span::styled(
                rest,
                Style::default().fg(ToadTheme::FOREGROUND),
            ));

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
        assert!(prompt.is_focused);
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
        assert!(prompt.is_focused);

        prompt.set_focused(false);
        assert!(!prompt.is_focused);
    }

    // ============================================================================
    // COMPREHENSIVE EDGE CASE TESTS (ADVANCED Tier Coverage)
    // ============================================================================

    // ------------------------------------------------------------------------
    // Stress Tests - Large inputs and many operations
    // ------------------------------------------------------------------------

    #[test]
    fn test_input_prompt_1000_characters() {
        let mut prompt = InputPrompt::new("Title", "Message");
        let text = "a".repeat(1000);
        prompt.set_value(text.clone());
        assert_eq!(prompt.value(), text);
        assert_eq!(prompt.cursor_position(), 1000);
    }

    #[test]
    fn test_input_prompt_10000_insertions() {
        let mut prompt = InputPrompt::new("Title", "Message");
        for _ in 0..10000 {
            prompt.insert_char('x');
        }
        assert_eq!(prompt.value().len(), 10000);
        assert_eq!(prompt.cursor_position(), 10000);
    }

    #[test]
    fn test_input_prompt_10000_deletions() {
        let mut prompt = InputPrompt::new("Title", "Message");
        let text = "x".repeat(10000);
        prompt.set_value(text);

        for _ in 0..10000 {
            prompt.delete_char();
        }

        assert_eq!(prompt.value(), "");
        assert_eq!(prompt.cursor_position(), 0);
    }

    #[test]
    fn test_input_prompt_rapid_cursor_movement() {
        let mut prompt = InputPrompt::new("Title", "Message");
        prompt.set_value("test".to_string());

        // Move cursor 1000 times in various directions
        for _ in 0..250 {
            prompt.move_cursor_start();
            prompt.move_cursor_end();
            prompt.move_cursor_left();
            prompt.move_cursor_right();
        }

        // Should still be valid
        assert_eq!(prompt.value(), "test");
    }

    #[test]
    fn test_input_prompt_alternating_insert_delete_1000_times() {
        let mut prompt = InputPrompt::new("Title", "Message");

        for i in 0..1000 {
            prompt.insert_char('a');
            if i % 2 == 0 {
                prompt.delete_char();
            }
        }

        // Should have 500 characters (1000 inserts - 500 deletes)
        assert_eq!(prompt.value().len(), 500);
    }

    // ------------------------------------------------------------------------
    // Unicode Tests - RTL, emoji, Japanese, mixed scripts, combining chars
    // ------------------------------------------------------------------------

    #[test]
    fn test_input_prompt_rtl_arabic() {
        let mut prompt = InputPrompt::new("Title", "Message");
        let arabic = "مرحبا بك";
        prompt.set_value(arabic.to_string());
        assert_eq!(prompt.value(), arabic);
        assert_eq!(prompt.cursor_position(), arabic.len());
    }

    #[test]
    fn test_input_prompt_rtl_hebrew() {
        let mut prompt = InputPrompt::new("Title", "Message");
        let hebrew = "שלום עולם";
        prompt.set_value(hebrew.to_string());
        assert_eq!(prompt.value(), hebrew);
        assert_eq!(prompt.cursor_position(), hebrew.len());
    }

    #[test]
    fn test_input_prompt_emoji() {
        let mut prompt = InputPrompt::new("Title", "Message");
        prompt.insert_char('😀');
        prompt.insert_char('🎉');
        prompt.insert_char('👍');
        assert_eq!(prompt.value(), "😀🎉👍");

        // Delete one emoji
        prompt.delete_char();
        assert_eq!(prompt.value(), "😀🎉");
    }

    #[test]
    fn test_input_prompt_japanese() {
        let mut prompt = InputPrompt::new("Title", "Message");
        let japanese = "こんにちは世界";
        prompt.set_value(japanese.to_string());
        assert_eq!(prompt.value(), japanese);

        // Move cursor and insert
        prompt.move_cursor_start();
        prompt.insert_char('新');
        assert!(prompt.value().starts_with('新'));
    }

    #[test]
    fn test_input_prompt_mixed_scripts() {
        let mut prompt = InputPrompt::new("Title", "Message");
        let mixed = "Hello مرحبا こんにちは 🎉";
        prompt.set_value(mixed.to_string());
        assert_eq!(prompt.value(), mixed);

        // Navigate through mixed scripts
        prompt.move_cursor_start();
        for _ in 0..5 {
            prompt.move_cursor_right();
        }
        prompt.insert_char('!');
        assert!(prompt.value().contains('!'));
    }

    #[test]
    fn test_input_prompt_combining_characters() {
        let mut prompt = InputPrompt::new("Title", "Message");
        // "é" as 'e' + combining acute accent
        let combining = "e\u{0301}";
        prompt.set_value(combining.to_string());
        assert_eq!(prompt.value(), combining);
    }

    #[test]
    fn test_input_prompt_zero_width_characters() {
        let mut prompt = InputPrompt::new("Title", "Message");
        // Zero-width joiner
        let zwj = "a\u{200D}b";
        prompt.set_value(zwj.to_string());
        assert_eq!(prompt.value(), zwj);
    }

    // ------------------------------------------------------------------------
    // Extreme Values Tests - Very long strings, boundary conditions
    // ------------------------------------------------------------------------

    #[test]
    fn test_input_prompt_100k_character_string() {
        let mut prompt = InputPrompt::new("Title", "Message");
        let huge_text = "x".repeat(100_000);
        prompt.set_value(huge_text.clone());
        assert_eq!(prompt.value().len(), 100_000);
        assert_eq!(prompt.cursor_position(), 100_000);
    }

    #[test]
    fn test_input_prompt_delete_on_empty() {
        let mut prompt = InputPrompt::new("Title", "Message");
        // Delete on empty should not panic
        prompt.delete_char();
        assert_eq!(prompt.value(), "");
        assert_eq!(prompt.cursor_position(), 0);
    }

    #[test]
    fn test_input_prompt_cursor_left_at_start() {
        let mut prompt = InputPrompt::new("Title", "Message");
        prompt.set_value("test".to_string());
        prompt.move_cursor_start();

        // Try to move left from position 0
        prompt.move_cursor_left();
        assert_eq!(prompt.cursor_position(), 0);
    }

    #[test]
    fn test_input_prompt_cursor_right_at_end() {
        let mut prompt = InputPrompt::new("Title", "Message");
        prompt.set_value("test".to_string());

        // Already at end, try to move right
        prompt.move_cursor_right();
        assert_eq!(prompt.cursor_position(), 4);
    }

    #[test]
    fn test_input_prompt_very_long_title_and_message() {
        let long_title = "T".repeat(1000);
        let long_message = "M".repeat(1000);
        let prompt = InputPrompt::new(long_title.clone(), long_message.clone());
        assert_eq!(prompt.title, long_title);
        assert_eq!(prompt.message, long_message);
    }

    // ------------------------------------------------------------------------
    // Cursor Navigation and Editing Edge Cases
    // ------------------------------------------------------------------------

    #[test]
    fn test_input_prompt_insert_at_middle() {
        let mut prompt = InputPrompt::new("Title", "Message");
        prompt.set_value("Hello".to_string());
        prompt.move_cursor_start();
        prompt.move_cursor_right();
        prompt.move_cursor_right();

        // Cursor at position 2 (between 'e' and 'l')
        prompt.insert_char('X');
        assert_eq!(prompt.value(), "HeXllo");
    }

    #[test]
    fn test_input_prompt_delete_at_middle() {
        let mut prompt = InputPrompt::new("Title", "Message");
        prompt.set_value("Hello".to_string());
        prompt.move_cursor_start();
        prompt.move_cursor_right();
        prompt.move_cursor_right();
        prompt.move_cursor_right();

        // Cursor at position 3, delete 'l'
        prompt.delete_char();
        assert_eq!(prompt.value(), "Helo");
    }

    #[test]
    fn test_input_prompt_multiple_cursor_movements() {
        let mut prompt = InputPrompt::new("Title", "Message");
        prompt.set_value("abcdefgh".to_string());

        // Complex navigation
        prompt.move_cursor_start();
        assert_eq!(prompt.cursor_position(), 0);

        for _ in 0..3 {
            prompt.move_cursor_right();
        }
        assert_eq!(prompt.cursor_position(), 3);

        for _ in 0..2 {
            prompt.move_cursor_left();
        }
        assert_eq!(prompt.cursor_position(), 1);

        prompt.move_cursor_end();
        assert_eq!(prompt.cursor_position(), 8);
    }

    #[test]
    fn test_input_prompt_insert_unicode_at_different_positions() {
        let mut prompt = InputPrompt::new("Title", "Message");
        prompt.set_value("Hello".to_string());

        // Insert emoji at start
        prompt.move_cursor_start();
        prompt.insert_char('😀');
        assert_eq!(prompt.value(), "😀Hello");

        // Insert emoji at end
        prompt.move_cursor_end();
        prompt.insert_char('🎉');
        assert_eq!(prompt.value(), "😀Hello🎉");
    }

    #[test]
    fn test_input_prompt_navigation_wraparound_prevention() {
        let mut prompt = InputPrompt::new("Title", "Message");
        prompt.set_value("test".to_string());

        // Try to move right past end many times
        for _ in 0..100 {
            prompt.move_cursor_right();
        }
        assert_eq!(prompt.cursor_position(), 4);

        // Try to move left past start many times
        for _ in 0..100 {
            prompt.move_cursor_left();
        }
        assert_eq!(prompt.cursor_position(), 0);
    }

    // ------------------------------------------------------------------------
    // Builder Pattern Tests
    // ------------------------------------------------------------------------

    #[test]
    fn test_input_prompt_builder_chaining() {
        let prompt = InputPrompt::new("Title", "Message")
            .with_placeholder("Enter email...");

        assert_eq!(prompt.placeholder, "Enter email...");
        assert_eq!(prompt.title, "Title");
        assert_eq!(prompt.message, "Message");
    }

    #[test]
    fn test_input_prompt_builder_empty_placeholder() {
        let prompt = InputPrompt::new("Title", "Message").with_placeholder("");
        assert_eq!(prompt.placeholder, "");
    }

    #[test]
    fn test_input_prompt_builder_unicode_placeholder() {
        let prompt = InputPrompt::new("Title", "Message")
            .with_placeholder("输入文本... 😀");
        assert_eq!(prompt.placeholder, "输入文本... 😀");
    }

    // ------------------------------------------------------------------------
    // Complex Workflows - Multi-phase editing scenarios
    // ------------------------------------------------------------------------

    #[test]
    fn test_input_prompt_complex_editing_workflow() {
        let mut prompt = InputPrompt::new("Edit File", "Enter filename:");

        // Phase 1: Initial typing
        prompt.insert_char('f');
        prompt.insert_char('i');
        prompt.insert_char('l');
        prompt.insert_char('e');
        assert_eq!(prompt.value(), "file");

        // Phase 2: Add extension
        prompt.insert_char('.');
        prompt.insert_char('t');
        prompt.insert_char('x');
        prompt.insert_char('t');
        assert_eq!(prompt.value(), "file.txt");

        // Phase 3: Realize mistake, go back and fix
        prompt.move_cursor_start();
        for _ in 0..5 {
            prompt.move_cursor_right();
        }
        prompt.delete_char(); // Delete '.' (backspace deletes char BEFORE cursor)
        assert_eq!(prompt.value(), "filetxt");

        // Phase 4: Re-insert period at correct position
        prompt.insert_char('.');
        assert_eq!(prompt.value(), "file.txt");

        // Phase 5: Change extension
        prompt.move_cursor_end();
        for _ in 0..3 {
            prompt.delete_char();
        }
        assert_eq!(prompt.value(), "file.");

        // Phase 6: Add new extension
        prompt.insert_char('r');
        prompt.insert_char('s');
        assert_eq!(prompt.value(), "file.rs");

        // Phase 7: Add path prefix
        prompt.move_cursor_start();
        prompt.insert_char('s');
        prompt.insert_char('r');
        prompt.insert_char('c');
        prompt.insert_char('/');
        assert_eq!(prompt.value(), "src/file.rs");

        // Phase 8: Test focus toggle
        prompt.set_focused(false);
        assert!(!prompt.is_focused());
        prompt.set_focused(true);
        assert!(prompt.is_focused());

        // Phase 9: Clear and start over
        prompt.clear();
        assert_eq!(prompt.value(), "");
        assert_eq!(prompt.cursor_position(), 0);

        // Phase 10: Type new value
        prompt.set_value("main.rs".to_string());
        assert_eq!(prompt.value(), "main.rs");
        assert_eq!(prompt.cursor_position(), 7);
    }

    #[test]
    fn test_input_prompt_unicode_editing_workflow() {
        let mut prompt = InputPrompt::new("Title", "Message");

        // Phase 1: Type English
        prompt.set_value("Hello".to_string());
        assert_eq!(prompt.value(), "Hello");

        // Phase 2: Add space and Japanese
        prompt.insert_char(' ');
        prompt.insert_char('世');
        prompt.insert_char('界');
        assert_eq!(prompt.value(), "Hello 世界");

        // Phase 3: Navigate and insert emoji
        prompt.move_cursor_start();
        for _ in 0..5 {
            prompt.move_cursor_right();
        }
        prompt.insert_char('😀');
        assert!(prompt.value().contains('😀'));

        // Phase 4: Add Arabic at end
        prompt.move_cursor_end();
        prompt.insert_char(' ');
        prompt.insert_char('م');
        prompt.insert_char('ر');
        prompt.insert_char('ح');
        prompt.insert_char('ب');
        prompt.insert_char('ا');
        assert!(prompt.value().contains("مرحبا"));

        // Phase 5: Clear and verify
        prompt.clear();
        assert_eq!(prompt.value(), "");
    }

    #[test]
    fn test_input_prompt_repeated_clear_and_fill() {
        let mut prompt = InputPrompt::new("Title", "Message");

        for i in 0..100 {
            let text = format!("iteration_{}", i);
            prompt.set_value(text.clone());
            assert_eq!(prompt.value(), text);

            prompt.clear();
            assert_eq!(prompt.value(), "");
        }
    }

    // ------------------------------------------------------------------------
    // Empty State Tests
    // ------------------------------------------------------------------------

    #[test]
    fn test_input_prompt_operations_on_empty() {
        let mut prompt = InputPrompt::new("Title", "Message");

        // All cursor movements should be safe on empty
        prompt.move_cursor_left();
        prompt.move_cursor_right();
        prompt.move_cursor_start();
        prompt.move_cursor_end();
        assert_eq!(prompt.cursor_position(), 0);

        // Delete on empty should be safe
        prompt.delete_char();
        assert_eq!(prompt.value(), "");
    }

    #[test]
    fn test_input_prompt_empty_title_and_message() {
        let prompt = InputPrompt::new("", "");
        assert_eq!(prompt.title, "");
        assert_eq!(prompt.message, "");
        assert_eq!(prompt.value(), "");
    }

    #[test]
    fn test_input_prompt_set_value_to_empty() {
        let mut prompt = InputPrompt::new("Title", "Message");
        prompt.set_value("test".to_string());
        assert_eq!(prompt.value(), "test");

        prompt.set_value("".to_string());
        assert_eq!(prompt.value(), "");
        assert_eq!(prompt.cursor_position(), 0);
    }

    // ------------------------------------------------------------------------
    // Trait Coverage Tests
    // ------------------------------------------------------------------------

    #[test]
    fn test_input_prompt_debug_trait() {
        let prompt = InputPrompt::new("Title", "Message");
        let debug_str = format!("{:?}", prompt);
        assert!(debug_str.contains("InputPrompt"));
    }

    // ------------------------------------------------------------------------
    // Focus State Tests
    // ------------------------------------------------------------------------

    #[test]
    fn test_input_prompt_default_focused() {
        let prompt = InputPrompt::new("Title", "Message");
        assert!(prompt.is_focused());
    }

    #[test]
    fn test_input_prompt_focus_toggle_multiple_times() {
        let mut prompt = InputPrompt::new("Title", "Message");

        for _ in 0..100 {
            prompt.set_focused(false);
            assert!(!prompt.is_focused());
            prompt.set_focused(true);
            assert!(prompt.is_focused());
        }
    }

    // ------------------------------------------------------------------------
    // UTF-8 Cursor Position Tests
    // ------------------------------------------------------------------------

    #[test]
    fn test_input_prompt_cursor_position_with_multibyte_chars() {
        let mut prompt = InputPrompt::new("Title", "Message");
        prompt.insert_char('a'); // 1 byte
        assert_eq!(prompt.cursor_position(), 1);

        prompt.insert_char('é'); // 2 bytes
        assert_eq!(prompt.cursor_position(), 3);

        prompt.insert_char('世'); // 3 bytes
        assert_eq!(prompt.cursor_position(), 6);

        prompt.insert_char('😀'); // 4 bytes
        assert_eq!(prompt.cursor_position(), 10);
    }

    #[test]
    fn test_input_prompt_navigation_through_multibyte_chars() {
        let mut prompt = InputPrompt::new("Title", "Message");
        prompt.set_value("a世😀b".to_string());

        prompt.move_cursor_start();
        assert_eq!(prompt.cursor_position(), 0);

        prompt.move_cursor_right(); // Move past 'a' (1 byte)
        assert_eq!(prompt.cursor_position(), 1);

        prompt.move_cursor_right(); // Move past '世' (3 bytes)
        assert_eq!(prompt.cursor_position(), 4);

        prompt.move_cursor_right(); // Move past '😀' (4 bytes)
        assert_eq!(prompt.cursor_position(), 8);

        prompt.move_cursor_right(); // Move past 'b' (1 byte)
        assert_eq!(prompt.cursor_position(), 9);

        // Now navigate backwards
        prompt.move_cursor_left(); // Back over 'b'
        assert_eq!(prompt.cursor_position(), 8);

        prompt.move_cursor_left(); // Back over '😀'
        assert_eq!(prompt.cursor_position(), 4);

        prompt.move_cursor_left(); // Back over '世'
        assert_eq!(prompt.cursor_position(), 1);

        prompt.move_cursor_left(); // Back over 'a'
        assert_eq!(prompt.cursor_position(), 0);
    }

    #[test]
    fn test_input_prompt_delete_multibyte_chars() {
        let mut prompt = InputPrompt::new("Title", "Message");
        prompt.set_value("Hello世界😀".to_string());

        // Delete emoji at end
        prompt.delete_char();
        assert_eq!(prompt.value(), "Hello世界");

        // Delete '界'
        prompt.delete_char();
        assert_eq!(prompt.value(), "Hello世");

        // Delete '世'
        prompt.delete_char();
        assert_eq!(prompt.value(), "Hello");
    }

    // ------------------------------------------------------------------------
    // Comprehensive Stress Test
    // ------------------------------------------------------------------------

    #[test]
    fn test_input_prompt_comprehensive_stress() {
        let mut prompt = InputPrompt::new("Stress Test", "Testing all features");

        // Phase 1: Insert 1000 characters
        for i in 0..1000 {
            prompt.insert_char(char::from_u32((i % 26) + 97).unwrap());
        }
        assert_eq!(prompt.value().len(), 1000);

        // Phase 2: Navigate to middle
        prompt.move_cursor_start();
        for _ in 0..500 {
            prompt.move_cursor_right();
        }

        // Phase 3: Insert unicode
        prompt.insert_char('世');
        prompt.insert_char('😀');
        assert_eq!(prompt.value().len(), 1007); // 1000 + 3 (世) + 4 (😀)

        // Phase 4: Delete some characters
        for _ in 0..100 {
            prompt.delete_char();
        }

        // Phase 5: Clear and rebuild
        prompt.clear();
        assert_eq!(prompt.value(), "");

        // Phase 6: Build new content
        prompt.set_value("Final test 最終テスト 😀".to_string());
        assert!(prompt.value().contains("Final"));
        assert!(prompt.value().contains("最終"));
        assert!(prompt.value().contains('😀'));

        // Phase 7: Focus toggle
        prompt.set_focused(false);
        assert!(!prompt.is_focused());
        prompt.set_focused(true);
        assert!(prompt.is_focused());

        // Phase 8: Placeholder test
        prompt.clear();
        let with_placeholder = InputPrompt::new("Title", "Message")
            .with_placeholder("Custom placeholder");
        assert_eq!(with_placeholder.placeholder, "Custom placeholder");
    }
}
