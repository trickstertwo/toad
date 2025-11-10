//! Command processing and execution
//!
//! Handles parsing and executing user commands from input field and command palette.

use crate::core::app::App;

impl App {
    /// Process commands entered by the user
    ///
    /// Handles two types of commands:
    /// 1. **Slash commands** (e.g., `/help`, `/clear`): Simple UI commands
    /// 2. **Evaluation commands** (e.g., `eval`, `compare`): AI evaluation tasks
    ///
    /// # Slash Commands
    ///
    /// - `/help`: Show help screen
    /// - `/commands`: List available commands
    /// - `/clear`: Clear screen
    ///
    /// # Evaluation Commands
    ///
    /// - `eval --count N --milestone M`: Run evaluation
    /// - `compare --count N --baseline M1 --test M2`: A/B comparison
    /// - `show-config --milestone M`: Show milestone configuration
    ///
    /// # Regular Input
    ///
    /// Non-command input is treated as an AI query (coming soon).
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use toad::core::app::App;
    /// let mut app = App::new();
    /// app.process_command("/help");
    /// app.process_command("eval --count 10 --milestone 1");
    /// ```
    ///
    /// # Errors
    ///
    /// Shows error toast for invalid evaluation command syntax.
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
                        // Regular AI query/request
                        self.process_ai_query(input.to_string());
                    }
                }
            }
        }
    }

    /// Execute a command from the command palette
    ///
    /// The command palette provides quick access to common TUI commands via fuzzy search.
    ///
    /// # Supported Commands
    ///
    /// - `help`: Show help screen
    /// - `clear`: Clear screen
    /// - `quit`: Quit application
    /// - `vim_mode`: Toggle Vim mode
    /// - `theme_toggle`: Toggle theme (coming soon)
    /// - `split_horizontal`: Split pane horizontally (coming soon)
    /// - `split_vertical`: Split pane vertically (coming soon)
    /// - `open_file`: Open file picker (coming soon)
    /// - `search_files`: Search files (coming soon)
    /// - `git_status`: Show git status (coming soon)
    /// - `recent_files`: Show recent files (coming soon)
    ///
    /// # Parameters
    ///
    /// - `cmd_id`: Command identifier from the palette's command list
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use toad::core::app::App;
    /// let mut app = App::new();
    /// app.execute_palette_command("help");
    /// app.execute_palette_command("vim_mode");
    /// ```
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
}

#[cfg(test)]
mod tests {
    use crate::core::app::App;
    use crate::core::app_state::AppScreen;

    // ===== Process Command Tests =====

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

    // ===== Execute Palette Command Tests =====

    #[test]
    fn test_execute_palette_command_vim_mode() {
        let mut app = App::new();
        let initial = app.vim_mode();
        app.execute_palette_command("vim_mode");
        assert_ne!(
            app.vim_mode(),
            initial,
            "Palette command should toggle vim mode"
        );
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

    // ===== Enter Key Command Processing =====

    #[test]
    fn test_enter_processes_command() {
        use crate::core::event::Event;
        use crossterm::event::{KeyCode, KeyEvent};

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
        use crate::core::event::Event;
        use crossterm::event::{KeyCode, KeyEvent};

        let mut app = App::new();
        app.screen = AppScreen::Main;

        let event = Event::Key(KeyEvent::from(KeyCode::Enter));
        app.update(event).unwrap();

        // Should not process empty input
        assert_eq!(app.input_field.value(), "");
    }

    #[test]
    fn test_multiple_command_submissions() {
        use crate::core::event::Event;
        use crossterm::event::{KeyCode, KeyEvent};

        let mut app = App::new();
        app.screen = AppScreen::Main;

        // First command
        app.input_field.set_value("/commands".to_string());
        let event = Event::Key(KeyEvent::from(KeyCode::Enter));
        app.update(event).unwrap();

        let first_msg = app.status_message.clone();

        // Second command
        app.input_field.set_value("/clear".to_string());
        let event = Event::Key(KeyEvent::from(KeyCode::Enter));
        app.update(event).unwrap();

        let second_msg = app.status_message.clone();

        assert_ne!(first_msg, second_msg);
    }
}
