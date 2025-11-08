/// Minimap widget for document overview (VSCode-style)
///
/// Provides a scaled-down view of the entire document for quick navigation
///
/// # Examples
///
/// ```
/// use toad::widgets::Minimap;
///
/// let content = vec!["line 1", "line 2", "line 3"];
/// let minimap = Minimap::new(content);
/// assert_eq!(minimap.line_count(), 3);
/// ```
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};
use serde::{Deserialize, Serialize};

use crate::ui::theme::ToadTheme;

/// Minimap display mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MinimapMode {
    /// Show characters (scaled down)
    Characters,
    /// Show blocks/density
    Blocks,
    /// Show syntax highlighting colors
    Colors,
}

impl Default for MinimapMode {
    fn default() -> Self {
        MinimapMode::Blocks
    }
}

/// Minimap widget
#[derive(Debug, Clone)]
pub struct Minimap {
    /// Document lines
    lines: Vec<String>,
    /// Current viewport start line
    viewport_start: usize,
    /// Current viewport end line
    viewport_end: usize,
    /// Display mode
    mode: MinimapMode,
    /// Show border
    show_border: bool,
    /// Scale factor (chars per minimap cell)
    scale: usize,
}

impl Minimap {
    /// Create a new minimap
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Minimap;
    ///
    /// let lines = vec!["fn main() {", "    println!(\"Hello\");", "}"];
    /// let minimap = Minimap::new(lines);
    /// assert_eq!(minimap.line_count(), 3);
    /// ```
    pub fn new<S: Into<String>>(lines: Vec<S>) -> Self {
        let lines: Vec<String> = lines.into_iter().map(|s| s.into()).collect();

        Self {
            viewport_start: 0,
            viewport_end: lines.len().min(20),
            lines,
            mode: MinimapMode::Blocks,
            show_border: true,
            scale: 4,
        }
    }

    /// Set display mode
    pub fn with_mode(mut self, mode: MinimapMode) -> Self {
        self.mode = mode;
        self
    }

    /// Set whether to show border
    pub fn with_border(mut self, show: bool) -> Self {
        self.show_border = show;
        self
    }

    /// Set scale factor
    pub fn with_scale(mut self, scale: usize) -> Self {
        self.scale = scale.max(1);
        self
    }

    /// Set viewport
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Minimap;
    ///
    /// let mut minimap = Minimap::new(vec!["a", "b", "c", "d", "e"]);
    /// minimap.set_viewport(1, 3);
    /// assert_eq!(minimap.viewport(), (1, 3));
    /// ```
    pub fn set_viewport(&mut self, start: usize, end: usize) {
        self.viewport_start = start.min(self.lines.len());
        self.viewport_end = end.min(self.lines.len());
    }

    /// Get viewport
    pub fn viewport(&self) -> (usize, usize) {
        (self.viewport_start, self.viewport_end)
    }

    /// Get line count
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Minimap;
    ///
    /// let minimap = Minimap::new(vec!["a", "b", "c"]);
    /// assert_eq!(minimap.line_count(), 3);
    /// ```
    pub fn line_count(&self) -> usize {
        self.lines.len()
    }

    /// Set lines
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Minimap;
    ///
    /// let mut minimap = Minimap::new(vec!["a", "b"]);
    /// minimap.set_lines(vec!["x", "y", "z"]);
    /// assert_eq!(minimap.line_count(), 3);
    /// ```
    pub fn set_lines<S: Into<String>>(&mut self, lines: Vec<S>) {
        self.lines = lines.into_iter().map(|s| s.into()).collect();
        self.viewport_end = self.lines.len().min(self.viewport_end);
    }

    /// Scroll viewport
    pub fn scroll(&mut self, delta: isize) {
        let viewport_size = self.viewport_end.saturating_sub(self.viewport_start);

        if delta > 0 {
            let new_start = self.viewport_start.saturating_add(delta as usize);
            if new_start + viewport_size <= self.lines.len() {
                self.viewport_start = new_start;
                self.viewport_end = new_start + viewport_size;
            } else {
                // Clamp to maximum valid position
                self.viewport_end = self.lines.len();
                self.viewport_start = self.viewport_end.saturating_sub(viewport_size);
            }
        } else if delta < 0 {
            let sub = (-delta) as usize;
            self.viewport_start = self.viewport_start.saturating_sub(sub);
            self.viewport_end = self.viewport_start + viewport_size;
        }
    }

