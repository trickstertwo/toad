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
