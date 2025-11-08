//! Breadcrumbs widget for navigation trails
//!
//! Displays a navigation path showing the current location in a hierarchy.
//!
//! # Examples
//!
//! ```
//! use toad::widgets::Breadcrumbs;
//!
//! let breadcrumbs = Breadcrumbs::new()
//!     .add("Home")
//!     .add("Projects")
//!     .add("Toad")
//!     .add("src");
//!
//! assert_eq!(breadcrumbs.path_count(), 4);
//! ```

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Widget},
};

/// Breadcrumbs widget for navigation trails
///
/// Displays the current location in a hierarchy as a path.
///
/// # Examples
///
/// ```
/// use toad::widgets::Breadcrumbs;
///
/// let breadcrumbs = Breadcrumbs::new()
///     .add("Home")
///     .add("Documents")
///     .add("file.txt");
///
/// assert_eq!(breadcrumbs.current(), Some("file.txt"));
/// ```
#[derive(Debug, Clone)]
pub struct Breadcrumbs {
    /// Path components
    path: Vec<String>,
    /// Separator character
    separator: String,
    /// Show icons for path types
    show_icons: bool,
    /// Truncate long paths
    truncate: bool,
    /// Maximum path length before truncation
    max_length: usize,
}

impl Default for Breadcrumbs {
    fn default() -> Self {
        Self::new()
    }
}

impl Breadcrumbs {
    /// Create new breadcrumbs
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Breadcrumbs;
    ///
    /// let breadcrumbs = Breadcrumbs::new();
    /// assert_eq!(breadcrumbs.path_count(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            path: Vec::new(),
            separator: " > ".to_string(),
            show_icons: true,
            truncate: true,
            max_length: 50,
        }
    }

    /// Add a path component
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Breadcrumbs;
    ///
    /// let breadcrumbs = Breadcrumbs::new()
    ///     .add("Home")
    ///     .add("Projects");
    ///
    /// assert_eq!(breadcrumbs.path_count(), 2);
    /// ```
    #[allow(clippy::should_implement_trait)]
    pub fn add(mut self, component: impl Into<String>) -> Self {
        self.path.push(component.into());
        self
    }

    /// Set separator
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Breadcrumbs;
    ///
    /// let breadcrumbs = Breadcrumbs::new()
    ///     .with_separator(" / ");
    /// ```
    pub fn with_separator(mut self, separator: impl Into<String>) -> Self {
        self.separator = separator.into();
        self
    }

    /// Show or hide icons
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Breadcrumbs;
    ///
    /// let breadcrumbs = Breadcrumbs::new()
    ///     .with_icons(false);
    /// ```
    pub fn with_icons(mut self, show: bool) -> Self {
        self.show_icons = show;
        self
    }

    /// Enable or disable truncation
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Breadcrumbs;
    ///
    /// let breadcrumbs = Breadcrumbs::new()
    ///     .with_truncate(false);
    /// ```
    pub fn with_truncate(mut self, truncate: bool) -> Self {
        self.truncate = truncate;
        self
    }

    /// Set maximum length before truncation
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Breadcrumbs;
    ///
    /// let breadcrumbs = Breadcrumbs::new()
    ///     .with_max_length(100);
    /// ```
    pub fn with_max_length(mut self, max: usize) -> Self {
        self.max_length = max;
        self
    }

    /// Clear all path components
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Breadcrumbs;
    ///
    /// let mut breadcrumbs = Breadcrumbs::new()
    ///     .add("Home");
    ///
    /// breadcrumbs.clear();
    /// assert_eq!(breadcrumbs.path_count(), 0);
    /// ```
    pub fn clear(&mut self) {
        self.path.clear();
    }

    /// Get number of path components
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Breadcrumbs;
    ///
    /// let breadcrumbs = Breadcrumbs::new()
    ///     .add("Home")
    ///     .add("Projects");
    ///
    /// assert_eq!(breadcrumbs.path_count(), 2);
    /// ```
    pub fn path_count(&self) -> usize {
        self.path.len()
    }

    /// Get current (last) component
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Breadcrumbs;
    ///
    /// let breadcrumbs = Breadcrumbs::new()
    ///     .add("Home")
    ///     .add("file.txt");
    ///
    /// assert_eq!(breadcrumbs.current(), Some("file.txt"));
    /// ```
    pub fn current(&self) -> Option<&str> {
        self.path.last().map(|s| s.as_str())
    }

    /// Get all path components
    pub fn path(&self) -> &[String] {
        &self.path
    }

    /// Render breadcrumbs as a line
    pub fn render_line(&self) -> Line<'static> {
        if self.path.is_empty() {
            return Line::from("");
        }

        let mut spans = Vec::new();
        let total_len = self.path.iter().map(|s| s.len()).sum::<usize>()
            + (self.path.len() - 1) * self.separator.len();

        // Check if truncation is needed
        let should_truncate = self.truncate && total_len > self.max_length && self.path.len() > 2;

        if should_truncate {
            // Show first and last components with "..." in between
            if let Some(first) = self.path.first() {
                if self.show_icons {
                    spans.push(Span::styled("📁 ", Style::default().fg(Color::Blue)));
                }
                spans.push(Span::styled(
                    first.clone(),
                    Style::default().fg(Color::Gray),
                ));
            }

            spans.push(Span::raw(self.separator.clone()));
            spans.push(Span::styled("...", Style::default().fg(Color::DarkGray)));
            spans.push(Span::raw(self.separator.clone()));

            if let Some(last) = self.path.last() {
                if self.show_icons {
                    spans.push(Span::styled("📄 ", Style::default().fg(Color::Yellow)));
                }
                spans.push(Span::styled(
                    last.clone(),
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ));
            }
        } else {
            // Show full path
            for (i, component) in self.path.iter().enumerate() {
                if i > 0 {
                    spans.push(Span::raw(self.separator.clone()));
                }

                if self.show_icons {
                    let icon = if i == self.path.len() - 1 {
                        "📄 "
                    } else {
                        "📁 "
                    };
                    let color = if i == self.path.len() - 1 {
                        Color::Yellow
                    } else {
                        Color::Blue
                    };
                    spans.push(Span::styled(icon.to_string(), Style::default().fg(color)));
                }

                let style = if i == self.path.len() - 1 {
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::Gray)
                };

                spans.push(Span::styled(component.clone(), style));
            }
        }

        Line::from(spans)
    }
}

