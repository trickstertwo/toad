/// Live real-time updating graphs
///
/// Provides real-time data visualization with automatic updates,
/// ring buffers for efficient memory usage, and multiple chart types
///
/// # Examples
///
/// ```
/// use toad::live_graphs::{LiveGraph, GraphType};
///
/// let mut graph = LiveGraph::new("CPU Usage", GraphType::Line);
/// graph.add_data_point(50.0);
/// graph.add_data_point(75.0);
/// ```
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::time::{Duration, Instant};

/// Type of graph to display
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GraphType {
    /// Line graph
    Line,
    /// Bar graph
    Bar,
    /// Sparkline
    Sparkline,
}

/// Update frequency for live graphs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateFrequency {
    /// Update as fast as possible
    RealTime,
    /// Update at specific interval
    Interval(Duration),
}

/// A single data point with timestamp
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DataPoint {
    /// Data value
    pub value: f64,
    /// Timestamp when recorded
    pub timestamp: Instant,
}

impl DataPoint {
    /// Create a new data point
    pub fn new(value: f64) -> Self {
        Self {
            value,
            timestamp: Instant::now(),
        }
    }

    /// Create a data point with specific timestamp
    pub fn with_timestamp(value: f64, timestamp: Instant) -> Self {
        Self { value, timestamp }
    }
}

/// Live updating graph
#[derive(Debug, Clone)]
pub struct LiveGraph {
    /// Graph title
    title: String,
    /// Graph type
    graph_type: GraphType,
    /// Ring buffer for data points
    data: VecDeque<DataPoint>,
    /// Maximum data points to keep
    max_points: usize,
    /// Update frequency
    update_frequency: UpdateFrequency,
    /// Last update time
    last_update: Option<Instant>,
    /// Minimum value for scaling
    min_value: Option<f64>,
    /// Maximum value for scaling
    max_value: Option<f64>,
    /// Auto-scale range
    auto_scale: bool,
}

impl LiveGraph {
    /// Create a new live graph
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::live_graphs::{LiveGraph, GraphType};
    ///
    /// let graph = LiveGraph::new("CPU Usage", GraphType::Line);
    /// ```
    pub fn new(title: impl Into<String>, graph_type: GraphType) -> Self {
        Self {
            title: title.into(),
            graph_type,
            data: VecDeque::new(),
            max_points: 100,
            update_frequency: UpdateFrequency::RealTime,
            last_update: None,
            min_value: None,
            max_value: None,
            auto_scale: true,
        }
    }

    /// Set maximum data points
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::live_graphs::{LiveGraph, GraphType};
    ///
    /// let mut graph = LiveGraph::new("Test", GraphType::Line);
    /// graph.set_max_points(200);
    /// ```
    pub fn set_max_points(&mut self, max: usize) {
        self.max_points = max;
        // Trim existing data if needed
        while self.data.len() > self.max_points {
            self.data.pop_front();
        }
    }

    /// Set update frequency
    pub fn set_update_frequency(&mut self, frequency: UpdateFrequency) {
        self.update_frequency = frequency;
    }

    /// Set value range for fixed scaling
    pub fn set_range(&mut self, min: f64, max: f64) {
        self.min_value = Some(min);
        self.max_value = Some(max);
        self.auto_scale = false;
    }

    /// Enable auto-scaling
    pub fn enable_auto_scale(&mut self) {
        self.auto_scale = true;
        self.min_value = None;
        self.max_value = None;
    }

    /// Add a data point
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::live_graphs::{LiveGraph, GraphType};
    ///
    /// let mut graph = LiveGraph::new("Test", GraphType::Line);
    /// graph.add_data_point(42.0);
    /// assert_eq!(graph.data_count(), 1);
    /// ```
    pub fn add_data_point(&mut self, value: f64) {
        // Check update frequency
        if let UpdateFrequency::Interval(interval) = self.update_frequency
            && let Some(last) = self.last_update
                && last.elapsed() < interval {
                    return; // Skip this update
                }

        let point = DataPoint::new(value);
        self.data.push_back(point);
        self.last_update = Some(Instant::now());

        // Remove old points if over limit
        if self.data.len() > self.max_points {
            self.data.pop_front();
        }
    }

