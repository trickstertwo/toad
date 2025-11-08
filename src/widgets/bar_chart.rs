//! Bar chart widget for comparison visualization
//!
//! Displays data as vertical or horizontal bars, ideal for comparing values.
//!
//! # Examples
//!
//! ```
//! use toad::widgets::{BarChart, BarData};
//! use ratatui::style::Color;
//!
//! let data = vec![
//!     BarData::new("Q1", 100.0).with_color(Color::Blue),
//!     BarData::new("Q2", 150.0).with_color(Color::Green),
//!     BarData::new("Q3", 120.0).with_color(Color::Yellow),
//! ];
//!
//! let chart = BarChart::new(data)
//!     .with_title("Quarterly Sales")
//!     .with_value_label("Revenue ($K)");
//! ```

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Widget},
};

/// Direction for bar rendering
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BarDirection {
    /// Vertical bars (default)
    #[default]
    Vertical,
    /// Horizontal bars
    Horizontal,
}

/// A single bar in a bar chart
///
/// # Examples
///
/// ```
/// use toad::widgets::BarData;
/// use ratatui::style::Color;
///
/// let bar = BarData::new("Sales", 150.0)
///     .with_color(Color::Blue);
/// ```
#[derive(Debug, Clone)]
pub struct BarData {
    /// Bar label
    pub label: String,
    /// Bar value
    pub value: f64,
    /// Bar color
    pub color: Color,
}

impl BarData {
    /// Create a new bar
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::BarData;
    ///
    /// let bar = BarData::new("Item", 42.0);
    /// assert_eq!(bar.label, "Item");
    /// assert_eq!(bar.value, 42.0);
    /// ```
    pub fn new(label: impl Into<String>, value: f64) -> Self {
        Self {
            label: label.into(),
            value,
            color: Color::Cyan,
        }
    }

    /// Set bar color
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::BarData;
    /// use ratatui::style::Color;
    ///
    /// let bar = BarData::new("Item", 10.0)
    ///     .with_color(Color::Red);
    /// assert_eq!(bar.color, Color::Red);
    /// ```
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

/// Bar chart widget
///
/// Displays data as vertical or horizontal bars for easy comparison.
///
/// # Examples
///
/// ```
/// use toad::widgets::{BarChart, BarData};
///
/// let data = vec![
///     BarData::new("A", 10.0),
///     BarData::new("B", 20.0),
/// ];
///
/// let chart = BarChart::new(data)
///     .with_title("Comparison");
/// ```
#[derive(Debug, Clone)]
pub struct BarChart {
    /// Bar data
    bars: Vec<BarData>,
    /// Chart title
    title: Option<String>,
    /// Value axis label
    value_label: Option<String>,
    /// Bar direction
    direction: BarDirection,
    /// Show values on bars
    show_values: bool,
    /// Maximum value (for scaling)
    max_value: Option<f64>,
}

impl BarChart {
    /// Create a new bar chart
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{BarChart, BarData};
    ///
    /// let data = vec![BarData::new("A", 10.0)];
    /// let chart = BarChart::new(data);
    /// ```
    pub fn new(bars: Vec<BarData>) -> Self {
        Self {
            bars,
            title: None,
            value_label: None,
            direction: BarDirection::default(),
            show_values: true,
            max_value: None,
        }
    }

    /// Set chart title
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{BarChart, BarData};
    ///
    /// let chart = BarChart::new(vec![BarData::new("A", 10.0)])
    ///     .with_title("Sales");
    /// ```
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set value axis label
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{BarChart, BarData};
    ///
    /// let chart = BarChart::new(vec![BarData::new("A", 10.0)])
    ///     .with_value_label("Count");
    /// ```
    pub fn with_value_label(mut self, label: impl Into<String>) -> Self {
        self.value_label = Some(label.into());
        self
    }

