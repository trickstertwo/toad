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

#[cfg(test)]
mod tests {
    use crate::core::app::App;
    use crate::core::app_state::AppScreen;
    use crate::core::event::Event;
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    // ===== Evaluation Screen Tests =====

    #[test]
    fn test_evaluation_screen_esc_when_complete() {
        let mut app = App::new();
        app.screen = AppScreen::Evaluation;
        // Set evaluation_state to None to simulate completed evaluation
        app.evaluation_state = None;

        let event = Event::Key(KeyEvent::from(KeyCode::Esc));
        app.update(event).unwrap();

        // Should return to Main screen
        assert_eq!(app.screen, AppScreen::Main);
    }

    #[test]
    fn test_evaluation_screen_q_when_complete() {
        let mut app = App::new();
        app.screen = AppScreen::Evaluation;
        // Set evaluation_state to None to simulate completed evaluation
        app.evaluation_state = None;

        let event = Event::Key(KeyEvent::from(KeyCode::Char('q')));
        app.update(event).unwrap();

        // Should return to Main screen
        assert_eq!(app.screen, AppScreen::Main);
    }

    #[test]
    fn test_evaluation_screen_ctrl_c_when_complete() {
        let mut app = App::new();
        app.screen = AppScreen::Evaluation;
        // Set evaluation_state to None to simulate completed evaluation
        app.evaluation_state = None;

        let event = Event::Key(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL));
        app.update(event).unwrap();

        // Should return to Main screen
        assert_eq!(app.screen, AppScreen::Main);
    }

    #[test]
    fn test_evaluation_screen_ignores_other_keys() {
        let mut app = App::new();
        app.screen = AppScreen::Evaluation;
        app.evaluation_state = None;

        // Type regular characters - should be ignored
        let event = Event::Key(KeyEvent::from(KeyCode::Char('a')));
        app.update(event).unwrap();

        // Should still be on evaluation screen
        assert_eq!(app.screen, AppScreen::Evaluation);
    }
}
