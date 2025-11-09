//! Render profiling for debugging slow renders
//!
//! Tracks render times per component/widget to identify performance bottlenecks.
//!
//! # Examples
//!
//! ```
//! use toad::widgets::RenderProfiler;
//! use std::time::Duration;
//!
//! let mut profiler = RenderProfiler::new();
//! profiler.start_component("FileTree");
//! // ... render FileTree ...
//! profiler.end_component();
//!
//! assert!(profiler.component_count() > 0);
//! ```

use std::collections::HashMap;
use std::time::{Duration, Instant};

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph, Widget},
};

/// Component render statistics
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ComponentStats {
    /// Total render time
    pub total_time: Duration,
    /// Number of renders
    pub render_count: usize,
    /// Minimum render time
    pub min_time: Duration,
    /// Maximum render time
    pub max_time: Duration,
}

impl ComponentStats {
    /// Create new component stats
    pub fn new() -> Self {
        Self {
            total_time: Duration::ZERO,
            render_count: 0,
            min_time: Duration::MAX,
            max_time: Duration::ZERO,
        }
    }

    /// Get average render time in milliseconds
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::ComponentStats;
    /// use std::time::Duration;
    ///
    /// let mut stats = ComponentStats::new();
    /// assert_eq!(stats.average_ms(), 0.0);
    /// ```
    pub fn average_ms(&self) -> f64 {
        if self.render_count == 0 {
            return 0.0;
        }
        (self.total_time.as_secs_f64() / self.render_count as f64) * 1000.0
    }

    /// Get minimum render time in milliseconds
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::ComponentStats;
    ///
    /// let stats = ComponentStats::new();
    /// assert!(stats.min_ms() >= 0.0);
    /// ```
    pub fn min_ms(&self) -> f64 {
        if self.min_time == Duration::MAX {
            0.0
        } else {
            self.min_time.as_secs_f64() * 1000.0
        }
    }

    /// Get maximum render time in milliseconds
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::ComponentStats;
    ///
    /// let stats = ComponentStats::new();
    /// assert_eq!(stats.max_ms(), 0.0);
    /// ```
    pub fn max_ms(&self) -> f64 {
        self.max_time.as_secs_f64() * 1000.0
    }

    /// Update stats with new render time
    pub(crate) fn update(&mut self, duration: Duration) {
        self.total_time += duration;
        self.render_count += 1;
        self.min_time = self.min_time.min(duration);
        self.max_time = self.max_time.max(duration);
    }
}

impl Default for ComponentStats {
    fn default() -> Self {
        Self::new()
    }
}

/// Active component tracking
#[derive(Debug, Clone)]
struct ActiveComponent {
    name: String,
    start_time: Instant,
}

/// Render profiler
///
/// Tracks render performance per component to identify bottlenecks.
///
/// # Examples
///
/// ```
/// use toad::widgets::RenderProfiler;
///
/// let mut profiler = RenderProfiler::new();
/// profiler.start_component("Panel");
/// // ... render work ...
/// profiler.end_component();
///
/// let stats = profiler.get_stats("Panel");
/// assert!(stats.is_some());
/// ```
#[derive(Debug, Clone)]
pub struct RenderProfiler {
    /// Component statistics
    components: HashMap<String, ComponentStats>,
    /// Currently profiling component
    active: Option<ActiveComponent>,
    /// Display settings
    show_details: bool,
    /// Sort by time (true) or name (false)
    sort_by_time: bool,
    /// Warning threshold in ms
    warning_threshold_ms: f64,
    /// Critical threshold in ms
    critical_threshold_ms: f64,
}

