//! FPS (Frames Per Second) counter for performance monitoring
//!
//! Tracks rendering performance by measuring frame times and calculating
//! FPS statistics with rolling averages.
//!
//! # Examples
//!
//! ```
//! use toad::widgets::FpsCounter;
//!
//! let mut fps = FpsCounter::new();
//!
//! // Mark frame completion
//! fps.tick();
//!
//! // Get current FPS
//! let current = fps.current_fps();
//! assert!(current >= 0.0);
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

/// FPS counter with rolling average
///
/// Tracks frame rendering times and calculates FPS statistics.
/// Uses a rolling window of samples for smooth, accurate measurements.
///
/// # Examples
///
/// ```
/// use toad::widgets::FpsCounter;
///
/// let mut fps = FpsCounter::new();
/// fps.tick();
///
/// let current = fps.current_fps();
/// let average = fps.average_fps();
/// let min = fps.min_fps();
/// let max = fps.max_fps();
/// ```
#[derive(Debug, Clone)]
pub struct FpsCounter {
    /// Frame time samples (circular buffer)
    frame_times: VecDeque<Duration>,
    /// Maximum number of samples to keep
    max_samples: usize,
    /// Timestamp of last frame
    last_frame: Instant,
    /// Current FPS (most recent)
    current_fps: f64,
    /// Display style
    show_stats: bool,
    /// Color thresholds
    warning_threshold: f64,
    critical_threshold: f64,
}

impl FpsCounter {
    /// Create a new FPS counter
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::FpsCounter;
    ///
    /// let fps = FpsCounter::new();
    /// ```
    pub fn new() -> Self {
        Self::with_capacity(60)
    }

    /// Create FPS counter with specific sample capacity
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::FpsCounter;
    ///
    /// let fps = FpsCounter::with_capacity(120);
    /// ```
    pub fn with_capacity(max_samples: usize) -> Self {
        Self {
            frame_times: VecDeque::with_capacity(max_samples),
            max_samples,
            last_frame: Instant::now(),
            current_fps: 0.0,
            show_stats: false,
            warning_threshold: 30.0,
            critical_threshold: 15.0,
        }
    }

    /// Show detailed statistics (min/max/avg)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::FpsCounter;
    ///
    /// let fps = FpsCounter::new().with_stats(true);
    /// ```
    pub fn with_stats(mut self, show: bool) -> Self {
        self.show_stats = show;
        self
    }

    /// Set FPS warning threshold (yellow below this)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::FpsCounter;
    ///
    /// let fps = FpsCounter::new().with_warning_threshold(25.0);
    /// ```
    pub fn with_warning_threshold(mut self, threshold: f64) -> Self {
        self.warning_threshold = threshold;
        self
    }

    /// Set FPS critical threshold (red below this)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::FpsCounter;
    ///
    /// let fps = FpsCounter::new().with_critical_threshold(10.0);
    /// ```
    pub fn with_critical_threshold(mut self, threshold: f64) -> Self {
        self.critical_threshold = threshold;
        self
    }

    /// Mark completion of a frame
    ///
    /// Call this once per render cycle to update FPS metrics.
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::FpsCounter;
    ///
    /// let mut fps = FpsCounter::new();
    /// fps.tick(); // Frame 1
    /// fps.tick(); // Frame 2
    /// ```
    pub fn tick(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_frame);

        // Add new sample
        self.frame_times.push_back(elapsed);

        // Remove old samples if over capacity
        if self.frame_times.len() > self.max_samples {
            self.frame_times.pop_front();
        }

        // Calculate current FPS
        if elapsed.as_secs_f64() > 0.0 {
            self.current_fps = 1.0 / elapsed.as_secs_f64();
        }

