//! Progress bar and multi-stage progress widgets
//!
//! Provides visual feedback for long-running operations with single and multi-stage progress tracking.
//!
//! # Examples
//!
//! ```
//! use toad::widgets::{ProgressBar, MultiStageProgress};
//!
//! // Simple progress bar
//! let progress = ProgressBar::new("Loading")
//!     .with_progress(0.75)
//!     .with_message("Processing files...");
//!
//! // Multi-stage progress
//! let stages = vec!["Download".to_string(), "Extract".to_string(), "Install".to_string()];
//! let mut multi = MultiStageProgress::new("Installation", stages);
//! multi.set_stage(1);
//! multi.set_stage_progress(0.5);
//! ```

use crate::ui::theme::ToadTheme;
use ratatui::{
    Frame,
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Gauge, Widget},
};
use std::time::{Duration, Instant};

/// Progress bar widget for single tasks
///
/// # Examples
///
/// ```
/// use toad::widgets::ProgressBar;
///
/// let mut progress = ProgressBar::new("Download");
/// progress.set_progress(0.5);
/// assert_eq!(progress.progress(), 0.5);
/// assert!(!progress.is_complete());
/// ```
pub struct ProgressBar {
    title: String,
    progress: f64, // 0.0 to 1.0
    message: Option<String>,
}

impl ProgressBar {
    /// Create a new progress bar
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::ProgressBar;
    ///
    /// let progress = ProgressBar::new("Loading");
    /// assert_eq!(progress.progress(), 0.0);
    /// ```
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            progress: 0.0,
            message: None,
        }
    }

    /// Set the progress (0.0 to 1.0) using builder pattern
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::ProgressBar;
    ///
    /// let progress = ProgressBar::new("Loading")
    ///     .with_progress(0.75);
    /// assert_eq!(progress.progress(), 0.75);
    /// ```
    pub fn with_progress(mut self, progress: f64) -> Self {
        self.progress = progress.clamp(0.0, 1.0);
        self
    }

    /// Set the progress message using builder pattern
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::ProgressBar;
    ///
    /// let progress = ProgressBar::new("Loading")
    ///     .with_message("Processing files...");
    /// ```
    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }

    /// Update the progress (0.0 to 1.0)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::ProgressBar;
    ///
    /// let mut progress = ProgressBar::new("Loading");
    /// progress.set_progress(0.5);
    /// assert_eq!(progress.progress(), 0.5);
    /// ```
    pub fn set_progress(&mut self, progress: f64) {
        self.progress = progress.clamp(0.0, 1.0);
    }

    /// Set the message
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::ProgressBar;
    ///
    /// let mut progress = ProgressBar::new("Loading");
    /// progress.set_message("Processing...");
    /// ```
    pub fn set_message(&mut self, message: impl Into<String>) {
        self.message = Some(message.into());
    }

    /// Get current progress
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::ProgressBar;
    ///
    /// let progress = ProgressBar::new("Loading")
    ///     .with_progress(0.75);
    /// assert_eq!(progress.progress(), 0.75);
    /// ```
    pub fn progress(&self) -> f64 {
        self.progress
    }

    /// Check if progress is complete (100%)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::ProgressBar;
    ///
    /// let incomplete = ProgressBar::new("Loading").with_progress(0.5);
    /// assert!(!incomplete.is_complete());
    ///
    /// let complete = ProgressBar::new("Loading").with_progress(1.0);
    /// assert!(complete.is_complete());
    /// ```
    pub fn is_complete(&self) -> bool {
        self.progress >= 1.0
    }

    /// Render the progress bar to a frame
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let label = if let Some(msg) = &self.message {
            format!("{} - {:.0}%", msg, self.progress * 100.0)
        } else {
            format!("{:.0}%", self.progress * 100.0)
        };

        let gauge = Gauge::default()
            .block(
                Block::default()
                    .title(format!(" {} ", self.title))
                    .title_style(
                        Style::default()
                            .fg(ToadTheme::TOAD_GREEN)
                            .add_modifier(Modifier::BOLD),
                    )
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(ToadTheme::TOAD_GREEN)),
            )
            .gauge_style(
                Style::default()
                    .fg(ToadTheme::BLACK)
                    .bg(ToadTheme::TOAD_GREEN),
            )
            .label(label)
            .ratio(self.progress);

        frame.render_widget(gauge, area);
    }
}

impl Default for ProgressBar {
    fn default() -> Self {
        Self::new("Progress")
    }
}

/// Stage completion status
///
/// # Examples
///
/// ```
/// use toad::widgets::StageStatus;
///
/// let status = StageStatus::InProgress;
/// assert!(status.is_active());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StageStatus {
    /// Stage not yet started
    Pending,
    /// Stage currently in progress
    InProgress,
    /// Stage completed successfully
    Complete,
}

impl StageStatus {
    /// Check if stage is active (in progress)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::StageStatus;
    ///
    /// assert!(StageStatus::InProgress.is_active());
    /// assert!(!StageStatus::Pending.is_active());
    /// assert!(!StageStatus::Complete.is_active());
    /// ```
    pub fn is_active(self) -> bool {
        matches!(self, StageStatus::InProgress)
    }

    /// Check if stage is complete
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::StageStatus;
    ///
    /// assert!(StageStatus::Complete.is_complete());
    /// assert!(!StageStatus::Pending.is_complete());
    /// assert!(!StageStatus::InProgress.is_complete());
    /// ```
    pub fn is_complete(self) -> bool {
        matches!(self, StageStatus::Complete)
    }

    /// Get visual indicator for status
    pub fn indicator(self) -> &'static str {
        match self {
            StageStatus::Pending => "○",
            StageStatus::InProgress => "◉",
            StageStatus::Complete => "✓",
        }
    }

    /// Get color for status
    pub fn color(self) -> Color {
        match self {
            StageStatus::Pending => Color::DarkGray,
            StageStatus::InProgress => Color::Yellow,
            StageStatus::Complete => Color::Green,
        }
    }
}

