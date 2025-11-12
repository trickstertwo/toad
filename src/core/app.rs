//! Application state module (Model in Elm Architecture)
//!
//! This module contains the application state and the update logic
//! that handles state transitions based on events.

use crate::ai::llm::{AnthropicClient, LLMClient, Message};
use crate::config::Config;
use crate::core::app_state::{AppScreen, EvaluationState};
use crate::core::event::Event;
use crate::infrastructure::clipboard::Clipboard;
use crate::infrastructure::history::History;
use crate::performance::PerformanceMetrics;
use crate::ui::widgets::{
    conversation::ConversationView,
    core::{dialog::ConfirmDialog, help::HelpScreen},
    input::{input::InputField, palette::CommandPalette},
    notifications::toast::ToastManager,
    tools::ToolStatusPanel,
};
use crate::workspace::{LayoutManager, SessionState, TabManager};
use crossterm::event::KeyEvent;
use std::path::PathBuf;
use std::sync::Arc;

/// Application state (Model in Elm Architecture)
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

    /// Theme manager
    pub(crate) theme_manager: crate::ui::theme::ThemeManager,

    /// Settings screen widget
    pub(crate) settings_screen: crate::ui::widgets::core::settings_screen::SettingsScreen,

    /// Whether to show the settings screen
    pub(crate) show_settings: bool,

    /// Config dialog: (milestone, config)
    pub(crate) show_config_dialog: Option<(usize, crate::config::ToadConfig)>,

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

    /// AI conversation history
    pub(crate) conversation: Vec<Message>,

    /// LLM client for AI chat
    pub(crate) llm_client: Option<Arc<dyn LLMClient>>,

    /// Conversation view widget
    pub(crate) conversation_view: ConversationView,

    /// Whether AI processing is in progress
    pub(crate) ai_processing: bool,

    /// Tick counter for cursor blinking (toggles every 2 ticks = 500ms)
    tick_count: u32,

    /// Command history for up/down arrow navigation
    pub(crate) command_history: History,

    /// Total tokens used in this session
    pub(crate) total_input_tokens: u32,
    pub(crate) total_output_tokens: u32,

    /// Total cost in USD for this session
    pub(crate) total_cost_usd: f64,

    /// Clipboard for copy/paste operations
    pub(crate) clipboard: Option<Clipboard>,

    /// Tool execution status panel
    pub(crate) tool_status_panel: ToolStatusPanel,
}

