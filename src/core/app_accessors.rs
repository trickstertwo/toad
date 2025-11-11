//! Application state accessor methods
//!
//! Provides getter and setter methods for accessing and modifying application state.
//! These methods follow standard Rust conventions: immutable getters return references,
//! mutable getters return mutable references.

use crate::config::Config;
use crate::core::app::App;
use crate::core::app_state::{AppScreen, EvaluationState};
use crate::core::event::Event;
use crate::performance::PerformanceMetrics;
use crate::ui::widgets::{
    core::{dialog::ConfirmDialog, help::HelpScreen},
    input::{input::InputField, palette::CommandPalette},
    notifications::toast::ToastManager,
};
use crate::workspace::{LayoutManager, SessionState, TabManager};
use std::path::PathBuf;

impl App {
    /// Check if the application should quit
    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    /// Get the current status message
    pub fn status_message(&self) -> &str {
        &self.status_message
    }

    /// Get the application title
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Get the current screen
    pub fn screen(&self) -> &AppScreen {
        &self.screen
    }

    /// Get the current working directory
    pub fn working_directory(&self) -> &PathBuf {
        &self.working_directory
    }

    /// Get the trust dialog (if present)
    pub fn trust_dialog(&self) -> Option<&ConfirmDialog> {
        self.trust_dialog.as_ref()
    }

    /// Get mutable trust dialog (if present)
    pub fn trust_dialog_mut(&mut self) -> Option<&mut ConfirmDialog> {
        self.trust_dialog.as_mut()
    }

    /// Get the input field
    pub fn input_field(&self) -> &InputField {
        &self.input_field
    }

    /// Get mutable input field
    pub fn input_field_mut(&mut self) -> &mut InputField {
        &mut self.input_field
    }

    /// Get plugin count
    pub fn plugin_count(&self) -> usize {
        self.plugin_count
    }

    /// Get the help screen
    pub fn help_screen(&self) -> &HelpScreen {
        &self.help_screen
    }

    /// Check if help should be shown
    pub fn show_help(&self) -> bool {
        self.show_help
    }

    /// Get mutable command palette
    pub fn command_palette_mut(&mut self) -> &mut CommandPalette {
        &mut self.command_palette
    }

    /// Check if command palette should be shown
    pub fn show_palette(&self) -> bool {
        self.show_palette
    }

    /// Get the layout manager
    pub fn layout(&self) -> &LayoutManager {
        &self.layout
    }

    /// Get mutable layout manager
    pub fn layout_mut(&mut self) -> &mut LayoutManager {
        &mut self.layout
    }

    /// Check if Vim mode is enabled
    pub fn vim_mode(&self) -> bool {
        self.vim_mode
    }

    /// Toggle Vim mode
    pub fn toggle_vim_mode(&mut self) {
        self.vim_mode = !self.vim_mode;
        self.config.ui.vim_mode = self.vim_mode;
        self.status_message = format!(
            "Vim mode {}",
            if self.vim_mode { "enabled" } else { "disabled" }
        );
    }

    /// Get configuration
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Save configuration to file
    ///
    /// # Errors
    ///
    /// Returns `Err` if:
    /// - Parent directory cannot be created
    /// - Configuration file cannot be written
    /// - Serialization fails
    pub fn save_config(&self) -> crate::Result<()> {
        let path = Config::default_path();

        // Create parent directory if it doesn't exist
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        self.config
            .save_to_file(&path)
            .map_err(|e| color_eyre::eyre::eyre!("Failed to save config: {}", e))?;
        Ok(())
    }

    /// Get performance metrics
    pub fn performance(&self) -> &PerformanceMetrics {
        &self.performance
    }

    /// Get mutable performance metrics
    pub fn performance_mut(&mut self) -> &mut PerformanceMetrics {
        &mut self.performance
    }

    /// Check if performance overlay is shown
    pub fn show_performance(&self) -> bool {
        self.show_performance
    }

    /// Toggle performance overlay
    pub fn toggle_performance(&mut self) {
        self.show_performance = !self.show_performance;
        self.status_message = format!(
            "Performance overlay {}",
            if self.show_performance {
                "shown"
            } else {
                "hidden"
            }
        );
    }

    /// Get toast manager
    pub fn toasts(&self) -> &ToastManager {
        &self.toasts
    }

    /// Get mutable toast manager
    pub fn toasts_mut(&mut self) -> &mut ToastManager {
        &mut self.toasts
    }

    /// Show an info toast
    pub fn toast_info(&mut self, message: impl Into<String>) {
        self.toasts.info(message);
    }

    /// Show a success toast
    pub fn toast_success(&mut self, message: impl Into<String>) {
        self.toasts.success(message);
    }

