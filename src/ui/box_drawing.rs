/// Box drawing characters and utilities
///
/// Provides Unicode box drawing characters and utilities for creating beautiful terminal borders
///
/// # Examples
///
/// ```
/// use toad::box_drawing::{BoxStyle, BoxChars};
///
/// let heavy = BoxStyle::Heavy.chars();
/// assert_eq!(heavy.top_left, '┏');
/// assert_eq!(heavy.horizontal, '━');
/// ```
use serde::{Deserialize, Serialize};
use std::fmt;

/// Box drawing character set
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoxChars {
    /// Top-left corner
    pub top_left: char,
    /// Top-right corner
    pub top_right: char,
    /// Bottom-left corner
    pub bottom_left: char,
    /// Bottom-right corner
    pub bottom_right: char,
    /// Horizontal line
    pub horizontal: char,
    /// Vertical line
    pub vertical: char,
    /// T-junction pointing down
    pub t_down: char,
    /// T-junction pointing up
    pub t_up: char,
    /// T-junction pointing right
    pub t_right: char,
    /// T-junction pointing left
    pub t_left: char,
    /// Cross (four-way junction)
    pub cross: char,
}

impl BoxChars {
    /// Create a custom box character set
    pub const fn custom(
        top_left: char,
        top_right: char,
        bottom_left: char,
        bottom_right: char,
        horizontal: char,
        vertical: char,
        t_down: char,
        t_up: char,
        t_right: char,
        t_left: char,
        cross: char,
    ) -> Self {
        Self {
            top_left,
            top_right,
            bottom_left,
            bottom_right,
            horizontal,
            vertical,
            t_down,
            t_up,
            t_right,
            t_left,
            cross,
        }
    }

    /// Light box drawing characters (thin lines)
    pub const LIGHT: Self = Self {
        top_left: '┌',
        top_right: '┐',
        bottom_left: '└',
        bottom_right: '┘',
        horizontal: '─',
        vertical: '│',
        t_down: '┬',
        t_up: '┴',
        t_right: '├',
        t_left: '┤',
        cross: '┼',
    };

    /// Heavy box drawing characters (thick lines)
    pub const HEAVY: Self = Self {
        top_left: '┏',
        top_right: '┓',
        bottom_left: '┗',
        bottom_right: '┛',
        horizontal: '━',
        vertical: '┃',
        t_down: '┳',
        t_up: '┻',
        t_right: '┣',
        t_left: '┫',
        cross: '╋',
    };

    /// Double box drawing characters
    pub const DOUBLE: Self = Self {
        top_left: '╔',
        top_right: '╗',
        bottom_left: '╚',
        bottom_right: '╝',
        horizontal: '═',
        vertical: '║',
        t_down: '╦',
        t_up: '╩',
        t_right: '╠',
        t_left: '╣',
        cross: '╬',
    };

    /// Rounded box drawing characters
    pub const ROUNDED: Self = Self {
        top_left: '╭',
        top_right: '╮',
        bottom_left: '╰',
        bottom_right: '╯',
        horizontal: '─',
        vertical: '│',
        t_down: '┬',
        t_up: '┴',
        t_right: '├',
        t_left: '┤',
        cross: '┼',
    };

    /// ASCII box drawing characters (for compatibility)
    pub const ASCII: Self = Self {
        top_left: '+',
        top_right: '+',
        bottom_left: '+',
        bottom_right: '+',
        horizontal: '-',
        vertical: '|',
        t_down: '+',
        t_up: '+',
        t_right: '+',
        t_left: '+',
        cross: '+',
    };

    /// Draw a horizontal line
    pub fn horizontal_line(&self, width: usize) -> String {
        self.horizontal.to_string().repeat(width)
    }

    /// Draw a vertical line segment
    pub fn vertical_line(&self, height: usize) -> Vec<String> {
        vec![self.vertical.to_string(); height]
    }

    /// Draw a top border
    pub fn top_border(&self, width: usize) -> String {
        format!(
            "{}{}{}",
            self.top_left,
            self.horizontal_line(width),
            self.top_right
        )
    }

    /// Draw a bottom border
    pub fn bottom_border(&self, width: usize) -> String {
        format!(
            "{}{}{}",
            self.bottom_left,
            self.horizontal_line(width),
            self.bottom_right
        )
    }

    /// Draw a middle line (for separating sections)
    pub fn middle_line(&self, width: usize) -> String {
        format!(
            "{}{}{}",
            self.t_right,
            self.horizontal_line(width),
            self.t_left
        )
    }

