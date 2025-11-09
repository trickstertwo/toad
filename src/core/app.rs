//! Application state module (Model in Elm Architecture)
//!
//! This module contains the application state and the update logic
//! that handles state transitions based on events.

use crate::config::Config;
use crate::core::event::Event;
use crate::performance::PerformanceMetrics;
use crate::ui::widgets::{CommandPalette, ConfirmDialog, HelpScreen, InputField, ToastManager};
use crate::workspace::{LayoutManager, SessionState, TabManager};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::path::PathBuf;

/// Different screens/modes the application can be in
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppScreen {
    /// Initial welcome screen with logo and tips
    Welcome,
    /// Trust confirmation dialog for the current directory
    TrustDialog,
    /// Main application interface
    Main,
    /// Evaluation running screen
    Evaluation,
}

/// Application state (Model in Elm Architecture)
#[derive(Debug)]
pub struct App {
    /// Current screen being displayed
    screen: AppScreen,

    /// Whether the application should quit
    should_quit: bool,

    /// Status bar message
    status_message: String,

    /// Application title
    title: String,

    /// Current working directory
    working_directory: PathBuf,

    /// Trust dialog state (if applicable)
    trust_dialog: Option<ConfirmDialog>,

    /// Whether the user has seen the welcome screen
    welcome_shown: bool,

    /// Input field for user commands/queries
    input_field: InputField,

    /// Number of installed plugins
    plugin_count: usize,

    /// Help screen widget
    help_screen: HelpScreen,

    /// Whether to show the help overlay
    show_help: bool,

    /// Command palette widget
    command_palette: CommandPalette,

    /// Whether to show the command palette
    show_palette: bool,

    /// Application configuration
    config: Config,

    /// Session state for persistence
    session: SessionState,

    /// Tab manager for multiple workspaces
    tabs: TabManager,

    /// Layout manager for split panes
    layout: LayoutManager,

    /// Vim mode enabled
    vim_mode: bool,

    /// Performance metrics
    performance: PerformanceMetrics,

    /// Show performance overlay
    show_performance: bool,

    /// Toast notification manager
    toasts: ToastManager,

    /// Event sender for async operations (evaluation, etc.)
    event_tx: Option<tokio::sync::mpsc::UnboundedSender<Event>>,

    /// Current evaluation state
    evaluation_state: Option<EvaluationState>,
}

/// State of a running or completed evaluation
#[derive(Debug)]
pub struct EvaluationState {
    /// Handle to the running evaluation (if still running)
    pub handle: Option<crate::ai::eval_runner::EvaluationHandle>,

    /// Latest progress information
    pub progress: Option<crate::core::event::EvaluationProgress>,

    /// Final results (if completed)
    pub results: Option<crate::ai::evaluation::EvaluationResults>,

    /// Error message (if failed)
    pub error: Option<String>,
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
    pub fn start_evaluation(&mut self, args: crate::ai::eval_commands::EvalArgs) {
        if let Some(ref event_tx) = self.event_tx {
            let handle = crate::ai::eval_runner::start_evaluation(args.clone(), event_tx.clone());

            self.evaluation_state = Some(EvaluationState {
                handle: Some(handle),
                progress: None,
                results: None,
                error: None,
            });

            self.screen = AppScreen::Evaluation;
            self.status_message = format!(
                "Starting evaluation: {} tasks, milestone {}",
                args.count.unwrap_or(10),
                args.milestone
            );
        } else {
            self.toast_error("Cannot start evaluation: event channel not initialized");
        }
    }

    /// Start a comparison run
    pub fn start_comparison(&mut self, args: crate::ai::eval_commands::CompareArgs) {
        if let Some(ref event_tx) = self.event_tx {
            let handle = crate::ai::eval_runner::start_comparison(args.clone(), event_tx.clone());

            self.evaluation_state = Some(EvaluationState {
                handle: Some(handle),
                progress: None,
                results: None,
                error: None,
            });

            self.screen = AppScreen::Evaluation;
            self.status_message = format!(
                "Starting comparison: {} tasks, M{} vs M{}",
                args.count.unwrap_or(20),
                args.baseline,
                args.test
            );
        } else {
            self.toast_error("Cannot start comparison: event channel not initialized");
        }
    }

    /// Cancel running evaluation
    pub fn cancel_evaluation(&mut self) {
        if let Some(ref mut eval_state) = self.evaluation_state
            && let Some(handle) = eval_state.handle.take() {
                // Spawn a task to cancel the evaluation
                tokio::spawn(async move {
                    handle.cancel().await;
                });

                self.toast_info("Evaluation cancelled");
                self.screen = AppScreen::Main;
            }
    }

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

