//! Card preview widget for hover-to-expand functionality
//!
//! Displays card details in a popup overlay without opening full details view.
//! Supports hovering over cards to show expanded information.
//!
//! # Examples
//!
//! ```
//! use toad::ui::widgets::CardPreview;
//!
//! let preview = CardPreview::new("Buy groceries")
//!     .description("Get milk, eggs, and bread")
//!     .tags(vec!["shopping".to_string(), "urgent".to_string()])
//!     .priority("High");
//! ```

use crate::ui::theme::ToadTheme;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

/// Card priority level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardPriority {
    Low,
    Medium,
    High,
    Critical,
}

impl CardPriority {
    /// Get color for priority
    fn color(&self) -> ratatui::style::Color {
        match self {
            CardPriority::Low => ToadTheme::DARK_GRAY,
            CardPriority::Medium => ToadTheme::TOAD_GREEN,
            CardPriority::High => ratatui::style::Color::Yellow,
            CardPriority::Critical => ratatui::style::Color::Red,
        }
    }

    /// Get icon for priority
    fn icon(&self) -> &str {
        match self {
            CardPriority::Low => "▼",
            CardPriority::Medium => "●",
            CardPriority::High => "▲",
            CardPriority::Critical => "⚠",
        }
    }

    /// Parse priority from string
    fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "low" => CardPriority::Low,
            "medium" | "med" => CardPriority::Medium,
            "high" => CardPriority::High,
            "critical" | "crit" => CardPriority::Critical,
            _ => CardPriority::Medium,
        }
    }
}

/// Card preview position relative to the hovered card
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreviewPosition {
    /// Show to the right of the card
    Right,
    /// Show to the left of the card
    Left,
    /// Show above the card
    Above,
    /// Show below the card
    Below,
    /// Show centered overlay
    Centered,
}

/// Card preview widget
///
/// Displays card details in a hover overlay without opening full details.
/// Supports tags, priority, description, and metadata.
#[derive(Debug, Clone)]
pub struct CardPreview {
    /// Card title
    title: String,
    /// Card description
    description: Option<String>,
    /// Tags associated with the card
    tags: Vec<String>,
    /// Priority level
    priority: CardPriority,
    /// Status text (e.g., "In Progress", "Done")
    status: Option<String>,
    /// Due date
    due_date: Option<String>,
    /// Assigned to
    assignee: Option<String>,
    /// Scroll offset for long descriptions
    scroll_offset: u16,
    /// Preview position
    position: PreviewPosition,
    /// Whether to show metadata (tags, priority, etc.)
    show_metadata: bool,
}

