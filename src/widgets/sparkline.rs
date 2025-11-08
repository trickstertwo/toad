//! Sparkline widgets for inline metric visualization
//!
//! Provides compact inline graphs for displaying trends and metrics in limited space.
//!
//! # Examples
//!
//! ```
//! use toad::widgets::{Sparkline, SparklineStyle};
//!
//! // Create a bar sparkline
//! let data = vec![1.0, 3.0, 2.0, 5.0, 4.0];
//! let sparkline = Sparkline::new(data)
//!     .style(SparklineStyle::Bar)
//!     .show_markers(true);
//!
//! assert_eq!(sparkline.data_points(), 5);
//! ```

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::Widget,
};

/// Sparkline rendering styles
///
/// # Examples
///
/// ```
/// use toad::widgets::SparklineStyle;
///
/// let bar = SparklineStyle::Bar;
/// let line = SparklineStyle::Line;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SparklineStyle {
    /// Vertical bars: ▁ ▂ ▃ ▄ ▅ ▆ ▇ █
    #[default]
    Bar,
    /// Connected line graph
    Line,
}

/// Compact inline graph widget
///
/// Sparklines visualize data trends in minimal space, perfect for dashboards
/// and status displays.
///
/// # Examples
///
/// ```
/// use toad::widgets::Sparkline;
/// use ratatui::style::Color;
///
/// let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
/// let mut sparkline = Sparkline::new(data);
/// sparkline.set_color(Color::Cyan);
///
/// assert_eq!(sparkline.max(), Some(5.0));
/// assert_eq!(sparkline.min(), Some(1.0));
/// ```
#[derive(Debug, Clone)]
pub struct Sparkline {
    /// Data points to visualize
    data: Vec<f64>,
    /// Rendering style
    style: SparklineStyle,
    /// Whether to show min/max/avg markers
    show_markers: bool,
    /// Base color for sparkline
    color: Color,
    /// Whether to use gradient coloring
    use_gradient: bool,
    /// Gradient start color
    gradient_start: Color,
    /// Gradient end color
    gradient_end: Color,
}

impl Sparkline {
    /// Create a new sparkline with the given data
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Sparkline;
    ///
    /// let data = vec![1.0, 2.0, 3.0];
    /// let sparkline = Sparkline::new(data);
    /// assert_eq!(sparkline.data_points(), 3);
    /// ```
    pub fn new(data: Vec<f64>) -> Self {
        Self {
            data,
            style: SparklineStyle::default(),
            show_markers: false,
            color: Color::Green,
            use_gradient: false,
            gradient_start: Color::Green,
            gradient_end: Color::Red,
        }
    }

    /// Create an empty sparkline
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Sparkline;
    ///
    /// let sparkline = Sparkline::empty();
    /// assert_eq!(sparkline.data_points(), 0);
    /// ```
    pub fn empty() -> Self {
        Self::new(Vec::new())
    }

    /// Set the sparkline style
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{Sparkline, SparklineStyle};
    ///
    /// let sparkline = Sparkline::empty()
    ///     .style(SparklineStyle::Line);
    /// ```
    pub fn style(mut self, style: SparklineStyle) -> Self {
        self.style = style;
        self
    }

    /// Set whether to show statistical markers
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Sparkline;
    ///
    /// let sparkline = Sparkline::empty()
    ///     .show_markers(true);
    /// ```
    pub fn show_markers(mut self, show: bool) -> Self {
        self.show_markers = show;
        self
    }

    /// Set the sparkline color
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Sparkline;
    /// use ratatui::style::Color;
    ///
    /// let sparkline = Sparkline::empty()
    ///     .color(Color::Cyan);
    /// ```
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Enable gradient coloring from start to end color
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Sparkline;
    /// use ratatui::style::Color;
    ///
    /// let sparkline = Sparkline::empty()
    ///     .gradient(Color::Green, Color::Red);
    /// ```
    pub fn gradient(mut self, start: Color, end: Color) -> Self {
        self.use_gradient = true;
        self.gradient_start = start;
        self.gradient_end = end;
        self
    }

    /// Set the base color (mutable)
    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    /// Get the number of data points
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Sparkline;
    ///
    /// let data = vec![1.0, 2.0, 3.0, 4.0];
    /// let sparkline = Sparkline::new(data);
    /// assert_eq!(sparkline.data_points(), 4);
    /// ```
    pub fn data_points(&self) -> usize {
        self.data.len()
    }

    /// Add a data point
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Sparkline;
    ///
    /// let mut sparkline = Sparkline::empty();
    /// sparkline.push(5.0);
    /// assert_eq!(sparkline.data_points(), 1);
    /// ```
    pub fn push(&mut self, value: f64) {
        self.data.push(value);
    }

