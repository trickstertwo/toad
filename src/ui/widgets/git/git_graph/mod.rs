//! Git graph module

mod state;
#[cfg(test)]
mod tests;

pub use state::{GitCommit, GitGraph};
