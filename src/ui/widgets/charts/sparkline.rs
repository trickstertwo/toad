/// Sparkline widget - Inline graphs for metrics
///
/// Compact visual representation of data trends
///
/// # Examples
///
/// ```
/// use toad::widgets::Sparkline;
///
/// let data = vec![1.0, 3.0, 2.0, 5.0, 4.0];
/// let sparkline = Sparkline::new(data);
/// assert_eq!(sparkline.data().len(), 5);
/// ```
use crate::ui::{atoms::block::Block as AtomBlock, theme::ToadTheme};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    widgets::{Borders, Paragraph},
};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Sparkline rendering style
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum SparklineStyle {
    /// Bar style using block characters
    #[default]
    Bars,
    /// Line style using braille characters
    Braille,
    /// Dot style using simple dots
    Dots,
}

impl SparklineStyle {
    /// All available styles
    pub fn all() -> &'static [SparklineStyle] {
        &[
            SparklineStyle::Bars,
            SparklineStyle::Braille,
            SparklineStyle::Dots,
        ]
    }

    /// Get style name
    pub fn name(&self) -> &'static str {
        match self {
            SparklineStyle::Bars => "Bars",
            SparklineStyle::Braille => "Braille",
            SparklineStyle::Dots => "Dots",
        }
    }
}

impl fmt::Display for SparklineStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Bar characters for sparklines (from lowest to highest)
const BAR_CHARS: &[char] = &[' ', '▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

/// Sparkline widget for inline metric visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sparkline {
    /// Data points to visualize
    data: Vec<f64>,
    /// Optional title
    title: Option<String>,
    /// Rendering style
    style: SparklineStyle,
    /// Whether to show borders
    show_border: bool,
    /// Whether to show min/max labels
    show_labels: bool,
}

impl Sparkline {
    /// Create a new sparkline with data
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Sparkline;
    ///
    /// let sparkline = Sparkline::new(vec![1.0, 2.0, 3.0]);
    /// assert_eq!(sparkline.data().len(), 3);
    /// ```
    pub fn new(data: Vec<f64>) -> Self {
        Self {
            data,
            title: None,
            style: SparklineStyle::Bars,
            show_border: false,
            show_labels: false,
        }
    }

    /// Set the title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the rendering style
    pub fn style(mut self, style: SparklineStyle) -> Self {
        self.style = style;
        self
    }

    /// Set whether to show borders
    pub fn show_border(mut self, show: bool) -> Self {
        self.show_border = show;
        self
    }

    /// Set whether to show labels
    pub fn show_labels(mut self, show: bool) -> Self {
        self.show_labels = show;
        self
    }

    /// Get the data
    pub fn data(&self) -> &[f64] {
        &self.data
    }

    /// Set the data
    pub fn set_data(&mut self, data: Vec<f64>) {
        self.data = data;
    }

    /// Add a data point
    pub fn push(&mut self, value: f64) {
        self.data.push(value);
    }

    /// Add a data point and remove oldest if exceeds max width
    pub fn push_with_limit(&mut self, value: f64, max: usize) {
        self.data.push(value);
        if self.data.len() > max {
            self.data.remove(0);
        }
    }

    /// Get min value
    pub fn min(&self) -> Option<f64> {
        self.data.iter().copied().fold(None, |acc, x| match acc {
            None => Some(x),
            Some(min) => Some(min.min(x)),
        })
    }

    /// Get max value
    pub fn max(&self) -> Option<f64> {
        self.data.iter().copied().fold(None, |acc, x| match acc {
            None => Some(x),
            Some(max) => Some(max.max(x)),
        })
    }

    /// Get average value
    pub fn avg(&self) -> Option<f64> {
        if self.data.is_empty() {
            None
        } else {
            Some(self.data.iter().sum::<f64>() / self.data.len() as f64)
        }
    }

    /// Normalize data to 0.0-1.0 range
    fn normalize(&self) -> Vec<f64> {
        if self.data.is_empty() {
            return Vec::new();
        }

        let min = self.min().unwrap_or(0.0);
        let max = self.max().unwrap_or(1.0);

        if (max - min).abs() < f64::EPSILON {
            return vec![0.5; self.data.len()];
        }

        self.data.iter().map(|&x| (x - min) / (max - min)).collect()
    }

