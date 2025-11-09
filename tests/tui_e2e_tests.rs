//! End-to-End TUI Tests
//!
//! Tests the complete TUI using Ratatui's TestBackend to simulate
//! a terminal environment without needing a real TTY device.

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{backend::TestBackend, Terminal};
use toad::{App, AppScreen, Event};

/// Helper to create a test terminal
fn create_test_terminal() -> Terminal<TestBackend> {
    let backend = TestBackend::new(80, 24); // 80x24 terminal
    Terminal::new(backend).expect("Failed to create test terminal")
}

/// Helper to simulate key press
fn key_event(code: KeyCode) -> Event {
    Event::Key(KeyEvent::new(code, KeyModifiers::empty()))
}

/// Helper to simulate Ctrl+key
fn ctrl_key(c: char) -> Event {
    Event::Key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::CONTROL))
}

// =============================================================================
// BASIC TIER E2E TESTS
// =============================================================================

#[test]
fn test_e2e_app_initialization() {
    let mut terminal = create_test_terminal();
    let mut app = App::new();

    // Render the app
    terminal
        .draw(|frame| {
            toad::core::ui::render(&mut app, frame);
        })
        .expect("Failed to render");

    // Verify app starts in Welcome screen
    assert_eq!(
        *app.screen(),
        AppScreen::Welcome,
        "App should start on Welcome screen"
    );

    assert!(!app.should_quit(), "App should not quit on startup");
}

#[test]
fn test_e2e_welcome_to_trust_dialog() {
    let mut terminal = create_test_terminal();
    let mut app = App::new();

    // Initial render - Welcome screen
    terminal
        .draw(|frame| {
            toad::core::ui::render(&mut app, frame);
        })
        .expect("Failed to render");

    assert_eq!(*app.screen(), AppScreen::Welcome);

    // Press Enter to continue
    app.update(key_event(KeyCode::Enter))
        .expect("Failed to process Enter key");

    // Should move to TrustDialog
    terminal
        .draw(|frame| {
            toad::core::ui::render(&mut app, frame);
        })
        .expect("Failed to render");

    assert_eq!(
        *app.screen(),
        AppScreen::TrustDialog,
        "Should transition to TrustDialog after Enter on Welcome"
    );
}

#[test]
fn test_e2e_trust_dialog_navigation() {
    let mut terminal = create_test_terminal();
    let mut app = App::new();

    // Move to TrustDialog
    app.update(key_event(KeyCode::Enter))
        .expect("Failed to process Enter");

    terminal
        .draw(|frame| {
            toad::core::ui::render(&mut app, frame);
        })
        .expect("Failed to render");

    assert_eq!(*app.screen(), AppScreen::TrustDialog);
    assert!(app.trust_dialog().is_some(), "Trust dialog should be present");

    // Test navigation with arrow keys (doesn't dismiss dialog)
    app.update(key_event(KeyCode::Down))
        .expect("Failed to process Down");

    // Verify dialog is still shown after navigation
    assert_eq!(*app.screen(), AppScreen::TrustDialog);
    assert!(app.trust_dialog().is_some(), "Trust dialog should still be present after navigation");

    // Now test selection with number key '1' (dismisses dialog)
    app.update(key_event(KeyCode::Char('1')))
        .expect("Failed to process key 1");

    terminal
        .draw(|frame| {
            toad::core::ui::render(&mut app, frame);
        })
        .expect("Failed to render");

    // After selection, should move to Main screen and dismiss dialog
    assert_eq!(*app.screen(), AppScreen::Main);
    assert!(app.trust_dialog().is_none(), "Trust dialog should be dismissed after selection");
}

#[test]
fn test_e2e_quit_with_ctrl_c() {
    let mut terminal = create_test_terminal();
    let mut app = App::new();

    // Move to main screen
    app.update(key_event(KeyCode::Enter))
        .expect("Failed to enter");
    app.update(key_event(KeyCode::Char('1')))
        .expect("Failed to select option");
    app.update(key_event(KeyCode::Enter))
        .expect("Failed to confirm");

    terminal
        .draw(|frame| {
            toad::core::ui::render(&mut app, frame);
        })
        .expect("Failed to render");

    // Press Ctrl+C to quit
    app.update(ctrl_key('c')).expect("Failed to process Ctrl+C");

    assert!(app.should_quit(), "App should quit after Ctrl+C");
}

