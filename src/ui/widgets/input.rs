//! Input field widget
//!
//! Single-line text input with placeholder support

use crate::ui::theme::ToadTheme;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

/// A single-line text input widget
#[derive(Debug)]
pub struct InputField {
    /// The current input text
    value: String,
    /// Placeholder text when empty
    placeholder: String,
    /// Cursor position (byte index)
    cursor_position: usize,
    /// Whether the input is focused
    is_focused: bool,
}

impl InputField {
    pub fn new() -> Self {
        Self {
            value: String::new(),
            placeholder: "Enter @ to mention files or / for commands".to_string(),
            cursor_position: 0,
            is_focused: false,
        }
    }

    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn set_value(&mut self, value: String) {
        self.value = value;
        self.cursor_position = self.value.len();
    }

    pub fn is_focused(&self) -> bool {
        self.is_focused
    }

    pub fn set_focused(&mut self, focused: bool) {
        self.is_focused = focused;
    }

    /// Insert a character at the cursor position
    pub fn insert_char(&mut self, c: char) {
        self.value.insert(self.cursor_position, c);
        self.cursor_position += c.len_utf8();
    }

    /// Delete the character before the cursor
    pub fn delete_char(&mut self) {
        if self.cursor_position > 0 {
            let mut chars: Vec<char> = self.value.chars().collect();
            let char_pos = self.char_position();
            if char_pos > 0 {
                chars.remove(char_pos - 1);
                self.value = chars.into_iter().collect();
                self.cursor_position = self.char_to_byte_idx(char_pos - 1);
            }
        }
    }

    /// Move cursor left
    pub fn move_cursor_left(&mut self) {
        if self.cursor_position > 0 {
            let char_pos = self.char_position();
            if char_pos > 0 {
                self.cursor_position = self.char_to_byte_idx(char_pos - 1);
            }
        }
    }

