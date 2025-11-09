/// Statusline widget for bottom bar
///
/// Displays app state, mode indicators, help text, and status information
use crate::ui::theme::ToadTheme;
use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

/// Status level for styling
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusLevel {
    /// Normal status
    Normal,
    /// Info message
    Info,
    /// Warning message
    Warning,
    /// Error message
    Error,
    /// Success message
    Success,
}

impl StatusLevel {
    /// Get the color for this status level
    pub fn color(&self) -> ratatui::style::Color {
        match self {
            StatusLevel::Normal => ToadTheme::FOREGROUND,
            StatusLevel::Info => ToadTheme::BLUE,
            StatusLevel::Warning => ToadTheme::YELLOW,
            StatusLevel::Error => ToadTheme::RED,
            StatusLevel::Success => ToadTheme::TOAD_GREEN,
        }
    }
}

/// A section in the statusline
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatusSection {
    /// Text content
    pub text: String,
    /// Status level for styling
    pub level: StatusLevel,
    /// Whether to highlight this section
    pub highlight: bool,
}

impl StatusSection {
    /// Create a new status section
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            level: StatusLevel::Normal,
            highlight: false,
        }
    }

    /// Set the status level
    pub fn with_level(mut self, level: StatusLevel) -> Self {
        self.level = level;
        self
    }

    /// Set highlight
    pub fn with_highlight(mut self, highlight: bool) -> Self {
        self.highlight = highlight;
        self
    }
}

/// Alignment for status sections
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SectionAlignment {
    /// Left-aligned
    Left,
    /// Center-aligned
    Center,
    /// Right-aligned
    Right,
}

/// Statusline widget
#[derive(Debug, Clone)]
pub struct Statusline {
    /// Left-aligned sections
    left: Vec<StatusSection>,
    /// Center-aligned sections
    center: Vec<StatusSection>,
    /// Right-aligned sections
    right: Vec<StatusSection>,
    /// Separator between sections
    separator: String,
}

impl Statusline {
    /// Create a new statusline
    pub fn new() -> Self {
        Self {
            left: Vec::new(),
            center: Vec::new(),
            right: Vec::new(),
            separator: " │ ".to_string(),
        }
    }

    /// Add a section to the left
    pub fn add_left(&mut self, section: StatusSection) {
        self.left.push(section);
    }

    /// Add a section to the center
    pub fn add_center(&mut self, section: StatusSection) {
        self.center.push(section);
    }

    /// Add a section to the right
    pub fn add_right(&mut self, section: StatusSection) {
        self.right.push(section);
    }

    /// Set sections by alignment
    pub fn set_sections(&mut self, alignment: SectionAlignment, sections: Vec<StatusSection>) {
        match alignment {
            SectionAlignment::Left => self.left = sections,
            SectionAlignment::Center => self.center = sections,
            SectionAlignment::Right => self.right = sections,
        }
    }

    /// Get sections by alignment
    pub fn sections(&self, alignment: SectionAlignment) -> &[StatusSection] {
        match alignment {
            SectionAlignment::Left => &self.left,
            SectionAlignment::Center => &self.center,
            SectionAlignment::Right => &self.right,
        }
    }

    /// Set separator
    pub fn set_separator(&mut self, separator: impl Into<String>) {
        self.separator = separator.into();
    }

    /// Clear all sections
    pub fn clear(&mut self) {
        self.left.clear();
        self.center.clear();
        self.right.clear();
    }

    /// Clear sections by alignment
    pub fn clear_alignment(&mut self, alignment: SectionAlignment) {
        match alignment {
            SectionAlignment::Left => self.left.clear(),
            SectionAlignment::Center => self.center.clear(),
            SectionAlignment::Right => self.right.clear(),
        }
    }

    /// Build a line from sections
    fn build_line<'a>(
        &self,
        sections: &'a [StatusSection],
        default_separator: bool,
    ) -> Vec<Span<'a>> {
        let mut spans = Vec::new();

        for (i, section) in sections.iter().enumerate() {
            let style = if section.highlight {
                Style::default()
                    .fg(section.level.color())
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(section.level.color())
            };

            spans.push(Span::styled(&section.text, style));

            // Add separator if not last
            if default_separator && i < sections.len() - 1 {
                spans.push(Span::styled(
                    self.separator.clone(),
                    Style::default().fg(ToadTheme::DARK_GRAY),
                ));
            }
        }

        spans
    }

    /// Render the statusline
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        // Build left section
        let mut left_spans = self.build_line(&self.left, true);

        // Build right section
        let right_spans = self.build_line(&self.right, true);

        // Build center section
        let center_spans = self.build_line(&self.center, true);

        // Calculate padding for center alignment
        let left_width: usize = left_spans.iter().map(|s| s.content.len()).sum();
        let right_width: usize = right_spans.iter().map(|s| s.content.len()).sum();
        let center_width: usize = center_spans.iter().map(|s| s.content.len()).sum();

        let available = area.width as usize;
        let used = left_width + right_width;

        // Add padding between left and right
        if used < available {
            let padding = available - used - center_width;
            let left_padding = padding / 2;
            let right_padding = padding - left_padding;

            if !center_spans.is_empty() {
                // Add left padding before center
                left_spans.push(Span::raw(" ".repeat(left_padding)));
                left_spans.extend(center_spans);
                left_spans.push(Span::raw(" ".repeat(right_padding)));
            } else {
                // Just pad between left and right
                left_spans.push(Span::raw(" ".repeat(available - used)));
            }
        }

        // Add right section
        left_spans.extend(right_spans);

        let line = Line::from(left_spans);
        let paragraph = Paragraph::new(line)
            .style(Style::default().bg(ToadTheme::DARK_GRAY))
            .alignment(Alignment::Left);

        frame.render_widget(paragraph, area);
    }
}

