//! Toad - AI-powered coding terminal with semi-autonomous agents
//!
//! Main entry point for the application.

use std::time::Duration;
use toad::{App, Result, Tui};

fn main() -> Result<()> {
    // Initialize error handling
    install_panic_hook();

    // Initialize logging
    init_logging()?;

    tracing::info!("Starting Toad TUI");

    // Run the application
    let result = run();

    // Log shutdown
    if let Err(ref e) = result {
        tracing::error!("Application error: {}", e);
    } else {
        tracing::info!("Application shutdown successfully");
    }

    result
}

/// Main application loop
///
/// This implements the Elm Architecture:
/// 1. Init: Create initial state (App, Tui, EventHandler)
/// 2. Loop:
///    - Wait for events
///    - Update state based on events
///    - Render new state
/// 3. Cleanup: Restore terminal (handled by Tui Drop)
fn run() -> Result<()> {
    // Init: Create initial state
    let mut tui = Tui::new()?;
    let mut app = App::new();
    let event_handler = toad::event::EventHandler::new(Duration::from_millis(250));

    tracing::info!("TUI initialized, entering main loop");

    // Main event loop
    while !app.should_quit() {
        // View: Render the current state
        tui.draw(|frame| {
            toad::ui::render(&mut app, frame);
        })?;

        // Wait for event (blocking)
        let event = event_handler.next()?;

        // Update: Process event and update state
        app.update(event)?;
    }

    tracing::info!("Exiting main loop");

    Ok(())
}

/// Install a panic hook that restores the terminal before panicking
///
/// This ensures that even if the application panics, the terminal
/// is properly restored to its original state.
fn install_panic_hook() {
    let original_hook = std::panic::take_hook();

    std::panic::set_hook(Box::new(move |panic_info| {
        // Attempt to restore terminal
        let _ = crossterm::terminal::disable_raw_mode();
        let _ = crossterm::execute!(
            std::io::stdout(),
            crossterm::terminal::LeaveAlternateScreen,
            crossterm::event::DisableMouseCapture
        );

        // Call original panic hook
        original_hook(panic_info);
    }));
}

/// Initialize logging to a file
///
/// Logs are written to `toad.log` in the current directory.
fn init_logging() -> Result<()> {
    use tracing_subscriber::{fmt, prelude::*, EnvFilter};

    // Create log file
    let log_file = std::fs::File::create("toad.log")?;

    // Set up logging to file
    tracing_subscriber::registry()
        .with(
            fmt::layer()
                .with_writer(log_file)
                .with_ansi(false)
                .with_target(true)
                .with_line_number(true),
        )
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("toad=debug,info")),
        )
        .init();

    Ok(())
}