    /// Draw a complete box
    pub fn draw_box(&self, width: usize, height: usize, content: &[&str]) -> Vec<String> {
        let mut lines = Vec::new();

        // Top border
        lines.push(self.top_border(width));

        // Content lines
        for (i, line) in content.iter().take(height).enumerate() {
            let padded = format!("{:<width$}", line, width = width);
            lines.push(format!("{}{}{}", self.vertical, padded, self.vertical));

            // Add remaining empty lines if content is shorter than height
            if i == content.len() - 1 && content.len() < height {
                for _ in 0..(height - content.len()) {
                    lines.push(format!(
                        "{}{}{}",
                        self.vertical,
                        " ".repeat(width),
                        self.vertical
                    ));
                }
                break;
            }
        }

        // Bottom border
        lines.push(self.bottom_border(width));

        lines
    }
}

impl Default for BoxChars {
    fn default() -> Self {
        Self::LIGHT
    }
}

impl fmt::Display for BoxChars {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}{}{}", self.top_left, self.horizontal, self.top_right)?;
        writeln!(f, "{} {}", self.vertical, self.vertical)?;
        write!(
            f,
            "{}{}{}",
            self.bottom_left, self.horizontal, self.bottom_right
        )
    }
}

/// Box drawing style presets
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[derive(Default)]
pub enum BoxStyle {
    /// Light lines (thin)
    #[default]
    Light,
    /// Heavy lines (thick)
    Heavy,
    /// Double lines
    Double,
    /// Rounded corners
    Rounded,
    /// ASCII (for compatibility)
    Ascii,
}

impl BoxStyle {
    /// Get the character set for this style
    pub fn chars(&self) -> BoxChars {
        match self {
            BoxStyle::Light => BoxChars::LIGHT,
            BoxStyle::Heavy => BoxChars::HEAVY,
            BoxStyle::Double => BoxChars::DOUBLE,
            BoxStyle::Rounded => BoxChars::ROUNDED,
            BoxStyle::Ascii => BoxChars::ASCII,
        }
    }

    /// All available styles
    pub fn all() -> &'static [BoxStyle] {
        &[
            BoxStyle::Light,
            BoxStyle::Heavy,
            BoxStyle::Double,
            BoxStyle::Rounded,
            BoxStyle::Ascii,
        ]
    }

    /// Get style name
    pub fn name(&self) -> &'static str {
        match self {
            BoxStyle::Light => "Light",
            BoxStyle::Heavy => "Heavy",
            BoxStyle::Double => "Double",
            BoxStyle::Rounded => "Rounded",
            BoxStyle::Ascii => "ASCII",
        }
    }
}


impl fmt::Display for BoxStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Box builder for creating boxes with various options
#[derive(Debug, Clone)]
pub struct BoxBuilder {
    style: BoxStyle,
    width: usize,
    height: usize,
    title: Option<String>,
    padding: usize,
}

impl BoxBuilder {
    /// Create a new box builder
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::box_drawing::{BoxBuilder, BoxStyle};
    ///
    /// let builder = BoxBuilder::new(BoxStyle::Heavy, 20, 5);
    /// let lines = builder.build(&["Hello", "World"]);
    /// assert!(lines.len() > 0);
    /// ```
    pub fn new(style: BoxStyle, width: usize, height: usize) -> Self {
        Self {
            style,
            width,
            height,
            title: None,
            padding: 0,
        }
    }

    /// Set the box title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the padding
    pub fn padding(mut self, padding: usize) -> Self {
        self.padding = padding;
        self
    }

    /// Build the box with content
    pub fn build(&self, content: &[&str]) -> Vec<String> {
        let chars = self.style.chars();
        let mut lines = Vec::new();

        // Top border with optional title
        if let Some(ref title) = self.title {
            let title_len = title.chars().count();
            if title_len + 4 <= self.width {
                let left_pad = (self.width - title_len - 2) / 2;
                let right_pad = self.width - title_len - 2 - left_pad;
                lines.push(format!(
                    "{}{}[ {} ]{}{}",
                    chars.top_left,
                    chars.horizontal.to_string().repeat(left_pad),
                    title,
                    chars.horizontal.to_string().repeat(right_pad),
                    chars.top_right
                ));
            } else {
                lines.push(chars.top_border(self.width));
            }
        } else {
            lines.push(chars.top_border(self.width));
        }

        // Padding lines at top
        for _ in 0..self.padding {
            lines.push(format!(
                "{}{}{}",
                chars.vertical,
                " ".repeat(self.width),
                chars.vertical
            ));
        }

        // Content lines
        let content_width = self.width.saturating_sub(self.padding * 2);
        for line in content.iter().take(self.height) {
            let padded = format!("{:<width$}", line, width = content_width);
            let with_padding = format!(
                "{}{}",
                " ".repeat(self.padding),
                padded.chars().take(content_width).collect::<String>()
            );
            lines.push(format!(
                "{}{}{}",
                chars.vertical,
                format!("{:<width$}", with_padding, width = self.width),
                chars.vertical
            ));
        }

        // Fill remaining height
        let current_content = content.len() + self.padding;
        if current_content < self.height {
            for _ in 0..(self.height - current_content) {
                lines.push(format!(
                    "{}{}{}",
                    chars.vertical,
                    " ".repeat(self.width),
                    chars.vertical
                ));
            }
        }

        // Padding lines at bottom
        for _ in 0..self.padding {
            lines.push(format!(
                "{}{}{}",
                chars.vertical,
                " ".repeat(self.width),
                chars.vertical
            ));
        }

        // Bottom border
        lines.push(chars.bottom_border(self.width));

        lines
    }
}

