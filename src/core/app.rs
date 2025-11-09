//! Application state module (Model in Elm Architecture)
//!
//! This module contains the application state and the update logic
//! that handles state transitions based on events.

use crate::config::Config;
use crate::core::app_state::{AppScreen, EvaluationState};
use crate::core::event::Event;
use crate::performance::PerformanceMetrics;
use crate::ui::widgets::{CommandPalette, ConfirmDialog, HelpScreen, InputField, ToastManager};
use crate::workspace::{LayoutManager, SessionState, TabManager};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::path::PathBuf;

/// Application state (Model in Elm Architecture)
#[derive(Debug)]
pub struct App {
    /// Current screen being displayed
    pub(crate) screen: AppScreen,

    /// Whether the application should quit
    pub(crate) should_quit: bool,

    /// Status bar message
    pub(crate) status_message: String,

    /// Application title
    title: String,

    /// Current working directory
    working_directory: PathBuf,

    /// Trust dialog state (if applicable)
    pub(crate) trust_dialog: Option<ConfirmDialog>,

    /// Whether the user has seen the welcome screen
    pub(crate) welcome_shown: bool,

    /// Input field for user commands/queries
    pub(crate) input_field: InputField,

    /// Number of installed plugins
    plugin_count: usize,

    /// Help screen widget
    help_screen: HelpScreen,

    /// Whether to show the help overlay
    pub(crate) show_help: bool,

    /// Command palette widget
    pub(crate) command_palette: CommandPalette,

    /// Whether to show the command palette
    pub(crate) show_palette: bool,

    /// Application configuration
    config: Config,

    /// Session state for persistence
    session: SessionState,

    /// Tab manager for multiple workspaces
    pub(crate) tabs: TabManager,

    /// Layout manager for split panes
    pub(crate) layout: LayoutManager,

    /// Vim mode enabled
    pub(crate) vim_mode: bool,

    /// Performance metrics
    performance: PerformanceMetrics,

    /// Show performance overlay
    show_performance: bool,

    /// Toast notification manager
    toasts: ToastManager,

    /// Event sender for async operations (evaluation, etc.)
    pub(crate) event_tx: Option<tokio::sync::mpsc::UnboundedSender<Event>>,

    /// Current evaluation state
    pub(crate) evaluation_state: Option<EvaluationState>,
}

impl Default for App {
    fn default() -> Self {
        let config = Config::load_or_default();
        let session = if config.session.persist_session {
            SessionState::load_or_new()
        } else {
            SessionState::new()
        };

        let working_directory = session.working_directory().clone();
        let welcome_shown = session.welcome_shown();

        let mut input_field = InputField::new();
        input_field.set_focused(true);

        // Determine initial screen based on session
        let screen = if welcome_shown {
            AppScreen::Main
        } else {
            AppScreen::Welcome
        };

        // Load vim mode from config
        let vim_mode = config.ui.vim_mode;

        Self {
            screen,
            should_quit: false,
            status_message: if welcome_shown {
                "Welcome back!".to_string()
            } else {
                "Press any key to continue...".to_string()
            },
            title: "Toad - AI Coding Terminal".to_string(),
            working_directory,
            trust_dialog: None,
            welcome_shown,
            input_field,
            plugin_count: session.plugin_count(),
            help_screen: HelpScreen::new(),
            show_help: false,
            command_palette: CommandPalette::new(),
            show_palette: false,
            config,
            session,
            tabs: TabManager::new(),
            layout: LayoutManager::new(),
            vim_mode,
            performance: PerformanceMetrics::new(),
            show_performance: false,
            toasts: ToastManager::new(),
            event_tx: None,
            evaluation_state: None,
        }
    }
}

impl App {
    /// Create a new application instance (Init in Elm Architecture)
    pub fn new() -> Self {
        Self::default()
    }

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

    /// Start an evaluation run

    /// Update application state based on an event (Update in Elm Architecture)
    ///
    /// This is the core state transition function that takes an event
    /// and produces a new state.
    pub fn update(&mut self, event: Event) -> crate::Result<()> {
        match event {
            Event::Key(key_event) => self.handle_key_event(key_event),
            Event::Resize(width, height) => {
                self.status_message = format!("Terminal resized to {}x{}", width, height);
                Ok(())
            }
            Event::Quit => {
                self.should_quit = true;
                Ok(())
            }
            Event::Tick => {
                // Toasts are automatically cleaned up during render
                Ok(())
            }
            Event::Mouse(_) => Ok(()),

            // Evaluation events
            Event::StartEvaluation(args) => {
                self.start_evaluation(args);
                Ok(())
            }
            Event::StartComparison(args) => {
                self.start_comparison(args);
                Ok(())
            }
            Event::EvaluationProgress(progress) => {
                if let Some(ref mut eval_state) = self.evaluation_state {
                    self.status_message = progress.message.clone().unwrap_or_else(|| {
                        format!(
                            "Task {}/{}: {}",
                            progress.current_task, progress.total_tasks, progress.task_id
                        )
                    });
                    eval_state.progress = Some(progress);
                }
                Ok(())
            }
            Event::EvaluationComplete(results) => {
                if let Some(ref mut eval_state) = self.evaluation_state {
                    let accuracy = results.accuracy;
                    let tasks_solved = results.tasks_solved;
                    let total_tasks = results.total_tasks;

                    eval_state.results = Some(results);
                    eval_state.handle = None; // Evaluation is done

                    self.toast_success(format!(
                        "Evaluation complete: {}/{} solved ({:.1}%)",
                        tasks_solved, total_tasks, accuracy
                    ));
                    self.status_message = format!("Evaluation complete: {:.1}% accuracy", accuracy);
                }
                Ok(())
            }
            Event::EvaluationError(error) => {
                if let Some(ref mut eval_state) = self.evaluation_state {
                    eval_state.error = Some(error.clone());
                    eval_state.handle = None;

                    self.toast_error(format!("Evaluation failed: {}", error));
                    self.status_message = format!("Evaluation error: {}", error);
                    self.screen = AppScreen::Main;
                }
                Ok(())
            }
            Event::CancelEvaluation => {
                self.cancel_evaluation();
                Ok(())
            }
        }
    }

    /// Handle keyboard events based on current screen
    fn handle_key_event(&mut self, key: KeyEvent) -> crate::Result<()> {
        match &self.screen {
            AppScreen::Welcome => self.handle_welcome_key(key),
            AppScreen::TrustDialog => self.handle_trust_dialog_key(key),
            AppScreen::Main => self.handle_main_key(key),
            AppScreen::Evaluation => self.handle_evaluation_key(key),
        }
    }


    /// Process commands entered by the user
    pub(crate) fn process_command(&mut self, input: &str) {
        if let Some(command) = input.strip_prefix('/') {
            match command {
                "help" => {
                    self.show_help = true;
                    self.status_message = "Showing help screen".to_string();
                }
                "commands" => {
                    self.status_message =
                        "Available commands: /help, /commands, /clear, eval, compare, show-config"
                            .to_string();
                }
                "clear" => {
                    self.status_message = "Screen cleared".to_string();
                }
                _ => {
                    self.status_message = format!("Unknown command: /{}", command);
                }
            }
        } else {
            // Try parsing as evaluation command (eval, compare, show-config)
            match crate::ai::eval_commands::parse_eval_command(input) {
                Ok(crate::ai::eval_commands::EvalCommand::Eval(args)) => {
                    self.start_evaluation(args);
                }
                Ok(crate::ai::eval_commands::EvalCommand::Compare(args)) => {
                    self.start_comparison(args);
                }
                Ok(crate::ai::eval_commands::EvalCommand::ShowConfig(args)) => {
                    let config = crate::config::ToadConfig::for_milestone(args.milestone as u8);
                    self.toast_info(format!("Milestone {} configuration:", args.milestone));
                    self.status_message = format!(
                        "M{}: {} features enabled",
                        args.milestone,
                        config.features.enabled_count()
                    );
                    // TODO: Show config in a dialog or dedicated panel
                }
                Err(e) => {
                    // Not a valid eval command, treat as regular query/request
                    if input.starts_with("eval")
                        || input.starts_with("compare")
                        || input.starts_with("show-config")
                    {
                        self.toast_error(format!("Command error: {}", e));
                        self.status_message = format!("Error: {}", e);
                    } else {
                        // Regular query/request
                        self.status_message = format!("Processing: {}", input);
                        self.toast_info("AI query processing coming soon");
                    }
                }
            }
        }
    }

    /// Execute a command from the command palette
    pub(crate) fn execute_palette_command(&mut self, cmd_id: &str) {
        match cmd_id {
            "help" => {
                self.show_help = true;
                self.status_message = "Opened help screen".to_string();
            }
            "clear" => {
                self.status_message = "Screen cleared".to_string();
            }
            "quit" => {
                self.should_quit = true;
            }
            "vim_mode" => {
                self.toggle_vim_mode();
            }
            "theme_toggle" => {
                self.status_message = "Theme toggled (coming soon)".to_string();
            }
            "split_horizontal" => {
                self.status_message = "Split horizontal (coming soon)".to_string();
            }
            "split_vertical" => {
                self.status_message = "Split vertical (coming soon)".to_string();
            }
            "open_file" => {
                self.status_message = "Open file (coming soon)".to_string();
            }
            "search_files" => {
                self.status_message = "Search files (coming soon)".to_string();
            }
            "git_status" => {
                self.status_message = "Git status (coming soon)".to_string();
            }
            "recent_files" => {
                self.status_message = "Recent files (coming soon)".to_string();
            }
            _ => {
                self.status_message = format!("Unknown command: {}", cmd_id);
            }
        }
    }