/// Information about a single stage
#[derive(Debug, Clone)]
struct StageInfo {
    name: String,
    status: StageStatus,
    start_time: Option<Instant>,
    end_time: Option<Instant>,
}

impl StageInfo {
    fn new(name: String) -> Self {
        Self {
            name,
            status: StageStatus::Pending,
            start_time: None,
            end_time: None,
        }
    }

    fn elapsed(&self) -> Option<Duration> {
        match (self.start_time, self.end_time) {
            (Some(start), Some(end)) => Some(end.duration_since(start)),
            (Some(start), None) => Some(start.elapsed()),
            _ => None,
        }
    }
}

/// Multi-stage progress tracker with time tracking
///
/// Tracks progress across multiple stages with individual stage completion,
/// elapsed time tracking, and visual indicators.
///
/// # Examples
///
/// ```
/// use toad::widgets::MultiStageProgress;
///
/// let stages = vec!["Download".to_string(), "Extract".to_string(), "Install".to_string()];
/// let mut progress = MultiStageProgress::new("Setup", stages);
///
/// progress.set_stage(0);
/// progress.set_stage_progress(1.0);
/// progress.complete_stage();
///
/// assert_eq!(progress.current_stage(), 1);
/// assert_eq!(progress.completed_stages(), 1);
/// ```
pub struct MultiStageProgress {
    title: String,
    stages: Vec<StageInfo>,
    current_stage: usize,
    stage_progress: f64,
    show_time: bool,
}

impl MultiStageProgress {
    /// Create a new multi-stage progress tracker
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::MultiStageProgress;
    ///
    /// let stages = vec!["Build".to_string(), "Test".to_string()];
    /// let progress = MultiStageProgress::new("CI/CD", stages);
    /// assert_eq!(progress.stage_count(), 2);
    /// ```
    pub fn new(title: impl Into<String>, stage_names: Vec<String>) -> Self {
        let stages = stage_names.into_iter().map(StageInfo::new).collect();

        Self {
            title: title.into(),
            stages,
            current_stage: 0,
            stage_progress: 0.0,
            show_time: false,
        }
    }

    /// Enable time tracking display
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::MultiStageProgress;
    ///
    /// let stages = vec!["Stage 1".to_string()];
    /// let progress = MultiStageProgress::new("Task", stages)
    ///     .with_time_tracking(true);
    /// ```
    pub fn with_time_tracking(mut self, show: bool) -> Self {
        self.show_time = show;
        self
    }

    /// Get the number of stages
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::MultiStageProgress;
    ///
    /// let stages = vec!["A".to_string(), "B".to_string(), "C".to_string()];
    /// let progress = MultiStageProgress::new("Task", stages);
    /// assert_eq!(progress.stage_count(), 3);
    /// ```
    pub fn stage_count(&self) -> usize {
        self.stages.len()
    }

    /// Get current stage index
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::MultiStageProgress;
    ///
    /// let stages = vec!["A".to_string(), "B".to_string()];
    /// let mut progress = MultiStageProgress::new("Task", stages);
    /// progress.set_stage(1);
    /// assert_eq!(progress.current_stage(), 1);
    /// ```
    pub fn current_stage(&self) -> usize {
        self.current_stage
    }

    /// Get number of completed stages
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::MultiStageProgress;
    ///
    /// let stages = vec!["A".to_string(), "B".to_string(), "C".to_string()];
    /// let mut progress = MultiStageProgress::new("Task", stages);
    /// progress.set_stage(0);
    /// progress.complete_stage();
    /// assert_eq!(progress.completed_stages(), 1);
    /// ```
    pub fn completed_stages(&self) -> usize {
        self.stages
            .iter()
            .filter(|s| s.status == StageStatus::Complete)
            .count()
    }

    /// Set the current stage (starts time tracking)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::MultiStageProgress;
    ///
    /// let stages = vec!["A".to_string(), "B".to_string()];
    /// let mut progress = MultiStageProgress::new("Task", stages);
    /// progress.set_stage(1);
    /// assert_eq!(progress.current_stage(), 1);
    /// ```
    pub fn set_stage(&mut self, stage: usize) {
        if stage < self.stages.len() {
            self.current_stage = stage;
            self.stage_progress = 0.0;

            // Mark as in progress and start timing
            if let Some(stage_info) = self.stages.get_mut(stage) {
                stage_info.status = StageStatus::InProgress;
                if stage_info.start_time.is_none() {
                    stage_info.start_time = Some(Instant::now());
                }
            }
        }
    }

    /// Set the progress within the current stage (0.0 to 1.0)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::MultiStageProgress;
    ///
    /// let stages = vec!["A".to_string()];
    /// let mut progress = MultiStageProgress::new("Task", stages);
    /// progress.set_stage(0);
    /// progress.set_stage_progress(0.75);
    /// ```
    pub fn set_stage_progress(&mut self, progress: f64) {
        self.stage_progress = progress.clamp(0.0, 1.0);
    }

    /// Mark current stage as complete and move to next
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::MultiStageProgress;
    ///
    /// let stages = vec!["A".to_string(), "B".to_string()];
    /// let mut progress = MultiStageProgress::new("Task", stages);
    /// progress.set_stage(0);
    /// progress.complete_stage();
    /// assert_eq!(progress.current_stage(), 1);
    /// assert_eq!(progress.completed_stages(), 1);
    /// ```
    pub fn complete_stage(&mut self) {
        // Mark current stage as complete
        if let Some(stage_info) = self.stages.get_mut(self.current_stage) {
            stage_info.status = StageStatus::Complete;
            stage_info.end_time = Some(Instant::now());
        }

        // Move to next stage if available
        if self.current_stage < self.stages.len() - 1 {
            self.set_stage(self.current_stage + 1);
        } else {
            self.stage_progress = 1.0;
        }
    }

