/// Command mode for ex-style commands
///
/// Vim-inspired command mode with : prefix
///
/// # Examples
///
/// ```
/// use toad::command_mode::{CommandMode, Command};
///
/// let mut mode = CommandMode::new();
/// mode.start();
/// assert!(mode.is_active());
/// mode.input_char('q');
/// assert_eq!(mode.buffer(), "q");
/// ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

/// Command handler result
pub type CommandResult = Result<String, String>;

/// Command handler function type
pub type CommandHandler = Arc<dyn Fn(&[String]) -> CommandResult + Send + Sync>;

/// A single command definition
#[derive(Clone)]
pub struct Command {
    /// Command name
    pub name: String,
    /// Command aliases
    pub aliases: Vec<String>,
    /// Command description
    pub description: String,
    /// Whether command takes arguments
    pub takes_args: bool,
}

impl Command {
    /// Create a new command
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            aliases: Vec::new(),
            description: description.into(),
            takes_args: false,
        }
    }

    /// Add an alias
    pub fn alias(mut self, alias: impl Into<String>) -> Self {
        self.aliases.push(alias.into());
        self
    }

    /// Set whether the command takes arguments
    pub fn takes_args(mut self, takes_args: bool) -> Self {
        self.takes_args = takes_args;
        self
    }

    /// Check if this command matches a name (including aliases)
    pub fn matches(&self, name: &str) -> bool {
        self.name == name || self.aliases.iter().any(|a| a == name)
    }
}

impl fmt::Debug for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Command")
            .field("name", &self.name)
            .field("aliases", &self.aliases)
            .field("description", &self.description)
            .field("takes_args", &self.takes_args)
            .finish()
    }
}

/// Command mode state
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandMode {
    /// Whether command mode is active
    active: bool,
    /// Command input buffer
    buffer: String,
    /// Cursor position in buffer
    cursor: usize,
    /// Command history
    history: Vec<String>,
    /// Current position in history
    history_index: Option<usize>,
}

impl CommandMode {
    /// Create a new command mode
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::command_mode::CommandMode;
    ///
    /// let mode = CommandMode::new();
    /// assert!(!mode.is_active());
    /// ```
    pub fn new() -> Self {
        Self {
            active: false,
            buffer: String::new(),
            cursor: 0,
            history: Vec::new(),
            history_index: None,
        }
    }

    /// Start command mode
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::command_mode::CommandMode;
    ///
    /// let mut mode = CommandMode::new();
    /// mode.start();
    /// assert!(mode.is_active());
    /// assert_eq!(mode.buffer(), "");
    /// ```
    pub fn start(&mut self) {
        self.active = true;
        self.buffer.clear();
        self.cursor = 0;
        self.history_index = None;
    }

    /// Cancel command mode
    pub fn cancel(&mut self) {
        self.active = false;
        self.buffer.clear();
        self.cursor = 0;
        self.history_index = None;
    }

    /// Check if command mode is active
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Get the current buffer
    pub fn buffer(&self) -> &str {
        &self.buffer
    }

    /// Get cursor position
    pub fn cursor(&self) -> usize {
        self.cursor
    }

    /// Input a character
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::command_mode::CommandMode;
    ///
    /// let mut mode = CommandMode::new();
    /// mode.start();
    /// mode.input_char('q');
    /// mode.input_char('u');
    /// mode.input_char('i');
    /// mode.input_char('t');
    /// assert_eq!(mode.buffer(), "quit");
    /// ```
    pub fn input_char(&mut self, c: char) {
        if !self.active {
            return;
        }

        self.buffer.insert(self.cursor, c);
        self.cursor += 1;
        self.history_index = None;
    }

    /// Delete character before cursor (backspace)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::command_mode::CommandMode;
    ///
    /// let mut mode = CommandMode::new();
    /// mode.start();
    /// mode.input_char('a');
    /// mode.input_char('b');
    /// mode.backspace();
    /// assert_eq!(mode.buffer(), "a");
    /// ```
    pub fn backspace(&mut self) {
        if !self.active || self.cursor == 0 {
            return;
        }

        self.buffer.remove(self.cursor - 1);
        self.cursor -= 1;
        self.history_index = None;
    }