    /// Add multiple data points
    pub fn add_data_points(&mut self, values: &[f64]) {
        for &value in values {
            self.add_data_point(value);
        }
    }

    /// Get current data points
    pub fn data(&self) -> &VecDeque<DataPoint> {
        &self.data
    }

    /// Get data values only (without timestamps)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::live_graphs::{LiveGraph, GraphType};
    ///
    /// let mut graph = LiveGraph::new("Test", GraphType::Line);
    /// graph.add_data_point(1.0);
    /// graph.add_data_point(2.0);
    /// let values = graph.values();
    /// assert_eq!(values, vec![1.0, 2.0]);
    /// ```
    pub fn values(&self) -> Vec<f64> {
        self.data.iter().map(|p| p.value).collect()
    }

    /// Get number of data points
    pub fn data_count(&self) -> usize {
        self.data.len()
    }

    /// Clear all data
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::live_graphs::{LiveGraph, GraphType};
    ///
    /// let mut graph = LiveGraph::new("Test", GraphType::Line);
    /// graph.add_data_point(42.0);
    /// graph.clear();
    /// assert_eq!(graph.data_count(), 0);
    /// ```
    pub fn clear(&mut self) {
        self.data.clear();
        self.last_update = None;
    }

    /// Get current value range
    pub fn value_range(&self) -> (f64, f64) {
        if !self.auto_scale {
            return (
                self.min_value.unwrap_or(0.0),
                self.max_value.unwrap_or(100.0),
            );
        }

        if self.data.is_empty() {
            return (0.0, 1.0);
        }

        let mut min = f64::INFINITY;
        let mut max = f64::NEG_INFINITY;

        for point in &self.data {
            min = min.min(point.value);
            max = max.max(point.value);
        }

        // Add 10% padding
        let range = max - min;
        let padding = range * 0.1;
        (min - padding, max + padding)
    }

    /// Get graph title
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Set graph title
    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
    }

    /// Get graph type
    pub fn graph_type(&self) -> GraphType {
        self.graph_type
    }

    /// Get average value
    pub fn average(&self) -> f64 {
        if self.data.is_empty() {
            return 0.0;
        }
        self.data.iter().map(|p| p.value).sum::<f64>() / self.data.len() as f64
    }

    /// Get latest value
    pub fn latest(&self) -> Option<f64> {
        self.data.back().map(|p| p.value)
    }

    /// Get rate of change (delta between last two points)
    pub fn rate_of_change(&self) -> Option<f64> {
        if self.data.len() < 2 {
            return None;
        }

        let last = self.data.back()?.value;
        let prev = self.data.get(self.data.len() - 2)?.value;
        Some(last - prev)
    }
}

/// Manager for multiple live graphs
#[derive(Debug, Clone, Default)]
pub struct LiveGraphManager {
    /// All managed graphs
    graphs: Vec<LiveGraph>,
}

impl LiveGraphManager {
    /// Create a new graph manager
    pub fn new() -> Self {
        Self { graphs: Vec::new() }
    }

    /// Add a graph
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::live_graphs::{LiveGraph, LiveGraphManager, GraphType};
    ///
    /// let mut manager = LiveGraphManager::new();
    /// manager.add_graph(LiveGraph::new("CPU", GraphType::Line));
    /// assert_eq!(manager.graph_count(), 1);
    /// ```
    pub fn add_graph(&mut self, graph: LiveGraph) {
        self.graphs.push(graph);
    }

    /// Get graph by index
    pub fn get_graph(&self, index: usize) -> Option<&LiveGraph> {
        self.graphs.get(index)
    }

    /// Get mutable graph by index
    pub fn get_graph_mut(&mut self, index: usize) -> Option<&mut LiveGraph> {
        self.graphs.get_mut(index)
    }

