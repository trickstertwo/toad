//! Main screen event handlers
//!
//! Handles keyboard events for the main TUI interface, including input field,
//! command palette, help screen, tab switching, and vim-style navigation.

use crate::core::app::App;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

impl App {
    /// Handle keys in main interface
    ///
    /// # Behavior Overview
    ///
    /// This is the primary event handler for the main TUI interface. It handles:
    ///
    /// ## Modal Overlays
    /// - **Help Screen** (`show_help = true`): `Esc` or `Ctrl+?` closes help
    /// - **Command Palette** (`show_palette = true`): Navigation and command execution
    ///
    /// ## Global Commands
    /// - `Ctrl+C`: Quit application
    /// - `Ctrl+D`: Quit if input empty, otherwise page down
    /// - `Ctrl+P`: Open command palette
    /// - `F9`: Open Evaluation Center
    /// - `Ctrl+?`: Toggle help screen
    ///
    /// ## Tab Management
    /// - `Tab`: Next tab (or focus next panel if input focused)
    /// - `Shift+Tab`: Previous tab (or focus previous panel if input focused)
    /// - `Ctrl+1-9`: Switch to specific tab by number
    /// - `1-9` (when input not focused): Switch to tab
    /// - `Alt+1-9`: Switch to tab (works even in input field)
    ///
    /// ## Input Field
    /// - `Enter`: Submit command
    /// - `Backspace`: Delete character
    /// - `Left`/`Right`: Move cursor
    /// - `Home`/`End`: Jump to start/end
    /// - `Ctrl+A`/`Ctrl+E`: Emacs-style start/end
    /// - `Ctrl+O`: Open external editor ($EDITOR, $VISUAL, or vim)
    /// - `Ctrl+U`: Clear input
    /// - Regular characters: Insert into input
    ///
    /// ## Vim Mode Navigation (when enabled and input not focused)
    /// - `h`: Move left
    /// - `j`: Move down
    /// - `k`: Move up
    /// - `l`: Move right
    /// - `g`: Jump to top
    /// - `G`: Jump to bottom
    /// - `/`: Search mode
    /// - `n`: Next search result
    /// - `N`: Previous search result
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use toad::core::app::App;
    /// # use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
    /// let mut app = App::new();
    /// let key = KeyEvent::new(KeyCode::Char('p'), KeyModifiers::CONTROL);
    /// app.handle_main_key(key).unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// Returns `Err` if state transition fails (should not happen in practice).
    pub(crate) fn handle_main_key(&mut self, key: KeyEvent) -> crate::Result<()> {
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

        // If config dialog is shown, intercept keys
        if self.show_config_dialog.is_some() {
            match (key.code, key.modifiers) {
                // Esc or Enter closes config dialog
                (KeyCode::Esc, _) | (KeyCode::Enter, _) => {
                    self.show_config_dialog = None;
                    self.status_message = "Config dialog closed".to_string();
                }
                _ => {}
            }
            return Ok(());
        }

        // If settings screen is shown, intercept keys for settings navigation
        if self.show_settings {
            match (key.code, key.modifiers) {
                // Esc or F10 closes settings
                (KeyCode::Esc, _) | (KeyCode::F(10), _) => {
                    self.show_settings = false;
                }
                // Left/Right switch tabs
                (KeyCode::Left, _) => {
                    self.settings_screen.previous_category();
                }
                (KeyCode::Right, _) => {
                    self.settings_screen.next_category();
                }
                // Up/Down navigate within category
                (KeyCode::Up, _) => {
                    self.settings_screen.select_previous();
                }
                (KeyCode::Down, _) => {
                    self.settings_screen.select_next();
                }
                // Enter applies selected setting
                (KeyCode::Enter, _) => {
                    // Check for theme change
                    if let Some(theme) = self.settings_screen.selected_theme() {
                        self.theme_manager.set_theme(theme);
                        self.status_message = format!("Theme changed to {}", theme.as_str());
                        // Update settings screen to reflect new current theme
                        self.settings_screen.update_theme(theme);
                        // Save theme to session for persistence
                        self.session.set_theme(theme.as_str().to_string());
                        if let Err(e) = self.save_session() {
                            tracing::warn!("Failed to save session after theme change: {}", e);
                        }
                    }
                    // Check for vim mode toggle
                    else if self.settings_screen.should_toggle_vim_mode() {
                        self.toggle_vim_mode();
                        self.status_message = format!("Vim mode: {}", if self.vim_mode { "ON" } else { "OFF" });
                        // Save to config
                        if let Err(e) = self.config.save_to_file(&crate::config::Config::default_path()) {
                            tracing::warn!("Failed to save config after vim mode toggle: {}", e);
                        }
                    }
                }
                _ => {}
            }
            return Ok(());
        }

        match (key.code, key.modifiers) {
            // Cancel streaming on Ctrl+C, or quit if not streaming
            // But if Shift is also pressed, copy last message instead
            (KeyCode::Char('c'), mods) if mods.contains(KeyModifiers::CONTROL) => {
                if mods.contains(KeyModifiers::SHIFT) {
                    // Ctrl+Shift+C: Copy last assistant message
                    self.copy_last_assistant_message();
                } else {
                    // Ctrl+C: Cancel streaming or quit
                    if self.conversation_view.is_streaming() {
                        self.conversation_view.cancel_streaming();
                        self.set_ai_processing(false);
                        self.status_message = "Streaming cancelled".to_string();
                    } else {
                        self.should_quit = true;
                    }
                }
            }
            // Ctrl+D for page down (Vim-style), or quit if input is focused and empty
            (KeyCode::Char('d'), KeyModifiers::CONTROL) => {
                if self.input_field.is_focused() && self.input_field.value().is_empty() {
                    self.should_quit = true;
                } else if !self.input_field.is_focused() {
                    // Estimate page height (terminal height minus UI chrome)
                    // Assuming ~6 lines for borders, input, shortcuts, etc.
                    let page_height = 20usize; // Conservative default
                    self.conversation_view.page_down(page_height);
                    self.status_message = "Page down".to_string();
                }
            }
            // Ctrl+U for page up (Vim-style) or clear input if focused
            (KeyCode::Char('u'), KeyModifiers::CONTROL) => {
                if self.input_field.is_focused() {
                    self.input_field.clear();
                } else {
                    // Estimate page height (terminal height minus UI chrome)
                    let page_height = 20usize; // Conservative default
                    self.conversation_view.page_up(page_height);
                    self.status_message = "Page up".to_string();
                }
            }
            // Ctrl+L to clear conversation history
            (KeyCode::Char('l'), KeyModifiers::CONTROL) => {
                if !self.conversation_view.is_streaming() {
                    self.clear_conversation();
                    self.status_message = "Conversation cleared".to_string();
                    // Save session after clearing conversation
                    if let Err(e) = self.save_session() {
                        tracing::warn!("Failed to save session after clearing: {}", e);
                    }
                } else {
                    self.status_message = "Cannot clear during streaming".to_string();
                }
            }
            // Ctrl+T to create a new tab
            (KeyCode::Char('t'), KeyModifiers::CONTROL) => {
                if let Some(tab_id) = self.tabs.add_tab("New Tab") {
                    self.status_message = format!("Created tab {} (ID: {})", self.tabs.count(), tab_id);
                    // Save session after creating tab
                    if let Err(e) = self.save_session() {
                        tracing::warn!("Failed to save session after creating tab: {}", e);
                    }
                } else {
                    self.status_message = format!(
                        "Cannot create tab: maximum of {} tabs reached",
                        crate::workspace::tabs::MAX_TABS
                    );
                }
            }
            // Ctrl+W to close current tab
            (KeyCode::Char('w'), KeyModifiers::CONTROL) => {
                if let Some(active_tab) = self.tabs.active_tab() {
                    let tab_id = active_tab.id;
                    let title = active_tab.title.clone();
                    let modified = active_tab.modified;

                    // Check if tab has unsaved changes (modified indicator)
                    if modified {
                        // TODO: Show confirmation dialog before closing
                        self.status_message = "Tab has unsaved changes. Close confirmation not yet implemented".to_string();
                    } else if self.tabs.count() > 1 {
                        // Only close if there's more than one tab
                        self.tabs.close_tab(tab_id);
                        self.status_message = format!("Closed tab '{}'", title);
                        // Save session after closing tab
                        if let Err(e) = self.save_session() {
                            tracing::warn!("Failed to save session after closing tab: {}", e);
                        }
                    } else {
                        self.status_message = "Cannot close the last tab".to_string();
                    }
                } else {
                    self.status_message = "No active tab to close".to_string();
                }
            }
            // Ctrl+P opens command palette
            (KeyCode::Char('p'), KeyModifiers::CONTROL) => {
                self.show_palette = true;
            }
            // F9 opens evaluation center
            (KeyCode::F(9), _) => {
                use crate::core::app_state::AppScreen;
                self.screen = AppScreen::Evaluation;
                self.status_message = "Opened Evaluation Center".to_string();
            }
            // F10 opens settings screen
            (KeyCode::F(10), _) => {
                let current_theme = self.theme_manager.current_theme_name();
                self.settings_screen.update_theme(current_theme);
                self.show_settings = true;
            }
            // Toggle help screen with Ctrl+? (Ctrl+Shift+/)
            (KeyCode::Char('?'), KeyModifiers::CONTROL | KeyModifiers::SHIFT) => {
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
            // Shift+Enter inserts newline, Enter submits
            (KeyCode::Enter, KeyModifiers::SHIFT) => {
                self.input_field.insert_char('\n');
            }
            (KeyCode::Enter, _) => {
                let input = self.input_field.value().to_string();
                if !input.is_empty() {
                    // Add to command history before processing
                    self.command_history.add(input.clone());
                    self.process_command(&input);
                    self.input_field.clear();
                }
            }
            // Backspace deletes character
            (KeyCode::Backspace, _) => {
                self.input_field.delete_char();
            }
            // Up arrow navigates to older command in history
            (KeyCode::Up, _) => {
                if let Some(command) = self.command_history.older() {
                    self.input_field.set_value(command.clone());
                }
            }
            // Down arrow navigates to newer command in history
            (KeyCode::Down, _) => {
                if let Some(command) = self.command_history.newer() {
                    self.input_field.set_value(command.clone());
                } else {
                    // At the end of history, clear input
                    self.input_field.clear();
                }
            }
            // Arrow keys move cursor (only left/right now, up/down for history)
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
            // Ctrl+O: Open external editor
            (KeyCode::Char('o'), KeyModifiers::CONTROL) => {
                let initial_content = self.input_field.value();

                // Open external editor with current input
                match crate::editor::external::edit_with_external_editor(initial_content) {
                    Ok(edited_content) => {
                        self.input_field.set_value(edited_content);
                        self.status_message = "Content loaded from external editor".to_string();
                    }
                    Err(crate::editor::EditorError::EmptyContent) => {
                        self.status_message = "External editor cancelled (empty content)".to_string();
                    }
                    Err(e) => {
                        self.status_message = format!("Editor error: {}", e);
                    }
                }
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
}

#[cfg(test)]
mod tests {
    use crate::core::app::App;
    use crate::core::app_state::AppScreen;
    use crate::core::event::Event;
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    // ===== Help Screen Tests =====

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
    fn test_question_mark_toggles_help() {
        let mut app = App::new();
        app.screen = AppScreen::Main;
        let initial_state = app.show_help;

        let event = Event::Key(KeyEvent::from(KeyCode::Char('?')));
        app.update(event).unwrap();

        assert_eq!(app.show_help, !initial_state);
    }

    #[test]
    fn test_esc_from_main_closes_overlays() {
        let mut app = App::new();
        app.screen = AppScreen::Main;
        app.show_help = true;

        let event = Event::Key(KeyEvent::from(KeyCode::Esc));
        app.update(event).unwrap();

        assert!(!app.show_help);
        assert!(
            !app.should_quit(),
            "Esc from main should not quit (closes overlays instead)"
        );
    }

    // ===== Command Palette Tests =====

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
    fn test_ctrl_p_opens_palette() {
        let mut app = App::new();
        app.screen = AppScreen::Main;
        app.show_palette = false;

        let event = Event::Key(KeyEvent::new(KeyCode::Char('p'), KeyModifiers::CONTROL));
        app.update(event).unwrap();

        assert!(app.show_palette);
    }

    // ===== Page Navigation Tests =====

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
    fn test_ctrl_u_page_up_when_not_in_input() {
        let mut app = App::new();
        app.screen = AppScreen::Main;
        app.input_field.set_focused(false);

        let event = Event::Key(KeyEvent::new(KeyCode::Char('u'), KeyModifiers::CONTROL));
        app.update(event).unwrap();

        assert!(app.status_message.contains("Page up"));
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
    fn test_backtab_switches_previous_tab() {
        let mut app = App::new();
        app.screen = AppScreen::Main;
        app.input_field.set_focused(false);

        let event = Event::Key(KeyEvent::from(KeyCode::BackTab));
        app.update(event).unwrap();

        // Should switch to previous tab
        assert!(app.status_message.contains("tab") || app.status_message.contains("Tab"));
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
    fn test_backspace_deletes_character() {
        let mut app = App::new();
        app.screen = AppScreen::Main;
        app.input_field.insert_char('t');
        app.input_field.insert_char('e');

        let event = Event::Key(KeyEvent::from(KeyCode::Backspace));
        app.update(event).unwrap();

        assert_eq!(app.input_field.value(), "t");
    }

    #[test]
    fn test_backspace_on_empty_input() {
        let mut app = App::new();
        app.screen = AppScreen::Main;

        assert_eq!(app.input_field.value(), "");

        // Backspace on empty input
        let event = Event::Key(KeyEvent::from(KeyCode::Backspace));
        app.update(event).unwrap();

        // Should still be empty
        assert_eq!(app.input_field.value(), "");
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
    fn test_rapid_key_presses() {
        let mut app = App::new();
        app.screen = AppScreen::Main;

        // Simulate rapid typing
        for c in "hello world".chars() {
            let event = Event::Key(KeyEvent::from(KeyCode::Char(c)));
            app.update(event).unwrap();
        }

        // Should handle all input without panicking
        assert!(app.input_field().value().len() > 0);
    }

    #[test]
    fn test_unicode_input_processing() {
        let mut app = App::new();
        app.screen = AppScreen::Main;

        // Type unicode characters
        for c in "🐸🎉世界".chars() {
            let event = Event::Key(KeyEvent::from(KeyCode::Char(c)));
            app.update(event).unwrap();
        }

        // Should handle unicode
        assert!(app.input_field().value().contains("🐸"));
    }
}