impl RenderProfiler {
    /// Create a new render profiler
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::RenderProfiler;
    ///
    /// let profiler = RenderProfiler::new();
    /// ```
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
            active: None,
            show_details: true,
            sort_by_time: true,
            warning_threshold_ms: 8.0,   // 120 FPS target
            critical_threshold_ms: 16.0, // 60 FPS minimum
        }
    }

    /// Show detailed statistics
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::RenderProfiler;
    ///
    /// let profiler = RenderProfiler::new().with_details(false);
    /// ```
    pub fn with_details(mut self, show: bool) -> Self {
        self.show_details = show;
        self
    }

    /// Sort results by time (true) or name (false)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::RenderProfiler;
    ///
    /// let profiler = RenderProfiler::new().with_sort_by_time(true);
    /// ```
    pub fn with_sort_by_time(mut self, by_time: bool) -> Self {
        self.sort_by_time = by_time;
        self
    }

    /// Set warning threshold in milliseconds
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::RenderProfiler;
    ///
    /// let profiler = RenderProfiler::new().with_warning_threshold(5.0);
    /// ```
    pub fn with_warning_threshold(mut self, ms: f64) -> Self {
        self.warning_threshold_ms = ms;
        self
    }

    /// Set critical threshold in milliseconds
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::RenderProfiler;
    ///
    /// let profiler = RenderProfiler::new().with_critical_threshold(10.0);
    /// ```
    pub fn with_critical_threshold(mut self, ms: f64) -> Self {
        self.critical_threshold_ms = ms;
        self
    }

    /// Start profiling a component
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::RenderProfiler;
    ///
    /// let mut profiler = RenderProfiler::new();
    /// profiler.start_component("FileTree");
    /// profiler.end_component();
    /// ```
    pub fn start_component(&mut self, name: &str) {
        self.active = Some(ActiveComponent {
            name: name.to_string(),
            start_time: Instant::now(),
        });
    }

    /// End profiling current component
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::RenderProfiler;
    ///
    /// let mut profiler = RenderProfiler::new();
    /// profiler.start_component("Panel");
    /// profiler.end_component();
    /// assert_eq!(profiler.component_count(), 1);
    /// ```
    pub fn end_component(&mut self) {
        if let Some(active) = self.active.take() {
            let duration = active.start_time.elapsed();
            let stats = self.components.entry(active.name).or_default();
            stats.update(duration);
        }
    }

    /// Profile a component render with closure
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::RenderProfiler;
    ///
    /// let mut profiler = RenderProfiler::new();
    /// profiler.profile("Button", || {
    ///     // render button
    /// });
    /// ```
    pub fn profile<F: FnOnce()>(&mut self, name: &str, f: F) {
        self.start_component(name);
        f();
        self.end_component();
    }

    /// Get statistics for a component
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::RenderProfiler;
    ///
    /// let mut profiler = RenderProfiler::new();
    /// profiler.start_component("Widget");
    /// profiler.end_component();
    ///
    /// let stats = profiler.get_stats("Widget");
    /// assert!(stats.is_some());
    /// ```
    pub fn get_stats(&self, name: &str) -> Option<ComponentStats> {
        self.components.get(name).copied()
    }

    /// Get number of tracked components
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::RenderProfiler;
    ///
    /// let mut profiler = RenderProfiler::new();
    /// assert_eq!(profiler.component_count(), 0);
    ///
    /// profiler.start_component("A");
    /// profiler.end_component();
    /// assert_eq!(profiler.component_count(), 1);
    /// ```
    pub fn component_count(&self) -> usize {
        self.components.len()
    }

    /// Get slowest component
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::RenderProfiler;
    ///
    /// let profiler = RenderProfiler::new();
    /// assert!(profiler.slowest_component().is_none());
    /// ```
    pub fn slowest_component(&self) -> Option<(&str, ComponentStats)> {
        self.components
            .iter()
            .max_by(|a, b| a.1.average_ms().total_cmp(&b.1.average_ms()))
            .map(|(name, stats)| (name.as_str(), *stats))
    }

    /// Get total render time across all components
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::RenderProfiler;
    ///
    /// let profiler = RenderProfiler::new();
    /// assert_eq!(profiler.total_time_ms(), 0.0);
    /// ```
    pub fn total_time_ms(&self) -> f64 {
        self.components
            .values()
            .map(|s| s.total_time.as_secs_f64())
            .sum::<f64>()
            * 1000.0
    }

    /// Reset all statistics
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::RenderProfiler;
    ///
    /// let mut profiler = RenderProfiler::new();
    /// profiler.start_component("A");
    /// profiler.end_component();
    /// profiler.reset();
    /// assert_eq!(profiler.component_count(), 0);
    /// ```
    pub fn reset(&mut self) {
        self.components.clear();
        self.active = None;
    }

    /// Get color for component based on average render time
    fn component_color(&self, avg_ms: f64) -> Color {
        if avg_ms >= self.critical_threshold_ms {
            Color::Red
        } else if avg_ms >= self.warning_threshold_ms {
            Color::Yellow
        } else {
            Color::Green
        }
    }

    /// Render as lines for display
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::RenderProfiler;
    ///
    /// let mut profiler = RenderProfiler::new();
    /// profiler.start_component("Test");
    /// profiler.end_component();
    ///
    /// let lines = profiler.render_lines();
    /// assert!(!lines.is_empty());
    /// ```
    pub fn render_lines(&self) -> Vec<Line<'static>> {
        let mut lines = Vec::new();

        // Header
        lines.push(Line::from(vec![
            Span::styled("Render Profiler", Style::default().fg(Color::Cyan)),
            Span::raw(format!(" ({} components)", self.component_count())),
        ]));

        if self.components.is_empty() {
            lines.push(Line::from("No components profiled yet"));
            return lines;
        }

        // Sort components
        let mut items: Vec<_> = self.components.iter().collect();
        if self.sort_by_time {
            items.sort_by(|a, b| b.1.average_ms().total_cmp(&a.1.average_ms()));
        } else {
            items.sort_by_key(|&(name, _)| name);
        }

        // Component details
        for (name, stats) in items {
            let avg_ms = stats.average_ms();
            let color = self.component_color(avg_ms);

            if self.show_details {
                lines.push(Line::from(vec![
                    Span::styled(format!("  {}", name), Style::default().fg(color)),
                    Span::raw(format!(
                        ": {:.2}ms avg ({:.2}-{:.2}ms, {} renders)",
                        avg_ms,
                        stats.min_ms(),
                        stats.max_ms(),
                        stats.render_count
                    )),
                ]));
            } else {
                lines.push(Line::from(vec![
                    Span::styled(format!("  {}", name), Style::default().fg(color)),
                    Span::raw(format!(": {:.2}ms", avg_ms)),
                ]));
            }
        }

        // Summary
        lines.push(Line::from(""));
        lines.push(Line::from(format!("Total: {:.2}ms", self.total_time_ms())));

        if let Some((name, stats)) = self.slowest_component() {
            lines.push(Line::from(format!(
                "Slowest: {} ({:.2}ms)",
                name,
                stats.average_ms()
            )));
        }

        lines
    }
}