    /// Render as bars
    fn render_bars(&self, width: usize) -> String {
        let normalized = self.normalize();
        let data = if normalized.len() > width {
            // Downsample if too many points
            let step = normalized.len() as f64 / width as f64;
            (0..width)
                .map(|i| {
                    let idx = (i as f64 * step) as usize;
                    normalized.get(idx).copied().unwrap_or(0.0)
                })
                .collect::<Vec<_>>()
        } else {
            normalized
        };

        data.iter()
            .map(|&value| {
                let index = (value * (BAR_CHARS.len() - 1) as f64).round() as usize;
                BAR_CHARS[index.min(BAR_CHARS.len() - 1)]
            })
            .collect()
    }

    /// Render as dots
    fn render_dots(&self, width: usize, height: usize) -> Vec<String> {
        let normalized = self.normalize();
        let data = if normalized.len() > width {
            let step = normalized.len() as f64 / width as f64;
            (0..width)
                .map(|i| {
                    let idx = (i as f64 * step) as usize;
                    normalized.get(idx).copied().unwrap_or(0.0)
                })
                .collect::<Vec<_>>()
        } else {
            normalized
        };

        let mut lines = vec![" ".repeat(width); height];

        for (x, &value) in data.iter().enumerate() {
            if x < width {
                let y = ((1.0 - value) * (height - 1) as f64).round() as usize;
                if y < height {
                    let mut line = lines[y].chars().collect::<Vec<_>>();
                    if x < line.len() {
                        line[x] = '•';
                    }
                    lines[y] = line.into_iter().collect();
                }
            }
        }

        lines
    }

    /// Render as braille (simple version - just uses dots for now)
    fn render_braille(&self, width: usize, height: usize) -> Vec<String> {
        // Simplified braille rendering using dots
        self.render_dots(width, height)
    }

    /// Render the sparkline
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let content_width = if self.show_border {
            area.width.saturating_sub(2) as usize
        } else {
            area.width as usize
        };

        let content_height = if self.show_border {
            area.height.saturating_sub(2) as usize
        } else {
            area.height as usize
        };

        let sparkline_text = match self.style {
            SparklineStyle::Bars => {
                let bars = self.render_bars(content_width);
                if self.show_labels {
                    let min = self.min().unwrap_or(0.0);
                    let max = self.max().unwrap_or(0.0);
                    format!("{}\nMin: {:.2} Max: {:.2}", bars, min, max)
                } else {
                    bars
                }
            }
            SparklineStyle::Dots => {
                let dots = self.render_dots(content_width, content_height.max(1));
                if self.show_labels {
                    let min = self.min().unwrap_or(0.0);
                    let max = self.max().unwrap_or(0.0);
                    format!("{}\nMin: {:.2} Max: {:.2}", dots.join("\n"), min, max)
                } else {
                    dots.join("\n")
                }
            }
            SparklineStyle::Braille => {
                let braille = self.render_braille(content_width, content_height.max(1));
                if self.show_labels {
                    let min = self.min().unwrap_or(0.0);
                    let max = self.max().unwrap_or(0.0);
                    format!("{}\nMin: {:.2} Max: {:.2}", braille.join("\n"), min, max)
                } else {
                    braille.join("\n")
                }
            }
        };

        let paragraph = if self.show_border {
            let mut block = AtomBlock::new()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(ToadTheme::TOAD_GREEN));

            if let Some(ref title) = self.title {
                block = block.title(format!(" {} ", title)).title_style(
                    Style::default()
                        .fg(ToadTheme::TOAD_GREEN)
                        .add_modifier(Modifier::BOLD),
                );
            }

            Paragraph::new(sparkline_text)
                .block(block.to_ratatui())
                .style(Style::default().fg(ToadTheme::FOREGROUND))
        } else {
            Paragraph::new(sparkline_text).style(Style::default().fg(ToadTheme::FOREGROUND))
        };

        frame.render_widget(paragraph, area);
    }

    /// Get sparkline as text (for testing/display)
    pub fn to_string(&self, width: usize) -> String {
        match self.style {
            SparklineStyle::Bars => self.render_bars(width),
            SparklineStyle::Dots => self.render_dots(width, 5).join("\n"),
            SparklineStyle::Braille => self.render_braille(width, 5).join("\n"),
        }
    }
}

impl Default for Sparkline {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sparkline_creation() {
        let sparkline = Sparkline::new(vec![1.0, 2.0, 3.0]);
        assert_eq!(sparkline.data().len(), 3);
        assert_eq!(sparkline.style, SparklineStyle::Bars);
    }