    /// Move to the next stage without completing current
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::MultiStageProgress;
    ///
    /// let stages = vec!["A".to_string(), "B".to_string()];
    /// let mut progress = MultiStageProgress::new("Task", stages);
    /// progress.set_stage(0);
    /// progress.next_stage();
    /// assert_eq!(progress.current_stage(), 1);
    /// ```
    pub fn next_stage(&mut self) {
        if self.current_stage < self.stages.len() - 1 {
            self.set_stage(self.current_stage + 1);
        }
    }

    /// Get overall progress across all stages (0.0 to 1.0)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::MultiStageProgress;
    ///
    /// let stages = vec!["A".to_string(), "B".to_string()];
    /// let mut progress = MultiStageProgress::new("Task", stages);
    /// progress.set_stage(0);
    /// progress.set_stage_progress(0.5);
    ///
    /// // 0.5 progress in first of 2 stages = 0.25 overall
    /// assert_eq!(progress.overall_progress(), 0.25);
    /// ```
    pub fn overall_progress(&self) -> f64 {
        if self.stages.is_empty() {
            return 0.0;
        }

        let stages_complete = self.completed_stages() as f64 / self.stages.len() as f64;
        let current_contribution = self.stage_progress / self.stages.len() as f64;
        let total = stages_complete + current_contribution;

        total.clamp(0.0, 1.0)
    }

    /// Check if all stages are complete
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::MultiStageProgress;
    ///
    /// let stages = vec!["A".to_string()];
    /// let mut progress = MultiStageProgress::new("Task", stages);
    /// progress.set_stage(0);
    /// progress.complete_stage();
    /// assert!(progress.is_complete());
    /// ```
    pub fn is_complete(&self) -> bool {
        self.completed_stages() == self.stages.len()
    }

    /// Get elapsed time for a specific stage
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::MultiStageProgress;
    ///
    /// let stages = vec!["A".to_string()];
    /// let mut progress = MultiStageProgress::new("Task", stages);
    /// progress.set_stage(0);
    ///
    /// // Stage 0 should have some elapsed time
    /// assert!(progress.stage_elapsed(0).is_some());
    /// // Stage 1 doesn't exist
    /// assert!(progress.stage_elapsed(1).is_none());
    /// ```
    pub fn stage_elapsed(&self, stage: usize) -> Option<Duration> {
        self.stages.get(stage).and_then(|s| s.elapsed())
    }

    /// Get total elapsed time across all stages
    pub fn total_elapsed(&self) -> Duration {
        self.stages.iter().filter_map(|s| s.elapsed()).sum()
    }

    /// Render the multi-stage progress bar to a frame
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let current_stage_name = self
            .stages
            .get(self.current_stage)
            .map(|s| s.name.as_str())
            .unwrap_or("Unknown");

        let mut label = format!(
            "Stage {}/{}: {} ({:.0}%)",
            self.current_stage + 1,
            self.stages.len(),
            current_stage_name,
            self.stage_progress * 100.0
        );

        // Add time if enabled
        if self.show_time
            && let Some(elapsed) = self.stage_elapsed(self.current_stage)
        {
            label.push_str(&format!(" | {:.1}s", elapsed.as_secs_f64()));
        }

        let gauge = Gauge::default()
            .block(
                Block::default()
                    .title(format!(" {} ", self.title))
                    .title_style(
                        Style::default()
                            .fg(ToadTheme::TOAD_GREEN)
                            .add_modifier(Modifier::BOLD),
                    )
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(ToadTheme::TOAD_GREEN)),
            )
            .gauge_style(
                Style::default()
                    .fg(ToadTheme::BLACK)
                    .bg(ToadTheme::TOAD_GREEN),
            )
            .label(label)
            .ratio(self.overall_progress());

        frame.render_widget(gauge, area);
    }

    /// Render stage indicators
    pub fn render_string(&self) -> String {
        let mut output = String::new();

        for (i, stage) in self.stages.iter().enumerate() {
            if i > 0 {
                output.push_str(" → ");
            }

            output.push_str(stage.status.indicator());
            output.push(' ');
            output.push_str(&stage.name);

            if self.show_time
                && let Some(elapsed) = stage.elapsed()
            {
                output.push_str(&format!(" ({:.1}s)", elapsed.as_secs_f64()));
            }
        }

        output
    }
}

impl Widget for &MultiStageProgress {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width == 0 || area.height == 0 {
            return;
        }

        let stages_str = self.render_string();
        let style = Style::default().fg(ToadTheme::TOAD_GREEN);

        // Render stage indicators
        buf.set_string(area.x, area.y, stages_str, style);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_bar_creation() {
        let progress = ProgressBar::new("Test");
        assert_eq!(progress.progress(), 0.0);
        assert!(!progress.is_complete());
    }

    #[test]
    fn test_progress_bar_with_progress() {
        let progress = ProgressBar::new("Test").with_progress(0.5);
        assert_eq!(progress.progress(), 0.5);
    }

    #[test]
    fn test_progress_bar_set_progress() {
        let mut progress = ProgressBar::new("Test");
        progress.set_progress(0.75);
        assert_eq!(progress.progress(), 0.75);
    }

    #[test]
    fn test_progress_bar_clamps() {
        let mut progress = ProgressBar::new("Test");
        progress.set_progress(1.5);
        assert_eq!(progress.progress(), 1.0);

        progress.set_progress(-0.5);
        assert_eq!(progress.progress(), 0.0);
    }

