//! MessageBubble molecule - Individual chat message display
//!
//! Displays a single message in a conversation with role-based styling.
//!
//! # Architecture
//!
//! Following Atomic Design:
//! - **Molecule**: Composed of Text atoms
//! - **Role-based styling**: Different colors for User vs Assistant
//! - **Pure rendering**: No state mutation
//!
//! # Examples
//!
//! ```
//! use toad::ui::molecules::MessageBubble;
//! use toad::ai::llm::{Message, Role};
//! use ratatui::text::Line;
//!
//! let msg = Message::user("Hello!");
//! let bubble = MessageBubble::new(&msg);
//! let lines = bubble.to_lines(80); // Max width 80 chars
//! ```

use crate::ai::llm::{Message, Role};
use crate::ui::atoms::{MarkdownRenderer, Text};
use crate::ui::theme::ToadTheme;
use ratatui::{
    style::{Modifier, Style},
    text::Line,
};

/// Message bubble displaying a single chat message
///
/// Composes Text atoms to render user or assistant messages with role-based styling.
///
/// # Examples
///
/// ```
/// use toad::ui::molecules::MessageBubble;
/// use toad::ai::llm::Message;
///
/// let msg = Message::user("Test message");
/// let bubble = MessageBubble::new(&msg);
/// assert_eq!(bubble.role_label(), "You");
/// ```
#[derive(Debug, Clone)]
pub struct MessageBubble<'a> {
    /// Reference to the message
    message: &'a Message,
}

impl<'a> MessageBubble<'a> {
    /// Create a new message bubble
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::MessageBubble;
    /// use toad::ai::llm::Message;
    ///
    /// let msg = Message::assistant("Response");
    /// let bubble = MessageBubble::new(&msg);
    /// ```
    pub fn new(message: &'a Message) -> Self {
        Self { message }
    }

