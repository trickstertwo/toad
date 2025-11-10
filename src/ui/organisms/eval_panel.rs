//! EvalPanel organism - Evaluation progress display
//!
//! Composes MetricCard, TaskItem, and ProgressBar molecules into a comprehensive
//! evaluation progress panel.
//!
//! # Architecture
//!
//! Following Atomic Design principles:
//! - **Organism**: Composes multiple molecules (MetricCard, TaskItem, ProgressBar)
//! - **Complex**: Displays complete evaluation status
//! - **Stateful**: Holds evaluation progress data
//! - **Composable**: Used by evaluation screen
//!
//! # Examples
//!
//! ```
//! use toad::ui::organisms::eval_panel::EvalPanel;
//!
//! let panel = EvalPanel::new()
//!     .current_task(5)
//!     .total_tasks(10)
//!     .accuracy(85.2)
//!     .cost(0.45)
//!     .duration_secs(120);
//!
//! let widget = panel.render(area, buf);
//! ```

use crate::ui::atoms::{Block, Icon};
use crate::ui::molecules::{MetricCard, ProgressBar, TaskItem};
use crate::ui::nerd_fonts::UiIcon;
use crate::ui::theme::ToadTheme;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    text::Line,
    widgets::{Paragraph, Widget},
};

/// Evaluation progress panel
///
/// Composes molecules to display comprehensive evaluation status including:
/// - Overall progress bar
/// - Metrics (accuracy, cost, duration)
/// - Task list with status
///
/// # Examples
///
/// ```
/// use toad::ui::organisms::eval_panel::EvalPanel;
///
/// let panel = EvalPanel::new()
///     .current_task(5)
///     .total_tasks(10);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct EvalPanel {
    /// Current task number
    pub current_task: usize,
    /// Total number of tasks
    pub total_tasks: usize,
    /// Accuracy percentage (0.0 - 100.0)
    pub accuracy: Option<f64>,
    /// Cost in USD
    pub cost: Option<f64>,
    /// Duration in seconds
    pub duration_secs: Option<u64>,
    /// List of task statuses
    pub tasks: Vec<TaskStatus>,
    /// Panel title
    pub title: String,
}

/// Task status for display
#[derive(Debug, Clone, PartialEq)]
pub struct TaskStatus {
    /// Task name/ID
    pub name: String,
    /// Task status
    pub status: TaskState,
    /// Optional status message
    pub message: Option<String>,
}

/// Task state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskState {
    /// Task is pending
    Pending,
    /// Task is in progress
    InProgress,
    /// Task completed successfully
    Completed,
    /// Task failed
    Failed,
}

impl EvalPanel {
    /// Create a new evaluation panel
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::organisms::eval_panel::EvalPanel;
    ///
    /// let panel = EvalPanel::new();
    /// ```
    pub fn new() -> Self {
        Self {
            current_task: 0,
            total_tasks: 0,
            accuracy: None,
            cost: None,
            duration_secs: None,
            tasks: Vec::new(),
            title: "Evaluation Progress".to_string(),
        }
    }

    /// Set the panel title
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::organisms::eval_panel::EvalPanel;
    ///
    /// let panel = EvalPanel::new().title("M1 Baseline Evaluation");
    /// ```
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Set current task number
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::organisms::eval_panel::EvalPanel;
    ///
    /// let panel = EvalPanel::new().current_task(5);
    /// ```
    pub fn current_task(mut self, current: usize) -> Self {
        self.current_task = current;
        self
    }

    /// Set total number of tasks
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::organisms::eval_panel::EvalPanel;
    ///
    /// let panel = EvalPanel::new().total_tasks(10);
    /// ```
    pub fn total_tasks(mut self, total: usize) -> Self {
        self.total_tasks = total;
        self
    }

    /// Set accuracy percentage
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::organisms::eval_panel::EvalPanel;
    ///
    /// let panel = EvalPanel::new().accuracy(85.2);
    /// ```
    pub fn accuracy(mut self, accuracy: f64) -> Self {
        self.accuracy = Some(accuracy);
        self
    }

    /// Set cost in USD
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::organisms::eval_panel::EvalPanel;
    ///
    /// let panel = EvalPanel::new().cost(0.45);
    /// ```
    pub fn cost(mut self, cost: f64) -> Self {
        self.cost = Some(cost);
        self
    }

    /// Set duration in seconds
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::organisms::eval_panel::EvalPanel;
    ///
    /// let panel = EvalPanel::new().duration_secs(120);
    /// ```
    pub fn duration_secs(mut self, secs: u64) -> Self {
        self.duration_secs = Some(secs);
        self
    }

    /// Add a task status
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::organisms::eval_panel::{EvalPanel, TaskStatus, TaskState};
    ///
    /// let panel = EvalPanel::new().add_task(TaskStatus {
    ///     name: "Task 1".to_string(),
    ///     status: TaskState::Completed,
    ///     message: Some("Success".to_string()),
    /// });
    /// ```
    pub fn add_task(mut self, task: TaskStatus) -> Self {
        self.tasks.push(task);
        self
    }