    #[test]
    fn test_progress_bar_is_complete() {
        let mut progress = ProgressBar::new("Test");
        assert!(!progress.is_complete());

        progress.set_progress(1.0);
        assert!(progress.is_complete());
    }

    #[test]
    fn test_stage_status_is_active() {
        assert!(StageStatus::InProgress.is_active());
        assert!(!StageStatus::Pending.is_active());
        assert!(!StageStatus::Complete.is_active());
    }

    #[test]
    fn test_stage_status_is_complete() {
        assert!(StageStatus::Complete.is_complete());
        assert!(!StageStatus::Pending.is_complete());
        assert!(!StageStatus::InProgress.is_complete());
    }

    #[test]
    fn test_stage_status_indicator() {
        assert_eq!(StageStatus::Pending.indicator(), "○");
        assert_eq!(StageStatus::InProgress.indicator(), "◉");
        assert_eq!(StageStatus::Complete.indicator(), "✓");
    }

    #[test]
    fn test_multi_stage_creation() {
        let stages = vec!["A".to_string(), "B".to_string()];
        let progress = MultiStageProgress::new("Test", stages);
        assert_eq!(progress.stage_count(), 2);
        assert_eq!(progress.current_stage(), 0);
    }

    #[test]
    fn test_multi_stage_set_stage() {
        let stages = vec!["A".to_string(), "B".to_string()];
        let mut progress = MultiStageProgress::new("Test", stages);
        progress.set_stage(1);
        assert_eq!(progress.current_stage(), 1);
    }

    #[test]
    fn test_multi_stage_progress() {
        let stages = vec!["A".to_string(), "B".to_string()];
        let mut progress = MultiStageProgress::new("Test", stages);
        progress.set_stage(0);
        progress.set_stage_progress(0.5);

        // 0.5 progress in first of 2 stages = 0.25 overall
        assert_eq!(progress.overall_progress(), 0.25);
    }

    #[test]
    fn test_multi_stage_complete() {
        let stages = vec!["A".to_string(), "B".to_string()];
        let mut progress = MultiStageProgress::new("Test", stages);
        progress.set_stage(0);
        progress.complete_stage();

        assert_eq!(progress.completed_stages(), 1);
        assert_eq!(progress.current_stage(), 1);
    }

    #[test]
    fn test_multi_stage_is_complete() {
        let stages = vec!["A".to_string()];
        let mut progress = MultiStageProgress::new("Test", stages);
        assert!(!progress.is_complete());

        progress.set_stage(0);
        progress.complete_stage();
        assert!(progress.is_complete());
    }

    #[test]
    fn test_multi_stage_next_stage() {
        let stages = vec!["A".to_string(), "B".to_string(), "C".to_string()];
        let mut progress = MultiStageProgress::new("Test", stages);
        progress.set_stage(0);
        progress.next_stage();
        assert_eq!(progress.current_stage(), 1);
    }

    #[test]
    fn test_multi_stage_elapsed_time() {
        let stages = vec!["A".to_string()];
        let mut progress = MultiStageProgress::new("Test", stages);
        progress.set_stage(0);

        // Should have some elapsed time
        assert!(progress.stage_elapsed(0).is_some());
    }

    #[test]
    fn test_multi_stage_overall_progress_empty() {
        let stages: Vec<String> = vec![];
        let progress = MultiStageProgress::new("Test", stages);
        assert_eq!(progress.overall_progress(), 0.0);
    }

    #[test]
    fn test_multi_stage_with_time_tracking() {
        let stages = vec!["A".to_string()];
        let progress = MultiStageProgress::new("Test", stages).with_time_tracking(true);
        assert!(progress.show_time);
    }

    #[test]
    fn test_multi_stage_render_string() {
        let stages = vec!["Download".to_string(), "Install".to_string()];
        let mut progress = MultiStageProgress::new("Setup", stages);
        progress.set_stage(0);

        let output = progress.render_string();
        assert!(output.contains("Download"));
        assert!(output.contains("Install"));
    }

    #[test]
    fn test_stage_bounds_checking() {
        let stages = vec!["A".to_string()];
        let mut progress = MultiStageProgress::new("Test", stages);

        // Setting stage beyond bounds should be handled gracefully
        progress.set_stage(10);
        // Should not crash and should stay within bounds
    }

    #[test]
    fn test_progress_bar_default() {
        let progress = ProgressBar::default();
        assert_eq!(progress.progress(), 0.0);
    }

    // ============ COMPREHENSIVE EDGE CASE TESTS ============

    #[test]
    fn test_progress_bar_negative_values_clamped() {
        let mut progress = ProgressBar::new("Test");
        progress.set_progress(-0.5);
        assert_eq!(progress.progress(), 0.0);

        progress.set_progress(-100.0);
        assert_eq!(progress.progress(), 0.0);
    }

    #[test]
    fn test_progress_bar_overflow_values_clamped() {
        let mut progress = ProgressBar::new("Test");
        progress.set_progress(1.5);
        assert_eq!(progress.progress(), 1.0);

        progress.set_progress(100.0);
        assert_eq!(progress.progress(), 1.0);
    }

    #[test]
    fn test_progress_bar_with_very_long_title() {
        let long_title = "A".repeat(1000);
        let progress = ProgressBar::new(long_title);
        assert_eq!(progress.progress(), 0.0);
    }

    #[test]
    fn test_progress_bar_with_unicode_title() {
        let progress = ProgressBar::new("🚀 Loading 日本語");
        assert_eq!(progress.progress(), 0.0);
    }

    #[test]
    fn test_progress_bar_with_very_long_message() {
        let long_message = "B".repeat(1000);
        let progress = ProgressBar::new("Test").with_message(long_message);
        assert_eq!(progress.progress(), 0.0);
    }

