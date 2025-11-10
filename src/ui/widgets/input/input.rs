//! Input field widget
//!
//! Single-line text input with placeholder support
//!
//! # Architecture
//!
//! Following Elm Architecture and Separation of Concerns:
//! - **InputState**: Pure data (value, cursor) - testable, serializable
//! - **InputField**: Widget layer (state + placeholder + focus + rendering)
//!
//! This design allows:
//! - Testing state logic without UI dependencies
//! - Sharing state between widgets
//! - Serializing/deserializing input state

use crate::ui::theme::ToadTheme;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

/// Pure input state (Model in Elm Architecture)
///
/// Contains only the text content and cursor position, with no UI dependencies.
/// This allows the state to be:
/// - Tested independently without rendering infrastructure
/// - Serialized/deserialized for session persistence
/// - Shared between different UI representations
///
/// # Examples
///
/// ```
/// use toad::ui::widgets::input::InputState;
///
/// let mut state = InputState::new();
/// state.insert_char('H');
/// state.insert_char('i');
/// assert_eq!(state.value(), "Hi");
/// assert_eq!(state.cursor_position(), 2);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputState {
    /// The current input text
    value: String,
    /// Cursor position (byte index, not character index)
    cursor_position: usize,
}

impl InputState {
    /// Create a new empty input state
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::widgets::input::InputState;
    ///
    /// let state = InputState::new();
    /// assert_eq!(state.value(), "");
    /// assert_eq!(state.cursor_position(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            value: String::new(),
            cursor_position: 0,
        }
    }

    /// Create input state with initial value (cursor at end)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::widgets::input::InputState;
    ///
    /// let state = InputState::with_value("hello".to_string());
    /// assert_eq!(state.value(), "hello");
    /// assert_eq!(state.cursor_position(), 5);
    /// ```
    pub fn with_value(value: String) -> Self {
        let cursor_position = value.len();
        Self {
            value,
            cursor_position,
        }
    }

    /// Get the current text value
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::widgets::input::InputState;
    ///
    /// let state = InputState::with_value("test".to_string());
    /// assert_eq!(state.value(), "test");
    /// ```
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Get the cursor position (byte index)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::widgets::input::InputState;
    ///
    /// let mut state = InputState::new();
    /// state.insert_char('a');
    /// assert_eq!(state.cursor_position(), 1);
    /// ```
    pub fn cursor_position(&self) -> usize {
        self.cursor_position
    }

    /// Replace the current value with new text (cursor moves to end)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::widgets::input::InputState;
    ///
    /// let mut state = InputState::new();
    /// state.set_value("new text".to_string());
    /// assert_eq!(state.value(), "new text");
    /// assert_eq!(state.cursor_position(), 8);
    /// ```
    pub fn set_value(&mut self, value: String) {
        self.value = value;
        self.cursor_position = self.value.len();
    }

    /// Insert a character at the cursor position
    ///
    /// The cursor moves forward by the character's byte length.
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::widgets::input::InputState;
    ///
    /// let mut state = InputState::new();
    /// state.insert_char('a');
    /// state.insert_char('b');
    /// assert_eq!(state.value(), "ab");
    /// ```
    pub fn insert_char(&mut self, c: char) {
        self.value.insert(self.cursor_position, c);
        self.cursor_position += c.len_utf8();
    }

    /// Delete the character before the cursor (backspace behavior)
    ///
    /// If the cursor is at the start, does nothing.
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::widgets::input::InputState;
    ///
    /// let mut state = InputState::with_value("abc".to_string());
    /// state.delete_char();
    /// assert_eq!(state.value(), "ab");
    /// ```
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

    /// Move cursor one character left
    ///
    /// If already at start, does nothing.
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::widgets::input::InputState;
    ///
    /// let mut state = InputState::with_value("ab".to_string());
    /// state.move_cursor_left();
    /// assert_eq!(state.cursor_position(), 1);
    /// ```
    pub fn move_cursor_left(&mut self) {
        if self.cursor_position > 0 {
            let char_pos = self.char_position();
            if char_pos > 0 {
                self.cursor_position = self.char_to_byte_idx(char_pos - 1);
            }
        }
    }

    /// Move cursor one character right
    ///
    /// If already at end, does nothing.
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::widgets::input::InputState;
    ///
    /// let mut state = InputState::new();
    /// state.insert_char('a');
    /// state.insert_char('b');
    /// state.move_cursor_left();
    /// state.move_cursor_left();
    /// state.move_cursor_right();
    /// assert_eq!(state.cursor_position(), 1);
    /// ```
    pub fn move_cursor_right(&mut self) {
        let char_pos = self.char_position();
        let char_count = self.value.chars().count();
        if char_pos < char_count {
            self.cursor_position = self.char_to_byte_idx(char_pos + 1);
        }
    }

    /// Move cursor to the start of the text
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::widgets::input::InputState;
    ///
    /// let mut state = InputState::with_value("test".to_string());
    /// state.move_cursor_start();
    /// assert_eq!(state.cursor_position(), 0);
    /// ```
    pub fn move_cursor_start(&mut self) {
        self.cursor_position = 0;
    }

    /// Move cursor to the end of the text
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::widgets::input::InputState;
    ///
    /// let mut state = InputState::with_value("test".to_string());
    /// state.cursor_position = 0;
    /// state.move_cursor_end();
    /// assert_eq!(state.cursor_position(), 4);
    /// ```
    pub fn move_cursor_end(&mut self) {
        self.cursor_position = self.value.len();
    }

    /// Clear all text and reset cursor to start
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::widgets::input::InputState;
    ///
    /// let mut state = InputState::with_value("text".to_string());
    /// state.clear();
    /// assert_eq!(state.value(), "");
    /// assert_eq!(state.cursor_position(), 0);
    /// ```
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
}

