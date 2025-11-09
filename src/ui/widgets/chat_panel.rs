//! Chat Panel widget for AI conversational interaction
//!
//! Displays a scrollable chat history with user messages and AI responses,
//! supporting markdown rendering, code blocks, and streaming responses.

use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{
        Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, Wrap,
    },
};
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Role of a message in the conversation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageRole {
    /// User message
    User,
    /// AI assistant message
    Assistant,
    /// System message or error
    System,
}

/// A single message in the chat
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    /// Message role (user, assistant, system)
    pub role: MessageRole,
    /// Message content
    pub content: String,
    /// Timestamp (seconds since epoch)
    pub timestamp: u64,
    /// Whether this message is currently streaming
    pub streaming: bool,
    /// Whether this message contains code
    pub has_code: bool,
}

impl ChatMessage {
    /// Create a new chat message
    pub fn new(role: MessageRole, content: impl Into<String>) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_else(|_| Duration::ZERO)
            .as_secs();

        let content = content.into();
        let has_code = content.contains("```");

        Self {
            role,
            content,
            timestamp: now,
            streaming: false,
            has_code,
        }
    }

    /// Create a streaming message (incomplete)
    pub fn streaming(role: MessageRole, content: impl Into<String>) -> Self {
        let mut msg = Self::new(role, content);
        msg.streaming = true;
        msg
    }

    /// Append content to a streaming message
    pub fn append(&mut self, text: &str) {
        self.content.push_str(text);
        self.has_code = self.content.contains("```");
    }

    /// Finalize a streaming message
    pub fn finish_streaming(&mut self) {
        self.streaming = false;
    }

    /// Get formatted timestamp
    pub fn formatted_time(&self) -> String {
        // Convert timestamp to HH:MM format
        let secs = self.timestamp % 86400; // seconds in a day
        let hours = secs / 3600;
        let minutes = (secs % 3600) / 60;
        format!("{:02}:{:02}", hours, minutes)
    }
}

/// Chat panel widget
pub struct ChatPanel {
    /// Chat messages
    messages: Vec<ChatMessage>,
    /// Scroll offset (number of lines scrolled from bottom)
    scroll_offset: usize,
    /// Auto-scroll to bottom on new messages
    auto_scroll: bool,
    /// Show timestamps
    show_timestamps: bool,
    /// Maximum messages to keep in history
    max_history: usize,
    /// User color
    user_color: Color,
    /// Assistant color
    assistant_color: Color,
    /// System color
    system_color: Color,
}

impl Default for ChatPanel {
    fn default() -> Self {
        Self::new()
    }
}