impl Default for RenderProfiler {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for &RenderProfiler {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let lines = self.render_lines();
        let paragraph = Paragraph::new(lines).block(Block::default());
        paragraph.render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_component_stats_new() {
        let stats = ComponentStats::new();
        assert_eq!(stats.render_count, 0);
        assert_eq!(stats.average_ms(), 0.0);
    }

    #[test]
    fn test_component_stats_update() {
        let mut stats = ComponentStats::new();
        stats.update(Duration::from_millis(10));
        stats.update(Duration::from_millis(20));

        assert_eq!(stats.render_count, 2);
        assert!((stats.average_ms() - 15.0).abs() < 0.1);
        assert!((stats.min_ms() - 10.0).abs() < 0.1);
        assert!((stats.max_ms() - 20.0).abs() < 0.1);
    }

    #[test]
    fn test_new() {
        let profiler = RenderProfiler::new();
        assert_eq!(profiler.component_count(), 0);
        assert!(profiler.show_details);
        assert!(profiler.sort_by_time);
    }

    #[test]
    fn test_with_details() {
        let profiler = RenderProfiler::new().with_details(false);
        assert!(!profiler.show_details);
    }

    #[test]
    fn test_with_sort() {
        let profiler = RenderProfiler::new().with_sort_by_time(false);
        assert!(!profiler.sort_by_time);
    }