impl Widget for &Breadcrumbs {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let line = self.render_line();
        let block = Block::default().borders(Borders::NONE);
        let inner = block.inner(area);

        block.render(area, buf);
        buf.set_line(inner.x, inner.y, &line, inner.width);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breadcrumbs_new() {
        let breadcrumbs = Breadcrumbs::new();
        assert_eq!(breadcrumbs.path_count(), 0);
        assert_eq!(breadcrumbs.separator, " > ");
        assert!(breadcrumbs.show_icons);
        assert!(breadcrumbs.truncate);
    }

    #[test]
    fn test_breadcrumbs_default() {
        let breadcrumbs = Breadcrumbs::default();
        assert_eq!(breadcrumbs.path_count(), 0);
    }

    #[test]
    fn test_breadcrumbs_add() {
        let breadcrumbs = Breadcrumbs::new()
            .add("Home")
            .add("Projects")
            .add("Toad");

        assert_eq!(breadcrumbs.path_count(), 3);
        assert_eq!(breadcrumbs.current(), Some("Toad"));
    }

    #[test]
    fn test_breadcrumbs_with_separator() {
        let breadcrumbs = Breadcrumbs::new()
            .with_separator(" / ");

        assert_eq!(breadcrumbs.separator, " / ");
    }

    #[test]
    fn test_breadcrumbs_with_icons() {
        let breadcrumbs = Breadcrumbs::new()
            .with_icons(false);

        assert!(!breadcrumbs.show_icons);
    }

    #[test]
    fn test_breadcrumbs_with_truncate() {
        let breadcrumbs = Breadcrumbs::new()
            .with_truncate(false);

        assert!(!breadcrumbs.truncate);
    }

    #[test]
    fn test_breadcrumbs_with_max_length() {
        let breadcrumbs = Breadcrumbs::new()
            .with_max_length(100);

        assert_eq!(breadcrumbs.max_length, 100);
    }

    #[test]
    fn test_breadcrumbs_clear() {
        let mut breadcrumbs = Breadcrumbs::new()
            .add("Home")
            .add("Projects");

        assert_eq!(breadcrumbs.path_count(), 2);
        breadcrumbs.clear();
        assert_eq!(breadcrumbs.path_count(), 0);
    }

    #[test]
    fn test_breadcrumbs_current() {
        let breadcrumbs = Breadcrumbs::new()
            .add("Home")
            .add("file.txt");

        assert_eq!(breadcrumbs.current(), Some("file.txt"));

        let empty = Breadcrumbs::new();
        assert_eq!(empty.current(), None);
    }

    #[test]
    fn test_breadcrumbs_path() {
        let breadcrumbs = Breadcrumbs::new()
            .add("Home")
            .add("Projects");

        let path = breadcrumbs.path();
        assert_eq!(path.len(), 2);
        assert_eq!(path[0], "Home");
        assert_eq!(path[1], "Projects");
    }

    #[test]
    fn test_breadcrumbs_render_line_empty() {
        let breadcrumbs = Breadcrumbs::new();
        let line = breadcrumbs.render_line();
        assert!(line.spans.is_empty() || line.spans[0].content.is_empty());
    }

    #[test]
    fn test_breadcrumbs_render_line() {
        let breadcrumbs = Breadcrumbs::new()
            .add("Home")
            .add("Projects");

        let line = breadcrumbs.render_line();
        assert!(!line.spans.is_empty());
    }

    #[test]
    fn test_breadcrumbs_builder_pattern() {
        let breadcrumbs = Breadcrumbs::new()
            .add("Home")
            .add("Projects")
            .with_separator(" / ")
            .with_icons(false)
            .with_truncate(true)
            .with_max_length(80);

        assert_eq!(breadcrumbs.path_count(), 2);
        assert_eq!(breadcrumbs.separator, " / ");
        assert!(!breadcrumbs.show_icons);
        assert!(breadcrumbs.truncate);
        assert_eq!(breadcrumbs.max_length, 80);
    }
}
