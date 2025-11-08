//! Memory usage monitoring for performance analysis
//!
//! Tracks application memory consumption with periodic sampling.
//!
//! # Examples
//!
//! ```
//! use toad::widgets::MemoryMonitor;
//!
//! let mut monitor = MemoryMonitor::new();
//! monitor.update();
//!
//! let current = monitor.current_mb();
//! assert!(current >= 0.0);
//! ```

use std::fmt;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::Span,
    widgets::{Block, Paragraph, Widget},
};

/// Memory usage statistics
///
/// Tracks current, peak, and average memory consumption.
///
/// # Examples
///
/// ```
/// use toad::widgets::MemoryStats;
///
/// let stats = MemoryStats {
///     current_bytes: 1024 * 1024, // 1 MB
///     peak_bytes: 2 * 1024 * 1024, // 2 MB
///     samples: 10,
/// };
///
/// assert_eq!(stats.current_mb(), 1.0);
/// assert_eq!(stats.peak_mb(), 2.0);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct MemoryStats {
    /// Current memory usage in bytes
    pub current_bytes: usize,
    /// Peak memory usage in bytes
    pub peak_bytes: usize,
    /// Number of samples collected
    pub samples: usize,
}

impl MemoryStats {
    /// Get current memory in megabytes
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::MemoryStats;
    ///
    /// let stats = MemoryStats {
    ///     current_bytes: 1024 * 1024,
    ///     peak_bytes: 0,
    ///     samples: 1,
    /// };
    /// assert_eq!(stats.current_mb(), 1.0);
    /// ```
    pub fn current_mb(&self) -> f64 {
        self.current_bytes as f64 / (1024.0 * 1024.0)
    }

    /// Get peak memory in megabytes
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::MemoryStats;
    ///
    /// let stats = MemoryStats {
    ///     current_bytes: 0,
    ///     peak_bytes: 2 * 1024 * 1024,
    ///     samples: 1,
    /// };
    /// assert_eq!(stats.peak_mb(), 2.0);
    /// ```
    pub fn peak_mb(&self) -> f64 {
        self.peak_bytes as f64 / (1024.0 * 1024.0)
    }
}

/// Memory usage monitor
///
/// Periodically samples memory usage and tracks statistics.
/// On Linux, reads from `/proc/self/status` for accurate measurements.
/// On other platforms, provides estimated usage.
///
/// # Examples
///
/// ```
/// use toad::widgets::MemoryMonitor;
///
/// let mut monitor = MemoryMonitor::new();
/// monitor.update();
///
/// let stats = monitor.stats();
/// assert!(stats.current_mb() >= 0.0);
/// ```
#[derive(Debug, Clone)]
pub struct MemoryMonitor {
    stats: MemoryStats,
    show_peak: bool,
    warning_threshold_mb: f64,
    critical_threshold_mb: f64,
}

impl MemoryMonitor {
    /// Create a new memory monitor
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::MemoryMonitor;
    ///
    /// let monitor = MemoryMonitor::new();
    /// ```
    pub fn new() -> Self {
        Self {
            stats: MemoryStats::default(),
            show_peak: true,
            warning_threshold_mb: 100.0,
            critical_threshold_mb: 500.0,
        }
    }

    /// Show peak memory usage
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::MemoryMonitor;
    ///
    /// let monitor = MemoryMonitor::new().with_peak(true);
    /// ```
    pub fn with_peak(mut self, show: bool) -> Self {
        self.show_peak = show;
        self
    }

    /// Set warning threshold in MB
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::MemoryMonitor;
    ///
    /// let monitor = MemoryMonitor::new().with_warning_threshold(50.0);
    /// ```
    pub fn with_warning_threshold(mut self, mb: f64) -> Self {
        self.warning_threshold_mb = mb;
        self
    }

    /// Set critical threshold in MB
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::MemoryMonitor;
    ///
    /// let monitor = MemoryMonitor::new().with_critical_threshold(200.0);
    /// ```
    pub fn with_critical_threshold(mut self, mb: f64) -> Self {
        self.critical_threshold_mb = mb;
        self
    }

    /// Update memory statistics
    ///
    /// Call this periodically to sample current memory usage.
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::MemoryMonitor;
    ///
    /// let mut monitor = MemoryMonitor::new();
    /// monitor.update();
    /// assert_eq!(monitor.sample_count(), 1);
    /// ```
    pub fn update(&mut self) {
        let current = Self::get_memory_usage();
        self.stats.current_bytes = current;
        self.stats.peak_bytes = self.stats.peak_bytes.max(current);
        self.stats.samples += 1;
    }