    /// Find graph by title
    pub fn find_graph(&self, title: &str) -> Option<&LiveGraph> {
        self.graphs.iter().find(|g| g.title == title)
    }

    /// Find mutable graph by title
    pub fn find_graph_mut(&mut self, title: &str) -> Option<&mut LiveGraph> {
        self.graphs.iter_mut().find(|g| g.title == title)
    }

    /// Remove graph by index
    pub fn remove_graph(&mut self, index: usize) -> Option<LiveGraph> {
        if index < self.graphs.len() {
            Some(self.graphs.remove(index))
        } else {
            None
        }
    }

    /// Get all graphs
    pub fn graphs(&self) -> &[LiveGraph] {
        &self.graphs
    }

    /// Get number of graphs
    pub fn graph_count(&self) -> usize {
        self.graphs.len()
    }

    /// Clear all graphs
    pub fn clear(&mut self) {
        self.graphs.clear();
    }

    /// Update all graphs with new data
    pub fn update_all(&mut self, data: &[f64]) {
        for (i, &value) in data.iter().enumerate() {
            if let Some(graph) = self.graphs.get_mut(i) {
                graph.add_data_point(value);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_data_point_creation() {
        let point = DataPoint::new(42.0);
        assert_eq!(point.value, 42.0);
    }

    #[test]
    fn test_live_graph_creation() {
        let graph = LiveGraph::new("Test", GraphType::Line);
        assert_eq!(graph.title(), "Test");
        assert_eq!(graph.graph_type(), GraphType::Line);
        assert_eq!(graph.data_count(), 0);
    }

    #[test]
    fn test_add_data_point() {
        let mut graph = LiveGraph::new("Test", GraphType::Line);
        graph.add_data_point(42.0);
        assert_eq!(graph.data_count(), 1);
        assert_eq!(graph.latest(), Some(42.0));
    }

    #[test]
    fn test_add_multiple_data_points() {
        let mut graph = LiveGraph::new("Test", GraphType::Line);
        graph.add_data_points(&[1.0, 2.0, 3.0]);
        assert_eq!(graph.data_count(), 3);
    }

    #[test]
    fn test_max_points_limit() {
        let mut graph = LiveGraph::new("Test", GraphType::Line);
        graph.set_max_points(3);

        graph.add_data_points(&[1.0, 2.0, 3.0, 4.0, 5.0]);
        assert_eq!(graph.data_count(), 3);
        assert_eq!(graph.values(), vec![3.0, 4.0, 5.0]);
    }

    #[test]
    fn test_clear() {
        let mut graph = LiveGraph::new("Test", GraphType::Line);
        graph.add_data_point(42.0);
        graph.clear();
        assert_eq!(graph.data_count(), 0);
    }

    #[test]
    fn test_values() {
        let mut graph = LiveGraph::new("Test", GraphType::Line);
        graph.add_data_points(&[1.0, 2.0, 3.0]);
        assert_eq!(graph.values(), vec![1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_value_range_auto() {
        let mut graph = LiveGraph::new("Test", GraphType::Line);
        graph.add_data_points(&[10.0, 20.0, 30.0]);
        let (min, max) = graph.value_range();
        assert!(min < 10.0); // Should have padding
        assert!(max > 30.0); // Should have padding
    }

    #[test]
    fn test_value_range_fixed() {
        let mut graph = LiveGraph::new("Test", GraphType::Line);
        graph.set_range(0.0, 100.0);
        let (min, max) = graph.value_range();
        assert_eq!(min, 0.0);
        assert_eq!(max, 100.0);
    }

    #[test]
    fn test_average() {
        let mut graph = LiveGraph::new("Test", GraphType::Line);
        graph.add_data_points(&[10.0, 20.0, 30.0]);
        assert_eq!(graph.average(), 20.0);
    }

    #[test]
    fn test_average_empty() {
        let graph = LiveGraph::new("Test", GraphType::Line);
        assert_eq!(graph.average(), 0.0);
    }

    #[test]
    fn test_latest() {
        let mut graph = LiveGraph::new("Test", GraphType::Line);
        graph.add_data_point(42.0);
        assert_eq!(graph.latest(), Some(42.0));
    }

    #[test]
    fn test_latest_empty() {
        let graph = LiveGraph::new("Test", GraphType::Line);
        assert_eq!(graph.latest(), None);
    }

    #[test]
    fn test_rate_of_change() {
        let mut graph = LiveGraph::new("Test", GraphType::Line);
        graph.add_data_point(10.0);
        graph.add_data_point(15.0);
        assert_eq!(graph.rate_of_change(), Some(5.0));
    }

    #[test]
    fn test_rate_of_change_insufficient_data() {
        let mut graph = LiveGraph::new("Test", GraphType::Line);
        graph.add_data_point(10.0);
        assert_eq!(graph.rate_of_change(), None);
    }

    #[test]
    fn test_set_title() {
        let mut graph = LiveGraph::new("Old", GraphType::Line);
        graph.set_title("New");
        assert_eq!(graph.title(), "New");
    }

    #[test]
    fn test_update_frequency_interval() {
        let mut graph = LiveGraph::new("Test", GraphType::Line);
        graph.set_update_frequency(UpdateFrequency::Interval(Duration::from_millis(100)));

        graph.add_data_point(1.0);
        graph.add_data_point(2.0); // Should be skipped due to interval
        assert_eq!(graph.data_count(), 1);

        thread::sleep(Duration::from_millis(150));
        graph.add_data_point(3.0); // Should be added
        assert_eq!(graph.data_count(), 2);
    }

    #[test]
    fn test_graph_manager_creation() {
        let manager = LiveGraphManager::new();
        assert_eq!(manager.graph_count(), 0);
    }

    #[test]
    fn test_graph_manager_add() {
        let mut manager = LiveGraphManager::new();
        manager.add_graph(LiveGraph::new("CPU", GraphType::Line));
        assert_eq!(manager.graph_count(), 1);
    }

    #[test]
    fn test_graph_manager_get() {
        let mut manager = LiveGraphManager::new();
        manager.add_graph(LiveGraph::new("CPU", GraphType::Line));
        assert!(manager.get_graph(0).is_some());
        assert!(manager.get_graph(1).is_none());
    }

    #[test]
    fn test_graph_manager_find() {
        let mut manager = LiveGraphManager::new();
        manager.add_graph(LiveGraph::new("CPU", GraphType::Line));
        assert!(manager.find_graph("CPU").is_some());
        assert!(manager.find_graph("Memory").is_none());
    }

    #[test]
    fn test_graph_manager_remove() {
        let mut manager = LiveGraphManager::new();
        manager.add_graph(LiveGraph::new("CPU", GraphType::Line));
        manager.remove_graph(0);
        assert_eq!(manager.graph_count(), 0);
    }

    #[test]
    fn test_graph_manager_clear() {
        let mut manager = LiveGraphManager::new();
        manager.add_graph(LiveGraph::new("CPU", GraphType::Line));
        manager.add_graph(LiveGraph::new("Memory", GraphType::Bar));
        manager.clear();
        assert_eq!(manager.graph_count(), 0);
    }

    #[test]
    fn test_graph_manager_update_all() {
        let mut manager = LiveGraphManager::new();
        manager.add_graph(LiveGraph::new("CPU", GraphType::Line));
        manager.add_graph(LiveGraph::new("Memory", GraphType::Line));

        manager.update_all(&[50.0, 75.0]);

        assert_eq!(manager.get_graph(0).unwrap().latest(), Some(50.0));
        assert_eq!(manager.get_graph(1).unwrap().latest(), Some(75.0));
    }

    #[test]
    fn test_graph_types() {
        // Test all type variants
        let _types = [GraphType::Line, GraphType::Bar, GraphType::Sparkline];
    }
}