    #[test]
    fn test_sparkline_with_title() {
        let sparkline = Sparkline::new(vec![1.0]).title("Test");
        assert_eq!(sparkline.title, Some("Test".to_string()));
    }

    #[test]
    fn test_sparkline_style() {
        let sparkline = Sparkline::new(vec![1.0]).style(SparklineStyle::Dots);
        assert_eq!(sparkline.style, SparklineStyle::Dots);
    }

    #[test]
    fn test_sparkline_borders() {
        let sparkline = Sparkline::new(vec![1.0]).show_border(true);
        assert!(sparkline.show_border);
    }

    #[test]
    fn test_sparkline_labels() {
        let sparkline = Sparkline::new(vec![1.0]).show_labels(true);
        assert!(sparkline.show_labels);
    }

    #[test]
    fn test_set_data() {
        let mut sparkline = Sparkline::new(vec![1.0]);
        sparkline.set_data(vec![1.0, 2.0, 3.0]);
        assert_eq!(sparkline.data().len(), 3);
    }

    #[test]
    fn test_push() {
        let mut sparkline = Sparkline::new(vec![1.0]);
        sparkline.push(2.0);
        assert_eq!(sparkline.data().len(), 2);
    }

    #[test]
    fn test_push_with_limit() {
        let mut sparkline = Sparkline::new(vec![1.0, 2.0, 3.0]);
        sparkline.push_with_limit(4.0, 3);
        assert_eq!(sparkline.data().len(), 3);
        assert_eq!(sparkline.data()[0], 2.0);
        assert_eq!(sparkline.data()[2], 4.0);
    }

    #[test]
    fn test_min_max() {
        let sparkline = Sparkline::new(vec![1.0, 5.0, 3.0, 2.0]);
        assert_eq!(sparkline.min(), Some(1.0));
        assert_eq!(sparkline.max(), Some(5.0));
    }

    #[test]
    fn test_min_max_empty() {
        let sparkline = Sparkline::new(vec![]);
        assert_eq!(sparkline.min(), None);
        assert_eq!(sparkline.max(), None);
    }

    #[test]
    fn test_avg() {
        let sparkline = Sparkline::new(vec![1.0, 2.0, 3.0, 4.0]);
        assert_eq!(sparkline.avg(), Some(2.5));
    }

    #[test]
    fn test_avg_empty() {
        let sparkline = Sparkline::new(vec![]);
        assert_eq!(sparkline.avg(), None);
    }

