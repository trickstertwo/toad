//! Live graph widget for real-time data visualization
//!
//! Wraps chart widgets (LineChart, BarChart, ScatterPlot) with automatic
//! data updates, windowing, and time-series management.
//!
//! # Examples
//!
//! ```
//! use toad::widgets::{LiveGraph, GraphType};
//! use std::time::Duration;
//!
//! let mut graph = LiveGraph::new(GraphType::Line)
//!     .with_title("CPU Usage")
//!     .with_max_points(100)
//!     .with_update_interval(Duration::from_millis(500));
//!
//! // Add data points
//! graph.add_point(45.2);
//! graph.add_point(52.8);
//! graph.add_point(48.1);
//! ```

use crate::ui::widgets::{
    BarChartWidget as BarChart, BarData, DataSeries, LineChartWidget as LineChart,
    ScatterPlotWidget as ScatterPlot, ScatterSeries,
};
use ratatui::{buffer::Buffer, layout::Rect, style::Color, widgets::Widget};
use std::time::{Duration, Instant};

/// Type of graph to display
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GraphType {
    /// Line chart for continuous data
    #[default]
    Line,
    /// Bar chart for discrete comparisons
    Bar,
    /// Scatter plot for distribution
    Scatter,
}

/// Live graph widget for real-time data
///
/// Automatically manages data updates, windowing, and rendering
/// for streaming data visualization.
///
/// # Examples
///
/// ```
/// use toad::widgets::{LiveGraph, GraphType};
/// use std::time::Duration;
///
/// let mut graph = LiveGraph::new(GraphType::Line)
///     .with_title("Network Traffic")
///     .with_max_points(50);
///
/// // Simulate data updates
/// for i in 0..10 {
///     graph.add_point(i as f64 * 10.0);
/// }
///
/// assert_eq!(graph.data_points(), 10);
/// ```
#[derive(Debug, Clone)]
pub struct LiveGraph {
    /// Type of graph
    graph_type: GraphType,
    /// Data points
    data: Vec<f64>,
    /// Maximum number of points to keep
    max_points: usize,
    /// Graph title
    title: Option<String>,
    /// Y-axis label
    y_label: Option<String>,
    /// Graph color
    color: Color,
    /// Update interval
    update_interval: Duration,
    /// Last update time
    last_update: Option<Instant>,
    /// Auto-scale Y axis
    auto_scale: bool,
    /// Manual Y bounds
    y_bounds: Option<(f64, f64)>,
}

impl Default for LiveGraph {
    fn default() -> Self {
        Self::new(GraphType::default())
    }
}

impl LiveGraph {
    /// Create a new live graph
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{LiveGraph, GraphType};
    ///
    /// let graph = LiveGraph::new(GraphType::Line);
    /// assert_eq!(graph.data_points(), 0);
    /// ```
    pub fn new(graph_type: GraphType) -> Self {
        Self {
            graph_type,
            data: Vec::new(),
            max_points: 100,
            title: None,
            y_label: None,
            color: Color::Cyan,
            update_interval: Duration::from_millis(100),
            last_update: None,
            auto_scale: true,
            y_bounds: None,
        }
    }

    /// Set graph title
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{LiveGraph, GraphType};
    ///
    /// let graph = LiveGraph::new(GraphType::Line)
    ///     .with_title("Temperature");
    /// ```
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set Y-axis label
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{LiveGraph, GraphType};
    ///
    /// let graph = LiveGraph::new(GraphType::Bar)
    ///     .with_y_label("Requests/s");
    /// ```
    pub fn with_y_label(mut self, label: impl Into<String>) -> Self {
        self.y_label = Some(label.into());
        self
    }

    /// Set maximum number of points to display
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{LiveGraph, GraphType};
    ///
    /// let mut graph = LiveGraph::new(GraphType::Line)
    ///     .with_max_points(50);
    ///
    /// for i in 0..100 {
    ///     graph.add_point(i as f64);
    /// }
    ///
    /// assert_eq!(graph.data_points(), 50);
    /// ```
    pub fn with_max_points(mut self, max: usize) -> Self {
        self.max_points = max;
        self
    }