    /// Get line density (0.0 = empty, 1.0 = full)
    fn line_density(&self, line_idx: usize) -> f32 {
        if let Some(line) = self.lines.get(line_idx) {
            let non_whitespace = line.chars().filter(|c| !c.is_whitespace()).count();
            let total = line.len().max(1);
            non_whitespace as f32 / total as f32
        } else {
            0.0
        }
    }

    /// Get block character for density
    fn density_char(density: f32) -> char {
        if density < 0.1 {
            ' '
        } else if density < 0.3 {
            '░'
        } else if density < 0.6 {
            '▒'
        } else if density < 0.9 {
            '▓'
        } else {
            '█'
        }
    }

    /// Get color for line (simplified syntax highlighting)
    fn line_color(&self, line_idx: usize) -> Color {
        if let Some(line) = self.lines.get(line_idx) {
            let trimmed = line.trim();
            if trimmed.starts_with("//") || trimmed.starts_with('#') {
                ToadTheme::GRAY // Comment
            } else if trimmed.starts_with("fn ")
                || trimmed.starts_with("pub ")
                || trimmed.starts_with("struct ")
                || trimmed.starts_with("impl ")
            {
                ToadTheme::TOAD_GREEN // Keywords
            } else if trimmed.starts_with('"') || trimmed.contains("\"") {
                ToadTheme::YELLOW // Strings
            } else if !trimmed.is_empty() {
                ToadTheme::LIGHT_GRAY // Code
            } else {
                ToadTheme::DARKER_GRAY // Empty
            }
        } else {
            ToadTheme::DARKER_GRAY
        }
    }

    /// Render the minimap
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let mut lines_to_render: Vec<Line> = Vec::new();

        for (idx, line) in self.lines.iter().enumerate() {
            let is_in_viewport = idx >= self.viewport_start && idx < self.viewport_end;

            let line_text = match self.mode {
                MinimapMode::Characters => {
                    // Show scaled-down characters
                    let chars: String = line
                        .chars()
                        .step_by(self.scale)
                        .take(area.width as usize)
                        .collect();
                    chars
                }
                MinimapMode::Blocks => {
                    // Show density blocks
                    let density = self.line_density(idx);
                    let char = Self::density_char(density);
                    char.to_string()
                }
                MinimapMode::Colors => {
                    // Show colored blocks
                    "█".to_string()
                }
            };

            let style = if is_in_viewport {
                Style::default()
                    .fg(ToadTheme::TOAD_GREEN)
                    .add_modifier(Modifier::BOLD)
            } else {
                match self.mode {
                    MinimapMode::Colors => Style::default().fg(self.line_color(idx)),
                    _ => Style::default().fg(ToadTheme::GRAY),
                }
            };

            let span = Span::styled(line_text, style);
            lines_to_render.push(Line::from(vec![span]));
        }

        let paragraph = if self.show_border {
            Paragraph::new(lines_to_render)
                .block(Block::default().borders(Borders::ALL).title("Minimap"))
                .wrap(Wrap { trim: false })
        } else {
            Paragraph::new(lines_to_render).wrap(Wrap { trim: false })
        };

