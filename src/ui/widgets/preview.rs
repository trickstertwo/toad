/// Preview pane widget for showing content in split view
///
/// Displays preview content alongside main content
///
/// # Examples
///
/// ```
/// use toad::widgets::PreviewPane;
///
/// let preview = PreviewPane::new("File contents here...");
/// assert_eq!(preview.content(), "File contents here...");
/// ```
use crate::ui::theme::ToadTheme;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

/// Preview pane widget
#[derive(Debug, Clone)]
pub struct PreviewPane {
    /// Preview content
    content: String,
    /// Title for preview pane
    title: Option<String>,
    /// Scroll offset (line number)
    scroll_offset: u16,
    /// Whether to show line numbers
    show_line_numbers: bool,
    /// Whether to wrap lines
    wrap_lines: bool,
}

impl PreviewPane {
    /// Create a new preview pane
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::PreviewPane;
    ///
    /// let preview = PreviewPane::new("Content");
    /// assert_eq!(preview.content(), "Content");
    /// ```
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            title: None,
            scroll_offset: 0,
            show_line_numbers: false,
            wrap_lines: true,
        }
    }

    /// Set title
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::PreviewPane;
    ///
    /// let preview = PreviewPane::new("Content").title("Preview");
    /// ```
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set scroll offset
    pub fn scroll_offset(mut self, offset: u16) -> Self {
        self.scroll_offset = offset;
        self
    }

    /// Show line numbers
    pub fn show_line_numbers(mut self, show: bool) -> Self {
        self.show_line_numbers = show;
        self
    }

    /// Set line wrapping
    pub fn wrap_lines(mut self, wrap: bool) -> Self {
        self.wrap_lines = wrap;
        self
    }

    /// Get content
    pub fn content(&self) -> &str {
        &self.content
    }

    /// Set content
    pub fn set_content(&mut self, content: impl Into<String>) {
        self.content = content.into();
        self.scroll_offset = 0;
    }

    /// Get scroll offset
    pub fn get_scroll_offset(&self) -> u16 {
        self.scroll_offset
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

    /// Scroll to top
    pub fn scroll_to_top(&mut self) {
        self.scroll_offset = 0;
    }

    /// Get line count
    pub fn line_count(&self) -> usize {
        self.content.lines().count()
    }

    /// Render the preview pane
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let title = self.title.as_deref().unwrap_or("Preview");

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(ToadTheme::DARK_GRAY))
            .title(title)
            .title_style(
                Style::default()
                    .fg(ToadTheme::TOAD_GREEN)
                    .add_modifier(Modifier::BOLD),
            );

        if self.show_line_numbers {
            self.render_with_line_numbers(frame, area, block);
        } else {
            self.render_plain(frame, area, block);
        }
    }

    /// Render without line numbers
    fn render_plain(&self, frame: &mut Frame, area: Rect, block: Block) {
        let mut paragraph = Paragraph::new(self.content.as_str())
            .block(block)
            .style(Style::default().fg(ToadTheme::FOREGROUND))
            .scroll((self.scroll_offset, 0));

        if self.wrap_lines {
            paragraph = paragraph.wrap(Wrap { trim: false });
        }

        frame.render_widget(paragraph, area);
    }

    /// Render with line numbers
    fn render_with_line_numbers(&self, frame: &mut Frame, area: Rect, block: Block) {
        let lines: Vec<Line> = self
            .content
            .lines()
            .enumerate()
            .skip(self.scroll_offset as usize)
            .map(|(idx, line)| {
                let line_num = idx + 1;
                Line::from(vec![
                    Span::styled(
                        format!("{:4} ", line_num),
                        Style::default()
                            .fg(ToadTheme::DARK_GRAY)
                            .add_modifier(Modifier::DIM),
                    ),
                    Span::styled(line, Style::default().fg(ToadTheme::FOREGROUND)),
                ])
            })
            .collect();

        let paragraph = Paragraph::new(lines).block(block);
        frame.render_widget(paragraph, area);
    }
}

impl Default for PreviewPane {
    fn default() -> Self {
        Self::new("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preview_pane_creation() {
        let preview = PreviewPane::new("Test content");
        assert_eq!(preview.content(), "Test content");
        assert_eq!(preview.get_scroll_offset(), 0);
        assert!(!preview.show_line_numbers);
        assert!(preview.wrap_lines);
    }

    #[test]
    fn test_preview_pane_with_title() {
        let preview = PreviewPane::new("Content").title("My Preview");
        assert_eq!(preview.title.as_deref(), Some("My Preview"));
    }

    #[test]
    fn test_set_content() {
        let mut preview = PreviewPane::new("Old");
        preview.set_content("New");
        assert_eq!(preview.content(), "New");
        assert_eq!(preview.get_scroll_offset(), 0); // Should reset scroll
    }

    #[test]
    fn test_scroll_down() {
        let mut preview = PreviewPane::new("Content");
        preview.scroll_down(5);
        assert_eq!(preview.get_scroll_offset(), 5);
    }

    #[test]
    fn test_scroll_up() {
        let mut preview = PreviewPane::new("Content");
        preview.set_scroll_offset(10);

        preview.scroll_up(3);
        assert_eq!(preview.get_scroll_offset(), 7);

        preview.scroll_up(20); // Should clamp at 0
        assert_eq!(preview.get_scroll_offset(), 0);
    }

    #[test]
    fn test_scroll_to_top() {
        let mut preview = PreviewPane::new("Content");
        preview.set_scroll_offset(50);

        preview.scroll_to_top();
        assert_eq!(preview.get_scroll_offset(), 0);
    }

    #[test]
    fn test_line_count() {
        let preview = PreviewPane::new("Line 1\nLine 2\nLine 3");
        assert_eq!(preview.line_count(), 3);
    }

    #[test]
    fn test_show_line_numbers() {
        let preview = PreviewPane::new("Content").show_line_numbers(true);
        assert!(preview.show_line_numbers);
    }

    #[test]
    fn test_wrap_lines() {
        let preview = PreviewPane::new("Content").wrap_lines(false);
        assert!(!preview.wrap_lines);
    }

    #[test]
    fn test_scroll_offset_builder() {
        let preview = PreviewPane::new("Content").scroll_offset(10);
        assert_eq!(preview.get_scroll_offset(), 10);
    }
}