    /// Set graph color
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{LiveGraph, GraphType};
    /// use ratatui::style::Color;
    ///
    /// let graph = LiveGraph::new(GraphType::Line)
    ///     .with_color(Color::Green);
    /// ```
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Set update interval
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{LiveGraph, GraphType};
    /// use std::time::Duration;
    ///
    /// let graph = LiveGraph::new(GraphType::Line)
    ///     .with_update_interval(Duration::from_millis(200));
    /// ```
    pub fn with_update_interval(mut self, interval: Duration) -> Self {
        self.update_interval = interval;
        self
    }

    /// Set Y-axis bounds (disables auto-scaling)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{LiveGraph, GraphType};
    ///
    /// let graph = LiveGraph::new(GraphType::Line)
    ///     .with_y_bounds(0.0, 100.0);
    /// ```
    pub fn with_y_bounds(mut self, min: f64, max: f64) -> Self {
        self.y_bounds = Some((min, max));
        self.auto_scale = false;
        self
    }

    /// Enable or disable auto-scaling
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{LiveGraph, GraphType};
    ///
    /// let graph = LiveGraph::new(GraphType::Line)
    ///     .with_auto_scale(false);
    /// ```
    pub fn with_auto_scale(mut self, enable: bool) -> Self {
        self.auto_scale = enable;
        if enable {
            self.y_bounds = None;
        }
        self
    }

    /// Add a data point
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{LiveGraph, GraphType};
    ///
    /// let mut graph = LiveGraph::new(GraphType::Line);
    /// graph.add_point(42.5);
    /// assert_eq!(graph.data_points(), 1);
    /// ```
    pub fn add_point(&mut self, value: f64) {
        self.data.push(value);
        self.last_update = Some(Instant::now());

        // Apply windowing
        if self.data.len() > self.max_points {
            self.data.remove(0);
        }
    }

    /// Add multiple data points
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{LiveGraph, GraphType};
    ///
    /// let mut graph = LiveGraph::new(GraphType::Line);
    /// graph.add_points(&[10.0, 20.0, 30.0]);
    /// assert_eq!(graph.data_points(), 3);
    /// ```
    pub fn add_points(&mut self, values: &[f64]) {
        for &value in values {
            self.add_point(value);
        }
    }

    /// Clear all data points
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{LiveGraph, GraphType};
    ///
    /// let mut graph = LiveGraph::new(GraphType::Line);
    /// graph.add_point(10.0);
    /// graph.add_point(20.0);
    /// graph.clear();
    /// assert_eq!(graph.data_points(), 0);
    /// ```
    pub fn clear(&mut self) {
        self.data.clear();
        self.last_update = None;
    }

    /// Get number of data points
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{LiveGraph, GraphType};
    ///
    /// let mut graph = LiveGraph::new(GraphType::Line);
    /// assert_eq!(graph.data_points(), 0);
    /// graph.add_point(5.0);
    /// assert_eq!(graph.data_points(), 1);
    /// ```
    pub fn data_points(&self) -> usize {
        self.data.len()
    }

    /// Check if data should be updated based on interval
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{LiveGraph, GraphType};
    /// use std::time::Duration;
    ///
    /// let mut graph = LiveGraph::new(GraphType::Line)
    ///     .with_update_interval(Duration::from_millis(100));
    ///
    /// assert!(graph.should_update());
    /// graph.add_point(10.0);
    /// // Immediately after update, should return false
    /// assert!(!graph.should_update());
    /// ```
    pub fn should_update(&self) -> bool {
        match self.last_update {
            None => true,
            Some(last) => last.elapsed() >= self.update_interval,
        }
    }

    /// Get the latest data point
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{LiveGraph, GraphType};
    ///
    /// let mut graph = LiveGraph::new(GraphType::Line);
    /// assert_eq!(graph.latest(), None);
    /// graph.add_point(42.0);
    /// assert_eq!(graph.latest(), Some(42.0));
    /// ```
    pub fn latest(&self) -> Option<f64> {
        self.data.last().copied()
    }

