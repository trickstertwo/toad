//! Slash command parser for power user commands
//!
//! Provides parsing and execution of slash commands (`/command args`) for
//! context management, model switching, git operations, and session management.
//!
//! # Features
//!
//! - Slash command parsing (`/add`, `/model`, `/commit`, etc.)
//! - Argument parsing and validation
//! - Tab completion support
//! - Command aliases (`/m` → `/model`)
//! - Fuzzy matching for command names
//!
//! # Examples
//!
//! ```
//! use toad::commands::slash_parser::{SlashCommand, parse_slash_command};
//!
//! let cmd = parse_slash_command("/add src/**/*.rs").unwrap();
//! assert_eq!(cmd.name, "add");
//! assert_eq!(cmd.args, vec!["src/**/*.rs"]);
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A parsed slash command
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SlashCommand {
    /// Command name (without `/` prefix)
    pub name: String,
    /// Command arguments
    pub args: Vec<String>,
    /// Raw input string
    pub raw: String,
}

impl SlashCommand {
    /// Create a new slash command
    pub fn new(name: impl Into<String>, args: Vec<String>) -> Self {
        let name = name.into();
        let raw = if args.is_empty() {
            format!("/{}", name)
        } else {
            format!("/{} {}", name, args.join(" "))
        };

        Self { name, args, raw }
    }

    /// Check if command has arguments
    pub fn has_args(&self) -> bool {
        !self.args.is_empty()
    }

    /// Get first argument
    pub fn first_arg(&self) -> Option<&str> {
        self.args.first().map(|s| s.as_str())
    }

    /// Get all arguments as a single string
    pub fn args_string(&self) -> String {
        self.args.join(" ")
    }
}

/// Slash command definition
#[derive(Debug, Clone)]
pub struct SlashCommandDef {
    /// Command name
    pub name: String,
    /// Command aliases
    pub aliases: Vec<String>,
    /// Command description
    pub description: String,
    /// Usage example
    pub usage: String,
    /// Whether command requires arguments
    pub requires_args: bool,
    /// Argument count (None = any, Some(n) = exactly n)
    pub arg_count: Option<usize>,
}

impl SlashCommandDef {
    /// Create a new command definition
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        let name = name.into();
        let description = description.into();
        let usage = format!("/{}", name);

        Self {
            name,
            aliases: Vec::new(),
            description,
            usage,
            requires_args: false,
            arg_count: None,
        }
    }

    /// Add an alias
    pub fn alias(mut self, alias: impl Into<String>) -> Self {
        self.aliases.push(alias.into());
        self
    }

    /// Set usage example
    pub fn usage(mut self, usage: impl Into<String>) -> Self {
        self.usage = usage.into();
        self
    }

    /// Require arguments
    pub fn requires_args(mut self) -> Self {
        self.requires_args = true;
        self
    }

    /// Set exact argument count
    pub fn arg_count(mut self, count: usize) -> Self {
        self.arg_count = Some(count);
        self
    }

    /// Check if this command matches a name (including aliases)
    pub fn matches(&self, name: &str) -> bool {
        self.name == name || self.aliases.iter().any(|a| a == name)
    }

    /// Validate command arguments
    pub fn validate(&self, cmd: &SlashCommand) -> Result<(), String> {
        if self.requires_args && cmd.args.is_empty() {
            return Err(format!("Command '{}' requires arguments", self.name));
        }

        if let Some(expected) = self.arg_count {
            if cmd.args.len() != expected {
                return Err(format!(
                    "Command '{}' requires exactly {} argument(s), got {}",
                    self.name,
                    expected,
                    cmd.args.len()
                ));
            }
        }

        Ok(())
    }
}

/// Slash command registry
#[derive(Debug, Clone)]
pub struct SlashCommandRegistry {
    /// Registered commands
    commands: HashMap<String, SlashCommandDef>,
}

impl Default for SlashCommandRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl SlashCommandRegistry {
    /// Create a new registry with default commands
    pub fn new() -> Self {
        let mut registry = Self {
            commands: HashMap::new(),
        };

        registry.register_default_commands();
        registry
    }