    /// Show a warning toast
    pub fn toast_warning(&mut self, message: impl Into<String>) {
        self.toasts.warning(message);
    }

    /// Show an error toast
    pub fn toast_error(&mut self, message: impl Into<String>) {
        self.toasts.error(message);
    }

    /// Set the event sender for async operations
    pub fn set_event_tx(&mut self, tx: tokio::sync::mpsc::UnboundedSender<Event>) {
        self.event_tx = Some(tx);
    }

    /// Get the evaluation state
    pub fn evaluation_state(&self) -> Option<&EvaluationState> {
        self.evaluation_state.as_ref()
    }

    /// Get mutable evaluation state
    pub fn evaluation_state_mut(&mut self) -> Option<&mut EvaluationState> {
        self.evaluation_state.as_mut()
    }

    /// Get reference to session state
    pub fn session(&self) -> &SessionState {
        &self.session
    }

    /// Get mutable reference to session state
    pub fn session_mut(&mut self) -> &mut SessionState {
        &mut self.session
    }

    /// Get reference to tab manager
    pub fn tabs(&self) -> &TabManager {
        &self.tabs
    }

    /// Get mutable reference to tab manager
    pub fn tabs_mut(&mut self) -> &mut TabManager {
        &mut self.tabs
    }
}

#[cfg(test)]
mod tests {
    use crate::core::app::App;

    // ===== Mutable Accessor Tests =====

    #[test]
    fn test_input_field_mut_accessor() {
        let mut app = App::new();
        let input = app.input_field_mut();
        input.set_value("test".to_string());
        assert_eq!(app.input_field().value(), "test");
    }

    #[test]
    fn test_command_palette_mut_accessor() {
        let mut app = App::new();
        let _palette = app.command_palette_mut();
        // Should not panic
    }

    #[test]
    fn test_layout_mut_accessor() {
        let mut app = App::new();
        let _layout = app.layout_mut();
        // Should not panic
    }

    #[test]
    fn test_performance_mut_accessor() {
        let mut app = App::new();
        let _perf = app.performance_mut();
        // Should not panic
    }

    #[test]
    fn test_toasts_mut_accessor() {
        let mut app = App::new();
        let _toasts = app.toasts_mut();
        // Should not panic
    }

    #[test]
    fn test_evaluation_state_mut_accessor() {
        let mut app = App::new();
        let state = app.evaluation_state_mut();
        assert!(
            state.is_none(),
            "Initial mutable evaluation state should be None"
        );
    }

    #[test]
    fn test_trust_dialog_mut_accessor() {
        let mut app = App::new();
        let dialog = app.trust_dialog_mut();
        // Initially should be None or Some depending on directory trust
        let _ = dialog;
    }

    #[test]
    fn test_session_mut_accessor() {
        let mut app = App::new();
        let session = app.session_mut();
        session.set_plugin_count(42);
        assert_eq!(app.session().plugin_count(), 42);
    }

    #[test]
    fn test_tabs_mut_accessor() {
        let mut app = App::new();
        let tabs = app.tabs_mut();
        // Should allow mutation
        let _ = tabs;
    }

    // ===== State Mutation Tests =====

    #[test]
    fn test_toggle_vim_mode() {
        let mut app = App::new();
        let initial = app.vim_mode();
        app.toggle_vim_mode();
        assert_ne!(app.vim_mode(), initial, "Vim mode should toggle");
        app.toggle_vim_mode();
        assert_eq!(app.vim_mode(), initial, "Vim mode should toggle back");
    }

    #[test]
    fn test_toggle_performance() {
        let mut app = App::new();
        assert!(!app.show_performance(), "Performance should start hidden");
        app.toggle_performance();
        assert!(
            app.show_performance(),
            "Performance should be shown after toggle"
        );
        app.toggle_performance();
        assert!(
            !app.show_performance(),
            "Performance should be hidden after second toggle"
        );
    }

    // ===== Toast Notification Tests =====

    #[test]
    fn test_toast_info() {
        let mut app = App::new();
        app.toast_info("Information message");
        // Should not panic
    }

    #[test]
    fn test_toast_success() {
        let mut app = App::new();
        app.toast_success("Success message");
        // Should not panic
    }

    #[test]
    fn test_toast_warning() {
        let mut app = App::new();
        app.toast_warning("Warning message");
        // Should not panic
    }

    #[test]
    fn test_toast_error() {
        let mut app = App::new();
        app.toast_error("Error message");
        // Should not panic
    }

    #[test]
    fn test_toast_with_string_types() {
        let mut app = App::new();
        app.toast_info(String::from("String message"));
        app.toast_success("&str message");
        app.toast_warning(format!("Formatted {}", "message"));
        // Should handle various Into<String> types
    }
}