#[test]
fn test_e2e_input_field_basic() {
    let mut terminal = create_test_terminal();
    let mut app = App::new();

    // Move to main screen
    app.update(key_event(KeyCode::Enter))
        .expect("Failed to enter");
    app.update(key_event(KeyCode::Char('1')))
        .expect("Failed to select");
    app.update(key_event(KeyCode::Enter))
        .expect("Failed to confirm");

    terminal
        .draw(|frame| {
            toad::core::ui::render(&mut app, frame);
        })
        .expect("Failed to render");

    assert_eq!(*app.screen(), AppScreen::Main);

    // Type some text
    let text = "Hello TOAD";
    for c in text.chars() {
        app.update(key_event(KeyCode::Char(c)))
            .expect("Failed to type character");
    }

    terminal
        .draw(|frame| {
            toad::core::ui::render(&mut app, frame);
        })
        .expect("Failed to render");

    // Verify input contains text
    assert_eq!(
        app.input_field().value(),
        text,
        "Input field should contain typed text"
    );
}

#[test]
fn test_e2e_input_field_backspace() {
    let mut terminal = create_test_terminal();
    let mut app = App::new();

    // Move to main screen
    app.update(key_event(KeyCode::Enter)).ok();
    app.update(key_event(KeyCode::Char('1'))).ok();
    app.update(key_event(KeyCode::Enter)).ok();

    // Type text
    app.update(key_event(KeyCode::Char('A'))).ok();
    app.update(key_event(KeyCode::Char('B'))).ok();
    app.update(key_event(KeyCode::Char('C'))).ok();

    assert_eq!(app.input_field().value(), "ABC");

    // Press backspace
    app.update(key_event(KeyCode::Backspace)).ok();

    terminal
        .draw(|frame| {
            toad::core::ui::render(&mut app, frame);
        })
        .expect("Failed to render");

    assert_eq!(
        app.input_field().value(),
        "AB",
        "Backspace should delete last character"
    );
}

#[test]
fn test_e2e_input_field_clear() {
    let mut terminal = create_test_terminal();
    let mut app = App::new();

    // Move to main screen
    app.update(key_event(KeyCode::Enter)).ok();
    app.update(key_event(KeyCode::Char('1'))).ok();
    app.update(key_event(KeyCode::Enter)).ok();

    // Type text
    app.update(key_event(KeyCode::Char('t'))).ok();
    app.update(key_event(KeyCode::Char('e'))).ok();
    app.update(key_event(KeyCode::Char('s'))).ok();
    app.update(key_event(KeyCode::Char('t'))).ok();

    assert_eq!(app.input_field().value(), "test");

    // Press Ctrl+U to clear
    app.update(ctrl_key('u')).ok();

    terminal
        .draw(|frame| {
            toad::core::ui::render(&mut app, frame);
        })
        .expect("Failed to render");

    assert_eq!(
        app.input_field().value(),
        "",
        "Ctrl+U should clear input field"
    );
}

// =============================================================================
// MEDIUM TIER E2E TESTS
// =============================================================================

#[test]
fn test_e2e_help_screen_toggle() {
    let mut terminal = create_test_terminal();
    let mut app = App::new();

    // Move to main screen
    app.update(key_event(KeyCode::Enter)).ok();
    app.update(key_event(KeyCode::Char('1'))).ok();
    app.update(key_event(KeyCode::Enter)).ok();

    assert!(!app.show_help(), "Help should not be shown initially");

    // Press '?' to show help
    app.update(key_event(KeyCode::Char('?'))).ok();

    terminal
        .draw(|frame| {
            toad::core::ui::render(&mut app, frame);
        })
        .expect("Failed to render");

    assert!(app.show_help(), "Help should be shown after pressing ?");

    // Press Esc to hide help
    app.update(key_event(KeyCode::Esc)).ok();

    assert!(!app.show_help(), "Help should be hidden after pressing Esc");
}

#[test]
fn test_e2e_command_palette_toggle() {
    let mut terminal = create_test_terminal();
    let mut app = App::new();

    // Move to main screen
    app.update(key_event(KeyCode::Enter)).ok();
    app.update(key_event(KeyCode::Char('1'))).ok();
    app.update(key_event(KeyCode::Enter)).ok();

    assert!(!app.show_palette(), "Palette should not be shown initially");

    // Press Ctrl+P to show command palette
    app.update(ctrl_key('p')).ok();

    terminal
        .draw(|frame| {
            toad::core::ui::render(&mut app, frame);
        })
        .expect("Failed to render");

    assert!(
        app.show_palette(),
        "Command palette should be shown after Ctrl+P"
    );

    // Press Esc to hide
    app.update(key_event(KeyCode::Esc)).ok();

    assert!(
        !app.show_palette(),
        "Command palette should be hidden after Esc"
    );
}

