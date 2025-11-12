//! Multi-step progress tracking widget
//!
//! Displays progress for operations with multiple sequential steps, showing
//! overall progress, individual step status, timing information, and cancellation support.
//!
//! # Features
//!
//! - Overall progress bar with percentage
//! - Step-by-step status tracking (✓ Complete, ⟳ Running, ⏳ Queued, ❌ Failed)
//! - Elapsed time and ETA calculation
//! - Current activity display
//! - Cancellation support
//! - Resumable from last completed step
//! - Per-step progress percentage
//!
//! # Examples
//!
//! ```
//! use toad::ui::widgets::progress::MultiStepProgress;
//!
//! let steps = vec![
//!     "Analyzing code structure".to_string(),
//!     "Running tests".to_string(),
//!     "Generating report".to_string(),
//! ];
//! let mut progress = MultiStepProgress::new(steps);
//! progress.start_step(0);
//! progress.update_step_progress(0, 50);
//! ```

use crate::ui::atoms::Block;
use crate::ui::theme::ToadTheme;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Gauge, List, ListItem, ListState, Paragraph},
    Frame,
};
use std::time::{Duration, Instant};

/// Status of an individual step
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepStatus {
    /// Step is queued, not yet started
    Queued,
    /// Step is currently running
    Running,
    /// Step completed successfully
    Complete,
    /// Step failed with error
    Failed,
}

impl StepStatus {
    /// Get color for status
    pub fn color(&self) -> Color {
        match self {
            StepStatus::Queued => ToadTheme::GRAY,
            StepStatus::Running => ToadTheme::BLUE,
            StepStatus::Complete => ToadTheme::TOAD_GREEN,
            StepStatus::Failed => ToadTheme::ERROR,
        }
    }

    /// Get symbol for status
    pub fn symbol(&self) -> &'static str {
        match self {
            StepStatus::Queued => "⏳",
            StepStatus::Running => "⟳",
            StepStatus::Complete => "✓",
            StepStatus::Failed => "❌",
        }
    }

    /// Get text description
    pub fn text(&self) -> &'static str {
        match self {
            StepStatus::Queued => "Queued",
            StepStatus::Running => "Running",
            StepStatus::Complete => "Complete",
            StepStatus::Failed => "Failed",
        }
    }
}

/// A single step in the multi-step operation
#[derive(Debug, Clone)]
pub struct Step {
    /// Step description
    pub description: String,
    /// Current status
    pub status: StepStatus,
    /// Progress percentage (0-100) if running
    pub progress: u8,
    /// Start time if running or completed
    pub start_time: Option<Instant>,
    /// End time if completed or failed
    pub end_time: Option<Instant>,
    /// Error message if failed
    pub error: Option<String>,
    /// Current activity within this step
    pub activity: Option<String>,
}

impl Step {
    /// Create a new queued step
    pub fn new(description: String) -> Self {
        Self {
            description,
            status: StepStatus::Queued,
            progress: 0,
            start_time: None,
            end_time: None,
            error: None,
            activity: None,
        }
    }

    /// Start the step
    pub fn start(&mut self) {
        self.status = StepStatus::Running;
        self.start_time = Some(Instant::now());
        self.progress = 0;
    }

    /// Update progress percentage
    pub fn update_progress(&mut self, progress: u8) {
        self.progress = progress.min(100);
    }

    /// Set current activity
    pub fn set_activity(&mut self, activity: impl Into<String>) {
        self.activity = Some(activity.into());
    }

    /// Complete the step
    pub fn complete(&mut self) {
        self.status = StepStatus::Complete;
        self.end_time = Some(Instant::now());
        self.progress = 100;
    }

    /// Fail the step
    pub fn fail(&mut self, error: impl Into<String>) {
        self.status = StepStatus::Failed;
        self.end_time = Some(Instant::now());
        self.error = Some(error.into());
    }