    /// Register default slash commands
    fn register_default_commands(&mut self) {
        // Context management
        self.register(
            SlashCommandDef::new("add", "Add files to context")
                .alias("a")
                .usage("/add <pattern>")
                .requires_args()
                .arg_count(1),
        );

        self.register(
            SlashCommandDef::new("drop", "Remove file from context")
                .alias("d")
                .alias("remove")
                .usage("/drop <file>")
                .requires_args()
                .arg_count(1),
        );

        self.register(
            SlashCommandDef::new("clear-context", "Remove all files from context")
                .alias("cc")
                .usage("/clear-context"),
        );

        // Model/Provider switching
        self.register(
            SlashCommandDef::new("model", "Switch model")
                .alias("m")
                .usage("/model <name>")
                .requires_args()
                .arg_count(1),
        );

        self.register(
            SlashCommandDef::new("provider", "Switch provider")
                .alias("p")
                .usage("/provider <name>")
                .requires_args()
                .arg_count(1),
        );

        // Git operations
        self.register(
            SlashCommandDef::new("undo", "Revert last AI change")
                .alias("u")
                .usage("/undo"),
        );

        self.register(
            SlashCommandDef::new("diff", "Show changes since last commit")
                .usage("/diff"),
        );

        self.register(
            SlashCommandDef::new("commit", "Manual commit")
                .usage("/commit <message>")
                .requires_args(),
        );

        // Session management
        self.register(
            SlashCommandDef::new("save", "Save session")
                .alias("s")
                .usage("/save <name>"),
        );

        self.register(
            SlashCommandDef::new("load", "Load session")
                .alias("l")
                .usage("/load <name>")
                .requires_args()
                .arg_count(1),
        );

        // Conversation management
        self.register(
            SlashCommandDef::new("clear", "Clear conversation")
                .usage("/clear"),
        );

        self.register(
            SlashCommandDef::new("reset", "Full reset")
                .usage("/reset"),
        );

        // Help
        self.register(
            SlashCommandDef::new("help", "Show help")
                .alias("h")
                .alias("?")
                .usage("/help [command]"),
        );
    }

    /// Register a command
    pub fn register(&mut self, def: SlashCommandDef) {
        // Register main name
        self.commands.insert(def.name.clone(), def.clone());

        // Register aliases
        for alias in &def.aliases {
            self.commands.insert(alias.clone(), def.clone());
        }
    }

    /// Get command definition by name
    pub fn get(&self, name: &str) -> Option<&SlashCommandDef> {
        self.commands.get(name)
    }

    /// Get all unique command definitions
    pub fn all_commands(&self) -> Vec<&SlashCommandDef> {
        let mut unique: HashMap<String, &SlashCommandDef> = HashMap::new();
        for def in self.commands.values() {
            unique.insert(def.name.clone(), def);
        }
        unique.values().copied().collect()
    }

    /// Find commands matching a prefix (for autocomplete)
    pub fn find_matches(&self, prefix: &str) -> Vec<String> {
        let mut matches: Vec<String> = self
            .commands
            .keys()
            .filter(|name| name.starts_with(prefix))
            .cloned()
            .collect();

        matches.sort();
        matches.dedup();
        matches
    }

    /// Fuzzy match command name
    pub fn fuzzy_match(&self, input: &str) -> Option<String> {
        // Try exact match first
        if self.commands.contains_key(input) {
            return Some(input.to_string());
        }

        // Try prefix match
        let matches = self.find_matches(input);
        if matches.len() == 1 {
            return Some(matches[0].clone());
        }

        // Try substring match
        let substring_matches: Vec<_> = self
            .commands
            .keys()
            .filter(|name| name.contains(input))
            .collect();

        if substring_matches.len() == 1 {
            return Some(substring_matches[0].clone());
        }

        None
    }

    /// Validate a command
    pub fn validate(&self, cmd: &SlashCommand) -> Result<(), String> {
        let def = self
            .get(&cmd.name)
            .ok_or_else(|| format!("Unknown command: '{}'", cmd.name))?;

        def.validate(cmd)
    }
}

/// Parse a slash command from input
///
/// # Examples
///
/// ```
/// use toad::commands::slash_parser::parse_slash_command;
///
/// let cmd = parse_slash_command("/add src/**/*.rs").unwrap();
/// assert_eq!(cmd.name, "add");
/// assert_eq!(cmd.args, vec!["src/**/*.rs"]);
///
/// let cmd2 = parse_slash_command("/model claude-sonnet").unwrap();
/// assert_eq!(cmd2.name, "model");
/// assert_eq!(cmd2.first_arg(), Some("claude-sonnet"));
/// ```
pub fn parse_slash_command(input: &str) -> Option<SlashCommand> {
    let input = input.trim();

    // Check for slash prefix
    if !input.starts_with('/') {
        return None;
    }

    // Remove leading slash
    let input = &input[1..];

    // Split into parts
    let parts: Vec<&str> = input.split_whitespace().collect();

    if parts.is_empty() {
        return None;
    }

    let name = parts[0].to_string();
    let args: Vec<String> = parts[1..].iter().map(|s| s.to_string()).collect();

    Some(SlashCommand::new(name, args))
}

