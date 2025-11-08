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
