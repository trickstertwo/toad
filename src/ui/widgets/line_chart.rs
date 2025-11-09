//! Line chart widget for time-series data visualization
//!
//! Displays data as connected lines, ideal for showing trends over time.
//!
//! # Examples
//!
//! ```
//! use toad::widgets::{LineChart, DataSeries};
//! use ratatui::style::Color;
//!
//! let data = vec![1.0, 2.0, 3.0, 4.0, 3.0, 2.0];
//! let series = DataSeries::new("Temperature", data).with_color(Color::Red);
//!
//! let chart = LineChart::new()
//!     .add_series(series)
//!     .with_title("Temperature Over Time")
//!     .with_x_label("Time")
//!     .with_y_label("°C");
//! ```

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Widget},
};

/// A data series for line charts
///
/// # Examples
///
/// ```
/// use toad::widgets::DataSeries;
/// use ratatui::style::Color;
///
/// let series = DataSeries::new("Sales", vec![100.0, 150.0, 120.0])
///     .with_color(Color::Blue);
/// ```
#[derive(Debug, Clone)]
pub struct DataSeries {
    /// Series name
    pub name: String,
    /// Data points (Y values)
    pub data: Vec<f64>,
    /// Line color
    pub color: Color,
    /// Show markers at data points
    pub show_markers: bool,
}

impl DataSeries {
    /// Create a new data series
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::DataSeries;
    ///
    /// let series = DataSeries::new("Revenue", vec![100.0, 200.0, 150.0]);
    /// assert_eq!(series.name, "Revenue");
    /// assert_eq!(series.data.len(), 3);
    /// ```
    pub fn new(name: impl Into<String>, data: Vec<f64>) -> Self {
        Self {
            name: name.into(),
            data,
            color: Color::Cyan,
            show_markers: false,
        }
    }

    /// Set the line color
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::DataSeries;
    /// use ratatui::style::Color;
    ///
    /// let series = DataSeries::new("Data", vec![1.0, 2.0])
    ///     .with_color(Color::Red);
    /// assert_eq!(series.color, Color::Red);
    /// ```
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Show markers at data points
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::DataSeries;
    ///
    /// let series = DataSeries::new("Data", vec![1.0, 2.0])
    ///     .with_markers(true);
    /// assert!(series.show_markers);
    /// ```
    pub fn with_markers(mut self, show: bool) -> Self {
        self.show_markers = show;
        self
    }

    /// Get minimum value
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::DataSeries;
    ///
    /// let series = DataSeries::new("Data", vec![3.0, 1.0, 2.0]);
    /// assert_eq!(series.min(), Some(1.0));
    /// ```
    pub fn min(&self) -> Option<f64> {
        self.data
            .iter()
            .copied()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
    }

    /// Get maximum value
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::DataSeries;
    ///
    /// let series = DataSeries::new("Data", vec![3.0, 1.0, 2.0]);
    /// assert_eq!(series.max(), Some(3.0));
    /// ```
    pub fn max(&self) -> Option<f64> {
        self.data
            .iter()
            .copied()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
    }
}

/// Line chart widget
///
/// Displays multiple data series as connected lines.
///
/// # Examples
///
/// ```
/// use toad::widgets::{LineChart, DataSeries};
///
/// let mut chart = LineChart::new();
/// chart.add_series(DataSeries::new("Series 1", vec![1.0, 2.0, 3.0]));
/// ```
#[derive(Debug, Clone)]
pub struct LineChart {
    /// Data series to display
    series: Vec<DataSeries>,
    /// Chart title
    title: Option<String>,
    /// X-axis label
    x_label: Option<String>,
    /// Y-axis label
    y_label: Option<String>,
    /// Show legend
    show_legend: bool,
    /// Show grid
    show_grid: bool,
    /// Manual Y-axis bounds (min, max)
    y_bounds: Option<(f64, f64)>,
}

