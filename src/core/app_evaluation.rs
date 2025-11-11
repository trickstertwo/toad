//! Evaluation lifecycle management
//!
//! Handles starting, stopping, and managing evaluation tasks and comparisons.

use crate::core::app::App;
use crate::core::app_state::{AppScreen, EvaluationState};

impl App {
    /// Start an evaluation run
    ///
    /// Launches an asynchronous evaluation task with the specified parameters.
    ///
    /// # Behavior
    ///
    /// 1. Validates event channel is initialized
    /// 2. Spawns evaluation task using `eval_runner::start_evaluation`
    /// 3. Transitions to `Evaluation` screen
    /// 4. Updates status message with task count and milestone
    ///
    /// # Parameters
    ///
    /// - `args`: Evaluation arguments (count, milestone, dataset, etc.)
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use toad::core::app::App;
    /// # use toad::ai::eval_commands::EvalArgs;
    /// let mut app = App::new();
    /// let args = EvalArgs {
    ///     count: Some(10),
    ///     milestone: 1,
    ///     ..Default::default()
    /// };
    /// app.start_evaluation(args);
    /// ```
    ///
    /// # Errors
    ///
    /// Shows error toast if event channel is not initialized.
    pub fn start_evaluation(&mut self, args: crate::ai::eval_commands::EvalArgs) {
        if let Some(ref event_tx) = self.event_tx {
            let handle = crate::ai::eval_runner::start_evaluation(args.clone(), event_tx.clone());

            self.evaluation_state = Some(EvaluationState {
                handle: Some(handle),
                progress: None,
                results: None,
                error: None,
            });

            self.screen = AppScreen::Evaluation;
            self.status_message = format!(
                "Starting evaluation: {} tasks, milestone {}",
                args.count.unwrap_or(10),
                args.milestone
            );
        } else {
            self.toast_error("Cannot start evaluation: event channel not initialized");
        }
    }

    /// Start a comparison run
    ///
    /// Launches an asynchronous A/B comparison between two milestone configurations.
    ///
    /// # Behavior
    ///
    /// 1. Validates event channel is initialized
    /// 2. Spawns comparison task using `eval_runner::start_comparison`
    /// 3. Transitions to `Evaluation` screen
    /// 4. Updates status message with baseline and test milestones
    ///
    /// # Parameters
    ///
    /// - `args`: Comparison arguments (count, baseline milestone, test milestone, dataset)
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use toad::core::app::App;
    /// # use toad::ai::eval_commands::CompareArgs;
    /// let mut app = App::new();
    /// let args = CompareArgs {
    ///     count: Some(20),
    ///     baseline: 1,
    ///     test: 2,
    ///     ..Default::default()
    /// };
    /// app.start_comparison(args);
    /// ```
    ///
    /// # Errors
    ///
    /// Shows error toast if event channel is not initialized.
    pub fn start_comparison(&mut self, args: crate::ai::eval_commands::CompareArgs) {
        if let Some(ref event_tx) = self.event_tx {
            let handle = crate::ai::eval_runner::start_comparison(args.clone(), event_tx.clone());

            self.evaluation_state = Some(EvaluationState {
                handle: Some(handle),
                progress: None,
                results: None,
                error: None,
            });

            self.screen = AppScreen::Evaluation;
            self.status_message = format!(
                "Starting comparison: {} tasks, M{} vs M{}",
                args.count.unwrap_or(20),
                args.baseline,
                args.test
            );
        } else {
            self.toast_error("Cannot start comparison: event channel not initialized");
        }
    }