    /// Get the role label for this message
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::MessageBubble;
    /// use toad::ai::llm::Message;
    ///
    /// let user_msg = Message::user("Test");
    /// let bubble = MessageBubble::new(&user_msg);
    /// assert_eq!(bubble.role_label(), "You");
    ///
    /// let assistant_msg = Message::assistant("Response");
    /// let bubble = MessageBubble::new(&assistant_msg);
    /// assert_eq!(bubble.role_label(), "Assistant");
    /// ```
    pub fn role_label(&self) -> &'static str {
        match self.message.role {
            Role::User => "You",
            Role::Assistant => "Assistant",
        }
    }

    /// Get the style for the role label
    fn role_style(&self) -> Style {
        match self.message.role {
            Role::User => Style::default()
                .fg(ToadTheme::TOAD_GREEN)
                .add_modifier(Modifier::BOLD),
            Role::Assistant => Style::default()
                .fg(ToadTheme::BLUE)
                .add_modifier(Modifier::BOLD),
        }
    }

    /// Get the style for message content
    fn content_style(&self) -> Style {
        Style::default().fg(ToadTheme::FOREGROUND)
    }

    /// Convert message to rendered lines with word wrapping
    ///
    /// # Parameters
    ///
    /// - `max_width`: Maximum width for wrapping (typically terminal width - 2 for borders)
    ///
    /// # Returns
    ///
    /// Vector of Lines ready for rendering
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::MessageBubble;
    /// use toad::ai::llm::Message;
    ///
    /// let msg = Message::user("Short message");
    /// let bubble = MessageBubble::new(&msg);
    /// let lines = bubble.to_lines(80);
    /// assert!(lines.len() >= 2); // At least role header + content
    /// ```
    pub fn to_lines(&self, max_width: usize) -> Vec<Line<'static>> {
        let mut lines = Vec::new();

        // Role header with timestamp using Text atom
        let time_str = self.message.timestamp.format("%H:%M").to_string();
        let role_text = Text::new(format!("{} [{}]:", self.role_label(), time_str))
            .style(self.role_style());
        lines.push(role_text.to_line());

        // Render content based on role
        match self.message.role {
            Role::Assistant => {
                // Use markdown rendering for assistant messages
                let renderer = MarkdownRenderer::new();
                let markdown_lines = renderer.render(&self.message.content);

                // Indent each line by prepending "  " span
                for line in markdown_lines {
                    let mut indented_spans = vec![ratatui::text::Span::raw("  ")];
                    indented_spans.extend(line.spans);
                    lines.push(Line::from(indented_spans));
                }
            }
            Role::User => {
                // Plain text rendering for user messages
                let content_style = self.content_style();
                for content_line in self.message.content.lines() {
                    if content_line.is_empty() {
                        lines.push(Line::from(""));
                    } else {
                        // Word wrap if needed
                        if content_line.len() <= max_width {
                            let text =
                                Text::new(format!("  {}", content_line)).style(content_style);
                            lines.push(text.to_line());
                        } else {
                            // Simple word wrapping
                            let words: Vec<&str> = content_line.split_whitespace().collect();
                            let mut current_line = String::from("  ");

                            for word in words {
                                if current_line.len() + word.len() < max_width {
                                    if current_line.len() > 2 {
                                        current_line.push(' ');
                                    }
                                    current_line.push_str(word);
                                } else {
                                    let text =
                                        Text::new(current_line.clone()).style(content_style);
                                    lines.push(text.to_line());
                                    current_line = format!("  {}", word);
                                }
                            }

                            if current_line.len() > 2 {
                                let text = Text::new(current_line).style(content_style);
                                lines.push(text.to_line());
                            }
                        }
                    }
                }
            }
        }

        // Add spacing after message
        lines.push(Line::from(""));

        lines
    }

    /// Get the message reference
    pub fn message(&self) -> &Message {
        self.message
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_bubble_new() {
        let msg = Message::user("Test");
        let bubble = MessageBubble::new(&msg);
        assert_eq!(bubble.message().content, "Test");
    }

    #[test]
    fn test_role_label_user() {
        let msg = Message::user("Test");
        let bubble = MessageBubble::new(&msg);
        assert_eq!(bubble.role_label(), "You");
    }

    #[test]
    fn test_role_label_assistant() {
        let msg = Message::assistant("Test");
        let bubble = MessageBubble::new(&msg);
        assert_eq!(bubble.role_label(), "Assistant");
    }

    #[test]
    fn test_to_lines_simple() {
        let msg = Message::user("Hello");
        let bubble = MessageBubble::new(&msg);
        let lines = bubble.to_lines(80);

        // Should have: role header + content + spacing
        assert_eq!(lines.len(), 3);
    }

    #[test]
    fn test_to_lines_multiline() {
        let msg = Message::user("Line 1\nLine 2\nLine 3");
        let bubble = MessageBubble::new(&msg);
        let lines = bubble.to_lines(80);

        // Should have: role header + 3 content lines + spacing
        assert_eq!(lines.len(), 5);
    }

    #[test]
    fn test_to_lines_word_wrapping() {
        let long_message = "This is a very long message that should wrap across multiple lines when the width is constrained";
        let msg = Message::user(long_message);
        let bubble = MessageBubble::new(&msg);
        let lines = bubble.to_lines(40);

        // Should wrap into multiple lines
        assert!(lines.len() > 3); // More than just role + content + spacing
    }

    #[test]
    fn test_to_lines_empty_lines() {
        let msg = Message::user("Line 1\n\nLine 3");
        let bubble = MessageBubble::new(&msg);
        let lines = bubble.to_lines(80);

        // Should preserve empty lines
        assert_eq!(lines.len(), 5); // role + line1 + empty + line3 + spacing
    }

    #[test]
    fn test_clone() {
        let msg = Message::user("Test");
        let bubble1 = MessageBubble::new(&msg);
        let bubble2 = bubble1.clone();

        assert_eq!(bubble1.role_label(), bubble2.role_label());
        assert_eq!(bubble1.message().content, bubble2.message().content);
    }

    #[test]
    fn test_role_styles_different() {
        let user_msg = Message::user("User message");
        let assistant_msg = Message::assistant("Assistant message");

        let user_bubble = MessageBubble::new(&user_msg);
        let assistant_bubble = MessageBubble::new(&assistant_msg);

        // Styles should be different
        assert_ne!(
            user_bubble.role_style().fg,
            assistant_bubble.role_style().fg
        );
    }

    #[test]
    fn test_message_accessor() {
        let msg = Message::user("Test content");
        let bubble = MessageBubble::new(&msg);

        assert_eq!(bubble.message().content, "Test content");
        assert_eq!(bubble.message().role, Role::User);
    }
}