impl LineChart {
    /// Create a new line chart
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::LineChart;
    ///
    /// let chart = LineChart::new();
    /// ```
    pub fn new() -> Self {
        Self {
            series: Vec::new(),
            title: None,
            x_label: None,
            y_label: None,
            show_legend: true,
            show_grid: false,
            y_bounds: None,
        }
    }

    /// Add a data series
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{LineChart, DataSeries};
    ///
    /// let chart = LineChart::new()
    ///     .add_series(DataSeries::new("Data", vec![1.0, 2.0]));
    /// assert_eq!(chart.series_count(), 1);
    /// ```
    pub fn add_series(mut self, series: DataSeries) -> Self {
        self.series.push(series);
        self
    }

    /// Set chart title
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::LineChart;
    ///
    /// let chart = LineChart::new()
    ///     .with_title("My Chart");
    /// ```
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set X-axis label
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::LineChart;
    ///
    /// let chart = LineChart::new()
    ///     .with_x_label("Time");
    /// ```
    pub fn with_x_label(mut self, label: impl Into<String>) -> Self {
        self.x_label = Some(label.into());
        self
    }

    /// Set Y-axis label
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::LineChart;
    ///
    /// let chart = LineChart::new()
    ///     .with_y_label("Value");
    /// ```
    pub fn with_y_label(mut self, label: impl Into<String>) -> Self {
        self.y_label = Some(label.into());
        self
    }

    /// Show or hide legend
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::LineChart;
    ///
    /// let chart = LineChart::new()
    ///     .with_legend(false);
    /// ```
    pub fn with_legend(mut self, show: bool) -> Self {
        self.show_legend = show;
        self
    }

    /// Show or hide grid
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::LineChart;
    ///
    /// let chart = LineChart::new()
    ///     .with_grid(true);
    /// ```
    pub fn with_grid(mut self, show: bool) -> Self {
        self.show_grid = show;
        self
    }

    /// Set Y-axis bounds manually
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::LineChart;
    ///
    /// let chart = LineChart::new()
    ///     .with_y_bounds(0.0, 100.0);
    /// ```
    pub fn with_y_bounds(mut self, min: f64, max: f64) -> Self {
        self.y_bounds = Some((min, max));
        self
    }

    /// Get number of series
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{LineChart, DataSeries};
    ///
    /// let chart = LineChart::new();
    /// assert_eq!(chart.series_count(), 0);
    ///
    /// let chart = chart.add_series(DataSeries::new("A", vec![1.0]));
    /// assert_eq!(chart.series_count(), 1);
    /// ```
    pub fn series_count(&self) -> usize {
        self.series.len()
    }

    /// Get Y-axis bounds (min, max)
    fn calculate_y_bounds(&self) -> (f64, f64) {
        if let Some(bounds) = self.y_bounds {
            return bounds;
        }

        let mut min = f64::MAX;
        let mut max = f64::MIN;

        for series in &self.series {
            if let Some(s_min) = series.min() {
                min = min.min(s_min);
            }
            if let Some(s_max) = series.max() {
                max = max.max(s_max);
            }
        }

        if min == f64::MAX || max == f64::MIN {
            return (0.0, 1.0);
        }

        // Add 10% padding
        let padding = (max - min) * 0.1;
        (min - padding, max + padding)
    }

    /// Render the chart as text lines
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{LineChart, DataSeries};
    ///
    /// let chart = LineChart::new()
    ///     .add_series(DataSeries::new("Data", vec![1.0, 2.0]));
    ///
    /// let lines = chart.render_lines(20, 10);
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

        if self.series.is_empty() {
            lines.push(Line::from("No data"));
            return lines;
        }

        // Y-axis label
        if let Some(label) = &self.y_label {
            lines.push(Line::from(format!("Y: {}", label)));
        }

        let (y_min, y_max) = self.calculate_y_bounds();
        let max_points = self.series.iter().map(|s| s.data.len()).max().unwrap_or(0);

