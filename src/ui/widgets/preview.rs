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

    // ============================================================================
    // ADDITIONAL COMPREHENSIVE EDGE CASE TESTS (ADVANCED TIER - Fuzzy Finding)
    // ============================================================================

    // ============ Stress Tests ============

    #[test]
    fn test_preview_very_large_content_100k_lines() {
        let mut lines = Vec::new();
        for i in 0..100000 {
            lines.push(format!("Line {}", i));
        }
        let content = lines.join("\n");

        let preview = PreviewPane::new(content);
        assert_eq!(preview.line_count(), 100000);
    }

    #[test]
    fn test_preview_very_long_single_line_100k_chars() {
        let content = "x".repeat(100000);
        let preview = PreviewPane::new(content);
        assert_eq!(preview.content().len(), 100000);
        assert_eq!(preview.line_count(), 1);
    }

    #[test]
    fn test_preview_rapid_scrolling_1000_ops() {
        let mut preview = PreviewPane::new("Content");

        for _ in 0..500 {
            preview.scroll_down(1);
        }
        assert_eq!(preview.get_scroll_offset(), 500);

        for _ in 0..500 {
            preview.scroll_up(1);
        }
        assert_eq!(preview.get_scroll_offset(), 0);
    }

    #[test]
    fn test_preview_rapid_content_changes_1000() {
        let mut preview = PreviewPane::new("Initial");

        for i in 0..1000 {
            preview.set_content(format!("Content {}", i));
            assert_eq!(preview.get_scroll_offset(), 0); // Should reset each time
        }

        assert_eq!(preview.content(), "Content 999");
    }

    #[test]
    fn test_preview_alternating_scroll_operations() {
        let mut preview = PreviewPane::new("Content");

        for _ in 0..1000 {
            preview.scroll_down(5);
            preview.scroll_up(5);
        }

        assert_eq!(preview.get_scroll_offset(), 0);
    }

    // ============ Unicode Edge Cases ============

    #[test]
    fn test_preview_content_with_emoji() {
        let content = "Line 1 🚀\nLine 2 🐸\nLine 3 💚";
        let preview = PreviewPane::new(content);
        assert_eq!(preview.line_count(), 3);
        assert!(preview.content().contains('🚀'));
    }

    #[test]
    fn test_preview_content_with_rtl_text() {
        let content = "مرحبا\nשלום\nHello";
        let preview = PreviewPane::new(content);
        assert_eq!(preview.line_count(), 3);
        assert!(preview.content().contains("مرحبا"));
    }

    #[test]
    fn test_preview_content_with_japanese() {
        let content = "日本語\nテスト\n内容";
        let preview = PreviewPane::new(content);
        assert_eq!(preview.line_count(), 3);
        assert_eq!(preview.content(), "日本語\nテスト\n内容");
    }

    #[test]
    fn test_preview_content_with_combining_chars() {
        let content = "é̂ñ̃\nCafé\nnaïve";
        let preview = PreviewPane::new(content);
        assert_eq!(preview.line_count(), 3);
        assert!(preview.content().len() > 10);
    }

    #[test]
    fn test_preview_content_with_zero_width() {
        let content = "Test\u{200B}Zero\u{200C}Width\u{200D}";
        let preview = PreviewPane::new(content);
        assert!(preview.content().contains("Test"));
        assert!(preview.content().contains("Zero"));
    }

    #[test]
    fn test_preview_content_with_mixed_scripts() {
        let content = "Hello日本مرحبا🚀\nMixed\nScripts";
        let preview = PreviewPane::new(content);
        assert_eq!(preview.line_count(), 3);
        assert!(preview.content().contains("Hello"));
        assert!(preview.content().contains("日本"));
    }

    #[test]
    fn test_preview_line_count_with_unicode_lines() {
        let content = "🚀\n日本語\nمرحبا\nHello";
        let preview = PreviewPane::new(content);
        assert_eq!(preview.line_count(), 4);
    }

    // ============ Scroll Edge Cases ============

    #[test]
    fn test_preview_scroll_beyond_max_u16() {
        let mut preview = PreviewPane::new("Content");
        preview.set_scroll_offset(u16::MAX);
        assert_eq!(preview.get_scroll_offset(), u16::MAX);
    }

    #[test]
    fn test_preview_scroll_up_from_zero() {
        let mut preview = PreviewPane::new("Content");
        preview.scroll_up(100); // Should saturate at 0
        assert_eq!(preview.get_scroll_offset(), 0);
    }

    #[test]
    fn test_preview_scroll_up_saturating() {
        let mut preview = PreviewPane::new("Content");
        preview.set_scroll_offset(10);
        preview.scroll_up(20); // Should saturate at 0
        assert_eq!(preview.get_scroll_offset(), 0);
    }

    #[test]
    fn test_preview_scroll_with_empty_content() {
        let mut preview = PreviewPane::new("");
        preview.scroll_down(10);
        preview.scroll_up(5);
        assert_eq!(preview.get_scroll_offset(), 5);
    }

    #[test]
    fn test_preview_scroll_to_top_from_max() {
        let mut preview = PreviewPane::new("Content");
        preview.set_scroll_offset(u16::MAX);
        preview.scroll_to_top();
        assert_eq!(preview.get_scroll_offset(), 0);
    }

    // ============ Content Edge Cases ============

    #[test]
    fn test_preview_empty_content() {
        let preview = PreviewPane::new("");
        assert_eq!(preview.content(), "");
        assert_eq!(preview.line_count(), 0);
    }

    #[test]
    fn test_preview_single_char_content() {
        let preview = PreviewPane::new("x");
        assert_eq!(preview.content(), "x");
        assert_eq!(preview.line_count(), 1);
    }

    #[test]
    fn test_preview_many_newlines() {
        let content = "\n\n\n\n\n\n\n\n\n\n";
        let preview = PreviewPane::new(content);
        assert_eq!(preview.line_count(), 10);
    }

    #[test]
    fn test_preview_only_newlines_empty_lines() {
        let content = "\n\n\n";
        let preview = PreviewPane::new(content);
        assert_eq!(preview.line_count(), 3);
    }

    #[test]
    fn test_preview_single_line_no_newline() {
        let content = "This is a single long line without any newlines";
        let preview = PreviewPane::new(content);
        assert_eq!(preview.line_count(), 1);
    }

    #[test]
    fn test_preview_whitespace_only_content() {
        let content = "   \n  \n    \n";
        let preview = PreviewPane::new(content);
        assert_eq!(preview.line_count(), 3);
    }

    #[test]
    fn test_preview_tabs_and_spaces() {
        let content = "\t\tTabbed\n    Spaced\n\t  Mixed";
        let preview = PreviewPane::new(content);
        assert_eq!(preview.line_count(), 3);
    }

    // ============ Clone and Debug Traits ============

    #[test]
    fn test_preview_clone() {
        let preview = PreviewPane::new("Test content")
            .title("Test Title")
            .scroll_offset(10)
            .show_line_numbers(true)
            .wrap_lines(false);

        let cloned = preview.clone();
        assert_eq!(preview.content(), cloned.content());
        assert_eq!(preview.get_scroll_offset(), cloned.get_scroll_offset());
        assert_eq!(preview.show_line_numbers, cloned.show_line_numbers);
        assert_eq!(preview.wrap_lines, cloned.wrap_lines);
    }

    #[test]
    fn test_preview_debug() {
        let preview = PreviewPane::new("Content");
        let debug_str = format!("{:?}", preview);
        assert!(debug_str.contains("PreviewPane"));
    }

    // ============ Complex Workflow Tests ============

    #[test]
    fn test_preview_workflow_set_scroll_reset() {
        let mut preview = PreviewPane::new("Initial content");
        preview.scroll_down(50);
        assert_eq!(preview.get_scroll_offset(), 50);

        preview.set_content("New content");
        assert_eq!(preview.get_scroll_offset(), 0); // Should reset

        preview.scroll_down(10);
        assert_eq!(preview.get_scroll_offset(), 10);
    }

    #[test]
    fn test_preview_builder_pattern_chaining() {
        let preview = PreviewPane::new("Content")
            .title("My Preview")
            .scroll_offset(5)
            .show_line_numbers(true)
            .wrap_lines(false);

        assert_eq!(preview.content(), "Content");
        assert_eq!(preview.title.as_deref(), Some("My Preview"));
        assert_eq!(preview.get_scroll_offset(), 5);
        assert!(preview.show_line_numbers);
        assert!(!preview.wrap_lines);
    }

    #[test]
    fn test_preview_rapid_operations_mixed() {
        let mut preview = PreviewPane::new("Initial");

        for i in 0..100 {
            preview.scroll_down(1);
            if i % 10 == 0 {
                preview.set_content(format!("Content {}", i));
            }
            preview.scroll_up(1);
        }

        assert_eq!(preview.get_scroll_offset(), 0);
    }

    #[test]
    fn test_preview_workflow_unicode_operations() {
        let mut preview = PreviewPane::new("日本語");
        preview.scroll_down(5);
        assert_eq!(preview.get_scroll_offset(), 5);

        preview.set_content("مرحبا\n🚀\nTest");
        assert_eq!(preview.line_count(), 3);
        assert_eq!(preview.get_scroll_offset(), 0);
    }

    // ============ Comprehensive Stress Test ============

    #[test]
    fn test_comprehensive_preview_stress() {
        let mut preview = PreviewPane::new("Initial content");

        // Phase 1: Large content with many lines
        let mut lines = Vec::new();
        for i in 0..1000 {
            lines.push(format!("Line {} with some content", i));
        }
        preview.set_content(lines.join("\n"));
        assert_eq!(preview.line_count(), 1000);
        assert_eq!(preview.get_scroll_offset(), 0);

        // Phase 2: Scroll operations
        preview.scroll_down(100);
        assert_eq!(preview.get_scroll_offset(), 100);

        preview.scroll_up(50);
        assert_eq!(preview.get_scroll_offset(), 50);

        preview.scroll_to_top();
        assert_eq!(preview.get_scroll_offset(), 0);

        // Phase 3: Builder pattern modifications
        let preview2 = preview
            .clone()
            .title("Stress Test")
            .show_line_numbers(true)
            .wrap_lines(false)
            .scroll_offset(25);

        assert_eq!(preview2.title.as_deref(), Some("Stress Test"));
        assert!(preview2.show_line_numbers);
        assert!(!preview2.wrap_lines);
        assert_eq!(preview2.get_scroll_offset(), 25);

        // Phase 4: Unicode content
        preview.set_content("🚀 日本語 مرحبا\n".repeat(100));
        assert_eq!(preview.line_count(), 100);

        // Phase 5: Extreme scrolling
        preview.scroll_down(u16::MAX / 2);
        assert!(preview.get_scroll_offset() > 0);

        preview.scroll_to_top();
        assert_eq!(preview.get_scroll_offset(), 0);

        // Phase 6: Empty content
        preview.set_content("");
        assert_eq!(preview.line_count(), 0);
        assert_eq!(preview.get_scroll_offset(), 0);
    }

    // ============ Line Count Edge Cases ============

    #[test]
    fn test_preview_line_count_trailing_newline() {
        let content = "Line 1\nLine 2\n";
        let preview = PreviewPane::new(content);
        // lines() doesn't count trailing empty line after newline
        assert_eq!(preview.line_count(), 2);
    }

    #[test]
    fn test_preview_line_count_no_trailing_newline() {
        let content = "Line 1\nLine 2";
        let preview = PreviewPane::new(content);
        assert_eq!(preview.line_count(), 2);
    }

    #[test]
    fn test_preview_line_count_windows_newlines() {
        let content = "Line 1\r\nLine 2\r\nLine 3";
        let preview = PreviewPane::new(content);
        // lines() handles both \n and \r\n
        assert_eq!(preview.line_count(), 3);
    }

    // ============ Title Edge Cases ============

    #[test]
    fn test_preview_title_empty() {
        let preview = PreviewPane::new("Content").title("");
        assert_eq!(preview.title.as_deref(), Some(""));
    }

    #[test]
    fn test_preview_title_unicode() {
        let preview = PreviewPane::new("Content").title("日本語 Title 🚀");
        assert_eq!(preview.title.as_deref(), Some("日本語 Title 🚀"));
    }

    #[test]
    fn test_preview_title_very_long() {
        let long_title = "x".repeat(1000);
        let preview = PreviewPane::new("Content").title(long_title.clone());
        assert_eq!(preview.title.as_deref(), Some(long_title.as_str()));
    }
}