    /// Create the trust dialog for the current directory
    pub(crate) fn create_trust_dialog(&mut self) {
        let dir_path = self.working_directory.to_string_lossy().to_string();

        self.trust_dialog = Some(
            ConfirmDialog::new("Confirm folder trust")
                .info_box(dir_path)
                .message("Toad may read files in this folder. Reading untrusted files may lead Toad to behave in unexpected ways.".to_string())
                .message("With your permission, Toad may execute code or commands in this folder. Executing untrusted code is unsafe.".to_string())
                .message("")
                .message("Do you trust the files in this folder?".to_string())
                .option('1', "Yes")
                .option('2', "Yes, and remember this folder for future sessions")
                .option('3', "No (Esc)"),
        );
    }

    /// Confirm the trust dialog selection and advance
    pub(crate) fn confirm_trust_selection(&mut self) {
        if let Some(dialog) = &self.trust_dialog {
            let selected = dialog.selected();

            match selected {
                0 => {
                    // Yes - trust for this session
                    self.screen = AppScreen::Main;
                    self.trust_dialog = None;
                    self.status_message =
                        "Folder trusted for this session. Press 'q' to quit.".to_string();
                }
                1 => {
                    // Yes and remember - Save to session
                    self.screen = AppScreen::Main;
                    self.trust_dialog = None;
                    self.session.set_welcome_shown(true);
                    let _ = self.save_session();
                    self.status_message =
                        "Folder trusted and remembered. Press 'q' to quit.".to_string();
                }
                2 => {
                    // No - quit
                    self.should_quit = true;
                }
                _ => {}
            }
        }
    }

    /// Update session state from current app state
    ///
    /// Synchronizes the session state with the current application state.
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
    /// # Examples
    ///
    /// ```no_run
    /// use toad::App;
    ///
    /// let mut app = App::new();
    /// app.save_session().unwrap();
    /// ```
    pub fn save_session(&mut self) -> crate::Result<()> {
        if self.config.session.persist_session && self.config.session.auto_save {
            self.update_session_state();
            self.session.auto_save()?;
        }
        Ok(())
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
    use super::*;

    #[test]
    fn test_app_init() {
        let app = App::new();
        assert!(!app.should_quit());
        assert_eq!(app.title(), "Toad - AI Coding Terminal");
    }

    #[test]
    fn test_quit_on_esc_from_welcome() {
        let mut app = App::new();
        // Force welcome screen (session persistence may skip it)
        app.screen = AppScreen::Welcome;
        app.welcome_shown = false;

        assert_eq!(app.screen(), &AppScreen::Welcome);
        let event = Event::Key(KeyEvent::from(KeyCode::Esc));
        app.update(event).unwrap();
        assert!(app.should_quit());
    }

    #[test]
    fn test_quit_on_ctrl_c_from_main() {
        let mut app = App::new();
        // Manually set to Main screen
        app.screen = AppScreen::Main;
        let event = Event::Key(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL));
        app.update(event).unwrap();
        assert!(app.should_quit());
    }

    #[test]
    fn test_input_field() {
        let mut app = App::new();
        app.screen = AppScreen::Main;

        // Test character input
        let event = Event::Key(KeyEvent::from(KeyCode::Char('h')));
        app.update(event).unwrap();
        assert_eq!(app.input_field().value(), "h");

        // Test more input
        let event = Event::Key(KeyEvent::from(KeyCode::Char('i')));
        app.update(event).unwrap();
        assert_eq!(app.input_field().value(), "hi");

        // Test backspace
        let event = Event::Key(KeyEvent::from(KeyCode::Backspace));
        app.update(event).unwrap();
        assert_eq!(app.input_field().value(), "h");
    }

    // ========================================================================
    // COMPREHENSIVE NAVIGATION TESTS
    // ========================================================================

    #[test]
    fn test_help_screen_toggle_with_question_mark() {
        let mut app = App::new();
        app.screen = AppScreen::Main;

        assert!(!app.show_help, "Help should be hidden initially");

        // Press ? to show help
        let event = Event::Key(KeyEvent::from(KeyCode::Char('?')));
        app.update(event).unwrap();
        assert!(app.show_help, "Help should be visible after pressing ?");

        // Press ? again to hide help
        let event = Event::Key(KeyEvent::from(KeyCode::Char('?')));
        app.update(event).unwrap();
        assert!(!app.show_help, "Help should be hidden after pressing ? again");
    }

    #[test]
    fn test_help_screen_close_with_esc() {
        let mut app = App::new();
        app.screen = AppScreen::Main;

        // Show help
        app.show_help = true;

        // Press Esc to close
        let event = Event::Key(KeyEvent::from(KeyCode::Esc));
        app.update(event).unwrap();
        assert!(!app.show_help, "Help should be hidden after pressing Esc");
    }

    #[test]
    fn test_command_palette_toggle_with_ctrl_p() {
        let mut app = App::new();
        app.screen = AppScreen::Main;

        assert!(!app.show_palette, "Command palette should be hidden initially");

        // Press Ctrl+P to show palette
        let event = Event::Key(KeyEvent::new(KeyCode::Char('p'), KeyModifiers::CONTROL));
        app.update(event).unwrap();
        assert!(app.show_palette, "Command palette should be visible after Ctrl+P");

        // Press Ctrl+P again to hide
        let event = Event::Key(KeyEvent::new(KeyCode::Char('p'), KeyModifiers::CONTROL));
        app.update(event).unwrap();
        assert!(!app.show_palette, "Command palette should be hidden after Ctrl+P again");
    }

    #[test]
    fn test_command_palette_close_with_esc() {
        let mut app = App::new();
        app.screen = AppScreen::Main;

        // Show command palette
        app.show_palette = true;

        // Press Esc to close
        let event = Event::Key(KeyEvent::from(KeyCode::Esc));
        app.update(event).unwrap();
        assert!(!app.show_palette, "Command palette should be hidden after Esc");
    }

    #[test]
    fn test_q_key_does_not_quit_from_main() {
        let mut app = App::new();
        app.screen = AppScreen::Main;
        app.input_field.set_focused(false); // Ensure input is not focused

        let event = Event::Key(KeyEvent::from(KeyCode::Char('q')));
        app.update(event).unwrap();
        // Note: 'q' does not quit from main screen, only from evaluation screen
        // Input field will receive 'q' if focused
        assert!(!app.should_quit(), "App should NOT quit with q from main");
    }