impl Default for Statusline {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_level_colors() {
        assert_eq!(StatusLevel::Normal.color(), ToadTheme::FOREGROUND);
        assert_eq!(StatusLevel::Info.color(), ToadTheme::BLUE);
        assert_eq!(StatusLevel::Warning.color(), ToadTheme::YELLOW);
        assert_eq!(StatusLevel::Error.color(), ToadTheme::RED);
        assert_eq!(StatusLevel::Success.color(), ToadTheme::TOAD_GREEN);
    }

    #[test]
    fn test_status_section_creation() {
        let section = StatusSection::new("Test")
            .with_level(StatusLevel::Info)
            .with_highlight(true);

        assert_eq!(section.text, "Test");
        assert_eq!(section.level, StatusLevel::Info);
        assert!(section.highlight);
    }

    #[test]
    fn test_statusline_creation() {
        let statusline = Statusline::new();
        assert!(statusline.left.is_empty());
        assert!(statusline.center.is_empty());
        assert!(statusline.right.is_empty());
    }

    #[test]
    fn test_statusline_add_sections() {
        let mut statusline = Statusline::new();

        statusline.add_left(StatusSection::new("Left"));
        statusline.add_center(StatusSection::new("Center"));
        statusline.add_right(StatusSection::new("Right"));

        assert_eq!(statusline.left.len(), 1);
        assert_eq!(statusline.center.len(), 1);
        assert_eq!(statusline.right.len(), 1);
    }

    #[test]
    fn test_statusline_set_sections() {
        let mut statusline = Statusline::new();

        let sections = vec![
            StatusSection::new("A"),
            StatusSection::new("B"),
            StatusSection::new("C"),
        ];

        statusline.set_sections(SectionAlignment::Left, sections.clone());
        assert_eq!(statusline.sections(SectionAlignment::Left).len(), 3);

        statusline.set_sections(SectionAlignment::Center, sections.clone());
        assert_eq!(statusline.sections(SectionAlignment::Center).len(), 3);

        statusline.set_sections(SectionAlignment::Right, sections.clone());
        assert_eq!(statusline.sections(SectionAlignment::Right).len(), 3);
    }

    #[test]
    fn test_statusline_clear() {
        let mut statusline = Statusline::new();

        statusline.add_left(StatusSection::new("L"));
        statusline.add_center(StatusSection::new("C"));
        statusline.add_right(StatusSection::new("R"));

        statusline.clear();

        assert!(statusline.left.is_empty());
        assert!(statusline.center.is_empty());
        assert!(statusline.right.is_empty());
    }

    #[test]
    fn test_statusline_clear_alignment() {
        let mut statusline = Statusline::new();

        statusline.add_left(StatusSection::new("L"));
        statusline.add_center(StatusSection::new("C"));
        statusline.add_right(StatusSection::new("R"));

        statusline.clear_alignment(SectionAlignment::Left);

        assert!(statusline.left.is_empty());
        assert_eq!(statusline.center.len(), 1);
        assert_eq!(statusline.right.len(), 1);
    }

    #[test]
    fn test_statusline_separator() {
        let mut statusline = Statusline::new();
        assert_eq!(statusline.separator, " │ ");

        statusline.set_separator(" | ");
        assert_eq!(statusline.separator, " | ");
    }

    #[test]
    fn test_build_line() {
        let statusline = Statusline::new();

        let sections = vec![
            StatusSection::new("A").with_level(StatusLevel::Info),
            StatusSection::new("B").with_level(StatusLevel::Warning),
        ];

        let spans = statusline.build_line(&sections, true);

        // Should have: "A", separator, "B"
        assert_eq!(spans.len(), 3);
        assert_eq!(spans[0].content, "A");
        assert_eq!(spans[1].content, " │ ");
        assert_eq!(spans[2].content, "B");
    }

    #[test]
    fn test_build_line_no_separator() {
        let statusline = Statusline::new();

        let sections = vec![StatusSection::new("A"), StatusSection::new("B")];

        let spans = statusline.build_line(&sections, false);

        // Should have: "A", "B" (no separator)
        assert_eq!(spans.len(), 2);
        assert_eq!(spans[0].content, "A");
        assert_eq!(spans[1].content, "B");
    }

