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

use crate::theme::ToadTheme;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Gauge, Widget},
    Frame,
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
        let stages = stage_names
            .into_iter()
            .map(StageInfo::new)
            .collect();

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
        self.stages
            .iter()
            .filter_map(|s| s.elapsed())
            .sum()
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
        let progress = MultiStageProgress::new("Test", stages)
            .with_time_tracking(true);
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
}
