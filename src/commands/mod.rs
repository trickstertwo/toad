//! Commands domain
//!
//! Contains command-related functionality including command mode, aliases,
//! quick actions, autocomplete, smart suggestions, and slash commands.

pub mod aliases;
pub mod autocomplete;
pub mod command_mode;
pub mod quick_actions;
pub mod slash_parser;
pub mod smart_suggestions;

pub use aliases::{Alias, AliasManager};
pub use autocomplete::{
    AutocompleteManager, AutocompleteProvider, CommandProvider, Suggestion, WordProvider,
};
pub use command_mode::{Command, CommandHandler, CommandMode, CommandRegistry, CommandResult};
pub use quick_actions::{ActionCategory, QuickAction, QuickActionManager};
pub use slash_parser::{
    parse_slash_command, parse_slash_command_quoted, SlashCommand, SlashCommandDef,
    SlashCommandRegistry,
};
pub use smart_suggestions::{
    ContextBuilder, SmartSuggestions, Suggestion as SmartSuggestion, SuggestionContext,
    SuggestionType,
};
