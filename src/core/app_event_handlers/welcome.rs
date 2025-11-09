//! Welcome screen event handlers
//!
//! Handles keyboard events when the user is on the welcome screen.

use crate::core::app::App;
use crate::core::app_state::AppScreen;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

impl App {
    /// Handle keys on the welcome screen
    ///
    /// # Behavior
    ///
    /// - `Esc` or `Ctrl+C`: Quit the application
    /// - Any other key: Advance to trust dialog screen
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use toad::core::app::App;
    /// # use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
    /// let mut app = App::new();
    /// let key = KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE);
    /// app.handle_welcome_key(key).unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// Returns `Err` if state transition fails (should not happen in practice).
    pub(crate) fn handle_welcome_key(&mut self, key: KeyEvent) -> crate::Result<()> {
        match (key.code, key.modifiers) {
            // Quit on Escape or Ctrl+C
            (KeyCode::Esc, _) | (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                self.should_quit = true;
            }
            // Any other key advances to trust dialog
            _ => {
                self.welcome_shown = true;
                self.screen = AppScreen::TrustDialog;
                self.create_trust_dialog();
                self.status_message = "Confirm folder trust to continue".to_string();
            }
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

    // ===== Welcome Screen Tests =====

    #[test]
    fn test_welcome_screen_any_key_advances() {
        let mut app = App::new();
        app.screen = AppScreen::Welcome;
        app.welcome_shown = false;

        // Press any regular key
        let event = Event::Key(KeyEvent::from(KeyCode::Char('a')));
        app.update(event).unwrap();

        // Should advance to trust dialog
        assert_eq!(*app.screen(), AppScreen::TrustDialog);
        assert!(app.welcome_shown);
    }

    #[test]
    fn test_welcome_screen_space_advances() {
        let mut app = App::new();
        app.screen = AppScreen::Welcome;
        app.welcome_shown = false;

        let event = Event::Key(KeyEvent::from(KeyCode::Char(' ')));
        app.update(event).unwrap();

        assert_eq!(*app.screen(), AppScreen::TrustDialog);
    }

    #[test]
    fn test_welcome_screen_enter_advances() {
        let mut app = App::new();
        app.screen = AppScreen::Welcome;
        app.welcome_shown = false;

        let event = Event::Key(KeyEvent::from(KeyCode::Enter));
        app.update(event).unwrap();

        assert_eq!(*app.screen(), AppScreen::TrustDialog);
    }

    #[test]
    fn test_welcome_screen_ctrl_c_quits() {
        let mut app = App::new();
        app.screen = AppScreen::Welcome;

        let event = Event::Key(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL));
        app.update(event).unwrap();

        assert!(app.should_quit());
    }

    #[test]
    fn test_welcome_screen_esc_quits() {
        let mut app = App::new();
        app.screen = AppScreen::Welcome;

        let event = Event::Key(KeyEvent::from(KeyCode::Esc));
        app.update(event).unwrap();

        assert!(app.should_quit());
    }

    #[test]
    fn test_welcome_shown_flag_set_on_advance() {
        let mut app = App::new();
        app.screen = AppScreen::Welcome;
        app.welcome_shown = false;

        let event = Event::Key(KeyEvent::from(KeyCode::Char(' ')));
        app.update(event).unwrap();

        assert!(app.welcome_shown, "Welcome shown flag should be set");
    }

    #[test]
    fn test_welcome_screen_creates_trust_dialog() {
        let mut app = App::new();
        app.screen = AppScreen::Welcome;
        app.trust_dialog = None;

        let event = Event::Key(KeyEvent::from(KeyCode::Enter));
        app.update(event).unwrap();

        assert!(app.trust_dialog.is_some(), "Trust dialog should be created");
    }

    #[test]
    fn test_welcome_screen_status_message_updated() {
        let mut app = App::new();
        app.screen = AppScreen::Welcome;

        let event = Event::Key(KeyEvent::from(KeyCode::Char('a')));
        app.update(event).unwrap();

        assert!(
            app.status_message.contains("trust") || app.status_message.contains("Confirm"),
            "Status message should mention trust: {}",
            app.status_message
        );
    }
}