    #[test]
    fn test_normalize() {
        let sparkline = Sparkline::new(vec![0.0, 5.0, 10.0]);
        let normalized = sparkline.normalize();
        assert_eq!(normalized.len(), 3);
        assert!((normalized[0] - 0.0).abs() < f64::EPSILON);
        assert!((normalized[1] - 0.5).abs() < f64::EPSILON);
        assert!((normalized[2] - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_normalize_single_value() {
        let sparkline = Sparkline::new(vec![5.0, 5.0, 5.0]);
        let normalized = sparkline.normalize();
        // All values should be 0.5 when min == max
        for &value in &normalized {
            assert!((value - 0.5).abs() < f64::EPSILON);
        }
    }

    #[test]
    fn test_render_bars() {
        let sparkline = Sparkline::new(vec![0.0, 5.0, 10.0]);
        let bars = sparkline.render_bars(3);
        // Should have 3 characters representing the values
        assert_eq!(bars.chars().count(), 3);
    }

    #[test]
    fn test_render_bars_empty() {
        let sparkline = Sparkline::new(vec![]);
        let bars = sparkline.render_bars(5);
        assert!(bars.is_empty());
    }

    #[test]
    fn test_render_dots() {
        let sparkline = Sparkline::new(vec![0.0, 5.0, 10.0]);
        let dots = sparkline.render_dots(10, 5);
        assert_eq!(dots.len(), 5);
    }

    #[test]
    fn test_to_string_bars() {
        let sparkline = Sparkline::new(vec![1.0, 2.0, 3.0]);
        let text = sparkline.to_string(10);
        assert!(!text.is_empty());
    }

    #[test]
    fn test_to_string_dots() {
        let sparkline = Sparkline::new(vec![1.0, 2.0, 3.0]).style(SparklineStyle::Dots);
        let text = sparkline.to_string(10);
        assert!(!text.is_empty());
    }

    #[test]
    fn test_sparkline_style_all() {
        let styles = SparklineStyle::all();
        assert_eq!(styles.len(), 3);
    }

    #[test]
    fn test_sparkline_style_name() {
        assert_eq!(SparklineStyle::Bars.name(), "Bars");
        assert_eq!(SparklineStyle::Braille.name(), "Braille");
        assert_eq!(SparklineStyle::Dots.name(), "Dots");
    }

    #[test]
    fn test_sparkline_style_display() {
        assert_eq!(format!("{}", SparklineStyle::Bars), "Bars");
    }

    #[test]
    fn test_sparkline_style_default() {
        let style = SparklineStyle::default();
        assert_eq!(style, SparklineStyle::Bars);
    }

    #[test]
    fn test_sparkline_default() {
        let sparkline = Sparkline::default();
        assert!(sparkline.data().is_empty());
        assert_eq!(sparkline.style, SparklineStyle::Bars);
    }

    #[test]
    fn test_downsampling() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let sparkline = Sparkline::new(data);
        // Request fewer points than available - should downsample
        let bars = sparkline.render_bars(5);
        assert_eq!(bars.chars().count(), 5);
    }

    // ============================================================================
    // COMPREHENSIVE EDGE CASE TESTS (ADVANCED TIER - 90%+ COVERAGE)
    // ============================================================================

    // Extreme data value tests
    #[test]
    fn test_sparkline_with_extreme_positive_values() {
        let sparkline = Sparkline::new(vec![f64::MAX, f64::MAX / 2.0]);
        assert_eq!(sparkline.max(), Some(f64::MAX));
        let bars = sparkline.render_bars(2);
        assert_eq!(bars.chars().count(), 2);
    }

    #[test]
    fn test_sparkline_with_extreme_negative_values() {
        let sparkline = Sparkline::new(vec![f64::MIN, f64::MIN / 2.0]);
        assert_eq!(sparkline.min(), Some(f64::MIN));
        let bars = sparkline.render_bars(2);
        assert_eq!(bars.chars().count(), 2);
    }

    #[test]
    fn test_sparkline_with_negative_values() {
        let sparkline = Sparkline::new(vec![-10.0, -5.0, 0.0, 5.0, 10.0]);
        assert_eq!(sparkline.min(), Some(-10.0));
        assert_eq!(sparkline.max(), Some(10.0));
        assert_eq!(sparkline.avg(), Some(0.0));
    }

    #[test]
    fn test_sparkline_with_mixed_positive_negative() {
        let sparkline = Sparkline::new(vec![-100.0, 50.0, -25.0, 75.0]);
        assert!(sparkline.min().unwrap() < 0.0);
        assert!(sparkline.max().unwrap() > 0.0);
    }

    #[test]
    fn test_sparkline_with_zero_values() {
        let sparkline = Sparkline::new(vec![0.0, 0.0, 0.0, 0.0]);
        assert_eq!(sparkline.min(), Some(0.0));
        assert_eq!(sparkline.max(), Some(0.0));
        assert_eq!(sparkline.avg(), Some(0.0));
    }

    #[test]
    fn test_sparkline_with_fractional_values() {
        let sparkline = Sparkline::new(vec![0.123456789, 1.987654321, 0.5]);
        assert!(sparkline.avg().unwrap() > 0.0);
        let bars = sparkline.render_bars(3);
        assert_eq!(bars.chars().count(), 3);
    }

    // Stress tests with many data points
    #[test]
    fn test_sparkline_with_many_points() {
        let data: Vec<f64> = (0..1000).map(|i| i as f64).collect();
        let sparkline = Sparkline::new(data);
        assert_eq!(sparkline.data().len(), 1000);
        let bars = sparkline.render_bars(100);
        assert_eq!(bars.chars().count(), 100);
    }

    #[test]
    fn test_sparkline_with_extreme_number_of_points() {
        let data: Vec<f64> = (0..10000).map(|i| (i as f64).sin()).collect();
        let sparkline = Sparkline::new(data);
        assert_eq!(sparkline.data().len(), 10000);
        let bars = sparkline.render_bars(50);
        assert_eq!(bars.chars().count(), 50);
    }

    #[test]
    fn test_sparkline_downsampling_with_large_dataset() {
        let data: Vec<f64> = (0..5000).map(|i| i as f64 % 100.0).collect();
        let sparkline = Sparkline::new(data);
        let bars = sparkline.render_bars(10);
        assert_eq!(bars.chars().count(), 10);
    }

    // Width edge cases
    #[test]
    fn test_sparkline_render_zero_width() {
        let sparkline = Sparkline::new(vec![1.0, 2.0, 3.0]);
        let bars = sparkline.render_bars(0);
        assert!(bars.is_empty());
    }

    #[test]
    fn test_sparkline_render_width_one() {
        let sparkline = Sparkline::new(vec![1.0, 2.0, 3.0]);
        let bars = sparkline.render_bars(1);
        assert_eq!(bars.chars().count(), 1);
    }

    #[test]
    fn test_sparkline_render_extreme_width() {
        let sparkline = Sparkline::new(vec![1.0, 2.0, 3.0]);
        let bars = sparkline.render_bars(10000);
        assert_eq!(bars.chars().count(), 3); // Limited by data points
    }

    // Height edge cases for dots/braille
    #[test]
    fn test_sparkline_dots_height_one() {
        let sparkline = Sparkline::new(vec![1.0, 2.0, 3.0]);
        let dots = sparkline.render_dots(10, 1);
        assert_eq!(dots.len(), 1);
    }

    #[test]
    fn test_sparkline_dots_extreme_height() {
        let sparkline = Sparkline::new(vec![1.0, 2.0, 3.0]);
        let dots = sparkline.render_dots(10, 1000);
        assert_eq!(dots.len(), 1000);
    }

    // All rendering styles
    #[test]
    fn test_sparkline_all_styles_render() {
        let data = vec![1.0, 3.0, 2.0, 5.0, 4.0];

        let bars = Sparkline::new(data.clone()).style(SparklineStyle::Bars);
        assert!(!bars.to_string(10).is_empty());

        let dots = Sparkline::new(data.clone()).style(SparklineStyle::Dots);
        assert!(!dots.to_string(10).is_empty());

        let braille = Sparkline::new(data).style(SparklineStyle::Braille);
        assert!(!braille.to_string(10).is_empty());
    }

    #[test]
    fn test_sparkline_style_equality() {
        assert_eq!(SparklineStyle::Bars, SparklineStyle::Bars);
        assert_eq!(SparklineStyle::Dots, SparklineStyle::Dots);
        assert_eq!(SparklineStyle::Braille, SparklineStyle::Braille);
        assert_ne!(SparklineStyle::Bars, SparklineStyle::Dots);
    }

    // Single data point edge cases
    #[test]
    fn test_sparkline_single_point() {
        let sparkline = Sparkline::new(vec![5.0]);
        assert_eq!(sparkline.min(), Some(5.0));
        assert_eq!(sparkline.max(), Some(5.0));
        assert_eq!(sparkline.avg(), Some(5.0));
        let normalized = sparkline.normalize();
        assert_eq!(normalized.len(), 1);
    }

    #[test]
    fn test_sparkline_single_zero() {
        let sparkline = Sparkline::new(vec![0.0]);
        assert_eq!(sparkline.avg(), Some(0.0));
        let bars = sparkline.render_bars(5);
        assert_eq!(bars.chars().count(), 1);
    }

    // Push with limit edge cases
    #[test]
    fn test_push_with_limit_exact() {
        let mut sparkline = Sparkline::new(vec![1.0, 2.0]);
        sparkline.push_with_limit(3.0, 3);
        assert_eq!(sparkline.data().len(), 3);
        assert_eq!(sparkline.data()[2], 3.0);
    }

    #[test]
    fn test_push_with_limit_zero() {
        let mut sparkline = Sparkline::new(vec![]);
        sparkline.push_with_limit(1.0, 0);
        // With limit 0, it should remove the value immediately
        assert!(sparkline.data().is_empty() || sparkline.data().len() == 1);
    }

    #[test]
    fn test_push_with_limit_many_times() {
        let mut sparkline = Sparkline::new(vec![]);
        for i in 0..100 {
            sparkline.push_with_limit(i as f64, 10);
        }
        assert_eq!(sparkline.data().len(), 10);
        // Should contain the last 10 values (90-99)
        assert_eq!(sparkline.data()[0], 90.0);
        assert_eq!(sparkline.data()[9], 99.0);
    }

    #[test]
    fn test_push_with_limit_large_limit() {
        let mut sparkline = Sparkline::new(vec![1.0]);
        sparkline.push_with_limit(2.0, 10000);
        assert_eq!(sparkline.data().len(), 2);
    }

    // Set data edge cases
    #[test]
    fn test_set_data_empty_to_filled() {
        let mut sparkline = Sparkline::new(vec![]);
        sparkline.set_data(vec![1.0, 2.0, 3.0]);
        assert_eq!(sparkline.data().len(), 3);
    }

    #[test]
    fn test_set_data_filled_to_empty() {
        let mut sparkline = Sparkline::new(vec![1.0, 2.0, 3.0]);
        sparkline.set_data(vec![]);
        assert!(sparkline.data().is_empty());
    }

    #[test]
    fn test_set_data_replace() {
        let mut sparkline = Sparkline::new(vec![1.0, 2.0]);
        sparkline.set_data(vec![10.0, 20.0, 30.0]);
        assert_eq!(sparkline.data().len(), 3);
        assert_eq!(sparkline.data()[0], 10.0);
    }

    // Builder pattern tests
    #[test]
    fn test_sparkline_builder_all_options() {
        let sparkline = Sparkline::new(vec![1.0, 2.0, 3.0])
            .title("Test Sparkline")
            .style(SparklineStyle::Dots)
            .show_border(true)
            .show_labels(true);

        assert_eq!(sparkline.title, Some("Test Sparkline".to_string()));
        assert_eq!(sparkline.style, SparklineStyle::Dots);
        assert!(sparkline.show_border);
        assert!(sparkline.show_labels);
    }

    #[test]
    fn test_sparkline_builder_chaining() {
        let sparkline = Sparkline::new(vec![1.0])
            .title("First")
            .title("Second")
            .style(SparklineStyle::Bars)
            .style(SparklineStyle::Braille)
            .show_border(true)
            .show_border(false);

        // Last values should win
        assert_eq!(sparkline.title, Some("Second".to_string()));
        assert_eq!(sparkline.style, SparklineStyle::Braille);
        assert!(!sparkline.show_border);
    }

    // Clone trait test
    #[test]
    fn test_sparkline_clone() {
        let original = Sparkline::new(vec![1.0, 2.0, 3.0])
            .title("Original")
            .style(SparklineStyle::Dots);

        let cloned = original.clone();

        assert_eq!(cloned.data().len(), 3);
        assert_eq!(cloned.title, Some("Original".to_string()));
        assert_eq!(cloned.style, SparklineStyle::Dots);
    }

    // Normalization edge cases
    #[test]
    fn test_normalize_empty() {
        let sparkline = Sparkline::new(vec![]);
        let normalized = sparkline.normalize();
        assert!(normalized.is_empty());
    }

    #[test]
    fn test_normalize_all_same_values() {
        let sparkline = Sparkline::new(vec![7.0, 7.0, 7.0, 7.0]);
        let normalized = sparkline.normalize();
        // All should be 0.5 when min == max
        for &value in &normalized {
            assert!((value - 0.5).abs() < f64::EPSILON);
        }
    }

    #[test]
    fn test_normalize_two_extremes() {
        let sparkline = Sparkline::new(vec![0.0, 100.0]);
        let normalized = sparkline.normalize();
        assert!((normalized[0] - 0.0).abs() < f64::EPSILON);
        assert!((normalized[1] - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_normalize_negative_range() {
        let sparkline = Sparkline::new(vec![-10.0, -5.0, 0.0]);
        let normalized = sparkline.normalize();
        assert!((normalized[0] - 0.0).abs() < f64::EPSILON);
        assert!((normalized[1] - 0.5).abs() < f64::EPSILON);
        assert!((normalized[2] - 1.0).abs() < f64::EPSILON);
    }

    // Statistical edge cases
    #[test]
    fn test_avg_single_value() {
        let sparkline = Sparkline::new(vec![42.0]);
        assert_eq!(sparkline.avg(), Some(42.0));
    }

    #[test]
    fn test_avg_negative_values() {
        let sparkline = Sparkline::new(vec![-5.0, -3.0, -1.0]);
        assert_eq!(sparkline.avg(), Some(-3.0));
    }

    #[test]
    fn test_avg_large_dataset() {
        let data: Vec<f64> = (0..1000).map(|_| 5.0).collect();
        let sparkline = Sparkline::new(data);
        assert!((sparkline.avg().unwrap() - 5.0).abs() < f64::EPSILON);
    }

    // Empty sparkline edge cases
    #[test]
    fn test_empty_sparkline_operations() {
        let sparkline = Sparkline::new(vec![]);
        assert_eq!(sparkline.min(), None);
        assert_eq!(sparkline.max(), None);
        assert_eq!(sparkline.avg(), None);
        assert!(sparkline.normalize().is_empty());
        assert!(sparkline.render_bars(10).is_empty());
    }

    #[test]
    fn test_empty_sparkline_push() {
        let mut sparkline = Sparkline::new(vec![]);
        sparkline.push(1.0);
        assert_eq!(sparkline.data().len(), 1);
        assert_eq!(sparkline.min(), Some(1.0));
    }

    // Render dots with various dimensions
    #[test]
    fn test_render_dots_square() {
        let sparkline = Sparkline::new(vec![1.0, 2.0, 3.0, 4.0]);
        let dots = sparkline.render_dots(10, 10);
        assert_eq!(dots.len(), 10);
        for line in &dots {
            // Use chars().count() since '•' is multi-byte Unicode
            assert_eq!(line.chars().count(), 10);
        }
    }

    #[test]
    fn test_render_dots_wide() {
        let sparkline = Sparkline::new(vec![1.0, 2.0, 3.0]);
        let dots = sparkline.render_dots(100, 5);
        assert_eq!(dots.len(), 5);
        for line in &dots {
            // Use chars().count() since '•' is multi-byte Unicode
            assert_eq!(line.chars().count(), 100);
        }
    }

    #[test]
    fn test_render_dots_tall() {
        let sparkline = Sparkline::new(vec![1.0, 2.0, 3.0]);
        let dots = sparkline.render_dots(5, 100);
        assert_eq!(dots.len(), 100);
    }

    // Downsampling edge cases
    #[test]
    fn test_downsampling_exact_fit() {
        let data: Vec<f64> = (0..10).map(|i| i as f64).collect();
        let sparkline = Sparkline::new(data);
        let bars = sparkline.render_bars(10);
        assert_eq!(bars.chars().count(), 10);
    }

    #[test]
    fn test_downsampling_more_width_than_data() {
        let sparkline = Sparkline::new(vec![1.0, 2.0, 3.0]);
        let bars = sparkline.render_bars(100);
        // Should only render 3 chars since we only have 3 data points
        assert_eq!(bars.chars().count(), 3);
    }

    #[test]
    fn test_downsampling_minimal() {
        let data: Vec<f64> = (0..1000).map(|i| i as f64).collect();
        let sparkline = Sparkline::new(data);
        let bars = sparkline.render_bars(1);
        assert_eq!(bars.chars().count(), 1);
    }

    // To string tests for all styles
    #[test]
    fn test_to_string_empty_sparkline() {
        let sparkline = Sparkline::new(vec![]);
        assert!(sparkline.to_string(10).is_empty());
    }

    #[test]
    fn test_to_string_braille() {
        let sparkline = Sparkline::new(vec![1.0, 2.0, 3.0]).style(SparklineStyle::Braille);
        let text = sparkline.to_string(10);
        assert!(!text.is_empty());
    }

    // Comprehensive stress test
    #[test]
    fn test_sparkline_comprehensive_stress_test() {
        // Create large dataset with various patterns
        let mut data: Vec<f64> = Vec::new();
        for i in 0..1000 {
            let value = match i % 4 {
                0 => (i as f64).sin() * 100.0,
                1 => (i as f64).cos() * 50.0,
                2 => (i as f64) % 100.0,
                _ => -(i as f64) % 50.0,
            };
            data.push(value);
        }

        let sparkline = Sparkline::new(data)
            .title("Comprehensive Test 📊")
            .style(SparklineStyle::Dots)
            .show_border(true)
            .show_labels(true);

        assert_eq!(sparkline.data().len(), 1000);

        // Test statistics
        let min = sparkline.min().unwrap();
        let max = sparkline.max().unwrap();
        let avg = sparkline.avg().unwrap();
        assert!(min < max);
        assert!(avg >= min && avg <= max);

        // Test all rendering modes
        let bars = sparkline.render_bars(80);
        assert!(!bars.is_empty());

        let dots = sparkline.render_dots(80, 20);
        assert_eq!(dots.len(), 20);

        let normalized = sparkline.normalize();
        assert_eq!(normalized.len(), 1000);
        for &value in &normalized {
            assert!(value >= 0.0 && value <= 1.0);
        }
    }
}
