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
use crate::ui::theme::ToadTheme;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    widgets::{Block, Borders, Paragraph},
};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Sparkline rendering style
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[derive(Default)]
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
            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(ToadTheme::TOAD_GREEN));

            let block = if let Some(ref title) = self.title {
                block.title(format!(" {} ", title)).title_style(
                    Style::default()
                        .fg(ToadTheme::TOAD_GREEN)
                        .add_modifier(Modifier::BOLD),
                )
            } else {
                block
            };

            Paragraph::new(sparkline_text)
                .block(block)
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
}
