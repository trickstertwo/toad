//! Common test utilities for app integration tests

use toad::core::{App, AppScreen, Event};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

// ===== Test Modules =====

mod init_tests;
mod accessor_tests;
mod event_tests;

// ===== Test Utilities =====

/// Create a test app instance
pub fn test_app() -> App {
    App::new()
}

/// Create a key event from KeyCode
pub fn key(code: KeyCode) -> Event {
    Event::Key(KeyEvent::from(code))
}

/// Create a key event with modifiers
pub fn key_with_mods(code: KeyCode, mods: KeyModifiers) -> Event {
    Event::Key(KeyEvent::new(code, mods))
}