        self.last_frame = now;
    }

    /// Get current FPS (most recent frame)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::FpsCounter;
    ///
    /// let mut fps = FpsCounter::new();
    /// fps.tick();
    /// assert!(fps.current_fps() >= 0.0);
    /// ```
    pub fn current_fps(&self) -> f64 {
        self.current_fps
    }

    /// Get average FPS across all samples
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::FpsCounter;
    ///
    /// let mut fps = FpsCounter::new();
    /// fps.tick();
    /// assert!(fps.average_fps() >= 0.0);
    /// ```
    pub fn average_fps(&self) -> f64 {
        if self.frame_times.is_empty() {
            return 0.0;
        }

        let total: Duration = self.frame_times.iter().sum();
        let avg_time = total.as_secs_f64() / self.frame_times.len() as f64;

        if avg_time > 0.0 { 1.0 / avg_time } else { 0.0 }
    }

    /// Get minimum FPS (worst frame)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::FpsCounter;
    ///
    /// let mut fps = FpsCounter::new();
    /// fps.tick();
    /// assert!(fps.min_fps() >= 0.0);
    /// ```
    pub fn min_fps(&self) -> f64 {
        self.frame_times
            .iter()
            .max()
            .map(|max_time| {
                let secs = max_time.as_secs_f64();
                if secs > 0.0 { 1.0 / secs } else { 0.0 }
            })
            .unwrap_or(0.0)
    }

    /// Get maximum FPS (best frame)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::FpsCounter;
    ///
    /// let mut fps = FpsCounter::new();
    /// fps.tick();
    /// assert!(fps.max_fps() >= 0.0);
    /// ```
    pub fn max_fps(&self) -> f64 {
        self.frame_times
            .iter()
            .min()
            .map(|min_time| {
                let secs = min_time.as_secs_f64();
                if secs > 0.0 { 1.0 / secs } else { 0.0 }
            })
            .unwrap_or(0.0)
    }

    /// Get average frame time in milliseconds
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::FpsCounter;
    ///
    /// let mut fps = FpsCounter::new();
    /// fps.tick();
    /// assert!(fps.average_frame_time_ms() >= 0.0);
    /// ```
    pub fn average_frame_time_ms(&self) -> f64 {
        if self.frame_times.is_empty() {
            return 0.0;
        }

        let total: Duration = self.frame_times.iter().sum();
        (total.as_secs_f64() / self.frame_times.len() as f64) * 1000.0
    }

    /// Get number of samples collected
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::FpsCounter;
    ///
    /// let mut fps = FpsCounter::new();
    /// assert_eq!(fps.sample_count(), 0);
    /// fps.tick();
    /// assert_eq!(fps.sample_count(), 1);
    /// ```
    pub fn sample_count(&self) -> usize {
        self.frame_times.len()
    }

    /// Reset all metrics
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::FpsCounter;
    ///
    /// let mut fps = FpsCounter::new();
    /// fps.tick();
    /// fps.reset();
    /// assert_eq!(fps.sample_count(), 0);
    /// ```
    pub fn reset(&mut self) {
        self.frame_times.clear();
        self.current_fps = 0.0;
        self.last_frame = Instant::now();
    }

    /// Get color for FPS display based on thresholds
    fn fps_color(&self) -> Color {
        let fps = self.current_fps();
        if fps < self.critical_threshold {
            Color::Red
        } else if fps < self.warning_threshold {
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
    /// use toad::widgets::FpsCounter;
    ///
    /// let mut fps = FpsCounter::new();
    /// fps.tick();
    /// let display = fps.render_string();
    /// assert!(display.contains("FPS"));
    /// ```
    pub fn render_string(&self) -> String {
        if self.show_stats {
            format!(
                "FPS: {:.1} (avg: {:.1}, min: {:.1}, max: {:.1}) | {:.2}ms",
                self.current_fps(),
                self.average_fps(),
                self.min_fps(),
                self.max_fps(),
                self.average_frame_time_ms()
            )
        } else {
            format!("FPS: {:.1}", self.current_fps())
        }
    }
}

impl Default for FpsCounter {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for &FpsCounter {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let text = self.render_string();
        let color = self.fps_color();

        let paragraph =
            Paragraph::new(Span::styled(text, Style::default().fg(color))).block(Block::default());

        paragraph.render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_new() {
        let fps = FpsCounter::new();
        assert_eq!(fps.sample_count(), 0);
        assert_eq!(fps.current_fps(), 0.0);
    }

    #[test]
    fn test_with_capacity() {
        let fps = FpsCounter::with_capacity(120);
        assert_eq!(fps.max_samples, 120);
    }

    #[test]
    fn test_with_stats() {
        let fps = FpsCounter::new().with_stats(true);
        assert!(fps.show_stats);
    }

    #[test]
    fn test_with_thresholds() {
        let fps = FpsCounter::new()
            .with_warning_threshold(25.0)
            .with_critical_threshold(10.0);
        assert_eq!(fps.warning_threshold, 25.0);
        assert_eq!(fps.critical_threshold, 10.0);
    }

    #[test]
    fn test_tick() {
        let mut fps = FpsCounter::new();
        fps.tick();
        assert_eq!(fps.sample_count(), 1);

        fps.tick();
        assert_eq!(fps.sample_count(), 2);
    }

    #[test]
    fn test_current_fps() {
        let mut fps = FpsCounter::new();
        thread::sleep(Duration::from_millis(10));
        fps.tick();

        let current = fps.current_fps();
        assert!(current > 0.0);
        assert!(current < 1000.0); // Sanity check
    }

    #[test]
    fn test_average_fps() {
        let mut fps = FpsCounter::new();

        for _ in 0..5 {
            thread::sleep(Duration::from_millis(10));
            fps.tick();
        }

        let avg = fps.average_fps();
        assert!(avg > 0.0);
        assert!(avg < 1000.0);
    }

    #[test]
    fn test_min_max_fps() {
        let mut fps = FpsCounter::new();

        // Fast frame
        thread::sleep(Duration::from_millis(5));
        fps.tick();

        // Slow frame
        thread::sleep(Duration::from_millis(20));
        fps.tick();

        let min = fps.min_fps();
        let max = fps.max_fps();

        assert!(min > 0.0);
        assert!(max > min);
    }

    #[test]
    fn test_average_frame_time_ms() {
        let mut fps = FpsCounter::new();

        thread::sleep(Duration::from_millis(10));
        fps.tick();

        let frame_time = fps.average_frame_time_ms();
        assert!(frame_time >= 10.0);
    }

    #[test]
    fn test_reset() {
        let mut fps = FpsCounter::new();
        fps.tick();
        fps.tick();

        assert_eq!(fps.sample_count(), 2);

        fps.reset();
        assert_eq!(fps.sample_count(), 0);
        assert_eq!(fps.current_fps(), 0.0);
    }

    #[test]
    fn test_max_samples() {
        let mut fps = FpsCounter::with_capacity(3);

        for _ in 0..5 {
            thread::sleep(Duration::from_millis(1));
            fps.tick();
        }

        // Should only keep last 3 samples
        assert_eq!(fps.sample_count(), 3);
    }

    #[test]
    fn test_render_string_simple() {
        let mut fps = FpsCounter::new();
        fps.tick();

        let display = fps.render_string();
        assert!(display.contains("FPS"));
        assert!(!display.contains("avg"));
    }

    #[test]
    fn test_render_string_with_stats() {
        let mut fps = FpsCounter::new().with_stats(true);
        fps.tick();

        let display = fps.render_string();
        assert!(display.contains("FPS"));
        assert!(display.contains("avg"));
        assert!(display.contains("min"));
        assert!(display.contains("max"));
        assert!(display.contains("ms"));
    }

    #[test]
    fn test_fps_color_green() {
        let mut fps = FpsCounter::new()
            .with_warning_threshold(30.0)
            .with_critical_threshold(15.0);

        // Simulate high FPS
        thread::sleep(Duration::from_millis(1));
        fps.tick();

        let color = fps.fps_color();
        assert_eq!(color, Color::Green);
    }

    #[test]
    fn test_zero_samples() {
        let fps = FpsCounter::new();
        assert_eq!(fps.average_fps(), 0.0);
        assert_eq!(fps.min_fps(), 0.0);
        assert_eq!(fps.max_fps(), 0.0);
        assert_eq!(fps.average_frame_time_ms(), 0.0);
    }

    #[test]
    fn test_builder_pattern() {
        let fps = FpsCounter::with_capacity(100)
            .with_stats(true)
            .with_warning_threshold(25.0)
            .with_critical_threshold(10.0);

        assert_eq!(fps.max_samples, 100);
        assert!(fps.show_stats);
        assert_eq!(fps.warning_threshold, 25.0);
        assert_eq!(fps.critical_threshold, 10.0);
    }

    #[test]
    fn test_multiple_ticks() {
        let mut fps = FpsCounter::new();

        for _ in 0..10 {
            thread::sleep(Duration::from_millis(5));
            fps.tick();
        }

        assert_eq!(fps.sample_count(), 10);
        assert!(fps.current_fps() > 0.0);
        assert!(fps.average_fps() > 0.0);
    }

    // ============ Default Trait Test ============

    #[test]
    fn test_fps_counter_default() {
        let fps = FpsCounter::default();
        assert_eq!(fps.sample_count(), 0);
        assert_eq!(fps.current_fps(), 0.0);
        assert_eq!(fps.max_samples, 60);
        assert!(!fps.show_stats);
    }

    // ============ Clone Trait Test ============

    #[test]
    fn test_fps_counter_clone() {
        let mut fps1 = FpsCounter::new().with_stats(true);
        thread::sleep(Duration::from_millis(10));
        fps1.tick();

        let fps2 = fps1.clone();
        assert_eq!(fps1.sample_count(), fps2.sample_count());
        assert_eq!(fps1.current_fps(), fps2.current_fps());
        assert_eq!(fps1.show_stats, fps2.show_stats);
    }

    // ============ Debug Trait Test ============

    #[test]
    fn test_fps_counter_debug() {
        let fps = FpsCounter::new();
        let debug_str = format!("{:?}", fps);
        assert!(debug_str.contains("FpsCounter"));
    }

    // ============ Extreme Stress Tests ============

    #[test]
    fn test_fps_10k_ticks() {
        let mut fps = FpsCounter::with_capacity(10000);

        for _ in 0..10000 {
            fps.tick();
        }

        assert_eq!(fps.sample_count(), 10000);
        assert!(fps.current_fps() > 0.0);
        assert!(fps.average_fps() > 0.0);
    }

    #[test]
    fn test_fps_large_capacity_10k() {
        let fps = FpsCounter::with_capacity(10000);
        assert_eq!(fps.max_samples, 10000);
        assert_eq!(fps.sample_count(), 0);
    }

    #[test]
    fn test_fps_rapid_tick_reset_cycles_1000() {
        let mut fps = FpsCounter::new();

        for _ in 0..1000 {
            fps.tick();
            fps.reset();
        }

        assert_eq!(fps.sample_count(), 0);
        assert_eq!(fps.current_fps(), 0.0);
    }

    #[test]
    fn test_fps_extreme_threshold_values() {
        let fps = FpsCounter::new()
            .with_warning_threshold(1000.0)
            .with_critical_threshold(0.001);

        assert_eq!(fps.warning_threshold, 1000.0);
        assert_eq!(fps.critical_threshold, 0.001);
    }

    #[test]
    fn test_fps_alternating_fast_slow_frames_1000() {
        let mut fps = FpsCounter::with_capacity(2000);

        for i in 0..1000 {
            if i % 2 == 0 {
                thread::sleep(Duration::from_micros(100));
            } else {
                thread::sleep(Duration::from_micros(500));
            }
            fps.tick();
        }

        assert_eq!(fps.sample_count(), 1000);
        assert!(fps.min_fps() > 0.0);
        assert!(fps.max_fps() > fps.min_fps());
    }

    // ============ FPS Color Edge Cases ============

    #[test]
    fn test_fps_color_critical_red() {
        let mut fps = FpsCounter::new()
            .with_warning_threshold(30.0)
            .with_critical_threshold(15.0);

        // Force very low FPS by setting current_fps directly via tick with long delay
        thread::sleep(Duration::from_millis(100)); // ~10 FPS
        fps.tick();

        let color = fps.fps_color();
        assert_eq!(color, Color::Red);
    }

    #[test]
    fn test_fps_color_warning_yellow() {
        let mut fps = FpsCounter::new()
            .with_warning_threshold(100.0)
            .with_critical_threshold(50.0);

        thread::sleep(Duration::from_millis(15)); // ~66 FPS (between 50 and 100)
        fps.tick();

        let color = fps.fps_color();
        assert_eq!(color, Color::Yellow);
    }

    #[test]
    fn test_fps_color_boundary_exactly_at_warning() {
        let mut fps = FpsCounter::new()
            .with_warning_threshold(30.0)
            .with_critical_threshold(15.0);

        // Manually set current_fps to exactly warning threshold
        fps.current_fps = 30.0;

        let color = fps.fps_color();
        // At exactly 30.0, it's not < 30.0, so should be green
        assert_eq!(color, Color::Green);
    }

    // ============ Sample Capacity Edge Cases ============

    #[test]
    fn test_fps_capacity_zero() {
        let mut fps = FpsCounter::with_capacity(0);
        fps.tick();
        // With capacity 0, should immediately remove samples
        assert_eq!(fps.sample_count(), 0);
    }

    #[test]
    fn test_fps_capacity_one() {
        let mut fps = FpsCounter::with_capacity(1);

        thread::sleep(Duration::from_millis(10));
        fps.tick();
        assert_eq!(fps.sample_count(), 1);

        thread::sleep(Duration::from_millis(10));
        fps.tick();
        // Should only keep 1 sample
        assert_eq!(fps.sample_count(), 1);
    }

    #[test]
    fn test_fps_exact_capacity_boundary() {
        let mut fps = FpsCounter::with_capacity(5);

        for _ in 0..5 {
            thread::sleep(Duration::from_millis(1));
            fps.tick();
        }

        assert_eq!(fps.sample_count(), 5);

        // One more should evict oldest
        thread::sleep(Duration::from_millis(1));
        fps.tick();
        assert_eq!(fps.sample_count(), 5);
    }

    #[test]
    fn test_fps_way_over_capacity() {
        let mut fps = FpsCounter::with_capacity(10);

        for _ in 0..100 {
            fps.tick();
        }

        // Should never exceed capacity
        assert_eq!(fps.sample_count(), 10);
    }

    // ============ Empty State Operations ============

    #[test]
    fn test_fps_all_getters_on_empty() {
        let fps = FpsCounter::new();

        assert_eq!(fps.current_fps(), 0.0);
        assert_eq!(fps.average_fps(), 0.0);
        assert_eq!(fps.min_fps(), 0.0);
        assert_eq!(fps.max_fps(), 0.0);
        assert_eq!(fps.average_frame_time_ms(), 0.0);
        assert_eq!(fps.sample_count(), 0);
    }

    #[test]
    fn test_fps_reset_on_empty() {
        let mut fps = FpsCounter::new();
        fps.reset();

        assert_eq!(fps.sample_count(), 0);
        assert_eq!(fps.current_fps(), 0.0);
    }

    #[test]
    fn test_fps_render_on_empty() {
        let fps = FpsCounter::new();
        let display = fps.render_string();

        assert!(display.contains("FPS"));
        assert!(display.contains("0.0"));
    }

    // ============ Multi-Phase Comprehensive Workflow ============

    #[test]
    fn test_fps_10_phase_comprehensive_workflow() {
        let mut fps = FpsCounter::with_capacity(100)
            .with_stats(true)
            .with_warning_threshold(50.0)
            .with_critical_threshold(20.0);

        // Phase 1: Initial state
        assert_eq!(fps.sample_count(), 0);

        // Phase 2: Add some fast frames
        for _ in 0..10 {
            thread::sleep(Duration::from_micros(100));
            fps.tick();
        }
        assert_eq!(fps.sample_count(), 10);

        // Phase 3: Add some slow frames
        for _ in 0..5 {
            thread::sleep(Duration::from_millis(5));
            fps.tick();
        }
        assert_eq!(fps.sample_count(), 15);

        // Phase 4: Check statistics
        assert!(fps.current_fps() > 0.0);
        assert!(fps.average_fps() > 0.0);
        assert!(fps.min_fps() > 0.0);
        assert!(fps.max_fps() >= fps.min_fps());

        // Phase 5: Fill to capacity
        for _ in 0..85 {
            fps.tick();
        }
        assert_eq!(fps.sample_count(), 100);

        // Phase 6: Overflow capacity
        for _ in 0..20 {
            fps.tick();
        }
        assert_eq!(fps.sample_count(), 100); // Should stay at capacity

        // Phase 7: Check render with stats
        let display = fps.render_string();
        assert!(display.contains("avg"));
        assert!(display.contains("min"));
        assert!(display.contains("max"));
        assert!(display.contains("ms"));

        // Phase 8: Clone and verify independence
        let fps2 = fps.clone();
        assert_eq!(fps.sample_count(), fps2.sample_count());

        // Phase 9: Reset
        fps.reset();
        assert_eq!(fps.sample_count(), 0);
        assert_eq!(fps.current_fps(), 0.0);

        // Phase 10: Verify original still works
        fps.tick();
        assert_eq!(fps.sample_count(), 1);
        assert!(fps.current_fps() > 0.0);
    }

    // ============ Frame Time Edge Cases ============

    #[test]
    fn test_fps_very_fast_frames_microseconds() {
        let mut fps = FpsCounter::new();

        for _ in 0..10 {
            thread::sleep(Duration::from_micros(10));
            fps.tick();
        }

        assert_eq!(fps.sample_count(), 10);
        // Very fast frames should produce high FPS
        assert!(fps.current_fps() > 1000.0);
    }

    #[test]
    fn test_fps_very_slow_frames_seconds() {
        let mut fps = FpsCounter::new();

        thread::sleep(Duration::from_secs(1));
        fps.tick();

        assert_eq!(fps.sample_count(), 1);
        // 1 second frame should be ~1 FPS
        assert!(fps.current_fps() < 2.0);
        assert!(fps.current_fps() > 0.0);
    }

    #[test]
    fn test_fps_mixed_frame_times() {
        let mut fps = FpsCounter::new();

        // Mix of different frame times
        thread::sleep(Duration::from_micros(100));
        fps.tick();

        thread::sleep(Duration::from_millis(1));
        fps.tick();

        thread::sleep(Duration::from_millis(10));
        fps.tick();

        thread::sleep(Duration::from_millis(50));
        fps.tick();

        assert_eq!(fps.sample_count(), 4);
        assert!(fps.min_fps() > 0.0);
        assert!(fps.max_fps() > fps.min_fps());
    }

    #[test]
    fn test_fps_consistent_frame_times() {
        let mut fps = FpsCounter::new();

        // All frames same duration
        for _ in 0..5 {
            thread::sleep(Duration::from_millis(16)); // ~60 FPS
            fps.tick();
        }

        assert_eq!(fps.sample_count(), 5);
        // With consistent timing, min/max/avg should be very close
        let avg = fps.average_fps();
        let min = fps.min_fps();
        let max = fps.max_fps();

        assert!(avg > 50.0 && avg < 70.0); // ~60 FPS
        assert!(max - min < 20.0); // Should be close
    }

    // ============ Render String Edge Cases ============

    #[test]
    fn test_fps_render_very_high_fps() {
        let mut fps = FpsCounter::new();
        fps.current_fps = 999999.9;

        let display = fps.render_string();
        assert!(display.contains("999999.9"));
    }

    #[test]
    fn test_fps_render_very_low_fps() {
        let mut fps = FpsCounter::new();
        fps.current_fps = 0.1;

        let display = fps.render_string();
        assert!(display.contains("0.1"));
    }

    #[test]
    fn test_fps_render_toggle_stats() {
        let mut fps = FpsCounter::new();
        thread::sleep(Duration::from_millis(10));
        fps.tick();

        // Without stats
        let display1 = fps.render_string();
        assert!(!display1.contains("avg"));

        // With stats
        fps.show_stats = true;
        let display2 = fps.render_string();
        assert!(display2.contains("avg"));
        assert!(display2.contains("min"));
        assert!(display2.contains("max"));
    }

    // ============ Threshold Boundary Tests ============

    #[test]
    fn test_fps_exactly_at_critical_threshold() {
        let mut fps = FpsCounter::new()
            .with_warning_threshold(30.0)
            .with_critical_threshold(15.0);

        fps.current_fps = 15.0;

        let color = fps.fps_color();
        // At exactly 15.0, it's not < 15.0, so should be yellow (warning)
        assert_eq!(color, Color::Yellow);
    }

    #[test]
    fn test_fps_between_thresholds() {
        let mut fps = FpsCounter::new()
            .with_warning_threshold(30.0)
            .with_critical_threshold(15.0);

        fps.current_fps = 20.0; // Between 15 and 30

        let color = fps.fps_color();
        assert_eq!(color, Color::Yellow);
    }

    #[test]
    fn test_fps_just_below_critical() {
        let mut fps = FpsCounter::new()
            .with_warning_threshold(30.0)
            .with_critical_threshold(15.0);

        fps.current_fps = 14.9;

        let color = fps.fps_color();
        assert_eq!(color, Color::Red);
    }

    // ============ Sample Statistics Edge Cases ============

    #[test]
    fn test_fps_single_sample_stats() {
        let mut fps = FpsCounter::new();

        thread::sleep(Duration::from_millis(10));
        fps.tick();

        assert_eq!(fps.sample_count(), 1);
        // With single sample, min/max/avg should all be equal
        assert_eq!(fps.min_fps(), fps.max_fps());
        assert_eq!(fps.min_fps(), fps.average_fps());
    }

    #[test]
    fn test_fps_all_identical_samples() {
        let mut fps = FpsCounter::new();

        // Create identical samples (as close as possible)
        for _ in 0..5 {
            thread::sleep(Duration::from_millis(10));
            fps.tick();
        }

        assert_eq!(fps.sample_count(), 5);
        // Should have very similar values
        let avg = fps.average_fps();
        let min = fps.min_fps();
        let max = fps.max_fps();

        assert!((max - min) / avg < 0.2); // Less than 20% variation
    }

    #[test]
    fn test_fps_highly_variable_samples() {
        let mut fps = FpsCounter::new();

        // Create highly variable samples
        thread::sleep(Duration::from_micros(100)); // Very fast
        fps.tick();

        thread::sleep(Duration::from_millis(100)); // Very slow
        fps.tick();

        assert_eq!(fps.sample_count(), 2);
        let min = fps.min_fps();
        let max = fps.max_fps();

        // Should have large difference
        assert!(max > min * 10.0); // At least 10x difference
    }

    // ============ Builder Pattern Comprehensive ============

    #[test]
    fn test_fps_builder_all_methods_chained() {
        let fps = FpsCounter::with_capacity(200)
            .with_stats(true)
            .with_warning_threshold(45.0)
            .with_critical_threshold(12.0);

        assert_eq!(fps.max_samples, 200);
        assert!(fps.show_stats);
        assert_eq!(fps.warning_threshold, 45.0);
        assert_eq!(fps.critical_threshold, 12.0);
    }

    #[test]
    fn test_fps_builder_overwrite_values() {
        let fps = FpsCounter::new()
            .with_warning_threshold(30.0)
            .with_warning_threshold(40.0) // Overwrite
            .with_critical_threshold(10.0)
            .with_critical_threshold(5.0); // Overwrite

        // Should use last values
        assert_eq!(fps.warning_threshold, 40.0);
        assert_eq!(fps.critical_threshold, 5.0);
    }

    // ============ Frame Time Milliseconds Edge Cases ============

    #[test]
    fn test_fps_average_frame_time_ms_empty() {
        let fps = FpsCounter::new();
        assert_eq!(fps.average_frame_time_ms(), 0.0);
    }

    #[test]
    fn test_fps_average_frame_time_ms_single_sample() {
        let mut fps = FpsCounter::new();

        thread::sleep(Duration::from_millis(20));
        fps.tick();

        let frame_time = fps.average_frame_time_ms();
        assert!(frame_time >= 20.0);
        assert!(frame_time < 30.0); // Allow some variance
    }

    // ============ Clone Independence ============

    #[test]
    fn test_fps_clone_independence() {
        let mut fps1 = FpsCounter::new();
        thread::sleep(Duration::from_millis(10));
        fps1.tick();

        let mut fps2 = fps1.clone();

        // Modify fps2
        thread::sleep(Duration::from_millis(10));
        fps2.tick();

        // fps1 should be unchanged
        assert_eq!(fps1.sample_count(), 1);
        assert_eq!(fps2.sample_count(), 2);
    }
}
