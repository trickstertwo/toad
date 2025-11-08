//! Minimap widget for document overview
//!
//! Provides a zoomed-out view of a document showing structure and current position.
//!
//! # Examples
//!
//! ```
//! use toad::widgets::Minimap;
//!
//! let content = vec!["line 1", "line 2", "line 3"];
//! let minimap = Minimap::new(content)
//!     .with_viewport(0, 10);
//!
//! assert_eq!(minimap.line_count(), 3);
//! ```

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Widget},
};

/// Minimap widget for document overview
///
/// Displays a condensed view of a document with viewport highlighting.
///
/// # Examples
///
/// ```
/// use toad::widgets::Minimap;
///
/// let lines: Vec<&str> = vec!["fn main() {", "    println!()", "}"];
/// let minimap = Minimap::new(lines)
///     .with_viewport(0, 3);
///
/// assert_eq!(minimap.line_count(), 3);
/// ```
#[derive(Debug, Clone)]
pub struct Minimap {
    /// Document lines
    lines: Vec<String>,
    /// Viewport start line
    viewport_start: usize,
    /// Viewport end line
    viewport_end: usize,
    /// Show line numbers
    show_line_numbers: bool,
    /// Highlight viewport
    highlight_viewport: bool,
    /// Character to represent code
    code_char: char,
}

impl Minimap {
    /// Create a new minimap
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Minimap;
    ///
    /// let lines = vec!["line 1", "line 2"];
    /// let minimap = Minimap::new(lines);
    /// assert_eq!(minimap.line_count(), 2);
    /// ```
    pub fn new<S: Into<String>>(lines: Vec<S>) -> Self {
        Self {
            lines: lines.into_iter().map(|s| s.into()).collect(),
            viewport_start: 0,
            viewport_end: 0,
            show_line_numbers: true,
            highlight_viewport: true,
            code_char: '▎',
        }
    }

    /// Set viewport range
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Minimap;
    ///
    /// let lines = vec!["1", "2", "3", "4", "5"];
    /// let minimap = Minimap::new(lines)
    ///     .with_viewport(1, 3);
    /// ```
    pub fn with_viewport(mut self, start: usize, end: usize) -> Self {
        self.viewport_start = start;
        self.viewport_end = end;
        self
    }

    /// Show or hide line numbers
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Minimap;
    ///
    /// let minimap = Minimap::new(vec!["test"])
    ///     .with_line_numbers(false);
    /// ```
    pub fn with_line_numbers(mut self, show: bool) -> Self {
        self.show_line_numbers = show;
        self
    }

    /// Highlight viewport region
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Minimap;
    ///
    /// let minimap = Minimap::new(vec!["test"])
    ///     .with_highlight(false);
    /// ```
    pub fn with_highlight(mut self, highlight: bool) -> Self {
        self.highlight_viewport = highlight;
        self
    }

    /// Set character for code representation
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Minimap;
    ///
    /// let minimap = Minimap::new(vec!["test"])
    ///     .with_code_char('█');
    /// ```
    pub fn with_code_char(mut self, ch: char) -> Self {
        self.code_char = ch;
        self
    }

    /// Get number of lines
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Minimap;
    ///
    /// let minimap = Minimap::new(vec!["line 1", "line 2"]);
    /// assert_eq!(minimap.line_count(), 2);
    /// ```
    pub fn line_count(&self) -> usize {
        self.lines.len()
    }

    /// Update viewport position
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Minimap;
    ///
    /// let mut minimap = Minimap::new(vec!["1", "2", "3"]);
    /// minimap.set_viewport(0, 2);
    /// ```
    pub fn set_viewport(&mut self, start: usize, end: usize) {
        self.viewport_start = start;
        self.viewport_end = end;
    }

    /// Update content
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Minimap;
    ///
    /// let mut minimap = Minimap::new(vec!["old"]);
    /// minimap.set_content(vec!["new", "content"]);
    /// assert_eq!(minimap.line_count(), 2);
    /// ```
    pub fn set_content<S: Into<String>>(&mut self, lines: Vec<S>) {
        self.lines = lines.into_iter().map(|s| s.into()).collect();
    }

    /// Render minimap lines
    fn render_lines(&self, height: u16) -> Vec<Line<'static>> {
        let mut lines = Vec::new();
        let max_lines = height as usize;

        // Calculate step size for sampling
        let step = if self.lines.len() > max_lines {
            self.lines.len() as f64 / max_lines as f64
        } else {
            1.0
        };