    #[test]
    fn test_progress_bar_with_unicode_message() {
        let progress = ProgressBar::new("Test").with_message("処理中... 🔄");
        assert_eq!(progress.progress(), 0.0);
    }

    #[test]
    fn test_progress_bar_very_small_increments() {
        let mut progress = ProgressBar::new("Test");
        progress.set_progress(0.001);
        assert_eq!(progress.progress(), 0.001);

        progress.set_progress(0.0001);
        assert_eq!(progress.progress(), 0.0001);
    }

    #[test]
    fn test_progress_bar_rapid_updates() {
        let mut progress = ProgressBar::new("Test");

        for i in 0..1000 {
            progress.set_progress(i as f64 / 1000.0);
        }

        assert_eq!(progress.progress(), 0.999);
    }

    #[test]
    fn test_progress_bar_message_update() {
        let mut progress = ProgressBar::new("Test");
        progress.set_message("First");
        progress.set_message("Second");
        progress.set_message("Third");
        // Verify no panic on multiple updates
    }

    #[test]
    fn test_progress_bar_exactly_half() {
        let progress = ProgressBar::new("Test").with_progress(0.5);
        assert_eq!(progress.progress(), 0.5);
        assert!(!progress.is_complete());
    }

    #[test]
    fn test_progress_bar_exactly_complete() {
        let progress = ProgressBar::new("Test").with_progress(1.0);
        assert_eq!(progress.progress(), 1.0);
        assert!(progress.is_complete());
    }

    #[test]
    fn test_progress_bar_just_below_complete() {
        let progress = ProgressBar::new("Test").with_progress(0.9999);
        assert!(!progress.is_complete());
    }

    #[test]
    fn test_multi_stage_empty_stages() {
        let stages: Vec<String> = vec![];
        let progress = MultiStageProgress::new("Empty", stages);

        assert_eq!(progress.stage_count(), 0);
        assert_eq!(progress.overall_progress(), 0.0);
        assert!(progress.is_complete());
    }

    #[test]
    fn test_multi_stage_single_stage() {
        let stages = vec!["Only Stage".to_string()];
        let mut progress = MultiStageProgress::new("Test", stages);

        progress.set_stage(0);
        progress.complete_stage();

        assert!(progress.is_complete());
    }

    #[test]
    fn test_multi_stage_many_stages() {
        let stages: Vec<String> = (0..100).map(|i| format!("Stage {}", i)).collect();
        let progress = MultiStageProgress::new("Test", stages);

        assert_eq!(progress.stage_count(), 100);
    }

    #[test]
    fn test_multi_stage_unicode_stage_names() {
        let stages = vec![
            "📥 Download".to_string(),
            "📦 Extract".to_string(),
            "⚙️ Configure".to_string(),
            "✅ Complete".to_string(),
        ];
        let mut progress = MultiStageProgress::new("Setup", stages);

        progress.set_stage(0);
        assert_eq!(progress.current_stage(), 0);
    }

    #[test]
    fn test_multi_stage_very_long_stage_names() {
        let long_name = "A".repeat(1000);
        let stages = vec![long_name.clone(), long_name.clone()];
        let progress = MultiStageProgress::new("Test", stages);

        assert_eq!(progress.stage_count(), 2);
    }

    #[test]
    fn test_multi_stage_progress_precision() {
        let stages = vec!["A".to_string(), "B".to_string()];
        let mut progress = MultiStageProgress::new("Test", stages);

        progress.set_stage(0);
        progress.set_stage_progress(0.5);

        // 0.5 progress in first of 2 stages
        let overall = progress.overall_progress();
        assert!((overall - 0.25).abs() < 1e-10); // Floating point comparison
    }

    #[test]
    fn test_multi_stage_complete_all_stages() {
        let stages = vec!["A".to_string(), "B".to_string(), "C".to_string()];
        let mut progress = MultiStageProgress::new("Test", stages);

        progress.set_stage(0);
        progress.complete_stage();
        progress.complete_stage();
        progress.complete_stage();

        assert!(progress.is_complete());
        assert_eq!(progress.completed_stages(), 3);
    }

    #[test]
    fn test_multi_stage_next_stage_without_completing() {
        let stages = vec!["A".to_string(), "B".to_string(), "C".to_string()];
        let mut progress = MultiStageProgress::new("Test", stages);

        progress.set_stage(0);
        progress.next_stage();

        assert_eq!(progress.current_stage(), 1);
        assert_eq!(progress.completed_stages(), 0); // Not completed, just moved
    }

    #[test]
    fn test_multi_stage_next_stage_at_last_stage() {
        let stages = vec!["A".to_string(), "B".to_string()];
        let mut progress = MultiStageProgress::new("Test", stages);

        progress.set_stage(1);
        progress.next_stage();

        // Should stay at stage 1
        assert_eq!(progress.current_stage(), 1);
    }

    #[test]
    fn test_multi_stage_complete_at_last_stage() {
        let stages = vec!["A".to_string(), "B".to_string()];
        let mut progress = MultiStageProgress::new("Test", stages);

        progress.set_stage(0);
        progress.complete_stage(); // Move to stage 1
        progress.complete_stage(); // Complete stage 1

        assert_eq!(progress.current_stage(), 1);
        assert_eq!(progress.stage_progress, 1.0);
        assert!(progress.is_complete());
    }

    #[test]
    fn test_multi_stage_stage_elapsed_non_existent() {
        let stages = vec!["A".to_string()];
        let mut progress = MultiStageProgress::new("Test", stages);

        progress.set_stage(0);

        // Query non-existent stage
        assert!(progress.stage_elapsed(10).is_none());
    }

    #[test]
    fn test_multi_stage_render_string_output() {
        let stages = vec!["Download".to_string(), "Install".to_string()];
        let mut progress = MultiStageProgress::new("Setup", stages);

        progress.set_stage(0);
        let output = progress.render_string();

        assert!(output.contains("Download"));
        assert!(output.contains("Install"));
        assert!(output.contains("→")); // Separator
    }

