//! Evaluation screen event handlers
//!
//! Handles keyboard events when an evaluation is running or has completed.

use crate::core::app::App;
use crate::core::app_state::AppScreen;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

impl App {
    /// Handle keys during evaluation screen
    ///
    /// # Behavior
    ///
    /// - `Esc` or `Ctrl+C`: Cancel running evaluation (or return to main if complete)
    /// - `q`: Return to main screen (only if evaluation is complete)
    /// - Other keys: Ignored during evaluation
    ///
    /// # State Handling
    ///
    /// The handler checks the evaluation state to determine if the evaluation is still running:
    /// - **Running**: `Esc`/`Ctrl+C` cancels the evaluation
    /// - **Complete**: `Esc`/`Ctrl+C`/`q` returns to main screen
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use toad::core::app::App;
    /// # use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
    /// let mut app = App::new();
    /// let key = KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE);
    /// app.handle_evaluation_key(key).unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// Returns `Err` if state transition fails (should not happen in practice).
    pub(crate) fn handle_evaluation_key(&mut self, key: KeyEvent) -> crate::Result<()> {
        match (key.code, key.modifiers) {
            // Escape or Ctrl+C cancels running evaluation
            (KeyCode::Esc, _) | (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                // Check if evaluation is still running
                let is_running = self
                    .evaluation_state
                    .as_ref()
                    .and_then(|s| s.handle.as_ref())
                    .map(|h| h.is_running())
                    .unwrap_or(false);

                if is_running {
                    // Cancel the running evaluation
                    self.cancel_evaluation();
                } else {
                    // Evaluation is complete, just go back to main
                    self.screen = AppScreen::Main;
                }
            }
            // 'q' to go back to main screen (only if evaluation is complete)
            (KeyCode::Char('q'), KeyModifiers::NONE) => {
                let is_running = self
                    .evaluation_state
                    .as_ref()
                    .and_then(|s| s.handle.as_ref())
                    .map(|h| h.is_running())
                    .unwrap_or(false);

                if !is_running {
                    self.screen = AppScreen::Main;
                }
            }
            // Other keys are ignored during evaluation
            _ => {}
        }
        Ok(())
    }
}