        for i in 0..max_lines.min(self.lines.len()) {
            let line_idx = (i as f64 * step) as usize;
            if line_idx >= self.lines.len() {
                break;
            }

            let in_viewport = line_idx >= self.viewport_start && line_idx < self.viewport_end;

            let mut spans = Vec::new();

            // Line number
            if self.show_line_numbers {
                let num = format!("{:3} ", line_idx + 1);
                let style = if in_viewport && self.highlight_viewport {
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::DarkGray)
                };
                spans.push(Span::styled(num, style));
            }

            // Line content representation
            let line = &self.lines[line_idx];
            let content_len = line.trim().len();

            if content_len > 0 {
                let bar = self.code_char.to_string().repeat((content_len / 4).clamp(1, 10));
                let style = if in_viewport && self.highlight_viewport {
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD)
                } else if line.trim().starts_with("fn ") || line.trim().starts_with("pub ")
                    || line.trim().starts_with("struct ") || line.trim().starts_with("impl ")
                {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default().fg(Color::Blue)
                };

                spans.push(Span::styled(bar, style));
            }

            lines.push(Line::from(spans));
        }

        lines
    }
}

impl Widget for &Minimap {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let lines = self.render_lines(area.height);
        let block = Block::default()
            .borders(Borders::ALL)
            .title("Map");
        let inner = block.inner(area);

        block.render(area, buf);

        for (i, line) in lines.iter().enumerate() {
            if i >= inner.height as usize {
                break;
            }
            let y = inner.y + i as u16;
            buf.set_line(inner.x, y, line, inner.width);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimap_new() {
        let lines = vec!["line 1", "line 2"];
        let minimap = Minimap::new(lines);

        assert_eq!(minimap.line_count(), 2);
        assert_eq!(minimap.viewport_start, 0);
        assert_eq!(minimap.viewport_end, 0);
        assert!(minimap.show_line_numbers);
        assert!(minimap.highlight_viewport);
    }

    #[test]
    fn test_minimap_with_viewport() {
        let minimap = Minimap::new(vec!["1", "2", "3"])
            .with_viewport(1, 3);

        assert_eq!(minimap.viewport_start, 1);
        assert_eq!(minimap.viewport_end, 3);
    }

    #[test]
    fn test_minimap_with_line_numbers() {
        let minimap = Minimap::new(vec!["test"])
            .with_line_numbers(false);

        assert!(!minimap.show_line_numbers);
    }

    #[test]
    fn test_minimap_with_highlight() {
        let minimap = Minimap::new(vec!["test"])
            .with_highlight(false);

        assert!(!minimap.highlight_viewport);
    }

    #[test]
    fn test_minimap_with_code_char() {
        let minimap = Minimap::new(vec!["test"])
            .with_code_char('█');

        assert_eq!(minimap.code_char, '█');
    }

    #[test]
    fn test_minimap_line_count() {
        let minimap = Minimap::new(vec!["1", "2", "3", "4"]);
        assert_eq!(minimap.line_count(), 4);
    }

    #[test]
    fn test_minimap_set_viewport() {
        let mut minimap = Minimap::new(vec!["1", "2", "3"]);
        minimap.set_viewport(1, 2);

        assert_eq!(minimap.viewport_start, 1);
        assert_eq!(minimap.viewport_end, 2);
    }

    #[test]
    fn test_minimap_set_content() {
        let mut minimap = Minimap::new(vec!["old"]);
        assert_eq!(minimap.line_count(), 1);

        minimap.set_content(vec!["new1", "new2"]);
        assert_eq!(minimap.line_count(), 2);
    }

    #[test]
    fn test_minimap_render_lines() {
        let minimap = Minimap::new(vec!["line 1", "line 2", "line 3"]);
        let lines = minimap.render_lines(10);

        assert!(!lines.is_empty());
    }

    #[test]
    fn test_minimap_render_lines_sampling() {
        // Test with more lines than height
        let content: Vec<String> = (0..100).map(|i| format!("line {}", i)).collect();
        let minimap = Minimap::new(content);
        let lines = minimap.render_lines(20);

        assert!(lines.len() <= 20);
    }

    #[test]
    fn test_minimap_builder_pattern() {
        let minimap = Minimap::new(vec!["1", "2", "3"])
            .with_viewport(1, 2)
            .with_line_numbers(false)
            .with_highlight(true)
            .with_code_char('█');

        assert_eq!(minimap.line_count(), 3);
        assert_eq!(minimap.viewport_start, 1);
        assert_eq!(minimap.viewport_end, 2);
        assert!(!minimap.show_line_numbers);
        assert!(minimap.highlight_viewport);
        assert_eq!(minimap.code_char, '█');
    }
}
