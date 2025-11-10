//! Statusline widget module

mod state;
#[cfg(test)]
mod tests;

pub use state::{SectionAlignment, StatusLevel, StatusSection, Statusline};