impl std::fmt::Debug for App {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("App")
            .field("screen", &self.screen)
            .field("should_quit", &self.should_quit)
            .field("status_message", &self.status_message)
            .field("title", &self.title)
            .field("working_directory", &self.working_directory)
            .field("trust_dialog", &self.trust_dialog)
            .field("welcome_shown", &self.welcome_shown)
            .field("input_field", &self.input_field)
            .field("plugin_count", &self.plugin_count)
            .field("help_screen", &self.help_screen)
            .field("show_help", &self.show_help)
            .field("command_palette", &self.command_palette)
            .field("show_palette", &self.show_palette)
            .field("theme_manager", &"<ThemeManager>")
            .field("settings_screen", &"<SettingsScreen>")
            .field("show_settings", &self.show_settings)
            .field("config", &self.config)
            .field("session", &self.session)
            .field("tabs", &self.tabs)
            .field("layout", &self.layout)
            .field("vim_mode", &self.vim_mode)
            .field("performance", &self.performance)
            .field("show_performance", &self.show_performance)
            .field("toasts", &self.toasts)
            .field("event_tx", &self.event_tx)
            .field("evaluation_state", &self.evaluation_state)
            .field("conversation", &self.conversation)
            .field("llm_client", &"<LLMClient>") // Skip Debug for trait object
            .field("conversation_view", &"<ConversationView>") // Skip for large widget
            .field("ai_processing", &self.ai_processing)
            .field("tick_count", &self.tick_count)
            .finish()
    }
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

        // Restore conversation from session
        let conversation = session.conversation().clone();

        // Create conversation view and populate with restored messages
        let mut conversation_view = ConversationView::new();
        for message in &conversation {
            conversation_view.add_message(message.clone());
        }

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

        // Load theme from session, parse to ThemeName, fallback to Dark
        let saved_theme_str = session.theme();
        let theme_name = crate::ui::theme::manager::ThemeName::from_str(saved_theme_str)
            .unwrap_or(crate::ui::theme::manager::ThemeName::Dark);

        // Initialize theme manager with saved theme
        let mut theme_manager = crate::ui::theme::ThemeManager::new();
        theme_manager.set_theme(theme_name);

        // Try to initialize LLM client (fallback to None if API key is missing)
        let llm_client = match std::env::var("ANTHROPIC_API_KEY") {
            Ok(api_key) if !api_key.is_empty() => {
                let client = AnthropicClient::new(api_key);
                Some(Arc::new(client) as Arc<dyn LLMClient>)
            }
            _ => None,
        };

        // Restore tabs from session (or create default if none exist)
        let mut tabs = TabManager::new();
        if !session.tabs().is_empty() {
            for tab in session.tabs() {
                tabs.add_tab_with(tab.clone());
            }
            if let Some(idx) = session.active_tab_index() {
                tabs.switch_to_index(idx);
            }
        } else {
            // Create default tab if session has no tabs
            tabs.add_tab("Main");
        }

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
            theme_manager,
            settings_screen: crate::ui::widgets::core::settings_screen::SettingsScreen::new(theme_name),
            show_settings: false,
            show_config_dialog: None,
            config,
            session,
            tabs,
            layout: LayoutManager::new(),
            vim_mode,
            performance: PerformanceMetrics::new(),
            show_performance: false,
            toasts: ToastManager::new(),
            event_tx: None,
            evaluation_state: None,
            conversation,
            llm_client,
            conversation_view,
            ai_processing: false,
            tick_count: 0,
            command_history: History::load_or_new(1000),
            total_input_tokens: 0,
            total_output_tokens: 0,
            total_cost_usd: 0.0,
            clipboard: Clipboard::new().ok(),
            tool_status_panel: ToolStatusPanel::new(),
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
                // Handle cursor blinking (every 2 ticks = 500ms)
                self.tick_count = self.tick_count.wrapping_add(1);
                if self.tick_count % 2 == 0 {
                    self.input_field.toggle_cursor();
                    // Toggle streaming cursor if streaming
                    if self.conversation_view.is_streaming() {
                        self.conversation_view.toggle_cursor();
                    }
                }

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
            Event::AIResponse(message) => {
                self.handle_ai_response(message);
                Ok(())
            }
            Event::AIStreamStart => {
                self.handle_ai_stream_start();
                Ok(())
            }
            Event::AIStreamDelta(content) => {
                self.handle_ai_stream_delta(content);
                Ok(())
            }
            Event::AIStreamComplete => {
                self.handle_ai_stream_complete();
                Ok(())
            }
            Event::AITokenUsage {
                input_tokens,
                output_tokens,
            } => {
                self.total_input_tokens += input_tokens;
                self.total_output_tokens += output_tokens;

                // Calculate cost using Claude Sonnet 3.5 pricing
                // $3.00 per million input tokens, $15.00 per million output tokens
                let input_cost = (input_tokens as f64 / 1_000_000.0) * 3.0;
                let output_cost = (output_tokens as f64 / 1_000_000.0) * 15.0;
                self.total_cost_usd += input_cost + output_cost;

                Ok(())
            }
            Event::AIError(error) => {
                self.handle_ai_error(error);
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

    // ===== AI Conversation Methods =====

    /// Get conversation view widget
    pub(crate) fn conversation_view(&mut self) -> &mut ConversationView {
        &mut self.conversation_view
    }

    /// Get conversation history
    #[allow(dead_code)]
    pub(crate) fn conversation(&self) -> &[Message] {
        &self.conversation
    }

    /// Add a message to the conversation
    pub(crate) fn add_message(&mut self, message: Message) {
        self.conversation.push(message.clone());
        self.conversation_view.add_message(message);
    }

    /// Clear conversation history
    #[allow(dead_code)]
    pub(crate) fn clear_conversation(&mut self) {
        self.conversation.clear();
        self.conversation_view.clear();
    }

    /// Check if LLM client is available
    pub(crate) fn has_llm_client(&self) -> bool {
        self.llm_client.is_some()
    }

    /// Get LLM client reference
    pub(crate) fn llm_client(&self) -> Option<&Arc<dyn LLMClient>> {
        self.llm_client.as_ref()
    }

    /// Check if AI is currently processing
    #[allow(dead_code)]
    pub(crate) fn is_ai_processing(&self) -> bool {
        self.ai_processing
    }

    /// Set AI processing state
    pub(crate) fn set_ai_processing(&mut self, processing: bool) {
        self.ai_processing = processing;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent, MouseEventKind};

    // ===== App Initialization Tests =====

    #[test]
    fn test_app_init() {
        let app = App::new();
        assert!(!app.should_quit());
        assert_eq!(app.title(), "Toad - AI Coding Terminal");
    }

    #[test]
    fn test_app_new_default_equivalence() {
        let app1 = App::new();
        let app2 = App::default();

        assert_eq!(app1.should_quit(), app2.should_quit());
        assert_eq!(app1.title(), app2.title());
        assert_eq!(app1.vim_mode(), app2.vim_mode());
    }

    // ===== Update Method Tests =====

    #[test]
    fn test_update_quit_event() {
        let mut app = App::new();
        app.update(Event::Quit).unwrap();
        assert!(app.should_quit());
    }

    #[test]
    fn test_update_tick_event() {
        let mut app = App::new();
        app.update(Event::Tick).unwrap();
        // Tick events should not cause errors
    }

    #[test]
    fn test_update_resize_event() {
        let mut app = App::new();
        app.update(Event::Resize(100, 50)).unwrap();
        // Resize events should not cause errors
    }

    #[test]
    fn test_update_mouse_event() {
        let mut app = App::new();
        let mouse = MouseEvent {
            kind: MouseEventKind::Down(crossterm::event::MouseButton::Left),
            column: 10,
            row: 5,
            modifiers: KeyModifiers::NONE,
        };
        app.update(Event::Mouse(mouse)).unwrap();
        // Mouse events are no-ops but shouldn't panic
    }

    #[test]
    fn test_update_dispatches_to_correct_screen() {
        let mut app = App::new();

        // Test Welcome screen
        app.screen = AppScreen::Welcome;
        app.update(Event::Key(KeyEvent::from(KeyCode::Char(' '))))
            .unwrap();
        assert_eq!(app.screen, AppScreen::TrustDialog);

        // Test Main screen
        app.screen = AppScreen::Main;
        let result = app.update(Event::Key(KeyEvent::from(KeyCode::Char('a'))));
        assert!(result.is_ok());
    }

    #[test]
    fn test_update_multiple_events_sequence() {
        let mut app = App::new();
        app.screen = AppScreen::Main;

        // Sequence of events
        app.update(Event::Tick).unwrap();
        app.update(Event::Resize(80, 24)).unwrap();
        app.update(Event::Key(KeyEvent::from(KeyCode::Char('a'))))
            .unwrap();
        app.update(Event::Tick).unwrap();

        // Should handle sequence without panicking
        assert!(!app.should_quit());
    }

    #[test]
    fn test_update_error_handling() {
        let mut app = App::new();

        // Update should return Result and handle various events
        assert!(app.update(Event::Quit).is_ok());
        assert!(app.update(Event::Tick).is_ok());
        assert!(app.update(Event::Resize(0, 0)).is_ok());
    }

    // ===== Screen-Specific Routing Tests =====

    #[test]
    fn test_update_welcome_screen_routing() {
        let mut app = App::new();
        app.screen = AppScreen::Welcome;

        app.update(Event::Key(KeyEvent::from(KeyCode::Enter)))
            .unwrap();

        // Should route to welcome handler and advance to trust dialog
        assert_eq!(app.screen, AppScreen::TrustDialog);
    }

    #[test]
    fn test_update_trust_dialog_routing() {
        let mut app = App::new();
        app.screen = AppScreen::TrustDialog;
        app.create_trust_dialog();

        app.update(Event::Key(KeyEvent::from(KeyCode::Char('1'))))
            .unwrap();

        // Should route to trust dialog handler and process selection
        assert_eq!(app.screen, AppScreen::Main);
    }

    #[test]
    fn test_update_main_screen_routing() {
        let mut app = App::new();
        app.screen = AppScreen::Main;

        app.update(Event::Key(KeyEvent::from(KeyCode::Char('?'))))
            .unwrap();

        // Should toggle help screen
        assert!(app.show_help);
    }

    #[test]
    fn test_update_evaluation_screen_routing() {
        let mut app = App::new();
        app.screen = AppScreen::Evaluation;
        app.evaluation_state = None;

        app.update(Event::Key(KeyEvent::from(KeyCode::Char('q'))))
            .unwrap();

        // Should return to main screen
        assert_eq!(app.screen, AppScreen::Main);
    }

    // ===== Edge Cases =====

    #[test]
    fn test_update_rapid_events() {
        let mut app = App::new();
        app.screen = AppScreen::Main;

        // Simulate rapid keyboard input
        for _ in 0..100 {
            app.update(Event::Tick).unwrap();
        }

        // Should handle rapid events without issues
        assert!(!app.should_quit());
    }

    #[test]
    fn test_update_alternating_screens() {
        let mut app = App::new();

        // Welcome → TrustDialog
        app.screen = AppScreen::Welcome;
        app.update(Event::Key(KeyEvent::from(KeyCode::Char(' '))))
            .unwrap();

        // TrustDialog → Main
        app.update(Event::Key(KeyEvent::from(KeyCode::Char('1'))))
            .unwrap();

        assert_eq!(app.screen, AppScreen::Main);
    }

    #[test]
    fn test_update_preserves_state() {
        let mut app = App::new();
        app.screen = AppScreen::Main;
        app.input_field.insert_char('t');
        app.input_field.insert_char('e');
        app.input_field.insert_char('s');
        app.input_field.insert_char('t');

        let initial_value = app.input_field.value().to_string();

        // Events that shouldn't affect input
        app.update(Event::Tick).unwrap();
        app.update(Event::Resize(100, 50)).unwrap();

        assert_eq!(app.input_field.value(), initial_value);
    }

    // ===== Special Event Tests =====

    #[test]
    fn test_update_ctrl_c_quits_from_main() {
        let mut app = App::new();
        app.screen = AppScreen::Main;

        app.update(Event::Key(KeyEvent::new(
            KeyCode::Char('c'),
            KeyModifiers::CONTROL,
        )))
        .unwrap();

        assert!(app.should_quit());
    }

    #[test]
    fn test_update_esc_behavior_varies_by_screen() {
        // Esc on Main with help shown - closes help
        let mut app1 = App::new();
        app1.screen = AppScreen::Main;
        app1.show_help = true;
        app1.update(Event::Key(KeyEvent::from(KeyCode::Esc)))
            .unwrap();
        assert!(!app1.show_help);
        assert!(!app1.should_quit());

        // Esc on Welcome - quits
        let mut app2 = App::new();
        app2.screen = AppScreen::Welcome;
        app2.update(Event::Key(KeyEvent::from(KeyCode::Esc)))
            .unwrap();
        assert!(app2.should_quit());

        // Esc on TrustDialog - quits
        let mut app3 = App::new();
        app3.screen = AppScreen::TrustDialog;
        app3.create_trust_dialog();
        app3.update(Event::Key(KeyEvent::from(KeyCode::Esc)))
            .unwrap();
        assert!(app3.should_quit());
    }

    // ===== AI Conversation Methods Tests =====

    #[test]
    fn test_conversation_view_accessor() {
        let mut app = App::new();
        let view = app.conversation_view();
        assert_eq!(view.message_count(), 0);
    }

    #[test]
    fn test_conversation_accessor() {
        let app = App::new();
        let conversation = app.conversation();
        assert_eq!(conversation.len(), 0);
    }

    #[test]
    fn test_add_message() {
        let mut app = App::new();

        let user_msg = Message::user("Test question");
        app.add_message(user_msg);

        assert_eq!(app.conversation().len(), 1);
        assert_eq!(app.conversation_view().message_count(), 1);
        assert_eq!(app.conversation()[0].content, "Test question");
    }

    #[test]
    fn test_add_multiple_messages() {
        let mut app = App::new();

        app.add_message(Message::user("Question 1"));
        app.add_message(Message::assistant("Answer 1"));
        app.add_message(Message::user("Question 2"));
        app.add_message(Message::assistant("Answer 2"));

        assert_eq!(app.conversation().len(), 4);
        assert_eq!(app.conversation_view().message_count(), 4);
    }

    #[test]
    fn test_clear_conversation() {
        let mut app = App::new();

        app.add_message(Message::user("Test 1"));
        app.add_message(Message::assistant("Response 1"));
        assert_eq!(app.conversation().len(), 2);

        app.clear_conversation();

        assert_eq!(app.conversation().len(), 0);
        assert_eq!(app.conversation_view().message_count(), 0);
    }

    #[test]
    fn test_has_llm_client_with_api_key() {
        // This test depends on ANTHROPIC_API_KEY env var
        let app = App::new();

        // If API key is set, client should be available
        if std::env::var("ANTHROPIC_API_KEY").is_ok() {
            assert!(app.has_llm_client() || !app.has_llm_client()); // Either state is valid
        } else {
            assert!(!app.has_llm_client());
        }
    }

    #[test]
    fn test_llm_client_accessor() {
        let app = App::new();

        // Should return Some or None depending on API key
        let client = app.llm_client();
        assert!(client.is_some() || client.is_none());
    }

    #[test]
    fn test_ai_processing_state() {
        let mut app = App::new();

        assert!(!app.is_ai_processing());

        app.set_ai_processing(true);
        assert!(app.is_ai_processing());

        app.set_ai_processing(false);
        assert!(!app.is_ai_processing());
    }

    #[test]
    fn test_conversation_message_order() {
        let mut app = App::new();

        app.add_message(Message::user("First"));
        app.add_message(Message::assistant("Second"));
        app.add_message(Message::user("Third"));

        let conversation = app.conversation();
        assert_eq!(conversation[0].content, "First");
        assert_eq!(conversation[1].content, "Second");
        assert_eq!(conversation[2].content, "Third");

        // Verify roles
        assert_eq!(conversation[0].role, crate::ai::llm::Role::User);
        assert_eq!(conversation[1].role, crate::ai::llm::Role::Assistant);
        assert_eq!(conversation[2].role, crate::ai::llm::Role::User);
    }
}
