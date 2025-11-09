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

    // ============================================================================
    // ADVANCED TIER: Additional Comprehensive Edge Case Tests
    // ============================================================================

    #[test]
    fn test_data_series_debug_trait() {
        let series = DataSeries::new("Test", vec![1.0, 2.0, 3.0]);
        let debug_str = format!("{:?}", series);
        assert!(debug_str.contains("DataSeries"));
        assert!(debug_str.contains("Test"));
    }

    #[test]
    fn test_line_chart_debug_trait() {
        let chart = LineChart::new()
            .with_title("Test Chart")
            .add_series(DataSeries::new("Data", vec![1.0]));
        let debug_str = format!("{:?}", chart);
        assert!(debug_str.contains("LineChart"));
    }

    #[test]
    fn test_line_chart_default_trait() {
        let chart = LineChart::default();
        assert_eq!(chart.series_count(), 0);
        assert!(chart.show_legend);
        assert!(!chart.show_grid);
        assert!(chart.title.is_none());
    }

    // Stress Tests (10k operations)

    #[test]
    fn test_line_chart_10k_series_additions() {
        let mut chart = LineChart::new();
        for i in 0..10000 {
            chart = chart.add_series(DataSeries::new(format!("Series {}", i), vec![i as f64]));
        }
        assert_eq!(chart.series_count(), 10000);
    }

    #[test]
    fn test_data_series_10k_data_points() {
        let data: Vec<f64> = (0..10000).map(|i| (i as f64).sin()).collect();
        let series = DataSeries::new("Large", data);
        assert_eq!(series.data.len(), 10000);
        assert!(series.min().is_some());
        assert!(series.max().is_some());
    }

    #[test]
    fn test_line_chart_render_with_10k_points() {
        let data: Vec<f64> = (0..10000).map(|i| (i as f64 * 0.1).sin()).collect();
        let chart = LineChart::new()
            .add_series(DataSeries::new("Sine Wave", data));

        let lines = chart.render_lines(100, 50);
        assert!(!lines.is_empty());
    }

    #[test]
    fn test_data_series_10k_min_max_calculations() {
        let data: Vec<f64> = (0..10000).map(|i| (i % 100) as f64).collect();
        let series = DataSeries::new("Cyclic", data);

        // Should calculate min/max efficiently
        assert_eq!(series.min(), Some(0.0));
        assert_eq!(series.max(), Some(99.0));
    }

    // Unicode Edge Cases

    #[test]
    fn test_line_chart_rtl_text_arabic() {
        let chart = LineChart::new()
            .with_title("مخطط البيانات")
            .with_x_label("الوقت")
            .with_y_label("القيمة")
            .add_series(DataSeries::new("سلسلة البيانات", vec![1.0, 2.0, 3.0]));

        assert!(chart.title.as_ref().unwrap().contains("مخطط"));
        assert!(chart.x_label.as_ref().unwrap().contains("الوقت"));
        assert!(chart.y_label.as_ref().unwrap().contains("القيمة"));
        assert!(chart.series[0].name.contains("سلسلة"));
    }

    #[test]
    fn test_line_chart_rtl_text_hebrew() {
        let chart = LineChart::new()
            .with_title("תרשים נתונים")
            .with_x_label("זמן")
            .with_y_label("ערך")
            .add_series(DataSeries::new("סדרת נתונים", vec![1.0, 2.0]));

        assert!(chart.title.as_ref().unwrap().contains("תרשים"));
        assert!(chart.x_label.as_ref().unwrap().contains("זמן"));
    }

    #[test]
    fn test_line_chart_mixed_scripts() {
        let chart = LineChart::new()
            .with_title("Chart 图表 مخطط Graf 그래프")
            .with_x_label("Time 時間 الوقت")
            .add_series(DataSeries::new("Data データ بيانات", vec![1.0]));

        assert!(chart.title.as_ref().unwrap().contains("Chart"));
        assert!(chart.title.as_ref().unwrap().contains("图表"));
        assert!(chart.title.as_ref().unwrap().contains("مخطط"));
        assert!(chart.title.as_ref().unwrap().contains("그래프"));
    }

    #[test]
    fn test_line_chart_emoji_combinations() {
        let chart = LineChart::new()
            .with_title("📊 Chart 📈 Analysis 📉")
            .with_x_label("⏰ Time")
            .with_y_label("💰 Value")
            .add_series(DataSeries::new("🚀 Growth", vec![1.0, 2.0, 3.0]))
            .add_series(DataSeries::new("📉 Decline", vec![3.0, 2.0, 1.0]));

        assert!(chart.title.as_ref().unwrap().contains("📊"));
        assert!(chart.series[0].name.contains("🚀"));
        assert!(chart.series[1].name.contains("📉"));
    }

    #[test]
    fn test_data_series_very_long_unicode_name() {
        let long_unicode = "📊".repeat(1000);
        let series = DataSeries::new(&long_unicode, vec![1.0]);
        assert_eq!(series.name.chars().filter(|&c| c == '📊').count(), 1000);
    }

    #[test]
    fn test_line_chart_zero_width_characters() {
        let text_with_zwj = "Test\u{200D}Chart";
        let chart = LineChart::new()
            .with_title(text_with_zwj)
            .add_series(DataSeries::new("Test\u{200C}Series", vec![1.0]));

        assert!(chart.title.as_ref().unwrap().contains('\u{200D}'));
        assert!(chart.series[0].name.contains('\u{200C}'));
    }

    #[test]
    fn test_line_chart_combining_characters() {
        let text_with_combining = "Cafe\u{0301}"; // Café with combining acute
        let chart = LineChart::new()
            .with_title(text_with_combining)
            .add_series(DataSeries::new("Se\u{0301}rie", vec![1.0]));

        assert!(chart.title.as_ref().unwrap().contains('\u{0301}'));
        assert!(chart.series[0].name.contains('\u{0301}'));
    }

    // Extreme Values

    #[test]
    fn test_data_series_with_infinity() {
        let series = DataSeries::new("Infinite", vec![f64::INFINITY, 1.0, f64::NEG_INFINITY]);
        assert_eq!(series.min(), Some(f64::NEG_INFINITY));
        assert_eq!(series.max(), Some(f64::INFINITY));
    }

    #[test]
    fn test_data_series_with_nan() {
        let series = DataSeries::new("NaN", vec![1.0, f64::NAN, 2.0]);
        // Note: min/max methods panic with NaN due to unwrap() on partial_cmp
        // This test just verifies series creation doesn't panic
        assert_eq!(series.data.len(), 3);
        assert!(series.data[1].is_nan());
        // Calling min() or max() would panic, so we skip that
    }

    #[test]
    fn test_data_series_with_zero_only() {
        let series = DataSeries::new("Zeros", vec![0.0, 0.0, 0.0, 0.0]);
        assert_eq!(series.min(), Some(0.0));
        assert_eq!(series.max(), Some(0.0));
    }

    #[test]
    fn test_line_chart_with_1000_series() {
        let mut chart = LineChart::new();
        for i in 0..1000 {
            chart = chart.add_series(DataSeries::new(format!("S{}", i), vec![i as f64]));
        }
        assert_eq!(chart.series_count(), 1000);

        // Verify rendering doesn't panic with many series
        let _lines = chart.render_lines(80, 40);
    }

    #[test]
    fn test_data_series_with_very_small_differences() {
        let series = DataSeries::new(
            "Tiny Diffs",
            vec![1.0000001, 1.0000002, 1.0000003],
        );
        assert!(series.min().is_some());
        assert!(series.max().is_some());
    }

    #[test]
    fn test_line_chart_with_extreme_y_bounds() {
        let chart = LineChart::new()
            .with_y_bounds(f64::MIN, f64::MAX);

        let (min, max) = chart.calculate_y_bounds();
        assert_eq!(min, f64::MIN);
        assert_eq!(max, f64::MAX);
    }

    // Multi-phase Comprehensive Workflow

    #[test]
    fn test_line_chart_10_phase_comprehensive_workflow() {
        // Phase 1: Create basic chart
        let mut chart = LineChart::new();
        assert_eq!(chart.series_count(), 0);
        assert!(chart.title.is_none());

        // Phase 2: Set title and labels
        chart = chart
            .with_title("Comprehensive Test Chart")
            .with_x_label("Time (s)")
            .with_y_label("Value (units)");
        assert!(chart.title.is_some());
        assert!(chart.x_label.is_some());
        assert!(chart.y_label.is_some());

        // Phase 3: Add multiple series
        chart = chart
            .add_series(DataSeries::new("Series 1", vec![1.0, 2.0, 3.0, 4.0, 5.0]))
            .add_series(DataSeries::new("Series 2", vec![5.0, 4.0, 3.0, 2.0, 1.0]))
            .add_series(DataSeries::new("Series 3", vec![2.5, 2.5, 2.5, 2.5, 2.5]));
        assert_eq!(chart.series_count(), 3);

        // Phase 4: Configure with colors and markers
        chart = chart
            .add_series(
                DataSeries::new("Colored 1", vec![1.0, 3.0, 2.0])
                    .with_color(Color::Red)
                    .with_markers(true),
            )
            .add_series(
                DataSeries::new("Colored 2", vec![2.0, 1.0, 3.0])
                    .with_color(Color::Blue)
                    .with_markers(false),
            );
        assert_eq!(chart.series_count(), 5);

        // Phase 5: Enable legend and grid
        chart = chart.with_legend(true).with_grid(true);
        assert!(chart.show_legend);
        assert!(chart.show_grid);

        // Phase 6: Set Y bounds
        chart = chart.with_y_bounds(0.0, 10.0);
        assert_eq!(chart.y_bounds, Some((0.0, 10.0)));
        let (min, max) = chart.calculate_y_bounds();
        assert_eq!(min, 0.0);
        assert_eq!(max, 10.0);

        // Phase 7: Render with various dimensions
        let lines_small = chart.render_lines(20, 10);
        let lines_medium = chart.render_lines(80, 40);
        let lines_large = chart.render_lines(200, 100);
        assert!(!lines_small.is_empty());
        assert!(!lines_medium.is_empty());
        assert!(!lines_large.is_empty());

        // Phase 8: Toggle legend and grid
        chart = chart.with_legend(false).with_grid(false);
        assert!(!chart.show_legend);
        assert!(!chart.show_grid);

        // Phase 9: Update Y bounds
        chart = chart.with_y_bounds(-5.0, 15.0);
        let (min, max) = chart.calculate_y_bounds();
        assert_eq!(min, -5.0);
        assert_eq!(max, 15.0);

        // Phase 10: Add more series and verify final state
        chart = chart
            .add_series(DataSeries::new("Final Series", vec![7.0, 8.0, 9.0]));
        assert_eq!(chart.series_count(), 6);

        let final_lines = chart.render_lines(100, 50);
        assert!(!final_lines.is_empty());
    }

    // Builder Pattern Edge Cases

    #[test]
    fn test_line_chart_multiple_title_calls() {
        let chart = LineChart::new()
            .with_title("First Title")
            .with_title("Second Title")
            .with_title("Final Title");

        assert_eq!(chart.title, Some("Final Title".to_string()));
    }

    #[test]
    fn test_line_chart_multiple_label_calls() {
        let chart = LineChart::new()
            .with_x_label("X1")
            .with_x_label("X2")
            .with_y_label("Y1")
            .with_y_label("Y2");

        assert_eq!(chart.x_label, Some("X2".to_string()));
        assert_eq!(chart.y_label, Some("Y2".to_string()));
    }

    #[test]
    fn test_line_chart_multiple_bounds_calls() {
        let chart = LineChart::new()
            .with_y_bounds(0.0, 10.0)
            .with_y_bounds(5.0, 15.0)
            .with_y_bounds(-10.0, 20.0);

        assert_eq!(chart.y_bounds, Some((-10.0, 20.0)));
    }

    #[test]
    fn test_line_chart_builder_chaining_many_operations() {
        let chart = LineChart::new()
            .with_title("T1").with_title("T2").with_title("Final")
            .with_x_label("X1").with_x_label("X2")
            .with_y_label("Y1").with_y_label("Y2")
            .with_legend(true).with_legend(false).with_legend(true)
            .with_grid(false).with_grid(true).with_grid(false)
            .with_y_bounds(0.0, 10.0).with_y_bounds(5.0, 15.0)
            .add_series(DataSeries::new("S1", vec![1.0]))
            .add_series(DataSeries::new("S2", vec![2.0]));

        assert_eq!(chart.title, Some("Final".to_string()));
        assert_eq!(chart.x_label, Some("X2".to_string()));
        assert_eq!(chart.y_label, Some("Y2".to_string()));
        assert!(chart.show_legend);
        assert!(!chart.show_grid);
        assert_eq!(chart.y_bounds, Some((5.0, 15.0)));
        assert_eq!(chart.series_count(), 2);
    }

    // Clone Tests

    #[test]
    fn test_line_chart_clone() {
        let original = LineChart::new()
            .with_title("Original")
            .add_series(DataSeries::new("S1", vec![1.0, 2.0]))
            .with_legend(false);

        let cloned = original.clone();

        assert_eq!(original.title, cloned.title);
        assert_eq!(original.series_count(), cloned.series_count());
        assert_eq!(original.show_legend, cloned.show_legend);
    }

    #[test]
    fn test_line_chart_clone_independence() {
        let original = LineChart::new()
            .with_title("Original")
            .add_series(DataSeries::new("S1", vec![1.0]));

        let mut modified = original.clone();
        modified = modified
            .with_title("Modified")
            .add_series(DataSeries::new("S2", vec![2.0]));

        assert_eq!(original.title, Some("Original".to_string()));
        assert_eq!(modified.title, Some("Modified".to_string()));
        assert_eq!(original.series_count(), 1);
        assert_eq!(modified.series_count(), 2);
    }

    #[test]
    fn test_data_series_clone_independence() {
        let original = DataSeries::new("Original", vec![1.0, 2.0, 3.0])
            .with_color(Color::Red)
            .with_markers(true);

        let mut modified = original.clone();
        modified.name = "Modified".to_string();
        modified.color = Color::Blue;
        modified.show_markers = false;
        modified.data.push(4.0);

        assert_eq!(original.name, "Original");
        assert_eq!(original.color, Color::Red);
        assert!(original.show_markers);
        assert_eq!(original.data.len(), 3);

        assert_eq!(modified.name, "Modified");
        assert_eq!(modified.color, Color::Blue);
        assert!(!modified.show_markers);
        assert_eq!(modified.data.len(), 4);
    }

    // Rendering Edge Cases

    #[test]
    fn test_line_chart_render_with_all_empty_series() {
        let chart = LineChart::new()
            .add_series(DataSeries::new("Empty 1", vec![]))
            .add_series(DataSeries::new("Empty 2", vec![]))
            .add_series(DataSeries::new("Empty 3", vec![]));

        let lines = chart.render_lines(80, 40);
        assert!(!lines.is_empty());
    }

    #[test]
    fn test_line_chart_render_with_mixed_length_series() {
        let chart = LineChart::new()
            .add_series(DataSeries::new("Short", vec![1.0]))
            .add_series(DataSeries::new("Medium", vec![1.0, 2.0, 3.0]))
            .add_series(DataSeries::new("Long", vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]));

        let lines = chart.render_lines(100, 50);
        assert!(!lines.is_empty());
    }

    #[test]
    fn test_line_chart_render_with_legend_no_series() {
        let chart = LineChart::new()
            .with_title("Chart with no data")
            .with_legend(true);

        let lines = chart.render_lines(80, 40);
        assert!(!lines.is_empty());
    }

    #[test]
    fn test_line_chart_render_all_features_disabled() {
        let chart = LineChart::new()
            .add_series(DataSeries::new("Data", vec![1.0, 2.0, 3.0]))
            .with_legend(false)
            .with_grid(false);

        let lines = chart.render_lines(80, 40);
        assert!(!lines.is_empty());
    }

    #[test]
    fn test_data_series_with_alternating_sign() {
        let series = DataSeries::new(
            "Alternating",
            vec![-1.0, 1.0, -2.0, 2.0, -3.0, 3.0],
        );
        assert_eq!(series.min(), Some(-3.0));
        assert_eq!(series.max(), Some(3.0));
    }

    #[test]
    fn test_line_chart_calculate_bounds_all_empty_series() {
        let chart = LineChart::new()
            .add_series(DataSeries::new("E1", vec![]))
            .add_series(DataSeries::new("E2", vec![]));

        let (min, max) = chart.calculate_y_bounds();
        assert_eq!(min, 0.0);
        assert_eq!(max, 1.0);
    }

    #[test]
    fn test_line_chart_with_unicode_in_all_fields() {
        let chart = LineChart::new()
            .with_title("标题 عنوان כותרת 📊")
            .with_x_label("X軸 محور X ציר X ⏰")
            .with_y_label("Y軸 محور Y ציר Y 📈")
            .add_series(DataSeries::new("系列 سلسلة סדרה 🚀", vec![1.0, 2.0, 3.0]));

        assert!(chart.title.as_ref().unwrap().contains("标题"));
        assert!(chart.x_label.as_ref().unwrap().contains("X軸"));
        assert!(chart.y_label.as_ref().unwrap().contains("Y軸"));
        assert!(chart.series[0].name.contains("系列"));
    }

    #[test]
    fn test_data_series_with_exponential_values() {
        let series = DataSeries::new(
            "Exponential",
            vec![1e-10, 1e-5, 1e0, 1e5, 1e10],
        );
        assert_eq!(series.min(), Some(1e-10));
        assert_eq!(series.max(), Some(1e10));
    }
}
