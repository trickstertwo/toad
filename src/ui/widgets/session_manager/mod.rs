//! Session manager module

mod state;
#[cfg(test)]
mod tests;

pub use state::{SessionData, SessionManager};