        frame.render_widget(paragraph, area);
    }

    /// Get lines in viewport
    pub fn viewport_lines(&self) -> Vec<&str> {
        self.lines[self.viewport_start..self.viewport_end]
            .iter()
            .map(|s| s.as_str())
            .collect()
    }

    /// Jump to line (updates viewport)
    pub fn jump_to(&mut self, line: usize) {
        let viewport_size = self.viewport_end.saturating_sub(self.viewport_start);
        let half_viewport = viewport_size / 2;

        if line < half_viewport {
            self.viewport_start = 0;
            self.viewport_end = viewport_size.min(self.lines.len());
        } else if line + half_viewport >= self.lines.len() {
            self.viewport_end = self.lines.len();
            self.viewport_start = self.viewport_end.saturating_sub(viewport_size);
        } else {
            self.viewport_start = line.saturating_sub(half_viewport);
            self.viewport_end = (self.viewport_start + viewport_size).min(self.lines.len());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimap_creation() {
        let minimap = Minimap::new(vec!["line1", "line2", "line3"]);
        assert_eq!(minimap.line_count(), 3);
        assert_eq!(minimap.viewport(), (0, 3));
    }

    #[test]
    fn test_minimap_with_mode() {
        let minimap = Minimap::new(vec!["test"]).with_mode(MinimapMode::Characters);
        assert_eq!(minimap.mode, MinimapMode::Characters);
    }

    #[test]
    fn test_minimap_with_border() {
        let minimap = Minimap::new(vec!["test"]).with_border(false);
        assert!(!minimap.show_border);
    }

    #[test]
    fn test_minimap_with_scale() {
        let minimap = Minimap::new(vec!["test"]).with_scale(8);
        assert_eq!(minimap.scale, 8);
    }

    #[test]
    fn test_minimap_set_viewport() {
        let mut minimap = Minimap::new(vec!["a", "b", "c", "d", "e"]);
        minimap.set_viewport(1, 3);
        assert_eq!(minimap.viewport(), (1, 3));
    }

    #[test]
    fn test_minimap_set_lines() {
        let mut minimap = Minimap::new(vec!["a", "b"]);
        minimap.set_lines(vec!["x", "y", "z"]);
        assert_eq!(minimap.line_count(), 3);
    }

    #[test]
    fn test_minimap_scroll_down() {
        let mut minimap = Minimap::new(vec!["a", "b", "c", "d", "e", "f", "g", "h"]);
        minimap.set_viewport(0, 3);

        minimap.scroll(2);
        assert_eq!(minimap.viewport(), (2, 5));
    }

    #[test]
    fn test_minimap_scroll_up() {
        let mut minimap = Minimap::new(vec!["a", "b", "c", "d", "e"]);
        minimap.set_viewport(2, 4);

        minimap.scroll(-1);
        assert_eq!(minimap.viewport(), (1, 3));
    }

    #[test]
    fn test_minimap_scroll_bounds() {
        let mut minimap = Minimap::new(vec!["a", "b", "c"]);
        minimap.set_viewport(0, 2);

        minimap.scroll(-5); // Should not go negative
        assert_eq!(minimap.viewport(), (0, 2));

        minimap.scroll(10); // Should not exceed bounds
        assert_eq!(minimap.viewport(), (1, 3));
    }

    #[test]
    fn test_minimap_line_density() {
        let minimap = Minimap::new(vec!["", "x", "xxxx"]);

        assert_eq!(minimap.line_density(0), 0.0);
        assert!(minimap.line_density(1) > 0.9);
        assert!(minimap.line_density(2) > 0.9);
    }

    #[test]
    fn test_minimap_density_char() {
        assert_eq!(Minimap::density_char(0.0), ' ');
        assert_eq!(Minimap::density_char(0.2), '░');
        assert_eq!(Minimap::density_char(0.5), '▒');
        assert_eq!(Minimap::density_char(0.7), '▓');
        assert_eq!(Minimap::density_char(1.0), '█');
    }

    #[test]
    fn test_minimap_line_color() {
        let minimap = Minimap::new(vec![
            "// comment",
            "fn main()",
            "\"string\"",
            "let x = 5",
            "",
        ]);

        assert_eq!(minimap.line_color(0), ToadTheme::GRAY);
        assert_eq!(minimap.line_color(1), ToadTheme::TOAD_GREEN);
        assert_eq!(minimap.line_color(2), ToadTheme::YELLOW);
        assert_eq!(minimap.line_color(3), ToadTheme::LIGHT_GRAY);
        assert_eq!(minimap.line_color(4), ToadTheme::DARKER_GRAY);
    }

    #[test]
    fn test_minimap_viewport_lines() {
        let minimap = Minimap::new(vec!["a", "b", "c", "d", "e"]);
        let viewport = minimap.viewport_lines();
        assert_eq!(viewport.len(), 5);
    }

    #[test]
    fn test_minimap_jump_to_start() {
        let mut minimap = Minimap::new(vec!["a", "b", "c", "d", "e", "f", "g", "h"]);
        minimap.set_viewport(4, 8);

        minimap.jump_to(1);
        assert_eq!(minimap.viewport().0, 0);
    }

    #[test]
    fn test_minimap_jump_to_middle() {
        let mut minimap = Minimap::new(vec!["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"]);
        minimap.set_viewport(0, 4);

        minimap.jump_to(5);
        let (start, end) = minimap.viewport();
        assert!(start <= 5 && 5 < end);
    }

    #[test]
    fn test_minimap_jump_to_end() {
        let mut minimap = Minimap::new(vec!["a", "b", "c", "d", "e"]);
        minimap.set_viewport(0, 2);

        minimap.jump_to(4);
        assert_eq!(minimap.viewport().1, 5);
    }

    #[test]
    fn test_minimap_mode_default() {
        let mode = MinimapMode::default();
        assert_eq!(mode, MinimapMode::Blocks);
    }
}