        // Render chart area (simplified text representation)
        let chart_height = height.saturating_sub(4); // Reserve space for labels
        for h in 0..chart_height {
            let y_value = y_max - (h as f64 / chart_height as f64) * (y_max - y_min);
            let mut line_str = format!("{:>6.1} |", y_value);

            // Plot points for each series (simplified)
            for x in 0..width.saturating_sub(10) {
                let x_idx =
                    (x as f64 / (width.saturating_sub(10)) as f64 * max_points as f64) as usize;
                let mut found = false;

                for series in &self.series {
                    if x_idx < series.data.len() {
                        let value = series.data[x_idx];
                        let y_pos =
                            ((y_max - value) / (y_max - y_min) * chart_height as f64) as u16;
                        if y_pos == h {
                            line_str.push('●');
                            found = true;
                            break;
                        }
                    }
                }

                if !found {
                    line_str.push(' ');
                }
            }

            lines.push(Line::from(line_str));
        }

        // X-axis
        let mut x_axis = String::from("       +");
        for _ in 0..width.saturating_sub(10) {
            x_axis.push('─');
        }
        lines.push(Line::from(x_axis));

        // X-axis label
        if let Some(label) = &self.x_label {
            lines.push(Line::from(format!("        X: {}", label)));
        }

        // Legend
        if self.show_legend && !self.series.is_empty() {
            lines.push(Line::from(""));
            lines.push(Line::from("Legend:"));
            for series in &self.series {
                lines.push(Line::from(vec![
                    Span::styled("  ●", Style::default().fg(series.color)),
                    Span::raw(format!(" {}", series.name)),
                ]));
            }
        }

        lines
    }
}

impl Default for LineChart {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for &LineChart {
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
    fn test_data_series_new() {
        let series = DataSeries::new("Test", vec![1.0, 2.0, 3.0]);
        assert_eq!(series.name, "Test");
        assert_eq!(series.data.len(), 3);
        assert_eq!(series.color, Color::Cyan);
        assert!(!series.show_markers);
    }

    #[test]
    fn test_data_series_with_color() {
        let series = DataSeries::new("Test", vec![1.0]).with_color(Color::Red);
        assert_eq!(series.color, Color::Red);
    }

    #[test]
    fn test_data_series_with_markers() {
        let series = DataSeries::new("Test", vec![1.0]).with_markers(true);
        assert!(series.show_markers);
    }

    #[test]
    fn test_data_series_min_max() {
        let series = DataSeries::new("Test", vec![3.0, 1.0, 5.0, 2.0]);
        assert_eq!(series.min(), Some(1.0));
        assert_eq!(series.max(), Some(5.0));
    }

    #[test]
    fn test_data_series_empty() {
        let series = DataSeries::new("Test", vec![]);
        assert_eq!(series.min(), None);
        assert_eq!(series.max(), None);
    }

    #[test]
    fn test_line_chart_new() {
        let chart = LineChart::new();
        assert_eq!(chart.series_count(), 0);
        assert!(chart.show_legend);
        assert!(!chart.show_grid);
    }

    #[test]
    fn test_line_chart_add_series() {
        let chart = LineChart::new()
            .add_series(DataSeries::new("A", vec![1.0]))
            .add_series(DataSeries::new("B", vec![2.0]));
        assert_eq!(chart.series_count(), 2);
    }

    #[test]
    fn test_line_chart_with_title() {
        let chart = LineChart::new().with_title("My Chart");
        assert_eq!(chart.title, Some("My Chart".to_string()));
    }

    #[test]
    fn test_line_chart_with_labels() {
        let chart = LineChart::new().with_x_label("Time").with_y_label("Value");
        assert_eq!(chart.x_label, Some("Time".to_string()));
        assert_eq!(chart.y_label, Some("Value".to_string()));
    }

    #[test]
    fn test_line_chart_with_legend() {
        let chart = LineChart::new().with_legend(false);
        assert!(!chart.show_legend);
    }

    #[test]
    fn test_line_chart_with_grid() {
        let chart = LineChart::new().with_grid(true);
        assert!(chart.show_grid);
    }

