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

use crate::widgets::{BarChart, BarData, DataSeries, LineChart, ScatterPlot, ScatterSeries};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Color,
    widgets::Widget,
};
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
        self.data.iter().copied().min_by(|a, b| a.partial_cmp(b).unwrap())
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
        self.data.iter().copied().max_by(|a, b| a.partial_cmp(b).unwrap())
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
                let series = DataSeries::new("data", self.data.clone())
                    .with_color(self.color);

                let mut chart = LineChart::new()
                    .add_series(series);

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
                        BarData::new(format!("{}", i + 1), value)
                            .with_color(self.color)
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

                let series = ScatterSeries::new("data", points)
                    .with_color(self.color);

                let mut chart = ScatterPlot::new()
                    .add_series(series);

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
        let mut graph = LiveGraph::new(GraphType::Line)
            .with_update_interval(Duration::from_millis(100));
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
}
