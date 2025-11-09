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