    /// Set bar direction
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{BarChart, BarData, BarDirection};
    ///
    /// let chart = BarChart::new(vec![BarData::new("A", 10.0)])
    ///     .with_direction(BarDirection::Horizontal);
    /// ```
    pub fn with_direction(mut self, direction: BarDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Show or hide values on bars
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{BarChart, BarData};
    ///
    /// let chart = BarChart::new(vec![BarData::new("A", 10.0)])
    ///     .with_values(false);
    /// ```
    pub fn with_values(mut self, show: bool) -> Self {
        self.show_values = show;
        self
    }

    /// Set maximum value for scaling
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{BarChart, BarData};
    ///
    /// let chart = BarChart::new(vec![BarData::new("A", 10.0)])
    ///     .with_max_value(100.0);
    /// ```
    pub fn with_max_value(mut self, max: f64) -> Self {
        self.max_value = Some(max);
        self
    }

    /// Get number of bars
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{BarChart, BarData};
    ///
    /// let chart = BarChart::new(vec![
    ///     BarData::new("A", 10.0),
    ///     BarData::new("B", 20.0),
    /// ]);
    /// assert_eq!(chart.bar_count(), 2);
    /// ```
    pub fn bar_count(&self) -> usize {
        self.bars.len()
    }

    /// Get maximum value from data
    fn calculate_max_value(&self) -> f64 {
        if let Some(max) = self.max_value {
            return max;
        }

        self.bars
            .iter()
            .map(|b| b.value)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(1.0)
    }

    /// Render as text lines
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{BarChart, BarData};
    ///
    /// let chart = BarChart::new(vec![BarData::new("A", 10.0)]);
    /// let lines = chart.render_lines(40, 20);
    /// assert!(!lines.is_empty());
    /// ```
    pub fn render_lines(&self, width: u16, height: u16) -> Vec<Line<'static>> {
        let mut lines = Vec::new();

        // Title
        if let Some(title) = &self.title {
            lines.push(Line::from(Span::styled(
                title.clone(),
                Style::default().fg(Color::Cyan),
            )));
        }

        if self.bars.is_empty() {
            lines.push(Line::from("No data"));
            return lines;
        }

        match self.direction {
            BarDirection::Vertical => self.render_vertical(&mut lines, width, height),
            BarDirection::Horizontal => self.render_horizontal(&mut lines, width, height),
        }

        lines
    }

    fn render_vertical(&self, lines: &mut Vec<Line<'static>>, width: u16, height: u16) {
        let max_value = self.calculate_max_value();
        let chart_height = height.saturating_sub(5); // Reserve space for labels

        // Value label
        if let Some(label) = &self.value_label {
            lines.push(Line::from(format!("Y: {}", label)));
        }

        let bar_width = (width as usize).saturating_sub(10) / self.bars.len().max(1);

        // Render bars from top to bottom
        for h in 0..chart_height {
            let threshold = max_value * (1.0 - h as f64 / chart_height as f64);
            let mut line_str = format!("{:>6.1} |", threshold);

            for bar in &self.bars {
                let bar_height = (bar.value / max_value * chart_height as f64) as u16;
                let filled = chart_height - h <= bar_height;

                for _ in 0..bar_width {
                    line_str.push(if filled { '█' } else { ' ' });
                }
            }

            lines.push(Line::from(line_str));
        }

        // X-axis
        let mut x_axis = String::from("       +");
        for _ in 0..(width.saturating_sub(10)) {
            x_axis.push('─');
        }
        lines.push(Line::from(x_axis));

        // Labels
        let mut label_line = String::from("        ");
        for bar in &self.bars {
            let padded_label = format!("{:^width$}", bar.label, width = bar_width);
            label_line.push_str(&padded_label);
        }
        lines.push(Line::from(label_line));

        // Values if enabled
        if self.show_values {
            let mut value_line = String::from("        ");
            for bar in &self.bars {
                let value_str = format!("{:.1}", bar.value);
                let padded = format!("{:^width$}", value_str, width = bar_width);
                value_line.push_str(&padded);
            }
            lines.push(Line::from(value_line));
        }
    }

    fn render_horizontal(&self, lines: &mut Vec<Line<'static>>, width: u16, _height: u16) {
        let max_value = self.calculate_max_value();
        let bar_width = width.saturating_sub(20) as f64;

        // Value label
        if let Some(label) = &self.value_label {
            lines.push(Line::from(format!("Value: {}", label)));
        }

        lines.push(Line::from(""));

        for bar in &self.bars {
            let filled_width = (bar.value / max_value * bar_width) as usize;
            let bar_str = "█".repeat(filled_width);

            let value_str = if self.show_values {
                format!(" {:.1}", bar.value)
            } else {
                String::new()
            };

            lines.push(Line::from(vec![
                Span::raw(format!("{:>10} |", bar.label)),
                Span::styled(bar_str, Style::default().fg(bar.color)),
                Span::raw(value_str),
            ]));
        }
    }
}

impl Widget for &BarChart {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let lines = self.render_lines(area.width, area.height);
        let block = Block::default().borders(Borders::ALL);
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
    fn test_bar_direction_default() {
        assert_eq!(BarDirection::default(), BarDirection::Vertical);
    }