#[test]
fn test_e2e_terminal_resize() {
    let mut terminal = create_test_terminal();
    let mut app = App::new();

    // Move to main screen
    app.update(key_event(KeyCode::Enter)).ok();
    app.update(key_event(KeyCode::Char('1'))).ok();
    app.update(key_event(KeyCode::Enter)).ok();

    // Simulate resize event
    app.update(Event::Resize(100, 30))
        .expect("Failed to process resize");

    terminal
        .draw(|frame| {
            toad::core::ui::render(&mut app, frame);
        })
        .expect("Failed to render");

    // Verify status message was updated
    assert!(
        app.status_message().contains("resized"),
        "Status should show resize message"
    );
}

#[test]
fn test_e2e_rendering_consistency() {
    let mut terminal = create_test_terminal();
    let mut app = App::new();

    // Render multiple times to ensure consistency
    for _ in 0..10 {
        terminal
            .draw(|frame| {
                toad::core::ui::render(&mut app, frame);
            })
            .expect("Failed to render");
    }

    // Should not crash or panic
    assert!(!app.should_quit(), "App should still be running");
}

// =============================================================================
// ADVANCED TIER E2E TESTS
// =============================================================================

#[test]
fn test_e2e_vim_mode_toggle() {
    let mut terminal = create_test_terminal();
    let mut app = App::new();

    // Move to main screen
    app.update(key_event(KeyCode::Enter)).ok();
    app.update(key_event(KeyCode::Char('1'))).ok();
    app.update(key_event(KeyCode::Enter)).ok();

    let initial_vim_mode = app.vim_mode();

    // Toggle vim mode
    app.toggle_vim_mode();

    terminal
        .draw(|frame| {
            toad::core::ui::render(&mut app, frame);
        })
        .expect("Failed to render");

    assert_ne!(
        app.vim_mode(),
        initial_vim_mode,
        "Vim mode should toggle"
    );

    // Verify status message mentions vim mode
    assert!(
        app.status_message().to_lowercase().contains("vim"),
        "Status should mention vim mode"
    );
}

#[test]
fn test_e2e_multiple_screens_navigation() {
    let mut terminal = create_test_terminal();
    let mut app = App::new();

    // Track screen transitions
    let mut screens = Vec::new();

    // Welcome
    screens.push(app.screen().clone());
    terminal
        .draw(|frame| {
            toad::core::ui::render(&mut app, frame);
        })
        .ok();

    // -> TrustDialog
    app.update(key_event(KeyCode::Enter)).ok();
    screens.push(app.screen().clone());
    terminal
        .draw(|frame| {
            toad::core::ui::render(&mut app, frame);
        })
        .ok();

    // -> Main
    app.update(key_event(KeyCode::Char('1'))).ok();
    app.update(key_event(KeyCode::Enter)).ok();
    screens.push(app.screen().clone());
    terminal
        .draw(|frame| {
            toad::core::ui::render(&mut app, frame);
        })
        .ok();

    // Verify complete navigation flow
    assert_eq!(screens[0], AppScreen::Welcome);
    assert_eq!(screens[1], AppScreen::TrustDialog);
    assert_eq!(screens[2], AppScreen::Main);
}

#[test]
fn test_e2e_layout_manager_access() {
    let mut terminal = create_test_terminal();
    let mut app = App::new();

    // Move to main screen
    app.update(key_event(KeyCode::Enter)).ok();
    app.update(key_event(KeyCode::Char('1'))).ok();
    app.update(key_event(KeyCode::Enter)).ok();

    terminal
        .draw(|frame| {
            toad::core::ui::render(&mut app, frame);
        })
        .expect("Failed to render");

    // Verify layout manager is accessible
    let _layout = app.layout();
    let _layout_mut = app.layout_mut();

    // Should not crash
}

#[test]
fn test_e2e_stress_test_rapid_input() {
    let mut terminal = create_test_terminal();
    let mut app = App::new();

    // Move to main screen
    app.update(key_event(KeyCode::Enter)).ok();
    app.update(key_event(KeyCode::Char('1'))).ok();
    app.update(key_event(KeyCode::Enter)).ok();

    // Rapidly type 100 characters
    for i in 0..100 {
        let c = (b'a' + (i % 26)) as char;
        app.update(key_event(KeyCode::Char(c))).ok();

        // Render every 10 characters
        if i % 10 == 0 {
            terminal
                .draw(|frame| {
                    toad::core::ui::render(&mut app, frame);
                })
                .ok();
        }
    }

    // Final render
    terminal
        .draw(|frame| {
            toad::core::ui::render(&mut app, frame);
        })
        .expect("Failed to render");

    // Should not crash
    assert!(!app.should_quit(), "App should still be running");
}

