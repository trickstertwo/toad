//! Performance monitoring and metrics tracking
//!
//! Provides FPS tracking, render time measurement, and performance logging
//! for debugging and optimization.
//!
//! # Examples
//!
//! ```
//! use toad::performance::PerformanceMonitor;
//!
//! let mut monitor = PerformanceMonitor::new();
//!
//! // Start a frame
//! monitor.start_frame();
//!
//! // ... do rendering work ...
//!
//! // End frame and record metrics
//! monitor.end_frame();
//!
//! // Get current FPS
//! let fps = monitor.fps();
//! assert!(fps >= 0.0);
//! ```

use std::collections::VecDeque;
use std::time::{Duration, Instant};

/// Performance metrics for a single frame
///
/// # Examples
///
/// ```
/// use toad::performance::FrameMetrics;
/// use std::time::Duration;
///
/// let metrics = FrameMetrics::new(Duration::from_millis(16));
/// assert_eq!(metrics.frame_time(), Duration::from_millis(16));
/// ```
#[derive(Debug, Clone, Copy)]
pub struct FrameMetrics {
    /// Time taken to render this frame
    frame_time: Duration,
    /// Timestamp when frame started
    timestamp: Instant,
}

impl FrameMetrics {
    /// Create new frame metrics
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::performance::FrameMetrics;
    /// use std::time::Duration;
    ///
    /// let metrics = FrameMetrics::new(Duration::from_millis(10));
    /// ```
    pub fn new(frame_time: Duration) -> Self {
        Self {
            frame_time,
            timestamp: Instant::now(),
        }
    }

    /// Get the frame time
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::performance::FrameMetrics;
    /// use std::time::Duration;
    ///
    /// let metrics = FrameMetrics::new(Duration::from_millis(16));
    /// assert_eq!(metrics.frame_time(), Duration::from_millis(16));
    /// ```
    pub fn frame_time(&self) -> Duration {
        self.frame_time
    }

    /// Get the frame timestamp
    pub fn timestamp(&self) -> Instant {
        self.timestamp
    }

    /// Get frame time in milliseconds
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::performance::FrameMetrics;
    /// use std::time::Duration;
    ///
    /// let metrics = FrameMetrics::new(Duration::from_millis(16));
    /// assert_eq!(metrics.frame_time_ms(), 16.0);
    /// ```
    pub fn frame_time_ms(&self) -> f64 {
        self.frame_time.as_secs_f64() * 1000.0
    }

    /// Calculate FPS from this frame time
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::performance::FrameMetrics;
    /// use std::time::Duration;
    ///
    /// let metrics = FrameMetrics::new(Duration::from_millis(16));
    /// let fps = metrics.fps();
    /// assert!(fps > 60.0 && fps < 63.0); // ~62.5 FPS
    /// ```
    pub fn fps(&self) -> f64 {
        if self.frame_time.as_secs_f64() > 0.0 {
            1.0 / self.frame_time.as_secs_f64()
        } else {
            0.0
        }
    }
}

/// Performance monitor for tracking FPS and render times
///
/// # Examples
///
/// ```
/// use toad::performance::PerformanceMonitor;
///
/// let mut monitor = PerformanceMonitor::new();
///
/// monitor.start_frame();
/// // ... render ...
/// monitor.end_frame();
///
/// assert!(monitor.fps() >= 0.0);
/// ```
#[derive(Debug)]
pub struct PerformanceMonitor {
    /// Recent frame metrics (limited history)
    frame_history: VecDeque<FrameMetrics>,

    /// Maximum number of frames to keep in history
    max_history: usize,

    /// Current frame start time
    current_frame_start: Option<Instant>,

    /// Total frames rendered
    total_frames: u64,

    /// Whether monitoring is enabled
    enabled: bool,
}

impl Default for PerformanceMonitor {
    fn default() -> Self {
        Self::new()
    }
}