    #[test]
    fn test_bar_data_new() {
        let bar = BarData::new("Test", 42.5);
        assert_eq!(bar.label, "Test");
        assert_eq!(bar.value, 42.5);
        assert_eq!(bar.color, Color::Cyan);
    }

    #[test]
    fn test_bar_data_with_color() {
        let bar = BarData::new("Test", 10.0).with_color(Color::Red);
        assert_eq!(bar.color, Color::Red);
    }

    #[test]
    fn test_bar_chart_new() {
        let data = vec![BarData::new("A", 10.0), BarData::new("B", 20.0)];
        let chart = BarChart::new(data);
        assert_eq!(chart.bar_count(), 2);
        assert!(chart.show_values);
        assert_eq!(chart.direction, BarDirection::Vertical);
    }

    #[test]
    fn test_bar_chart_with_title() {
        let chart = BarChart::new(vec![]).with_title("My Chart");
        assert_eq!(chart.title, Some("My Chart".to_string()));
    }

    #[test]
    fn test_bar_chart_with_value_label() {
        let chart = BarChart::new(vec![]).with_value_label("Count");
        assert_eq!(chart.value_label, Some("Count".to_string()));
    }

    #[test]
    fn test_bar_chart_with_direction() {
        let chart = BarChart::new(vec![]).with_direction(BarDirection::Horizontal);
        assert_eq!(chart.direction, BarDirection::Horizontal);
    }

    #[test]
    fn test_bar_chart_with_values() {
        let chart = BarChart::new(vec![]).with_values(false);
        assert!(!chart.show_values);
    }

    #[test]
    fn test_bar_chart_with_max_value() {
        let chart = BarChart::new(vec![]).with_max_value(100.0);
        assert_eq!(chart.max_value, Some(100.0));
    }

    #[test]
    fn test_bar_chart_bar_count() {
        let data = vec![
            BarData::new("A", 10.0),
            BarData::new("B", 20.0),
            BarData::new("C", 30.0),
        ];
        let chart = BarChart::new(data);
        assert_eq!(chart.bar_count(), 3);
    }

    #[test]
    fn test_bar_chart_calculate_max_value() {
        let data = vec![
            BarData::new("A", 10.0),
            BarData::new("B", 25.0),
            BarData::new("C", 15.0),
        ];
        let chart = BarChart::new(data);
        assert_eq!(chart.calculate_max_value(), 25.0);
    }

    #[test]
    fn test_bar_chart_manual_max_value() {
        let data = vec![BarData::new("A", 10.0)];
        let chart = BarChart::new(data).with_max_value(100.0);
        assert_eq!(chart.calculate_max_value(), 100.0);
    }

    #[test]
    fn test_bar_chart_empty_max_value() {
        let chart = BarChart::new(vec![]);
        assert_eq!(chart.calculate_max_value(), 1.0);
    }

    #[test]
    fn test_bar_chart_render_lines_empty() {
        let chart = BarChart::new(vec![]);
        let lines = chart.render_lines(40, 20);
        assert!(!lines.is_empty());
    }

    #[test]
    fn test_bar_chart_render_lines_vertical() {
        let data = vec![BarData::new("A", 10.0), BarData::new("B", 20.0)];
        let chart = BarChart::new(data).with_title("Test");
        let lines = chart.render_lines(40, 20);
        assert!(!lines.is_empty());
    }

    #[test]
    fn test_bar_chart_render_lines_horizontal() {
        let data = vec![BarData::new("A", 10.0), BarData::new("B", 20.0)];
        let chart = BarChart::new(data)
            .with_direction(BarDirection::Horizontal)
            .with_title("Test");
        let lines = chart.render_lines(40, 20);
        assert!(!lines.is_empty());
    }

    #[test]
    fn test_builder_pattern() {
        let chart = BarChart::new(vec![BarData::new("A", 10.0)])
            .with_title("Chart")
            .with_value_label("Value")
            .with_direction(BarDirection::Horizontal)
            .with_values(false)
            .with_max_value(50.0);

        assert_eq!(chart.title, Some("Chart".to_string()));
        assert_eq!(chart.value_label, Some("Value".to_string()));
        assert_eq!(chart.direction, BarDirection::Horizontal);
        assert!(!chart.show_values);
        assert_eq!(chart.max_value, Some(50.0));
    }
}