impl Default for InputState {
    fn default() -> Self {
        Self::new()
    }
}

/// A single-line text input widget (View in Elm Architecture)
///
/// This widget wraps [`InputState`] and adds UI-specific concerns:
/// - Placeholder text
/// - Focus state
/// - Rendering with theme
///
/// # Examples
///
/// ```no_run
/// use toad::ui::widgets::input::InputField;
/// use ratatui::Frame;
/// use ratatui::layout::Rect;
///
/// let mut input = InputField::new();
/// input.set_focused(true);
/// input.insert_char('H');
/// input.insert_char('i');
/// // Then render: input.render(&mut frame, area);
/// ```
#[derive(Debug)]
pub struct InputField {
    /// Pure state (text content and cursor)
    state: InputState,
    /// Placeholder text when empty
    placeholder: String,
    /// Whether the input is focused
    is_focused: bool,
}

impl InputField {
    /// Create a new input field with default placeholder
    pub fn new() -> Self {
        Self {
            state: InputState::new(),
            placeholder: "Enter @ to mention files or / for commands".to_string(),
            is_focused: false,
        }
    }

    /// Set a custom placeholder text
    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Get the current text value (delegates to state)
    pub fn value(&self) -> &str {
        self.state.value()
    }

    /// Replace the current value (delegates to state)
    pub fn set_value(&mut self, value: String) {
        self.state.set_value(value);
    }

    /// Get focus state
    pub fn is_focused(&self) -> bool {
        self.is_focused
    }

    /// Set focus state
    pub fn set_focused(&mut self, focused: bool) {
        self.is_focused = focused;
    }

    /// Insert a character at the cursor position (delegates to state)
    pub fn insert_char(&mut self, c: char) {
        self.state.insert_char(c);
    }

    /// Delete the character before the cursor (delegates to state)
    pub fn delete_char(&mut self) {
        self.state.delete_char();
    }

    /// Move cursor left (delegates to state)
    pub fn move_cursor_left(&mut self) {
        self.state.move_cursor_left();
    }

    /// Move cursor right (delegates to state)
    pub fn move_cursor_right(&mut self) {
        self.state.move_cursor_right();
    }

    /// Move cursor to start (delegates to state)
    pub fn move_cursor_start(&mut self) {
        self.state.move_cursor_start();
    }

    /// Move cursor to end (delegates to state)
    pub fn move_cursor_end(&mut self) {
        self.state.move_cursor_end();
    }

    /// Clear the input (delegates to state)
    pub fn clear(&mut self) {
        self.state.clear();
    }