impl Default for BoxBuilder {
    fn default() -> Self {
        Self::new(BoxStyle::Light, 40, 10)
    }
}

/// Utility functions for drawing boxes
pub mod utils {
    use super::*;

    /// Create a simple box with light style
    pub fn simple_box(width: usize, height: usize, content: &[&str]) -> Vec<String> {
        BoxChars::LIGHT.draw_box(width, height, content)
    }

    /// Create a fancy box with heavy style
    pub fn fancy_box(width: usize, height: usize, content: &[&str]) -> Vec<String> {
        BoxChars::HEAVY.draw_box(width, height, content)
    }

    /// Create a box with a title
    pub fn titled_box(
        style: BoxStyle,
        width: usize,
        height: usize,
        title: &str,
        content: &[&str],
    ) -> Vec<String> {
        BoxBuilder::new(style, width, height)
            .title(title)
            .build(content)
    }

    /// Join multiple boxes horizontally
    pub fn join_horizontal(boxes: &[Vec<String>]) -> Vec<String> {
        if boxes.is_empty() {
            return Vec::new();
        }

        let height = boxes[0].len();
        let mut result = Vec::new();

        for i in 0..height {
            let mut line = String::new();
            for box_lines in boxes {
                if i < box_lines.len() {
                    line.push_str(&box_lines[i]);
                    line.push(' ');
                }
            }
            result.push(line.trim_end().to_string());
        }

        result
    }

