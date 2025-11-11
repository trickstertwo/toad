//! Input widgets for text entry and editing
//!
//! This module contains widgets for user input including text areas,
//! input fields, command palettes, vim-style editing, macros, and mode indicators.

pub mod command_palette;
pub mod input;
pub mod input_dialog;
pub mod input_prompt;
pub mod mode_indicator;
pub mod palette;
pub mod textarea;
pub mod vim_macros;
pub mod vim_mode;

// Re-export all types for backwards compatibility
pub use command_palette::*;
pub use input::*;
pub use input_dialog::*;
pub use input_prompt::*;
pub use mode_indicator::*;
pub use palette::*;
pub use textarea::*;
pub use vim_macros::*;
pub use vim_mode::*;