    #[test]
    fn test_line_chart_with_y_bounds() {
        let chart = LineChart::new().with_y_bounds(0.0, 100.0);
        assert_eq!(chart.y_bounds, Some((0.0, 100.0)));
    }

    #[test]
    fn test_line_chart_calculate_y_bounds() {
        let chart = LineChart::new()
            .add_series(DataSeries::new("A", vec![10.0, 20.0, 30.0]))
            .add_series(DataSeries::new("B", vec![5.0, 15.0, 25.0]));

        let (min, max) = chart.calculate_y_bounds();
        assert!(min < 5.0); // With padding
        assert!(max > 30.0); // With padding
    }

    #[test]
    fn test_line_chart_manual_y_bounds() {
        let chart = LineChart::new()
            .with_y_bounds(0.0, 50.0)
            .add_series(DataSeries::new("A", vec![10.0, 20.0, 30.0]));

        let (min, max) = chart.calculate_y_bounds();
        assert_eq!(min, 0.0);
        assert_eq!(max, 50.0);
    }

    #[test]
    fn test_line_chart_empty_bounds() {
        let chart = LineChart::new();
        let (min, max) = chart.calculate_y_bounds();
        assert_eq!(min, 0.0);
        assert_eq!(max, 1.0);
    }

    #[test]
    fn test_line_chart_render_lines() {
        let chart = LineChart::new()
            .with_title("Test Chart")
            .add_series(DataSeries::new("Data", vec![1.0, 2.0, 3.0]));

        let lines = chart.render_lines(40, 20);
        assert!(!lines.is_empty());
    }

    #[test]
    fn test_line_chart_render_empty() {
        let chart = LineChart::new();
        let lines = chart.render_lines(40, 20);
        assert!(!lines.is_empty());
    }

    #[test]
    fn test_builder_pattern() {
        let chart = LineChart::new()
            .with_title("Chart")
            .with_x_label("X")
            .with_y_label("Y")
            .with_legend(false)
            .with_grid(true)
            .with_y_bounds(0.0, 100.0);

        assert_eq!(chart.title, Some("Chart".to_string()));
        assert_eq!(chart.x_label, Some("X".to_string()));
        assert_eq!(chart.y_label, Some("Y".to_string()));
        assert!(!chart.show_legend);
        assert!(chart.show_grid);
        assert_eq!(chart.y_bounds, Some((0.0, 100.0)));
    }

    // ============ COMPREHENSIVE EDGE CASE TESTS ============

    #[test]
    fn test_data_series_with_very_large_dataset() {
        let data: Vec<f64> = (0..10000).map(|i| i as f64).collect();
        let series = DataSeries::new("Large", data);
        assert_eq!(series.data.len(), 10000);
        assert_eq!(series.min(), Some(0.0));
        assert_eq!(series.max(), Some(9999.0));
    }

    #[test]
    fn test_data_series_with_negative_values() {
        let series = DataSeries::new("Negative", vec![-10.0, -5.0, 0.0, 5.0, 10.0]);
        assert_eq!(series.min(), Some(-10.0));
        assert_eq!(series.max(), Some(10.0));
    }

    #[test]
    fn test_data_series_with_single_value() {
        let series = DataSeries::new("Single", vec![42.0]);
        assert_eq!(series.min(), Some(42.0));
        assert_eq!(series.max(), Some(42.0));
        assert_eq!(series.data.len(), 1);
    }

    #[test]
    fn test_data_series_with_all_same_values() {
        let series = DataSeries::new("Flat", vec![5.0, 5.0, 5.0, 5.0]);
        assert_eq!(series.min(), Some(5.0));
        assert_eq!(series.max(), Some(5.0));
    }

    #[test]
    fn test_data_series_with_extreme_values() {
        let series = DataSeries::new("Extreme", vec![f64::MIN, f64::MAX]);
        assert_eq!(series.min(), Some(f64::MIN));
        assert_eq!(series.max(), Some(f64::MAX));
    }