    /// Join multiple boxes vertically
    pub fn join_vertical(boxes: &[Vec<String>]) -> Vec<String> {
        let mut result = Vec::new();
        for box_lines in boxes {
            result.extend_from_slice(box_lines);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box_chars_light() {
        let light = BoxChars::LIGHT;
        assert_eq!(light.top_left, '┌');
        assert_eq!(light.horizontal, '─');
        assert_eq!(light.vertical, '│');
    }

    #[test]
    fn test_box_chars_heavy() {
        let heavy = BoxChars::HEAVY;
        assert_eq!(heavy.top_left, '┏');
        assert_eq!(heavy.horizontal, '━');
        assert_eq!(heavy.vertical, '┃');
    }

    #[test]
    fn test_box_chars_double() {
        let double = BoxChars::DOUBLE;
        assert_eq!(double.top_left, '╔');
        assert_eq!(double.horizontal, '═');
        assert_eq!(double.vertical, '║');
    }

    #[test]
    fn test_box_chars_rounded() {
        let rounded = BoxChars::ROUNDED;
        assert_eq!(rounded.top_left, '╭');
        assert_eq!(rounded.top_right, '╮');
        assert_eq!(rounded.bottom_left, '╰');
        assert_eq!(rounded.bottom_right, '╯');
    }

    #[test]
    fn test_box_chars_ascii() {
        let ascii = BoxChars::ASCII;
        assert_eq!(ascii.top_left, '+');
        assert_eq!(ascii.horizontal, '-');
        assert_eq!(ascii.vertical, '|');
    }

    #[test]
    fn test_horizontal_line() {
        let light = BoxChars::LIGHT;
        assert_eq!(light.horizontal_line(5), "─────");
    }

    #[test]
    fn test_top_border() {
        let light = BoxChars::LIGHT;
        assert_eq!(light.top_border(3), "┌───┐");
    }

    #[test]
    fn test_bottom_border() {
        let light = BoxChars::LIGHT;
        assert_eq!(light.bottom_border(3), "└───┘");
    }

    #[test]
    fn test_middle_line() {
        let light = BoxChars::LIGHT;
        assert_eq!(light.middle_line(3), "├───┤");
    }

    #[test]
    fn test_draw_box() {
        let light = BoxChars::LIGHT;
        let content = vec!["Hello", "World"];
        let box_lines = light.draw_box(5, 2, &content);

        assert_eq!(box_lines.len(), 4); // top + 2 content + bottom
        assert_eq!(box_lines[0], "┌─────┐");
        assert_eq!(box_lines[1], "│Hello│");
        assert_eq!(box_lines[2], "│World│");
        assert_eq!(box_lines[3], "└─────┘");
    }

    #[test]
    fn test_draw_box_padding() {
        let light = BoxChars::LIGHT;
        let content = vec!["Hi"];
        let box_lines = light.draw_box(5, 3, &content);

        assert_eq!(box_lines.len(), 5); // top + 3 content + bottom
        assert_eq!(box_lines[0], "┌─────┐");
        assert_eq!(box_lines[1], "│Hi   │");
        assert_eq!(box_lines[2], "│     │");
        assert_eq!(box_lines[3], "│     │");
        assert_eq!(box_lines[4], "└─────┘");
    }

    #[test]
    fn test_box_style_chars() {
        let light = BoxStyle::Light.chars();
        assert_eq!(light.top_left, '┌');

        let heavy = BoxStyle::Heavy.chars();
        assert_eq!(heavy.top_left, '┏');
    }

    #[test]
    fn test_box_style_all() {
        let styles = BoxStyle::all();
        assert_eq!(styles.len(), 5);
    }

    #[test]
    fn test_box_style_name() {
        assert_eq!(BoxStyle::Light.name(), "Light");
        assert_eq!(BoxStyle::Heavy.name(), "Heavy");
        assert_eq!(BoxStyle::Double.name(), "Double");
        assert_eq!(BoxStyle::Rounded.name(), "Rounded");
        assert_eq!(BoxStyle::Ascii.name(), "ASCII");
    }

    #[test]
    fn test_box_builder() {
        let builder = BoxBuilder::new(BoxStyle::Light, 10, 3);
        let lines = builder.build(&["Test"]);

        assert!(!lines.is_empty());
        assert!(lines[0].contains('┌'));
        assert!(lines[lines.len() - 1].contains('└'));
    }

    #[test]
    fn test_box_builder_with_title() {
        let builder = BoxBuilder::new(BoxStyle::Light, 20, 3).title("My Title");
        let lines = builder.build(&["Content"]);

        assert!(lines[0].contains("My Title"));
        assert!(lines[0].contains('['));
        assert!(lines[0].contains(']'));
    }

    #[test]
    fn test_box_builder_with_padding() {
        let builder = BoxBuilder::new(BoxStyle::Light, 10, 5).padding(1);
        let lines = builder.build(&["Test"]);

        // Should have padding lines at top and bottom
        assert!(lines.len() >= 5);
    }

    #[test]
    fn test_utils_simple_box() {
        let lines = utils::simple_box(5, 2, &["Hi"]);
        assert_eq!(lines[0], "┌─────┐");
    }

    #[test]
    fn test_utils_fancy_box() {
        let lines = utils::fancy_box(5, 2, &["Hi"]);
        assert_eq!(lines[0], "┏━━━━━┓");
    }

    #[test]
    fn test_utils_titled_box() {
        let lines = utils::titled_box(BoxStyle::Light, 20, 3, "Title", &["Content"]);
        assert!(lines[0].contains("Title"));
    }

    #[test]
    fn test_utils_join_horizontal() {
        let box1 = utils::simple_box(5, 2, &["A"]);
        let box2 = utils::simple_box(5, 2, &["B"]);
        let joined = utils::join_horizontal(&[box1, box2]);

        assert_eq!(joined.len(), 4);
        assert!(joined[0].contains('┌'));
    }

    #[test]
    fn test_utils_join_vertical() {
        let box1 = utils::simple_box(5, 2, &["A"]);
        let box2 = utils::simple_box(5, 2, &["B"]);
        let joined = utils::join_vertical(&[box1, box2]);

        assert_eq!(joined.len(), 8); // 4 lines per box
    }

    #[test]
    fn test_custom_box_chars() {
        let custom = BoxChars::custom('a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k');
        assert_eq!(custom.top_left, 'a');
        assert_eq!(custom.top_right, 'b');
        assert_eq!(custom.bottom_left, 'c');
        assert_eq!(custom.horizontal, 'e');
    }

    #[test]
    fn test_box_chars_default() {
        let default = BoxChars::default();
        assert_eq!(default, BoxChars::LIGHT);
    }

    #[test]
    fn test_box_style_default() {
        let default = BoxStyle::default();
        assert_eq!(default, BoxStyle::Light);
    }

    #[test]
    fn test_box_builder_default() {
        let default = BoxBuilder::default();
        let lines = default.build(&["Test"]);
        assert!(!lines.is_empty());
    }

    #[test]
    fn test_vertical_line() {
        let light = BoxChars::LIGHT;
        let lines = light.vertical_line(3);
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0], "│");
    }

    #[test]
    fn test_box_style_display() {
        assert_eq!(format!("{}", BoxStyle::Light), "Light");
        assert_eq!(format!("{}", BoxStyle::Heavy), "Heavy");
    }

    #[test]
    fn test_box_chars_display() {
        let light = BoxChars::LIGHT;
        let display = format!("{}", light);
        assert!(display.contains('┌'));
        assert!(display.contains('│'));
        assert!(display.contains('└'));
    }
}
