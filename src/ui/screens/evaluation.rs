//! Evaluation screen - Real-time evaluation progress display
//!
//! The top-level screen for displaying evaluation progress using the EvalPanel organism.
//!
//! # Architecture
//!
//! Following Atomic Design principles:
//! - **Screen**: Top-level layout composing organisms
//! - **Stateful**: Manages evaluation state and updates
//! - **Interactive**: Responds to keyboard input
//!
//! # Examples
//!
//! ```
//! use toad::ui::screens::evaluation::EvaluationScreen;
//! use ratatui::{buffer::Buffer, layout::Rect};
//!
//! let mut screen = EvaluationScreen::new("M1 Baseline Evaluation");
//! screen.update_progress(5, 10);
//! screen.update_accuracy(85.2);
//!
//! let area = Rect::new(0, 0, 80, 24);
//! let mut buf = Buffer::empty(area);
//! screen.render(area, &mut buf);
//! ```

use crate::ui::organisms::eval_panel::{EvalPanel, TaskState, TaskStatus};
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

/// Evaluation screen state
///
/// Manages the evaluation display including progress, metrics, and task list.
///
/// # Examples
///
/// ```
/// use toad::ui::screens::evaluation::EvaluationScreen;
///
/// let mut screen = EvaluationScreen::new("M1 Baseline");
/// screen.update_progress(5, 10);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct EvaluationScreen {
    /// The evaluation panel organism
    panel: EvalPanel,
}

impl EvaluationScreen {
    /// Create a new evaluation screen
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::screens::evaluation::EvaluationScreen;
    ///
    /// let screen = EvaluationScreen::new("M1 Baseline Evaluation");
    /// ```
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            panel: EvalPanel::new().title(title),
        }
    }

    /// Update progress (current task / total tasks)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::screens::evaluation::EvaluationScreen;
    ///
    /// let mut screen = EvaluationScreen::new("Test");
    /// screen.update_progress(5, 10);
    /// ```
    pub fn update_progress(&mut self, current: usize, total: usize) {
        self.panel.current_task = current;
        self.panel.total_tasks = total;
    }

    /// Update accuracy metric
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::screens::evaluation::EvaluationScreen;
    ///
    /// let mut screen = EvaluationScreen::new("Test");
    /// screen.update_accuracy(85.2);
    /// ```
    pub fn update_accuracy(&mut self, accuracy: f64) {
        self.panel.accuracy = Some(accuracy);
    }

    /// Update cost metric
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::screens::evaluation::EvaluationScreen;
    ///
    /// let mut screen = EvaluationScreen::new("Test");
    /// screen.update_cost(0.45);
    /// ```
    pub fn update_cost(&mut self, cost: f64) {
        self.panel.cost = Some(cost);
    }

    /// Update duration metric
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::screens::evaluation::EvaluationScreen;
    ///
    /// let mut screen = EvaluationScreen::new("Test");
    /// screen.update_duration(120);
    /// ```
    pub fn update_duration(&mut self, duration_secs: u64) {
        self.panel.duration_secs = Some(duration_secs);
    }

    /// Add a task to the task list
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::screens::evaluation::{EvaluationScreen};
    /// use toad::ui::organisms::eval_panel::{TaskStatus, TaskState};
    ///
    /// let mut screen = EvaluationScreen::new("Test");
    /// screen.add_task(TaskStatus {
    ///     name: "Task 1".to_string(),
    ///     status: TaskState::Completed,
    ///     message: Some("Success".to_string()),
    /// });
    /// ```
    pub fn add_task(&mut self, task: TaskStatus) {
        self.panel.tasks.push(task);
    }

    /// Update a task's status by index
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::screens::evaluation::EvaluationScreen;
    /// use toad::ui::organisms::eval_panel::{TaskStatus, TaskState};
    ///
    /// let mut screen = EvaluationScreen::new("Test");
    /// screen.add_task(TaskStatus {
    ///     name: "Task 1".to_string(),
    ///     status: TaskState::Pending,
    ///     message: None,
    /// });
    /// screen.update_task(0, TaskState::Completed, Some("Done".to_string()));
    /// ```
    pub fn update_task(&mut self, index: usize, status: TaskState, message: Option<String>) {
        if let Some(task) = self.panel.tasks.get_mut(index) {
            task.status = status;
            task.message = message;
        }
    }

    /// Clear all tasks
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::screens::evaluation::EvaluationScreen;
    ///
    /// let mut screen = EvaluationScreen::new("Test");
    /// screen.clear_tasks();
    /// ```
    pub fn clear_tasks(&mut self) {
        self.panel.tasks.clear();
    }

    /// Get the current progress
    ///
    /// Returns (current, total)
    pub fn progress(&self) -> (usize, usize) {
        (self.panel.current_task, self.panel.total_tasks)
    }

    /// Get the current accuracy
    pub fn accuracy(&self) -> Option<f64> {
        self.panel.accuracy
    }

    /// Get the current cost
    pub fn cost(&self) -> Option<f64> {
        self.panel.cost
    }

    /// Get the current duration
    pub fn duration(&self) -> Option<u64> {
        self.panel.duration_secs
    }

    /// Get the number of tasks
    pub fn task_count(&self) -> usize {
        self.panel.tasks.len()
    }

    /// Render the screen
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::screens::evaluation::EvaluationScreen;
    /// use ratatui::{buffer::Buffer, layout::Rect};
    ///
    /// let screen = EvaluationScreen::new("Test");
    /// let area = Rect::new(0, 0, 80, 24);
    /// let mut buf = Buffer::empty(area);
    /// screen.render(area, &mut buf);
    /// ```
    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        (&self.panel).render(area, buf);
    }
}

