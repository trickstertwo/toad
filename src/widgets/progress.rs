//! Progress bar widget
//!
//! For showing progress of long-running operations

use crate::theme::ToadTheme;
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    widgets::{Block, Borders, Gauge},
    Frame,
};

/// Progress bar widget
pub struct ProgressBar {
    title: String,
    progress: f64, // 0.0 to 1.0
    message: Option<String>,
}

impl ProgressBar {
    /// Create a new progress bar
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            progress: 0.0,
            message: None,
        }
    }

    /// Set the progress (0.0 to 1.0)
    pub fn with_progress(mut self, progress: f64) -> Self {
        self.progress = progress.clamp(0.0, 1.0);
        self
    }

    /// Set the progress message
    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }

    /// Update the progress
    pub fn set_progress(&mut self, progress: f64) {
        self.progress = progress.clamp(0.0, 1.0);
    }

    /// Set the message
    pub fn set_message(&mut self, message: impl Into<String>) {
        self.message = Some(message.into());
    }

    /// Get current progress
    pub fn progress(&self) -> f64 {
        self.progress
    }

    /// Check if complete
    pub fn is_complete(&self) -> bool {
        self.progress >= 1.0
    }

    /// Render the progress bar
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

/// Multi-stage progress bar
pub struct MultiStageProgress {
    title: String,
    stages: Vec<String>,
    current_stage: usize,
    stage_progress: f64, // Progress within current stage
}

impl MultiStageProgress {
    /// Create a new multi-stage progress bar
    pub fn new(title: impl Into<String>, stages: Vec<String>) -> Self {
        Self {
            title: title.into(),
            stages,
            current_stage: 0,
            stage_progress: 0.0,
        }
    }

    /// Set the current stage
    pub fn set_stage(&mut self, stage: usize) {
        self.current_stage = stage.min(self.stages.len().saturating_sub(1));
        self.stage_progress = 0.0;
    }

    /// Set the progress within the current stage
    pub fn set_stage_progress(&mut self, progress: f64) {
        self.stage_progress = progress.clamp(0.0, 1.0);
    }

    /// Move to the next stage
    pub fn next_stage(&mut self) {
        if self.current_stage < self.stages.len().saturating_sub(1) {
            self.current_stage += 1;
            self.stage_progress = 0.0;
        }
    }

    /// Get overall progress
    pub fn overall_progress(&self) -> f64 {
        if self.stages.is_empty() {
            return 0.0;
        }

        let stages_complete = self.current_stage as f64;
        let current_contribution = self.stage_progress / self.stages.len() as f64;
        let total = (stages_complete + current_contribution) / self.stages.len() as f64;

        total.clamp(0.0, 1.0)
    }

    /// Check if all stages are complete
    pub fn is_complete(&self) -> bool {
        self.current_stage >= self.stages.len().saturating_sub(1) && self.stage_progress >= 1.0
    }

    /// Render the multi-stage progress bar
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let current_stage_name = self
            .stages
            .get(self.current_stage)
            .map(|s| s.as_str())
            .unwrap_or("Unknown");

        let label = format!(
            "Stage {}/{}: {} ({:.0}%)",
            self.current_stage + 1,
            self.stages.len(),
            current_stage_name,
            self.stage_progress * 100.0
        );

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
}