    /// Get average of all data points
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{LiveGraph, GraphType};
    ///
    /// let mut graph = LiveGraph::new(GraphType::Line);
    /// graph.add_points(&[10.0, 20.0, 30.0]);
    /// assert_eq!(graph.average(), Some(20.0));
    /// ```
    pub fn average(&self) -> Option<f64> {
        if self.data.is_empty() {
            None
        } else {
            Some(self.data.iter().sum::<f64>() / self.data.len() as f64)
        }
    }

    /// Get minimum data point
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{LiveGraph, GraphType};
    ///
    /// let mut graph = LiveGraph::new(GraphType::Line);
    /// graph.add_points(&[30.0, 10.0, 20.0]);
    /// assert_eq!(graph.min(), Some(10.0));
    /// ```
    pub fn min(&self) -> Option<f64> {
        self.data
            .iter()
            .copied()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
    }

    /// Get maximum data point
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{LiveGraph, GraphType};
    ///
    /// let mut graph = LiveGraph::new(GraphType::Line);
    /// graph.add_points(&[30.0, 10.0, 20.0]);
    /// assert_eq!(graph.max(), Some(30.0));
    /// ```
    pub fn max(&self) -> Option<f64> {
        self.data
            .iter()
            .copied()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
    }

    /// Set graph type
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{LiveGraph, GraphType};
    ///
    /// let mut graph = LiveGraph::new(GraphType::Line);
    /// graph.set_type(GraphType::Bar);
    /// ```
    pub fn set_type(&mut self, graph_type: GraphType) {
        self.graph_type = graph_type;
    }

    /// Render the appropriate chart widget
    fn render_chart(&self, area: Rect, buf: &mut Buffer) {
        if self.data.is_empty() {
            return;
        }

        match self.graph_type {
            GraphType::Line => {
                let series = DataSeries::new("data", self.data.clone()).with_color(self.color);

                let mut chart = LineChart::new().add_series(series);

                if let Some(ref title) = self.title {
                    chart = chart.with_title(title);
                }

                if let Some(ref label) = self.y_label {
                    chart = chart.with_y_label(label);
                }

                if let Some((min, max)) = self.y_bounds {
                    chart = chart.with_y_bounds(min, max);
                }

                (&chart).render(area, buf);
            }
            GraphType::Bar => {
                let bars: Vec<BarData> = self
                    .data
                    .iter()
                    .enumerate()
                    .map(|(i, &value)| {
                        BarData::new(format!("{}", i + 1), value).with_color(self.color)
                    })
                    .collect();

                let mut chart = BarChart::new(bars);

                if let Some(ref title) = self.title {
                    chart = chart.with_title(title);
                }

                if let Some(ref label) = self.y_label {
                    chart = chart.with_value_label(label);
                }

                if let Some((_, max)) = self.y_bounds {
                    chart = chart.with_max_value(max);
                }

                (&chart).render(area, buf);
            }
            GraphType::Scatter => {
                let points: Vec<(f64, f64)> = self
                    .data
                    .iter()
                    .enumerate()
                    .map(|(i, &value)| (i as f64, value))
                    .collect();

                let series = ScatterSeries::new("data", points).with_color(self.color);

                let mut chart = ScatterPlot::new().add_series(series);

                if let Some(ref title) = self.title {
                    chart = chart.with_title(title);
                }

                if let Some(ref label) = self.y_label {
                    chart = chart.with_y_label(label);
                }

                if let Some((min, max)) = self.y_bounds {
                    chart = chart.with_y_bounds(min, max);
                }

                (&chart).render(area, buf);
            }
        }
    }
}

