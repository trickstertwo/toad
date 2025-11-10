//! Vim mode module

mod state;
#[cfg(test)]
mod tests;

pub use state::{EditMode, ModeIndicator, Selection, VimMode};