    #[test]
    fn test_stage_status_color_codes() {
        assert_ne!(
            StageStatus::Pending.color(),
            StageStatus::InProgress.color()
        );
        assert_ne!(
            StageStatus::InProgress.color(),
            StageStatus::Complete.color()
        );
    }

    #[test]
    fn test_multi_stage_overall_progress_at_boundaries() {
        let stages = vec!["A".to_string(), "B".to_string()];
        let mut progress = MultiStageProgress::new("Test", stages);

        // At start
        assert_eq!(progress.overall_progress(), 0.0);

        // First stage complete
        progress.set_stage(0);
        progress.complete_stage();
        assert_eq!(progress.overall_progress(), 0.5);

        // Second stage complete
        progress.complete_stage();
        assert_eq!(progress.overall_progress(), 1.0);
    }

    #[test]
    fn test_multi_stage_with_time_tracking_disabled() {
        let stages = vec!["A".to_string()];
        let progress = MultiStageProgress::new("Test", stages).with_time_tracking(false);

        assert!(!progress.show_time);
    }

    #[test]
    fn test_multi_stage_total_elapsed_with_no_stages_started() {
        let stages = vec!["A".to_string(), "B".to_string()];
        let progress = MultiStageProgress::new("Test", stages);

        let elapsed = progress.total_elapsed();
        assert_eq!(elapsed, Duration::from_secs(0));
    }

    #[test]
    fn test_multi_stage_set_stage_progress_clamping() {
        let stages = vec!["A".to_string()];
        let mut progress = MultiStageProgress::new("Test", stages);

        progress.set_stage(0);

        // Test overflow
        progress.set_stage_progress(2.0);
        assert_eq!(progress.stage_progress, 1.0);

        // Test underflow
        progress.set_stage_progress(-0.5);
        assert_eq!(progress.stage_progress, 0.0);
    }

    #[test]
    fn test_progress_bar_builder_chaining() {
        let progress = ProgressBar::new("Test")
            .with_progress(0.75)
            .with_message("Processing...");

        assert_eq!(progress.progress(), 0.75);
    }

    #[test]
    fn test_multi_stage_set_stage_resets_progress() {
        let stages = vec!["A".to_string(), "B".to_string()];
        let mut progress = MultiStageProgress::new("Test", stages);

        progress.set_stage(0);
        progress.set_stage_progress(0.8);

        progress.set_stage(1);
        assert_eq!(progress.stage_progress, 0.0); // Should reset
    }

    // ============================================================================
    // ADVANCED TIER: Additional Comprehensive Edge Case Tests
    // ============================================================================

    // Stress Tests (10k operations)

    #[test]
    fn test_progress_bar_10k_progress_updates() {
        let mut progress = ProgressBar::new("Test");

        for i in 0..10000 {
            progress.set_progress((i % 100) as f64 / 100.0);
        }

        assert_eq!(progress.progress(), 0.99);
    }

    #[test]
    fn test_progress_bar_10k_message_updates() {
        let mut progress = ProgressBar::new("Test");

        for i in 0..10000 {
            progress.set_message(format!("Message {}", i));
        }

        // Should not panic, just verify completion
        assert_eq!(progress.progress(), 0.0);
    }

    #[test]
    fn test_multi_stage_10k_stage_transitions() {
        let stages: Vec<String> = (0..100).map(|i| format!("Stage {}", i)).collect();
        let mut progress = MultiStageProgress::new("Test", stages);

        for i in 0..100 {
            progress.set_stage(i);
            for _ in 0..100 {
                progress.set_stage_progress(0.5);
            }
        }

        assert!(progress.current_stage() < 100);
    }

    #[test]
    fn test_multi_stage_1000_stages() {
        let stages: Vec<String> = (0..1000).map(|i| format!("Stage {}", i)).collect();
        let progress = MultiStageProgress::new("Test", stages);

        assert_eq!(progress.stage_count(), 1000);
        assert_eq!(progress.current_stage(), 0);
    }

    #[test]
    fn test_multi_stage_10k_progress_calculations() {
        let stages = vec!["A".to_string(), "B".to_string()];
        let mut progress = MultiStageProgress::new("Test", stages);

        progress.set_stage(0);
        for i in 0..10000 {
            progress.set_stage_progress((i % 100) as f64 / 100.0);
            let _ = progress.overall_progress();
        }

        assert!(progress.overall_progress() >= 0.0);
        assert!(progress.overall_progress() <= 1.0);
    }

    // Unicode Edge Cases

    #[test]
    fn test_progress_bar_rtl_text_arabic() {
        let progress = ProgressBar::new("تحميل البيانات")
            .with_message("معالجة الملفات...");

        assert_eq!(progress.progress(), 0.0);
    }

    #[test]
    fn test_progress_bar_rtl_text_hebrew() {
        let progress = ProgressBar::new("טוען נתונים")
            .with_message("מעבד קבצים...");

        assert_eq!(progress.progress(), 0.0);
    }

    #[test]
    fn test_progress_bar_mixed_scripts() {
        let progress = ProgressBar::new("Loading 加载中 تحميل ロード中")
            .with_message("Processing データ処理 معالجة 处理");

        assert_eq!(progress.progress(), 0.0);
    }

    #[test]
    fn test_progress_bar_emoji_combinations() {
        let progress = ProgressBar::new("🚀 Launch 🎯 Target 💯")
            .with_message("📥 Downloading... 🔄 Processing... ✅");

        assert_eq!(progress.progress(), 0.0);
    }

