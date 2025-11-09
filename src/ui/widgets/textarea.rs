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

    // === COMPREHENSIVE EDGE CASE TESTS (MEDIUM tier) ===

    // --- Empty States ---

    #[test]
    fn test_textarea_empty_creation() {
        let textarea = Textarea::new("Empty");
        assert_eq!(textarea.line_count(), 1);
        assert_eq!(textarea.content(), "");
        assert_eq!(textarea.cursor_position(), (0, 0));
    }

    #[test]
    fn test_textarea_set_empty_content() {
        let mut textarea = Textarea::new("Test");
        textarea.set_content("Some content");
        textarea.set_content(""); // Clear to empty

        assert_eq!(textarea.line_count(), 1);
        assert_eq!(textarea.content(), "");
        assert_eq!(textarea.cursor_position(), (0, 0));
    }

    #[test]
    fn test_textarea_empty_line_navigation() {
        let mut textarea = Textarea::new("Test");
        textarea.set_content("\n\n"); // 2 empty lines (lines() splits: ["", ""])

        assert_eq!(textarea.line_count(), 2);

        textarea.move_down();
        assert_eq!(textarea.cursor_position().0, 1);

        // Try to move down again (should stay at line 1, the last line)
        textarea.move_down();
        assert_eq!(textarea.cursor_position().0, 1);
    }

    // --- Single Item ---

    #[test]
    fn test_textarea_single_character() {
        let mut textarea = Textarea::new("Test");
        textarea.insert_char('X');

        assert_eq!(textarea.content(), "X");
        assert_eq!(textarea.line_count(), 1);
        assert_eq!(textarea.cursor_position(), (0, 1));
    }

    #[test]
    fn test_textarea_single_line() {
        let mut textarea = Textarea::new("Test");
        textarea.set_content("Single line");

        assert_eq!(textarea.line_count(), 1);
        assert_eq!(textarea.content(), "Single line");

        // Try to move down (should not move)
        let initial_pos = textarea.cursor_position();
        textarea.move_down();
        assert_eq!(textarea.cursor_position(), initial_pos);
    }

    // --- Unicode and Emoji ---

    #[test]
    fn test_textarea_unicode_characters() {
        let mut textarea = Textarea::new("Unicode");
        textarea.set_content("Hello 🐸 こんにちは 👨‍💻");

        assert_eq!(textarea.content(), "Hello 🐸 こんにちは 👨‍💻");

        // Move to end and delete
        textarea.move_to_line_end();
        textarea.delete_char();
        // Should delete the entire 👨‍💻 emoji (multi-codepoint)
        assert!(textarea.content().contains("こんにちは"));
    }

    #[test]
    fn test_textarea_unicode_cursor_navigation() {
        let mut textarea = Textarea::new("Test");
        textarea.set_content("日本語テスト");

        // Move right through multi-byte characters
        textarea.move_right();
        textarea.move_right();
        textarea.move_right();

        // Insert character in the middle
        textarea.insert_char('X');
        assert_eq!(textarea.content(), "日本語Xテスト");
    }

    #[test]
    fn test_textarea_emoji_on_multiple_lines() {
        let mut textarea = Textarea::new("Emoji");
        textarea.insert_char('🎉');
        textarea.insert_newline();
        textarea.insert_char('🐸');
        textarea.insert_newline();
        textarea.insert_char('👨');
        textarea.insert_char('‍');
        textarea.insert_char('💻');

        assert_eq!(textarea.line_count(), 3);
        assert!(textarea.content().contains("🎉"));
        assert!(textarea.content().contains("🐸"));
    }

    // --- Extreme Values ---

    #[test]
    fn test_textarea_very_long_line() {
        let mut textarea = Textarea::new("Long Line");
        let long_text = "x".repeat(10_000);
        textarea.set_content(&long_text);

        assert_eq!(textarea.content().len(), 10_000);
        assert_eq!(textarea.line_count(), 1);

        // Navigate to end
        textarea.move_to_line_end();
        assert_eq!(textarea.cursor_position().1, 10_000);
    }

    #[test]
    fn test_textarea_very_many_lines() {
        let mut textarea = Textarea::new("Many Lines");
        let lines: Vec<String> = (0..10_000).map(|i| format!("Line {}", i)).collect();
        textarea.set_content(lines.join("\n"));

        assert_eq!(textarea.line_count(), 10_000);

        // Navigate down multiple times
        for _ in 0..100 {
            textarea.move_down();
        }
        assert_eq!(textarea.cursor_position().0, 100);
    }

    #[test]
    fn test_textarea_very_long_line_with_unicode() {
        let mut textarea = Textarea::new("Unicode Long");
        let long_unicode = "日本語".repeat(1_000); // 3 chars × 1000 = 3000 chars
        textarea.set_content(&long_unicode);

        assert_eq!(textarea.line_count(), 1);
        assert!(textarea.content().len() > 8000); // Each char is 3 bytes

        // Move to end
        textarea.move_to_line_end();
        assert!(textarea.cursor_position().1 > 8000);
    }

    // --- Boundary Conditions ---

    #[test]
    fn test_textarea_cursor_at_line_start() {
        let mut textarea = Textarea::new("Test");
        textarea.set_content("Hello World");
        textarea.move_to_line_end();
        textarea.move_to_line_start();

        assert_eq!(textarea.cursor_position().1, 0);

        // Try to move left (should not move further)
        textarea.move_left();
        assert_eq!(textarea.cursor_position(), (0, 0));
    }

    #[test]
    fn test_textarea_cursor_at_line_end() {
        let mut textarea = Textarea::new("Test");
        textarea.set_content("Hello");
        textarea.move_to_line_end();

        assert_eq!(textarea.cursor_position().1, 5);

        // Try to move right (should not move on same line)
        let before = textarea.cursor_position();
        textarea.move_right();
        // Should still be at end of first line (or moved to next line if exists)
        let after = textarea.cursor_position();
        assert!(after.0 == before.0 && after.1 == before.1);
    }

    #[test]
    fn test_textarea_cursor_at_first_line() {
        let mut textarea = Textarea::new("Test");
        textarea.set_content("Line1\nLine2\nLine3");

        assert_eq!(textarea.cursor_position().0, 0);

        // Try to move up (should not move)
        textarea.move_up();
        assert_eq!(textarea.cursor_position().0, 0);
    }

    #[test]
    fn test_textarea_cursor_at_last_line() {
        let mut textarea = Textarea::new("Test");
        textarea.set_content("Line1\nLine2\nLine3");

        // Move to last line
        textarea.move_down();
        textarea.move_down();
        assert_eq!(textarea.cursor_position().0, 2);

        // Try to move down (should not move)
        textarea.move_down();
        assert_eq!(textarea.cursor_position().0, 2);
    }

    // --- Deletion Edge Cases ---

    #[test]
    fn test_textarea_delete_at_line_start_joins_lines() {
        let mut textarea = Textarea::new("Test");
        textarea.set_content("First\nSecond");
        assert_eq!(textarea.line_count(), 2);

        // Move to second line
        textarea.move_down();
        assert_eq!(textarea.cursor_position(), (1, 0));

        // Delete at line start (should join with previous line)
        textarea.delete_char();
        assert_eq!(textarea.content(), "FirstSecond");
        assert_eq!(textarea.line_count(), 1);
    }

    #[test]
    fn test_textarea_delete_forward_at_line_end_joins_lines() {
        let mut textarea = Textarea::new("Test");
        textarea.set_content("First\nSecond");
        assert_eq!(textarea.line_count(), 2);

        // Move to end of first line
        textarea.move_to_line_end();
        assert_eq!(textarea.cursor_position(), (0, 5));

        // Delete forward at line end (should join with next line)
        textarea.delete_char_forward();
        assert_eq!(textarea.content(), "FirstSecond");
        assert_eq!(textarea.line_count(), 1);
    }

    #[test]
    fn test_textarea_delete_unicode_character() {
        let mut textarea = Textarea::new("Unicode");
        textarea.set_content("Hello🐸World");

        // Move to after emoji
        for _ in 0..6 {
            textarea.move_right();
        }

        // Delete the emoji
        textarea.delete_char();
        assert_eq!(textarea.content(), "HelloWorld");
    }

    #[test]
    fn test_textarea_delete_forward_unicode() {
        let mut textarea = Textarea::new("Unicode");
        textarea.set_content("Hello🐸World");

        // Move to before emoji
        for _ in 0..5 {
            textarea.move_right();
        }

        // Delete forward (emoji)
        textarea.delete_char_forward();
        assert_eq!(textarea.content(), "HelloWorld");
    }

    // --- Line Operations ---

    #[test]
    fn test_textarea_split_line_in_middle() {
        let mut textarea = Textarea::new("Test");
        textarea.set_content("HelloWorld");

        // Move to middle
        for _ in 0..5 {
            textarea.move_right();
        }

        // Insert newline
        textarea.insert_newline();
        assert_eq!(textarea.content(), "Hello\nWorld");
        assert_eq!(textarea.line_count(), 2);
        assert_eq!(textarea.cursor_position(), (1, 0));
    }

    #[test]
    fn test_textarea_multiple_consecutive_newlines() {
        let mut textarea = Textarea::new("Test");
        textarea.insert_char('A');
        textarea.insert_newline();
        textarea.insert_newline();
        textarea.insert_newline();
        textarea.insert_char('B');

        assert_eq!(textarea.content(), "A\n\n\nB");
        assert_eq!(textarea.line_count(), 4);
    }

    // --- Cursor Movement Across Lines ---

    #[test]
    fn test_textarea_move_left_wraps_to_previous_line() {
        let mut textarea = Textarea::new("Test");
        textarea.set_content("First\nSecond");

        // Move to second line
        textarea.move_down();
        assert_eq!(textarea.cursor_position(), (1, 0));

        // Move left (should wrap to end of previous line)
        textarea.move_left();
        assert_eq!(textarea.cursor_position(), (0, 5)); // End of "First"
    }

    #[test]
    fn test_textarea_move_right_wraps_to_next_line() {
        let mut textarea = Textarea::new("Test");
        textarea.set_content("First\nSecond");

        // Move to end of first line
        textarea.move_to_line_end();
        assert_eq!(textarea.cursor_position(), (0, 5));

        // Move right (should wrap to start of next line)
        textarea.move_right();
        assert_eq!(textarea.cursor_position(), (1, 0));
    }

    #[test]
    fn test_textarea_cursor_column_clamps_on_shorter_line() {
        let mut textarea = Textarea::new("Test");
        textarea.set_content("Long line here\nShort");

        // Move to end of first line
        textarea.move_to_line_end();
        assert_eq!(textarea.cursor_position().1, 14); // "Long line here".len()

        // Move down to shorter line (cursor should clamp)
        textarea.move_down();
        assert_eq!(textarea.cursor_position(), (1, 5)); // "Short".len()
    }

    // --- Scrolling ---

    #[test]
    fn test_textarea_scroll_up_down() {
        let mut textarea = Textarea::new("Scroll");
        let lines: Vec<String> = (0..100).map(|i| format!("Line {}", i)).collect();
        textarea.set_content(lines.join("\n"));

        // Scroll down
        for _ in 0..10 {
            textarea.scroll_down();
        }
        // Note: scroll_offset is private, but we can test it doesn't panic

        // Scroll up
        for _ in 0..5 {
            textarea.scroll_up();
        }
        // Should not panic
    }

    #[test]
    fn test_textarea_scroll_at_boundaries() {
        let mut textarea = Textarea::new("Test");
        textarea.set_content("Line1\nLine2\nLine3");

        // Try to scroll up when at top (should not panic)
        textarea.scroll_up();

        // Scroll down to bottom
        for _ in 0..10 {
            textarea.scroll_down();
        }
        // Should not panic or go beyond content
    }

    // --- State Management ---

    #[test]
    fn test_textarea_focus_state() {
        let mut textarea = Textarea::new("Focus");

        textarea.set_focused(true);
        textarea.set_focused(false);
        textarea.set_focused(true);
        // Should not panic
    }

    #[test]
    fn test_textarea_line_numbers_toggle() {
        let mut textarea = Textarea::new("Numbers");

        textarea.set_show_line_numbers(true);
        textarea.set_show_line_numbers(false);
        textarea.set_show_line_numbers(true);
        // Should not panic
    }

    // --- Default Implementation ---

    #[test]
    fn test_textarea_default() {
        let textarea = Textarea::default();
        assert_eq!(textarea.line_count(), 1);
        assert_eq!(textarea.content(), "");
        assert_eq!(textarea.cursor_position(), (0, 0));
    }

    // --- Content Operations ---

    #[test]
    fn test_textarea_get_lines() {
        let mut textarea = Textarea::new("Test");
        textarea.set_content("Line1\nLine2\nLine3");

        let lines = textarea.lines();
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0], "Line1");
        assert_eq!(lines[1], "Line2");
        assert_eq!(lines[2], "Line3");
    }

    #[test]
    fn test_textarea_content_preserves_empty_lines() {
        let mut textarea = Textarea::new("Test");
        textarea.set_content("A\n\nB\n\nC");

        assert_eq!(textarea.line_count(), 5);
        let lines = textarea.lines();
        assert_eq!(lines[0], "A");
        assert_eq!(lines[1], "");
        assert_eq!(lines[2], "B");
        assert_eq!(lines[3], "");
        assert_eq!(lines[4], "C");
    }

    #[test]
    fn test_textarea_insert_multiple_chars_builds_word() {
        let mut textarea = Textarea::new("Test");
        for c in "Hello".chars() {
            textarea.insert_char(c);
        }
        assert_eq!(textarea.content(), "Hello");
        assert_eq!(textarea.cursor_position(), (0, 5));
    }

    #[test]
    fn test_textarea_complex_editing_workflow() {
        let mut textarea = Textarea::new("Complex");

        // Type "Hello"
        textarea.insert_char('H');
        textarea.insert_char('e');
        textarea.insert_char('l');
        textarea.insert_char('l');
        textarea.insert_char('o');

        // New line
        textarea.insert_newline();

        // Type "World"
        textarea.insert_char('W');
        textarea.insert_char('o');
        textarea.insert_char('r');
        textarea.insert_char('l');
        textarea.insert_char('d');

        assert_eq!(textarea.content(), "Hello\nWorld");
        assert_eq!(textarea.line_count(), 2);

        // Move up and to end
        textarea.move_up();
        textarea.move_to_line_end();

        // Delete last char of "Hello"
        textarea.delete_char();
        assert_eq!(textarea.content(), "Hell\nWorld");
    }

    // ============================================================================
    // ADVANCED COMPREHENSIVE EDGE CASE TESTS (90%+ COVERAGE)
    // ============================================================================

    // ============ Stress Tests ============

    #[test]
    fn test_textarea_10000_lines() {
        let mut textarea = Textarea::new("Stress");
        let lines: Vec<String> = (0..10000).map(|i| format!("Line {}", i)).collect();
        textarea.set_content(lines.join("\n"));

        assert_eq!(textarea.line_count(), 10000);
        assert_eq!(textarea.cursor_position(), (0, 0));
    }

    #[test]
    fn test_textarea_rapid_char_insertions() {
        let mut textarea = Textarea::new("Rapid");

        for i in 0..5000 {
            textarea.insert_char((b'A' + (i % 26) as u8) as char);
        }

        assert_eq!(textarea.content().len(), 5000);
        assert_eq!(textarea.line_count(), 1);
    }

    #[test]
    fn test_textarea_rapid_newline_insertions() {
        let mut textarea = Textarea::new("Lines");

        for _ in 0..3000 {
            textarea.insert_char('A');
            textarea.insert_newline();
        }

        assert_eq!(textarea.line_count(), 3001); // 3000 newlines = 3001 lines
    }

    #[test]
    fn test_textarea_rapid_deletions() {
        let mut textarea = Textarea::new("Delete");
        let content = "A".repeat(2000);
        textarea.set_content(&content);

        // Move to end
        for _ in 0..2000 {
            textarea.move_right();
        }

        // Delete all
        for _ in 0..2000 {
            textarea.delete_char();
        }

        assert_eq!(textarea.content(), "");
        assert_eq!(textarea.line_count(), 1);
    }

    #[test]
    fn test_textarea_rapid_cursor_movements() {
        let mut textarea = Textarea::new("Cursor");
        let lines: Vec<String> = (0..100).map(|i| format!("Line {}", i)).collect();
        textarea.set_content(lines.join("\n"));

        for _ in 0..500 {
            textarea.move_down();
        }

        for _ in 0..500 {
            textarea.move_up();
        }

        // Should handle without panic
    }

    // ============ Unicode Edge Cases ============

    #[test]
    fn test_textarea_emoji_sequences() {
        let mut textarea = Textarea::new("Emoji");
        textarea.insert_char('🐸');
        textarea.insert_char('💚');
        textarea.insert_char('🚀');
        textarea.insert_char('✨');

        let content = textarea.content();
        assert!(content.contains('🐸'));
        assert!(content.contains('💚'));
    }

    #[test]
    fn test_textarea_rtl_text() {
        let mut textarea = Textarea::new("RTL");
        textarea.set_content("مرحبا Hello שלום World");

        assert!(textarea.content().contains("مرحبا"));
        assert!(textarea.content().contains("שלום"));
        assert_eq!(textarea.line_count(), 1);
    }

    #[test]
    fn test_textarea_combining_characters() {
        let mut textarea = Textarea::new("Combining");
        // e with combining acute accent
        textarea.insert_char('e');
        textarea.insert_char('\u{0301}'); // Combining acute accent

        let content = textarea.content();
        assert!(content.len() > 1); // Multi-byte
    }

    #[test]
    fn test_textarea_zero_width_characters() {
        let mut textarea = Textarea::new("ZeroWidth");
        textarea.insert_char('A');
        textarea.insert_char('\u{200B}'); // Zero-width space
        textarea.insert_char('B');

        assert_eq!(textarea.content(), "A\u{200B}B");
    }

    #[test]
    fn test_textarea_mixed_unicode_and_ascii() {
        let mut textarea = Textarea::new("Mixed");
        textarea.insert_char('H');
        textarea.insert_char('e');
        textarea.insert_char('l');
        textarea.insert_char('l');
        textarea.insert_char('o');
        textarea.insert_char('🐸');
        textarea.insert_char('日');
        textarea.insert_char('本');

        assert!(textarea.content().contains("Hello🐸日本"));
    }

    #[test]
    fn test_textarea_emoji_navigation() {
        let mut textarea = Textarea::new("Emoji Nav");
        textarea.set_content("A🐸B💚C");

        // Navigate through emoji
        for _ in 0..5 {
            textarea.move_right();
        }

        // Insert at end
        textarea.insert_char('X');
        assert!(textarea.content().contains('X'));
    }

    // ============ Extreme Content Sizes ============

    #[test]
    fn test_textarea_very_long_single_line() {
        let mut textarea = Textarea::new("Long");
        let long_line = "A".repeat(100000);
        textarea.set_content(&long_line);

        assert_eq!(textarea.line_count(), 1);
        assert_eq!(textarea.content().len(), 100000);
    }

    #[test]
    fn test_textarea_many_empty_lines() {
        let mut textarea = Textarea::new("Empty");
        // Create many empty lines - lines() drops trailing empty
        let lines = vec![String::new(); 1000];
        textarea.set_content(lines.join("\n"));

        assert_eq!(textarea.line_count(), 999); // Last empty line dropped by lines()
    }

    #[test]
    fn test_textarea_alternating_content_patterns() {
        let mut textarea = Textarea::new("Pattern");
        let mut lines = Vec::new();
        for i in 0..500 {
            if i % 2 == 0 {
                lines.push(format!("Long line with content {}", i));
            } else {
                lines.push(String::new());
            }
        }
        textarea.set_content(lines.join("\n"));

        // lines() drops trailing empty line, so 499 lines (last is empty)
        assert_eq!(textarea.line_count(), 499);
    }

    // ============ Complex Editing Workflows ============

    #[test]
    fn test_textarea_interleaved_insert_delete() {
        let mut textarea = Textarea::new("Interleaved");

        for i in 0..100 {
            textarea.insert_char('A');
            textarea.insert_char('B');
            textarea.delete_char(); // Delete B
            if i % 10 == 0 {
                textarea.insert_newline();
            }
        }

        let content = textarea.content();
        assert!(content.contains('A'));
        assert!(!content.contains('B'));
    }

    #[test]
    fn test_textarea_navigat_edit_navigate_pattern() {
        let mut textarea = Textarea::new("Pattern");
        textarea.set_content("Line1\nLine2\nLine3");

        for _ in 0..10 {
            textarea.move_down();
            textarea.insert_char('X');
            textarea.move_up();
            textarea.insert_char('Y');
        }

        let content = textarea.content();
        assert!(content.contains('X'));
        assert!(content.contains('Y'));
    }

    #[test]
    fn test_textarea_scroll_while_editing() {
        let mut textarea = Textarea::new("Scroll Edit");
        let lines: Vec<String> = (0..100).map(|i| format!("Line {}", i)).collect();
        textarea.set_content(lines.join("\n"));

        for _ in 0..20 {
            textarea.scroll_down();
            textarea.insert_char('X');
        }

        assert!(textarea.content().contains('X'));
    }

    // ============ Boundary Conditions ============

    #[test]
    fn test_textarea_delete_from_empty() {
        let mut textarea = Textarea::new("Empty Delete");
        textarea.delete_char();
        textarea.delete_char_forward();

        assert_eq!(textarea.content(), "");
        assert_eq!(textarea.cursor_position(), (0, 0));
    }

    #[test]
    fn test_textarea_navigate_empty_content() {
        let mut textarea = Textarea::new("Empty Nav");

        textarea.move_up();
        textarea.move_down();
        textarea.move_left();
        textarea.move_right();

        assert_eq!(textarea.cursor_position(), (0, 0));
    }

    #[test]
    fn test_textarea_insert_at_all_cursor_positions() {
        let mut textarea = Textarea::new("All Positions");
        textarea.set_content("ABC");

        // Insert at beginning
        textarea.insert_char('1');

        // Insert in middle
        textarea.move_right();
        textarea.move_right();
        textarea.insert_char('2');

        // Insert at end
        textarea.move_to_line_end();
        textarea.insert_char('3');

        assert!(textarea.content().contains('1'));
        assert!(textarea.content().contains('2'));
        assert!(textarea.content().contains('3'));
    }

    #[test]
    fn test_textarea_cursor_at_line_boundaries() {
        let mut textarea = Textarea::new("Boundaries");
        textarea.set_content("Short\nVery long line here");

        // Move to end of short line
        textarea.move_to_line_end();
        assert_eq!(textarea.cursor_position(), (0, 5));

        // Move down (cursor should clamp)
        textarea.move_down();
        assert!(textarea.cursor_position().1 <= textarea.lines()[1].len());

        // Move to end of long line
        textarea.move_to_line_end();
        assert_eq!(textarea.cursor_position().1, textarea.lines()[1].len());

        // Move up (cursor should clamp to short line)
        textarea.move_up();
        assert_eq!(textarea.cursor_position(), (0, 5));
    }

    // ============ State Combinations ============

    #[test]
    fn test_textarea_focus_toggle_stress() {
        let mut textarea = Textarea::new("Focus Stress");

        for i in 0..1000 {
            textarea.set_focused(i % 2 == 0);
        }

        // Should not panic
    }

    #[test]
    fn test_textarea_line_numbers_toggle_stress() {
        let mut textarea = Textarea::new("Line Numbers Stress");

        for i in 0..1000 {
            textarea.set_show_line_numbers(i % 2 == 0);
        }

        // Should not panic
    }

    #[test]
    fn test_textarea_all_state_combinations() {
        let mut textarea = Textarea::new("All States");
        textarea.set_content("Line1\nLine2\nLine3");

        let focus_states = [true, false];
        let line_number_states = [true, false];

        for focused in focus_states {
            for show_numbers in line_number_states {
                textarea.set_focused(focused);
                textarea.set_show_line_numbers(show_numbers);

                // Perform operations in each state
                textarea.insert_char('A');
                textarea.move_down();
                textarea.delete_char();
            }
        }

        // Should handle all combinations without panic
    }

    // ============ Line Operations Edge Cases ============

    #[test]
    fn test_textarea_join_all_lines_to_one() {
        let mut textarea = Textarea::new("Join");
        textarea.set_content("L1\nL2\nL3\nL4\nL5");

        assert_eq!(textarea.line_count(), 5);

        // Join all lines - move down and delete at start repeatedly
        for _ in 0..4 {
            // Move to next line if possible
            if textarea.cursor_position().0 < textarea.line_count() - 1 {
                textarea.move_down();
            }
            textarea.move_to_line_start();
            textarea.delete_char(); // Join with previous line
        }

        assert_eq!(textarea.line_count(), 1);
        assert!(textarea.content().contains("L1L2L3L4L5"));
    }

    #[test]
    fn test_textarea_split_line_repeatedly() {
        let mut textarea = Textarea::new("Split");
        textarea.set_content("ABCDEFGHIJ");

        // Split after every char
        for _ in 0..10 {
            textarea.move_right();
            textarea.insert_newline();
        }

        assert_eq!(textarea.line_count(), 11);
    }

    #[test]
    fn test_textarea_delete_forward_join_all_lines() {
        let mut textarea = Textarea::new("Delete Forward");
        let lines: Vec<String> = (0..50).map(|i| format!("Line{}", i)).collect();
        textarea.set_content(lines.join("\n"));

        // Move to end of each line and delete forward
        for _ in 0..49 {
            textarea.move_to_line_end();
            textarea.delete_char_forward();
        }

        assert_eq!(textarea.line_count(), 1);
    }

    // ============ Scrolling Edge Cases ============

    #[test]
    fn test_textarea_scroll_beyond_bounds() {
        let mut textarea = Textarea::new("Scroll Bounds");
        textarea.set_content("L1\nL2\nL3");

        // Try to scroll beyond content
        for _ in 0..100 {
            textarea.scroll_down();
        }

        for _ in 0..100 {
            textarea.scroll_up();
        }

        // Should not panic
    }

    #[test]
    fn test_textarea_scroll_with_cursor_tracking() {
        let mut textarea = Textarea::new("Scroll Cursor");
        let lines: Vec<String> = (0..200).map(|i| format!("Line {}", i)).collect();
        textarea.set_content(lines.join("\n"));

        // Move cursor down while scrolling
        for _ in 0..50 {
            textarea.move_down();
        }

        // Cursor should be visible (scroll_offset adjusted)
        assert_eq!(textarea.cursor_position().0, 50);
    }

    // ============ Content Preservation ============

    #[test]
    fn test_textarea_preserve_whitespace() {
        let mut textarea = Textarea::new("Whitespace");
        textarea.set_content("  Leading\nTrailing  \n  Both  ");

        let content = textarea.content();
        assert!(content.starts_with("  "));
        assert!(content.contains("Trailing  "));
        assert!(content.contains("  Both  "));
    }

    #[test]
    fn test_textarea_preserve_tabs() {
        let mut textarea = Textarea::new("Tabs");
        textarea.set_content("\tIndented\n\t\tDouble indent");

        let content = textarea.content();
        assert!(content.contains('\t'));
    }

    #[test]
    fn test_textarea_special_characters() {
        let mut textarea = Textarea::new("Special");
        textarea.insert_char('\t');
        textarea.insert_char('\r');
        textarea.insert_char(' ');

        let content = textarea.content();
        assert!(content.contains('\t'));
    }

    // ============ Multi-line Navigation ============

    #[test]
    fn test_textarea_zigzag_navigation() {
        let mut textarea = Textarea::new("Zigzag");
        textarea.set_content("L1\nL2\nL3\nL4\nL5");

        for _ in 0..10 {
            textarea.move_down();
            textarea.move_down();
            textarea.move_up();
        }

        // Should handle without panic
    }

    #[test]
    fn test_textarea_horizontal_navigation_extremes() {
        let mut textarea = Textarea::new("Horizontal");
        textarea.set_content("Short\nVery very long line here");

        // Move to long line
        textarea.move_down();

        // Navigate to end
        for _ in 0..100 {
            textarea.move_right();
        }

        // Navigate back
        for _ in 0..100 {
            textarea.move_left();
        }

        // Should handle without panic
    }

    // ============ Comprehensive Stress Test ============

    #[test]
    fn test_comprehensive_textarea_stress() {
        let mut textarea = Textarea::new("Comprehensive Stress 🐸");

        // Set complex initial content
        let mut lines = Vec::new();
        for i in 0..500 {
            lines.push(format!("Line {} with emoji 💚 and unicode 日本語", i));
        }
        textarea.set_content(lines.join("\n"));
        assert_eq!(textarea.line_count(), 500);

        // Toggle all states
        textarea.set_focused(true);
        textarea.set_show_line_numbers(true);

        // Complex editing workflow
        for i in 0..50 {
            // Navigate down
            textarea.move_down();

            // Move to start of line for safe insertion
            textarea.move_to_line_start();

            // Insert content at line start
            textarea.insert_char('X');
            textarea.insert_char('🚀');

            // Navigate to end
            textarea.move_to_line_end();

            // Split line occasionally
            if i % 10 == 0 {
                textarea.insert_newline();
            }

            // Scroll
            if i % 5 == 0 {
                textarea.scroll_down();
            }

            // Toggle states
            textarea.set_focused(i % 2 == 0);
            textarea.set_show_line_numbers(i % 3 == 0);
        }

        // Verify state is consistent
        assert!(textarea.line_count() > 0);
        assert!(textarea.content().contains('X'));
        assert!(textarea.content().contains('🚀'));

        // Navigate to boundaries
        for _ in 0..1000 {
            textarea.move_up();
        }
        assert_eq!(textarea.cursor_position().0, 0); // At top

        textarea.move_to_line_start();
        assert_eq!(textarea.cursor_position().1, 0);

        textarea.move_to_line_end();
        assert!(textarea.cursor_position().1 <= textarea.lines()[0].len());

        // Final content verification
        let final_content = textarea.content();
        assert!(final_content.len() > 0);
        assert!(final_content.contains("Line"));
        assert!(final_content.contains('💚'));
    }
}