    /// Delete character at cursor (delete)
    pub fn delete(&mut self) {
        if !self.active || self.cursor >= self.buffer.len() {
            return;
        }

        self.buffer.remove(self.cursor);
        self.history_index = None;
    }

    /// Move cursor left
    pub fn cursor_left(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    /// Move cursor right
    pub fn cursor_right(&mut self) {
        if self.cursor < self.buffer.len() {
            self.cursor += 1;
        }
    }

    /// Move cursor to start
    pub fn cursor_home(&mut self) {
        self.cursor = 0;
    }

    /// Move cursor to end
    pub fn cursor_end(&mut self) {
        self.cursor = self.buffer.len();
    }

    /// Navigate history up
    pub fn history_up(&mut self) {
        if self.history.is_empty() {
            return;
        }

        let new_index = match self.history_index {
            None => Some(self.history.len() - 1),
            Some(idx) if idx > 0 => Some(idx - 1),
            Some(idx) => Some(idx),
        };

        if let Some(idx) = new_index {
            self.buffer = self.history[idx].clone();
            self.cursor = self.buffer.len();
            self.history_index = new_index;
        }
    }

    /// Navigate history down
    pub fn history_down(&mut self) {
        if self.history.is_empty() {
            return;
        }

        match self.history_index {
            Some(idx) if idx + 1 < self.history.len() => {
                let new_idx = idx + 1;
                self.buffer = self.history[new_idx].clone();
                self.cursor = self.buffer.len();
                self.history_index = Some(new_idx);
            }
            Some(_) => {
                // At the end, clear buffer
                self.buffer.clear();
                self.cursor = 0;
                self.history_index = None;
            }
            None => {}
        }
    }

    /// Execute the current command
    ///
    /// Returns the command and arguments, and resets the mode
    pub fn execute(&mut self) -> Option<(String, Vec<String>)> {
        if !self.active || self.buffer.is_empty() {
            self.cancel();
            return None;
        }

        let command = self.buffer.trim().to_string();

        // Add to history if not a duplicate of the last command
        if self.history.last() != Some(&command) {
            self.history.push(command.clone());
        }

        // Parse command and arguments
        let parts: Vec<String> = command.split_whitespace().map(String::from).collect();
        let cmd = parts.first()?.clone();
        let args = parts.into_iter().skip(1).collect();

        self.cancel();
        Some((cmd, args))
    }

    /// Get command history
    pub fn history(&self) -> &[String] {
        &self.history
    }

    /// Clear history
    pub fn clear_history(&mut self) {
        self.history.clear();
        self.history_index = None;
    }
}

impl Default for CommandMode {
    fn default() -> Self {
        Self::new()
    }
}

/// Command registry for managing available commands
pub struct CommandRegistry {
    /// Registered commands
    commands: Vec<Command>,
    /// Command handlers
    handlers: HashMap<String, CommandHandler>,
}

impl CommandRegistry {
    /// Create a new command registry
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::command_mode::CommandRegistry;
    ///
    /// let registry = CommandRegistry::new();
    /// assert_eq!(registry.commands().len(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
            handlers: HashMap::new(),
        }
    }

    /// Register a command
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::command_mode::{CommandRegistry, Command};
    /// use std::sync::Arc;
    ///
    /// let mut registry = CommandRegistry::new();
    /// let cmd = Command::new("quit", "Exit the application").alias("q");
    /// registry.register(cmd, Arc::new(|_| Ok("Quitting".to_string())));
    /// assert_eq!(registry.commands().len(), 1);
    /// ```
    pub fn register(&mut self, command: Command, handler: CommandHandler) {
        // Register handler for primary name
        self.handlers.insert(command.name.clone(), Arc::clone(&handler));

        // Register handler for all aliases
        for alias in &command.aliases {
            self.handlers.insert(alias.clone(), Arc::clone(&handler));
        }

        self.commands.push(command);
    }

    /// Execute a command
    pub fn execute(&self, name: &str, args: &[String]) -> CommandResult {
        if let Some(handler) = self.handlers.get(name) {
            handler(args)
        } else {
            Err(format!("Unknown command: {}", name))
        }
    }

    /// Get all registered commands
    pub fn commands(&self) -> &[Command] {
        &self.commands
    }

    /// Find command by name or alias
    pub fn find(&self, name: &str) -> Option<&Command> {
        self.commands.iter().find(|cmd| cmd.matches(name))
    }

    /// Get command suggestions for partial input
    pub fn suggest(&self, partial: &str) -> Vec<&Command> {
        if partial.is_empty() {
            return Vec::new();
        }

        self.commands
            .iter()
            .filter(|cmd| {
                cmd.name.starts_with(partial) || cmd.aliases.iter().any(|a| a.starts_with(partial))
            })
            .collect()
    }
}