impl PerformanceMonitor {
    /// Create a new performance monitor
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::performance::PerformanceMonitor;
    ///
    /// let monitor = PerformanceMonitor::new();
    /// assert_eq!(monitor.total_frames(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            frame_history: VecDeque::new(),
            max_history: 60, // Keep last 60 frames for ~1 second at 60 FPS
            current_frame_start: None,
            total_frames: 0,
            enabled: true,
        }
    }

    /// Create a monitor with custom history size
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::performance::PerformanceMonitor;
    ///
    /// let monitor = PerformanceMonitor::with_history(120);
    /// assert_eq!(monitor.max_history(), 120);
    /// ```
    pub fn with_history(max_history: usize) -> Self {
        Self {
            frame_history: VecDeque::new(),
            max_history,
            current_frame_start: None,
            total_frames: 0,
            enabled: true,
        }
    }

    /// Get maximum history size
    pub fn max_history(&self) -> usize {
        self.max_history
    }

    /// Check if monitoring is enabled
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::performance::PerformanceMonitor;
    ///
    /// let monitor = PerformanceMonitor::new();
    /// assert!(monitor.is_enabled());
    /// ```
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Enable or disable monitoring
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::performance::PerformanceMonitor;
    ///
    /// let mut monitor = PerformanceMonitor::new();
    /// monitor.set_enabled(false);
    /// assert!(!monitor.is_enabled());
    /// ```
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Start timing a new frame
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::performance::PerformanceMonitor;
    ///
    /// let mut monitor = PerformanceMonitor::new();
    /// monitor.start_frame();
    /// ```
    pub fn start_frame(&mut self) {
        if !self.enabled {
            return;
        }

        self.current_frame_start = Some(Instant::now());
    }

    /// End the current frame and record metrics
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::performance::PerformanceMonitor;
    ///
    /// let mut monitor = PerformanceMonitor::new();
    /// monitor.start_frame();
    /// monitor.end_frame();
    /// assert_eq!(monitor.total_frames(), 1);
    /// ```
    pub fn end_frame(&mut self) {
        if !self.enabled {
            return;
        }

        if let Some(start) = self.current_frame_start.take() {
            let frame_time = start.elapsed();
            let metrics = FrameMetrics::new(frame_time);

            self.frame_history.push_back(metrics);
            if self.frame_history.len() > self.max_history {
                self.frame_history.pop_front();
            }

            self.total_frames += 1;
        }
    }

    /// Get total number of frames rendered
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::performance::PerformanceMonitor;
    ///
    /// let mut monitor = PerformanceMonitor::new();
    /// assert_eq!(monitor.total_frames(), 0);
    ///
    /// monitor.start_frame();
    /// monitor.end_frame();
    /// assert_eq!(monitor.total_frames(), 1);
    /// ```
    pub fn total_frames(&self) -> u64 {
        self.total_frames
    }

    /// Get current FPS based on recent frame times
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::performance::PerformanceMonitor;
    ///
    /// let monitor = PerformanceMonitor::new();
    /// let fps = monitor.fps();
    /// assert!(fps >= 0.0);
    /// ```
    pub fn fps(&self) -> f64 {
        if self.frame_history.is_empty() {
            return 0.0;
        }

        let total_time: Duration = self.frame_history.iter().map(|m| m.frame_time()).sum();
        let avg_time = total_time.as_secs_f64() / self.frame_history.len() as f64;

        if avg_time > 0.0 {
            1.0 / avg_time
        } else {
            0.0
        }
    }

    /// Get average frame time in milliseconds
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::performance::PerformanceMonitor;
    ///
    /// let monitor = PerformanceMonitor::new();
    /// let avg = monitor.avg_frame_time_ms();
    /// assert!(avg >= 0.0);
    /// ```
    pub fn avg_frame_time_ms(&self) -> f64 {
        if self.frame_history.is_empty() {
            return 0.0;
        }

        let total_time: Duration = self.frame_history.iter().map(|m| m.frame_time()).sum();
        let avg_time = total_time.as_secs_f64() / self.frame_history.len() as f64;
        avg_time * 1000.0
    }

    /// Get minimum frame time in milliseconds
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::performance::PerformanceMonitor;
    ///
    /// let monitor = PerformanceMonitor::new();
    /// let min = monitor.min_frame_time_ms();
    /// assert!(min >= 0.0);
    /// ```
    pub fn min_frame_time_ms(&self) -> f64 {
        self.frame_history
            .iter()
            .map(|m| m.frame_time_ms())
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(0.0)
    }

    /// Get maximum frame time in milliseconds
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::performance::PerformanceMonitor;
    ///
    /// let monitor = PerformanceMonitor::new();
    /// let max = monitor.max_frame_time_ms();
    /// assert!(max >= 0.0);
    /// ```
    pub fn max_frame_time_ms(&self) -> f64 {
        self.frame_history
            .iter()
            .map(|m| m.frame_time_ms())
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(0.0)
    }

    /// Get frame history
    pub fn frame_history(&self) -> &VecDeque<FrameMetrics> {
        &self.frame_history
    }

    /// Clear all metrics
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::performance::PerformanceMonitor;
    ///
    /// let mut monitor = PerformanceMonitor::new();
    /// monitor.start_frame();
    /// monitor.end_frame();
    ///
    /// assert_eq!(monitor.total_frames(), 1);
    ///
    /// monitor.clear();
    /// assert_eq!(monitor.total_frames(), 0);
    /// ```
    pub fn clear(&mut self) {
        self.frame_history.clear();
        self.current_frame_start = None;
        self.total_frames = 0;
    }

    /// Get a summary report of performance metrics
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::performance::PerformanceMonitor;
    ///
    /// let monitor = PerformanceMonitor::new();
    /// let report = monitor.report();
    /// assert!(report.contains("FPS:"));
    /// ```
    pub fn report(&self) -> String {
        format!(
            "Performance: FPS: {:.1} | Avg: {:.2}ms | Min: {:.2}ms | Max: {:.2}ms | Frames: {}",
            self.fps(),
            self.avg_frame_time_ms(),
            self.min_frame_time_ms(),
            self.max_frame_time_ms(),
            self.total_frames
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_frame_metrics_creation() {
        let metrics = FrameMetrics::new(Duration::from_millis(16));
        assert_eq!(metrics.frame_time(), Duration::from_millis(16));
        assert_eq!(metrics.frame_time_ms(), 16.0);
    }

    #[test]
    fn test_frame_metrics_fps() {
        let metrics = FrameMetrics::new(Duration::from_millis(16));
        let fps = metrics.fps();
        assert!(fps > 60.0 && fps < 63.0); // ~62.5 FPS
    }

    #[test]
    fn test_monitor_creation() {
        let monitor = PerformanceMonitor::new();
        assert_eq!(monitor.total_frames(), 0);
        assert!(monitor.is_enabled());
        assert_eq!(monitor.max_history(), 60);
    }

    #[test]
    fn test_monitor_with_history() {
        let monitor = PerformanceMonitor::with_history(120);
        assert_eq!(monitor.max_history(), 120);
    }

    #[test]
    fn test_monitor_enable_disable() {
        let mut monitor = PerformanceMonitor::new();
        assert!(monitor.is_enabled());

        monitor.set_enabled(false);
        assert!(!monitor.is_enabled());

        monitor.set_enabled(true);
        assert!(monitor.is_enabled());
    }

    #[test]
    fn test_frame_timing() {
        let mut monitor = PerformanceMonitor::new();

        monitor.start_frame();
        thread::sleep(Duration::from_millis(10));
        monitor.end_frame();

        assert_eq!(monitor.total_frames(), 1);
        assert!(!monitor.frame_history().is_empty());
    }

    #[test]
    fn test_multiple_frames() {
        let mut monitor = PerformanceMonitor::new();

        for _ in 0..5 {
            monitor.start_frame();
            thread::sleep(Duration::from_millis(5));
            monitor.end_frame();
        }

        assert_eq!(monitor.total_frames(), 5);
        assert_eq!(monitor.frame_history().len(), 5);
    }

    #[test]
    fn test_history_limit() {
        let mut monitor = PerformanceMonitor::with_history(3);

        for _ in 0..5 {
            monitor.start_frame();
            monitor.end_frame();
        }

        assert_eq!(monitor.total_frames(), 5);
        assert_eq!(monitor.frame_history().len(), 3); // Limited to max_history
    }

    #[test]
    fn test_fps_calculation() {
        let mut monitor = PerformanceMonitor::new();

        monitor.start_frame();
        thread::sleep(Duration::from_millis(16)); // ~60 FPS
        monitor.end_frame();

        let fps = monitor.fps();
        assert!(fps > 0.0);
        assert!(fps < 100.0); // Reasonable upper bound
    }

    #[test]
    fn test_avg_frame_time() {
        let mut monitor = PerformanceMonitor::new();

        monitor.start_frame();
        thread::sleep(Duration::from_millis(10));
        monitor.end_frame();

        let avg = monitor.avg_frame_time_ms();
        assert!(avg >= 10.0); // At least 10ms (what we slept)
    }

    #[test]
    fn test_min_max_frame_time() {
        let mut monitor = PerformanceMonitor::new();

        // Fast frame
        monitor.start_frame();
        thread::sleep(Duration::from_millis(5));
        monitor.end_frame();

        // Slow frame
        monitor.start_frame();
        thread::sleep(Duration::from_millis(20));
        monitor.end_frame();

        let min = monitor.min_frame_time_ms();
        let max = monitor.max_frame_time_ms();

        assert!(min < max);
        assert!(min >= 5.0);
        assert!(max >= 20.0);
    }

    #[test]
    fn test_clear() {
        let mut monitor = PerformanceMonitor::new();

        monitor.start_frame();
        monitor.end_frame();

        assert_eq!(monitor.total_frames(), 1);

        monitor.clear();

        assert_eq!(monitor.total_frames(), 0);
        assert!(monitor.frame_history().is_empty());
    }

    #[test]
    fn test_disabled_monitor() {
        let mut monitor = PerformanceMonitor::new();
        monitor.set_enabled(false);

        monitor.start_frame();
        monitor.end_frame();

        // Should not record anything when disabled
        assert_eq!(monitor.total_frames(), 0);
    }

    #[test]
    fn test_report() {
        let mut monitor = PerformanceMonitor::new();

        monitor.start_frame();
        thread::sleep(Duration::from_millis(10));
        monitor.end_frame();

        let report = monitor.report();

        assert!(report.contains("FPS:"));
        assert!(report.contains("Avg:"));
        assert!(report.contains("Min:"));
        assert!(report.contains("Max:"));
        assert!(report.contains("Frames:"));
    }

    #[test]
    fn test_empty_monitor_metrics() {
        let monitor = PerformanceMonitor::new();

        assert_eq!(monitor.fps(), 0.0);
        assert_eq!(monitor.avg_frame_time_ms(), 0.0);
        assert_eq!(monitor.min_frame_time_ms(), 0.0);
        assert_eq!(monitor.max_frame_time_ms(), 0.0);
    }
}
