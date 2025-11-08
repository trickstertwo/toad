/// Breadcrumbs navigation widget
///
/// Shows hierarchical navigation path with clickable segments

use crate::theme::ToadTheme;
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

/// A single breadcrumb segment
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BreadcrumbSegment {
    /// Label for this segment
    pub label: String,
    /// Optional icon
    pub icon: Option<String>,
    /// Whether this segment is clickable
    pub clickable: bool,
}

impl BreadcrumbSegment {
    /// Create a new breadcrumb segment
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            icon: None,
            clickable: true,
        }
    }

    /// Set icon
    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set clickable
    pub fn with_clickable(mut self, clickable: bool) -> Self {
        self.clickable = clickable;
        self
    }
}

/// Breadcrumbs navigation widget
#[derive(Debug, Clone)]
pub struct Breadcrumbs {
    /// Path segments
    segments: Vec<BreadcrumbSegment>,
    /// Separator between segments
    separator: String,
    /// Currently hovered segment index
    hovered: Option<usize>,
}

impl Breadcrumbs {
    /// Create a new breadcrumbs widget
    pub fn new() -> Self {
        Self {
            segments: Vec::new(),
            separator: " / ".to_string(),
            hovered: None,
        }
    }

    /// Create from path string
    pub fn from_path(path: &str) -> Self {
        let segments = path
            .split('/')
            .filter(|s| !s.is_empty())
            .map(|s| BreadcrumbSegment::new(s))
            .collect();

        Self {
            segments,
            separator: " / ".to_string(),
            hovered: None,
        }
    }

    /// Set segments
    pub fn set_segments(&mut self, segments: Vec<BreadcrumbSegment>) {
        self.segments = segments;
    }

    /// Add a segment
    pub fn push(&mut self, segment: BreadcrumbSegment) {
        self.segments.push(segment);
    }

    /// Remove last segment
    pub fn pop(&mut self) -> Option<BreadcrumbSegment> {
        self.segments.pop()
    }

    /// Get segments
    pub fn segments(&self) -> &[BreadcrumbSegment] {
        &self.segments
    }

    /// Set separator
    pub fn set_separator(&mut self, separator: impl Into<String>) {
        self.separator = separator.into();
    }

    /// Set hovered segment
    pub fn set_hovered(&mut self, index: Option<usize>) {
        self.hovered = index;
    }

    /// Get hovered segment index
    pub fn hovered(&self) -> Option<usize> {
        self.hovered
    }

    /// Clear all segments
    pub fn clear(&mut self) {
        self.segments.clear();
        self.hovered = None;
    }

    /// Render the breadcrumbs
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        if self.segments.is_empty() {
            return;
        }

        let mut spans = Vec::new();

        for (i, segment) in self.segments.iter().enumerate() {
            // Add icon if present
            if let Some(icon) = &segment.icon {
                spans.push(Span::styled(
                    format!("{} ", icon),
                    Style::default().fg(ToadTheme::GRAY),
                ));
            }

            // Add label
            let is_last = i == self.segments.len() - 1;
            let is_hovered = self.hovered == Some(i);

            let style = if is_last {
                // Last segment is highlighted
                Style::default()
                    .fg(ToadTheme::TOAD_GREEN)
                    .add_modifier(Modifier::BOLD)
            } else if is_hovered && segment.clickable {
                // Hovered clickable segment
                Style::default()
                    .fg(ToadTheme::TOAD_GREEN_BRIGHT)
                    .add_modifier(Modifier::UNDERLINED)
            } else if segment.clickable {
                // Regular clickable segment
                Style::default().fg(ToadTheme::BLUE)
            } else {
                // Non-clickable segment
                Style::default().fg(ToadTheme::GRAY)
            };

            spans.push(Span::styled(&segment.label, style));

            // Add separator if not last
            if !is_last {
                spans.push(Span::styled(
                    &self.separator,
                    Style::default().fg(ToadTheme::DARK_GRAY),
                ));
            }
        }

        let paragraph = Paragraph::new(Line::from(spans));
        frame.render_widget(paragraph, area);
    }
}

impl Default for Breadcrumbs {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breadcrumbs_creation() {
        let breadcrumbs = Breadcrumbs::new();
        assert_eq!(breadcrumbs.segments().len(), 0);
    }

    #[test]
    fn test_from_path() {
        let breadcrumbs = Breadcrumbs::from_path("/home/user/projects");
        assert_eq!(breadcrumbs.segments().len(), 3);
        assert_eq!(breadcrumbs.segments()[0].label, "home");
        assert_eq!(breadcrumbs.segments()[1].label, "user");
        assert_eq!(breadcrumbs.segments()[2].label, "projects");
    }

    #[test]
    fn test_push_pop() {
        let mut breadcrumbs = Breadcrumbs::new();
        breadcrumbs.push(BreadcrumbSegment::new("root"));
        breadcrumbs.push(BreadcrumbSegment::new("folder"));

        assert_eq!(breadcrumbs.segments().len(), 2);

        let popped = breadcrumbs.pop();
        assert_eq!(popped.unwrap().label, "folder");
        assert_eq!(breadcrumbs.segments().len(), 1);
    }

    #[test]
    fn test_hover() {
        let mut breadcrumbs = Breadcrumbs::new();
        breadcrumbs.push(BreadcrumbSegment::new("a"));
        breadcrumbs.push(BreadcrumbSegment::new("b"));

        assert_eq!(breadcrumbs.hovered(), None);

        breadcrumbs.set_hovered(Some(1));
        assert_eq!(breadcrumbs.hovered(), Some(1));
    }

    #[test]
    fn test_clear() {
        let mut breadcrumbs = Breadcrumbs::from_path("/a/b/c");
        assert_eq!(breadcrumbs.segments().len(), 3);

        breadcrumbs.clear();
        assert_eq!(breadcrumbs.segments().len(), 0);
    }

    #[test]
    fn test_segment_with_icon() {
        let segment = BreadcrumbSegment::new("Home")
            .with_icon("🏠")
            .with_clickable(false);

        assert_eq!(segment.label, "Home");
        assert_eq!(segment.icon, Some("🏠".to_string()));
        assert!(!segment.clickable);
    }
}
