/// Statusline widget for bottom bar
///
/// Displays app state, mode indicators, help text, and status information
use crate::ui::{atoms::text::Text, theme::ToadTheme};
use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    text::Span,
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
    pub(super) left: Vec<StatusSection>,
    /// Center-aligned sections
    pub(super) center: Vec<StatusSection>,
    /// Right-aligned sections
    pub(super) right: Vec<StatusSection>,
    /// Separator between sections
    pub(super) separator: String,
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

    /// Build a line from sections using Text atoms
    pub(super) fn build_line<'a>(
        &self,
        sections: &'a [StatusSection],
        default_separator: bool,
    ) -> Vec<Span<'static>> {
        let mut spans = Vec::new();

        for (i, section) in sections.iter().enumerate() {
            let style = if section.highlight {
                Style::default()
                    .fg(section.level.color())
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(section.level.color())
            };

            // Use Text atom for section text
            let text = Text::new(&section.text).style(style);
            spans.push(text.to_span());

            // Add separator if not last
            if default_separator && i < sections.len() - 1 {
                let separator =
                    Text::new(&self.separator).style(Style::default().fg(ToadTheme::DARK_GRAY));
                spans.push(separator.to_span());
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

        // Add padding between left and right using Text atoms
        if used < available {
            let padding = available - used - center_width;
            let left_padding = padding / 2;
            let right_padding = padding - left_padding;

            if !center_spans.is_empty() {
                // Add left padding before center
                left_spans.push(Text::new(" ".repeat(left_padding)).to_span());
                left_spans.extend(center_spans);
                left_spans.push(Text::new(" ".repeat(right_padding)).to_span());
            } else {
                // Just pad between left and right
                left_spans.push(Text::new(" ".repeat(available - used)).to_span());
            }
        }

        // Add right section
        left_spans.extend(right_spans);

        let line = ratatui::text::Line::from(left_spans);
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
