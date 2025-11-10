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
    pub(crate) fn update_session_state(&mut self) {
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

#[cfg(test)]
mod tests {
    use crate::core::app::App;
    use crate::core::app_state::AppScreen;

    // ===== Update Session State Tests =====

    #[test]
    fn test_update_session_state() {
        let mut app = App::new();
        app.update_session_state();
        // Should not panic
    }

    #[test]
    fn test_update_session_state_multiple_calls() {
        let mut app = App::new();
        app.update_session_state();
        app.update_session_state();
        app.update_session_state();
        // Multiple calls should not cause issues
    }

    #[test]
    fn test_update_session_state_syncs_welcome_shown() {
        let mut app = App::new();
        app.welcome_shown = true;

        app.update_session_state();

        assert!(app.session().welcome_shown());
    }

    #[test]
    fn test_update_session_state_syncs_working_directory() {
        let mut app = App::new();
        let original_wd = app.working_directory.clone();

        app.update_session_state();

        assert_eq!(app.session().working_directory(), &original_wd);
    }

    #[test]
    fn test_update_session_state_syncs_plugin_count() {
        let mut app = App::new();
        app.plugin_count = 42;

        app.update_session_state();

        assert_eq!(app.session().plugin_count(), 42);
    }

    #[test]
    fn test_update_session_state_screen_mapping() {
        let mut app = App::new();

        // Test each screen type
        for screen in [AppScreen::Welcome, AppScreen::Main, AppScreen::TrustDialog] {
            app.screen = screen.clone();
            app.update_session_state();

            let last_screen = app.session().last_screen();
            assert!(!last_screen.is_empty());
        }
    }

    #[test]
    fn test_update_session_state_evaluation_maps_to_main() {
        let mut app = App::new();
        app.screen = AppScreen::Evaluation;

        app.update_session_state();

        // Evaluation is transient, should save as Main
        let last_screen = app.session().last_screen();
        assert_eq!(last_screen, "Main");
    }

    // ===== Session Save Tests =====

    #[test]
    fn test_save_session_with_persist_disabled() {
        let mut app = App::new();
        app.config.session.persist_session = false;
        app.config.session.auto_save = true;

        let result = app.save_session();
        assert!(result.is_ok(), "Should return Ok when persist is disabled");
    }

    #[test]
    fn test_save_session_with_auto_save_disabled() {
        let mut app = App::new();
        app.config.session.persist_session = true;
        app.config.session.auto_save = false;

        let result = app.save_session();
        assert!(
            result.is_ok(),
            "Should return Ok when auto_save is disabled"
        );
    }

    #[test]
    fn test_save_session_with_both_enabled() {
        let mut app = App::new();
        app.config.session.persist_session = true;
        app.config.session.auto_save = true;

        let result = app.save_session();

        match result {
            Ok(_) => {
                // Successfully saved
                assert!(true);
            }
            Err(e) => {
                // Permission errors are acceptable
                let err_msg = e.to_string();
                assert!(
                    err_msg.contains("Permission denied")
                        || err_msg.contains("No such file")
                        || err_msg.contains("Read-only")
                );
            }
        }
    }
}