    /// Get duration if started
    pub fn duration(&self) -> Option<Duration> {
        self.start_time.map(|start| {
            if let Some(end) = self.end_time {
                end.duration_since(start)
            } else {
                start.elapsed()
            }
        })
    }
}

/// Multi-step progress tracker widget
///
/// Displays and tracks progress for operations with multiple sequential steps.
///
/// # Examples
///
/// ```
/// use toad::ui::widgets::progress::MultiStepProgress;
///
/// let steps = vec![
///     "Step 1".to_string(),
///     "Step 2".to_string(),
/// ];
/// let mut progress = MultiStepProgress::new(steps);
/// assert_eq!(progress.step_count(), 2);
/// assert_eq!(progress.overall_progress(), 0);
/// ```
#[derive(Debug)]
pub struct MultiStepProgress {
    /// All steps in the operation
    steps: Vec<Step>,
    /// Current step index
    current_step: usize,
    /// Operation start time
    start_time: Option<Instant>,
    /// Whether operation is cancelled
    cancelled: bool,
    /// List state for rendering
    list_state: ListState,
    /// Operation name
    operation_name: Option<String>,
}

impl MultiStepProgress {
    /// Create a new multi-step progress tracker
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::widgets::progress::MultiStepProgress;
    ///
    /// let steps = vec!["Build".to_string(), "Test".to_string()];
    /// let progress = MultiStepProgress::new(steps);
    /// assert_eq!(progress.step_count(), 2);
    /// ```
    pub fn new(step_descriptions: Vec<String>) -> Self {
        let steps = step_descriptions.into_iter().map(Step::new).collect();

        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Self {
            steps,
            current_step: 0,
            start_time: None,
            cancelled: false,
            list_state,
            operation_name: None,
        }
    }