impl ChatPanel {
    /// Create a new chat panel
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
            scroll_offset: 0,
            auto_scroll: true,
            show_timestamps: true,
            max_history: 1000,
            user_color: Color::Cyan,
            assistant_color: Color::Green,
            system_color: Color::Yellow,
        }
    }

    /// Add a message to the chat
    pub fn add_message(&mut self, message: ChatMessage) {
        self.messages.push(message);

        // Trim history if too long
        if self.messages.len() > self.max_history {
            self.messages
                .drain(0..self.messages.len() - self.max_history);
        }

        // Auto-scroll to bottom
        if self.auto_scroll {
            self.scroll_offset = 0;
        }
    }

    /// Add a user message
    pub fn add_user_message(&mut self, content: impl Into<String>) {
        self.add_message(ChatMessage::new(MessageRole::User, content));
    }

    /// Add an assistant message
    pub fn add_assistant_message(&mut self, content: impl Into<String>) {
        self.add_message(ChatMessage::new(MessageRole::Assistant, content));
    }

    /// Add a system message
    pub fn add_system_message(&mut self, content: impl Into<String>) {
        self.add_message(ChatMessage::new(MessageRole::System, content));
    }

    /// Start streaming an assistant message
    pub fn start_streaming(&mut self) -> usize {
        let msg = ChatMessage::streaming(MessageRole::Assistant, "");
        self.messages.push(msg);
        self.messages.len() - 1
    }

    /// Append to the last streaming message
    pub fn append_streaming(&mut self, text: &str) {
        if let Some(last) = self.messages.last_mut()
            && last.streaming {
                last.append(text);
            }
    }

    /// Finish the current streaming message
    pub fn finish_streaming(&mut self) {
        if let Some(last) = self.messages.last_mut() {
            last.finish_streaming();
        }
    }

    /// Scroll up
    pub fn scroll_up(&mut self, lines: usize) {
        self.scroll_offset = self.scroll_offset.saturating_add(lines);
        self.auto_scroll = false;
    }

    /// Scroll down
    pub fn scroll_down(&mut self, lines: usize) {
        self.scroll_offset = self.scroll_offset.saturating_sub(lines);
        if self.scroll_offset == 0 {
            self.auto_scroll = true;
        }
    }

    /// Scroll to top
    pub fn scroll_to_top(&mut self) {
        self.scroll_offset = usize::MAX;
        self.auto_scroll = false;
    }

    /// Scroll to bottom
    pub fn scroll_to_bottom(&mut self) {
        self.scroll_offset = 0;
        self.auto_scroll = true;
    }

    /// Toggle auto-scroll
    pub fn toggle_auto_scroll(&mut self) {
        self.auto_scroll = !self.auto_scroll;
    }

    /// Toggle timestamps
    pub fn toggle_timestamps(&mut self) {
        self.show_timestamps = !self.show_timestamps;
    }

    /// Clear all messages
    pub fn clear(&mut self) {
        self.messages.clear();
        self.scroll_offset = 0;
    }

    /// Get number of messages
    pub fn message_count(&self) -> usize {
        self.messages.len()
    }

    /// Check if auto-scrolling
    pub fn is_auto_scrolling(&self) -> bool {
        self.auto_scroll
    }

    /// Set user color
    pub fn with_user_color(mut self, color: Color) -> Self {
        self.user_color = color;
        self
    }

    /// Set assistant color
    pub fn with_assistant_color(mut self, color: Color) -> Self {
        self.assistant_color = color;
        self
    }

    /// Set system color
    pub fn with_system_color(mut self, color: Color) -> Self {
        self.system_color = color;
        self
    }

    /// Set max history
    pub fn with_max_history(mut self, max: usize) -> Self {
        self.max_history = max;
        self
    }

    /// Render the chat panel
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        // Create block with border
        let block = Block::default()
            .borders(Borders::ALL)
            .title(" Chat ")
            .style(Style::default().fg(Color::White));

        let inner_area = block.inner(area);
        frame.render_widget(block, area);

        // Build chat content
        let mut lines = Vec::new();

        for msg in &self.messages {
            // Add message header (role + timestamp)
            let role_str = match msg.role {
                MessageRole::User => "You",
                MessageRole::Assistant => "Assistant",
                MessageRole::System => "System",
            };

            let color = match msg.role {
                MessageRole::User => self.user_color,
                MessageRole::Assistant => self.assistant_color,
                MessageRole::System => self.system_color,
            };

            let mut header_spans = vec![Span::styled(
                role_str,
                Style::default().fg(color).add_modifier(Modifier::BOLD),
            )];

            if self.show_timestamps {
                header_spans.push(Span::raw(" "));
                header_spans.push(Span::styled(
                    format!("[{}]", msg.formatted_time()),
                    Style::default().fg(Color::DarkGray),
                ));
            }

            if msg.streaming {
                header_spans.push(Span::raw(" "));
                header_spans.push(Span::styled("●", Style::default().fg(Color::Green)));
            }

            lines.push(Line::from(header_spans));

            // Add message content (word-wrapped)
            let content_lines: Vec<&str> = msg.content.lines().collect();
            for line in content_lines {
                if line.starts_with("```") {
                    // Code block delimiter
                    lines.push(Line::from(Span::styled(
                        line,
                        Style::default().fg(Color::DarkGray),
                    )));
                } else if msg.has_code && msg.content.contains("```") {
                    // Inside code block - use monospace styling
                    lines.push(Line::from(Span::styled(
                        format!("  {}", line),
                        Style::default().fg(Color::Cyan).add_modifier(Modifier::DIM),
                    )));
                } else {
                    // Regular text
                    lines.push(Line::from(Span::raw(format!("  {}", line))));
                }
            }

            // Add spacing between messages
            lines.push(Line::from(""));
        }

        // Create paragraph with scrolling
        let text = Text::from(lines);
        let paragraph = Paragraph::new(text)
            .wrap(Wrap { trim: false })
            .scroll((self.scroll_offset as u16, 0));

        frame.render_widget(paragraph, inner_area);

        // Render scrollbar if needed
        if self.messages.len() > inner_area.height as usize {
            let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓"));

            let mut scrollbar_state =
                ScrollbarState::new(self.messages.len()).position(self.scroll_offset);

            let scrollbar_area = Rect {
                x: area.x + area.width - 1,
                y: area.y + 1,
                width: 1,
                height: area.height - 2,
            };

            frame.render_stateful_widget(scrollbar, scrollbar_area, &mut scrollbar_state);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chat_message_creation() {
        let msg = ChatMessage::new(MessageRole::User, "Hello");
        assert_eq!(msg.role, MessageRole::User);
        assert_eq!(msg.content, "Hello");
        assert!(!msg.streaming);
        assert!(!msg.has_code);
    }

    #[test]
    fn test_chat_message_with_code() {
        let msg = ChatMessage::new(
            MessageRole::Assistant,
            "Here's code:\n```rust\nfn main() {}\n```",
        );
        assert!(msg.has_code);
    }

    #[test]
    fn test_streaming_message() {
        let mut msg = ChatMessage::streaming(MessageRole::Assistant, "Hello");
        assert!(msg.streaming);

        msg.append(" world");
        assert_eq!(msg.content, "Hello world");

        msg.finish_streaming();
        assert!(!msg.streaming);
    }

    #[test]
    fn test_chat_panel() {
        let mut panel = ChatPanel::new();
        assert_eq!(panel.message_count(), 0);

        panel.add_user_message("Test");
        assert_eq!(panel.message_count(), 1);

        panel.add_assistant_message("Response");
        assert_eq!(panel.message_count(), 2);

        panel.clear();
        assert_eq!(panel.message_count(), 0);
    }

    #[test]
    fn test_chat_panel_streaming() {
        let mut panel = ChatPanel::new();

        panel.start_streaming();
        panel.append_streaming("Hello");
        panel.append_streaming(" world");
        panel.finish_streaming();

        assert_eq!(panel.message_count(), 1);
        assert_eq!(panel.messages[0].content, "Hello world");
        assert!(!panel.messages[0].streaming);
    }

    #[test]
    fn test_chat_panel_scrolling() {
        let mut panel = ChatPanel::new();
        assert!(panel.is_auto_scrolling());

        panel.scroll_up(5);
        assert_eq!(panel.scroll_offset, 5);
        assert!(!panel.is_auto_scrolling());

        panel.scroll_down(3);
        assert_eq!(panel.scroll_offset, 2);

        panel.scroll_to_bottom();
        assert_eq!(panel.scroll_offset, 0);
        assert!(panel.is_auto_scrolling());
    }

    #[test]
    fn test_max_history() {
        let mut panel = ChatPanel::new().with_max_history(3);

        panel.add_user_message("1");
        panel.add_user_message("2");
        panel.add_user_message("3");
        panel.add_user_message("4");

        assert_eq!(panel.message_count(), 3);
        assert_eq!(panel.messages[0].content, "2");
    }

    // ============================================================================
    // COMPREHENSIVE EDGE CASE TESTS (ADVANCED TIER - Advanced Input)
    // ============================================================================

    // ============ Stress Tests ============

    #[test]
    fn test_panel_1000_messages() {
        let mut panel = ChatPanel::new();
        for i in 0..1000 {
            panel.add_user_message(format!("Message {}", i));
        }
        assert_eq!(panel.message_count(), 1000);
    }

    #[test]
    fn test_panel_10000_streaming_appends() {
        let mut panel = ChatPanel::new();
        panel.start_streaming();
        for _ in 0..10000 {
            panel.append_streaming("x");
        }
        panel.finish_streaming();
        assert_eq!(panel.messages[0].content.len(), 10000);
    }

    #[test]
    fn test_panel_rapid_scroll_1000_operations() {
        let mut panel = ChatPanel::new();
        for _ in 0..100 {
            panel.add_user_message("Test");
        }
        for _ in 0..1000 {
            panel.scroll_up(1);
            panel.scroll_down(1);
        }
        // Should handle rapid scrolling without panics
        assert_eq!(panel.scroll_offset, 0);
    }

    #[test]
    fn test_panel_rapid_color_changes_1000() {
        let mut panel = ChatPanel::new();
        for i in 0..1000 {
            panel = panel.with_user_color(Color::Rgb(i as u8, 0, 0));
        }
        // Should handle rapid color changes
        assert_eq!(panel.message_count(), 0);
    }

    // ============ Unicode Edge Cases ============

    #[test]
    fn test_message_with_emoji() {
        let msg = ChatMessage::new(MessageRole::User, "Hello 🚀 World 🐸");
        assert!(msg.content.contains('🚀'));
        assert!(msg.content.contains('🐸'));
    }

    #[test]
    fn test_message_rtl_arabic() {
        let msg = ChatMessage::new(MessageRole::User, "مرحبا بك في العالم");
        assert!(msg.content.len() > 0);
    }

    #[test]
    fn test_message_rtl_hebrew() {
        let msg = ChatMessage::new(MessageRole::User, "שלום עולם");
        assert!(msg.content.len() > 0);
    }

    #[test]
    fn test_message_japanese() {
        let msg = ChatMessage::new(MessageRole::User, "こんにちは世界");
        assert!(msg.content.len() > 0);
    }

    #[test]
    fn test_message_mixed_scripts() {
        let msg = ChatMessage::new(
            MessageRole::Assistant,
            "Hello مرحبا שלום こんにちは 🚀",
        );
        assert!(msg.content.len() > 0);
    }

    #[test]
    fn test_message_combining_characters() {
        let msg = ChatMessage::new(MessageRole::User, "e\u{0301}"); // é with combining accent
        assert!(msg.content.len() > 0);
    }

    #[test]
    fn test_message_zero_width_characters() {
        let msg = ChatMessage::new(MessageRole::User, "Test\u{200B}Zero\u{200B}Width");
        assert!(msg.content.contains('\u{200B}'));
    }

    #[test]
    fn test_panel_very_long_unicode_message() {
        let mut panel = ChatPanel::new();
        let long_msg = "🚀".repeat(10000);
        panel.add_user_message(long_msg.clone());
        assert_eq!(panel.messages[0].content, long_msg);
    }

    // ============ Extreme Values ============

    #[test]
    fn test_panel_max_history_usize_max() {
        let panel = ChatPanel::new().with_max_history(usize::MAX);
        assert_eq!(panel.max_history, usize::MAX);
    }

    #[test]
    fn test_panel_max_history_zero() {
        let mut panel = ChatPanel::new().with_max_history(0);
        panel.add_user_message("Test");
        // Should trim immediately
        assert_eq!(panel.message_count(), 0);
    }

    #[test]
    fn test_message_timestamp_zero() {
        let mut msg = ChatMessage::new(MessageRole::User, "Test");
        msg.timestamp = 0;
        let formatted = msg.formatted_time();
        assert_eq!(formatted, "00:00");
    }

    #[test]
    fn test_message_timestamp_max() {
        let mut msg = ChatMessage::new(MessageRole::User, "Test");
        msg.timestamp = u64::MAX;
        // Should not panic on formatting
        let _ = msg.formatted_time();
    }

    #[test]
    fn test_message_very_large_content() {
        let large_content = "A".repeat(100000);
        let msg = ChatMessage::new(MessageRole::Assistant, large_content.clone());
        assert_eq!(msg.content.len(), 100000);
    }

    #[test]
    fn test_scroll_offset_usize_max() {
        let mut panel = ChatPanel::new();
        panel.scroll_to_top();
        assert_eq!(panel.scroll_offset, usize::MAX);
    }

    // ============ Streaming Edge Cases ============

    #[test]
    fn test_finish_streaming_without_start() {
        let mut panel = ChatPanel::new();
        panel.add_user_message("Normal message");
        panel.finish_streaming();
        // Should not panic
        assert!(!panel.messages[0].streaming);
    }

    #[test]
    fn test_append_streaming_to_empty_panel() {
        let mut panel = ChatPanel::new();
        panel.append_streaming("Test");
        // Should not panic when no messages exist
        assert_eq!(panel.message_count(), 0);
    }

    #[test]
    fn test_append_streaming_to_non_streaming_message() {
        let mut panel = ChatPanel::new();
        panel.add_user_message("Normal");
        panel.append_streaming("More");
        // Should not append to non-streaming message
        assert_eq!(panel.messages[0].content, "Normal");
    }

    #[test]
    fn test_streaming_empty_message() {
        let mut panel = ChatPanel::new();
        panel.start_streaming();
        panel.finish_streaming();
        assert_eq!(panel.messages[0].content, "");
        assert!(!panel.messages[0].streaming);
    }

    #[test]
    fn test_streaming_code_detection() {
        let mut panel = ChatPanel::new();
        panel.start_streaming();
        panel.append_streaming("Here's code:\n```rust\n");
        assert!(panel.messages[0].has_code);
        panel.append_streaming("fn main() {}\n```");
        panel.finish_streaming();
        assert!(panel.messages[0].has_code);
    }

    // ============ Scroll Edge Cases ============

    #[test]
    fn test_scroll_empty_panel() {
        let mut panel = ChatPanel::new();
        panel.scroll_up(10);
        assert_eq!(panel.scroll_offset, 10);
        panel.scroll_down(5);
        assert_eq!(panel.scroll_offset, 5);
    }

    #[test]
    fn test_scroll_saturating_add() {
        let mut panel = ChatPanel::new();
        panel.scroll_offset = usize::MAX - 5;
        panel.scroll_up(10);
        assert_eq!(panel.scroll_offset, usize::MAX);
    }

    #[test]
    fn test_scroll_saturating_sub() {
        let mut panel = ChatPanel::new();
        panel.scroll_offset = 5;
        panel.scroll_down(10);
        assert_eq!(panel.scroll_offset, 0);
    }

    #[test]
    fn test_auto_scroll_disabled_on_scroll_up() {
        let mut panel = ChatPanel::new();
        assert!(panel.is_auto_scrolling());
        panel.scroll_up(1);
        assert!(!panel.is_auto_scrolling());
    }

    #[test]
    fn test_auto_scroll_enabled_on_scroll_to_bottom() {
        let mut panel = ChatPanel::new();
        panel.scroll_up(10);
        assert!(!panel.is_auto_scrolling());
        panel.scroll_to_bottom();
        assert!(panel.is_auto_scrolling());
    }

    #[test]
    fn test_auto_scroll_toggle() {
        let mut panel = ChatPanel::new();
        assert!(panel.is_auto_scrolling());
        panel.toggle_auto_scroll();
        assert!(!panel.is_auto_scrolling());
        panel.toggle_auto_scroll();
        assert!(panel.is_auto_scrolling());
    }

    // ============ Message Role Edge Cases ============

    #[test]
    fn test_all_message_roles() {
        let user_msg = ChatMessage::new(MessageRole::User, "User message");
        let assistant_msg = ChatMessage::new(MessageRole::Assistant, "Assistant message");
        let system_msg = ChatMessage::new(MessageRole::System, "System message");

        assert_eq!(user_msg.role, MessageRole::User);
        assert_eq!(assistant_msg.role, MessageRole::Assistant);
        assert_eq!(system_msg.role, MessageRole::System);
    }

    #[test]
    fn test_custom_role_colors() {
        let panel = ChatPanel::new()
            .with_user_color(Color::Red)
            .with_assistant_color(Color::Blue)
            .with_system_color(Color::Magenta);

        assert_eq!(panel.user_color, Color::Red);
        assert_eq!(panel.assistant_color, Color::Blue);
        assert_eq!(panel.system_color, Color::Magenta);
    }

    #[test]
    fn test_message_role_equality() {
        assert_eq!(MessageRole::User, MessageRole::User);
        assert_eq!(MessageRole::Assistant, MessageRole::Assistant);
        assert_eq!(MessageRole::System, MessageRole::System);
        assert_ne!(MessageRole::User, MessageRole::Assistant);
    }

    // ============ Trait Coverage ============

    #[test]
    fn test_message_role_clone() {
        let role = MessageRole::User;
        let cloned = role.clone();
        assert_eq!(role, cloned);
    }

    #[test]
    fn test_message_role_copy() {
        let role = MessageRole::Assistant;
        let copied = role;
        assert_eq!(role, copied);
    }

    #[test]
    fn test_message_role_debug() {
        let role = MessageRole::System;
        let debug_str = format!("{:?}", role);
        assert!(debug_str.contains("System"));
    }

    #[test]
    fn test_message_clone() {
        let msg = ChatMessage::new(MessageRole::User, "Test");
        let cloned = msg.clone();
        assert_eq!(msg.content, cloned.content);
        assert_eq!(msg.role, cloned.role);
    }

    #[test]
    fn test_message_debug() {
        let msg = ChatMessage::new(MessageRole::User, "Test");
        let debug_str = format!("{:?}", msg);
        assert!(debug_str.contains("ChatMessage"));
    }

    #[test]
    fn test_panel_default() {
        let panel = ChatPanel::default();
        assert_eq!(panel.message_count(), 0);
        assert!(panel.is_auto_scrolling());
        assert_eq!(panel.max_history, 1000);
    }

    #[test]
    fn test_message_role_serialize() {
        let role = MessageRole::User;
        let json = serde_json::to_string(&role).unwrap();
        assert!(json.contains("User"));
    }

    #[test]
    fn test_message_role_deserialize() {
        let json = "\"Assistant\"";
        let role: MessageRole = serde_json::from_str(json).unwrap();
        assert_eq!(role, MessageRole::Assistant);
    }

    #[test]
    fn test_message_serialize() {
        let msg = ChatMessage::new(MessageRole::User, "Test");
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("Test"));
        assert!(json.contains("User"));
    }

    #[test]
    fn test_message_deserialize() {
        let msg = ChatMessage::new(MessageRole::User, "Test");
        let json = serde_json::to_string(&msg).unwrap();
        let deserialized: ChatMessage = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.content, "Test");
        assert_eq!(deserialized.role, MessageRole::User);
    }

    // ============ Complex Workflows ============

    #[test]
    fn test_add_messages_while_scrolled() {
        let mut panel = ChatPanel::new();
        panel.add_user_message("Message 1");
        panel.scroll_up(5);
        assert!(!panel.is_auto_scrolling());

        panel.add_assistant_message("Message 2");
        // Should not auto-scroll since disabled
        assert_eq!(panel.scroll_offset, 5);
        assert_eq!(panel.message_count(), 2);
    }

    #[test]
    fn test_stream_while_scrolled() {
        let mut panel = ChatPanel::new();
        panel.scroll_up(10);
        panel.start_streaming();
        panel.append_streaming("Streaming content");
        panel.finish_streaming();

        assert_eq!(panel.message_count(), 1);
        assert_eq!(panel.scroll_offset, 10);
    }

    #[test]
    fn test_toggle_timestamps_with_messages() {
        let mut panel = ChatPanel::new();
        panel.add_user_message("Test");
        assert!(panel.show_timestamps);

        panel.toggle_timestamps();
        assert!(!panel.show_timestamps);

        panel.toggle_timestamps();
        assert!(panel.show_timestamps);
    }

    #[test]
    fn test_history_trimming_with_many_messages() {
        let mut panel = ChatPanel::new().with_max_history(2);
        panel.add_user_message("Message 1");
        panel.add_user_message("Message 2");
        panel.add_user_message("Message 3");
        panel.add_assistant_message("Message 4");

        // Should only keep last 2 messages
        assert_eq!(panel.message_count(), 2);
        assert_eq!(panel.messages[0].content, "Message 3");
        assert_eq!(panel.messages[1].content, "Message 4");
    }

    // ============ Comprehensive Stress Test ============

    #[test]
    fn test_comprehensive_chat_panel_stress() {
        let mut panel = ChatPanel::new()
            .with_max_history(100)
            .with_user_color(Color::Cyan)
            .with_assistant_color(Color::Green)
            .with_system_color(Color::Yellow);

        // Phase 1: Add 50 messages with varied content
        for i in 0..50 {
            match i % 4 {
                0 => panel.add_user_message(format!("ASCII Message {}", i)),
                1 => panel.add_assistant_message(format!("🚀 Emoji Message {}", i)),
                2 => panel.add_system_message(format!("日本語 Message {}", i)),
                _ => panel.add_user_message(format!("مرحبا Message {}", i)),
            }
        }
        assert_eq!(panel.message_count(), 50);

        // Phase 2: Scroll operations
        panel.scroll_up(20);
        assert_eq!(panel.scroll_offset, 20);
        panel.scroll_down(10);
        assert_eq!(panel.scroll_offset, 10);
        panel.scroll_to_top();
        assert_eq!(panel.scroll_offset, usize::MAX);
        panel.scroll_to_bottom();
        assert_eq!(panel.scroll_offset, 0);

        // Phase 3: Streaming with code
        panel.start_streaming();
        panel.append_streaming("Here's some code:\n```rust\n");
        panel.append_streaming("fn main() {\n");
        panel.append_streaming("    println!(\"Hello\");\n");
        panel.append_streaming("}\n```");
        panel.finish_streaming();
        assert!(panel.messages.last().unwrap().has_code);

        // Phase 4: Toggle features
        panel.toggle_timestamps();
        assert!(!panel.show_timestamps);
        panel.toggle_auto_scroll();
        assert!(!panel.is_auto_scrolling());

        // Phase 5: Add 60 more messages to trigger history trimming
        for i in 50..110 {
            panel.add_user_message(format!("Extra {}", i));
        }
        assert_eq!(panel.message_count(), 100); // max_history limit

        // Phase 6: Clear and verify empty state
        panel.clear();
        assert_eq!(panel.message_count(), 0);
        assert_eq!(panel.scroll_offset, 0);

        // Phase 7: Add messages after clear
        panel.add_user_message("After clear");
        assert_eq!(panel.message_count(), 1);
    }

    // ============ Edge Cases for Formatting ============

    #[test]
    fn test_formatted_time_various_timestamps() {
        let mut msg = ChatMessage::new(MessageRole::User, "Test");

        // 00:00
        msg.timestamp = 0;
        assert_eq!(msg.formatted_time(), "00:00");

        // 12:34
        msg.timestamp = 12 * 3600 + 34 * 60;
        assert_eq!(msg.formatted_time(), "12:34");

        // 23:59
        msg.timestamp = 23 * 3600 + 59 * 60;
        assert_eq!(msg.formatted_time(), "23:59");
    }

    #[test]
    fn test_message_with_newlines() {
        let msg = ChatMessage::new(MessageRole::User, "Line 1\nLine 2\nLine 3");
        assert_eq!(msg.content.lines().count(), 3);
    }

    #[test]
    fn test_message_empty_content() {
        let msg = ChatMessage::new(MessageRole::User, "");
        assert_eq!(msg.content, "");
        assert!(!msg.has_code);
    }

    #[test]
    fn test_message_whitespace_only() {
        let msg = ChatMessage::new(MessageRole::User, "   \n\t\n   ");
        assert!(msg.content.len() > 0);
    }

    #[test]
    fn test_panel_builder_chain() {
        let panel = ChatPanel::new()
            .with_max_history(500)
            .with_user_color(Color::Red)
            .with_assistant_color(Color::Blue)
            .with_system_color(Color::Magenta);

        assert_eq!(panel.max_history, 500);
        assert_eq!(panel.user_color, Color::Red);
        assert_eq!(panel.assistant_color, Color::Blue);
        assert_eq!(panel.system_color, Color::Magenta);
    }

    // ============ ADDITIONAL FUNCTIONAL TESTS FROM INTEGRATION ============

    #[test]
    fn test_chat_message_formatted_time_01_01() {
        let msg = ChatMessage {
            role: MessageRole::User,
            content: "Test".to_string(),
            timestamp: 3661, // 1 hour, 1 minute, 1 second
            streaming: false,
            has_code: false,
        };
        assert_eq!(msg.formatted_time(), "01:01");
    }

    #[test]
    fn test_chat_message_formatted_time_midnight_explicit() {
        let msg = ChatMessage {
            role: MessageRole::User,
            content: "Test".to_string(),
            timestamp: 0,
            streaming: false,
            has_code: false,
        };
        assert_eq!(msg.formatted_time(), "00:00");
    }

    #[test]
    fn test_chat_message_formatted_time_end_of_day_explicit() {
        let msg = ChatMessage {
            role: MessageRole::User,
            content: "Test".to_string(),
            timestamp: 86399, // 23:59:59
            streaming: false,
            has_code: false,
        };
        assert_eq!(msg.formatted_time(), "23:59");
    }

    #[test]
    fn test_chat_message_system_role_specific() {
        let msg = ChatMessage::new(MessageRole::System, "System message");
        assert_eq!(msg.role, MessageRole::System);
        assert_eq!(msg.content, "System message");
    }

    #[test]
    fn test_chat_message_append_updates_has_code_flag() {
        let mut msg = ChatMessage::streaming(MessageRole::Assistant, "No code yet");
        assert!(!msg.has_code);
        msg.append("\n```rust\nfn main() {}\n```");
        assert!(msg.has_code);
    }

    #[test]
    fn test_chat_message_very_long_content_10k() {
        let long_text = "a".repeat(10_000);
        let msg = ChatMessage::new(MessageRole::Assistant, &long_text);
        assert_eq!(msg.content.len(), 10_000);
    }

    #[test]
    fn test_chat_message_multiple_code_blocks_detection() {
        let content = "First:\n```js\ncode1();\n```\nSecond:\n```py\ncode2()\n```";
        let msg = ChatMessage::new(MessageRole::Assistant, content);
        assert!(msg.has_code);
    }

    #[test]
    fn test_chat_panel_toggle_timestamps_method() {
        let mut panel = ChatPanel::new();
        assert!(panel.show_timestamps);
        panel.toggle_timestamps();
        assert!(!panel.show_timestamps);
        panel.toggle_timestamps();
        assert!(panel.show_timestamps);
    }

    #[test]
    fn test_chat_panel_toggle_auto_scroll_method() {
        let mut panel = ChatPanel::new();
        assert!(panel.auto_scroll);
        panel.toggle_auto_scroll();
        assert!(!panel.auto_scroll);
        panel.toggle_auto_scroll();
        assert!(panel.auto_scroll);
    }

    #[test]
    fn test_chat_panel_with_colors_builder() {
        let panel = ChatPanel::new()
            .with_user_color(Color::Red)
            .with_assistant_color(Color::Blue)
            .with_system_color(Color::Magenta);
        assert_eq!(panel.user_color, Color::Red);
        assert_eq!(panel.assistant_color, Color::Blue);
        assert_eq!(panel.system_color, Color::Magenta);
    }

    #[test]
    fn test_chat_panel_add_system_message_method() {
        let mut panel = ChatPanel::new();
        panel.add_system_message("Error occurred");
        assert_eq!(panel.message_count(), 1);
        assert_eq!(panel.messages[0].role, MessageRole::System);
        assert_eq!(panel.messages[0].content, "Error occurred");
    }

    #[test]
    fn test_chat_panel_scroll_to_top_method() {
        let mut panel = ChatPanel::new();
        panel.scroll_to_top();
        assert_eq!(panel.scroll_offset, usize::MAX);
        assert!(!panel.is_auto_scrolling());
    }

    #[test]
    fn test_chat_panel_scroll_down_to_zero_enables_auto_scroll_behavior() {
        let mut panel = ChatPanel::new();
        panel.scroll_up(10);
        assert!(!panel.is_auto_scrolling());
        panel.scroll_down(10);
        assert_eq!(panel.scroll_offset, 0);
        assert!(panel.is_auto_scrolling());
    }

    #[test]
    fn test_chat_panel_streaming_when_not_streaming_noop() {
        let mut panel = ChatPanel::new();
        panel.append_streaming("This should be ignored");
        assert_eq!(panel.message_count(), 0);
        panel.add_user_message("Regular message");
        panel.append_streaming("Also ignored");
        assert_eq!(panel.messages[0].content, "Regular message");
    }

    #[test]
    fn test_chat_panel_finish_streaming_when_not_streaming_noop() {
        let mut panel = ChatPanel::new();
        panel.finish_streaming();
        assert_eq!(panel.message_count(), 0);
        panel.add_user_message("Not streaming");
        panel.finish_streaming();
        assert!(!panel.messages[0].streaming);
    }

    #[test]
    fn test_chat_panel_multiple_streaming_appends_concatenation() {
        let mut panel = ChatPanel::new();
        panel.start_streaming();
        for i in 0..10 {
            panel.append_streaming(&format!("chunk{} ", i));
        }
        panel.finish_streaming();
        assert_eq!(panel.message_count(), 1);
        assert_eq!(panel.messages[0].content, "chunk0 chunk1 chunk2 chunk3 chunk4 chunk5 chunk6 chunk7 chunk8 chunk9 ");
    }

    #[test]
    fn test_chat_panel_auto_scroll_on_add_behavior() {
        let mut panel = ChatPanel::new();
        panel.scroll_up(5);
        assert_eq!(panel.scroll_offset, 5);
        panel.auto_scroll = false;
        panel.add_user_message("Test");
        assert_eq!(panel.scroll_offset, 5);
        panel.auto_scroll = true;
        panel.add_user_message("Test 2");
        assert_eq!(panel.scroll_offset, 0);
    }

    #[test]
    fn test_chat_panel_max_history_exact_boundary_behavior() {
        let mut panel = ChatPanel::new().with_max_history(5);
        for i in 1..=5 {
            panel.add_user_message(&i.to_string());
        }
        assert_eq!(panel.message_count(), 5);
        assert_eq!(panel.messages[0].content, "1");
        assert_eq!(panel.messages[4].content, "5");
        panel.add_user_message("6");
        assert_eq!(panel.message_count(), 5);
        assert_eq!(panel.messages[0].content, "2");
        assert_eq!(panel.messages[4].content, "6");
    }

    #[test]
    fn test_chat_panel_start_streaming_returns_index_value() {
        let mut panel = ChatPanel::new();
        panel.add_user_message("First");
        let index = panel.start_streaming();
        assert_eq!(index, 1);
        assert_eq!(panel.message_count(), 2);
        assert!(panel.messages[1].streaming);
    }

    #[test]
    fn test_chat_panel_mixed_message_types_all_roles() {
        let mut panel = ChatPanel::new();
        panel.add_user_message("User query");
        panel.add_assistant_message("Assistant response");
        panel.add_system_message("System notification");
        assert_eq!(panel.message_count(), 3);
        assert_eq!(panel.messages[0].role, MessageRole::User);
        assert_eq!(panel.messages[1].role, MessageRole::Assistant);
        assert_eq!(panel.messages[2].role, MessageRole::System);
    }
}
