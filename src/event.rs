//! Event handling module
//!
//! This module defines the Event type which represents all possible
//! messages/events in the Elm Architecture pattern.

use crossterm::event::{self, KeyEvent, MouseEvent};
use std::time::Duration;

/// Events that can occur in the application (Message in Elm Architecture)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    /// Terminal tick event (for animations, etc.)
    Tick,

    /// Key press event
    Key(KeyEvent),

    /// Mouse event
    Mouse(MouseEvent),

    /// Terminal resize event
    Resize(u16, u16),

    /// Application should quit
    Quit,
}

/// Event handler that polls for terminal events
pub struct EventHandler {
    #[allow(dead_code)]
    tick_rate: Duration,
}

impl EventHandler {
    /// Create a new event handler with the given tick rate
    pub fn new(tick_rate: Duration) -> Self {
        Self { tick_rate }
    }

    /// Poll for the next event
    ///
    /// This blocks until an event is available or the timeout is reached.
    pub fn next(&self) -> crate::Result<Event> {
        // Check if there's an event available
        if event::poll(self.tick_rate)? {
            match event::read()? {
                event::Event::Key(key) => Ok(Event::Key(key)),
                event::Event::Mouse(mouse) => Ok(Event::Mouse(mouse)),
                event::Event::Resize(width, height) => Ok(Event::Resize(width, height)),
                _ => Ok(Event::Tick),
            }
        } else {
            Ok(Event::Tick)
        }
    }
}