    /// Clear all data points
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Sparkline;
    ///
    /// let mut sparkline = Sparkline::new(vec![1.0, 2.0, 3.0]);
    /// sparkline.clear();
    /// assert_eq!(sparkline.data_points(), 0);
    /// ```
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Get maximum value in dataset
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Sparkline;
    ///
    /// let sparkline = Sparkline::new(vec![1.0, 5.0, 3.0]);
    /// assert_eq!(sparkline.max(), Some(5.0));
    /// ```
    pub fn max(&self) -> Option<f64> {
        self.data.iter().copied().fold(None, |acc, x| {
            Some(acc.map_or(x, |a| a.max(x)))
        })
    }

    /// Get minimum value in dataset
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Sparkline;
    ///
    /// let sparkline = Sparkline::new(vec![1.0, 5.0, 3.0]);
    /// assert_eq!(sparkline.min(), Some(1.0));
    /// ```
    pub fn min(&self) -> Option<f64> {
        self.data.iter().copied().fold(None, |acc, x| {
            Some(acc.map_or(x, |a| a.min(x)))
        })
    }

    /// Get average value of dataset
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Sparkline;
    ///
    /// let sparkline = Sparkline::new(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
    /// assert_eq!(sparkline.avg(), Some(3.0));
    /// ```
    pub fn avg(&self) -> Option<f64> {
        if self.data.is_empty() {
            None
        } else {
            Some(self.data.iter().sum::<f64>() / self.data.len() as f64)
        }
    }

    /// Get bar character for a normalized value (0.0 - 1.0)
    fn bar_char(normalized: f64) -> &'static str {
        match (normalized * 8.0) as usize {
            0 => " ",
            1 => "▁",
            2 => "▂",
            3 => "▃",
            4 => "▄",
            5 => "▅",
            6 => "▆",
            7 => "▇",
            _ => "█",
        }
    }

    /// Normalize a value to 0.0 - 1.0 range
    fn normalize(&self, value: f64) -> f64 {
        if let (Some(min), Some(max)) = (self.min(), self.max()) {
            if max == min {
                0.5 // All values are equal, center them
            } else {
                (value - min) / (max - min)
            }
        } else {
            0.0
        }
    }

    /// Get color for a value based on gradient settings
    fn get_color(&self, value: f64) -> Color {
        if !self.use_gradient {
            return self.color;
        }

        let normalized = self.normalize(value);

        // Simple gradient interpolation
        match (normalized * 5.0) as usize {
            0 => self.gradient_start,
            1 => Color::Yellow,
            2 => Color::LightYellow,
            3 => Color::LightRed,
            _ => self.gradient_end,
        }
    }

    /// Render sparkline to a string representation
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{Sparkline, SparklineStyle};
    ///
    /// let data = vec![1.0, 3.0, 2.0];
    /// let sparkline = Sparkline::new(data)
    ///     .style(SparklineStyle::Bar);
    /// let output = sparkline.render_string(10);
    /// assert!(output.len() > 0);
    /// ```
    pub fn render_string(&self, max_width: usize) -> String {
        if self.data.is_empty() {
            return String::new();
        }

        let mut output = String::new();

        match self.style {
            SparklineStyle::Bar => {
                for value in &self.data {
                    let normalized = self.normalize(*value);
                    output.push_str(Self::bar_char(normalized));

                    if output.len() >= max_width {
                        break;
                    }
                }
            }
            SparklineStyle::Line => {
                // For line style, use bar chars but connect them visually
                for value in &self.data {
                    let normalized = self.normalize(*value);
                    output.push_str(Self::bar_char(normalized));

                    if output.len() >= max_width {
                        break;
                    }
                }
            }
        }

        // Add markers if enabled
        if self.show_markers && !output.is_empty()
            && let (Some(min), Some(max), Some(avg)) = (self.min(), self.max(), self.avg())
        {
            output.push_str(&format!(" ↓{:.1} ↑{:.1} ~{:.1}", min, max, avg));
        }

        output
    }
}

impl Widget for Sparkline {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width == 0 || area.height == 0 || self.data.is_empty() {
            return;
        }

        let max_width = area.width as usize;
        let sparkline_str = self.render_string(max_width);

        // Render with appropriate coloring
        let mut x = area.x;
        let y = area.y;