/// Parse a slash command with quoted argument support
///
/// Supports both single and double quotes for arguments with spaces:
/// - `/commit "feat: add new feature"`
/// - `/commit 'fix: bug in parser'`
pub fn parse_slash_command_quoted(input: &str) -> Option<SlashCommand> {
    let input = input.trim();

    // Check for slash prefix
    if !input.starts_with('/') {
        return None;
    }

    // Remove leading slash
    let input = &input[1..];

    if input.is_empty() {
        return None;
    }

    // Parse with quoted strings
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut in_quote = false;
    let mut quote_char = ' ';
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '\'' | '"' if !in_quote => {
                in_quote = true;
                quote_char = ch;
            }
            c if in_quote && c == quote_char => {
                in_quote = false;
                if !current.is_empty() {
                    parts.push(current.clone());
                    current.clear();
                }
            }
            ' ' if !in_quote => {
                if !current.is_empty() {
                    parts.push(current.clone());
                    current.clear();
                }
            }
            c => {
                current.push(c);
            }
        }
    }

    // Push remaining
    if !current.is_empty() {
        parts.push(current);
    }

    if parts.is_empty() {
        return None;
    }

    let name = parts[0].clone();
    let args = parts[1..].to_vec();

    Some(SlashCommand::new(name, args))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_command() {
        let cmd = parse_slash_command("/add").unwrap();
        assert_eq!(cmd.name, "add");
        assert_eq!(cmd.args, Vec::<String>::new());
    }

    #[test]
    fn test_parse_command_with_args() {
        let cmd = parse_slash_command("/add src/**/*.rs").unwrap();
        assert_eq!(cmd.name, "add");
        assert_eq!(cmd.args, vec!["src/**/*.rs"]);
    }

    #[test]
    fn test_parse_command_with_multiple_args() {
        let cmd = parse_slash_command("/model claude-sonnet --temp 0.7").unwrap();
        assert_eq!(cmd.name, "model");
        assert_eq!(cmd.args, vec!["claude-sonnet", "--temp", "0.7"]);
    }

    #[test]
    fn test_parse_invalid_command() {
        assert!(parse_slash_command("add src").is_none());
        assert!(parse_slash_command("").is_none());
        assert!(parse_slash_command("/").is_none());
    }

    #[test]
    fn test_parse_quoted_command() {
        let cmd = parse_slash_command_quoted("/commit \"feat: add new feature\"").unwrap();
        assert_eq!(cmd.name, "commit");
        assert_eq!(cmd.args, vec!["feat: add new feature"]);
    }

    #[test]
    fn test_parse_quoted_command_single_quotes() {
        let cmd = parse_slash_command_quoted("/commit 'fix: bug in parser'").unwrap();
        assert_eq!(cmd.name, "commit");
        assert_eq!(cmd.args, vec!["fix: bug in parser"]);
    }

    #[test]
    fn test_command_def_validation() {
        let def = SlashCommandDef::new("add", "Add files")
            .requires_args()
            .arg_count(1);

        let cmd_valid = SlashCommand::new("add", vec!["test.rs".to_string()]);
        assert!(def.validate(&cmd_valid).is_ok());

        let cmd_no_args = SlashCommand::new("add", vec![]);
        assert!(def.validate(&cmd_no_args).is_err());

        let cmd_too_many = SlashCommand::new("add", vec!["a".to_string(), "b".to_string()]);
        assert!(def.validate(&cmd_too_many).is_err());
    }

    #[test]
    fn test_registry_get() {
        let registry = SlashCommandRegistry::new();
        assert!(registry.get("add").is_some());
        assert!(registry.get("a").is_some()); // alias
        assert!(registry.get("nonexistent").is_none());
    }

    #[test]
    fn test_registry_find_matches() {
        let registry = SlashCommandRegistry::new();
        let matches = registry.find_matches("ad");
        assert!(matches.contains(&"add".to_string()));
    }

    #[test]
    fn test_registry_fuzzy_match() {
        let registry = SlashCommandRegistry::new();
        assert_eq!(registry.fuzzy_match("add"), Some("add".to_string()));
        assert_eq!(registry.fuzzy_match("a"), Some("a".to_string()));
        assert_eq!(registry.fuzzy_match("ad"), Some("add".to_string()));
    }

    #[test]
    fn test_registry_validate() {
        let registry = SlashCommandRegistry::new();

        let cmd_valid = SlashCommand::new("add", vec!["test.rs".to_string()]);
        assert!(registry.validate(&cmd_valid).is_ok());

        let cmd_invalid = SlashCommand::new("add", vec![]);
        assert!(registry.validate(&cmd_invalid).is_err());

        let cmd_unknown = SlashCommand::new("unknown", vec![]);
        assert!(registry.validate(&cmd_unknown).is_err());
    }

    #[test]
    fn test_slash_command_methods() {
        let cmd = SlashCommand::new("add", vec!["file1.rs".to_string(), "file2.rs".to_string()]);
        assert!(cmd.has_args());
        assert_eq!(cmd.first_arg(), Some("file1.rs"));
        assert_eq!(cmd.args_string(), "file1.rs file2.rs");
    }
}