    /// Set operation name
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.operation_name = Some(name.into());
        self
    }

    /// Get number of steps
    pub fn step_count(&self) -> usize {
        self.steps.len()
    }

    /// Start the operation
    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
        if !self.steps.is_empty() {
            self.steps[0].start();
        }
    }

    /// Start a specific step
    pub fn start_step(&mut self, index: usize) {
        if let Some(step) = self.steps.get_mut(index) {
            step.start();
            self.current_step = index;
            self.list_state.select(Some(index));
        }
    }

    /// Update progress for current step
    pub fn update_step_progress(&mut self, step_index: usize, progress: u8) {
        if let Some(step) = self.steps.get_mut(step_index) {
            step.update_progress(progress);
        }
    }

    /// Set activity for current step
    pub fn set_step_activity(&mut self, step_index: usize, activity: impl Into<String>) {
        if let Some(step) = self.steps.get_mut(step_index) {
            step.set_activity(activity);
        }
    }

    /// Complete current step and move to next
    pub fn complete_step(&mut self, step_index: usize) {
        if let Some(step) = self.steps.get_mut(step_index) {
            step.complete();
        }

        // Auto-advance to next step
        if step_index + 1 < self.steps.len() {
            self.start_step(step_index + 1);
        }
    }

    /// Fail current step
    pub fn fail_step(&mut self, step_index: usize, error: impl Into<String>) {
        if let Some(step) = self.steps.get_mut(step_index) {
            step.fail(error);
        }
    }

    /// Cancel the operation
    pub fn cancel(&mut self) {
        self.cancelled = true;
    }

    /// Check if cancelled
    pub fn is_cancelled(&self) -> bool {
        self.cancelled
    }

    /// Check if operation is complete (all steps complete)
    pub fn is_complete(&self) -> bool {
        self.steps.iter().all(|s| s.status == StepStatus::Complete)
    }

    /// Check if any step has failed
    pub fn has_failed(&self) -> bool {
        self.steps.iter().any(|s| s.status == StepStatus::Failed)
    }

    /// Get overall progress percentage (0-100)
    pub fn overall_progress(&self) -> u8 {
        if self.steps.is_empty() {
            return 100;
        }

        let total_progress: u32 = self
            .steps
            .iter()
            .map(|s| match s.status {
                StepStatus::Complete => 100,
                StepStatus::Running => s.progress as u32,
                _ => 0,
            })
            .sum();

        (total_progress / self.steps.len() as u32) as u8
    }

    /// Get completed step count
    pub fn completed_count(&self) -> usize {
        self.steps
            .iter()
            .filter(|s| s.status == StepStatus::Complete)
            .count()
    }

    /// Get total elapsed time
    pub fn elapsed_time(&self) -> Option<Duration> {
        self.start_time.map(|start| start.elapsed())
    }

    /// Estimate time remaining based on current progress
    pub fn eta(&self) -> Option<Duration> {
        let elapsed = self.elapsed_time()?;
        let progress = self.overall_progress();

        if progress == 0 {
            return None;
        }

        let total_estimate = elapsed.as_secs_f64() * (100.0 / progress as f64);
        let remaining = total_estimate - elapsed.as_secs_f64();

        Some(Duration::from_secs_f64(remaining.max(0.0)))
    }

    /// Get current step
    pub fn current_step(&self) -> Option<&Step> {
        self.steps.get(self.current_step)
    }

    /// Get step by index
    pub fn step(&self, index: usize) -> Option<&Step> {
        self.steps.get(index)
    }

    /// Get all steps
    pub fn steps(&self) -> &[Step] {
        &self.steps
    }

    /// Get index of last completed step (for resumption)
    pub fn last_completed_step(&self) -> Option<usize> {
        self.steps
            .iter()
            .enumerate()
            .rev()
            .find(|(_, s)| s.status == StepStatus::Complete)
            .map(|(i, _)| i)
    }

    /// Resume from last completed step
    pub fn resume(&mut self) {
        if let Some(last_completed) = self.last_completed_step() {
            let next_step = last_completed + 1;
            if next_step < self.steps.len() {
                self.start_step(next_step);
            }
        } else {
            self.start();
        }
    }

    /// Render the progress tracker
    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header with overall progress
                Constraint::Min(0),    // Step list
                Constraint::Length(3), // Footer with timing
            ])
            .split(area);

        self.render_header(frame, chunks[0]);
        self.render_steps(frame, chunks[1]);
        self.render_footer(frame, chunks[2]);
    }

    /// Render header with overall progress
    fn render_header(&self, frame: &mut Frame, area: Rect) {
        let title = if let Some(ref name) = self.operation_name {
            format!("Progress: {}", name)
        } else {
            "Progress".to_string()
        };

        let block = Block::themed(&title).to_ratatui();
        let inner = block.inner(area);
        frame.render_widget(block, area);

        let progress = self.overall_progress();
        let label = format!("{}%", progress);

        let gauge = Gauge::default()
            .block(ratatui::widgets::Block::default())
            .gauge_style(
                Style::default()
                    .fg(ToadTheme::TOAD_GREEN)
                    .bg(ToadTheme::DARK_GRAY),
            )
            .label(label)
            .ratio(progress as f64 / 100.0);

        frame.render_widget(gauge, inner);
    }

    /// Render step list
    fn render_steps(&mut self, frame: &mut Frame, area: Rect) {
        let block = Block::themed("Steps").to_ratatui();
        let inner = block.inner(area);
        frame.render_widget(block, area);

        let items: Vec<ListItem> = self
            .steps
            .iter()
            .enumerate()
            .map(|(idx, step)| {
                let mut spans = vec![];

                // Step number
                spans.push(Span::styled(
                    format!("{}. ", idx + 1),
                    Style::default().fg(ToadTheme::GRAY),
                ));

                // Status symbol
                spans.push(Span::styled(
                    format!("{} ", step.status.symbol()),
                    Style::default().fg(step.status.color()),
                ));

                // Description
                spans.push(Span::styled(
                    &step.description,
                    Style::default().fg(ToadTheme::FOREGROUND),
                ));

                // Progress percentage if running
                if step.status == StepStatus::Running && step.progress > 0 {
                    spans.push(Span::styled(
                        format!(" ({}%)", step.progress),
                        Style::default().fg(ToadTheme::BLUE),
                    ));
                }

                // Duration
                if let Some(duration) = step.duration() {
                    spans.push(Span::styled(
                        format!(" - {}s", duration.as_secs()),
                        Style::default().fg(ToadTheme::GRAY),
                    ));
                }

                // Activity
                if let Some(ref activity) = step.activity {
                    spans.push(Span::raw("\n    "));
                    spans.push(Span::styled(
                        activity,
                        Style::default()
                            .fg(ToadTheme::GRAY)
                            .add_modifier(Modifier::ITALIC),
                    ));
                }

                // Error
                if let Some(ref error) = step.error {
                    spans.push(Span::raw("\n    "));
                    spans.push(Span::styled(
                        format!("Error: {}", error),
                        Style::default().fg(ToadTheme::ERROR),
                    ));
                }

                ListItem::new(Line::from(spans))
            })
            .collect();

        let list = List::new(items).highlight_style(
            Style::default()
                .bg(ToadTheme::DARK_GRAY)
                .add_modifier(Modifier::BOLD),
        );

        frame.render_stateful_widget(list, inner, &mut self.list_state);
    }

    /// Render footer with timing information
    fn render_footer(&self, frame: &mut Frame, area: Rect) {
        let block = Block::themed("Status").to_ratatui();
        let inner = block.inner(area);
        frame.render_widget(block, area);

        let mut info_parts = vec![];

        // Completed count
        info_parts.push(format!(
            "Completed: {}/{}",
            self.completed_count(),
            self.step_count()
        ));

        // Elapsed time
        if let Some(elapsed) = self.elapsed_time() {
            info_parts.push(format!("Elapsed: {}s", elapsed.as_secs()));
        }

        // ETA
        if let Some(eta) = self.eta() {
            if eta.as_secs() > 0 {
                info_parts.push(format!("ETA: {}s", eta.as_secs()));
            }
        }

        // Cancellation status
        if self.cancelled {
            info_parts.push("CANCELLED".to_string());
        } else if self.has_failed() {
            info_parts.push("FAILED".to_string());
        } else if self.is_complete() {
            info_parts.push("COMPLETE".to_string());
        }

        let info = info_parts.join(" | ");
        let paragraph = Paragraph::new(info);
        frame.render_widget(paragraph, inner);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step_status_symbols() {
        assert_eq!(StepStatus::Queued.symbol(), "⏳");
        assert_eq!(StepStatus::Running.symbol(), "⟳");
        assert_eq!(StepStatus::Complete.symbol(), "✓");
        assert_eq!(StepStatus::Failed.symbol(), "❌");
    }

    #[test]
    fn test_step_creation() {
        let step = Step::new("Test step".to_string());
        assert_eq!(step.description, "Test step");
        assert_eq!(step.status, StepStatus::Queued);
        assert_eq!(step.progress, 0);
        assert!(step.start_time.is_none());
    }

    #[test]
    fn test_step_lifecycle() {
        let mut step = Step::new("Test".to_string());

        step.start();
        assert_eq!(step.status, StepStatus::Running);
        assert!(step.start_time.is_some());

        step.update_progress(50);
        assert_eq!(step.progress, 50);

        step.complete();
        assert_eq!(step.status, StepStatus::Complete);
        assert_eq!(step.progress, 100);
        assert!(step.end_time.is_some());
    }

    #[test]
    fn test_step_failure() {
        let mut step = Step::new("Test".to_string());
        step.start();
        step.fail("Test error");

        assert_eq!(step.status, StepStatus::Failed);
        assert_eq!(step.error, Some("Test error".to_string()));
        assert!(step.end_time.is_some());
    }

    #[test]
    fn test_progress_creation() {
        let steps = vec!["Step 1".to_string(), "Step 2".to_string()];
        let progress = MultiStepProgress::new(steps);

        assert_eq!(progress.step_count(), 2);
        assert_eq!(progress.current_step, 0);
        assert!(!progress.is_cancelled());
    }

    #[test]
    fn test_overall_progress_calculation() {
        let steps = vec![
            "Step 1".to_string(),
            "Step 2".to_string(),
            "Step 3".to_string(),
        ];
        let mut progress = MultiStepProgress::new(steps);

        assert_eq!(progress.overall_progress(), 0);

        progress.start_step(0);
        progress.update_step_progress(0, 50);
        assert_eq!(progress.overall_progress(), 16); // 50/3 ≈ 16

        progress.complete_step(0);
        assert_eq!(progress.overall_progress(), 33); // 100/3 ≈ 33

        progress.complete_step(1);
        assert_eq!(progress.overall_progress(), 66); // 200/3 ≈ 66

        progress.complete_step(2);
        assert_eq!(progress.overall_progress(), 100); // 300/3 = 100
    }

    #[test]
    fn test_completed_count() {
        let steps = vec!["Step 1".to_string(), "Step 2".to_string()];
        let mut progress = MultiStepProgress::new(steps);

        assert_eq!(progress.completed_count(), 0);

        progress.start_step(0);
        progress.complete_step(0);
        assert_eq!(progress.completed_count(), 1);

        progress.complete_step(1);
        assert_eq!(progress.completed_count(), 2);
    }

    #[test]
    fn test_is_complete() {
        let steps = vec!["Step 1".to_string(), "Step 2".to_string()];
        let mut progress = MultiStepProgress::new(steps);

        assert!(!progress.is_complete());

        progress.start_step(0);
        progress.complete_step(0);
        assert!(!progress.is_complete());

        progress.complete_step(1);
        assert!(progress.is_complete());
    }

    #[test]
    fn test_has_failed() {
        let steps = vec!["Step 1".to_string(), "Step 2".to_string()];
        let mut progress = MultiStepProgress::new(steps);

        assert!(!progress.has_failed());

        progress.start_step(0);
        progress.fail_step(0, "Test error");
        assert!(progress.has_failed());
    }

    #[test]
    fn test_cancellation() {
        let steps = vec!["Step 1".to_string()];
        let mut progress = MultiStepProgress::new(steps);

        assert!(!progress.is_cancelled());

        progress.cancel();
        assert!(progress.is_cancelled());
    }

    #[test]
    fn test_last_completed_step() {
        let steps = vec![
            "Step 1".to_string(),
            "Step 2".to_string(),
            "Step 3".to_string(),
        ];
        let mut progress = MultiStepProgress::new(steps);

        assert_eq!(progress.last_completed_step(), None);

        progress.start_step(0);
        progress.complete_step(0);
        assert_eq!(progress.last_completed_step(), Some(0));

        progress.complete_step(1);
        assert_eq!(progress.last_completed_step(), Some(1));
    }

    #[test]
    fn test_with_name() {
        let steps = vec!["Step 1".to_string()];
        let progress = MultiStepProgress::new(steps).with_name("My Operation");
        assert_eq!(progress.operation_name, Some("My Operation".to_string()));
    }

    #[test]
    fn test_step_activity() {
        let mut step = Step::new("Test".to_string());
        step.set_activity("Processing file.rs");
        assert_eq!(step.activity, Some("Processing file.rs".to_string()));
    }

    #[test]
    fn test_update_progress_clamping() {
        let mut step = Step::new("Test".to_string());
        step.start();
        step.update_progress(150);
        assert_eq!(step.progress, 100); // Clamped to 100
    }

    #[test]
    fn test_empty_steps() {
        let progress = MultiStepProgress::new(vec![]);
        assert_eq!(progress.step_count(), 0);
        assert_eq!(progress.overall_progress(), 100); // Empty = 100% complete
    }
}
