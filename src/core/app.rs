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
    pub(crate) title: String,

    /// Current working directory
    pub(crate) working_directory: PathBuf,

    /// Trust dialog state (if applicable)
    pub(crate) trust_dialog: Option<ConfirmDialog>,

    /// Whether the user has seen the welcome screen
    pub(crate) welcome_shown: bool,

    /// Input field for user commands/queries
    pub(crate) input_field: InputField,

    /// Number of installed plugins
    pub(crate) plugin_count: usize,

    /// Help screen widget
    pub(crate) help_screen: HelpScreen,

    /// Whether to show the help overlay
    pub(crate) show_help: bool,

    /// Command palette widget
    pub(crate) command_palette: CommandPalette,

    /// Whether to show the command palette
    pub(crate) show_palette: bool,

    /// Application configuration
    pub(crate) config: Config,

    /// Session state for persistence
    pub(crate) session: SessionState,

    /// Tab manager for multiple workspaces
    pub(crate) tabs: TabManager,

    /// Layout manager for split panes
    pub(crate) layout: LayoutManager,

    /// Vim mode enabled
    pub(crate) vim_mode: bool,

    /// Performance metrics
    pub(crate) performance: PerformanceMetrics,

    /// Show performance overlay
    pub(crate) show_performance: bool,

    /// Toast notification manager
    pub(crate) toasts: ToastManager,

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
}