    /// Render the input field
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let value = self.state.value();
        let cursor_position = self.state.cursor_position();

        let display_text = if value.is_empty() {
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
            let before_cursor = &value[..cursor_position];
            let after_cursor = &value[cursor_position..];

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
        assert_eq!(input.state.cursor_position(), 0);
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
        assert_eq!(input.state.cursor_position(), 4); // Cursor at end
    }

    #[test]
    fn test_input_field_insert_char() {
        let mut input = InputField::new();
        input.insert_char('a');
        input.insert_char('b');
        input.insert_char('c');
        assert_eq!(input.value(), "abc");
        assert_eq!(input.state.cursor_position(), 3);
    }

    #[test]
    fn test_input_field_insert_unicode() {
        let mut input = InputField::new();
        input.insert_char('🐸');
        input.insert_char('日');
        input.insert_char('本');
        assert_eq!(input.value(), "🐸日本");
        // Cursor position is in bytes, emoji is 4 bytes, Japanese chars are 3 bytes each
        assert!(input.state.cursor_position() > 3);
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
        assert_eq!(input.state.cursor_position(), 2);

        input.move_cursor_left();
        assert_eq!(input.state.cursor_position(), 1);
    }

    #[test]
    fn test_input_field_move_cursor_left_boundary() {
        let mut input = InputField::new();
        input.set_value("a".to_string());

        input.move_cursor_left();
        assert_eq!(input.state.cursor_position(), 0);

        input.move_cursor_left(); // Should not go negative
        assert_eq!(input.state.cursor_position(), 0);
    }

    #[test]
    fn test_input_field_move_cursor_right() {
        let mut input = InputField::new();
        input.set_value("abc".to_string());
        input.state.cursor_position = 0;

        input.move_cursor_right();
        assert_eq!(input.state.cursor_position(), 1);

        input.move_cursor_right();
        assert_eq!(input.state.cursor_position(), 2);
    }

    #[test]
    fn test_input_field_move_cursor_right_boundary() {
        let mut input = InputField::new();
        input.set_value("ab".to_string());

        input.move_cursor_right(); // Already at end, should not move
        assert_eq!(input.state.cursor_position(), 2);
    }

    #[test]
    fn test_input_field_move_cursor_start() {
        let mut input = InputField::new();
        input.set_value("abc".to_string());

        input.move_cursor_start();
        assert_eq!(input.state.cursor_position(), 0);
    }

    #[test]
    fn test_input_field_move_cursor_end() {
        let mut input = InputField::new();
        input.set_value("abc".to_string());
        input.state.cursor_position = 0;

        input.move_cursor_end();
        assert_eq!(input.state.cursor_position(), 3);
    }

    #[test]
    fn test_input_field_clear() {
        let mut input = InputField::new();
        input.set_value("test".to_string());

        input.clear();
        assert_eq!(input.value(), "");
        assert_eq!(input.state.cursor_position(), 0);
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
        assert_eq!(input.state.cursor_position(), 10000);
    }

    #[test]
    fn test_input_field_insert_at_middle() {
        let mut input = InputField::new();
        input.set_value("ac".to_string());
        input.state.cursor_position = 1; // Between 'a' and 'c'

        input.insert_char('b');
        assert_eq!(input.value(), "abc");
    }

    #[test]
    fn test_input_field_delete_at_middle() {
        let mut input = InputField::new();
        input.set_value("abc".to_string());
        input.state.cursor_position = 2; // After 'b'

        input.delete_char();
        assert_eq!(input.value(), "ac");
    }

