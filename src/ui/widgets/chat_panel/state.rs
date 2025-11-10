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
            .unwrap_or(Duration::ZERO)
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
    pub(super) messages: Vec<ChatMessage>,
    /// Scroll offset (number of lines scrolled from bottom)
    pub(super) scroll_offset: usize,
    /// Auto-scroll to bottom on new messages
    pub(super) auto_scroll: bool,
    /// Show timestamps
    pub(super) show_timestamps: bool,
    /// Maximum messages to keep in history
    pub(super) max_history: usize,
    /// User color
    pub(super) user_color: Color,
    /// Assistant color
    pub(super) assistant_color: Color,
    /// System color
    pub(super) system_color: Color,
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

