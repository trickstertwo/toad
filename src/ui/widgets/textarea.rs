//! Textarea widget
//!
//! Multi-line text editing with cursor navigation

use crate::ui::theme::ToadTheme;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
};

/// Multi-line text editor
pub struct Textarea {
    title: String,
    lines: Vec<String>,
    cursor_row: usize,
    cursor_col: usize,
    scroll_offset: usize,
    show_line_numbers: bool,
    is_focused: bool,
}

impl Textarea {
    /// Create a new textarea
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            lines: vec![String::new()],
            cursor_row: 0,
            cursor_col: 0,
            scroll_offset: 0,
            show_line_numbers: false,
            is_focused: false,
        }
    }

    /// Set the content
    pub fn set_content(&mut self, content: impl Into<String>) {
        let content = content.into();
        if content.is_empty() {
            self.lines = vec![String::new()];
        } else {
            self.lines = content.lines().map(|s| s.to_string()).collect();
        }
        self.cursor_row = 0;
        self.cursor_col = 0;
        self.scroll_offset = 0;
    }

    /// Get the content as a single string
    pub fn content(&self) -> String {
        self.lines.join("\n")
    }

    /// Get the lines
    pub fn lines(&self) -> &[String] {
        &self.lines
    }

    /// Set focused state
    pub fn set_focused(&mut self, focused: bool) {
        self.is_focused = focused;
    }

    /// Toggle line numbers
    pub fn set_show_line_numbers(&mut self, show: bool) {
        self.show_line_numbers = show;
    }

    /// Insert a character at cursor
    pub fn insert_char(&mut self, c: char) {
        if c == '\n' {
            self.insert_newline();
        } else if let Some(line) = self.lines.get_mut(self.cursor_row) {
            line.insert(self.cursor_col, c);
            self.cursor_col += c.len_utf8();
        }
    }

    /// Insert a newline at cursor
    pub fn insert_newline(&mut self) {
        if let Some(line) = self.lines.get_mut(self.cursor_row) {
            let rest = line.split_off(self.cursor_col);
            self.lines.insert(self.cursor_row + 1, rest);
            self.cursor_row += 1;
            self.cursor_col = 0;
            self.ensure_cursor_visible();
        }
    }

    /// Delete character before cursor (backspace)
    pub fn delete_char(&mut self) {
        if self.cursor_col > 0 {
            if let Some(line) = self.lines.get_mut(self.cursor_row) {
                let chars: Vec<char> = line.chars().collect();
                let char_pos = line[..self.cursor_col].chars().count();
                if char_pos > 0 {
                    let mut new_chars = chars;
                    new_chars.remove(char_pos - 1);
                    *line = new_chars.into_iter().collect();
                    self.cursor_col = self.char_to_byte_idx(self.cursor_row, char_pos - 1);
                }
            }
        } else if self.cursor_row > 0 {
            // Join with previous line
            let current_line = self.lines.remove(self.cursor_row);
            self.cursor_row -= 1;
            if let Some(prev_line) = self.lines.get_mut(self.cursor_row) {
                self.cursor_col = prev_line.len();
                prev_line.push_str(&current_line);
            }
            self.ensure_cursor_visible();
        }
    }

    /// Delete character at cursor (delete key)
    pub fn delete_char_forward(&mut self) {
        if let Some(line) = self.lines.get_mut(self.cursor_row) {
            if self.cursor_col < line.len() {
                let chars: Vec<char> = line.chars().collect();
                let char_pos = line[..self.cursor_col].chars().count();
                let mut new_chars = chars;
                if char_pos < new_chars.len() {
                    new_chars.remove(char_pos);
                    *line = new_chars.into_iter().collect();
                }
            } else if self.cursor_row < self.lines.len() - 1 {
                // Join with next line
                let next_line = self.lines.remove(self.cursor_row + 1);
                if let Some(current_line) = self.lines.get_mut(self.cursor_row) {
                    current_line.push_str(&next_line);
                }
            }
        }
    }

    /// Move cursor up
    pub fn move_up(&mut self) {
        if self.cursor_row > 0 {
            self.cursor_row -= 1;
            self.clamp_cursor_col();
            self.ensure_cursor_visible();
        }
    }

    /// Move cursor down
    pub fn move_down(&mut self) {
        if self.cursor_row < self.lines.len() - 1 {
            self.cursor_row += 1;
            self.clamp_cursor_col();
            self.ensure_cursor_visible();
        }
    }

    /// Move cursor left
    pub fn move_left(&mut self) {
        if self.cursor_col > 0 {
            let char_pos = self.char_position(self.cursor_row);
            if char_pos > 0 {
                self.cursor_col = self.char_to_byte_idx(self.cursor_row, char_pos - 1);
            }
        } else if self.cursor_row > 0 {
            self.cursor_row -= 1;
            self.cursor_col = self
                .lines
                .get(self.cursor_row)
                .map(|l| l.len())
                .unwrap_or(0);
            self.ensure_cursor_visible();
        }
    }

    /// Move cursor right
    pub fn move_right(&mut self) {
        if let Some(line) = self.lines.get(self.cursor_row) {
            if self.cursor_col < line.len() {
                let char_pos = self.char_position(self.cursor_row);
                self.cursor_col = self.char_to_byte_idx(self.cursor_row, char_pos + 1);
            } else if self.cursor_row < self.lines.len() - 1 {
                self.cursor_row += 1;
                self.cursor_col = 0;
                self.ensure_cursor_visible();
            }
        }
    }

    /// Move cursor to start of line
    pub fn move_to_line_start(&mut self) {
        self.cursor_col = 0;
    }

    /// Move cursor to end of line
    pub fn move_to_line_end(&mut self) {
        if let Some(line) = self.lines.get(self.cursor_row) {
            self.cursor_col = line.len();
        }
    }

    /// Scroll up by one line
    pub fn scroll_up(&mut self) {
        if self.scroll_offset > 0 {
            self.scroll_offset -= 1;
        }
    }

    /// Scroll down by one line
    pub fn scroll_down(&mut self) {
        if self.scroll_offset < self.lines.len().saturating_sub(1) {
            self.scroll_offset += 1;
        }
    }

    /// Get cursor position (row, col)
    pub fn cursor_position(&self) -> (usize, usize) {
        (self.cursor_row, self.cursor_col)
    }

    /// Get line count
    pub fn line_count(&self) -> usize {
        self.lines.len()
    }

    /// Clamp cursor column to line length
    fn clamp_cursor_col(&mut self) {
        if let Some(line) = self.lines.get(self.cursor_row)
            && self.cursor_col > line.len() {
                self.cursor_col = line.len();
            }
    }

    /// Ensure cursor is visible in the viewport
    fn ensure_cursor_visible(&mut self) {
        // This would need viewport height, simplified for now
        if self.cursor_row < self.scroll_offset {
            self.scroll_offset = self.cursor_row;
        }
    }

    /// Get character position from byte position
    fn char_position(&self, row: usize) -> usize {
        if let Some(line) = self.lines.get(row) {
            line[..self.cursor_col].chars().count()
        } else {
            0
        }
    }

    /// Convert character index to byte index
    fn char_to_byte_idx(&self, row: usize, char_idx: usize) -> usize {
        if let Some(line) = self.lines.get(row) {
            line.char_indices()
                .nth(char_idx)
                .map(|(idx, _)| idx)
                .unwrap_or(line.len())
        } else {
            0
        }
    }

    /// Render the textarea
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let block = Block::default()
            .title(format!(" {} ", self.title))
            .title_style(
                Style::default()
                    .fg(ToadTheme::TOAD_GREEN)
                    .add_modifier(Modifier::BOLD),
            )
            .borders(Borders::ALL)
            .border_style(Style::default().fg(ToadTheme::TOAD_GREEN));

        let inner = block.inner(area);
        frame.render_widget(block, area);

        let visible_height = inner.height as usize;
        let end_line = (self.scroll_offset + visible_height).min(self.lines.len());

        // Build visible lines
        let mut display_lines = Vec::new();
        for (idx, line) in self.lines[self.scroll_offset..end_line].iter().enumerate() {
            let line_number = self.scroll_offset + idx;
            let is_cursor_line = line_number == self.cursor_row;

            let line_text = if self.show_line_numbers {
                format!("{:4} │ {}", line_number + 1, line)
            } else {
                line.clone()
            };

            let style = if is_cursor_line && self.is_focused {
                Style::default()
                    .fg(ToadTheme::FOREGROUND)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(ToadTheme::FOREGROUND)
            };

            display_lines.push(Line::from(Span::styled(line_text, style)));
        }

        let paragraph = Paragraph::new(display_lines);
        frame.render_widget(paragraph, inner);

        // Render scrollbar
        if self.lines.len() > visible_height {
            let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .style(Style::default().fg(ToadTheme::DARK_GRAY))
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓"));

            let mut scrollbar_state =
                ScrollbarState::new(self.lines.len()).position(self.scroll_offset);

            frame.render_stateful_widget(scrollbar, inner, &mut scrollbar_state);
        }
    }
}