impl CardPreview {
    /// Create a new card preview
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::widgets::CardPreview;
    ///
    /// let preview = CardPreview::new("Fix bug in login");
    /// assert_eq!(preview.title(), "Fix bug in login");
    /// ```
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: None,
            tags: Vec::new(),
            priority: CardPriority::Medium,
            status: None,
            due_date: None,
            assignee: None,
            scroll_offset: 0,
            position: PreviewPosition::Centered,
            show_metadata: true,
        }
    }

    /// Set description
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::widgets::CardPreview;
    ///
    /// let preview = CardPreview::new("Task").description("Details here");
    /// ```
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set tags
    pub fn tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    /// Add a single tag
    pub fn add_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }

    /// Set priority from string
    pub fn priority(mut self, priority: &str) -> Self {
        self.priority = CardPriority::from_str(priority);
        self
    }

    /// Set priority level directly
    pub fn priority_level(mut self, priority: CardPriority) -> Self {
        self.priority = priority;
        self
    }

    /// Set status
    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }

    /// Set due date
    pub fn due_date(mut self, date: impl Into<String>) -> Self {
        self.due_date = Some(date.into());
        self
    }

    /// Set assignee
    pub fn assignee(mut self, assignee: impl Into<String>) -> Self {
        self.assignee = Some(assignee.into());
        self
    }

    /// Set preview position
    pub fn position(mut self, pos: PreviewPosition) -> Self {
        self.position = pos;
        self
    }

    /// Set whether to show metadata
    pub fn show_metadata(mut self, show: bool) -> Self {
        self.show_metadata = show;
        self
    }

    /// Get title
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Get description
    pub fn get_description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Get tags
    pub fn get_tags(&self) -> &[String] {
        &self.tags
    }

    /// Get priority
    pub fn get_priority(&self) -> CardPriority {
        self.priority
    }

    /// Set scroll offset
    pub fn set_scroll_offset(&mut self, offset: u16) {
        self.scroll_offset = offset;
    }

    /// Scroll down
    pub fn scroll_down(&mut self, lines: u16) {
        self.scroll_offset += lines;
    }

    /// Scroll up
    pub fn scroll_up(&mut self, lines: u16) {
        self.scroll_offset = self.scroll_offset.saturating_sub(lines);
    }

    /// Render the card preview
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let preview_area = self.calculate_position(area);
        self.render_content(frame, preview_area);
    }

    /// Calculate preview position based on position setting
    fn calculate_position(&self, area: Rect) -> Rect {
        match self.position {
            PreviewPosition::Centered => {
                let vertical = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Percentage(20),
                        Constraint::Percentage(60),
                        Constraint::Percentage(20),
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

                horizontal[1]
            }
            PreviewPosition::Right => {
                // Show on right side (60% of width)
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
                    .split(area);
                chunks[1]
            }
            PreviewPosition::Left => {
                // Show on left side (60% of width)
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
                    .split(area);
                chunks[0]
            }
            PreviewPosition::Above => {
                // Show on top half
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                    .split(area);
                chunks[0]
            }
            PreviewPosition::Below => {
                // Show on bottom half
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                    .split(area);
                chunks[1]
            }
        }
    }

    /// Render the preview content
    fn render_content(&self, frame: &mut Frame, area: Rect) {
        // Create border with priority color
        let border_color = self.priority.color();

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color))
            .title(format!(
                " {} {} ",
                self.priority.icon(),
                self.title
            ))
            .title_style(
                Style::default()
                    .fg(border_color)
                    .add_modifier(Modifier::BOLD),
            );

        // Create content lines
        let mut lines = Vec::new();

        // Add status if present
        if let Some(status) = &self.status {
            lines.push(Line::from(vec![
                Span::styled(
                    "Status: ",
                    Style::default()
                        .fg(ToadTheme::DARK_GRAY)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(status, Style::default().fg(ToadTheme::TOAD_GREEN)),
            ]));
            lines.push(Line::from(""));
        }

        // Add description if present
        if let Some(description) = &self.description {
            lines.push(Line::from(vec![Span::styled(
                "Description:",
                Style::default()
                    .fg(ToadTheme::DARK_GRAY)
                    .add_modifier(Modifier::BOLD),
            )]));

            // Split description into lines for scrolling
            for line in description.lines().skip(self.scroll_offset as usize) {
                lines.push(Line::from(Span::styled(
                    line,
                    Style::default().fg(ToadTheme::FOREGROUND),
                )));
            }
            lines.push(Line::from(""));
        }

        // Add metadata if enabled
        if self.show_metadata {
            // Tags
            if !self.tags.is_empty() {
                lines.push(Line::from(vec![Span::styled(
                    "Tags: ",
                    Style::default()
                        .fg(ToadTheme::DARK_GRAY)
                        .add_modifier(Modifier::BOLD),
                )]));

                let tag_spans: Vec<Span> = self
                    .tags
                    .iter()
                    .flat_map(|tag| {
                        vec![
                            Span::styled(
                                format!(" #{} ", tag),
                                Style::default()
                                    .fg(ToadTheme::TOAD_GREEN)
                                    .bg(ToadTheme::DARK_GRAY),
                            ),
                            Span::raw(" "),
                        ]
                    })
                    .collect();
                lines.push(Line::from(tag_spans));
                lines.push(Line::from(""));
            }

            // Due date
            if let Some(due_date) = &self.due_date {
                lines.push(Line::from(vec![
                    Span::styled(
                        "Due: ",
                        Style::default()
                            .fg(ToadTheme::DARK_GRAY)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(due_date, Style::default().fg(ratatui::style::Color::Yellow)),
                ]));
            }

            // Assignee
            if let Some(assignee) = &self.assignee {
                lines.push(Line::from(vec![
                    Span::styled(
                        "Assigned to: ",
                        Style::default()
                            .fg(ToadTheme::DARK_GRAY)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(assignee, Style::default().fg(ToadTheme::FOREGROUND)),
                ]));
            }
        }

        // Render paragraph with content
        let paragraph = Paragraph::new(lines)
            .block(block)
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: false });

        frame.render_widget(paragraph, area);
    }
}

