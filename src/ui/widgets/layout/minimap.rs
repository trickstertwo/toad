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
    text::Line,
    widgets::{Borders, Paragraph, Wrap},
};
use serde::{Deserialize, Serialize};

use crate::ui::{atoms::{block::Block as AtomBlock, text::Text}, theme::ToadTheme};

/// Minimap display mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum MinimapMode {
    /// Show characters (scaled down)
    Characters,
    /// Show blocks/density
    #[default]
    Blocks,
    /// Show syntax highlighting colors
    Colors,
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

            let text = Text::new(line_text).style(style);
            lines_to_render.push(Line::from(vec![text.to_span()]));
        }

        let paragraph = if self.show_border {
            let block = AtomBlock::new()
                .borders(Borders::ALL)
                .title("Minimap")
                .to_ratatui();
            Paragraph::new(lines_to_render)
                .block(block)
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

    // ============================================================================
    // COMPREHENSIVE EDGE CASE TESTS (ADVANCED TIER - 90%+ COVERAGE)
    // ============================================================================

    // Unicode and emoji tests
    #[test]
    fn test_minimap_with_unicode_lines() {
        let lines = vec![
            "// 日本語のコメント 📝",
            "fn メイン() {",
            "    println!(\"こんにちは 👋\");",
            "}",
        ];
        let minimap = Minimap::new(lines);
        assert_eq!(minimap.line_count(), 4);
    }

    #[test]
    fn test_minimap_with_emoji_heavy_content() {
        let lines = vec![
            "🚀🎯🔥💡⚡🌟✨🎨🎭🎪",
            "📊📈📉💻🖥️⌨️🖱️💾💿📀",
            "🐛🐞🐜🦟🕷️🕸️🦂🦗🪲🪳",
        ];
        let minimap = Minimap::new(lines);
        assert_eq!(minimap.line_count(), 3);
    }

    #[test]
    fn test_minimap_with_mixed_unicode() {
        let minimap = Minimap::new(vec![
            "fn main() { // English",
            "// 日本語コメント",
            "let x = \"Привет\";",
            "println!(\"مرحبا\");",
            "// 中文注释",
        ]);
        assert_eq!(minimap.line_count(), 5);
    }

    // Very long line tests
    #[test]
    fn test_minimap_with_very_long_line() {
        let long_line = "x".repeat(10000);
        let minimap = Minimap::new(vec![&long_line]);
        assert_eq!(minimap.line_count(), 1);
        assert!(minimap.line_density(0) > 0.9);
    }

    #[test]
    fn test_minimap_with_very_long_lines() {
        let lines: Vec<String> = (0..100)
            .map(|i| "code line ".repeat(1000) + &i.to_string())
            .collect();
        let minimap = Minimap::new(lines);
        assert_eq!(minimap.line_count(), 100);
    }

    #[test]
    fn test_minimap_with_extreme_line_length() {
        let extreme_line = "fn very_long_function_name_".repeat(500);
        let minimap = Minimap::new(vec![&extreme_line]);
        assert_eq!(minimap.line_count(), 1);
    }

    // Stress tests with many lines
    #[test]
    fn test_minimap_with_many_lines() {
        let lines: Vec<String> = (0..1000).map(|i| format!("line {}", i)).collect();
        let minimap = Minimap::new(lines);
        assert_eq!(minimap.line_count(), 1000);
    }

    #[test]
    fn test_minimap_with_extreme_number_of_lines() {
        let lines: Vec<String> = (0..10000).map(|i| format!("L{}", i)).collect();
        let minimap = Minimap::new(lines);
        assert_eq!(minimap.line_count(), 10000);
        assert_eq!(minimap.viewport().0, 0);
    }

    #[test]
    fn test_minimap_viewport_with_many_lines() {
        let lines: Vec<String> = (0..5000).map(|i| format!("line {}", i)).collect();
        let mut minimap = Minimap::new(lines);
        minimap.set_viewport(2000, 2100);
        assert_eq!(minimap.viewport(), (2000, 2100));
    }

    // Scale edge cases
    #[test]
    fn test_minimap_with_scale_zero() {
        let minimap = Minimap::new(vec!["test"]).with_scale(0);
        assert_eq!(minimap.scale, 1); // Should clamp to minimum of 1
    }

    #[test]
    fn test_minimap_with_scale_one() {
        let minimap = Minimap::new(vec!["test"]).with_scale(1);
        assert_eq!(minimap.scale, 1);
    }

    #[test]
    fn test_minimap_with_extreme_scale() {
        let minimap = Minimap::new(vec!["test"]).with_scale(10000);
        assert_eq!(minimap.scale, 10000);
    }

    // Viewport edge cases
    #[test]
    fn test_minimap_viewport_beyond_bounds() {
        let mut minimap = Minimap::new(vec!["a", "b", "c"]);
        minimap.set_viewport(10, 20);
        assert_eq!(minimap.viewport(), (3, 3)); // Clamped to max
    }

    #[test]
    fn test_minimap_viewport_reversed() {
        let mut minimap = Minimap::new(vec!["a", "b", "c", "d", "e"]);
        minimap.set_viewport(4, 2);
        // Accepts any values, but end is clamped
        let (start, end) = minimap.viewport();
        assert!(start <= minimap.line_count());
        assert!(end <= minimap.line_count());
    }

    #[test]
    fn test_minimap_viewport_empty_range() {
        let mut minimap = Minimap::new(vec!["a", "b", "c"]);
        minimap.set_viewport(2, 2);
        assert_eq!(minimap.viewport(), (2, 2));
    }

    #[test]
    fn test_minimap_viewport_on_empty_minimap() {
        let mut minimap = Minimap::new(Vec::<String>::new());
        minimap.set_viewport(0, 5);
        assert_eq!(minimap.viewport(), (0, 0));
    }

    // Scroll edge cases
    #[test]
    fn test_minimap_scroll_extreme_positive() {
        let mut minimap = Minimap::new(vec!["a", "b", "c", "d", "e"]);
        minimap.set_viewport(0, 2);
        minimap.scroll(10000);
        // Should clamp to valid range
        let (start, end) = minimap.viewport();
        assert!(end <= minimap.line_count());
        assert!(start <= end);
    }

    #[test]
    fn test_minimap_scroll_extreme_negative() {
        let mut minimap = Minimap::new(vec!["a", "b", "c", "d", "e"]);
        minimap.set_viewport(3, 5);
        minimap.scroll(-10000);
        // Should clamp to 0
        assert_eq!(minimap.viewport().0, 0);
    }

    #[test]
    fn test_minimap_scroll_zero() {
        let mut minimap = Minimap::new(vec!["a", "b", "c"]);
        minimap.set_viewport(1, 2);
        let before = minimap.viewport();
        minimap.scroll(0);
        assert_eq!(minimap.viewport(), before);
    }

    #[test]
    fn test_minimap_scroll_on_empty_minimap() {
        let mut minimap = Minimap::new(Vec::<String>::new());
        minimap.scroll(5);
        assert_eq!(minimap.viewport(), (0, 0));
    }

    // Jump-to edge cases
    #[test]
    fn test_minimap_jump_to_zero() {
        let mut minimap = Minimap::new(vec!["a", "b", "c", "d", "e"]);
        minimap.set_viewport(2, 4);
        minimap.jump_to(0);
        assert_eq!(minimap.viewport().0, 0);
    }

    #[test]
    fn test_minimap_jump_to_beyond_end() {
        let mut minimap = Minimap::new(vec!["a", "b", "c"]);
        minimap.set_viewport(0, 2);
        minimap.jump_to(10000);
        // Should clamp to valid range
        let (_, end) = minimap.viewport();
        assert_eq!(end, 3);
    }

    #[test]
    fn test_minimap_jump_to_exact_end() {
        let mut minimap = Minimap::new(vec!["a", "b", "c", "d", "e"]);
        minimap.set_viewport(0, 3);
        minimap.jump_to(4);
        let (_, end) = minimap.viewport();
        assert_eq!(end, 5);
    }

    #[test]
    fn test_minimap_jump_to_on_single_line() {
        let mut minimap = Minimap::new(vec!["only line"]);
        minimap.jump_to(0);
        assert_eq!(minimap.viewport(), (0, 1));
    }

    // Display mode tests
    #[test]
    fn test_minimap_all_display_modes() {
        let lines = vec!["fn main() {", "    println!(\"test\");", "}"];

        let char_mode = Minimap::new(lines.clone()).with_mode(MinimapMode::Characters);
        assert_eq!(char_mode.mode, MinimapMode::Characters);

        let block_mode = Minimap::new(lines.clone()).with_mode(MinimapMode::Blocks);
        assert_eq!(block_mode.mode, MinimapMode::Blocks);

        let color_mode = Minimap::new(lines).with_mode(MinimapMode::Colors);
        assert_eq!(color_mode.mode, MinimapMode::Colors);
    }

    #[test]
    fn test_minimap_mode_equality() {
        assert_eq!(MinimapMode::Characters, MinimapMode::Characters);
        assert_eq!(MinimapMode::Blocks, MinimapMode::Blocks);
        assert_eq!(MinimapMode::Colors, MinimapMode::Colors);
        assert_ne!(MinimapMode::Characters, MinimapMode::Blocks);
    }

    // Line density edge cases
    #[test]
    fn test_minimap_density_all_whitespace() {
        let minimap = Minimap::new(vec!["     ", "\t\t\t", "  \t  "]);
        assert_eq!(minimap.line_density(0), 0.0);
        assert_eq!(minimap.line_density(1), 0.0);
        assert_eq!(minimap.line_density(2), 0.0);
    }

    #[test]
    fn test_minimap_density_no_whitespace() {
        let minimap = Minimap::new(vec!["xxxxx", "12345", "abcde"]);
        assert!(minimap.line_density(0) > 0.99);
        assert!(minimap.line_density(1) > 0.99);
        assert!(minimap.line_density(2) > 0.99);
    }

    #[test]
    fn test_minimap_density_mixed() {
        let minimap = Minimap::new(vec!["x    ", "  x  ", "    x"]);
        // Each has 1 non-whitespace out of 5 chars = 0.2
        assert!((minimap.line_density(0) - 0.2).abs() < 0.01);
        assert!((minimap.line_density(1) - 0.2).abs() < 0.01);
        assert!((minimap.line_density(2) - 0.2).abs() < 0.01);
    }

    #[test]
    fn test_minimap_density_out_of_bounds() {
        let minimap = Minimap::new(vec!["test"]);
        assert_eq!(minimap.line_density(10), 0.0);
        assert_eq!(minimap.line_density(1000), 0.0);
    }

    #[test]
    fn test_minimap_density_empty_line() {
        let minimap = Minimap::new(vec![""]);
        assert_eq!(minimap.line_density(0), 0.0);
    }

    // Density character mapping
    #[test]
    fn test_density_char_boundaries() {
        assert_eq!(Minimap::density_char(0.0), ' ');
        assert_eq!(Minimap::density_char(0.09), ' ');
        assert_eq!(Minimap::density_char(0.1), '░');
        assert_eq!(Minimap::density_char(0.29), '░');
        assert_eq!(Minimap::density_char(0.3), '▒');
        assert_eq!(Minimap::density_char(0.59), '▒');
        assert_eq!(Minimap::density_char(0.6), '▓');
        assert_eq!(Minimap::density_char(0.89), '▓');
        assert_eq!(Minimap::density_char(0.9), '█');
        assert_eq!(Minimap::density_char(1.0), '█');
    }

    // Line color detection edge cases
    #[test]
    fn test_minimap_color_for_different_comment_styles() {
        let minimap = Minimap::new(vec![
            "// Single line comment",
            "# Python/shell comment",
            "/* Multi-line not detected */",
            "  // Indented comment",
            "code // inline comment",
        ]);

        assert_eq!(minimap.line_color(0), ToadTheme::GRAY);
        assert_eq!(minimap.line_color(1), ToadTheme::GRAY);
        // /* is not detected as comment start
        assert_eq!(minimap.line_color(2), ToadTheme::LIGHT_GRAY);
        assert_eq!(minimap.line_color(3), ToadTheme::GRAY);
        // Line doesn't start with //, so it's code
        assert_eq!(minimap.line_color(4), ToadTheme::LIGHT_GRAY);
    }

    #[test]
    fn test_minimap_color_for_keywords() {
        let minimap = Minimap::new(vec![
            "fn main() {",
            "pub fn test() {",
            "struct Foo {",
            "impl Bar {",
            "  fn nested() {", // Indented, not at start
        ]);

        assert_eq!(minimap.line_color(0), ToadTheme::TOAD_GREEN);
        assert_eq!(minimap.line_color(1), ToadTheme::TOAD_GREEN);
        assert_eq!(minimap.line_color(2), ToadTheme::TOAD_GREEN);
        assert_eq!(minimap.line_color(3), ToadTheme::TOAD_GREEN);
        // Indented fn is not at start after trim, but trimmed it is
        assert_eq!(minimap.line_color(4), ToadTheme::TOAD_GREEN);
    }

    #[test]
    fn test_minimap_color_for_strings() {
        let minimap = Minimap::new(vec![
            "\"string literal\"",
            "let s = \"test\";",
            "println!(\"hello\");",
        ]);

        assert_eq!(minimap.line_color(0), ToadTheme::YELLOW);
        assert_eq!(minimap.line_color(1), ToadTheme::YELLOW);
        assert_eq!(minimap.line_color(2), ToadTheme::YELLOW);
    }

    #[test]
    fn test_minimap_color_out_of_bounds() {
        let minimap = Minimap::new(vec!["test"]);
        assert_eq!(minimap.line_color(10), ToadTheme::DARKER_GRAY);
    }

    // Builder pattern tests
    #[test]
    fn test_minimap_builder_pattern_all_options() {
        let lines = vec!["line1", "line2", "line3"];
        let minimap = Minimap::new(lines)
            .with_mode(MinimapMode::Colors)
            .with_border(false)
            .with_scale(8);

        assert_eq!(minimap.mode, MinimapMode::Colors);
        assert!(!minimap.show_border);
        assert_eq!(minimap.scale, 8);
    }

    #[test]
    fn test_minimap_builder_pattern_chaining() {
        let minimap = Minimap::new(vec!["test"])
            .with_mode(MinimapMode::Characters)
            .with_border(true)
            .with_scale(2)
            .with_mode(MinimapMode::Blocks);

        // Last mode should win
        assert_eq!(minimap.mode, MinimapMode::Blocks);
        assert!(minimap.show_border);
        assert_eq!(minimap.scale, 2);
    }

    // Clone trait test
    #[test]
    fn test_minimap_clone() {
        let original = Minimap::new(vec!["a", "b", "c"])
            .with_mode(MinimapMode::Characters)
            .with_scale(4);

        let cloned = original.clone();

        assert_eq!(cloned.line_count(), 3);
        assert_eq!(cloned.mode, MinimapMode::Characters);
        assert_eq!(cloned.scale, 4);
    }

    // Empty minimap tests
    #[test]
    fn test_minimap_empty_creation() {
        let minimap = Minimap::new(Vec::<String>::new());
        assert_eq!(minimap.line_count(), 0);
        assert_eq!(minimap.viewport(), (0, 0));
    }

    #[test]
    fn test_minimap_set_empty_lines() {
        let mut minimap = Minimap::new(vec!["a", "b", "c"]);
        minimap.set_lines(Vec::<String>::new());
        assert_eq!(minimap.line_count(), 0);
    }

    #[test]
    fn test_minimap_viewport_lines_empty() {
        let minimap = Minimap::new(Vec::<String>::new());
        let viewport = minimap.viewport_lines();
        assert_eq!(viewport.len(), 0);
    }

    // Viewport lines tests
    #[test]
    fn test_minimap_viewport_lines_partial() {
        let mut minimap = Minimap::new(vec!["a", "b", "c", "d", "e"]);
        minimap.set_viewport(1, 3);
        let viewport = minimap.viewport_lines();
        assert_eq!(viewport, vec!["b", "c"]);
    }

    #[test]
    fn test_minimap_viewport_lines_full() {
        let minimap = Minimap::new(vec!["a", "b", "c"]);
        let viewport = minimap.viewport_lines();
        assert_eq!(viewport, vec!["a", "b", "c"]);
    }

    // Set lines with viewport adjustment
    #[test]
    fn test_minimap_set_lines_adjusts_viewport() {
        let mut minimap = Minimap::new(vec!["a", "b", "c", "d", "e"]);
        minimap.set_viewport(0, 5);
        minimap.set_lines(vec!["x", "y"]);
        let (_, end) = minimap.viewport();
        assert!(end <= 2);
    }

    #[test]
    fn test_minimap_set_lines_preserves_start() {
        let mut minimap = Minimap::new(vec!["a", "b"]);
        minimap.set_viewport(1, 2);
        minimap.set_lines(vec!["x", "y", "z"]);
        let (start, _) = minimap.viewport();
        assert_eq!(start, 1);
    }

    // Complex scrolling scenarios
    #[test]
    fn test_minimap_multiple_scrolls() {
        let mut minimap = Minimap::new(vec!["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"]);
        minimap.set_viewport(0, 3);

        minimap.scroll(2);
        assert_eq!(minimap.viewport(), (2, 5));

        minimap.scroll(2);
        assert_eq!(minimap.viewport(), (4, 7));

        minimap.scroll(-3);
        assert_eq!(minimap.viewport(), (1, 4));
    }

    #[test]
    fn test_minimap_scroll_maintains_viewport_size() {
        let mut minimap = Minimap::new(vec!["a", "b", "c", "d", "e", "f", "g", "h"]);
        minimap.set_viewport(2, 5); // Size = 3

        minimap.scroll(1);
        let (start, end) = minimap.viewport();
        assert_eq!(end - start, 3);

        minimap.scroll(-1);
        let (start, end) = minimap.viewport();
        assert_eq!(end - start, 3);
    }

    // Stress test combining all features
    #[test]
    fn test_minimap_comprehensive_stress_test() {
        let lines: Vec<String> = (0..1000)
            .map(|i| match i % 5 {
                0 => format!("// Comment line {} 📝", i),
                1 => format!("fn function_{}() {{", i),
                2 => format!("    let x = \"string {}\";", i),
                3 => format!("    println!(\"Line {}\");", i),
                _ => format!("}}  // End block {}", i),
            })
            .collect();

        let mut minimap = Minimap::new(lines)
            .with_mode(MinimapMode::Colors)
            .with_border(true)
            .with_scale(4);

        assert_eq!(minimap.line_count(), 1000);

        // Test various operations
        minimap.set_viewport(100, 150);
        assert_eq!(minimap.viewport(), (100, 150));

        minimap.scroll(50);
        assert_eq!(minimap.viewport(), (150, 200));

        minimap.jump_to(500);
        let (start, end) = minimap.viewport();
        assert!(start <= 500 && 500 < end);

        // Verify density calculations work
        for i in 0..10 {
            let _ = minimap.line_density(i);
        }

        // Verify color detection works
        for i in 0..10 {
            let _ = minimap.line_color(i);
        }
    }
}
