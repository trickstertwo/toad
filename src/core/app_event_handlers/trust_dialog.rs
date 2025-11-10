//! Trust dialog event handlers
//!
//! Handles keyboard events when the user is in the directory trust confirmation dialog.

use crate::core::app::App;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

impl App {
    /// Handle keys in the trust dialog
    ///
    /// # Behavior
    ///
    /// - `Esc` or `Ctrl+C`: Quit the application
    /// - `Up`/`Down` arrows: Navigate between trust options
    /// - `1`-`3`: Select option by number directly
    /// - `Enter`: Confirm the current selection
    ///
    /// # Trust Options
    ///
    /// The trust dialog typically offers three options:
    /// 1. Trust this directory
    /// 2. Trust this directory and all subdirectories
    /// 3. Don't trust (quit)
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use toad::core::app::App;
    /// # use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
    /// let mut app = App::new();
    /// let key = KeyEvent::new(KeyCode::Down, KeyModifiers::NONE);
    /// app.handle_trust_dialog_key(key).unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// Returns `Err` if state transition fails (should not happen in practice).
    pub(crate) fn handle_trust_dialog_key(&mut self, key: KeyEvent) -> crate::Result<()> {
        match (key.code, key.modifiers) {
            // Escape cancels
            (KeyCode::Esc, _) => {
                self.should_quit = true;
            }
            // Ctrl+C quits
            (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                self.should_quit = true;
            }
            // Arrow keys navigate
            (KeyCode::Up, _) => {
                if let Some(dialog) = &mut self.trust_dialog {
                    dialog.select_previous();
                }
            }
            (KeyCode::Down, _) => {
                if let Some(dialog) = &mut self.trust_dialog {
                    dialog.select_next();
                }
            }
            // Number keys select directly
            (KeyCode::Char(c @ '1'..='3'), _) => {
                if let Some(dialog) = &mut self.trust_dialog
                    && dialog.select_by_key(c).is_some()
                {
                    self.confirm_trust_selection();
                }
            }
            // Enter confirms selection
            (KeyCode::Enter, _) => {
                self.confirm_trust_selection();
            }
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

    // ===== Trust Dialog Workflow Tests =====

    #[test]
    fn test_create_trust_dialog() {
        let mut app = App::new();
        app.create_trust_dialog();

        // Should create a trust dialog
        assert!(app.trust_dialog.is_some());
    }

    #[test]
    fn test_create_trust_dialog_multiple_calls() {
        let mut app = App::new();
        app.create_trust_dialog();
        let first = app.trust_dialog.is_some();

        app.create_trust_dialog();
        let second = app.trust_dialog.is_some();

        assert_eq!(first, second, "Multiple calls should maintain trust dialog");
    }

    #[test]
    fn test_confirm_trust_selection_without_dialog() {
        let mut app = App::new();
        app.trust_dialog = None;
        app.confirm_trust_selection();
        // Should handle None case gracefully
    }

    #[test]
    fn test_confirm_trust_selection_yes_session_only() {
        let mut app = App::new();
        app.screen = AppScreen::TrustDialog;
        app.create_trust_dialog();

        // Select option 0 (Yes for this session)
        if let Some(dialog) = &mut app.trust_dialog {
            while dialog.selected() != 0 {
                dialog.select_next();
            }
        }

        app.confirm_trust_selection();

        assert_eq!(*app.screen(), AppScreen::Main);
        assert!(app.trust_dialog.is_none());
    }

    #[test]
    fn test_confirm_trust_selection_yes_and_remember() {
        let mut app = App::new();
        app.screen = AppScreen::TrustDialog;
        app.create_trust_dialog();

        // Select option 1 (Yes and remember)
        if let Some(dialog) = &mut app.trust_dialog {
            dialog.select_next(); // Move to option 1
        }

        app.confirm_trust_selection();

        assert_eq!(*app.screen(), AppScreen::Main);
        assert!(app.trust_dialog.is_none());
        // Check status message instead of session state (which might be loaded from disk)
        assert!(app.status_message.contains("remembered"));
    }

    #[test]
    fn test_confirm_trust_selection_no_quit() {
        let mut app = App::new();
        app.screen = AppScreen::TrustDialog;
        app.create_trust_dialog();

        // Select option 2 (No - quit)
        if let Some(dialog) = &mut app.trust_dialog {
            dialog.select_next();
            dialog.select_next(); // Move to option 2
        }

        app.confirm_trust_selection();

        assert!(app.should_quit());
    }

    // ===== Keyboard Handling Tests =====

    #[test]
    fn test_trust_dialog_esc_quits() {
        let mut app = App::new();
        app.screen = AppScreen::TrustDialog;
        app.create_trust_dialog();

        let event = Event::Key(KeyEvent::from(KeyCode::Esc));
        app.update(event).unwrap();

        assert!(app.should_quit());
    }

    #[test]
    fn test_trust_dialog_ctrl_c_quits() {
        let mut app = App::new();
        app.screen = AppScreen::TrustDialog;
        app.create_trust_dialog();

        let event = Event::Key(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL));
        app.update(event).unwrap();

        assert!(app.should_quit());
    }

    #[test]
    fn test_trust_dialog_enter_confirms() {
        let mut app = App::new();
        app.screen = AppScreen::TrustDialog;
        app.create_trust_dialog();

        let event = Event::Key(KeyEvent::from(KeyCode::Enter));
        app.update(event).unwrap();

        // Should confirm whichever option is selected
        assert!(app.screen() != &AppScreen::TrustDialog || app.should_quit());
    }

    #[test]
    fn test_trust_dialog_up_arrow_navigates() {
        let mut app = App::new();
        app.screen = AppScreen::TrustDialog;
        app.create_trust_dialog();

        let event = Event::Key(KeyEvent::from(KeyCode::Up));
        app.update(event).unwrap();

        // Should not panic
    }

    #[test]
    fn test_trust_dialog_down_arrow_navigates() {
        let mut app = App::new();
        app.screen = AppScreen::TrustDialog;
        app.create_trust_dialog();

        let event = Event::Key(KeyEvent::from(KeyCode::Down));
        app.update(event).unwrap();

        // Should not panic
    }

    #[test]
    fn test_trust_dialog_number_key_selection() {
        let mut app = App::new();
        app.screen = AppScreen::TrustDialog;
        app.create_trust_dialog();

        // Press '1' to select first option
        let event = Event::Key(KeyEvent::from(KeyCode::Char('1')));
        app.update(event).unwrap();

        // Should confirm and move to main screen
        assert_eq!(*app.screen(), AppScreen::Main);
    }
}
