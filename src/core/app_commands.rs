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
                        // Regular query/request
                        self.status_message = format!("Processing: {}", input);
                        self.toast_info("AI query processing coming soon");
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