impl Default for CommandRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for CommandRegistry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CommandRegistry")
            .field("commands", &self.commands)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_creation() {
        let cmd = Command::new("quit", "Exit application");
        assert_eq!(cmd.name, "quit");
        assert_eq!(cmd.description, "Exit application");
        assert!(!cmd.takes_args);
    }

    #[test]
    fn test_command_with_alias() {
        let cmd = Command::new("quit", "Exit").alias("q").alias("exit");
        assert_eq!(cmd.aliases.len(), 2);
        assert!(cmd.matches("quit"));
        assert!(cmd.matches("q"));
        assert!(cmd.matches("exit"));
        assert!(!cmd.matches("x"));
    }

    #[test]
    fn test_command_with_args() {
        let cmd = Command::new("open", "Open file").takes_args(true);
        assert!(cmd.takes_args);
    }

    #[test]
    fn test_command_mode_creation() {
        let mode = CommandMode::new();
        assert!(!mode.is_active());
        assert_eq!(mode.buffer(), "");
        assert_eq!(mode.cursor(), 0);
    }

    #[test]
    fn test_command_mode_start() {
        let mut mode = CommandMode::new();
        mode.start();
        assert!(mode.is_active());
        assert_eq!(mode.buffer(), "");
    }

    #[test]
    fn test_command_mode_cancel() {
        let mut mode = CommandMode::new();
        mode.start();
        mode.input_char('q');
        mode.cancel();
        assert!(!mode.is_active());
        assert_eq!(mode.buffer(), "");
    }

    #[test]
    fn test_input_char() {
        let mut mode = CommandMode::new();
        mode.start();
        mode.input_char('q');
        mode.input_char('u');
        mode.input_char('i');
        mode.input_char('t');
        assert_eq!(mode.buffer(), "quit");
        assert_eq!(mode.cursor(), 4);
    }

    #[test]
    fn test_backspace() {
        let mut mode = CommandMode::new();
        mode.start();
        mode.input_char('a');
        mode.input_char('b');
        mode.input_char('c');
        mode.backspace();
        assert_eq!(mode.buffer(), "ab");
        assert_eq!(mode.cursor(), 2);
    }

    #[test]
    fn test_backspace_empty() {
        let mut mode = CommandMode::new();
        mode.start();
        mode.backspace();
        assert_eq!(mode.buffer(), "");
    }

    #[test]
    fn test_delete() {
        let mut mode = CommandMode::new();
        mode.start();
        mode.input_char('a');
        mode.input_char('b');
        mode.cursor_left();
        mode.delete();
        assert_eq!(mode.buffer(), "a");
    }

    #[test]
    fn test_cursor_movement() {
        let mut mode = CommandMode::new();
        mode.start();
        mode.input_char('a');
        mode.input_char('b');
        mode.input_char('c');

        mode.cursor_left();
        assert_eq!(mode.cursor(), 2);

        mode.cursor_right();
        assert_eq!(mode.cursor(), 3);

        mode.cursor_home();
        assert_eq!(mode.cursor(), 0);

        mode.cursor_end();
        assert_eq!(mode.cursor(), 3);
    }

    #[test]
    fn test_execute() {
        let mut mode = CommandMode::new();
        mode.start();
        mode.input_char('q');
        mode.input_char('u');
        mode.input_char('i');
        mode.input_char('t');

        let result = mode.execute();
        assert_eq!(result, Some(("quit".to_string(), vec![])));
        assert!(!mode.is_active());
    }

    #[test]
    fn test_execute_with_args() {
        let mut mode = CommandMode::new();
        mode.start();
        for c in "open file.txt".chars() {
            mode.input_char(c);
        }

        let result = mode.execute();
        assert_eq!(
            result,
            Some(("open".to_string(), vec!["file.txt".to_string()]))
        );
    }

    #[test]
    fn test_execute_empty() {
        let mut mode = CommandMode::new();
        mode.start();

        let result = mode.execute();
        assert_eq!(result, None);
    }

    #[test]
    fn test_history() {
        let mut mode = CommandMode::new();

        mode.start();
        mode.input_char('q');
        mode.execute();

        mode.start();
        for c in "help".chars() {
            mode.input_char(c);
        }
        mode.execute();

        assert_eq!(mode.history().len(), 2);
        assert_eq!(mode.history()[0], "q");
        assert_eq!(mode.history()[1], "help");
    }

    #[test]
    fn test_history_navigation() {
        let mut mode = CommandMode::new();

        mode.start();
        mode.input_char('q');
        mode.execute();

        mode.start();
        for c in "help".chars() {
            mode.input_char(c);
        }
        mode.execute();

        mode.start();
        mode.history_up();
        assert_eq!(mode.buffer(), "help");

        mode.history_up();
        assert_eq!(mode.buffer(), "q");

        mode.history_down();
        assert_eq!(mode.buffer(), "help");

        mode.history_down();
        assert_eq!(mode.buffer(), "");
    }

    #[test]
    fn test_clear_history() {
        let mut mode = CommandMode::new();

        mode.start();
        mode.input_char('q');
        mode.execute();

        assert_eq!(mode.history().len(), 1);

        mode.clear_history();
        assert_eq!(mode.history().len(), 0);
    }

    #[test]
    fn test_registry_creation() {
        let registry = CommandRegistry::new();
        assert_eq!(registry.commands().len(), 0);
    }

    #[test]
    fn test_registry_register() {
        let mut registry = CommandRegistry::new();
        let cmd = Command::new("quit", "Exit").alias("q");
        registry.register(cmd, Arc::new(|_| Ok("Quitting".to_string())));

        assert_eq!(registry.commands().len(), 1);
    }

    #[test]
    fn test_registry_execute() {
        let mut registry = CommandRegistry::new();
        let cmd = Command::new("echo", "Echo message").takes_args(true);
        registry.register(cmd, Arc::new(|args| Ok(args.join(" "))));

        let result = registry.execute("echo", &["hello".to_string(), "world".to_string()]);
        assert_eq!(result, Ok("hello world".to_string()));
    }

    #[test]
    fn test_registry_execute_unknown() {
        let registry = CommandRegistry::new();
        let result = registry.execute("unknown", &[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_registry_find() {
        let mut registry = CommandRegistry::new();
        let cmd = Command::new("quit", "Exit").alias("q");
        registry.register(cmd, Arc::new(|_| Ok("".to_string())));

        assert!(registry.find("quit").is_some());
        assert!(registry.find("q").is_some());
        assert!(registry.find("unknown").is_none());
    }

    #[test]
    fn test_registry_suggest() {
        let mut registry = CommandRegistry::new();
        registry.register(
            Command::new("quit", "Exit").alias("q"),
            Arc::new(|_| Ok("".to_string())),
        );
        registry.register(
            Command::new("query", "Search"),
            Arc::new(|_| Ok("".to_string())),
        );

        let suggestions = registry.suggest("q");
        assert_eq!(suggestions.len(), 2); // Both 'quit' and 'query' start with 'q'
    }

    #[test]
    fn test_registry_suggest_empty() {
        let mut registry = CommandRegistry::new();
        registry.register(
            Command::new("quit", "Exit"),
            Arc::new(|_| Ok("".to_string())),
        );

        let suggestions = registry.suggest("");
        assert_eq!(suggestions.len(), 0);
    }

    #[test]
    fn test_command_mode_default() {
        let mode = CommandMode::default();
        assert!(!mode.is_active());
    }

    #[test]
    fn test_registry_default() {
        let registry = CommandRegistry::default();
        assert_eq!(registry.commands().len(), 0);
    }
}