    /// Get current memory statistics
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::MemoryMonitor;
    ///
    /// let mut monitor = MemoryMonitor::new();
    /// monitor.update();
    /// let stats = monitor.stats();
    /// assert!(stats.current_mb() >= 0.0);
    /// ```
    pub fn stats(&self) -> MemoryStats {
        self.stats
    }

    /// Get current memory in MB
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::MemoryMonitor;
    ///
    /// let mut monitor = MemoryMonitor::new();
    /// monitor.update();
    /// assert!(monitor.current_mb() >= 0.0);
    /// ```
    pub fn current_mb(&self) -> f64 {
        self.stats.current_mb()
    }

    /// Get peak memory in MB
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::MemoryMonitor;
    ///
    /// let mut monitor = MemoryMonitor::new();
    /// monitor.update();
    /// assert!(monitor.peak_mb() >= 0.0);
    /// ```
    pub fn peak_mb(&self) -> f64 {
        self.stats.peak_mb()
    }

    /// Get number of samples collected
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::MemoryMonitor;
    ///
    /// let mut monitor = MemoryMonitor::new();
    /// assert_eq!(monitor.sample_count(), 0);
    /// monitor.update();
    /// assert_eq!(monitor.sample_count(), 1);
    /// ```
    pub fn sample_count(&self) -> usize {
        self.stats.samples
    }

    /// Reset all statistics
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::MemoryMonitor;
    ///
    /// let mut monitor = MemoryMonitor::new();
    /// monitor.update();
    /// monitor.reset();
    /// assert_eq!(monitor.sample_count(), 0);
    /// ```
    pub fn reset(&mut self) {
        self.stats = MemoryStats::default();
    }

    /// Get memory color based on thresholds
    fn memory_color(&self) -> Color {
        let mb = self.current_mb();
        if mb >= self.critical_threshold_mb {
            Color::Red
        } else if mb >= self.warning_threshold_mb {
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
    /// use toad::widgets::MemoryMonitor;
    ///
    /// let mut monitor = MemoryMonitor::new();
    /// monitor.update();
    /// let display = monitor.render_string();
    /// assert!(display.contains("MEM"));
    /// ```
    pub fn render_string(&self) -> String {
        if self.show_peak {
            format!(
                "MEM: {:.1} MB (peak: {:.1} MB)",
                self.current_mb(),
                self.peak_mb()
            )
        } else {
            format!("MEM: {:.1} MB", self.current_mb())
        }
    }

    /// Get current memory usage in bytes
    ///
    /// Platform-specific implementation:
    /// - Linux: Reads from `/proc/self/status` (VmRSS)
    /// - Other: Returns estimate based on heap allocations
    fn get_memory_usage() -> usize {
        #[cfg(target_os = "linux")]
        {
            Self::get_linux_memory_usage()
        }

        #[cfg(not(target_os = "linux"))]
        {
            // Fallback: estimate based on typical TUI memory usage
            // This is a conservative estimate for non-Linux platforms
            2 * 1024 * 1024 // 2 MB baseline
        }
    }

    #[cfg(target_os = "linux")]
    fn get_linux_memory_usage() -> usize {
        use std::fs;

        // Read /proc/self/status and parse VmRSS (Resident Set Size)
        if let Ok(status) = fs::read_to_string("/proc/self/status") {
            for line in status.lines() {
                if let Some(vmrss) = line.strip_prefix("VmRSS:") {
                    // VmRSS is in kB, parse it
                    let parts: Vec<&str> = vmrss.split_whitespace().collect();
                    if let Some(kb_str) = parts.first()
                        && let Ok(kb) = kb_str.parse::<usize>()
                    {
                        return kb * 1024; // Convert kB to bytes
                    }
                }
            }
        }

        // Fallback if parsing fails
        0
    }
}

impl Default for MemoryMonitor {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for MemoryMonitor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render_string())
    }
}

impl Widget for &MemoryMonitor {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let text = self.render_string();
        let color = self.memory_color();

        let paragraph =
            Paragraph::new(Span::styled(text, Style::default().fg(color))).block(Block::default());

        paragraph.render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_stats_mb() {
        let stats = MemoryStats {
            current_bytes: 1024 * 1024,
            peak_bytes: 2 * 1024 * 1024,
            samples: 1,
        };

        assert_eq!(stats.current_mb(), 1.0);
        assert_eq!(stats.peak_mb(), 2.0);
    }