    /// Set all tasks
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::organisms::eval_panel::{EvalPanel, TaskStatus, TaskState};
    ///
    /// let tasks = vec![
    ///     TaskStatus {
    ///         name: "Task 1".to_string(),
    ///         status: TaskState::Completed,
    ///         message: None,
    ///     },
    /// ];
    /// let panel = EvalPanel::new().tasks(tasks);
    /// ```
    pub fn tasks(mut self, tasks: Vec<TaskStatus>) -> Self {
        self.tasks = tasks;
        self
    }

    /// Render the panel to a buffer
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::organisms::eval_panel::EvalPanel;
    /// use ratatui::{buffer::Buffer, layout::Rect};
    ///
    /// let panel = EvalPanel::new().current_task(5).total_tasks(10);
    /// let area = Rect::new(0, 0, 80, 24);
    /// let mut buf = Buffer::empty(area);
    /// panel.render(area, &mut buf);
    /// ```
    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        // Create border
        let block = Block::themed(&self.title).to_ratatui();
        let inner = block.inner(area);
        block.render(area, buf);

        // Split layout: progress bar | metrics | tasks
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // Progress bar
                Constraint::Length(1), // Spacer
                Constraint::Length(3), // Metrics
                Constraint::Length(1), // Spacer
                Constraint::Min(0),    // Tasks
            ])
            .split(inner);

        // Render progress bar
        self.render_progress(chunks[0], buf);

        // Render metrics
        self.render_metrics(chunks[2], buf);

        // Render tasks
        self.render_tasks(chunks[4], buf);
    }

    /// Render progress bar
    fn render_progress(&self, area: Rect, buf: &mut Buffer) {
        let progress = if self.total_tasks > 0 {
            ProgressBar::new("Progress", self.current_task, self.total_tasks)
                .width(20)
                .bar_style(Style::default().fg(ToadTheme::TOAD_GREEN))
                .label_style(Style::default().fg(ToadTheme::WHITE))
        } else {
            ProgressBar::new("Progress", 0, 1)
                .width(20)
                .bar_style(Style::default().fg(ToadTheme::GRAY))
                .label_style(Style::default().fg(ToadTheme::GRAY))
        };

        let line = progress.to_line();
        Paragraph::new(line).render(area, buf);
    }

    /// Render metrics
    fn render_metrics(&self, area: Rect, buf: &mut Buffer) {
        let mut lines = Vec::new();

        // Accuracy
        if let Some(acc) = self.accuracy {
            let card = MetricCard::new("Accuracy", format!("{:.1}%", acc))
                .icon(Icon::ui(UiIcon::Success).style(Style::default().fg(ToadTheme::TOAD_GREEN)))
                .label_style(Style::default().fg(ToadTheme::GRAY))
                .value_style(Style::default().fg(ToadTheme::TOAD_GREEN));
            lines.push(card.to_line());
        }

        // Cost
        if let Some(cost) = self.cost {
            let card = MetricCard::new("Cost", format!("${:.3}", cost))
                .icon(Icon::ui(UiIcon::Tag).style(Style::default().fg(ToadTheme::YELLOW)))
                .label_style(Style::default().fg(ToadTheme::GRAY))
                .value_style(Style::default().fg(ToadTheme::WHITE));
            lines.push(card.to_line());
        }

        // Duration
        if let Some(secs) = self.duration_secs {
            let duration_str = if secs < 60 {
                format!("{}s", secs)
            } else {
                format!("{}m {}s", secs / 60, secs % 60)
            };
            let card = MetricCard::new("Duration", duration_str)
                .icon(Icon::ui(UiIcon::Clock).style(Style::default().fg(ToadTheme::BLUE)))
                .label_style(Style::default().fg(ToadTheme::GRAY))
                .value_style(Style::default().fg(ToadTheme::WHITE));
            lines.push(card.to_line());
        }

        Paragraph::new(lines).render(area, buf);
    }

    /// Render task list
    fn render_tasks(&self, area: Rect, buf: &mut Buffer) {
        let lines: Vec<Line> = self
            .tasks
            .iter()
            .map(|task| {
                let item = match task.status {
                    TaskState::Pending => TaskItem::pending(&task.name),
                    TaskState::InProgress => TaskItem::in_progress(&task.name),
                    TaskState::Completed => TaskItem::completed(&task.name),
                    TaskState::Failed => TaskItem::failed(&task.name),
                };

                if let Some(ref msg) = task.message {
                    item.status(msg).to_line()
                } else {
                    item.to_line()
                }
            })
            .collect();

        Paragraph::new(lines).render(area, buf);
    }
}

impl Default for EvalPanel {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for EvalPanel {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.render(area, buf);
    }
}