        for (i, ch) in sparkline_str.chars().enumerate() {
            if x >= area.x + area.width {
                break;
            }

            // Get color for this data point
            let color = if i < self.data.len() {
                self.get_color(self.data[i])
            } else {
                self.color
            };

            let style = Style::default().fg(color);
            buf.set_string(x, y, ch.to_string(), style);
            x += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sparkline_creation() {
        let data = vec![1.0, 2.0, 3.0];
        let sparkline = Sparkline::new(data);
        assert_eq!(sparkline.data_points(), 3);
    }

    #[test]
    fn test_sparkline_empty() {
        let sparkline = Sparkline::empty();
        assert_eq!(sparkline.data_points(), 0);
    }

    #[test]
    fn test_sparkline_push() {
        let mut sparkline = Sparkline::empty();
        sparkline.push(1.0);
        sparkline.push(2.0);
        assert_eq!(sparkline.data_points(), 2);
    }

    #[test]
    fn test_sparkline_clear() {
        let mut sparkline = Sparkline::new(vec![1.0, 2.0, 3.0]);
        assert_eq!(sparkline.data_points(), 3);

        sparkline.clear();
        assert_eq!(sparkline.data_points(), 0);
    }

    #[test]
    fn test_sparkline_max() {
        let sparkline = Sparkline::new(vec![1.0, 5.0, 3.0, 2.0]);
        assert_eq!(sparkline.max(), Some(5.0));
    }

    #[test]
    fn test_sparkline_max_empty() {
        let sparkline = Sparkline::empty();
        assert_eq!(sparkline.max(), None);
    }

    #[test]
    fn test_sparkline_min() {
        let sparkline = Sparkline::new(vec![3.0, 1.0, 5.0, 2.0]);
        assert_eq!(sparkline.min(), Some(1.0));
    }

    #[test]
    fn test_sparkline_min_empty() {
        let sparkline = Sparkline::empty();
        assert_eq!(sparkline.min(), None);
    }

    #[test]
    fn test_sparkline_avg() {
        let sparkline = Sparkline::new(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        assert_eq!(sparkline.avg(), Some(3.0));
    }

    #[test]
    fn test_sparkline_avg_empty() {
        let sparkline = Sparkline::empty();
        assert_eq!(sparkline.avg(), None);
    }

    #[test]
    fn test_sparkline_builder_style() {
        let sparkline = Sparkline::empty()
            .style(SparklineStyle::Line);
        assert_eq!(sparkline.style, SparklineStyle::Line);
    }

    #[test]
    fn test_sparkline_builder_color() {
        let sparkline = Sparkline::empty()
            .color(Color::Cyan);
        assert_eq!(sparkline.color, Color::Cyan);
    }

    #[test]
    fn test_sparkline_builder_markers() {
        let sparkline = Sparkline::empty()
            .show_markers(true);
        assert!(sparkline.show_markers);
    }

    #[test]
    fn test_sparkline_builder_gradient() {
        let sparkline = Sparkline::empty()
            .gradient(Color::Green, Color::Red);
        assert!(sparkline.use_gradient);
        assert_eq!(sparkline.gradient_start, Color::Green);
        assert_eq!(sparkline.gradient_end, Color::Red);
    }

    #[test]
    fn test_sparkline_normalize() {
        let sparkline = Sparkline::new(vec![0.0, 5.0, 10.0]);
        assert_eq!(sparkline.normalize(0.0), 0.0);
        assert_eq!(sparkline.normalize(5.0), 0.5);
        assert_eq!(sparkline.normalize(10.0), 1.0);
    }

    #[test]
    fn test_sparkline_normalize_equal_values() {
        let sparkline = Sparkline::new(vec![5.0, 5.0, 5.0]);
        assert_eq!(sparkline.normalize(5.0), 0.5);
    }

    #[test]
    fn test_sparkline_bar_char() {
        assert_eq!(Sparkline::bar_char(0.0), " ");
        assert_eq!(Sparkline::bar_char(0.5), "▄");
        assert_eq!(Sparkline::bar_char(1.0), "█");
    }

    #[test]
    fn test_sparkline_render_string_empty() {
        let sparkline = Sparkline::empty();
        let output = sparkline.render_string(10);
        assert_eq!(output, "");
    }

    #[test]
    fn test_sparkline_render_string_bar() {
        let sparkline = Sparkline::new(vec![0.0, 5.0, 10.0])
            .style(SparklineStyle::Bar);
        let output = sparkline.render_string(10);
        assert!(!output.is_empty());
    }

    #[test]
    fn test_sparkline_render_string_with_markers() {
        let sparkline = Sparkline::new(vec![1.0, 2.0, 3.0])
            .show_markers(true);
        let output = sparkline.render_string(50);
        assert!(output.contains("↓"));
        assert!(output.contains("↑"));
        assert!(output.contains("~"));
    }

    #[test]
    fn test_sparkline_set_color() {
        let mut sparkline = Sparkline::empty();
        sparkline.set_color(Color::Magenta);
        assert_eq!(sparkline.color, Color::Magenta);
    }

    #[test]
    fn test_sparkline_chained_builders() {
        let sparkline = Sparkline::new(vec![1.0, 2.0, 3.0])
            .style(SparklineStyle::Line)
            .color(Color::Cyan)
            .show_markers(true)
            .gradient(Color::Green, Color::Red);

        assert_eq!(sparkline.style, SparklineStyle::Line);
        assert_eq!(sparkline.color, Color::Cyan);
        assert!(sparkline.show_markers);
        assert!(sparkline.use_gradient);
    }
}