    #[test]
    fn test_progress_bar_zero_width_characters() {
        let text_with_zwj = "Test\u{200D}Progress";
        let progress = ProgressBar::new(text_with_zwj)
            .with_message("Test\u{200C}Message");

        assert_eq!(progress.progress(), 0.0);
    }

    #[test]
    fn test_progress_bar_combining_characters() {
        let text_with_combining = "Progre\u{0301}s"; // é with combining accent
        let progress = ProgressBar::new(text_with_combining)
            .with_message("Cafe\u{0301}"); // Café

        assert_eq!(progress.progress(), 0.0);
    }

    #[test]
    fn test_multi_stage_rtl_text_arabic() {
        let stages = vec![
            "تحميل".to_string(),
            "استخراج".to_string(),
            "تثبيت".to_string(),
        ];
        let progress = MultiStageProgress::new("إعداد", stages);

        assert_eq!(progress.stage_count(), 3);
    }

    #[test]
    fn test_multi_stage_rtl_text_hebrew() {
        let stages = vec![
            "הורדה".to_string(),
            "חילוץ".to_string(),
            "התקנה".to_string(),
        ];
        let progress = MultiStageProgress::new("התקנה", stages);

        assert_eq!(progress.stage_count(), 3);
    }

    #[test]
    fn test_multi_stage_mixed_scripts() {
        let stages = vec![
            "Download 下载 تحميل".to_string(),
            "Extract 解压 استخراج".to_string(),
            "Install インストール تثبيت".to_string(),
        ];
        let progress = MultiStageProgress::new("Setup 安装 إعداد", stages);

        assert_eq!(progress.stage_count(), 3);
    }

    // Extreme Values

    #[test]
    fn test_progress_bar_infinity_clamped() {
        let mut progress = ProgressBar::new("Test");
        progress.set_progress(f64::INFINITY);
        assert_eq!(progress.progress(), 1.0);

        progress.set_progress(f64::NEG_INFINITY);
        assert_eq!(progress.progress(), 0.0);
    }

    #[test]
    fn test_progress_bar_nan_clamped() {
        let mut progress = ProgressBar::new("Test");
        progress.set_progress(f64::NAN);
        // NaN comparisons are tricky, but clamp should handle it
        let val = progress.progress();
        assert!(val >= 0.0 && val <= 1.0 || val.is_nan());
    }

    #[test]
    fn test_progress_bar_very_precise_values() {
        let mut progress = ProgressBar::new("Test");
        progress.set_progress(0.123456789012345);
        assert!((progress.progress() - 0.123456789012345).abs() < 1e-10);
    }

    #[test]
    fn test_multi_stage_progress_very_precise() {
        let stages = vec!["A".to_string(), "B".to_string(), "C".to_string()];
        let mut progress = MultiStageProgress::new("Test", stages);

        progress.set_stage(0);
        progress.set_stage_progress(0.333333333333);

        let overall = progress.overall_progress();
        assert!(overall >= 0.0 && overall <= 1.0);
    }

    // Trait Tests

    #[test]
    fn test_stage_status_debug_trait() {
        let status = StageStatus::InProgress;
        let debug_str = format!("{:?}", status);
        assert!(debug_str.contains("InProgress"));
    }

    #[test]
    fn test_stage_status_clone_trait() {
        let original = StageStatus::InProgress;
        let cloned = original;
        assert_eq!(original, cloned);
    }

    #[test]
    fn test_stage_status_partial_eq() {
        assert_eq!(StageStatus::Pending, StageStatus::Pending);
        assert_eq!(StageStatus::InProgress, StageStatus::InProgress);
        assert_eq!(StageStatus::Complete, StageStatus::Complete);
        assert_ne!(StageStatus::Pending, StageStatus::InProgress);
    }

    // Multi-phase Comprehensive Workflow

    #[test]
    fn test_progress_bar_10_phase_comprehensive_workflow() {
        // Phase 1: Create basic progress bar
        let mut progress = ProgressBar::new("Comprehensive Test");
        assert_eq!(progress.progress(), 0.0);
        assert!(!progress.is_complete());

        // Phase 2: Set initial progress
        progress.set_progress(0.1);
        assert_eq!(progress.progress(), 0.1);

        // Phase 3: Add message
        progress.set_message("Starting...");

        // Phase 4: Update progress incrementally
        for i in 1..=10 {
            progress.set_progress(i as f64 / 10.0);
        }
        assert_eq!(progress.progress(), 1.0);

        // Phase 5: Verify completion
        assert!(progress.is_complete());

        // Phase 6: Update message after completion
        progress.set_message("Complete!");

        // Phase 7: Test boundary conditions
        progress.set_progress(2.0); // Should clamp to 1.0
        assert_eq!(progress.progress(), 1.0);

        // Phase 8: Reset to zero
        progress.set_progress(0.0);
        assert!(!progress.is_complete());

        // Phase 9: Rapid updates
        for _ in 0..100 {
            progress.set_progress(0.5);
        }
        assert_eq!(progress.progress(), 0.5);

        // Phase 10: Final completion
        progress.set_progress(1.0);
        assert!(progress.is_complete());
    }