impl Default for EvaluationScreen {
    fn default() -> Self {
        Self::new("Evaluation")
    }
}

impl Widget for EvaluationScreen {
    fn render(self, area: Rect, buf: &mut Buffer) {
        (&self).render(area, buf);
    }
}

impl Widget for &EvaluationScreen {
    fn render(self, area: Rect, buf: &mut Buffer) {
        (&self.panel).render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluation_screen_new() {
        let screen = EvaluationScreen::new("Test Evaluation");
        assert_eq!(screen.panel.title, "Test Evaluation");
    }

    #[test]
    fn test_evaluation_screen_update_progress() {
        let mut screen = EvaluationScreen::new("Test");
        screen.update_progress(5, 10);
        assert_eq!(screen.progress(), (5, 10));
    }

    #[test]
    fn test_evaluation_screen_update_accuracy() {
        let mut screen = EvaluationScreen::new("Test");
        screen.update_accuracy(85.2);
        assert_eq!(screen.accuracy(), Some(85.2));
    }

    #[test]
    fn test_evaluation_screen_update_cost() {
        let mut screen = EvaluationScreen::new("Test");
        screen.update_cost(0.45);
        assert_eq!(screen.cost(), Some(0.45));
    }

    #[test]
    fn test_evaluation_screen_update_duration() {
        let mut screen = EvaluationScreen::new("Test");
        screen.update_duration(120);
        assert_eq!(screen.duration(), Some(120));
    }

    #[test]
    fn test_evaluation_screen_add_task() {
        let mut screen = EvaluationScreen::new("Test");
        screen.add_task(TaskStatus {
            name: "Task 1".to_string(),
            status: TaskState::Completed,
            message: None,
        });
        assert_eq!(screen.task_count(), 1);
    }

    #[test]
    fn test_evaluation_screen_update_task() {
        let mut screen = EvaluationScreen::new("Test");
        screen.add_task(TaskStatus {
            name: "Task 1".to_string(),
            status: TaskState::Pending,
            message: None,
        });
        screen.update_task(0, TaskState::Completed, Some("Done".to_string()));
        assert_eq!(screen.panel.tasks[0].status, TaskState::Completed);
        assert_eq!(screen.panel.tasks[0].message, Some("Done".to_string()));
    }

    #[test]
    fn test_evaluation_screen_update_task_invalid_index() {
        let mut screen = EvaluationScreen::new("Test");
        screen.update_task(0, TaskState::Completed, None);
        // Should not panic
    }

    #[test]
    fn test_evaluation_screen_clear_tasks() {
        let mut screen = EvaluationScreen::new("Test");
        screen.add_task(TaskStatus {
            name: "Task 1".to_string(),
            status: TaskState::Completed,
            message: None,
        });
        screen.clear_tasks();
        assert_eq!(screen.task_count(), 0);
    }

    #[test]
    fn test_evaluation_screen_multiple_updates() {
        let mut screen = EvaluationScreen::new("Test");
        screen.update_progress(1, 10);
        screen.update_accuracy(50.0);
        screen.update_cost(0.1);
        screen.update_duration(30);

        screen.update_progress(5, 10);
        screen.update_accuracy(75.0);
        screen.update_cost(0.3);
        screen.update_duration(90);

        assert_eq!(screen.progress(), (5, 10));
        assert_eq!(screen.accuracy(), Some(75.0));
        assert_eq!(screen.cost(), Some(0.3));
        assert_eq!(screen.duration(), Some(90));
    }

    #[test]
    fn test_evaluation_screen_render() {
        let mut screen = EvaluationScreen::new("Test");
        screen.update_progress(5, 10);
        screen.update_accuracy(85.0);

        let area = Rect::new(0, 0, 80, 24);
        let mut buf = Buffer::empty(area);
        screen.render(area, &mut buf);
        // Just verify it doesn't panic
    }

    #[test]
    fn test_evaluation_screen_widget_trait() {
        let screen = EvaluationScreen::new("Test");
        let area = Rect::new(0, 0, 80, 24);
        let mut buf = Buffer::empty(area);
        // Test both owned and borrowed widget rendering
        screen.clone().render(area, &mut buf);
        (&screen).render(area, &mut buf);
    }

    #[test]
    fn test_evaluation_screen_default() {
        let screen = EvaluationScreen::default();
        assert_eq!(screen.panel.title, "Evaluation");
    }

    #[test]
    fn test_evaluation_screen_clone() {
        let mut screen1 = EvaluationScreen::new("Test");
        screen1.update_progress(5, 10);
        let screen2 = screen1.clone();
        assert_eq!(screen1, screen2);
    }

    #[test]
    fn test_evaluation_screen_full_workflow() {
        let mut screen = EvaluationScreen::new("M1 Baseline Evaluation");

        // Start evaluation
        screen.update_progress(0, 10);

        // Add pending tasks
        for i in 0..10 {
            screen.add_task(TaskStatus {
                name: format!("Task {}", i + 1),
                status: TaskState::Pending,
                message: None,
            });
        }

        // Process tasks
        for i in 0..5 {
            screen.update_task(i, TaskState::InProgress, None);
            screen.update_progress(i + 1, 10);
            screen.update_task(i, TaskState::Completed, Some("Success".to_string()));
        }

        // Update metrics
        screen.update_accuracy(80.0);
        screen.update_cost(0.25);
        screen.update_duration(60);

        assert_eq!(screen.progress(), (5, 10));
        assert_eq!(screen.accuracy(), Some(80.0));
        assert_eq!(screen.task_count(), 10);
    }
}