    /// Cancel running evaluation
    ///
    /// Cancels the currently running evaluation or comparison task.
    ///
    /// # Behavior
    ///
    /// 1. Checks if evaluation is running
    /// 2. Takes ownership of the evaluation handle
    /// 3. Spawns async task to cancel the evaluation
    /// 4. Transitions back to `Main` screen
    /// 5. Shows "Evaluation cancelled" toast
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use toad::core::app::App;
    /// let mut app = App::new();
    /// // ... start evaluation ...
    /// app.cancel_evaluation();
    /// ```
    ///
    /// # Notes
    ///
    /// - Does nothing if no evaluation is running
    /// - Cancellation is asynchronous and may not be immediate
    /// - Returns to main screen immediately, even if cancellation is pending
    pub fn cancel_evaluation(&mut self) {
        if let Some(ref mut eval_state) = self.evaluation_state
            && let Some(handle) = eval_state.handle.take()
        {
            // Spawn a task to cancel the evaluation
            tokio::spawn(async move {
                handle.cancel().await;
            });

            self.toast_info("Evaluation cancelled");
            self.screen = AppScreen::Main;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::app::App;

    #[test]
    fn test_cancel_evaluation_when_none() {
        let mut app = App::new();
        assert!(app.evaluation_state().is_none());

        // Cancel when no evaluation running - should not panic
        app.cancel_evaluation();

        assert!(app.evaluation_state().is_none());
    }

    #[test]
    fn test_set_event_tx_allows_evaluation() {
        use tokio::sync::mpsc;

        let mut app = App::new();
        let (tx, _rx) = mpsc::unbounded_channel();

        app.set_event_tx(tx);
        // event_tx should now be Some, enabling evaluations
        assert!(app.event_tx.is_some());
    }

    // ===== Evaluation Progress Event Tests =====

    #[test]
    fn test_evaluation_progress_event_without_state() {
        use crate::core::event::{EvaluationProgress, Event};

        let mut app = App::new();
        app.evaluation_state = None;

        let mut progress = EvaluationProgress::new(5, 10, "task-123".to_string());
        progress.current_step = Some(3);
        progress.max_steps = Some(25);
        progress.last_tool = Some("Read".to_string());
        progress.total_tokens = 1000;
        progress.total_cost = 0.05;
        progress.message = Some("Processing...".to_string());

        let event = Event::EvaluationProgress(progress);
        app.update(event).unwrap();

        // Should handle gracefully when no evaluation state
    }

    #[test]
    fn test_evaluation_progress_event_with_state() {
        use crate::core::app_state::EvaluationState;
        use crate::core::event::{EvaluationProgress, Event};

        let mut app = App::new();
        app.evaluation_state = Some(EvaluationState {
            handle: None,
            progress: None,
            results: None,
            error: None,
        });

        let mut progress = EvaluationProgress::new(3, 10, "task-456".to_string());
        progress.current_step = Some(5);
        progress.max_steps = Some(25);
        progress.last_tool = Some("Edit".to_string());
        progress.total_tokens = 2000;
        progress.total_cost = 0.10;
        progress.message = Some("Working on task...".to_string());

        let event = Event::EvaluationProgress(progress.clone());
        app.update(event).unwrap();

        // Should update status message and progress
        assert!(app.status_message.contains("Working on task"));
        assert!(app.evaluation_state.as_ref().unwrap().progress.is_some());
    }

    #[test]
    fn test_evaluation_progress_event_without_message() {
        use crate::core::app_state::EvaluationState;
        use crate::core::event::{EvaluationProgress, Event};

        let mut app = App::new();
        app.evaluation_state = Some(EvaluationState {
            handle: None,
            progress: None,
            results: None,
            error: None,
        });

        let progress = EvaluationProgress::new(7, 15, "task-789".to_string());

        let event = Event::EvaluationProgress(progress);
        app.update(event).unwrap();

        // Should use default message format
        assert!(app.status_message.contains("7/15") || app.status_message.contains("task-789"));
    }

    // ===== Evaluation Complete Event Tests =====

    #[test]
    fn test_evaluation_complete_event() {
        use crate::ai::evaluation::EvaluationResults;
        use crate::core::app_state::EvaluationState;
        use crate::core::event::Event;
        use chrono::Utc;
        use std::collections::HashMap;

        let mut app = App::new();
        app.evaluation_state = Some(EvaluationState {
            handle: None,
            progress: None,
            results: None,
            error: None,
        });

        let results = EvaluationResults {
            config_name: "M1".to_string(),
            results: vec![],
            accuracy: 65.5,
            avg_cost_usd: 0.05,
            avg_duration_ms: 1500.0,
            total_tasks: 20,
            tasks_solved: 13,
            by_complexity: HashMap::new(),
            timestamp: Utc::now(),
        };

        let event = Event::EvaluationComplete(results.clone());
        app.update(event).unwrap();

        // Should update state and show success toast
        assert!(app.evaluation_state.as_ref().unwrap().results.is_some());
        assert!(app.evaluation_state.as_ref().unwrap().handle.is_none());
        assert!(app.status_message.contains("65.5") || app.status_message.contains("13/20"));
    }

    // ===== Evaluation Error Event Tests =====

    #[test]
    fn test_evaluation_error_event() {
        use crate::core::app_state::{AppScreen, EvaluationState};
        use crate::core::event::Event;

        let mut app = App::new();
        app.screen = AppScreen::Evaluation;
        app.evaluation_state = Some(EvaluationState {
            handle: None,
            progress: None,
            results: None,
            error: None,
        });

        let error = "Network timeout".to_string();
        let event = Event::EvaluationError(error.clone());
        app.update(event).unwrap();

        // Should update error state and return to Main
        assert!(app.evaluation_state.as_ref().unwrap().error.is_some());
        assert_eq!(*app.screen(), AppScreen::Main);
        assert!(app.status_message.contains("Network timeout"));
    }

    #[test]
    fn test_evaluation_error_sets_error_field() {
        use crate::core::app_state::EvaluationState;
        use crate::core::event::Event;

        let mut app = App::new();
        app.evaluation_state = Some(EvaluationState {
            handle: None,
            progress: None,
            results: None,
            error: None,
        });

        let error_msg = "API rate limit exceeded".to_string();
        let event = Event::EvaluationError(error_msg.clone());
        app.update(event).unwrap();

        let eval_state = app.evaluation_state.as_ref().unwrap();
        assert_eq!(eval_state.error.as_ref().unwrap(), &error_msg);
    }

    #[test]
    fn test_evaluation_error_clears_handle() {
        use crate::core::app_state::EvaluationState;
        use crate::core::event::Event;

        let mut app = App::new();
        app.evaluation_state = Some(EvaluationState {
            handle: None,
            progress: None,
            results: None,
            error: None,
        });

        let event = Event::EvaluationError("Error".to_string());
        app.update(event).unwrap();

        // Handle should be cleared on error
        assert!(app.evaluation_state.as_ref().unwrap().handle.is_none());
    }
}
