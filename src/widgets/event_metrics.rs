//! Event metrics for tracking input latency and event rates
//!
//! Monitors keyboard, mouse, and other events to measure responsiveness
//! and identify performance bottlenecks.
//!
//! # Examples
//!
//! ```
//! use toad::widgets::EventMetrics;
//! use std::time::Duration;
//!
//! let mut metrics = EventMetrics::new();
//! metrics.record_event("KeyPress", Duration::from_micros(500));
//!
//! assert_eq!(metrics.total_events(), 1);
//! ```

use std::collections::VecDeque;
use std::time::{Duration, Instant};

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::Span,
    widgets::{Block, Paragraph, Widget},
};

/// Event processing record
#[derive(Debug, Clone)]
struct EventRecord {
    /// Event type (KeyPress, Mouse, Resize, etc.)
    event_type: String,
    /// Time when event was received
    timestamp: Instant,
    /// Processing latency
    latency: Duration,
}

/// Event metrics tracker
///
/// Tracks event processing latency and event rates for performance monitoring.
///
/// # Examples
///
/// ```
/// use toad::widgets::EventMetrics;
/// use std::time::Duration;
///
/// let mut metrics = EventMetrics::new();
/// metrics.record_event("KeyPress", Duration::from_millis(1));
///
/// assert!(metrics.average_latency_ms() >= 0.0);
/// ```
#[derive(Debug, Clone)]
pub struct EventMetrics {
    /// Recent event records
    events: VecDeque<EventRecord>,
    /// Maximum events to keep
    max_events: usize,
    /// Total events processed
    total_events: usize,
    /// Display settings
    show_details: bool,
    /// Warning threshold in ms
    warning_threshold_ms: f64,
    /// Critical threshold in ms
    critical_threshold_ms: f64,
}

impl EventMetrics {
    /// Create a new event metrics tracker
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::EventMetrics;
    ///
    /// let metrics = EventMetrics::new();
    /// ```
    pub fn new() -> Self {
        Self::with_capacity(100)
    }

    /// Create with specific event capacity
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::EventMetrics;
    ///
    /// let metrics = EventMetrics::with_capacity(200);
    /// ```
    pub fn with_capacity(max_events: usize) -> Self {
        Self {
            events: VecDeque::with_capacity(max_events),
            max_events,
            total_events: 0,
            show_details: false,
            warning_threshold_ms: 16.0,  // ~60 FPS
            critical_threshold_ms: 33.0, // ~30 FPS
        }
    }

    /// Show detailed metrics (event types, rates)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::EventMetrics;
    ///
    /// let metrics = EventMetrics::new().with_details(true);
    /// ```
    pub fn with_details(mut self, show: bool) -> Self {
        self.show_details = show;
        self
    }

    /// Set latency warning threshold in milliseconds
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::EventMetrics;
    ///
    /// let metrics = EventMetrics::new().with_warning_threshold(10.0);
    /// ```
    pub fn with_warning_threshold(mut self, ms: f64) -> Self {
        self.warning_threshold_ms = ms;
        self
    }

    /// Set latency critical threshold in milliseconds
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::EventMetrics;
    ///
    /// let metrics = EventMetrics::new().with_critical_threshold(20.0);
    /// ```
    pub fn with_critical_threshold(mut self, ms: f64) -> Self {
        self.critical_threshold_ms = ms;
        self
    }

    /// Record an event with its processing latency
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::EventMetrics;
    /// use std::time::Duration;
    ///
    /// let mut metrics = EventMetrics::new();
    /// metrics.record_event("KeyPress", Duration::from_micros(500));
    /// assert_eq!(metrics.total_events(), 1);
    /// ```
    pub fn record_event(&mut self, event_type: &str, latency: Duration) {
        let record = EventRecord {
            event_type: event_type.to_string(),
            timestamp: Instant::now(),
            latency,
        };

        self.events.push_back(record);
        self.total_events += 1;

        // Remove old events if over capacity
        if self.events.len() > self.max_events {
            self.events.pop_front();
        }
    }

    /// Get total events processed
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::EventMetrics;
    /// use std::time::Duration;
    ///
    /// let mut metrics = EventMetrics::new();
    /// metrics.record_event("Key", Duration::from_millis(1));
    /// assert_eq!(metrics.total_events(), 1);
    /// ```
    pub fn total_events(&self) -> usize {
        self.total_events
    }

    /// Get number of recent events in buffer
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::EventMetrics;
    /// use std::time::Duration;
    ///
    /// let mut metrics = EventMetrics::new();
    /// metrics.record_event("Key", Duration::from_millis(1));
    /// assert_eq!(metrics.recent_events(), 1);
    /// ```
    pub fn recent_events(&self) -> usize {
        self.events.len()
    }