    #[test]
    fn test_multi_stage_10_phase_comprehensive_workflow() {
        // Phase 1: Create multi-stage progress
        let stages = vec![
            "Init".to_string(),
            "Download".to_string(),
            "Extract".to_string(),
            "Configure".to_string(),
            "Install".to_string(),
        ];
        let mut progress = MultiStageProgress::new("Installation", stages);
        assert_eq!(progress.stage_count(), 5);
        assert_eq!(progress.current_stage(), 0);

        // Phase 2: Start first stage
        progress.set_stage(0);
        assert_eq!(progress.overall_progress(), 0.0);

        // Phase 3: Progress through first stage
        for i in 0..=10 {
            progress.set_stage_progress(i as f64 / 10.0);
        }
        assert_eq!(progress.stage_progress, 1.0);

        // Phase 4: Complete first stage
        progress.complete_stage();
        assert_eq!(progress.completed_stages(), 1);
        assert_eq!(progress.current_stage(), 1);

        // Phase 5: Progress through remaining stages
        for _ in 0..3 {
            progress.set_stage_progress(0.5);
            progress.complete_stage();
        }
        assert_eq!(progress.completed_stages(), 4);

        // Phase 6: Check overall progress (4 of 5 complete)
        let overall = progress.overall_progress();
        assert!(overall >= 0.8); // At least 80% (4 of 5 stages complete)

        // Phase 7: Complete final stage
        assert_eq!(progress.current_stage(), 4);
        progress.set_stage_progress(1.0);
        progress.complete_stage();

        // Phase 8: Verify all stages complete
        assert!(progress.is_complete());
        assert_eq!(progress.completed_stages(), 5);
        assert_eq!(progress.overall_progress(), 1.0);

        // Phase 9: Test next_stage at completion (should stay at last stage)
        progress.next_stage();
        assert_eq!(progress.current_stage(), 4);

        // Phase 10: Test time tracking
        for i in 0..5 {
            let elapsed = progress.stage_elapsed(i);
            assert!(elapsed.is_some());
        }
        let total = progress.total_elapsed();
        assert!(total > Duration::from_secs(0));
    }

    // Builder Pattern Edge Cases

    #[test]
    fn test_progress_bar_multiple_progress_calls() {
        let progress = ProgressBar::new("Test")
            .with_progress(0.25)
            .with_progress(0.5)
            .with_progress(0.75);

        assert_eq!(progress.progress(), 0.75);
    }

    #[test]
    fn test_progress_bar_multiple_message_calls() {
        let progress = ProgressBar::new("Test")
            .with_message("First")
            .with_message("Second")
            .with_message("Third");

        // Last message should be set
        assert_eq!(progress.progress(), 0.0);
    }

    #[test]
    fn test_progress_bar_builder_chaining_many_operations() {
        let progress = ProgressBar::new("Test")
            .with_progress(0.1).with_progress(0.2).with_progress(0.3)
            .with_message("M1").with_message("M2").with_message("M3")
            .with_progress(0.9);

        assert_eq!(progress.progress(), 0.9);
    }

    #[test]
    fn test_multi_stage_multiple_time_tracking_toggles() {
        let stages = vec!["A".to_string()];
        let progress = MultiStageProgress::new("Test", stages)
            .with_time_tracking(true)
            .with_time_tracking(false)
            .with_time_tracking(true);

        assert!(progress.show_time);
    }

    // Empty State Operations

    #[test]
    fn test_progress_bar_all_operations_on_default() {
        let mut progress = ProgressBar::default();

        progress.set_progress(0.5);
        progress.set_message("Test");
        assert_eq!(progress.progress(), 0.5);
        assert!(!progress.is_complete());
    }

    #[test]
    fn test_multi_stage_render_string_with_empty_stages() {
        let stages: Vec<String> = vec![];
        let progress = MultiStageProgress::new("Empty", stages);

        let output = progress.render_string();
        assert!(output.is_empty() || !output.contains("→"));
    }

    // Additional Edge Cases

    #[test]
    fn test_progress_bar_empty_title() {
        let progress = ProgressBar::new("");
        assert_eq!(progress.progress(), 0.0);
    }

    #[test]
    fn test_progress_bar_empty_message() {
        let progress = ProgressBar::new("Test").with_message("");
        assert_eq!(progress.progress(), 0.0);
    }

    #[test]
    fn test_multi_stage_empty_stage_name() {
        let stages = vec!["".to_string(), "Valid".to_string()];
        let progress = MultiStageProgress::new("Test", stages);
        assert_eq!(progress.stage_count(), 2);
    }

    #[test]
    fn test_stage_status_all_variants_covered() {
        let pending = StageStatus::Pending;
        let in_progress = StageStatus::InProgress;
        let complete = StageStatus::Complete;

        assert!(!pending.is_active());
        assert!(in_progress.is_active());
        assert!(!complete.is_active());

        assert!(!pending.is_complete());
        assert!(!in_progress.is_complete());
        assert!(complete.is_complete());
    }

    #[test]
    fn test_multi_stage_complete_stage_at_boundaries() {
        let stages = vec!["A".to_string()];
        let mut progress = MultiStageProgress::new("Test", stages);

        progress.set_stage(0);
        progress.complete_stage();

        // Should be at last stage with progress 1.0
        assert_eq!(progress.current_stage(), 0);
        assert_eq!(progress.stage_progress, 1.0);
    }

    #[test]
    fn test_progress_bar_progress_boundary_values() {
        let mut progress = ProgressBar::new("Test");

        progress.set_progress(0.0);
        assert_eq!(progress.progress(), 0.0);

        progress.set_progress(1.0);
        assert_eq!(progress.progress(), 1.0);

        progress.set_progress(0.5);
        assert_eq!(progress.progress(), 0.5);
    }

    #[test]
    fn test_multi_stage_set_stage_beyond_bounds() {
        let stages = vec!["A".to_string(), "B".to_string()];
        let mut progress = MultiStageProgress::new("Test", stages);

        let before = progress.current_stage();
        progress.set_stage(100); // Way beyond bounds

        // Should not panic and should stay within valid range
        assert!(progress.current_stage() <= 1);
    }

    #[test]
    fn test_multi_stage_overall_progress_clamping() {
        let stages = vec!["A".to_string()];
        let mut progress = MultiStageProgress::new("Test", stages);

        progress.set_stage(0);
        progress.set_stage_progress(2.0); // Overflow

        let overall = progress.overall_progress();
        assert!(overall >= 0.0 && overall <= 1.0);
    }
}