    #[test]
    fn test_data_series_with_very_small_values() {
        let series = DataSeries::new("Small", vec![0.0001, 0.0002, 0.0003]);
        assert_eq!(series.min(), Some(0.0001));
        assert_eq!(series.max(), Some(0.0003));
    }

    #[test]
    fn test_data_series_with_unicode_name() {
        let series = DataSeries::new("温度 🌡️", vec![20.0, 25.0, 30.0]);
        assert!(series.name.contains("温度"));
        assert!(series.name.contains("🌡️"));
    }

    #[test]
    fn test_data_series_with_very_long_name() {
        let long_name = "A".repeat(1000);
        let series = DataSeries::new(long_name.clone(), vec![1.0]);
        assert_eq!(series.name, long_name);
    }

    #[test]
    fn test_data_series_with_empty_name() {
        let series = DataSeries::new("", vec![1.0]);
        assert_eq!(series.name, "");
    }

    #[test]
    fn test_data_series_clone() {
        let original = DataSeries::new("Test", vec![1.0, 2.0, 3.0])
            .with_color(Color::Red)
            .with_markers(true);
        let cloned = original.clone();

        assert_eq!(original.name, cloned.name);
        assert_eq!(original.data, cloned.data);
        assert_eq!(original.color, cloned.color);
        assert_eq!(original.show_markers, cloned.show_markers);
    }

    #[test]
    fn test_line_chart_with_many_series() {
        let mut chart = LineChart::new();
        for i in 0..50 {
            chart = chart.add_series(DataSeries::new(format!("Series {}", i), vec![i as f64]));
        }
        assert_eq!(chart.series_count(), 50);
    }

    #[test]
    fn test_line_chart_with_unicode_title() {
        let chart = LineChart::new().with_title("グラフ 📊 Graph");
        assert!(chart.title.as_ref().unwrap().contains("グラフ"));
        assert!(chart.title.as_ref().unwrap().contains("📊"));
    }

    #[test]
    fn test_line_chart_with_very_long_title() {
        let long_title = "B".repeat(1000);
        let chart = LineChart::new().with_title(long_title.clone());
        assert_eq!(chart.title, Some(long_title));
    }

    #[test]
    fn test_line_chart_with_unicode_labels() {
        let chart = LineChart::new()
            .with_x_label("時間 ⏰")
            .with_y_label("値 📈");
        assert!(chart.x_label.as_ref().unwrap().contains("時間"));
        assert!(chart.y_label.as_ref().unwrap().contains("値"));
    }

    #[test]
    fn test_line_chart_with_empty_labels() {
        let chart = LineChart::new().with_x_label("").with_y_label("");
        assert_eq!(chart.x_label, Some("".to_string()));
        assert_eq!(chart.y_label, Some("".to_string()));
    }

    #[test]
    fn test_line_chart_with_negative_bounds() {
        let chart = LineChart::new().with_y_bounds(-100.0, -50.0);
        assert_eq!(chart.y_bounds, Some((-100.0, -50.0)));
    }

    #[test]
    fn test_line_chart_with_inverted_bounds() {
        // Max < Min (inverted)
        let chart = LineChart::new().with_y_bounds(100.0, 0.0);
        assert_eq!(chart.y_bounds, Some((100.0, 0.0)));
    }

    #[test]
    fn test_line_chart_with_same_bounds() {
        let chart = LineChart::new().with_y_bounds(50.0, 50.0);
        assert_eq!(chart.y_bounds, Some((50.0, 50.0)));
    }

    #[test]
    fn test_line_chart_calculate_bounds_with_mixed_series() {
        let chart = LineChart::new()
            .add_series(DataSeries::new("Positive", vec![10.0, 20.0, 30.0]))
            .add_series(DataSeries::new("Negative", vec![-10.0, -20.0, -30.0]))
            .add_series(DataSeries::new("Mixed", vec![-5.0, 0.0, 5.0]));

        let (min, max) = chart.calculate_y_bounds();
        assert!(min < -30.0); // With padding
        assert!(max > 30.0); // With padding
    }