    /// Get average latency in milliseconds
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::EventMetrics;
    /// use std::time::Duration;
    ///
    /// let mut metrics = EventMetrics::new();
    /// metrics.record_event("Key", Duration::from_millis(2));
    /// assert!(metrics.average_latency_ms() >= 0.0);
    /// ```
    pub fn average_latency_ms(&self) -> f64 {
        if self.events.is_empty() {
            return 0.0;
        }

        let total: Duration = self.events.iter().map(|e| e.latency).sum();
        (total.as_secs_f64() / self.events.len() as f64) * 1000.0
    }

    /// Get minimum latency in milliseconds
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::EventMetrics;
    /// use std::time::Duration;
    ///
    /// let mut metrics = EventMetrics::new();
    /// metrics.record_event("Key", Duration::from_millis(1));
    /// assert!(metrics.min_latency_ms() >= 0.0);
    /// ```
    pub fn min_latency_ms(&self) -> f64 {
        self.events
            .iter()
            .map(|e| e.latency)
            .min()
            .map(|d| d.as_secs_f64() * 1000.0)
            .unwrap_or(0.0)
    }

    /// Get maximum latency in milliseconds
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::EventMetrics;
    /// use std::time::Duration;
    ///
    /// let mut metrics = EventMetrics::new();
    /// metrics.record_event("Key", Duration::from_millis(5));
    /// assert!(metrics.max_latency_ms() >= 0.0);
    /// ```
    pub fn max_latency_ms(&self) -> f64 {
        self.events
            .iter()
            .map(|e| e.latency)
            .max()
            .map(|d| d.as_secs_f64() * 1000.0)
            .unwrap_or(0.0)
    }

    /// Get event rate in events per second
    ///
    /// Based on recent events within the buffer time window.
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::EventMetrics;
    /// use std::time::Duration;
    ///
    /// let mut metrics = EventMetrics::new();
    /// metrics.record_event("Key", Duration::from_micros(100));
    /// assert!(metrics.event_rate() >= 0.0);
    /// ```
    pub fn event_rate(&self) -> f64 {
        if self.events.len() < 2 {
            return 0.0;
        }

        let first = &self.events[0];
        let last = &self.events[self.events.len() - 1];
        let duration = last.timestamp.duration_since(first.timestamp);

        if duration.as_secs_f64() > 0.0 {
            self.events.len() as f64 / duration.as_secs_f64()
        } else {
            0.0
        }
    }

    /// Get count of events by type
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::EventMetrics;
    /// use std::time::Duration;
    ///
    /// let mut metrics = EventMetrics::new();
    /// metrics.record_event("KeyPress", Duration::from_micros(100));
    /// metrics.record_event("KeyPress", Duration::from_micros(100));
    /// metrics.record_event("Mouse", Duration::from_micros(100));
    ///
    /// let count = metrics.event_type_count("KeyPress");
    /// assert_eq!(count, 2);
    /// ```
    pub fn event_type_count(&self, event_type: &str) -> usize {
        self.events
            .iter()
            .filter(|e| e.event_type == event_type)
            .count()
    }

    /// Reset all metrics
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::EventMetrics;
    /// use std::time::Duration;
    ///
    /// let mut metrics = EventMetrics::new();
    /// metrics.record_event("Key", Duration::from_millis(1));
    /// metrics.reset();
    /// assert_eq!(metrics.total_events(), 0);
    /// ```
    pub fn reset(&mut self) {
        self.events.clear();
        self.total_events = 0;
    }

    /// Get latency color based on thresholds
    fn latency_color(&self) -> Color {
        let avg = self.average_latency_ms();
        if avg >= self.critical_threshold_ms {
            Color::Red
        } else if avg >= self.warning_threshold_ms {
            Color::Yellow
        } else {
            Color::Green
        }
    }

    /// Render as a compact string
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::EventMetrics;
    /// use std::time::Duration;
    ///
    /// let mut metrics = EventMetrics::new();
    /// metrics.record_event("Key", Duration::from_millis(1));
    /// let display = metrics.render_string();
    /// assert!(display.contains("LAG"));
    /// ```
    pub fn render_string(&self) -> String {
        if self.show_details {
            format!(
                "LAG: {:.2}ms (min: {:.2}, max: {:.2}) | {:.1} ev/s | {} total",
                self.average_latency_ms(),
                self.min_latency_ms(),
                self.max_latency_ms(),
                self.event_rate(),
                self.total_events
            )
        } else {
            format!(
                "LAG: {:.2}ms | {:.1} ev/s",
                self.average_latency_ms(),
                self.event_rate()
            )
        }
    }
}

impl Default for EventMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for &EventMetrics {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let text = self.render_string();
        let color = self.latency_color();

        let paragraph = Paragraph::new(Span::styled(text, Style::default().fg(color)))
            .block(Block::default());

        paragraph.render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_new() {
        let metrics = EventMetrics::new();
        assert_eq!(metrics.total_events(), 0);
        assert_eq!(metrics.recent_events(), 0);
    }

