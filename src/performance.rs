/// Performance metrics tracking for the TUI
///
/// Tracks FPS, render times, and other performance metrics

use std::collections::VecDeque;
use std::time::{Duration, Instant};

/// Performance metrics tracker
#[derive(Debug)]
pub struct PerformanceMetrics {
    /// Recent frame times (for FPS calculation)
    frame_times: VecDeque<Duration>,
    /// Maximum number of frame times to track
    max_samples: usize,
    /// Last frame timestamp
    last_frame: Option<Instant>,
    /// Total frames rendered
    total_frames: u64,
    /// Start time of metrics tracking
    start_time: Instant,
}

impl PerformanceMetrics {
    /// Create a new performance metrics tracker
    pub fn new() -> Self {
        Self {
            frame_times: VecDeque::with_capacity(60),
            max_samples: 60, // Track last 60 frames
            last_frame: None,
            total_frames: 0,
            start_time: Instant::now(),
        }
    }

    /// Record a new frame
    pub fn record_frame(&mut self) {
        let now = Instant::now();

        if let Some(last) = self.last_frame {
            let frame_time = now.duration_since(last);
            self.frame_times.push_back(frame_time);

            // Keep only max_samples
            if self.frame_times.len() > self.max_samples {
                self.frame_times.pop_front();
            }
        }

        self.last_frame = Some(now);
        self.total_frames += 1;
    }

    /// Get current FPS (frames per second)
    pub fn fps(&self) -> f64 {
        if self.frame_times.is_empty() {
            return 0.0;
        }

        let total: Duration = self.frame_times.iter().sum();
        let avg_frame_time = total.as_secs_f64() / self.frame_times.len() as f64;

        if avg_frame_time > 0.0 {
            1.0 / avg_frame_time
        } else {
            0.0
        }
    }

    /// Get average frame time in milliseconds
    pub fn avg_frame_time_ms(&self) -> f64 {
        if self.frame_times.is_empty() {
            return 0.0;
        }

        let total: Duration = self.frame_times.iter().sum();
        (total.as_secs_f64() / self.frame_times.len() as f64) * 1000.0
    }

    /// Get minimum frame time in milliseconds
    pub fn min_frame_time_ms(&self) -> f64 {
        self.frame_times
            .iter()
            .min()
            .map(|d| d.as_secs_f64() * 1000.0)
            .unwrap_or(0.0)
    }

    /// Get maximum frame time in milliseconds
    pub fn max_frame_time_ms(&self) -> f64 {
        self.frame_times
            .iter()
            .max()
            .map(|d| d.as_secs_f64() * 1000.0)
            .unwrap_or(0.0)
    }

    /// Get total frames rendered
    pub fn total_frames(&self) -> u64 {
        self.total_frames
    }

    /// Get uptime in seconds
    pub fn uptime_secs(&self) -> f64 {
        self.start_time.elapsed().as_secs_f64()
    }

    /// Get average FPS over entire session
    pub fn avg_fps(&self) -> f64 {
        let uptime = self.uptime_secs();
        if uptime > 0.0 {
            self.total_frames as f64 / uptime
        } else {
            0.0
        }
    }

    /// Reset all metrics
    pub fn reset(&mut self) {
        self.frame_times.clear();
        self.last_frame = None;
        self.total_frames = 0;
        self.start_time = Instant::now();
    }

    /// Get a summary string of metrics
    pub fn summary(&self) -> String {
        format!(
            "FPS: {:.1} | Avg: {:.2}ms | Min: {:.2}ms | Max: {:.2}ms | Frames: {} | Uptime: {:.1}s",
            self.fps(),
            self.avg_frame_time_ms(),
            self.min_frame_time_ms(),
            self.max_frame_time_ms(),
            self.total_frames,
            self.uptime_secs()
        )
    }
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_metrics_creation() {
        let metrics = PerformanceMetrics::new();
        assert_eq!(metrics.total_frames(), 0);
        assert_eq!(metrics.fps(), 0.0);
    }

    #[test]
    fn test_frame_recording() {
        let mut metrics = PerformanceMetrics::new();

        // Record some frames with small delays
        for _ in 0..10 {
            metrics.record_frame();
            thread::sleep(Duration::from_millis(10));
        }

        assert_eq!(metrics.total_frames(), 10);
        assert!(metrics.fps() > 0.0);
        assert!(metrics.avg_frame_time_ms() > 0.0);
    }

    #[test]
    fn test_metrics_reset() {
        let mut metrics = PerformanceMetrics::new();

        metrics.record_frame();
        metrics.record_frame();

        assert_eq!(metrics.total_frames(), 2);

        metrics.reset();

        assert_eq!(metrics.total_frames(), 0);
        assert_eq!(metrics.fps(), 0.0);
    }

    #[test]
    fn test_max_samples_limit() {
        let mut metrics = PerformanceMetrics::new();

        // Record more frames than max_samples
        for _ in 0..100 {
            metrics.record_frame();
        }

        // Should only keep max_samples frame times
        assert_eq!(metrics.frame_times.len(), metrics.max_samples);
        assert_eq!(metrics.total_frames(), 100);
    }

    #[test]
    fn test_summary() {
        let mut metrics = PerformanceMetrics::new();
        metrics.record_frame();

        let summary = metrics.summary();
        assert!(summary.contains("FPS:"));
        assert!(summary.contains("Avg:"));
        assert!(summary.contains("Frames:"));
    }
}