    #[test]
    fn test_with_thresholds() {
        let profiler = RenderProfiler::new()
            .with_warning_threshold(5.0)
            .with_critical_threshold(10.0);

        assert_eq!(profiler.warning_threshold_ms, 5.0);
        assert_eq!(profiler.critical_threshold_ms, 10.0);
    }

    #[test]
    fn test_start_end_component() {
        let mut profiler = RenderProfiler::new();
        profiler.start_component("Test");
        thread::sleep(Duration::from_millis(1));
        profiler.end_component();

        assert_eq!(profiler.component_count(), 1);
        let stats = profiler.get_stats("Test");
        assert!(stats.is_some());
        assert_eq!(stats.expect("stats should exist").render_count, 1);
    }

    #[test]
    fn test_multiple_renders() {
        let mut profiler = RenderProfiler::new();

        for _ in 0..3 {
            profiler.start_component("Widget");
            thread::sleep(Duration::from_millis(1));
            profiler.end_component();
        }

        let stats = profiler.get_stats("Widget").expect("Widget stats should exist");
        assert_eq!(stats.render_count, 3);
    }

    #[test]
    fn test_multiple_components() {
        let mut profiler = RenderProfiler::new();

        profiler.start_component("A");
        profiler.end_component();

        profiler.start_component("B");
        profiler.end_component();

        assert_eq!(profiler.component_count(), 2);
    }

    #[test]
    fn test_profile_closure() {
        let mut profiler = RenderProfiler::new();
        profiler.profile("Test", || {
            thread::sleep(Duration::from_millis(1));
        });

        assert_eq!(profiler.component_count(), 1);
    }

    #[test]
    fn test_get_stats() {
        let mut profiler = RenderProfiler::new();
        profiler.start_component("Widget");
        profiler.end_component();

        let stats = profiler.get_stats("Widget");
        assert!(stats.is_some());

        let missing = profiler.get_stats("Missing");
        assert!(missing.is_none());
    }

    #[test]
    fn test_slowest_component() {
        let mut profiler = RenderProfiler::new();

        profiler.start_component("Fast");
        profiler.end_component();

        profiler.start_component("Slow");
        thread::sleep(Duration::from_millis(10));
        profiler.end_component();

        let (name, _) = profiler.slowest_component().expect("should have slowest component");
        assert_eq!(name, "Slow");
    }

    #[test]
    fn test_total_time() {
        let mut profiler = RenderProfiler::new();

        profiler.start_component("A");
        thread::sleep(Duration::from_millis(5));
        profiler.end_component();

        profiler.start_component("B");
        thread::sleep(Duration::from_millis(5));
        profiler.end_component();

        let total = profiler.total_time_ms();
        assert!(total >= 10.0);
    }

    #[test]
    fn test_reset() {
        let mut profiler = RenderProfiler::new();
        profiler.start_component("Test");
        profiler.end_component();

        assert_eq!(profiler.component_count(), 1);

        profiler.reset();
        assert_eq!(profiler.component_count(), 0);
    }

    #[test]
    fn test_render_lines() {
        let mut profiler = RenderProfiler::new();
        profiler.start_component("Test");
        profiler.end_component();

        let lines = profiler.render_lines();
        assert!(!lines.is_empty());
    }

    #[test]
    fn test_component_color_green() {
        let profiler = RenderProfiler::new()
            .with_warning_threshold(10.0)
            .with_critical_threshold(20.0);

        let color = profiler.component_color(5.0);
        assert_eq!(color, Color::Green);
    }

    #[test]
    fn test_component_color_yellow() {
        let profiler = RenderProfiler::new()
            .with_warning_threshold(10.0)
            .with_critical_threshold(20.0);

        let color = profiler.component_color(15.0);
        assert_eq!(color, Color::Yellow);
    }