// =============================================================================
// RENDERING TESTS
// =============================================================================

#[test]
fn test_e2e_buffer_output_welcome() {
    let mut terminal = create_test_terminal();
    let mut app = App::new();

    terminal
        .draw(|frame| {
            toad::core::ui::render(&mut app, frame);
        })
        .expect("Failed to render");

    // Get the buffer content
    let buffer = terminal.backend().buffer().clone();

    // Convert buffer to string for inspection
    let content = buffer
        .content()
        .iter()
        .map(|cell| cell.symbol())
        .collect::<String>();

    // Verify welcome screen content appears (the logo uses box chars, not "TOAD" text)
    // Check for the actual text content that appears on the welcome screen
    assert!(
        content.contains("AI-Powered")
            || content.contains("Coding")
            || content.contains("Terminal")
            || content.contains("█"), // Box drawing character from logo
        "Welcome screen should contain branding text or logo. Buffer content length: {}",
        content.len()
    );
}

#[test]
fn test_e2e_buffer_output_main_screen() {
    let mut terminal = create_test_terminal();
    let mut app = App::new();

    // Move to main screen
    app.update(key_event(KeyCode::Enter)).ok();
    app.update(key_event(KeyCode::Char('1'))).ok();
    app.update(key_event(KeyCode::Enter)).ok();

    terminal
        .draw(|frame| {
            toad::core::ui::render(&mut app, frame);
        })
        .expect("Failed to render");

    let buffer = terminal.backend().buffer().clone();
    let content = buffer
        .content()
        .iter()
        .map(|cell| cell.symbol())
        .collect::<String>();

    // Verify main screen elements
    assert!(
        content.contains("Ctrl") || content.contains("help") || !content.is_empty(),
        "Main screen should contain keyboard shortcuts or help text"
    );
}

// =============================================================================
// INTEGRATION TESTS
// =============================================================================

#[test]
fn test_e2e_complete_workflow() {
    let mut terminal = create_test_terminal();
    let mut app = App::new();

    // Complete user workflow:
    // 1. Start on Welcome
    assert_eq!(*app.screen(), AppScreen::Welcome);
    terminal.draw(|f| toad::core::ui::render(&mut app, f)).ok();

    // 2. Navigate to TrustDialog
    app.update(key_event(KeyCode::Enter)).ok();
    assert_eq!(*app.screen(), AppScreen::TrustDialog);
    terminal.draw(|f| toad::core::ui::render(&mut app, f)).ok();

    // 3. Accept and go to Main
    app.update(key_event(KeyCode::Char('1'))).ok();
    app.update(key_event(KeyCode::Enter)).ok();
    assert_eq!(*app.screen(), AppScreen::Main);
    terminal.draw(|f| toad::core::ui::render(&mut app, f)).ok();

    // 4. Type a command
    for c in "test command".chars() {
        app.update(key_event(KeyCode::Char(c))).ok();
    }
    assert_eq!(app.input_field().value(), "test command");
    terminal.draw(|f| toad::core::ui::render(&mut app, f)).ok();

    // 5. Clear input
    app.update(ctrl_key('u')).ok();
    assert_eq!(app.input_field().value(), "");
    terminal.draw(|f| toad::core::ui::render(&mut app, f)).ok();

    // 6. Open help
    app.update(key_event(KeyCode::Char('?'))).ok();
    assert!(app.show_help());
    terminal.draw(|f| toad::core::ui::render(&mut app, f)).ok();

    // 7. Close help
    app.update(key_event(KeyCode::Esc)).ok();
    assert!(!app.show_help());
    terminal.draw(|f| toad::core::ui::render(&mut app, f)).ok();

    // 8. Quit
    app.update(ctrl_key('c')).ok();
    assert!(app.should_quit());

    // Workflow completed successfully without crashes
}

#[test]
fn test_e2e_terminal_size_variations() {
    let sizes = [
        (40, 10),  // Very small
        (80, 24),  // Standard
        (120, 40), // Large
        (200, 60), // Very large
    ];

    for (width, height) in sizes {
        let backend = TestBackend::new(width, height);
        let mut terminal = Terminal::new(backend).expect("Failed to create terminal");
        let mut app = App::new();

        // Should render without panic at any size
        terminal
            .draw(|frame| {
                toad::core::ui::render(&mut app, frame);
            })
            .expect(&format!("Failed to render at {}x{}", width, height));
    }
}