    // ========================================================================
    // EDGE CASE TESTS (Added for comprehensive coverage)
    // ========================================================================

    #[test]
    fn test_status_section_very_long_text() {
        let long_text = "x".repeat(1000);
        let section = StatusSection::new(long_text.clone());
        assert_eq!(section.text.len(), 1000);
        assert_eq!(section.text, long_text);
    }

    #[test]
    fn test_status_section_with_unicode() {
        let section = StatusSection::new("🐸 日本語 Status");
        assert!(section.text.contains('🐸'));
        assert!(section.text.contains("日本語"));
    }

    #[test]
    fn test_status_section_empty_text() {
        let section = StatusSection::new("");
        assert_eq!(section.text, "");
        assert_eq!(section.level, StatusLevel::Normal);
    }

    #[test]
    fn test_status_section_with_newlines() {
        let section = StatusSection::new("Line1\nLine2\nLine3");
        assert!(section.text.contains('\n'));
        // Note: statusline typically doesn't handle newlines specially
    }

    #[test]
    fn test_statusline_many_sections() {
        let mut statusline = Statusline::new();

        // Add 50 sections to left
        for i in 0..50 {
            statusline.add_left(StatusSection::new(format!("Section {}", i)));
        }

        assert_eq!(statusline.left.len(), 50);
    }

    #[test]
    fn test_statusline_default_implementation() {
        let default_statusline = Statusline::default();
        let new_statusline = Statusline::new();

        assert_eq!(default_statusline.left.len(), new_statusline.left.len());
        assert_eq!(default_statusline.separator, new_statusline.separator);
    }

    #[test]
    fn test_statusline_separator_with_unicode() {
        let mut statusline = Statusline::new();
        statusline.set_separator(" 🐸 ");
        assert_eq!(statusline.separator, " 🐸 ");
    }

    #[test]
    fn test_statusline_separator_empty() {
        let mut statusline = Statusline::new();
        statusline.set_separator("");
        assert_eq!(statusline.separator, "");
    }

    #[test]
    fn test_status_section_highlight_default() {
        let section = StatusSection::new("Test");
        assert!(!section.highlight, "Highlight should be false by default");
    }

    #[test]
    fn test_status_section_level_default() {
        let section = StatusSection::new("Test");
        assert_eq!(section.level, StatusLevel::Normal, "Level should be Normal by default");
    }

    #[test]
    fn test_status_level_all_variants() {
        // Ensure all status level variants work
        let levels = vec![
            StatusLevel::Normal,
            StatusLevel::Info,
            StatusLevel::Warning,
            StatusLevel::Error,
            StatusLevel::Success,
        ];

        for level in levels {
            // Should not panic
            let _color = level.color();
        }
    }

    #[test]
    fn test_section_alignment_all_variants() {
        // Ensure all alignment variants work
        let alignments = vec![
            SectionAlignment::Left,
            SectionAlignment::Center,
            SectionAlignment::Right,
        ];

        let mut statusline = Statusline::new();
        for alignment in alignments {
            statusline.clear_alignment(alignment);
            assert_eq!(statusline.sections(alignment).len(), 0);
        }
    }

    #[test]
    fn test_statusline_mixed_levels() {
        let mut statusline = Statusline::new();

        statusline.add_left(StatusSection::new("Normal").with_level(StatusLevel::Normal));
        statusline.add_left(StatusSection::new("Info").with_level(StatusLevel::Info));
        statusline.add_left(StatusSection::new("Warning").with_level(StatusLevel::Warning));
        statusline.add_left(StatusSection::new("Error").with_level(StatusLevel::Error));
        statusline.add_left(StatusSection::new("Success").with_level(StatusLevel::Success));

        assert_eq!(statusline.left.len(), 5);
        assert_eq!(statusline.left[0].level, StatusLevel::Normal);
        assert_eq!(statusline.left[1].level, StatusLevel::Info);
        assert_eq!(statusline.left[2].level, StatusLevel::Warning);
        assert_eq!(statusline.left[3].level, StatusLevel::Error);
        assert_eq!(statusline.left[4].level, StatusLevel::Success);
    }

    #[test]
    fn test_build_line_single_section() {
        let statusline = Statusline::new();
        let sections = vec![StatusSection::new("Single")];

        let spans = statusline.build_line(&sections, true);

        // Single section = no separator needed
        assert_eq!(spans.len(), 1);
        assert_eq!(spans[0].content, "Single");
    }

    #[test]
    fn test_build_line_empty_sections() {
        let statusline = Statusline::new();
        let sections = vec![];

        let spans = statusline.build_line(&sections, true);

        assert_eq!(spans.len(), 0, "Empty sections should produce empty spans");
    }

    #[test]
    fn test_statusline_clear_preserves_separator() {
        let mut statusline = Statusline::new();
        statusline.set_separator(" | ");

        statusline.add_left(StatusSection::new("Test"));
        statusline.clear();

        assert_eq!(statusline.separator, " | ", "Clear should preserve separator");
    }
}