    #[test]
    fn test_component_color_red() {
        let profiler = RenderProfiler::new()
            .with_warning_threshold(10.0)
            .with_critical_threshold(20.0);

        let color = profiler.component_color(25.0);
        assert_eq!(color, Color::Red);
    }

    #[test]
    fn test_builder_pattern() {
        let profiler = RenderProfiler::new()
            .with_details(false)
            .with_sort_by_time(false)
            .with_warning_threshold(5.0)
            .with_critical_threshold(10.0);

        assert!(!profiler.show_details);
        assert!(!profiler.sort_by_time);
        assert_eq!(profiler.warning_threshold_ms, 5.0);
        assert_eq!(profiler.critical_threshold_ms, 10.0);
    }

    // Edge case tests for robustness

    #[test]
    fn test_slowest_component_empty() {
        // Empty profiler should return None, not panic
        let profiler = RenderProfiler::new();
        assert!(profiler.slowest_component().is_none());
    }

    #[test]
    fn test_slowest_component_single() {
        // Single component should be returned
        let mut profiler = RenderProfiler::new();
        profiler.start_component("Only");
        profiler.end_component();

        let (name, _) = profiler.slowest_component().expect("should have one component");
        assert_eq!(name, "Only");
    }

    #[test]
    fn test_slowest_component_multiple_same_time() {
        // When multiple components have same time, should not panic
        let mut profiler = RenderProfiler::new();

        profiler.start_component("A");
        profiler.end_component();

        profiler.start_component("B");
        profiler.end_component();

        // Should return one of them without panicking
        let result = profiler.slowest_component();
        assert!(result.is_some());
    }

    #[test]
    fn test_sort_by_time_multiple_components() {
        // Test sorting doesn't panic with multiple components
        let mut profiler = RenderProfiler::new().with_sort_by_time(true);

        profiler.start_component("Fast");
        profiler.end_component();

        profiler.start_component("Medium");
        thread::sleep(Duration::from_millis(2));
        profiler.end_component();

        profiler.start_component("Slow");
        thread::sleep(Duration::from_millis(5));
        profiler.end_component();

        let lines = profiler.render_lines();
        // Should render without panicking
        assert!(!lines.is_empty());
    }

    #[test]
    fn test_component_stats_zero_renders() {
        // Zero renders should return 0.0 average, not NaN
        let stats = ComponentStats::new();
        assert_eq!(stats.average_ms(), 0.0);
        assert!(!stats.average_ms().is_nan());
    }

    #[test]
    fn test_component_stats_single_render() {
        let mut stats = ComponentStats::new();
        stats.update(Duration::from_millis(10));

        assert_eq!(stats.render_count, 1);
        assert!(stats.average_ms() > 0.0);
        assert!(!stats.average_ms().is_nan());
    }

    #[test]
    fn test_component_stats_multiple_renders() {
        let mut stats = ComponentStats::new();
        stats.update(Duration::from_millis(5));
        stats.update(Duration::from_millis(15));
        stats.update(Duration::from_millis(10));

        assert_eq!(stats.render_count, 3);
        assert!((stats.average_ms() - 10.0).abs() < 1.0); // ~10ms average
        assert!(!stats.average_ms().is_nan());
    }

    #[test]
    fn test_component_stats_very_short_duration() {
        // Test with nanosecond-level durations
        let mut stats = ComponentStats::new();
        stats.update(Duration::from_nanos(100));

        assert_eq!(stats.render_count, 1);
        assert!(stats.average_ms() >= 0.0);
        assert!(!stats.average_ms().is_nan());
    }

    #[test]
    fn test_total_cmp_handles_edge_cases() {
        // Verify total_cmp is used correctly (doesn't panic on equal values)
        let mut profiler = RenderProfiler::new().with_sort_by_time(true);

        // Add many components with potentially equal times
        for i in 0..10 {
            profiler.start_component(&format!("Component{}", i));
            profiler.end_component();
        }

        // Should not panic during sorting
        let lines = profiler.render_lines();
        assert!(!lines.is_empty());

        // Should not panic finding slowest
        let _ = profiler.slowest_component();
    }
}
