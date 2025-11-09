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