    #[test]
    fn test_line_chart_render_lines_with_small_dimensions() {
        let chart = LineChart::new().add_series(DataSeries::new("Data", vec![1.0, 2.0, 3.0]));

        // Very small rendering area
        let lines = chart.render_lines(5, 3);
        assert!(!lines.is_empty());
    }

    #[test]
    fn test_line_chart_render_lines_with_large_dimensions() {
        let chart = LineChart::new().add_series(DataSeries::new("Data", vec![1.0, 2.0, 3.0]));

        // Very large rendering area
        let lines = chart.render_lines(1000, 500);
        assert!(!lines.is_empty());
    }

    #[test]
    fn test_line_chart_render_lines_with_zero_dimensions() {
        let chart = LineChart::new().add_series(DataSeries::new("Data", vec![1.0, 2.0, 3.0]));

        // Should not panic with zero dimensions
        let _lines = chart.render_lines(0, 0);
        // Just verify it doesn't crash - output behavior may vary
    }

    #[test]
    fn test_line_chart_with_no_series() {
        let chart = LineChart::new()
            .with_title("Empty Chart")
            .with_legend(true)
            .with_grid(true);

        assert_eq!(chart.series_count(), 0);
        let lines = chart.render_lines(40, 20);
        assert!(!lines.is_empty());
    }

    #[test]
    fn test_line_chart_series_count_incremental() {
        let chart = LineChart::new();
        assert_eq!(chart.series_count(), 0);

        let chart = chart.add_series(DataSeries::new("A", vec![1.0]));
        assert_eq!(chart.series_count(), 1);

        let chart = chart.add_series(DataSeries::new("B", vec![2.0]));
        assert_eq!(chart.series_count(), 2);
    }

    #[test]
    fn test_line_chart_toggle_legend() {
        let chart = LineChart::new();
        assert!(chart.show_legend); // Default true

        let chart = chart.with_legend(false);
        assert!(!chart.show_legend);

        let chart = chart.with_legend(true);
        assert!(chart.show_legend);
    }

    #[test]
    fn test_line_chart_toggle_grid() {
        let chart = LineChart::new();
        assert!(!chart.show_grid); // Default false

        let chart = chart.with_grid(true);
        assert!(chart.show_grid);

        let chart = chart.with_grid(false);
        assert!(!chart.show_grid);
    }

    #[test]
    fn test_line_chart_with_all_features_enabled() {
        let chart = LineChart::new()
            .with_title("Full Featured Chart")
            .with_x_label("X Axis")
            .with_y_label("Y Axis")
            .with_legend(true)
            .with_grid(true)
            .with_y_bounds(0.0, 100.0)
            .add_series(
                DataSeries::new("Series 1", vec![10.0, 20.0, 30.0])
                    .with_color(Color::Red)
                    .with_markers(true),
            )
            .add_series(
                DataSeries::new("Series 2", vec![15.0, 25.0, 35.0])
                    .with_color(Color::Blue)
                    .with_markers(false),
            );

        assert_eq!(chart.series_count(), 2);
        assert!(chart.show_legend);
        assert!(chart.show_grid);
        assert!(chart.title.is_some());
        assert!(chart.x_label.is_some());
        assert!(chart.y_label.is_some());
        assert!(chart.y_bounds.is_some());
    }

    #[test]
    fn test_data_series_with_decimal_precision() {
        let series = DataSeries::new(
            "Precision",
            vec![1.123456789, 2.987654321, 3.141592653],
        );
        assert_eq!(series.min(), Some(1.123456789));
        assert_eq!(series.max(), Some(3.141592653));
    }

    #[test]
    fn test_line_chart_multiple_adds() {
        let chart = LineChart::new()
            .add_series(DataSeries::new("A", vec![1.0]))
            .add_series(DataSeries::new("B", vec![2.0]))
            .add_series(DataSeries::new("C", vec![3.0]))
            .add_series(DataSeries::new("D", vec![4.0]))
            .add_series(DataSeries::new("E", vec![5.0]));

        assert_eq!(chart.series_count(), 5);
    }
}
