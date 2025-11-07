//! Application state module (Model in Elm Architecture)
//!
//! This module contains the application state and the update logic
//! that handles state transitions based on events.

use crate::event::Event;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Application state (Model in Elm Architecture)
#[derive(Debug)]
pub struct App {
    /// Whether the application should quit
    should_quit: bool,

    /// Status bar message
    status_message: String,

    /// Application title
    title: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            should_quit: false,
            status_message: "Press 'q' to quit".to_string(),
            title: "Toad - AI Coding Terminal".to_string(),
        }
    }
}

impl App {
    /// Create a new application instance (Init in Elm Architecture)
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if the application should quit
    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    /// Get the current status message
    pub fn status_message(&self) -> &str {
        &self.status_message
    }

    /// Get the application title
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Update application state based on an event (Update in Elm Architecture)
    ///
    /// This is the core state transition function that takes an event
    /// and produces a new state.
    pub fn update(&mut self, event: Event) -> crate::Result<()> {
        match event {
            Event::Key(key_event) => self.handle_key_event(key_event),
            Event::Resize(width, height) => {
                self.status_message = format!("Terminal resized to {}x{}", width, height);
                Ok(())
            }
            Event::Quit => {
                self.should_quit = true;
                Ok(())
            }
            Event::Tick => Ok(()),
            _ => Ok(()),
        }
    }

    /// Handle keyboard events
    fn handle_key_event(&mut self, key: KeyEvent) -> crate::Result<()> {
        match (key.code, key.modifiers) {
            // Quit on 'q' or Ctrl+C
            (KeyCode::Char('q'), _) | (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                self.should_quit = true;
            }
            // Escape key
            (KeyCode::Esc, _) => {
                self.should_quit = true;
            }
            // Display key press in status bar
            (KeyCode::Char(c), _) => {
                self.status_message = format!("Pressed: '{}'", c);
            }
            _ => {}
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_init() {
        let app = App::new();
        assert!(!app.should_quit());
        assert_eq!(app.title(), "Toad - AI Coding Terminal");
    }

    #[test]
    fn test_quit_on_q() {
        let mut app = App::new();
        let event = Event::Key(KeyEvent::from(KeyCode::Char('q')));
        app.update(event).unwrap();
        assert!(app.should_quit());
    }
}
