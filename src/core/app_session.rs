//! Session state management and persistence
//!
//! Handles saving and restoring application session state to disk.

use crate::core::app::App;
use crate::core::app_state::AppScreen;

impl App {
    /// Update session state with current application state
    ///
    /// Synchronizes the session state with the current application state before saving.
    ///
    /// # Fields Updated
    ///
    /// - `welcome_shown`: Whether the welcome screen has been shown
    /// - `working_directory`: Current working directory path
    /// - `plugin_count`: Number of installed plugins
    /// - `last_screen`: Last active screen (Welcome, TrustDialog, or Main)
    ///
    /// # Note
    ///
    /// The `Evaluation` screen is saved as `Main` since it's transient.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use toad::core::app::App;
    /// let mut app = App::new();
    /// // ... modify app state ...
    /// app.update_session_state();
    /// ```
    fn update_session_state(&mut self) {
        self.session.set_welcome_shown(self.welcome_shown);
        self.session
            .set_working_directory(self.working_directory.clone());
        self.session.set_plugin_count(self.plugin_count);

        // Convert current screen to string for session
        let screen_str = match self.screen {
            AppScreen::Welcome => "Welcome",
            AppScreen::TrustDialog => "TrustDialog",
            AppScreen::Main => "Main",
            AppScreen::Evaluation => "Main", // Save as Main since Evaluation is transient
        };
        self.session.set_last_screen(screen_str.to_string());
    }

    /// Save session state to disk
    ///
    /// Persists the current application state to disk if session persistence is enabled.
    ///
    /// # Behavior
    ///
    /// 1. Checks if session persistence is enabled (`config.session.persist_session`)
    /// 2. Checks if auto-save is enabled (`config.session.auto_save`)
    /// 3. Updates session state with current app state
    /// 4. Writes session to disk (typically `~/.config/toad/session.json`)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use toad::App;
    ///
    /// let mut app = App::new();
    /// app.save_session().unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// Returns `Err` if:
    /// - Session file cannot be written
    /// - Serialization fails
    /// - File system errors occur
    pub fn save_session(&mut self) -> crate::Result<()> {
        if self.config.session.persist_session && self.config.session.auto_save {
            self.update_session_state();
            self.session.auto_save()?;
        }
        Ok(())
    }
}