impl Widget for &EvalPanel {
    fn render(self, area: Rect, buf: &mut Buffer) {
        EvalPanel::render(self, area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_panel_new() {
        let panel = EvalPanel::new();
        assert_eq!(panel.current_task, 0);
        assert_eq!(panel.total_tasks, 0);
        assert_eq!(panel.accuracy, None);
        assert_eq!(panel.cost, None);
        assert_eq!(panel.duration_secs, None);
    }

    #[test]
    fn test_eval_panel_title() {
        let panel = EvalPanel::new().title("Test Evaluation");
        assert_eq!(panel.title, "Test Evaluation");
    }

    #[test]
    fn test_eval_panel_current_task() {
        let panel = EvalPanel::new().current_task(5);
        assert_eq!(panel.current_task, 5);
    }

    #[test]
    fn test_eval_panel_total_tasks() {
        let panel = EvalPanel::new().total_tasks(10);
        assert_eq!(panel.total_tasks, 10);
    }

    #[test]
    fn test_eval_panel_accuracy() {
        let panel = EvalPanel::new().accuracy(85.2);
        assert_eq!(panel.accuracy, Some(85.2));
    }

    #[test]
    fn test_eval_panel_cost() {
        let panel = EvalPanel::new().cost(0.45);
        assert_eq!(panel.cost, Some(0.45));
    }

    #[test]
    fn test_eval_panel_duration() {
        let panel = EvalPanel::new().duration_secs(120);
        assert_eq!(panel.duration_secs, Some(120));
    }

    #[test]
    fn test_eval_panel_add_task() {
        let panel = EvalPanel::new().add_task(TaskStatus {
            name: "Task 1".to_string(),
            status: TaskState::Completed,
            message: None,
        });
        assert_eq!(panel.tasks.len(), 1);
    }

    #[test]
    fn test_eval_panel_tasks() {
        let tasks = vec![
            TaskStatus {
                name: "Task 1".to_string(),
                status: TaskState::Completed,
                message: None,
            },
            TaskStatus {
                name: "Task 2".to_string(),
                status: TaskState::InProgress,
                message: Some("Running".to_string()),
            },
        ];
        let panel = EvalPanel::new().tasks(tasks.clone());
        assert_eq!(panel.tasks, tasks);
    }

    #[test]
    fn test_eval_panel_chaining() {
        let panel = EvalPanel::new()
            .title("Test")
            .current_task(5)
            .total_tasks(10)
            .accuracy(85.0)
            .cost(0.5)
            .duration_secs(100);

        assert_eq!(panel.title, "Test");
        assert_eq!(panel.current_task, 5);
        assert_eq!(panel.total_tasks, 10);
        assert_eq!(panel.accuracy, Some(85.0));
        assert_eq!(panel.cost, Some(0.5));
        assert_eq!(panel.duration_secs, Some(100));
    }

    #[test]
    fn test_eval_panel_render() {
        let panel = EvalPanel::new()
            .current_task(5)
            .total_tasks(10)
            .accuracy(85.0)
            .cost(0.45)
            .duration_secs(120);

        let area = Rect::new(0, 0, 80, 24);
        let mut buf = Buffer::empty(area);
        panel.render(area, &mut buf);
        // Just verify it doesn't panic
    }

    #[test]
    fn test_eval_panel_render_with_tasks() {
        let tasks = vec![
            TaskStatus {
                name: "Task 1".to_string(),
                status: TaskState::Completed,
                message: Some("Success".to_string()),
            },
            TaskStatus {
                name: "Task 2".to_string(),
                status: TaskState::InProgress,
                message: None,
            },
        ];

        let panel = EvalPanel::new().tasks(tasks);
        let area = Rect::new(0, 0, 80, 24);
        let mut buf = Buffer::empty(area);
        panel.render(area, &mut buf);
        // Just verify it doesn't panic
    }

    #[test]
    fn test_eval_panel_widget_trait() {
        let panel = EvalPanel::new();
        let area = Rect::new(0, 0, 80, 24);
        let mut buf = Buffer::empty(area);
        // Test both owned and borrowed widget rendering
        panel.clone().render(area, &mut buf);
        (&panel).render(area, &mut buf);
    }

    #[test]
    fn test_eval_panel_default() {
        let panel = EvalPanel::default();
        assert_eq!(panel.current_task, 0);
        assert_eq!(panel.total_tasks, 0);
    }

    #[test]
    fn test_task_status() {
        let status = TaskStatus {
            name: "Test".to_string(),
            status: TaskState::InProgress,
            message: Some("Running".to_string()),
        };
        assert_eq!(status.name, "Test");
        assert_eq!(status.status, TaskState::InProgress);
        assert_eq!(status.message, Some("Running".to_string()));
    }

    #[test]
    fn test_task_state_equality() {
        assert_eq!(TaskState::Pending, TaskState::Pending);
        assert_ne!(TaskState::Pending, TaskState::InProgress);
    }

    #[test]
    fn test_eval_panel_clone() {
        let panel1 = EvalPanel::new().current_task(5);
        let panel2 = panel1.clone();
        assert_eq!(panel1, panel2);
    }

    #[test]
    fn test_eval_panel_empty_render() {
        let panel = EvalPanel::new();
        let area = Rect::new(0, 0, 80, 24);
        let mut buf = Buffer::empty(area);
        panel.render(area, &mut buf);
        // Verify empty panel renders without panic
    }
}