    /// Move cursor right
    pub fn move_cursor_right(&mut self) {
        let char_pos = self.char_position();
        let char_count = self.value.chars().count();
        if char_pos < char_count {
            self.cursor_position = self.char_to_byte_idx(char_pos + 1);
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
    }

    /// Get character position from byte position
    fn char_position(&self) -> usize {
        self.value[..self.cursor_position].chars().count()
    }

    /// Convert character index to byte index
    fn char_to_byte_idx(&self, char_idx: usize) -> usize {
        self.value
            .char_indices()
            .nth(char_idx)
            .map(|(idx, _)| idx)
            .unwrap_or(self.value.len())
    }

    /// Render the input field
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let display_text = if self.value.is_empty() {
            // Show placeholder
            Line::from(vec![
                Span::styled("> ", Style::default().fg(ToadTheme::TOAD_GREEN)),
                Span::styled(
                    &self.placeholder,
                    Style::default()
                        .fg(ToadTheme::DARK_GRAY)
                        .add_modifier(Modifier::ITALIC),
                ),
            ])
        } else {
            // Show actual input
            let before_cursor = &self.value[..self.cursor_position];
            let after_cursor = &self.value[self.cursor_position..];

            let cursor_char = if after_cursor.is_empty() {
                " "
            } else {
                &after_cursor[..1]
            };

            let rest = if after_cursor.is_empty() {
                ""
            } else {
                &after_cursor[1..]
            };

            let mut spans = vec![
                Span::styled("> ", Style::default().fg(ToadTheme::TOAD_GREEN)),
                Span::styled(before_cursor, Style::default().fg(ToadTheme::FOREGROUND)),
            ];

            if self.is_focused {
                // Show cursor
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

        let paragraph = Paragraph::new(display_text);
        frame.render_widget(paragraph, area);
    }
}

impl Default for InputField {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_field_new() {
        let input = InputField::new();
        assert_eq!(input.value(), "");
        assert_eq!(input.cursor_position, 0);
        assert!(!input.is_focused());
    }

    #[test]
    fn test_input_field_default() {
        let input = InputField::default();
        assert_eq!(input.value(), "");
        assert!(!input.is_focused());
    }

    #[test]
    fn test_input_field_with_placeholder() {
        let input = InputField::new().with_placeholder("Custom placeholder");
        assert_eq!(input.placeholder, "Custom placeholder");
    }

    #[test]
    fn test_input_field_set_value() {
        let mut input = InputField::new();
        input.set_value("test".to_string());
        assert_eq!(input.value(), "test");
        assert_eq!(input.cursor_position, 4); // Cursor at end
    }

    #[test]
    fn test_input_field_insert_char() {
        let mut input = InputField::new();
        input.insert_char('a');
        input.insert_char('b');
        input.insert_char('c');
        assert_eq!(input.value(), "abc");
        assert_eq!(input.cursor_position, 3);
    }

    #[test]
    fn test_input_field_insert_unicode() {
        let mut input = InputField::new();
        input.insert_char('🐸');
        input.insert_char('日');
        input.insert_char('本');
        assert_eq!(input.value(), "🐸日本");
        // Cursor position is in bytes, emoji is 4 bytes, Japanese chars are 3 bytes each
        assert!(input.cursor_position > 3);
    }

    #[test]
    fn test_input_field_delete_char() {
        let mut input = InputField::new();
        input.insert_char('a');
        input.insert_char('b');
        input.insert_char('c');

        input.delete_char();
        assert_eq!(input.value(), "ab");

        input.delete_char();
        assert_eq!(input.value(), "a");

        input.delete_char();
        assert_eq!(input.value(), "");
    }

    #[test]
    fn test_input_field_delete_char_empty() {
        let mut input = InputField::new();
        input.delete_char(); // Should not panic on empty
        assert_eq!(input.value(), "");
    }

    #[test]
    fn test_input_field_delete_unicode() {
        let mut input = InputField::new();
        input.insert_char('🐸');
        input.delete_char();
        assert_eq!(input.value(), "");
    }

    #[test]
    fn test_input_field_move_cursor_left() {
        let mut input = InputField::new();
        input.set_value("abc".to_string());

        input.move_cursor_left();
        assert_eq!(input.cursor_position, 2);

        input.move_cursor_left();
        assert_eq!(input.cursor_position, 1);
    }

    #[test]
    fn test_input_field_move_cursor_left_boundary() {
        let mut input = InputField::new();
        input.set_value("a".to_string());

        input.move_cursor_left();
        assert_eq!(input.cursor_position, 0);

        input.move_cursor_left(); // Should not go negative
        assert_eq!(input.cursor_position, 0);
    }

    #[test]
    fn test_input_field_move_cursor_right() {
        let mut input = InputField::new();
        input.set_value("abc".to_string());
        input.cursor_position = 0;

        input.move_cursor_right();
        assert_eq!(input.cursor_position, 1);

        input.move_cursor_right();
        assert_eq!(input.cursor_position, 2);
    }

    #[test]
    fn test_input_field_move_cursor_right_boundary() {
        let mut input = InputField::new();
        input.set_value("ab".to_string());

        input.move_cursor_right(); // Already at end, should not move
        assert_eq!(input.cursor_position, 2);
    }

    #[test]
    fn test_input_field_move_cursor_start() {
        let mut input = InputField::new();
        input.set_value("abc".to_string());

        input.move_cursor_start();
        assert_eq!(input.cursor_position, 0);
    }

    #[test]
    fn test_input_field_move_cursor_end() {
        let mut input = InputField::new();
        input.set_value("abc".to_string());
        input.cursor_position = 0;

        input.move_cursor_end();
        assert_eq!(input.cursor_position, 3);
    }

    #[test]
    fn test_input_field_clear() {
        let mut input = InputField::new();
        input.set_value("test".to_string());

        input.clear();
        assert_eq!(input.value(), "");
        assert_eq!(input.cursor_position, 0);
    }

    #[test]
    fn test_input_field_set_focused() {
        let mut input = InputField::new();

        input.set_focused(true);
        assert!(input.is_focused());

        input.set_focused(false);
        assert!(!input.is_focused());
    }

    #[test]
    fn test_input_field_very_long_text() {
        let mut input = InputField::new();
        let long_text = "a".repeat(10000);
        input.set_value(long_text.clone());

        assert_eq!(input.value(), long_text);
        assert_eq!(input.cursor_position, 10000);
    }

    #[test]
    fn test_input_field_insert_at_middle() {
        let mut input = InputField::new();
        input.set_value("ac".to_string());
        input.cursor_position = 1; // Between 'a' and 'c'

        input.insert_char('b');
        assert_eq!(input.value(), "abc");
    }

    #[test]
    fn test_input_field_delete_at_middle() {
        let mut input = InputField::new();
        input.set_value("abc".to_string());
        input.cursor_position = 2; // After 'b'

        input.delete_char();
        assert_eq!(input.value(), "ac");
    }

    #[test]
    fn test_input_field_cursor_movement_sequence() {
        let mut input = InputField::new();
        input.set_value("hello".to_string());

        input.move_cursor_start();
        assert_eq!(input.cursor_position, 0);

        input.move_cursor_right();
        input.move_cursor_right();
        assert_eq!(input.cursor_position, 2);

        input.delete_char();
        assert_eq!(input.value(), "hllo");

        input.insert_char('e');
        assert_eq!(input.value(), "hello");
    }

    #[test]
    fn test_input_field_unicode_cursor_navigation() {
        let mut input = InputField::new();
        input.insert_char('a');
        input.insert_char('🐸');
        input.insert_char('b');

        assert_eq!(input.value(), "a🐸b");

        input.move_cursor_left(); // Before 'b'
        input.move_cursor_left(); // Before emoji
        input.move_cursor_left(); // Before 'a'
        assert_eq!(input.cursor_position, 0);
    }

    #[test]
    fn test_input_field_empty_value() {
        let input = InputField::new();
        assert_eq!(input.value(), "");
        assert_eq!(input.value().len(), 0);
    }

    #[test]
    fn test_input_field_multiple_instances() {
        let input1 = InputField::new();
        let mut input2 = InputField::new();

        input2.set_value("test".to_string());

        assert_eq!(input1.value(), "");
        assert_eq!(input2.value(), "test");
    }

    #[test]
    fn test_input_field_char_position() {
        let mut input = InputField::new();
        input.set_value("abc".to_string());

        // char_position is private, but we can test it indirectly
        // by testing cursor movement
        input.cursor_position = 0;
        input.move_cursor_right();
        assert_eq!(input.cursor_position, 1);
    }

    #[test]
    fn test_input_field_special_characters() {
        let mut input = InputField::new();
        input.insert_char('\n');
        input.insert_char('\t');
        input.insert_char(' ');

        assert_eq!(input.value(), "\n\t ");
    }

    #[test]
    fn test_input_field_emoji_sequence() {
        let mut input = InputField::new();
        input.insert_char('👨');
        input.insert_char('‍');
        input.insert_char('💻');

        // Complex emoji sequences
        assert!(input.value().len() > 3);
    }
}
