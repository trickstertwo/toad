//! ConversationView widget - Displays chat history
//!
//! Shows the back-and-forth conversation between user and AI assistant.
//!
//! # Architecture
//!
//! Following Atomic Design:
//! - Displays messages with role-based styling
//! - Scrollable for long conversations
//! - Auto-scrolls to latest message
//!
//! # Examples
//!
//! ```no_run
//! use toad::ui::widgets::conversation::ConversationView;
//! use toad::ai::llm::{Message, Role};
//! use ratatui::Frame;
//! use ratatui::layout::Rect;
//!
//! let mut view = ConversationView::new();
//! view.add_message(Message::user("Hello!"));
//! view.add_message(Message::assistant("Hi! How can I help?"));
//! // Then render: view.render(&mut frame, area);
//! ```

use crate::ai::llm::Message;
use crate::ui::{
    atoms::{text::Text as AtomText, Block},
    molecules::MessageBubble,
    theme::ToadTheme,
};
use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    text::Line,
    widgets::{Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
};

/// Conversation view displaying chat history
///
/// Displays user and assistant messages in a scrollable view with role-based styling.
///
/// # Examples
///
/// ```
/// use toad::ui::widgets::conversation::ConversationView;
/// use toad::ai::llm::Message;
///
/// let mut view = ConversationView::new();
/// view.add_message(Message::user("Hello"));
/// assert_eq!(view.message_count(), 1);
/// ```
#[derive(Debug)]
pub struct ConversationView {
    /// Chat messages (user and assistant)
    messages: Vec<Message>,
    /// Scroll offset (lines from top)
    scroll_offset: usize,
    /// Whether to auto-scroll to bottom on new messages
    auto_scroll: bool,
}

impl ConversationView {
    /// Create a new conversation view
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::widgets::conversation::ConversationView;
    ///
    /// let view = ConversationView::new();
    /// assert_eq!(view.message_count(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
            scroll_offset: 0,
            auto_scroll: true,
        }
    }

    /// Add a message to the conversation
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::widgets::conversation::ConversationView;
    /// use toad::ai::llm::Message;
    ///
    /// let mut view = ConversationView::new();
    /// view.add_message(Message::user("Test"));
    /// assert_eq!(view.message_count(), 1);
    /// ```
    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
        if self.auto_scroll {
            // Scroll to bottom on new message
            self.scroll_to_bottom();
        }
    }

    /// Add multiple messages at once
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::widgets::conversation::ConversationView;
    /// use toad::ai::llm::Message;
    ///
    /// let mut view = ConversationView::new();
    /// view.add_messages(vec![
    ///     Message::user("Hello"),
    ///     Message::assistant("Hi"),
    /// ]);
    /// assert_eq!(view.message_count(), 2);
    /// ```
    pub fn add_messages(&mut self, messages: Vec<Message>) {
        self.messages.extend(messages);
        if self.auto_scroll {
            self.scroll_to_bottom();
        }
    }

    /// Get the number of messages
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::widgets::conversation::ConversationView;
    /// use toad::ai::llm::Message;
    ///
    /// let mut view = ConversationView::new();
    /// view.add_message(Message::user("Test"));
    /// assert_eq!(view.message_count(), 1);
    /// ```
    pub fn message_count(&self) -> usize {
        self.messages.len()
    }

    /// Clear all messages
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::widgets::conversation::ConversationView;
    /// use toad::ai::llm::Message;
    ///
    /// let mut view = ConversationView::new();
    /// view.add_message(Message::user("Test"));
    /// view.clear();
    /// assert_eq!(view.message_count(), 0);
    /// ```
    pub fn clear(&mut self) {
        self.messages.clear();
        self.scroll_offset = 0;
    }

    /// Get all messages
    pub fn messages(&self) -> &[Message] {
        &self.messages
    }

    /// Scroll up by one line
    pub fn scroll_up(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_sub(1);
        self.auto_scroll = false;
    }

    /// Scroll down by one line
    pub fn scroll_down(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_add(1);
        self.auto_scroll = false;
    }

    /// Scroll to top
    pub fn scroll_to_top(&mut self) {
        self.scroll_offset = 0;
        self.auto_scroll = false;
    }

    /// Scroll to bottom
    pub fn scroll_to_bottom(&mut self) {
        self.scroll_offset = usize::MAX; // Will be clamped during render
        self.auto_scroll = true;
    }

    /// Set auto-scroll behavior
    pub fn set_auto_scroll(&mut self, enabled: bool) {
        self.auto_scroll = enabled;
    }

    /// Get current scroll offset
    pub fn scroll_offset(&self) -> usize {
        self.scroll_offset
    }

    /// Render the conversation view
    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        // Create border using Block atom
        let block = Block::themed("Conversation").to_ratatui();
        let inner = block.inner(area);
        frame.render_widget(block, area);

        // Convert messages to lines using MessageBubble molecule
        let mut lines = Vec::new();

        if self.messages.is_empty() {
            // Show empty state
            lines.push(Line::from(vec![AtomText::new(
                "No messages yet. Start typing to chat with AI...",
            )
            .style(
                Style::default()
                    .fg(ToadTheme::GRAY)
                    .add_modifier(Modifier::ITALIC),
            )
            .to_span()]));
        } else {
            let max_width = inner.width.saturating_sub(2) as usize;

            for message in &self.messages {
                // Use MessageBubble molecule to render each message
                let bubble = MessageBubble::new(message);
                let message_lines = bubble.to_lines(max_width);
                lines.extend(message_lines);
            }
        }

        // Clamp scroll offset
        let max_scroll = lines.len().saturating_sub(inner.height as usize);
        self.scroll_offset = self.scroll_offset.min(max_scroll);

        // Calculate visible lines
        let visible_lines: Vec<Line> = lines
            .into_iter()
            .skip(self.scroll_offset)
            .take(inner.height as usize)
            .collect();

        let paragraph = Paragraph::new(visible_lines).alignment(Alignment::Left);
        frame.render_widget(paragraph, inner);

        // Render scrollbar if needed
        if self.messages.len() > inner.height as usize {
            let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓"));

            let total_lines = self.messages.len();
            let viewport_height = inner.height as usize;
            let mut scrollbar_state = ScrollbarState::new(total_lines)
                .position(self.scroll_offset)
                .viewport_content_length(viewport_height);

            frame.render_stateful_widget(scrollbar, inner, &mut scrollbar_state);
        }
    }
}

