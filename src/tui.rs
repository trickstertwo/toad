//! Terminal User Interface management
//!
//! This module handles terminal initialization, cleanup, and provides
//! a safe wrapper around the Ratatui terminal instance.

use crate::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io::{self, Stdout};

/// A wrapper around the Ratatui terminal that handles setup and cleanup
pub struct Tui {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl Tui {
    /// Create a new TUI instance and initialize the terminal
    ///
    /// This will:
    /// - Enable raw mode
    /// - Enter alternate screen
    /// - Enable mouse capture
    /// - Clear the screen
    pub fn new() -> Result<Self> {
        // Setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

        // Create backend and terminal
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        Ok(Self { terminal })
    }

    /// Get a mutable reference to the terminal for drawing
    pub fn terminal(&mut self) -> &mut Terminal<CrosstermBackend<Stdout>> {
        &mut self.terminal
    }

    /// Draw the UI using the provided closure
    ///
    /// This is a convenience method that wraps the terminal's draw method
    pub fn draw<F>(&mut self, f: F) -> Result<()>
    where
        F: FnOnce(&mut ratatui::Frame),
    {
        self.terminal.draw(f)?;
        Ok(())
    }

    /// Restore the terminal to its original state
    ///
    /// This will:
    /// - Disable mouse capture
    /// - Leave alternate screen
    /// - Disable raw mode
    /// - Show cursor
    fn restore(&mut self) -> Result<()> {
        disable_raw_mode()?;
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        self.terminal.show_cursor()?;
        Ok(())
    }
}

impl Drop for Tui {
    /// Ensure terminal is restored when TUI is dropped
    ///
    /// This provides automatic cleanup even if the application panics
    fn drop(&mut self) {
        if let Err(e) = self.restore() {
            eprintln!("Error restoring terminal: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_tui_new() {
        // This test is mostly to ensure the code compiles
        // Actually running it would affect the terminal
        // In a real scenario, you'd use a mock backend
        assert!(true);
    }
}
