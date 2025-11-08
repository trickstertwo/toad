//! Application state module (Model in Elm Architecture)
//!
//! This module contains the application state and the update logic
//! that handles state transitions based on events.

use crate::event::Event;
use crate::layout::LayoutManager;
use crate::widgets::{CommandPalette, ConfirmDialog, HelpScreen, InputField};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::env;
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

    /// Layout manager for split panes
    layout: LayoutManager,

    /// Vim mode enabled
    vim_mode: bool,
}

impl Default for App {
    fn default() -> Self {
        let working_directory = env::current_dir().unwrap_or_else(|_| PathBuf::from("/"));
        let mut input_field = InputField::new();
        input_field.set_focused(true);

        Self {
            screen: AppScreen::Welcome,
            should_quit: false,
            status_message: "Press any key to continue...".to_string(),
            title: "Toad - AI Coding Terminal".to_string(),
            working_directory,
            trust_dialog: None,
            welcome_shown: false,
            input_field,
            plugin_count: 0,
            help_screen: HelpScreen::new(),
            show_help: false,
            command_palette: CommandPalette::new(),
            show_palette: false,
            layout: LayoutManager::new(),
            vim_mode: false,
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
        self.status_message = format!(
            "Vim mode {}",
            if self.vim_mode { "enabled" } else { "disabled" }
        );
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
            Event::Tick => Ok(()),
            _ => Ok(()),
        }
    }

    /// Handle keyboard events based on current screen
    fn handle_key_event(&mut self, key: KeyEvent) -> crate::Result<()> {
        match &self.screen {
            AppScreen::Welcome => self.handle_welcome_key(key),
            AppScreen::TrustDialog => self.handle_trust_dialog_key(key),
            AppScreen::Main => self.handle_main_key(key),
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
                self.status_message =
                    "Confirm folder trust to continue".to_string();
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
                if let Some(dialog) = &mut self.trust_dialog {
                    if dialog.select_by_key(c).is_some() {
                        self.confirm_trust_selection();
                    }
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
                (KeyCode::Char(c), KeyModifiers::NONE) | (KeyCode::Char(c), KeyModifiers::SHIFT) => {
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
            // Tab for next panel
            (KeyCode::Tab, KeyModifiers::NONE) => {
                self.layout.focus_next();
                self.status_message = format!("Focused panel {}", self.layout.focused());
            }
            // Shift+Tab for previous panel
            (KeyCode::Tab, KeyModifiers::SHIFT) | (KeyCode::BackTab, _) => {
                self.layout.focus_previous();
                self.status_message = format!("Focused panel {}", self.layout.focused());
            }
            // Vim-style navigation (when not in input field and vim mode enabled)
            (KeyCode::Char('h'), KeyModifiers::NONE) if self.vim_mode && !self.input_field.is_focused() => {
                self.status_message = "Vim: move left".to_string();
                // TODO: Implement vim-style left navigation
            }
            (KeyCode::Char('j'), KeyModifiers::NONE) if self.vim_mode && !self.input_field.is_focused() => {
                self.status_message = "Vim: move down".to_string();
                // TODO: Implement vim-style down navigation
            }
            (KeyCode::Char('k'), KeyModifiers::NONE) if self.vim_mode && !self.input_field.is_focused() => {
                self.status_message = "Vim: move up".to_string();
                // TODO: Implement vim-style up navigation
            }
            (KeyCode::Char('l'), KeyModifiers::NONE) if self.vim_mode && !self.input_field.is_focused() => {
                self.status_message = "Vim: move right".to_string();
                // TODO: Implement vim-style right navigation
            }
            // g for jump to top (Vim-style)
            (KeyCode::Char('g'), KeyModifiers::NONE) if self.vim_mode && !self.input_field.is_focused() => {
                self.status_message = "Vim: jump to top".to_string();
                // TODO: Implement jump to top
            }
            // G for jump to bottom (Vim-style)
            (KeyCode::Char('G'), KeyModifiers::SHIFT) if self.vim_mode && !self.input_field.is_focused() => {
                self.status_message = "Vim: jump to bottom".to_string();
                // TODO: Implement jump to bottom
            }
            // Forward slash for search
            (KeyCode::Char('/'), KeyModifiers::NONE) if !self.input_field.is_focused() => {
                self.status_message = "Search mode (coming soon)".to_string();
                // TODO: Implement search mode
            }
            // n for next search result
            (KeyCode::Char('n'), KeyModifiers::NONE) if self.vim_mode && !self.input_field.is_focused() => {
                self.status_message = "Next search result (coming soon)".to_string();
                // TODO: Implement next search
            }
            // N for previous search result
            (KeyCode::Char('N'), KeyModifiers::SHIFT) if self.vim_mode && !self.input_field.is_focused() => {
                self.status_message = "Previous search result (coming soon)".to_string();
                // TODO: Implement previous search
            }
            // Regular character input
            (KeyCode::Char(c), KeyModifiers::NONE) | (KeyCode::Char(c), KeyModifiers::SHIFT) => {
                self.input_field.insert_char(c);
            }
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
                    self.status_message = "Available commands: /help, /commands, /clear".to_string();
                }
                "clear" => {
                    self.status_message = "Screen cleared".to_string();
                }
                _ => {
                    self.status_message = format!("Unknown command: /{}", command);
                }
            }
        } else {
            // Regular query/request
            self.status_message = format!("Processing: {}", input);
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
        let dir_path = self
            .working_directory
            .to_string_lossy()
            .to_string();

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
                    // Yes and remember - TODO: Save to config
                    self.screen = AppScreen::Main;
                    self.trust_dialog = None;
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

    #[test]
    fn test_quit_on_esc_from_welcome() {
        let mut app = App::new();
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
}