impl Default for CardPreview {
    fn default() -> Self {
        Self::new("Untitled Card")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_preview_creation() {
        let preview = CardPreview::new("Test Task");
        assert_eq!(preview.title(), "Test Task");
        assert_eq!(preview.get_description(), None);
        assert!(preview.get_tags().is_empty());
        assert_eq!(preview.get_priority(), CardPriority::Medium);
    }

    #[test]
    fn test_card_preview_with_description() {
        let preview = CardPreview::new("Task").description("This is a test task");
        assert_eq!(preview.get_description(), Some("This is a test task"));
    }

    #[test]
    fn test_card_preview_with_tags() {
        let preview = CardPreview::new("Task").tags(vec!["urgent".to_string(), "bug".to_string()]);
        assert_eq!(preview.get_tags(), &["urgent", "bug"]);
    }

    #[test]
    fn test_card_preview_add_tag() {
        let preview = CardPreview::new("Task").add_tag("test").add_tag("feature");
        assert_eq!(preview.get_tags(), &["test", "feature"]);
    }

    #[test]
    fn test_card_priority_parsing() {
        assert_eq!(CardPriority::from_str("low"), CardPriority::Low);
        assert_eq!(CardPriority::from_str("medium"), CardPriority::Medium);
        assert_eq!(CardPriority::from_str("med"), CardPriority::Medium);
        assert_eq!(CardPriority::from_str("high"), CardPriority::High);
        assert_eq!(CardPriority::from_str("critical"), CardPriority::Critical);
        assert_eq!(CardPriority::from_str("crit"), CardPriority::Critical);
        assert_eq!(CardPriority::from_str("unknown"), CardPriority::Medium);
    }

    #[test]
    fn test_card_preview_with_priority() {
        let preview = CardPreview::new("Task").priority("high");
        assert_eq!(preview.get_priority(), CardPriority::High);
    }

    #[test]
    fn test_card_preview_with_priority_level() {
        let preview = CardPreview::new("Task").priority_level(CardPriority::Critical);
        assert_eq!(preview.get_priority(), CardPriority::Critical);
    }

    #[test]
    fn test_card_preview_with_status() {
        let preview = CardPreview::new("Task").status("In Progress");
        assert_eq!(preview.status, Some("In Progress".to_string()));
    }

    #[test]
    fn test_card_preview_with_due_date() {
        let preview = CardPreview::new("Task").due_date("2025-01-15");
        assert_eq!(preview.due_date, Some("2025-01-15".to_string()));
    }

    #[test]
    fn test_card_preview_with_assignee() {
        let preview = CardPreview::new("Task").assignee("John Doe");
        assert_eq!(preview.assignee, Some("John Doe".to_string()));
    }

    #[test]
    fn test_card_preview_scrolling() {
        let mut preview = CardPreview::new("Task");
        preview.scroll_down(5);
        assert_eq!(preview.scroll_offset, 5);

        preview.scroll_up(2);
        assert_eq!(preview.scroll_offset, 3);

        preview.scroll_up(10); // Should saturate at 0
        assert_eq!(preview.scroll_offset, 0);
    }

    #[test]
    fn test_card_preview_position() {
        let preview = CardPreview::new("Task").position(PreviewPosition::Right);
        assert_eq!(preview.position, PreviewPosition::Right);
    }

    #[test]
    fn test_card_preview_show_metadata() {
        let preview = CardPreview::new("Task").show_metadata(false);
        assert!(!preview.show_metadata);
    }

    #[test]
    fn test_priority_colors() {
        assert_eq!(CardPriority::Low.color(), ToadTheme::DARK_GRAY);
        assert_eq!(CardPriority::Medium.color(), ToadTheme::TOAD_GREEN);
        assert_eq!(CardPriority::High.color(), ratatui::style::Color::Yellow);
        assert_eq!(CardPriority::Critical.color(), ratatui::style::Color::Red);
    }

    #[test]
    fn test_priority_icons() {
        assert_eq!(CardPriority::Low.icon(), "▼");
        assert_eq!(CardPriority::Medium.icon(), "●");
        assert_eq!(CardPriority::High.icon(), "▲");
        assert_eq!(CardPriority::Critical.icon(), "⚠");
    }

    #[test]
    fn test_full_card_preview() {
        let preview = CardPreview::new("Fix authentication bug")
            .description("Users unable to log in with SSO")
            .tags(vec!["urgent".to_string(), "security".to_string()])
            .priority("critical")
            .status("In Progress")
            .due_date("2025-01-10")
            .assignee("Alice Smith")
            .position(PreviewPosition::Right);

        assert_eq!(preview.title(), "Fix authentication bug");
        assert_eq!(preview.get_description(), Some("Users unable to log in with SSO"));
        assert_eq!(preview.get_tags(), &["urgent", "security"]);
        assert_eq!(preview.get_priority(), CardPriority::Critical);
        assert_eq!(preview.position, PreviewPosition::Right);
    }

    #[test]
    fn test_default_card_preview() {
        let preview = CardPreview::default();
        assert_eq!(preview.title(), "Untitled Card");
        assert_eq!(preview.get_priority(), CardPriority::Medium);
        assert_eq!(preview.position, PreviewPosition::Centered);
        assert!(preview.show_metadata);
    }
}