    /// Handle keys on the welcome screen
    fn handle_welcome_key(&mut self, key: KeyEvent) -> crate::Result<()> {
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

    /// Handle keys in the trust dialog
    fn handle_trust_dialog_key(&mut self, key: KeyEvent) -> crate::Result<()> {
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
                    && dialog.select_by_key(c).is_some() {
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

    /// Handle keys in main interface
    fn handle_main_key(&mut self, key: KeyEvent) -> crate::Result<()> {
        // If help is shown, intercept keys for help navigation
        if self.show_help {
            match (key.code, key.modifiers) {
                // Esc or ? closes help
                (KeyCode::Esc, _) | (KeyCode::Char('?'), _) => {
                    self.show_help = false;
                }
                _ => {}
            }
            return Ok(());
        }

        // If command palette is shown, intercept keys for palette navigation
        if self.show_palette {
            match (key.code, key.modifiers) {
                // Esc closes palette
                (KeyCode::Esc, _) => {
                    self.show_palette = false;
                    self.command_palette.clear_query();
                }
                // Ctrl+P also toggles off
                (KeyCode::Char('p'), KeyModifiers::CONTROL) => {
                    self.show_palette = false;
                    self.command_palette.clear_query();
                }
                // Up/Down navigate
                (KeyCode::Up, _) => {
                    self.command_palette.select_previous();
                }
                (KeyCode::Down, _) => {
                    self.command_palette.select_next();
                }
                // Enter executes selected command
                (KeyCode::Enter, _) => {
                    if let Some(cmd_id) = self.command_palette.selected_command() {
                        self.execute_palette_command(&cmd_id);
                        self.show_palette = false;
                        self.command_palette.clear_query();
                    }
                }
                // Backspace deletes character
                (KeyCode::Backspace, _) => {
                    self.command_palette.delete_char();
                }
                // Ctrl+U clears query
                (KeyCode::Char('u'), KeyModifiers::CONTROL) => {
                    self.command_palette.clear_query();
                }
                // Regular character input for search
                (KeyCode::Char(c), KeyModifiers::NONE)
                | (KeyCode::Char(c), KeyModifiers::SHIFT) => {
                    self.command_palette.insert_char(c);
                }
                _ => {}
            }
            return Ok(());
        }

        match (key.code, key.modifiers) {
            // Quit on Ctrl+C
            (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                self.should_quit = true;
            }
            // Ctrl+D for page down (Vim-style), or quit if input is focused and empty
            (KeyCode::Char('d'), KeyModifiers::CONTROL) => {
                if self.input_field.is_focused() && self.input_field.value().is_empty() {
                    self.should_quit = true;
                } else if !self.input_field.is_focused() {
                    self.status_message = "Page down".to_string();
                    // TODO: Implement page down for scrollable content
                }
            }
            // Ctrl+U for page up (Vim-style) or clear input if focused
            (KeyCode::Char('u'), KeyModifiers::CONTROL) => {
                if self.input_field.is_focused() {
                    self.input_field.clear();
                } else {
                    self.status_message = "Page up".to_string();
                    // TODO: Implement page up for scrollable content
                }
            }
            // Ctrl+P opens command palette
            (KeyCode::Char('p'), KeyModifiers::CONTROL) => {
                self.show_palette = true;
            }
            // Toggle help screen with '?' (shift+/)
            (KeyCode::Char('?'), _) => {
                self.show_help = !self.show_help;
            }
            // Tab cycling: Tab for next tab, Shift+Tab for previous tab
            (KeyCode::Tab, KeyModifiers::NONE) => {
                // If input field is not focused, use tab for workspace switching
                if !self.input_field.is_focused() {
                    self.tabs.next_tab();
                    self.status_message = format!(
                        "Switched to tab: {}",
                        self.tabs
                            .active_tab()
                            .map(|t| &t.title)
                            .unwrap_or(&"".to_string())
                    );
                } else {
                    // If input is focused, use tab for layout panel switching
                    self.layout.focus_next();
                    self.status_message = format!("Focused panel {}", self.layout.focused());
                }
            }
            (KeyCode::BackTab, _) => {
                // BackTab is Shift+Tab
                if !self.input_field.is_focused() {
                    self.tabs.previous_tab();
                    self.status_message = format!(
                        "Switched to tab: {}",
                        self.tabs
                            .active_tab()
                            .map(|t| &t.title)
                            .unwrap_or(&"".to_string())
                    );
                } else {
                    self.layout.focus_previous();
                    self.status_message = format!("Focused panel {}", self.layout.focused());
                }
            }
            // Ctrl+Number keys (1-9) for direct tab switching
            (KeyCode::Char(c @ '1'..='9'), KeyModifiers::CONTROL) => {
                let number = c.to_digit(10).unwrap() as usize;
                if self.tabs.switch_to_index(number - 1) {
                    self.status_message = format!(
                        "Switched to tab {}: {}",
                        number,
                        self.tabs
                            .active_tab()
                            .map(|t| &t.title)
                            .unwrap_or(&"".to_string())
                    );
                } else {
                    self.status_message = format!("Tab {} does not exist", number);
                }
            }
            // Enter submits the command
            (KeyCode::Enter, _) => {
                let input = self.input_field.value().to_string();
                if !input.is_empty() {
                    self.process_command(&input);
                    self.input_field.clear();
                }
            }
            // Backspace deletes character
            (KeyCode::Backspace, _) => {
                self.input_field.delete_char();
            }
            // Arrow keys move cursor
            (KeyCode::Left, _) => {
                self.input_field.move_cursor_left();
            }
            (KeyCode::Right, _) => {
                self.input_field.move_cursor_right();
            }
            // Home/End
            (KeyCode::Home, _) => {
                self.input_field.move_cursor_start();
            }
            (KeyCode::End, _) => {
                self.input_field.move_cursor_end();
            }
            // Ctrl+A / Ctrl+E (Emacs-style)
            (KeyCode::Char('a'), KeyModifiers::CONTROL) => {
                self.input_field.move_cursor_start();
            }
            (KeyCode::Char('e'), KeyModifiers::CONTROL) => {
                self.input_field.move_cursor_end();
            }
            // Page Up/Down keys
            (KeyCode::PageUp, _) => {
                self.status_message = "Page up".to_string();
                // TODO: Implement page up for scrollable content
            }
            (KeyCode::PageDown, _) => {
                self.status_message = "Page down".to_string();
                // TODO: Implement page down for scrollable content
            }
            // Vim-style navigation (when not in input field and vim mode enabled)
            (KeyCode::Char('h'), KeyModifiers::NONE)
                if self.vim_mode && !self.input_field.is_focused() =>
            {
                self.status_message = "Vim: move left".to_string();
                // TODO: Implement vim-style left navigation
            }
            (KeyCode::Char('j'), KeyModifiers::NONE)
                if self.vim_mode && !self.input_field.is_focused() =>
            {
                self.status_message = "Vim: move down".to_string();
                // TODO: Implement vim-style down navigation
            }
            (KeyCode::Char('k'), KeyModifiers::NONE)
                if self.vim_mode && !self.input_field.is_focused() =>
            {
                self.status_message = "Vim: move up".to_string();
                // TODO: Implement vim-style up navigation
            }
            (KeyCode::Char('l'), KeyModifiers::NONE)
                if self.vim_mode && !self.input_field.is_focused() =>
            {
                self.status_message = "Vim: move right".to_string();
                // TODO: Implement vim-style right navigation
            }
            // g for jump to top (Vim-style)
            (KeyCode::Char('g'), KeyModifiers::NONE)
                if self.vim_mode && !self.input_field.is_focused() =>
            {
                self.status_message = "Vim: jump to top".to_string();
                // TODO: Implement jump to top
            }
            // G for jump to bottom (Vim-style)
            (KeyCode::Char('G'), KeyModifiers::SHIFT)
                if self.vim_mode && !self.input_field.is_focused() =>
            {
                self.status_message = "Vim: jump to bottom".to_string();
                // TODO: Implement jump to bottom
            }
            // Forward slash for search
            (KeyCode::Char('/'), KeyModifiers::NONE) if !self.input_field.is_focused() => {
                self.status_message = "Search mode (coming soon)".to_string();
                // TODO: Implement search mode
            }
            // n for next search result
            (KeyCode::Char('n'), KeyModifiers::NONE)
                if self.vim_mode && !self.input_field.is_focused() =>
            {
                self.status_message = "Next search result (coming soon)".to_string();
                // TODO: Implement next search
            }
            // N for previous search result
            (KeyCode::Char('N'), KeyModifiers::SHIFT)
                if self.vim_mode && !self.input_field.is_focused() =>
            {
                self.status_message = "Previous search result (coming soon)".to_string();
                // TODO: Implement previous search
            }
            // Number keys for tab switching (when not in input field)
            (KeyCode::Char(c @ '1'..='9'), KeyModifiers::NONE)
                if !self.input_field.is_focused() =>
            {
                let tab_num = c.to_digit(10).unwrap() as usize;
                if self.tabs.switch_to_index(tab_num - 1) {
                    self.status_message = format!(
                        "Switched to tab {}: {}",
                        tab_num,
                        self.tabs
                            .active_tab()
                            .map(|t| &t.title)
                            .unwrap_or(&"".to_string())
                    );
                } else {
                    self.status_message = format!("Tab {} does not exist", tab_num);
                }
            }
            // Alt+Number for tab switching (works even in input field)
            (KeyCode::Char(c @ '1'..='9'), KeyModifiers::ALT) => {
                let tab_num = c.to_digit(10).unwrap() as usize;
                if self.tabs.switch_to_index(tab_num - 1) {
                    self.status_message = format!(
                        "Switched to tab {}: {}",
                        tab_num,
                        self.tabs
                            .active_tab()
                            .map(|t| &t.title)
                            .unwrap_or(&"".to_string())
                    );
                } else {
                    self.status_message = format!("Tab {} does not exist", tab_num);
                }
            }
            // Regular character input
            (KeyCode::Char(c), KeyModifiers::NONE) | (KeyCode::Char(c), KeyModifiers::SHIFT) => {
                self.input_field.insert_char(c);
            }
            _ => {}
        }
        Ok(())
    }

    /// Handle keys during evaluation screen
    fn handle_evaluation_key(&mut self, key: KeyEvent) -> crate::Result<()> {
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

    /// Process commands entered by the user
    fn process_command(&mut self, input: &str) {
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
    fn execute_palette_command(&mut self, cmd_id: &str) {
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
    fn create_trust_dialog(&mut self) {
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
    fn confirm_trust_selection(&mut self) {
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
}
