//! Commands domain
//!
//! Contains command-related functionality including command mode, aliases,
//! quick actions, autocomplete, and smart suggestions.

pub mod aliases;
pub mod autocomplete;
pub mod command_mode;
pub mod quick_actions;
pub mod smart_suggestions;

pub use aliases::{Alias, AliasManager};
pub use autocomplete::{
    AutocompleteManager, AutocompleteProvider, CommandProvider, Suggestion, WordProvider,
};
pub use command_mode::{Command, CommandHandler, CommandMode, CommandRegistry, CommandResult};
pub use quick_actions::{ActionCategory, QuickAction, QuickActionManager};
pub use smart_suggestions::{
    ContextBuilder, SmartSuggestions, Suggestion as SmartSuggestion, SuggestionContext,
    SuggestionType,
};