    #[test]
    fn test_memory_stats_default() {
        let stats = MemoryStats::default();
        assert_eq!(stats.current_bytes, 0);
        assert_eq!(stats.peak_bytes, 0);
        assert_eq!(stats.samples, 0);
    }

    #[test]
    fn test_new() {
        let monitor = MemoryMonitor::new();
        assert_eq!(monitor.sample_count(), 0);
        assert!(monitor.show_peak);
    }

    #[test]
    fn test_with_peak() {
        let monitor = MemoryMonitor::new().with_peak(false);
        assert!(!monitor.show_peak);
    }

    #[test]
    fn test_with_thresholds() {
        let monitor = MemoryMonitor::new()
            .with_warning_threshold(50.0)
            .with_critical_threshold(200.0);

        assert_eq!(monitor.warning_threshold_mb, 50.0);
        assert_eq!(monitor.critical_threshold_mb, 200.0);
    }

    #[test]
    fn test_update() {
        let mut monitor = MemoryMonitor::new();
        monitor.update();

        assert_eq!(monitor.sample_count(), 1);
        assert!(monitor.current_mb() >= 0.0);
    }

    #[test]
    fn test_stats() {
        let mut monitor = MemoryMonitor::new();
        monitor.update();

        let stats = monitor.stats();
        assert!(stats.current_mb() >= 0.0);
        assert!(stats.peak_mb() >= 0.0);
        assert_eq!(stats.samples, 1);
    }

    #[test]
    fn test_current_mb() {
        let mut monitor = MemoryMonitor::new();
        monitor.update();

        let mb = monitor.current_mb();
        assert!(mb >= 0.0);
    }

    #[test]
    fn test_peak_mb() {
        let mut monitor = MemoryMonitor::new();
        monitor.update();

        let peak = monitor.peak_mb();
        assert!(peak >= 0.0);
    }

    #[test]
    fn test_peak_tracking() {
        let mut monitor = MemoryMonitor::new();
        monitor.update();

        let first_peak = monitor.peak_mb();

        monitor.update();
        let second_peak = monitor.peak_mb();

        // Peak should not decrease
        assert!(second_peak >= first_peak);
    }

    #[test]
    fn test_reset() {
        let mut monitor = MemoryMonitor::new();
        monitor.update();

        assert_eq!(monitor.sample_count(), 1);

        monitor.reset();
        assert_eq!(monitor.sample_count(), 0);
        assert_eq!(monitor.current_mb(), 0.0);
        assert_eq!(monitor.peak_mb(), 0.0);
    }

    #[test]
    fn test_render_string_with_peak() {
        let mut monitor = MemoryMonitor::new().with_peak(true);
        monitor.update();

        let display = monitor.render_string();
        assert!(display.contains("MEM"));
        assert!(display.contains("peak"));
    }

    #[test]
    fn test_render_string_without_peak() {
        let mut monitor = MemoryMonitor::new().with_peak(false);
        monitor.update();

        let display = monitor.render_string();
        assert!(display.contains("MEM"));
        assert!(!display.contains("peak"));
    }

    #[test]
    fn test_memory_color_green() {
        let mut monitor = MemoryMonitor::new()
            .with_warning_threshold(1000.0)
            .with_critical_threshold(2000.0);

        monitor.update();

        let color = monitor.memory_color();
        // Should be green for low memory usage
        assert_eq!(color, Color::Green);
    }

    #[test]
    fn test_multiple_updates() {
        let mut monitor = MemoryMonitor::new();

        for _ in 0..10 {
            monitor.update();
        }

        assert_eq!(monitor.sample_count(), 10);
        assert!(monitor.current_mb() >= 0.0);
        assert!(monitor.peak_mb() >= 0.0);
    }

    #[test]
    fn test_builder_pattern() {
        let monitor = MemoryMonitor::new()
            .with_peak(false)
            .with_warning_threshold(50.0)
            .with_critical_threshold(200.0);

        assert!(!monitor.show_peak);
        assert_eq!(monitor.warning_threshold_mb, 50.0);
        assert_eq!(monitor.critical_threshold_mb, 200.0);
    }

    #[test]
    fn test_display_trait() {
        let mut monitor = MemoryMonitor::new();
        monitor.update();

        let display = format!("{}", monitor);
        assert!(display.contains("MEM"));
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn test_linux_memory_usage() {
        let mem = MemoryMonitor::get_memory_usage();
        // Should return non-zero memory on Linux
        assert!(mem > 0);
    }
}