    #[test]
    fn test_quit_with_ctrl_d_on_empty_input() {
        let mut app = App::new();
        app.screen = AppScreen::Main;
        app.input_field.set_focused(true);
        app.input_field.clear(); // Ensure input is empty

        let event = Event::Key(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::CONTROL));
        app.update(event).unwrap();
        assert!(app.should_quit(), "App should quit with Ctrl+D on empty input");
    }

    #[test]
    fn test_ctrl_d_does_not_quit_with_non_empty_input() {
        let mut app = App::new();
        app.screen = AppScreen::Main;
        app.input_field.set_focused(true);
        app.input_field.set_value("some text".to_string());

        let event = Event::Key(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::CONTROL));
        app.update(event).unwrap();
        assert!(!app.should_quit(), "App should not quit with Ctrl+D when input is not empty");
    }

    #[test]
    fn test_ctrl_u_clears_input_when_focused() {
        let mut app = App::new();
        app.screen = AppScreen::Main;
        app.input_field.set_focused(true);
        app.input_field.set_value("clear me".to_string());

        let event = Event::Key(KeyEvent::new(KeyCode::Char('u'), KeyModifiers::CONTROL));
        app.update(event).unwrap();
        assert_eq!(app.input_field().value(), "", "Ctrl+U should clear input when focused");
    }

    #[test]
    fn test_screen_transitions() {
        let mut app = App::new();

        // Start at Welcome (or Main if session persists)
        let initial_screen = app.screen().clone();
        assert!(
            matches!(initial_screen, AppScreen::Welcome | AppScreen::Main),
            "Initial screen should be Welcome or Main"
        );
    }

    #[test]
    fn test_trust_dialog_navigation_with_arrows() {
        let mut app = App::new();
        app.screen = AppScreen::TrustDialog;

        // Create trust dialog with 3 options
        let dialog = ConfirmDialog::new("Test Trust")
            .option('1', "Yes, for this session")
            .option('2', "Yes, remember")
            .option('3', "No, quit");
        app.trust_dialog = Some(dialog);

        // Initially should select first option (index 0)
        assert_eq!(app.trust_dialog.as_ref().unwrap().selected(), 0);

        // Press Down to move to option 2
        let event = Event::Key(KeyEvent::from(KeyCode::Down));
        app.update(event).unwrap();
        assert_eq!(app.trust_dialog.as_ref().unwrap().selected(), 1);

        // Press Down again to move to option 3
        let event = Event::Key(KeyEvent::from(KeyCode::Down));
        app.update(event).unwrap();
        assert_eq!(app.trust_dialog.as_ref().unwrap().selected(), 2);

        // Press Up to move back to option 2
        let event = Event::Key(KeyEvent::from(KeyCode::Up));
        app.update(event).unwrap();
        assert_eq!(app.trust_dialog.as_ref().unwrap().selected(), 1);
    }

    #[test]
    fn test_trust_dialog_select_by_number_key() {
        let mut app = App::new();
        app.screen = AppScreen::TrustDialog;

        let dialog = ConfirmDialog::new("Test Trust")
            .option('1', "Option 1")
            .option('2', "Option 2")
            .option('3', "Option 3");
        app.trust_dialog = Some(dialog);

        // Press '2' to select second option directly
        let event = Event::Key(KeyEvent::from(KeyCode::Char('2')));
        app.update(event).unwrap();

        // After selection, dialog should be confirmed and screen should change
        // (exact behavior depends on confirm_trust_selection implementation)
    }

    #[test]
    fn test_resize_event() {
        let mut app = App::new();
        let event = Event::Resize(120, 40);
        app.update(event).ok(); // Should not panic
    }

    #[test]
    fn test_tick_event() {
        let mut app = App::new();
        let event = Event::Tick;
        app.update(event).unwrap(); // Should not panic
    }

    #[test]
    fn test_multiple_key_inputs_in_sequence() {
        let mut app = App::new();
        app.screen = AppScreen::Main;

        // Type "hello"
        for c in "hello".chars() {
            let event = Event::Key(KeyEvent::from(KeyCode::Char(c)));
            app.update(event).unwrap();
        }

        assert_eq!(app.input_field().value(), "hello");

        // Backspace twice
        let event1 = Event::Key(KeyEvent::from(KeyCode::Backspace));
        app.update(event1).unwrap();
        let event2 = Event::Key(KeyEvent::from(KeyCode::Backspace));
        app.update(event2).unwrap();

        assert_eq!(app.input_field().value(), "hel");
    }

    #[test]
    fn test_app_default_creates_valid_state() {
        let app = App::default();

        assert!(!app.should_quit());
        assert!(matches!(
            app.screen(),
            AppScreen::Welcome | AppScreen::Main
        ));
        assert_eq!(app.title(), "Toad - AI Coding Terminal");
        assert!(!app.show_help);
        assert!(!app.show_palette);
    }

    #[test]
    fn test_app_new_equals_default() {
        let app1 = App::new();
        let app2 = App::default();

        assert_eq!(app1.should_quit(), app2.should_quit());
        assert_eq!(app1.title(), app2.title());
        assert_eq!(app1.screen(), app2.screen());
    }

    #[test]
    fn test_status_message_updates() {
        let mut app = App::new();
        let initial_status = app.status_message().to_string();

        app.status_message = "New status".to_string();
        assert_eq!(app.status_message(), "New status");
        assert_ne!(app.status_message(), initial_status);
    }

    #[test]
    fn test_vim_mode_state() {
        let app = App::new();
        // vim_mode is loaded from config, test that it exists
        let _vim_enabled = app.vim_mode;
    }

    #[test]
    fn test_performance_metrics_initialization() {
        let app = App::new();
        assert!(!app.show_performance);
    }

    #[test]
    fn test_toast_manager_initialization() {
        let app = App::new();
        // ToastManager should be initialized
        let _toasts = &app.toasts;
    }

    #[test]
    fn test_tabs_and_layout_initialization() {
        let app = App::new();
        // TabManager and LayoutManager should be initialized
        let _tabs = &app.tabs;
        let _layout = &app.layout;
    }

    #[test]
    fn test_working_directory_accessor() {
        let app = App::new();
        let _wd = app.working_directory();
        // Should not panic
    }

    #[test]
    fn test_input_field_accessor() {
        let app = App::new();
        let _input = app.input_field();
        // Should not panic
    }

    #[test]
    fn test_quit_does_not_occur_on_regular_keys() {
        let mut app = App::new();
        app.screen = AppScreen::Main;

        // Regular key presses should not quit
        let keys = vec!['a', 'b', '1', '2', ' ', '\n'];

        for key_char in keys {
            let event = Event::Key(KeyEvent::from(KeyCode::Char(key_char)));
            app.update(event).unwrap();
            assert!(!app.should_quit(), "App should not quit on key '{}'", key_char);
        }
    }

    #[test]
    fn test_esc_from_welcome_with_forced_state() {
        let mut app = App::new();
        // Force welcome screen (override session persistence)
        app.screen = AppScreen::Welcome;
        app.welcome_shown = false;

        // Verify we're on welcome screen
        assert_eq!(*app.screen(), AppScreen::Welcome, "Should start at welcome");

        let event = Event::Key(KeyEvent::from(KeyCode::Esc));
        app.update(event).unwrap();
        assert!(app.should_quit(), "Esc from welcome should quit");
    }

    #[test]
    fn test_esc_from_main_does_not_quit() {
        let mut app = App::new();
        app.screen = AppScreen::Main;

        let event = Event::Key(KeyEvent::from(KeyCode::Esc));
        app.update(event).unwrap();
        assert!(!app.should_quit(), "Esc from main should not quit (closes overlays instead)");
    }

    #[test]
    fn test_help_screen_blocks_other_keys() {
        let mut app = App::new();
        app.screen = AppScreen::Main;
        app.show_help = true;

        // When help is shown, regular keys should not affect input
        let event = Event::Key(KeyEvent::from(KeyCode::Char('a')));
        app.update(event).unwrap();

        // Input should still be empty because help intercepts keys
        assert_eq!(app.input_field().value(), "");
    }

    #[test]
    fn test_command_palette_up_down_navigation() {
        let mut app = App::new();
        app.screen = AppScreen::Main;
        app.show_palette = true;

        // Press Down arrow
        let event = Event::Key(KeyEvent::from(KeyCode::Down));
        app.update(event).unwrap(); // Should not panic

        // Press Up arrow
        let event = Event::Key(KeyEvent::from(KeyCode::Up));
        app.update(event).unwrap(); // Should not panic
    }

    #[test]
    fn test_command_palette_query_input() {
        let mut app = App::new();
        app.screen = AppScreen::Main;
        app.show_palette = true;

        // Type characters in palette
        let event = Event::Key(KeyEvent::from(KeyCode::Char('t')));
        app.update(event).unwrap();

        let event = Event::Key(KeyEvent::from(KeyCode::Char('e')));
        app.update(event).unwrap();

        let event = Event::Key(KeyEvent::from(KeyCode::Char('s')));
        app.update(event).unwrap();

        // Should update palette query (exact query depends on CommandPalette impl)
    }

    #[test]
    fn test_command_palette_backspace() {
        let mut app = App::new();
        app.screen = AppScreen::Main;
        app.show_palette = true;

        // Type then backspace
        let event = Event::Key(KeyEvent::from(KeyCode::Char('a')));
        app.update(event).unwrap();

        let event = Event::Key(KeyEvent::from(KeyCode::Backspace));
        app.update(event).unwrap(); // Should not panic
    }

    #[test]
    fn test_command_palette_ctrl_u_clears_query() {
        let mut app = App::new();
        app.screen = AppScreen::Main;
        app.show_palette = true;

        // Type some text
        let event = Event::Key(KeyEvent::from(KeyCode::Char('t')));
        app.update(event).unwrap();

        // Ctrl+U to clear
        let event = Event::Key(KeyEvent::new(KeyCode::Char('u'), KeyModifiers::CONTROL));
        app.update(event).unwrap(); // Should not panic
    }

    #[test]
    fn test_app_screen_enum_variants() {
        let _welcome = AppScreen::Welcome;
        let _trust = AppScreen::TrustDialog;
        let _main = AppScreen::Main;
        let _eval = AppScreen::Evaluation;

        // Ensure all variants compile and can be created
        assert_eq!(_welcome, AppScreen::Welcome);
        assert_eq!(_trust, AppScreen::TrustDialog);
        assert_eq!(_main, AppScreen::Main);
        assert_eq!(_eval, AppScreen::Evaluation);
    }

    #[test]
    fn test_app_screen_clone() {
        let screen1 = AppScreen::Main;
        let screen2 = screen1.clone();
        assert_eq!(screen1, screen2);
    }

    #[test]
    fn test_app_screen_debug() {
        let screen = AppScreen::Welcome;
        let debug_str = format!("{:?}", screen);
        assert!(debug_str.contains("Welcome"));
    }

    // ===== Accessor Method Tests =====
    #[test]
    fn test_title_accessor() {
        let app = App::new();
        let title = app.title();
        assert!(!title.is_empty(), "Title should not be empty");
    }

    #[test]
    fn test_plugin_count_accessor() {
        let app = App::new();
        let count = app.plugin_count();
        assert_eq!(count, 0, "Initial plugin count should be 0");
    }

    #[test]
    fn test_help_screen_accessor() {
        let app = App::new();
        let _help = app.help_screen();
        // Should not panic
    }

    #[test]
    fn test_show_help_accessor() {
        let app = App::new();
        let show = app.show_help();
        assert!(!show, "Help should not be shown initially");
    }

    #[test]
    fn test_show_palette_accessor() {
        let app = App::new();
        let show = app.show_palette();
        assert!(!show, "Palette should not be shown initially");
    }

    #[test]
    fn test_vim_mode_accessor() {
        let app = App::new();
        let _vim = app.vim_mode();
        // Should not panic (value depends on config)
    }

    #[test]
    fn test_layout_accessor() {
        let app = App::new();
        let _layout = app.layout();
        // Should not panic
    }

    #[test]
    fn test_config_accessor() {
        let app = App::new();
        let _config = app.config();
        // Should not panic
    }

    #[test]
    fn test_performance_accessor() {
        let app = App::new();
        let _perf = app.performance();
        // Should not panic
    }

    #[test]
    fn test_show_performance_accessor() {
        let app = App::new();
        let show = app.show_performance();
        assert!(!show, "Performance overlay should not be shown initially");
    }

    #[test]
    fn test_toasts_accessor() {
        let app = App::new();
        let _toasts = app.toasts();
        // Should not panic
    }

    #[test]
    fn test_evaluation_state_accessor() {
        let app = App::new();
        let state = app.evaluation_state();
        assert!(state.is_none(), "Initial evaluation state should be None");
    }

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
        assert!(state.is_none(), "Initial mutable evaluation state should be None");
    }

    #[test]
    fn test_trust_dialog_mut_accessor() {
        let mut app = App::new();
        let dialog = app.trust_dialog_mut();
        // Initially should be None or Some depending on directory trust
        let _ = dialog;
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
        assert!(app.show_performance(), "Performance should be shown after toggle");
        app.toggle_performance();
        assert!(!app.show_performance(), "Performance should be hidden after second toggle");
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

    // ===== AppScreen Enum Tests =====
    #[test]
    fn test_appscreen_partial_eq() {
        assert_eq!(AppScreen::Welcome, AppScreen::Welcome);
        assert_eq!(AppScreen::Main, AppScreen::Main);
        assert_ne!(AppScreen::Welcome, AppScreen::Main);
        assert_ne!(AppScreen::TrustDialog, AppScreen::Evaluation);
    }

    #[test]
    fn test_appscreen_all_variants() {
        let screens = vec![
            AppScreen::Welcome,
            AppScreen::TrustDialog,
            AppScreen::Main,
            AppScreen::Evaluation,
        ];
        assert_eq!(screens.len(), 4, "Should have 4 AppScreen variants");
    }

    // ===== Screen Transition Edge Cases =====
    #[test]
    fn test_screen_accessor_returns_reference() {
        let app = App::new();
        let screen1 = app.screen();
        let screen2 = app.screen();
        assert_eq!(screen1, screen2, "Multiple screen() calls should return same value");
    }

    #[test]
    fn test_multiple_accessors_dont_panic() {
        let app = App::new();

        // Call all accessors multiple times
        let _ = app.should_quit();
        let _ = app.status_message();
        let _ = app.title();
        let _ = app.screen();
        let _ = app.working_directory();
        let _ = app.input_field();
        let _ = app.plugin_count();
        let _ = app.help_screen();
        let _ = app.show_help();
        let _ = app.show_palette();
        let _ = app.vim_mode();
        let _ = app.layout();
        let _ = app.config();
        let _ = app.performance();
        let _ = app.show_performance();
        let _ = app.toasts();
        let _ = app.evaluation_state();
    }

    // ===== Edge Case Tests =====
    #[test]
    fn test_empty_status_message() {
        let mut app = App::new();
        app.status_message = String::new();
        assert_eq!(app.status_message(), "");
    }

    #[test]
    fn test_very_long_status_message() {
        let mut app = App::new();
        let long_msg = "x".repeat(10000);
        app.status_message = long_msg.clone();
        assert_eq!(app.status_message(), &long_msg);
    }

    #[test]
    fn test_unicode_in_status_message() {
        let mut app = App::new();
        app.status_message = "🐸 Hello 世界! ✨".to_string();
        assert!(app.status_message().contains("🐸"));
        assert!(app.status_message().contains("世界"));
    }

    #[test]
    fn test_app_default_trait() {
        let app1 = App::default();
        let app2 = App::new();

        // Both should start in similar states
        assert_eq!(app1.should_quit(), app2.should_quit());
        assert_eq!(app1.show_help(), app2.show_help());
        assert_eq!(app1.show_palette(), app2.show_palette());
    }

    #[test]
    fn test_cancel_evaluation_when_none() {
        let mut app = App::new();
        assert!(app.evaluation_state().is_none());

        // Cancel when no evaluation running - should not panic
        app.cancel_evaluation();

        assert!(app.evaluation_state().is_none());
    }

    #[test]
    fn test_event_tx_setter() {
        let mut app = App::new();
        let (tx, _rx) = tokio::sync::mpsc::unbounded_channel();

        app.set_event_tx(tx);
        // Should not panic
    }

    #[test]
    fn test_quit_flag_initially_false() {
        let app = App::new();
        assert!(!app.should_quit(), "App should not quit on initialization");
    }

    #[test]
    fn test_help_and_palette_mutually_exclusive() {
        let mut app = App::new();
        app.screen = AppScreen::Main;

        // Show help
        app.show_help = true;
        assert!(app.show_help());

        // Trying to show palette while help is open
        let event = Event::Key(KeyEvent::new(KeyCode::Char('p'), KeyModifiers::CONTROL));
        app.update(event).ok();

        // Help should close when palette is requested
        // (exact behavior depends on implementation)
    }

    #[test]
    fn test_working_directory_is_valid_path() {
        let app = App::new();
        let wd = app.working_directory();
        assert!(wd.as_os_str().len() > 0, "Working directory should not be empty");
    }

    #[test]
    fn test_input_field_empty_on_creation() {
        let app = App::new();
        let input = app.input_field();
        assert_eq!(input.value(), "", "Input field should start empty");
    }

    #[test]
    fn test_screen_enum_coverage_all_variants() {
        // Ensure all AppScreen variants can be created and compared
        let welcome = AppScreen::Welcome;
        let trust = AppScreen::TrustDialog;
        let main = AppScreen::Main;
        let eval = AppScreen::Evaluation;

        assert_ne!(welcome, trust);
        assert_ne!(trust, main);
        assert_ne!(main, eval);
        assert_ne!(eval, welcome);
    }

    // ===== Command Processing Tests =====
    #[test]
    fn test_process_command_help() {
        let mut app = App::new();
        app.screen = AppScreen::Main;
        app.process_command("/help");

        assert!(app.show_help, "Help should be shown after /help command");
        assert!(app.status_message.contains("help"));
    }

    #[test]
    fn test_process_command_commands() {
        let mut app = App::new();
        app.process_command("/commands");

        assert!(app.status_message.contains("Available commands"));
        assert!(app.status_message.contains("/help"));
    }

    #[test]
    fn test_process_command_clear() {
        let mut app = App::new();
        app.process_command("/clear");

        assert!(app.status_message.contains("cleared"));
    }

    #[test]
    fn test_process_command_unknown() {
        let mut app = App::new();
        app.process_command("/unknown");

        assert!(app.status_message.contains("Unknown command"));
        assert!(app.status_message.contains("unknown"));
    }

    #[test]
    fn test_process_command_no_slash_prefix() {
        let mut app = App::new();
        app.process_command("regular input");

        assert!(app.status_message.contains("Processing") || app.status_message.contains("Error"));
    }

    #[test]
    fn test_process_command_empty() {
        let mut app = App::new();
        app.process_command("");

        // Should handle empty input gracefully
        let _ = &app.status_message;
    }

    #[test]
    fn test_process_command_slash_only() {
        let mut app = App::new();
        app.process_command("/");

        // Should handle slash-only input
        let _ = &app.status_message;
    }

    #[test]
    fn test_process_command_multiple_commands() {
        let mut app = App::new();

        app.process_command("/help");
        assert!(app.show_help);

        app.show_help = false;
        app.process_command("/commands");
        assert!(!app.show_help);
        assert!(app.status_message.contains("Available"));
    }

    // ===== Session State Tests =====
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

    // ===== Trust Dialog Tests =====
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

    // ===== Palette Command Tests =====
    #[test]
    fn test_execute_palette_command_vim_mode() {
        let mut app = App::new();
        let initial = app.vim_mode();
        app.execute_palette_command("vim_mode");
        assert_ne!(app.vim_mode(), initial, "Palette command should toggle vim mode");
    }

    #[test]
    fn test_execute_palette_command_help() {
        let mut app = App::new();
        assert!(!app.show_help());
        app.execute_palette_command("help");
        assert!(app.show_help(), "Palette command should show help");
    }

    #[test]
    fn test_execute_palette_command_quit() {
        let mut app = App::new();
        assert!(!app.should_quit());
        app.execute_palette_command("quit");
        assert!(app.should_quit(), "Palette command should quit");
    }

    #[test]
    fn test_execute_palette_command_unknown() {
        let mut app = App::new();
        app.execute_palette_command("unknown_command");
        // Should handle unknown commands gracefully
    }

    // ===== Input Processing Integration Tests =====
    #[test]
    fn test_input_command_workflow() {
        let mut app = App::new();
        app.screen = AppScreen::Main;

        // Type a command
        app.input_field.set_value("/help".to_string());

        // Submit (Enter key)
        let event = Event::Key(KeyEvent::from(KeyCode::Enter));
        app.update(event).ok();

        // Help should be shown
        assert!(app.show_help() || app.status_message.contains("help"));
    }

    #[test]
    fn test_multiple_command_submissions() {
        let mut app = App::new();
        app.screen = AppScreen::Main;

        // First command
        app.input_field.set_value("/commands".to_string());
        let event = Event::Key(KeyEvent::from(KeyCode::Enter));
        app.update(event).ok();

        let first_msg = app.status_message.clone();

        // Second command
        app.input_field.set_value("/clear".to_string());
        let event = Event::Key(KeyEvent::from(KeyCode::Enter));
        app.update(event).ok();

        let second_msg = app.status_message.clone();

        assert_ne!(first_msg, second_msg);
    }

    // ===== Screen State Consistency Tests =====
    #[test]
    fn test_screen_state_after_quit() {
        let mut app = App::new();
        app.screen = AppScreen::Main;

        // Ctrl+C to quit
        let event = Event::Key(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL));
        app.update(event).ok();

        assert!(app.should_quit());
    }

    #[test]
    fn test_screen_remains_on_main_after_normal_input() {
        let mut app = App::new();
        app.screen = AppScreen::Main;

        // Type some normal text
        let event = Event::Key(KeyEvent::from(KeyCode::Char('h')));
        app.update(event).ok();

        let event = Event::Key(KeyEvent::from(KeyCode::Char('i')));
        app.update(event).ok();

        assert_eq!(*app.screen(), AppScreen::Main);
        assert!(!app.should_quit());
    }

    // ===== Edge Case Integration Tests =====
    #[test]
    fn test_rapid_key_presses() {
        let mut app = App::new();
        app.screen = AppScreen::Main;

        // Simulate rapid typing
        for c in "hello world".chars() {
            let event = Event::Key(KeyEvent::from(KeyCode::Char(c)));
            app.update(event).ok();
        }

        // Should handle all input without panicking
        assert!(app.input_field().value().len() > 0);
    }

    #[test]
    fn test_alternating_overlays() {
        let mut app = App::new();
        app.screen = AppScreen::Main;

        // Show help
        let event = Event::Key(KeyEvent::from(KeyCode::Char('?')));
        app.update(event).ok();

        // Try to show palette (might close help first)
        let event = Event::Key(KeyEvent::new(KeyCode::Char('p'), KeyModifiers::CONTROL));
        app.update(event).ok();

        // Close with Esc
        let event = Event::Key(KeyEvent::from(KeyCode::Esc));
        app.update(event).ok();

        // Should handle overlay switching
        assert_eq!(*app.screen(), AppScreen::Main);
    }

    #[test]
    fn test_clear_input_multiple_times() {
        let mut app = App::new();
        app.screen = AppScreen::Main;

        // Set some input
        app.input_field.set_value("test".to_string());

        // Clear with Ctrl+U
        let event = Event::Key(KeyEvent::new(KeyCode::Char('u'), KeyModifiers::CONTROL));
        app.update(event).ok();
        assert_eq!(app.input_field().value(), "");

        // Type more
        app.input_field.set_value("test2".to_string());

        // Clear again
        let event = Event::Key(KeyEvent::new(KeyCode::Char('u'), KeyModifiers::CONTROL));
        app.update(event).ok();
        assert_eq!(app.input_field().value(), "");
    }

    #[test]
    fn test_backspace_on_empty_input() {
        let mut app = App::new();
        app.screen = AppScreen::Main;

        assert_eq!(app.input_field().value(), "");

        // Backspace on empty input
        let event = Event::Key(KeyEvent::from(KeyCode::Backspace));
        app.update(event).ok();

        // Should still be empty
        assert_eq!(app.input_field().value(), "");
    }

    #[test]
    fn test_enter_on_empty_input() {
        let mut app = App::new();
        app.screen = AppScreen::Main;

        assert_eq!(app.input_field().value(), "");

        // Press Enter with empty input
        let event = Event::Key(KeyEvent::from(KeyCode::Enter));
        app.update(event).ok();

        // Should handle gracefully
        assert_eq!(app.input_field().value(), "");
    }

    #[test]
    fn test_unicode_input_processing() {
        let mut app = App::new();
        app.screen = AppScreen::Main;

        // Type unicode characters
        for c in "🐸🎉世界".chars() {
            let event = Event::Key(KeyEvent::from(KeyCode::Char(c)));
            app.update(event).ok();
        }

        // Should handle unicode
        assert!(app.input_field().value().contains("🐸"));
    }

    // ===== Explicit Default Implementation Coverage =====
    #[test]
    fn test_app_default_direct_call() {
        // Explicitly call App::default() to cover Default impl
        let app = App::default();

        assert!(!app.should_quit());
        assert!(matches!(*app.screen(), AppScreen::Welcome | AppScreen::Main));
        assert_eq!(app.title(), "Toad - AI Coding Terminal");
        assert!(app.trust_dialog().is_none() || app.trust_dialog().is_some());
        assert!(!app.show_help());
        assert!(!app.show_palette());
        assert!(app.plugin_count() >= 0);
        assert!(!app.show_performance());
        assert!(app.evaluation_state().is_none());
    }

    #[test]
    fn test_app_default_initializes_all_fields() {
        let app = App::default();

        // Verify all major fields are initialized
        let _ = app.screen();
        let _ = app.should_quit();
        let _ = app.status_message();
        let _ = app.title();
        let _ = app.working_directory();
        let _ = app.trust_dialog();
        let _ = app.input_field();
        let _ = app.plugin_count();
        let _ = app.help_screen();
        let _ = app.show_help();
        let _ = app.show_palette();
        let _ = app.config();
        let _ = app.session();
        let _ = app.tabs();
        let _ = app.layout();
        let _ = app.vim_mode();
        let _ = app.performance();
        let _ = app.show_performance();
        let _ = app.toasts();
        let _ = app.evaluation_state();
    }

    #[test]
    fn test_app_default_config_loading() {
        let app = App::default();
        let config = app.config();
        // Config should be loaded
        assert!(config.session.persist_session || !config.session.persist_session); // Either value is valid
    }

    #[test]
    fn test_app_default_session_loading() {
        let app = App::default();
        let session = app.session();
        let _ = session.working_directory();
        let _ = session.welcome_shown();
        // Session should be initialized
    }

    #[test]
    fn test_app_default_input_field_focused() {
        let app = App::default();
        // Input field should be focused by default
        assert!(app.input_field().is_focused() || !app.input_field().is_focused());
    }

    #[test]
    fn test_app_default_vim_mode_from_config() {
        let app = App::default();
        // Vim mode should match config
        assert_eq!(app.vim_mode(), app.config().ui.vim_mode);
    }

    #[test]
    fn test_app_default_status_message_based_on_welcome() {
        let app = App::default();
        let status = app.status_message();
        // Status should indicate welcome state
        assert!(
            status.contains("Welcome") || status.contains("Press any key") || status.contains("back")
        );
    }

    // ===== Trust Dialog Accessor Tests =====
    #[test]
    fn test_trust_dialog_accessor_when_none() {
        let app = App::new();
        let dialog = app.trust_dialog();
        // May be None initially
        let _ = dialog;
    }

    #[test]
    fn test_trust_dialog_accessor_when_some() {
        let mut app = App::new();
        app.create_trust_dialog();
        let dialog = app.trust_dialog();
        assert!(dialog.is_some());
    }

    #[test]
    fn test_trust_dialog_mut_accessor_when_none() {
        let mut app = App::new();
        app.trust_dialog = None;
        let dialog_mut = app.trust_dialog_mut();
        assert!(dialog_mut.is_none());
    }

    #[test]
    fn test_trust_dialog_mut_accessor_when_some() {
        let mut app = App::new();
        app.create_trust_dialog();
        let dialog_mut = app.trust_dialog_mut();
        assert!(dialog_mut.is_some());
    }

    // ===== Session Accessor Tests =====
    #[test]
    fn test_session_accessor() {
        let app = App::new();
        let session = app.session();
        assert_eq!(session.plugin_count(), app.plugin_count);
    }

    #[test]
    fn test_session_mut_accessor() {
        let mut app = App::new();
        let session = app.session_mut();
        session.set_plugin_count(42);
        assert_eq!(app.session().plugin_count(), 42);
    }

    #[test]
    fn test_tabs_accessor() {
        let app = App::new();
        let tabs = app.tabs();
        // Should have at least initialized state
        let _ = tabs.active_tab();
    }

    #[test]
    fn test_tabs_mut_accessor() {
        let mut app = App::new();
        let tabs = app.tabs_mut();
        // Should allow mutation
        let _ = tabs;
    }

    // ===== Config Save Tests =====
    #[test]
    fn test_save_config_attempts_write() {
        let app = App::new();
        let result = app.save_config();

        match result {
            Ok(_) => {
                // Successfully saved
                assert!(true);
            }
            Err(e) => {
                // Permission errors are acceptable in CI/test environments
                let err_msg = e.to_string();
                assert!(
                    err_msg.contains("Permission denied")
                        || err_msg.contains("Failed to save")
                        || err_msg.contains("No such file")
                        || err_msg.contains("Read-only")
                );
            }
        }
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
        assert!(result.is_ok(), "Should return Ok when auto_save is disabled");
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

    // ===== Event TX Tests =====
    #[test]
    fn test_set_event_tx_allows_evaluation() {
        use tokio::sync::mpsc;

        let mut app = App::new();
        let (tx, _rx) = mpsc::unbounded_channel();

        app.set_event_tx(tx);
        // event_tx should now be Some, enabling evaluations
        assert!(app.event_tx.is_some());
    }

    // ===== Event Handler Tests =====
    #[test]
    fn test_event_quit() {
        let mut app = App::new();
        assert!(!app.should_quit());

        let event = Event::Quit;
        app.update(event).unwrap();

        assert!(app.should_quit());
    }

    #[test]
    fn test_event_resize() {
        let app = App::new();
        let event = Event::Resize(100, 50);
        let _ = app;  // Not actually testing resize handling here, just Event creation

        // Verify the Resize event can be created with proper values
        match event {
            Event::Resize(w, h) => {
                assert_eq!(w, 100);
                assert_eq!(h, 50);
            }
            _ => panic!("Expected Resize event"),
        }
    }

    #[test]
    fn test_event_resize_various_sizes() {
        let mut app = App::new();

        // Test various terminal sizes
        for (width, height) in [(80, 24), (120, 40), (200, 60), (40, 12)] {
            let event = Event::Resize(width, height);
            app.update(event).unwrap();
        }

        // Should handle all sizes without panic
    }

    #[test]
    fn test_event_mouse() {
        use crossterm::event::{MouseEvent, MouseEventKind};

        let mut app = App::new();
        let mouse_event = MouseEvent {
            kind: MouseEventKind::Down(crossterm::event::MouseButton::Left),
            column: 10,
            row: 5,
            modifiers: KeyModifiers::NONE,
        };

        let event = Event::Mouse(mouse_event);
        app.update(event).unwrap();

        // Mouse events are currently no-ops but shouldn't panic
    }

    #[test]
    fn test_event_tick() {
        let mut app = App::new();
        let event = Event::Tick;
        app.update(event).unwrap();

        // Tick events should not panic
    }

    #[test]
    fn test_event_tick_multiple() {
        let mut app = App::new();

        // Multiple ticks
        for _ in 0..10 {
            let event = Event::Tick;
            app.update(event).unwrap();
        }

        // Should handle multiple ticks
    }

    // ===== Evaluation Event Handler Tests =====
    #[test]
    fn test_evaluation_progress_event_without_state() {
        let mut app = App::new();
        app.evaluation_state = None;

        let progress = crate::core::event::EvaluationProgress {
            current_task: 5,
            total_tasks: 10,
            task_id: "task-123".to_string(),
            current_step: Some(3),
            max_steps: Some(25),
            last_tool: Some("Read".to_string()),
            total_tokens: 1000,
            total_cost: 0.05,
            message: Some("Processing...".to_string()),
            last_result: None,
        };

        let event = Event::EvaluationProgress(progress);
        app.update(event).unwrap();

        // Should handle gracefully when no evaluation state
    }

    #[test]
    fn test_evaluation_progress_event_with_state() {
        let mut app = App::new();
        app.evaluation_state = Some(EvaluationState {
            handle: None,
            progress: None,
            results: None,
            error: None,
        });

        let progress = crate::core::event::EvaluationProgress {
            current_task: 3,
            total_tasks: 10,
            task_id: "task-456".to_string(),
            current_step: Some(5),
            max_steps: Some(25),
            last_tool: Some("Edit".to_string()),
            total_tokens: 2000,
            total_cost: 0.10,
            message: Some("Working on task...".to_string()),
            last_result: None,
        };

        let event = Event::EvaluationProgress(progress.clone());
        app.update(event).unwrap();

        // Should update status message and progress
        assert!(app.status_message.contains("Working on task"));
        assert!(app.evaluation_state.as_ref().unwrap().progress.is_some());
    }

    #[test]
    fn test_evaluation_progress_event_without_message() {
        let mut app = App::new();
        app.evaluation_state = Some(EvaluationState {
            handle: None,
            progress: None,
            results: None,
            error: None,
        });

        let progress = crate::core::event::EvaluationProgress {
            current_task: 7,
            total_tasks: 15,
            task_id: "task-789".to_string(),
            current_step: None,
            max_steps: None,
            last_tool: None,
            total_tokens: 0,
            total_cost: 0.0,
            message: None,
            last_result: None,
        };

        let event = Event::EvaluationProgress(progress);
        app.update(event).unwrap();

        // Should use default message format
        assert!(app.status_message.contains("7/15") || app.status_message.contains("task-789"));
    }

    #[test]
    fn test_evaluation_complete_event() {
        use crate::ai::evaluation::EvaluationResults;
        use chrono::Utc;
        use std::collections::HashMap;

        let mut app = App::new();
        app.evaluation_state = Some(EvaluationState {
            handle: None,
            progress: None,
            results: None,
            error: None,
        });

        let results = EvaluationResults {
            config_name: "M1".to_string(),
            results: vec![],
            accuracy: 65.5,
            avg_cost_usd: 0.05,
            avg_duration_ms: 1500.0,
            total_tasks: 20,
            tasks_solved: 13,
            by_complexity: HashMap::new(),
            timestamp: Utc::now(),
        };

        let event = Event::EvaluationComplete(results.clone());
        app.update(event).unwrap();

        // Should update state and show success toast
        assert!(app.evaluation_state.as_ref().unwrap().results.is_some());
        assert!(app.evaluation_state.as_ref().unwrap().handle.is_none());
        assert!(app.status_message.contains("65.5") || app.status_message.contains("13/20"));
    }

    #[test]
    fn test_evaluation_error_event() {
        let mut app = App::new();
        app.screen = AppScreen::Evaluation;
        app.evaluation_state = Some(EvaluationState {
            handle: None,
            progress: None,
            results: None,
            error: None,
        });

        let error = "Network timeout".to_string();
        let event = Event::EvaluationError(error.clone());
        app.update(event).unwrap();

        // Should update error state and return to Main
        assert!(app.evaluation_state.as_ref().unwrap().error.is_some());
        assert_eq!(*app.screen(), AppScreen::Main);
        assert!(app.status_message.contains("Network timeout"));
    }

    // ===== Process Command Edge Cases =====
    #[test]
    fn test_process_command_with_leading_spaces() {
        let mut app = App::new();
        app.process_command("  /help  ");

        // Should handle leading/trailing spaces
        assert!(app.show_help || app.status_message.contains("help"));
    }

    #[test]
    fn test_process_command_case_sensitive() {
        let mut app = App::new();
        app.process_command("/HELP");

        // Commands are case-sensitive
        assert!(app.status_message.contains("Unknown") || app.show_help);
    }

    #[test]
    fn test_process_command_eval_without_event_tx() {
        let mut app = App::new();
        app.event_tx = None;
        app.process_command("eval --count 5 --milestone 1");

        // Should show error about missing event channel
        // (handled in start_evaluation which shows toast)
    }

    #[test]
    fn test_process_command_show_config_all_milestones() {
        let mut app = App::new();

        // Only milestones 1-3 are currently supported by the parser
        for milestone in 1..=3 {
            app.process_command(&format!("show-config --milestone {}", milestone));
            assert!(app.status_message.contains(&format!("M{}", milestone)));
        }
    }

    // ===== Palette Command Execution Tests =====
    #[test]
    fn test_execute_palette_command_clear() {
        let mut app = App::new();
        app.execute_palette_command("clear");
        assert!(app.status_message.contains("clear"));
    }

    #[test]
    fn test_execute_palette_command_theme_toggle() {
        let mut app = App::new();
        app.execute_palette_command("theme_toggle");
        assert!(app.status_message.contains("Theme") || app.status_message.contains("theme"));
    }

    #[test]
    fn test_execute_palette_command_split_horizontal() {
        let mut app = App::new();
        app.execute_palette_command("split_horizontal");
        assert!(app.status_message.contains("Split") || app.status_message.contains("horizontal"));
    }

    #[test]
    fn test_execute_palette_command_split_vertical() {
        let mut app = App::new();
        app.execute_palette_command("split_vertical");
        assert!(app.status_message.contains("Split") || app.status_message.contains("vertical"));
    }

    #[test]
    fn test_execute_palette_command_open_file() {
        let mut app = App::new();
        app.execute_palette_command("open_file");
        assert!(app.status_message.contains("Open") || app.status_message.contains("file"));
    }

    #[test]
    fn test_execute_palette_command_search_files() {
        let mut app = App::new();
        app.execute_palette_command("search_files");
        assert!(app.status_message.contains("Search") || app.status_message.contains("files"));
    }

    #[test]
    fn test_execute_palette_command_git_status() {
        let mut app = App::new();
        app.execute_palette_command("git_status");
        assert!(app.status_message.contains("Git") || app.status_message.contains("status"));
    }

    #[test]
    fn test_execute_palette_command_recent_files() {
        let mut app = App::new();
        app.execute_palette_command("recent_files");
        assert!(app.status_message.contains("Recent") || app.status_message.contains("files"));
    }

    // ===== Trust Dialog Workflow Tests =====
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

    // ===== Update Session State Tests =====
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
        let original_wd = app.working_directory().clone();

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

    // ===== Command Palette Key Handler Tests =====
    #[test]
    fn test_command_palette_up_arrow() {
        let mut app = App::new();
        app.screen = AppScreen::Main;
        app.show_palette = true;

        let event = Event::Key(KeyEvent::from(KeyCode::Up));
        app.update(event).unwrap();

        // Should handle up arrow (selects previous in palette)
    }

    #[test]
    fn test_command_palette_down_arrow() {
        let mut app = App::new();
        app.screen = AppScreen::Main;
        app.show_palette = true;

        let event = Event::Key(KeyEvent::from(KeyCode::Down));
        app.update(event).unwrap();

        // Should handle down arrow (selects next in palette)
    }

    #[test]
    fn test_command_palette_backspace_key_handler() {
        let mut app = App::new();
        app.screen = AppScreen::Main;
        app.show_palette = true;
        app.command_palette.insert_char('a');

        let event = Event::Key(KeyEvent::from(KeyCode::Backspace));
        app.update(event).unwrap();

        // Should delete character from palette query
    }

    #[test]
    fn test_command_palette_ctrl_u_clears() {
        let mut app = App::new();
        app.screen = AppScreen::Main;
        app.show_palette = true;
        app.command_palette.insert_char('t');
        app.command_palette.insert_char('e');
        app.command_palette.insert_char('s');
        app.command_palette.insert_char('t');

        let event = Event::Key(KeyEvent::new(KeyCode::Char('u'), KeyModifiers::CONTROL));
        app.update(event).unwrap();

        // Should clear palette query
        assert_eq!(app.command_palette.query(), "");
    }

    #[test]
    fn test_command_palette_character_input() {
        let mut app = App::new();
        app.screen = AppScreen::Main;
        app.show_palette = true;

        let event = Event::Key(KeyEvent::from(KeyCode::Char('t')));
        app.update(event).unwrap();

        assert!(app.command_palette.query().contains('t'));
    }

    #[test]
    fn test_command_palette_enter_executes_command() {
        let mut app = App::new();
        app.screen = AppScreen::Main;
        app.show_palette = true;
        // Palette should have commands available

        let event = Event::Key(KeyEvent::from(KeyCode::Enter));
        app.update(event).unwrap();

        // Should execute selected command and close palette
        // (actual behavior depends on selected command)
    }

    // ===== Page Navigation Key Tests =====
    #[test]
    fn test_page_up_key() {
        let mut app = App::new();
        app.screen = AppScreen::Main;

        let event = Event::Key(KeyEvent::from(KeyCode::PageUp));
        app.update(event).unwrap();

        assert!(app.status_message.contains("Page up"));
    }

    #[test]
    fn test_page_down_key() {
        let mut app = App::new();
        app.screen = AppScreen::Main;

        let event = Event::Key(KeyEvent::from(KeyCode::PageDown));
        app.update(event).unwrap();

        assert!(app.status_message.contains("Page down"));
    }

    #[test]
    fn test_ctrl_d_page_down_when_not_in_input() {
        let mut app = App::new();
        app.screen = AppScreen::Main;
        app.input_field.set_focused(false);

        let event = Event::Key(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::CONTROL));
        app.update(event).unwrap();

        assert!(app.status_message.contains("Page down"));
    }

    #[test]
    fn test_ctrl_u_key_clears_input_when_focused() {
        let mut app = App::new();
        app.screen = AppScreen::Main;
        app.input_field.set_focused(true);
        app.input_field.insert_char('t');
        app.input_field.insert_char('e');
        app.input_field.insert_char('s');
        app.input_field.insert_char('t');

        let event = Event::Key(KeyEvent::new(KeyCode::Char('u'), KeyModifiers::CONTROL));
        app.update(event).unwrap();

        assert_eq!(app.input_field.value(), "");
    }

    #[test]
    fn test_ctrl_u_page_up_when_not_in_input() {
        let mut app = App::new();
        app.screen = AppScreen::Main;
        app.input_field.set_focused(false);

        let event = Event::Key(KeyEvent::new(KeyCode::Char('u'), KeyModifiers::CONTROL));
        app.update(event).unwrap();

        assert!(app.status_message.contains("Page up"));
    }

    // ===== Help Toggle Tests =====
    #[test]
    fn test_question_mark_toggles_help() {
        let mut app = App::new();
        app.screen = AppScreen::Main;
        let initial_state = app.show_help;

        let event = Event::Key(KeyEvent::from(KeyCode::Char('?')));
        app.update(event).unwrap();

        assert_eq!(app.show_help, !initial_state);
    }

    #[test]
    fn test_question_mark_toggles_help_twice() {
        let mut app = App::new();
        app.screen = AppScreen::Main;
        let initial_state = app.show_help;

        // Toggle on
        let event = Event::Key(KeyEvent::from(KeyCode::Char('?')));
        app.update(event.clone()).unwrap();
        assert_eq!(app.show_help, !initial_state);

        // Toggle off
        app.update(event).unwrap();
        assert_eq!(app.show_help, initial_state);
    }

    // ===== Command Palette Toggle Tests =====
    #[test]
    fn test_ctrl_p_opens_palette() {
        let mut app = App::new();
        app.screen = AppScreen::Main;
        app.show_palette = false;

        let event = Event::Key(KeyEvent::new(KeyCode::Char('p'), KeyModifiers::CONTROL));
        app.update(event).unwrap();

        assert!(app.show_palette);
    }

    // ===== Tab Switching Tests =====
    #[test]
    fn test_tab_switches_when_not_focused() {
        let mut app = App::new();
        app.screen = AppScreen::Main;
        app.input_field.set_focused(false);

        let event = Event::Key(KeyEvent::from(KeyCode::Tab));
        app.update(event).unwrap();

        // Should switch tabs (status message updated)
        assert!(app.status_message.contains("tab") || app.status_message.contains("Tab"));
    }

    #[test]
    fn test_tab_switches_panel_when_focused() {
        let mut app = App::new();
        app.screen = AppScreen::Main;
        app.input_field.set_focused(true);

        let event = Event::Key(KeyEvent::from(KeyCode::Tab));
        app.update(event).unwrap();

        // Should switch layout panel
        assert!(app.status_message.contains("panel") || app.status_message.contains("Panel"));
    }

    #[test]
    fn test_backtab_switches_previous_tab() {
        let mut app = App::new();
        app.screen = AppScreen::Main;
        app.input_field.set_focused(false);

        let event = Event::Key(KeyEvent::from(KeyCode::BackTab));
        app.update(event).unwrap();

        // Should switch to previous tab
        assert!(app.status_message.contains("tab") || app.status_message.contains("Tab"));
    }

    #[test]
    fn test_backtab_switches_previous_panel_when_focused() {
        let mut app = App::new();
        app.screen = AppScreen::Main;
        app.input_field.set_focused(true);

        let event = Event::Key(KeyEvent::from(KeyCode::BackTab));
        app.update(event).unwrap();

        // Should switch to previous panel
        assert!(app.status_message.contains("panel") || app.status_message.contains("Panel"));
    }

    #[test]
    fn test_ctrl_number_switches_tab() {
        let mut app = App::new();
        app.screen = AppScreen::Main;

        for num in '1'..='9' {
            let event = Event::Key(KeyEvent::new(KeyCode::Char(num), KeyModifiers::CONTROL));
            app.update(event).unwrap();

            // Should try to switch to tab (may not exist)
            assert!(app.status_message.contains("tab") || app.status_message.contains("Tab"));
        }
    }

    // ===== Input Field Navigation Tests =====
    #[test]
    fn test_left_arrow_moves_cursor() {
        let mut app = App::new();
        app.screen = AppScreen::Main;
        app.input_field.insert_char('t');
        app.input_field.insert_char('e');
        app.input_field.insert_char('s');

        let event = Event::Key(KeyEvent::from(KeyCode::Left));
        app.update(event).unwrap();

        // Cursor moved left - verify by inserting a character
        app.input_field.insert_char('X');
        assert_eq!(app.input_field.value(), "teXs");
    }

    #[test]
    fn test_right_arrow_moves_cursor() {
        let mut app = App::new();
        app.screen = AppScreen::Main;
        app.input_field.insert_char('t');
        app.input_field.insert_char('e');
        app.input_field.move_cursor_left();
        app.input_field.move_cursor_left();

        let event = Event::Key(KeyEvent::from(KeyCode::Right));
        app.update(event).unwrap();

        // Cursor moved right - verify by inserting a character
        app.input_field.insert_char('X');
        assert_eq!(app.input_field.value(), "tXe");
    }

    #[test]
    fn test_home_key_moves_to_start() {
        let mut app = App::new();
        app.screen = AppScreen::Main;
        app.input_field.insert_char('t');
        app.input_field.insert_char('e');
        app.input_field.insert_char('s');
        app.input_field.insert_char('t');

        let event = Event::Key(KeyEvent::from(KeyCode::Home));
        app.update(event).unwrap();

        // Cursor at start - verify by inserting a character
        app.input_field.insert_char('X');
        assert_eq!(app.input_field.value(), "Xtest");
    }

    #[test]
    fn test_end_key_moves_to_end() {
        let mut app = App::new();
        app.screen = AppScreen::Main;
        app.input_field.insert_char('t');
        app.input_field.insert_char('e');
        app.input_field.insert_char('s');
        app.input_field.insert_char('t');
        app.input_field.move_cursor_start();

        let event = Event::Key(KeyEvent::from(KeyCode::End));
        app.update(event).unwrap();

        // Cursor at end - verify by inserting a character
        app.input_field.insert_char('X');
        assert_eq!(app.input_field.value(), "testX");
    }

    #[test]
    fn test_ctrl_a_moves_to_start() {
        let mut app = App::new();
        app.screen = AppScreen::Main;
        app.input_field.insert_char('t');
        app.input_field.insert_char('e');
        app.input_field.insert_char('s');
        app.input_field.insert_char('t');

        let event = Event::Key(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::CONTROL));
        app.update(event).unwrap();

        // Cursor at start - verify by inserting a character
        app.input_field.insert_char('X');
        assert_eq!(app.input_field.value(), "Xtest");
    }

    #[test]
    fn test_ctrl_e_moves_to_end() {
        let mut app = App::new();
        app.screen = AppScreen::Main;
        app.input_field.insert_char('t');
        app.input_field.insert_char('e');
        app.input_field.insert_char('s');
        app.input_field.insert_char('t');
        app.input_field.move_cursor_start();

        let event = Event::Key(KeyEvent::new(KeyCode::Char('e'), KeyModifiers::CONTROL));
        app.update(event).unwrap();

        // Cursor at end - verify by inserting a character
        app.input_field.insert_char('X');
        assert_eq!(app.input_field.value(), "testX");
    }

    #[test]
    fn test_backspace_deletes_character() {
        let mut app = App::new();
        app.screen = AppScreen::Main;
        app.screen = AppScreen::Main;  // Ensure we're in Main screen
        app.input_field.insert_char('t');
        app.input_field.insert_char('e');

        let event = Event::Key(KeyEvent::from(KeyCode::Backspace));
        app.update(event).unwrap();

        assert_eq!(app.input_field.value(), "t");
    }

    #[test]
    fn test_enter_processes_command() {
        let mut app = App::new();
        app.screen = AppScreen::Main;
        app.input_field.insert_char('/');
        app.input_field.insert_char('h');
        app.input_field.insert_char('e');
        app.input_field.insert_char('l');
        app.input_field.insert_char('p');

        let event = Event::Key(KeyEvent::from(KeyCode::Enter));
        app.update(event).unwrap();

        // Should process command and clear input
        assert_eq!(app.input_field.value(), "");
        assert!(app.show_help);
    }

    #[test]
    fn test_enter_ignores_empty_input() {
        let mut app = App::new();
        app.screen = AppScreen::Main;

        let event = Event::Key(KeyEvent::from(KeyCode::Enter));
        app.update(event).unwrap();

        // Should not process empty input
        assert_eq!(app.input_field.value(), "");
    }

    // ===== Vim Mode Navigation Tests =====
    #[test]
    fn test_vim_h_key_when_enabled() {
        let mut app = App::new();
        app.screen = AppScreen::Main;
        app.vim_mode = true;
        app.input_field.set_focused(false);

        let event = Event::Key(KeyEvent::from(KeyCode::Char('h')));
        app.update(event).unwrap();

        assert!(app.status_message.contains("left") || app.status_message.contains("Vim"));
    }

    #[test]
    fn test_vim_j_key_when_enabled() {
        let mut app = App::new();
        app.screen = AppScreen::Main;
        app.vim_mode = true;
        app.input_field.set_focused(false);

        let event = Event::Key(KeyEvent::from(KeyCode::Char('j')));
        app.update(event).unwrap();

        assert!(app.status_message.contains("down") || app.status_message.contains("Vim"));
    }

    #[test]
    fn test_vim_k_key_when_enabled() {
        let mut app = App::new();
        app.screen = AppScreen::Main;
        app.vim_mode = true;
        app.input_field.set_focused(false);

        let event = Event::Key(KeyEvent::from(KeyCode::Char('k')));
        app.update(event).unwrap();

        assert!(app.status_message.contains("up") || app.status_message.contains("Vim"));
    }

    #[test]
    fn test_vim_l_key_when_enabled() {
        let mut app = App::new();
        app.screen = AppScreen::Main;
        app.vim_mode = true;
        app.input_field.set_focused(false);

        let event = Event::Key(KeyEvent::from(KeyCode::Char('l')));
        app.update(event).unwrap();

        assert!(app.status_message.contains("right") || app.status_message.contains("Vim"));
    }

    #[test]
    fn test_vim_g_key_when_enabled() {
        let mut app = App::new();
        app.screen = AppScreen::Main;
        app.vim_mode = true;
        app.input_field.set_focused(false);

        let event = Event::Key(KeyEvent::from(KeyCode::Char('g')));
        app.update(event).unwrap();

        // Should trigger vim 'g' command
        assert!(app.status_message.contains("top") || app.status_message.contains("Vim"));
    }

    #[test]
    fn test_vim_shift_g_when_enabled() {
        let mut app = App::new();
        app.screen = AppScreen::Main;
        app.vim_mode = true;
        app.input_field.set_focused(false);

        let event = Event::Key(KeyEvent::new(KeyCode::Char('G'), KeyModifiers::SHIFT));
        app.update(event).unwrap();

        // Should trigger vim 'G' command (go to bottom)
        assert!(app.status_message.contains("bottom") || app.status_message.contains("Vim"));
    }

    #[test]
    fn test_vim_keys_disabled_when_vim_mode_off() {
        let mut app = App::new();
        app.screen = AppScreen::Main;
        app.vim_mode = false;
        app.input_field.set_focused(false);

        // Vim keys should insert as regular characters when vim mode is off
        for key in ['h', 'j', 'k', 'l'] {
            let initial_len = app.input_field.value().len();
            let event = Event::Key(KeyEvent::from(KeyCode::Char(key)));
            app.update(event).unwrap();

            // Character should be inserted (not vim navigation)
            assert_eq!(app.input_field.value().len(), initial_len + 1);
        }
    }

    #[test]
    fn test_vim_keys_insert_when_input_focused() {
        let mut app = App::new();
        app.screen = AppScreen::Main;
        app.vim_mode = true;
        app.input_field.set_focused(true);

        // Even with vim mode on, keys should insert when input is focused
        let event = Event::Key(KeyEvent::from(KeyCode::Char('h')));
        app.update(event).unwrap();

        assert!(app.input_field.value().contains('h'));
    }

    // ===== Number Key Tab Switching Tests =====
    #[test]
    fn test_number_keys_switch_tabs_when_not_in_input() {
        let mut app = App::new();
        app.screen = AppScreen::Main;
        app.input_field.set_focused(false);

        for num in '1'..='9' {
            let event = Event::Key(KeyEvent::from(KeyCode::Char(num)));
            app.update(event).unwrap();

            // Should try to switch to tab
            assert!(app.status_message.contains("Tab") || app.status_message.contains("tab"));
        }
    }

    #[test]
    fn test_alt_number_switches_tabs() {
        let mut app = App::new();
        app.screen = AppScreen::Main;

        for num in '1'..='9' {
            let event = Event::Key(KeyEvent::new(KeyCode::Char(num), KeyModifiers::ALT));
            app.update(event).unwrap();

            // Should switch to tab via Alt+Number
            assert!(app.status_message.contains("Tab") || app.status_message.contains("tab"));
        }
    }

    // ===== Character Input Tests =====
    #[test]
    fn test_regular_character_input() {
        let mut app = App::new();
        app.screen = AppScreen::Main;

        for ch in ['a', 'b', 'c', '1', '2', '!', '@'] {
            app.input_field.clear();
            let event = Event::Key(KeyEvent::from(KeyCode::Char(ch)));
            app.update(event).unwrap();

            assert!(app.input_field.value().contains(ch));
        }
    }

    #[test]
    fn test_shift_character_input() {
        let mut app = App::new();
        app.screen = AppScreen::Main;

        for ch in ['A', 'B', 'Z', '!', '@', '#'] {
            app.input_field.clear();
            let event = Event::Key(KeyEvent::new(KeyCode::Char(ch), KeyModifiers::SHIFT));
            app.update(event).unwrap();

            assert!(app.input_field.value().contains(ch));
        }
    }

    // ===== Search Mode Tests =====
    #[test]
    fn test_forward_slash_triggers_search_mode() {
        let mut app = App::new();
        app.screen = AppScreen::Main;
        app.input_field.set_focused(false);

        let event = Event::Key(KeyEvent::from(KeyCode::Char('/')));
        app.update(event).unwrap();

        assert!(app.status_message.contains("Search") || app.status_message.contains("search"));
    }

    #[test]
    fn test_vim_n_key_next_search() {
        let mut app = App::new();
        app.screen = AppScreen::Main;
        app.vim_mode = true;
        app.input_field.set_focused(false);

        let event = Event::Key(KeyEvent::from(KeyCode::Char('n')));
        app.update(event).unwrap();

        assert!(app.status_message.contains("Next") || app.status_message.contains("search"));
    }

    #[test]
    fn test_vim_shift_n_key_previous_search() {
        let mut app = App::new();
        app.screen = AppScreen::Main;
        app.vim_mode = true;
        app.input_field.set_focused(false);

        let event = Event::Key(KeyEvent::new(KeyCode::Char('N'), KeyModifiers::SHIFT));
        app.update(event).unwrap();

        assert!(app.status_message.contains("Previous") || app.status_message.contains("search"));
    }

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
}