    #[test]
    fn test_input_field_cursor_movement_sequence() {
        let mut input = InputField::new();
        input.set_value("hello".to_string());

        input.move_cursor_start();
        assert_eq!(input.state.cursor_position(), 0);

        input.move_cursor_right();
        input.move_cursor_right();
        assert_eq!(input.state.cursor_position(), 2);

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
        assert_eq!(input.state.cursor_position(), 0);
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
        input.state.cursor_position = 0;
        input.move_cursor_right();
        assert_eq!(input.state.cursor_position(), 1);
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

    // ============================================================================
    // ADVANCED COMPREHENSIVE EDGE CASE TESTS (90%+ COVERAGE)
    // ============================================================================

    // ============ Stress Tests ============

    #[test]
    fn test_input_field_rapid_char_insertions() {
        let mut input = InputField::new();

        for i in 0..5000 {
            input.insert_char((b'A' + (i % 26) as u8) as char);
        }

        assert_eq!(input.value().len(), 5000);
        assert_eq!(input.state.cursor_position(), 5000);
    }

    #[test]
    fn test_input_field_rapid_deletions() {
        let mut input = InputField::new();
        input.set_value("A".repeat(3000));

        for _ in 0..3000 {
            input.delete_char();
        }

        assert_eq!(input.value(), "");
        assert_eq!(input.state.cursor_position(), 0);
    }

    #[test]
    fn test_input_field_rapid_cursor_movements() {
        let mut input = InputField::new();
        input.set_value("X".repeat(100));

        for _ in 0..1000 {
            input.move_cursor_left();
        }

        assert_eq!(input.state.cursor_position(), 0);

        for _ in 0..1000 {
            input.move_cursor_right();
        }

        assert_eq!(input.state.cursor_position(), 100);
    }

    #[test]
    fn test_input_field_alternating_insert_delete() {
        let mut input = InputField::new();

        for _ in 0..1000 {
            input.insert_char('A');
            input.delete_char();
        }

        assert_eq!(input.value(), "");
    }

    #[test]
    fn test_input_field_rapid_clear_refill() {
        let mut input = InputField::new();

        for i in 0..500 {
            input.set_value(format!("Text {}", i));
            input.clear();
        }

        assert_eq!(input.value(), "");
    }

    // ============ Unicode Edge Cases ============

    #[test]
    fn test_input_field_rtl_text() {
        let mut input = InputField::new();
        input.set_value("مرحبا Hello שלום".to_string());

        assert!(input.value().contains("مرحبا"));
        assert!(input.value().contains("שלום"));
    }

    #[test]
    fn test_input_field_mixed_scripts() {
        let mut input = InputField::new();
        input.insert_char('H');
        input.insert_char('e');
        input.insert_char('l');
        input.insert_char('l');
        input.insert_char('o');
        input.insert_char('🐸');
        input.insert_char('日');
        input.insert_char('本');
        input.insert_char('語');

        assert_eq!(input.value(), "Hello🐸日本語");
    }

    #[test]
    fn test_input_field_combining_characters() {
        let mut input = InputField::new();
        input.insert_char('e');
        input.insert_char('\u{0301}'); // Combining acute accent

        assert!(input.value().len() > 1);
    }

    #[test]
    fn test_input_field_zero_width_characters() {
        let mut input = InputField::new();
        input.insert_char('A');
        input.insert_char('\u{200B}'); // Zero-width space
        input.insert_char('B');

        assert_eq!(input.value(), "A\u{200B}B");
    }

    #[test]
    fn test_input_field_emoji_variations() {
        let emojis = ['🐸', '💚', '🚀', '✨', '🔥', '🌟', '🎯', '💎'];
        let mut input = InputField::new();

        for emoji in emojis {
            input.insert_char(emoji);
        }

        for emoji in emojis {
            assert!(input.value().contains(emoji));
        }
    }

    // ============ Extreme Input Sizes ============

    #[test]
    fn test_input_field_100k_characters() {
        let mut input = InputField::new();
        let huge_text = "X".repeat(100000);
        input.set_value(huge_text.clone());

        assert_eq!(input.value().len(), 100000);
        assert_eq!(input.state.cursor_position(), 100000);
    }

    #[test]
    fn test_input_field_single_unicode_char_100_times() {
        let mut input = InputField::new();

        for _ in 0..100 {
            input.insert_char('🐸');
        }

        // Each emoji is 4 bytes
        assert_eq!(input.value().len(), 400);
    }

    // ============ Complex Editing Workflows ============

    #[test]
    fn test_input_field_insert_middle_delete_workflow() {
        let mut input = InputField::new();
        input.set_value("HelloWorld".to_string());

        // Move to middle
        input.move_cursor_start();
        for _ in 0..5 {
            input.move_cursor_right();
        }

        // Insert space
        input.insert_char(' ');
        assert_eq!(input.value(), "Hello World");

        // Delete space
        input.delete_char();
        assert_eq!(input.value(), "HelloWorld");
    }

    #[test]
    fn test_input_field_zigzag_cursor_pattern() {
        let mut input = InputField::new();
        input.set_value("ABCDEFGHIJ".to_string());

        for _ in 0..100 {
            input.move_cursor_right();
            input.move_cursor_right();
            input.move_cursor_left();
        }

        // Should handle without panic
    }

    #[test]
    fn test_input_field_boundary_navigation() {
        let mut input = InputField::new();
        input.set_value("Test".to_string());

        // Move to boundaries repeatedly
        for _ in 0..50 {
            input.move_cursor_start();
            assert_eq!(input.state.cursor_position(), 0);

            input.move_cursor_end();
            assert_eq!(input.state.cursor_position(), 4);
        }
    }

    // ============ Placeholder Edge Cases ============

    #[test]
    fn test_input_field_very_long_placeholder() {
        let long_placeholder = "Placeholder ".repeat(1000);
        let input = InputField::new().with_placeholder(long_placeholder.clone());

        assert_eq!(input.placeholder.len(), long_placeholder.len());
    }

    #[test]
    fn test_input_field_unicode_placeholder() {
        let input = InputField::new().with_placeholder("Enter 🐸 to continue");

        assert!(input.placeholder.contains("🐸"));
    }

    #[test]
    fn test_input_field_empty_placeholder() {
        let input = InputField::new().with_placeholder("");

        assert_eq!(input.placeholder, "");
    }

    // ============ Focus State Transitions ============

    #[test]
    fn test_input_field_rapid_focus_toggle() {
        let mut input = InputField::new();

        for i in 0..1000 {
            input.set_focused(i % 2 == 0);
        }

        assert!(!input.is_focused()); // Last toggle (i=999) sets to false
    }

    #[test]
    fn test_input_field_focus_with_editing() {
        let mut input = InputField::new();

        for i in 0..100 {
            input.set_focused(i % 2 == 0);
            input.insert_char('A');
        }

        assert_eq!(input.value().len(), 100);
    }

    // ============ Cursor Position Edge Cases ============

    #[test]
    fn test_input_field_cursor_at_all_positions() {
        let mut input = InputField::new();
        input.set_value("ABCDEFGHIJ".to_string());

        // Test cursor at each position
        input.move_cursor_start();
        for i in 0..=10 {
            assert_eq!(input.state.cursor_position(), i);
            if i < 10 {
                input.move_cursor_right();
            }
        }
    }

    #[test]
    fn test_input_field_cursor_beyond_text_length() {
        let mut input = InputField::new();
        input.set_value("Short".to_string());

        // Try to move cursor beyond text
        for _ in 0..100 {
            input.move_cursor_right();
        }

        assert_eq!(input.state.cursor_position(), 5); // Stays at end
    }

    #[test]
    fn test_input_field_cursor_before_text_start() {
        let mut input = InputField::new();
        input.set_value("Text".to_string());
        input.move_cursor_start();

        // Try to move cursor before start
        for _ in 0..100 {
            input.move_cursor_left();
        }

        assert_eq!(input.state.cursor_position(), 0); // Stays at start
    }

    // ============ Delete Edge Cases ============

    #[test]
    fn test_input_field_delete_at_start() {
        let mut input = InputField::new();
        input.set_value("Test".to_string());
        input.move_cursor_start();

        input.delete_char();
        assert_eq!(input.value(), "Test"); // Nothing to delete
    }

    #[test]
    fn test_input_field_delete_all_one_by_one() {
        let mut input = InputField::new();
        input.set_value("ABCDEFGHIJ".to_string());

        for _ in 0..10 {
            input.delete_char();
        }

        assert_eq!(input.value(), "");
        assert_eq!(input.state.cursor_position(), 0);
    }

    #[test]
    fn test_input_field_delete_from_middle() {
        let mut input = InputField::new();
        input.set_value("ABCDEFGH".to_string());
        input.state.cursor_position = 4; // After 'D'

        for _ in 0..4 {
            input.delete_char();
        }

        assert_eq!(input.value(), "EFGH");
        assert_eq!(input.state.cursor_position(), 0);
    }

    // ============ Insert Edge Cases ============

    #[test]
    fn test_input_field_insert_at_start() {
        let mut input = InputField::new();
        input.set_value("World".to_string());
        input.move_cursor_start();

        input.insert_char('H');
        input.insert_char('e');
        input.insert_char('l');
        input.insert_char('l');
        input.insert_char('o');

        assert_eq!(input.value(), "HelloWorld");
    }

    #[test]
    fn test_input_field_insert_at_end() {
        let mut input = InputField::new();
        input.set_value("Hello".to_string());

        input.insert_char(' ');
        input.insert_char('W');
        input.insert_char('o');
        input.insert_char('r');
        input.insert_char('l');
        input.insert_char('d');

        assert_eq!(input.value(), "Hello World");
    }

    #[test]
    fn test_input_field_insert_all_ascii() {
        let mut input = InputField::new();

        for i in 32..=126 {
            input.insert_char(i as u8 as char);
        }

        assert_eq!(input.value().len(), 95); // All printable ASCII
    }

    // ============ Clear Edge Cases ============

    #[test]
    fn test_input_field_clear_when_empty() {
        let mut input = InputField::new();
        input.clear();

        assert_eq!(input.value(), "");
        assert_eq!(input.state.cursor_position(), 0);
    }

    #[test]
    fn test_input_field_clear_preserves_placeholder() {
        let mut input = InputField::new().with_placeholder("Test placeholder");
        input.set_value("Some text".to_string());
        input.clear();

        assert_eq!(input.value(), "");
        assert_eq!(input.placeholder, "Test placeholder");
    }

    #[test]
    fn test_input_field_clear_resets_cursor() {
        let mut input = InputField::new();
        input.set_value("Long text here".to_string());
        input.state.cursor_position = 7;

        input.clear();
        assert_eq!(input.state.cursor_position(), 0);
    }

    // ============ Value Setter Edge Cases ============

    #[test]
    fn test_input_field_set_value_replaces_existing() {
        let mut input = InputField::new();
        input.set_value("First".to_string());
        input.set_value("Second".to_string());

        assert_eq!(input.value(), "Second");
    }

    #[test]
    fn test_input_field_set_value_empty() {
        let mut input = InputField::new();
        input.set_value("Text".to_string());
        input.set_value(String::new());

        assert_eq!(input.value(), "");
        assert_eq!(input.state.cursor_position(), 0);
    }

    // ============ Trait Coverage ============

    #[test]
    fn test_input_field_debug() {
        let input = InputField::new();
        let debug_str = format!("{:?}", input);

        assert!(debug_str.contains("InputField"));
    }

    // ============ Comprehensive Stress Test ============

    #[test]
    fn test_comprehensive_input_stress() {
        let mut input = InputField::new().with_placeholder("Enter text 🐸");

        // Complex editing workflow
        for i in 0..100 {
            // Insert varied content
            match i % 4 {
                0 => input.insert_char('A'),
                1 => input.insert_char('🚀'),
                2 => input.insert_char('日'),
                _ => input.insert_char(' '),
            }
        }

        assert_eq!(input.value().len(), 225); // Mixed byte lengths: A(1)*25 + 🚀(4)*25 + 日(3)*25 + ' '(1)*25 = 225

        // Navigate and edit
        input.move_cursor_start();
        for _ in 0..50 {
            input.move_cursor_right();
        }

        // Insert at middle
        input.insert_char('X');

        // Delete some chars
        for _ in 0..10 {
            input.delete_char();
        }

        // Navigate to extremes
        input.move_cursor_end();
        input.move_cursor_start();

        // Toggle focus
        for i in 0..20 {
            input.set_focused(i % 2 == 0);
        }

        // Clear and refill
        input.clear();
        assert_eq!(input.value(), "");

        input.set_value("Final test 💚 مرحبا שלום".to_string());
        assert!(input.value().contains("💚"));
        assert!(input.value().contains("مرحبا"));
        assert!(input.value().contains("שלום"));

        // Navigate through unicode
        input.move_cursor_start();
        for _ in 0..20 {
            input.move_cursor_right();
        }

        // Verify state is consistent
        assert!(input.state.cursor_position() <= input.value().len());
    }
}