impl Widget for &LiveGraph {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.render_chart(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_type_default() {
        assert_eq!(GraphType::default(), GraphType::Line);
    }

    #[test]
    fn test_live_graph_new() {
        let graph = LiveGraph::new(GraphType::Line);
        assert_eq!(graph.data_points(), 0);
        assert_eq!(graph.max_points, 100);
        assert!(graph.auto_scale);
    }

    #[test]
    fn test_live_graph_default() {
        let graph = LiveGraph::default();
        assert_eq!(graph.graph_type, GraphType::Line);
    }

    #[test]
    fn test_with_title() {
        let graph = LiveGraph::new(GraphType::Line).with_title("Test");
        assert_eq!(graph.title, Some("Test".to_string()));
    }

    #[test]
    fn test_with_y_label() {
        let graph = LiveGraph::new(GraphType::Bar).with_y_label("Value");
        assert_eq!(graph.y_label, Some("Value".to_string()));
    }

    #[test]
    fn test_with_max_points() {
        let graph = LiveGraph::new(GraphType::Line).with_max_points(50);
        assert_eq!(graph.max_points, 50);
    }

    #[test]
    fn test_with_color() {
        let graph = LiveGraph::new(GraphType::Line).with_color(Color::Red);
        assert_eq!(graph.color, Color::Red);
    }

    #[test]
    fn test_with_update_interval() {
        let interval = Duration::from_millis(200);
        let graph = LiveGraph::new(GraphType::Line).with_update_interval(interval);
        assert_eq!(graph.update_interval, interval);
    }

    #[test]
    fn test_with_y_bounds() {
        let graph = LiveGraph::new(GraphType::Line).with_y_bounds(0.0, 100.0);
        assert_eq!(graph.y_bounds, Some((0.0, 100.0)));
        assert!(!graph.auto_scale);
    }

    #[test]
    fn test_with_auto_scale() {
        let graph = LiveGraph::new(GraphType::Line)
            .with_y_bounds(0.0, 100.0)
            .with_auto_scale(true);
        assert!(graph.auto_scale);
        assert_eq!(graph.y_bounds, None);
    }

    #[test]
    fn test_add_point() {
        let mut graph = LiveGraph::new(GraphType::Line);
        graph.add_point(42.5);
        assert_eq!(graph.data_points(), 1);
        assert_eq!(graph.latest(), Some(42.5));
    }

    #[test]
    fn test_add_points() {
        let mut graph = LiveGraph::new(GraphType::Line);
        graph.add_points(&[10.0, 20.0, 30.0]);
        assert_eq!(graph.data_points(), 3);
    }

    #[test]
    fn test_windowing() {
        let mut graph = LiveGraph::new(GraphType::Line).with_max_points(3);
        graph.add_points(&[1.0, 2.0, 3.0, 4.0, 5.0]);
        assert_eq!(graph.data_points(), 3);
        assert_eq!(graph.data, vec![3.0, 4.0, 5.0]);
    }

    #[test]
    fn test_clear() {
        let mut graph = LiveGraph::new(GraphType::Line);
        graph.add_points(&[10.0, 20.0, 30.0]);
        assert_eq!(graph.data_points(), 3);
        graph.clear();
        assert_eq!(graph.data_points(), 0);
        assert_eq!(graph.latest(), None);
    }

    #[test]
    fn test_should_update() {
        let mut graph =
            LiveGraph::new(GraphType::Line).with_update_interval(Duration::from_millis(100));
        assert!(graph.should_update());
        graph.add_point(10.0);
        assert!(!graph.should_update());
    }

    #[test]
    fn test_latest() {
        let mut graph = LiveGraph::new(GraphType::Line);
        assert_eq!(graph.latest(), None);
        graph.add_point(42.0);
        assert_eq!(graph.latest(), Some(42.0));
        graph.add_point(100.0);
        assert_eq!(graph.latest(), Some(100.0));
    }

    #[test]
    fn test_average() {
        let mut graph = LiveGraph::new(GraphType::Line);
        assert_eq!(graph.average(), None);
        graph.add_points(&[10.0, 20.0, 30.0]);
        assert_eq!(graph.average(), Some(20.0));
    }

    #[test]
    fn test_min() {
        let mut graph = LiveGraph::new(GraphType::Line);
        assert_eq!(graph.min(), None);
        graph.add_points(&[30.0, 10.0, 20.0]);
        assert_eq!(graph.min(), Some(10.0));
    }

    #[test]
    fn test_max() {
        let mut graph = LiveGraph::new(GraphType::Line);
        assert_eq!(graph.max(), None);
        graph.add_points(&[30.0, 10.0, 20.0]);
        assert_eq!(graph.max(), Some(30.0));
    }

    #[test]
    fn test_set_type() {
        let mut graph = LiveGraph::new(GraphType::Line);
        assert_eq!(graph.graph_type, GraphType::Line);
        graph.set_type(GraphType::Bar);
        assert_eq!(graph.graph_type, GraphType::Bar);
    }

    #[test]
    fn test_builder_pattern() {
        let graph = LiveGraph::new(GraphType::Line)
            .with_title("CPU")
            .with_y_label("%")
            .with_max_points(50)
            .with_color(Color::Green)
            .with_update_interval(Duration::from_millis(500))
            .with_y_bounds(0.0, 100.0);

        assert_eq!(graph.title, Some("CPU".to_string()));
        assert_eq!(graph.y_label, Some("%".to_string()));
        assert_eq!(graph.max_points, 50);
        assert_eq!(graph.color, Color::Green);
        assert_eq!(graph.update_interval, Duration::from_millis(500));
        assert_eq!(graph.y_bounds, Some((0.0, 100.0)));
    }

    #[test]
    fn test_all_graph_types() {
        let types = vec![GraphType::Line, GraphType::Bar, GraphType::Scatter];

        for graph_type in types {
            let mut graph = LiveGraph::new(graph_type);
            graph.add_points(&[10.0, 20.0, 30.0]);
            assert_eq!(graph.data_points(), 3);
        }
    }

    // ============ COMPREHENSIVE EDGE CASE TESTS ============

    #[test]
    fn test_live_graph_with_very_long_title() {
        let long_title = "A".repeat(10000);
        let graph = LiveGraph::new(GraphType::Line).with_title(long_title.clone());
        assert_eq!(graph.title, Some(long_title));
    }

    #[test]
    fn test_live_graph_with_unicode_title() {
        let graph = LiveGraph::new(GraphType::Line).with_title("📊 ライブグラフ 🎯");
        assert!(graph.title.clone().unwrap().contains("📊"));
        assert!(graph.title.clone().unwrap().contains("ライブグラフ"));
    }

    #[test]
    fn test_live_graph_with_empty_title() {
        let graph = LiveGraph::new(GraphType::Line).with_title("");
        assert_eq!(graph.title, Some("".to_string()));
    }

    #[test]
    fn test_live_graph_with_very_long_y_label() {
        let long_label = "Y".repeat(1000);
        let graph = LiveGraph::new(GraphType::Line).with_y_label(long_label.clone());
        assert_eq!(graph.y_label, Some(long_label));
    }

    #[test]
    fn test_live_graph_with_unicode_y_label() {
        let graph = LiveGraph::new(GraphType::Bar).with_y_label("値 📈");
        assert!(graph.y_label.clone().unwrap().contains("値"));
    }

    #[test]
    fn test_live_graph_with_empty_y_label() {
        let graph = LiveGraph::new(GraphType::Line).with_y_label("");
        assert_eq!(graph.y_label, Some("".to_string()));
    }

    #[test]
    fn test_live_graph_with_zero_max_points() {
        let mut graph = LiveGraph::new(GraphType::Line).with_max_points(0);
        graph.add_points(&[1.0, 2.0, 3.0]);
        assert_eq!(graph.data_points(), 0);
    }

    #[test]
    fn test_live_graph_with_one_max_point() {
        let mut graph = LiveGraph::new(GraphType::Line).with_max_points(1);
        graph.add_points(&[1.0, 2.0, 3.0, 4.0]);
        assert_eq!(graph.data_points(), 1);
        assert_eq!(graph.latest(), Some(4.0));
    }

    #[test]
    fn test_live_graph_with_very_large_max_points() {
        let graph = LiveGraph::new(GraphType::Line).with_max_points(1_000_000);
        assert_eq!(graph.max_points, 1_000_000);
    }

    #[test]
    fn test_live_graph_with_many_data_points() {
        let mut graph = LiveGraph::new(GraphType::Line).with_max_points(10_000);
        for i in 0..10_000 {
            graph.add_point(i as f64);
        }
        assert_eq!(graph.data_points(), 10_000);
        assert_eq!(graph.latest(), Some(9_999.0));
    }

    #[test]
    fn test_add_point_with_extreme_positive_value() {
        let mut graph = LiveGraph::new(GraphType::Line);
        graph.add_point(f64::MAX);
        assert_eq!(graph.latest(), Some(f64::MAX));
        assert_eq!(graph.max(), Some(f64::MAX));
    }

    #[test]
    fn test_add_point_with_extreme_negative_value() {
        let mut graph = LiveGraph::new(GraphType::Line);
        graph.add_point(f64::MIN);
        assert_eq!(graph.latest(), Some(f64::MIN));
        assert_eq!(graph.min(), Some(f64::MIN));
    }

    #[test]
    fn test_add_points_with_negative_values() {
        let mut graph = LiveGraph::new(GraphType::Line);
        graph.add_points(&[-100.0, -50.0, -75.0]);
        assert_eq!(graph.min(), Some(-100.0));
        assert_eq!(graph.max(), Some(-50.0));
        assert_eq!(graph.average(), Some(-75.0));
    }

    #[test]
    fn test_add_points_with_zero_values() {
        let mut graph = LiveGraph::new(GraphType::Line);
        graph.add_points(&[0.0, 0.0, 0.0]);
        assert_eq!(graph.min(), Some(0.0));
        assert_eq!(graph.max(), Some(0.0));
        assert_eq!(graph.average(), Some(0.0));
    }

    #[test]
    fn test_add_points_with_mixed_values() {
        let mut graph = LiveGraph::new(GraphType::Line);
        graph.add_points(&[-10.0, 0.0, 10.0, -5.0, 5.0]);
        assert_eq!(graph.min(), Some(-10.0));
        assert_eq!(graph.max(), Some(10.0));
    }

    #[test]
    fn test_add_points_with_fractional_values() {
        let mut graph = LiveGraph::new(GraphType::Line);
        graph.add_points(&[0.123456789, 3.141592653, 2.718281828]);
        assert_eq!(graph.data_points(), 3);
        let avg = graph.average().unwrap();
        let expected_avg = (0.123456789 + 3.141592653 + 2.718281828) / 3.0;
        assert!((avg - expected_avg).abs() < 1e-10);
    }

    #[test]
    fn test_add_points_empty_slice() {
        let mut graph = LiveGraph::new(GraphType::Line);
        graph.add_points(&[]);
        assert_eq!(graph.data_points(), 0);
    }

    #[test]
    fn test_windowing_with_single_point_overflow() {
        let mut graph = LiveGraph::new(GraphType::Line).with_max_points(1);
        for i in 0..100 {
            graph.add_point(i as f64);
        }
        assert_eq!(graph.data_points(), 1);
        assert_eq!(graph.latest(), Some(99.0));
    }

    #[test]
    fn test_clear_after_many_points() {
        let mut graph = LiveGraph::new(GraphType::Line).with_max_points(1000);
        for i in 0..1000 {
            graph.add_point(i as f64);
        }
        assert_eq!(graph.data_points(), 1000);
        graph.clear();
        assert_eq!(graph.data_points(), 0);
        assert_eq!(graph.latest(), None);
        assert_eq!(graph.min(), None);
        assert_eq!(graph.max(), None);
        assert_eq!(graph.average(), None);
    }

    #[test]
    fn test_with_extreme_y_bounds() {
        let graph = LiveGraph::new(GraphType::Line).with_y_bounds(f64::MIN, f64::MAX);
        assert_eq!(graph.y_bounds, Some((f64::MIN, f64::MAX)));
        assert!(!graph.auto_scale);
    }

    #[test]
    fn test_with_negative_y_bounds() {
        let graph = LiveGraph::new(GraphType::Line).with_y_bounds(-100.0, -10.0);
        assert_eq!(graph.y_bounds, Some((-100.0, -10.0)));
    }

    #[test]
    fn test_with_zero_sized_y_bounds() {
        let graph = LiveGraph::new(GraphType::Line).with_y_bounds(50.0, 50.0);
        assert_eq!(graph.y_bounds, Some((50.0, 50.0)));
    }

    #[test]
    fn test_with_inverted_y_bounds() {
        let graph = LiveGraph::new(GraphType::Line).with_y_bounds(100.0, 0.0);
        assert_eq!(graph.y_bounds, Some((100.0, 0.0)));
    }

    #[test]
    fn test_with_very_short_update_interval() {
        let interval = Duration::from_nanos(1);
        let graph = LiveGraph::new(GraphType::Line).with_update_interval(interval);
        assert_eq!(graph.update_interval, interval);
    }

    #[test]
    fn test_with_very_long_update_interval() {
        let interval = Duration::from_secs(3600);
        let graph = LiveGraph::new(GraphType::Line).with_update_interval(interval);
        assert_eq!(graph.update_interval, interval);
    }

    #[test]
    fn test_graph_type_equality() {
        assert_eq!(GraphType::Line, GraphType::Line);
        assert_eq!(GraphType::Bar, GraphType::Bar);
        assert_eq!(GraphType::Scatter, GraphType::Scatter);
        assert_ne!(GraphType::Line, GraphType::Bar);
        assert_ne!(GraphType::Bar, GraphType::Scatter);
        assert_ne!(GraphType::Scatter, GraphType::Line);
    }

    #[test]
    fn test_graph_type_copy() {
        let original = GraphType::Line;
        let copied = original;
        assert_eq!(original, copied);
    }

    #[test]
    fn test_set_type_all_combinations() {
        let mut graph = LiveGraph::new(GraphType::Line);

        graph.set_type(GraphType::Bar);
        assert_eq!(graph.graph_type, GraphType::Bar);

        graph.set_type(GraphType::Scatter);
        assert_eq!(graph.graph_type, GraphType::Scatter);

        graph.set_type(GraphType::Line);
        assert_eq!(graph.graph_type, GraphType::Line);
    }

    #[test]
    fn test_clone() {
        let mut original = LiveGraph::new(GraphType::Line)
            .with_title("Original")
            .with_max_points(50);
        original.add_points(&[1.0, 2.0, 3.0]);

        let cloned = original.clone();
        assert_eq!(original.title, cloned.title);
        assert_eq!(original.max_points, cloned.max_points);
        assert_eq!(original.data_points(), cloned.data_points());
        assert_eq!(original.latest(), cloned.latest());
    }

    #[test]
    fn test_multiple_title_calls() {
        let graph = LiveGraph::new(GraphType::Line)
            .with_title("First")
            .with_title("Second")
            .with_title("Third");
        assert_eq!(graph.title, Some("Third".to_string()));
    }

    #[test]
    fn test_multiple_y_label_calls() {
        let graph = LiveGraph::new(GraphType::Line)
            .with_y_label("First")
            .with_y_label("Second")
            .with_y_label("Third");
        assert_eq!(graph.y_label, Some("Third".to_string()));
    }

    #[test]
    fn test_multiple_max_points_calls() {
        let graph = LiveGraph::new(GraphType::Line)
            .with_max_points(10)
            .with_max_points(50)
            .with_max_points(100);
        assert_eq!(graph.max_points, 100);
    }

    #[test]
    fn test_multiple_y_bounds_calls() {
        let graph = LiveGraph::new(GraphType::Line)
            .with_y_bounds(0.0, 10.0)
            .with_y_bounds(10.0, 50.0)
            .with_y_bounds(50.0, 100.0);
        assert_eq!(graph.y_bounds, Some((50.0, 100.0)));
    }

    #[test]
    fn test_average_with_single_point() {
        let mut graph = LiveGraph::new(GraphType::Line);
        graph.add_point(42.0);
        assert_eq!(graph.average(), Some(42.0));
    }

    #[test]
    fn test_average_with_extreme_values() {
        let mut graph = LiveGraph::new(GraphType::Line);
        graph.add_points(&[f64::MIN, f64::MAX]);
        let avg = graph.average();
        assert!(avg.is_some());
    }

    #[test]
    fn test_min_max_with_single_point() {
        let mut graph = LiveGraph::new(GraphType::Line);
        graph.add_point(42.0);
        assert_eq!(graph.min(), Some(42.0));
        assert_eq!(graph.max(), Some(42.0));
    }

    #[test]
    fn test_min_max_with_duplicate_values() {
        let mut graph = LiveGraph::new(GraphType::Line);
        graph.add_points(&[42.0, 42.0, 42.0]);
        assert_eq!(graph.min(), Some(42.0));
        assert_eq!(graph.max(), Some(42.0));
    }

    #[test]
    fn test_auto_scale_toggle() {
        let graph1 = LiveGraph::new(GraphType::Line);
        assert!(graph1.auto_scale);

        let graph2 = LiveGraph::new(GraphType::Line).with_y_bounds(0.0, 100.0);
        assert!(!graph2.auto_scale);

        let graph3 = LiveGraph::new(GraphType::Line)
            .with_y_bounds(0.0, 100.0)
            .with_auto_scale(true);
        assert!(graph3.auto_scale);
        assert_eq!(graph3.y_bounds, None);
    }

    #[test]
    fn test_builder_pattern_chaining_complete() {
        let mut graph = LiveGraph::new(GraphType::Scatter)
            .with_title("Complete Test 📊")
            .with_y_label("Values 📈")
            .with_max_points(100)
            .with_color(Color::Magenta)
            .with_update_interval(Duration::from_millis(250))
            .with_y_bounds(-10.0, 110.0);

        graph.add_points(&[10.0, 20.0, 30.0, 40.0, 50.0]);

        assert_eq!(graph.graph_type, GraphType::Scatter);
        assert_eq!(graph.title, Some("Complete Test 📊".to_string()));
        assert_eq!(graph.y_label, Some("Values 📈".to_string()));
        assert_eq!(graph.max_points, 100);
        assert_eq!(graph.color, Color::Magenta);
        assert_eq!(graph.update_interval, Duration::from_millis(250));
        assert_eq!(graph.y_bounds, Some((-10.0, 110.0)));
        assert_eq!(graph.data_points(), 5);
    }

    #[test]
    fn test_all_graph_types_with_data() {
        for graph_type in [GraphType::Line, GraphType::Bar, GraphType::Scatter] {
            let mut graph = LiveGraph::new(graph_type)
                .with_title("Test")
                .with_y_label("Value")
                .with_max_points(50);

            graph.add_points(&[10.0, 20.0, 30.0, 40.0, 50.0]);

            assert_eq!(graph.data_points(), 5);
            assert_eq!(graph.min(), Some(10.0));
            assert_eq!(graph.max(), Some(50.0));
            assert_eq!(graph.average(), Some(30.0));
        }
    }

    #[test]
    fn test_default_configuration() {
        let graph = LiveGraph::default();
        assert_eq!(graph.graph_type, GraphType::Line);
        assert_eq!(graph.max_points, 100);
        assert_eq!(graph.color, Color::Cyan);
        assert_eq!(graph.update_interval, Duration::from_millis(100));
        assert!(graph.auto_scale);
        assert_eq!(graph.y_bounds, None);
        assert_eq!(graph.title, None);
        assert_eq!(graph.y_label, None);
        assert_eq!(graph.data_points(), 0);
    }

    #[test]
    fn test_rapid_updates() {
        let mut graph = LiveGraph::new(GraphType::Line).with_max_points(1000);
        for i in 0..1000 {
            graph.add_point(i as f64 * 0.1);
        }
        assert_eq!(graph.data_points(), 1000);
        assert_eq!(graph.min(), Some(0.0));
        assert!((graph.max().unwrap() - 99.9).abs() < 1e-10);
    }
}