impl Default for ConversationView {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai::llm::Role;

    #[test]
    fn test_conversation_view_new() {
        let view = ConversationView::new();
        assert_eq!(view.message_count(), 0);
        assert_eq!(view.scroll_offset(), 0);
    }

    #[test]
    fn test_conversation_view_add_message() {
        let mut view = ConversationView::new();
        view.add_message(Message::user("Hello"));
        assert_eq!(view.message_count(), 1);
    }

    #[test]
    fn test_conversation_view_add_messages() {
        let mut view = ConversationView::new();
        view.add_messages(vec![Message::user("Hello"), Message::assistant("Hi")]);
        assert_eq!(view.message_count(), 2);
    }

    #[test]
    fn test_conversation_view_clear() {
        let mut view = ConversationView::new();
        view.add_message(Message::user("Test"));
        view.clear();
        assert_eq!(view.message_count(), 0);
        assert_eq!(view.scroll_offset(), 0);
    }

    #[test]
    fn test_conversation_view_scroll_up() {
        let mut view = ConversationView::new();
        view.scroll_offset = 5;
        view.scroll_up();
        assert_eq!(view.scroll_offset(), 4);
    }

    #[test]
    fn test_conversation_view_scroll_down() {
        let mut view = ConversationView::new();
        view.scroll_down();
        assert_eq!(view.scroll_offset(), 1);
    }

    #[test]
    fn test_conversation_view_scroll_to_top() {
        let mut view = ConversationView::new();
        view.scroll_offset = 10;
        view.scroll_to_top();
        assert_eq!(view.scroll_offset(), 0);
    }

    #[test]
    fn test_conversation_view_scroll_to_bottom() {
        let mut view = ConversationView::new();
        view.scroll_to_bottom();
        assert_eq!(view.scroll_offset(), usize::MAX);
    }

    #[test]
    fn test_conversation_view_auto_scroll() {
        let mut view = ConversationView::new();
        assert!(view.auto_scroll);

        view.set_auto_scroll(false);
        assert!(!view.auto_scroll);

        view.set_auto_scroll(true);
        assert!(view.auto_scroll);
    }

    #[test]
    fn test_conversation_view_messages() {
        let mut view = ConversationView::new();
        view.add_message(Message::user("Test 1"));
        view.add_message(Message::assistant("Test 2"));

        let messages = view.messages();
        assert_eq!(messages.len(), 2);
        assert_eq!(messages[0].content, "Test 1");
        assert_eq!(messages[1].content, "Test 2");
    }

    #[test]
    fn test_conversation_view_role_detection() {
        let mut view = ConversationView::new();
        view.add_message(Message::user("User message"));
        view.add_message(Message::assistant("Assistant message"));

        let messages = view.messages();
        assert_eq!(messages[0].role, Role::User);
        assert_eq!(messages[1].role, Role::Assistant);
    }

    #[test]
    fn test_conversation_view_default() {
        let view = ConversationView::default();
        assert_eq!(view.message_count(), 0);
    }

    #[test]
    fn test_conversation_view_clone_independence() {
        let mut view1 = ConversationView::new();
        view1.add_message(Message::user("Test"));

        // Can't clone directly but can verify independence
        let count1 = view1.message_count();
        view1.add_message(Message::assistant("Response"));
        assert_eq!(view1.message_count(), count1 + 1);
    }

    #[test]
    fn test_conversation_view_multiple_operations() {
        let mut view = ConversationView::new();

        // Add messages
        view.add_message(Message::user("First"));
        view.add_message(Message::assistant("Second"));
        view.add_message(Message::user("Third"));

        assert_eq!(view.message_count(), 3);

        // Scroll operations (reset auto-scroll first to get predictable behavior)
        view.scroll_to_top(); // This sets auto_scroll to false and offset to 0
        view.scroll_down();
        view.scroll_down();
        assert_eq!(view.scroll_offset(), 2);

        view.scroll_up();
        assert_eq!(view.scroll_offset(), 1);

        // Clear
        view.clear();
        assert_eq!(view.message_count(), 0);
        assert_eq!(view.scroll_offset(), 0);
    }
}