    #[test]
    fn test_with_capacity() {
        let metrics = EventMetrics::with_capacity(50);
        assert_eq!(metrics.max_events, 50);
    }

    #[test]
    fn test_with_details() {
        let metrics = EventMetrics::new().with_details(true);
        assert!(metrics.show_details);
    }

    #[test]
    fn test_with_thresholds() {
        let metrics = EventMetrics::new()
            .with_warning_threshold(10.0)
            .with_critical_threshold(20.0);

        assert_eq!(metrics.warning_threshold_ms, 10.0);
        assert_eq!(metrics.critical_threshold_ms, 20.0);
    }

    #[test]
    fn test_record_event() {
        let mut metrics = EventMetrics::new();
        metrics.record_event("KeyPress", Duration::from_micros(500));

        assert_eq!(metrics.total_events(), 1);
        assert_eq!(metrics.recent_events(), 1);
    }

    #[test]
    fn test_multiple_events() {
        let mut metrics = EventMetrics::new();

        for i in 0..10 {
            metrics.record_event("Key", Duration::from_micros(i * 100));
            thread::sleep(Duration::from_millis(1));
        }

        assert_eq!(metrics.total_events(), 10);
        assert_eq!(metrics.recent_events(), 10);
    }

    #[test]
    fn test_max_capacity() {
        let mut metrics = EventMetrics::with_capacity(5);

        for i in 0..10 {
            metrics.record_event("Key", Duration::from_micros(i * 100));
        }

        assert_eq!(metrics.total_events(), 10);
        assert_eq!(metrics.recent_events(), 5); // Only last 5 kept
    }

    #[test]
    fn test_average_latency() {
        let mut metrics = EventMetrics::new();
        metrics.record_event("Key1", Duration::from_millis(1));
        metrics.record_event("Key2", Duration::from_millis(3));

        let avg = metrics.average_latency_ms();
        assert!((avg - 2.0).abs() < 0.1); // Should be ~2ms
    }

    #[test]
    fn test_min_max_latency() {
        let mut metrics = EventMetrics::new();
        metrics.record_event("Key1", Duration::from_millis(1));
        metrics.record_event("Key2", Duration::from_millis(5));
        metrics.record_event("Key3", Duration::from_millis(3));

        assert!((metrics.min_latency_ms() - 1.0).abs() < 0.1);
        assert!((metrics.max_latency_ms() - 5.0).abs() < 0.1);
    }

    #[test]
    fn test_event_rate() {
        let mut metrics = EventMetrics::new();

        for _ in 0..5 {
            metrics.record_event("Key", Duration::from_micros(100));
            thread::sleep(Duration::from_millis(10));
        }

        let rate = metrics.event_rate();
        assert!(rate > 0.0);
        assert!(rate < 200.0); // Sanity check
    }

    #[test]
    fn test_reset() {
        let mut metrics = EventMetrics::new();
        metrics.record_event("Key", Duration::from_millis(1));

        assert_eq!(metrics.total_events(), 1);

        metrics.reset();
        assert_eq!(metrics.total_events(), 0);
        assert_eq!(metrics.recent_events(), 0);
    }

    #[test]
    fn test_render_string_simple() {
        let mut metrics = EventMetrics::new();
        metrics.record_event("Key", Duration::from_millis(1));

        let display = metrics.render_string();
        assert!(display.contains("LAG"));
        assert!(!display.contains("min"));
    }

    #[test]
    fn test_render_string_detailed() {
        let mut metrics = EventMetrics::new().with_details(true);
        metrics.record_event("Key", Duration::from_millis(1));

        let display = metrics.render_string();
        assert!(display.contains("LAG"));
        assert!(display.contains("min"));
        assert!(display.contains("max"));
        assert!(display.contains("total"));
    }

    #[test]
    fn test_latency_color_green() {
        let mut metrics = EventMetrics::new()
            .with_warning_threshold(50.0)
            .with_critical_threshold(100.0);

        metrics.record_event("Key", Duration::from_millis(1));

        let color = metrics.latency_color();
        assert_eq!(color, Color::Green);
    }

    #[test]
    fn test_empty_metrics() {
        let metrics = EventMetrics::new();
        assert_eq!(metrics.average_latency_ms(), 0.0);
        assert_eq!(metrics.min_latency_ms(), 0.0);
        assert_eq!(metrics.max_latency_ms(), 0.0);
        assert_eq!(metrics.event_rate(), 0.0);
    }

    #[test]
    fn test_builder_pattern() {
        let metrics = EventMetrics::with_capacity(200)
            .with_details(true)
            .with_warning_threshold(10.0)
            .with_critical_threshold(20.0);

        assert_eq!(metrics.max_events, 200);
        assert!(metrics.show_details);
        assert_eq!(metrics.warning_threshold_ms, 10.0);
        assert_eq!(metrics.critical_threshold_ms, 20.0);
    }
}