impl Default for Textarea {
    fn default() -> Self {
        Self::new("Editor")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_textarea_creation() {
        let textarea = Textarea::new("Test");
        assert_eq!(textarea.line_count(), 1);
        assert_eq!(textarea.content(), "");
    }

    #[test]
    fn test_textarea_insert_char() {
        let mut textarea = Textarea::new("Test");
        textarea.insert_char('H');
        textarea.insert_char('i');
        assert_eq!(textarea.content(), "Hi");
    }

    #[test]
    fn test_textarea_newline() {
        let mut textarea = Textarea::new("Test");
        textarea.insert_char('A');
        textarea.insert_newline();
        textarea.insert_char('B');
        assert_eq!(textarea.content(), "A\nB");
        assert_eq!(textarea.line_count(), 2);
    }

    #[test]
    fn test_textarea_delete() {
        let mut textarea = Textarea::new("Test");
        textarea.set_content("Hello");
        textarea.move_to_line_end();
        textarea.delete_char();
        assert_eq!(textarea.content(), "Hell");
    }

    #[test]
    fn test_textarea_navigation() {
        let mut textarea = Textarea::new("Test");
        textarea.set_content("Line1\nLine2\nLine3");

        assert_eq!(textarea.cursor_position(), (0, 0));

        textarea.move_down();
        assert_eq!(textarea.cursor_position().0, 1);

        textarea.move_down();
        assert_eq!(textarea.cursor_position().0, 2);

        textarea.move_up();
        assert_eq!(textarea.cursor_position().0, 1);
    }
}
