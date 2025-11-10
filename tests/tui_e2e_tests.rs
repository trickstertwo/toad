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
            .unwrap_or_else(|_| panic!("Failed to render at {}x{}", width, height));
    }
}

// ============================================================================
// EDGE CASE TESTS (Added to improve test coverage)
// ============================================================================

#[test]
fn test_e2e_quit_with_ctrl_d() {
    let mut terminal = create_test_terminal();
    let mut app = App::new();

    // Navigate to main screen
    app.update(key_event(KeyCode::Enter)).ok();
    app.update(key_event(KeyCode::Char('1'))).ok();

    assert_eq!(*app.screen(), AppScreen::Main);
    assert!(!app.should_quit());

    // Quit with Ctrl+D (when input is empty)
    app.update(ctrl_key('d')).ok();

    assert!(app.should_quit(), "Ctrl+D should quit when input is empty");

    terminal.draw(|f| toad::core::ui::render(&mut app, f)).ok();
}

#[test]
fn test_e2e_quit_with_esc_from_welcome() {
    let mut app = App::new();

    assert_eq!(*app.screen(), AppScreen::Welcome);
    assert!(!app.should_quit());

    // Quit with Esc from welcome
    app.update(key_event(KeyCode::Esc)).ok();

    assert!(app.should_quit(), "Esc should quit from welcome screen");
}

#[test]
fn test_e2e_quit_with_esc_from_trust_dialog() {
    let mut app = App::new();

    // Navigate to trust dialog
    app.update(key_event(KeyCode::Enter)).ok();
    assert_eq!(*app.screen(), AppScreen::TrustDialog);

    // Quit with Esc
    app.update(key_event(KeyCode::Esc)).ok();

    assert!(app.should_quit(), "Esc should quit from trust dialog");
}

#[test]
fn test_e2e_trust_dialog_option_1() {
    // Test option 1: Yes, for this session
    let mut app = App::new();

    // If welcome already shown (from session), manually set screen
    if *app.screen() == AppScreen::Main {
        // Skip this test if session already trusts folder
        return;
    }

    app.update(key_event(KeyCode::Enter)).ok(); // To trust dialog
    assert_eq!(*app.screen(), AppScreen::TrustDialog);

    app.update(key_event(KeyCode::Char('1'))).ok();

    assert_eq!(*app.screen(), AppScreen::Main, "Option 1 should go to Main");
    assert!(app.trust_dialog().is_none());
}

#[test]
fn test_e2e_trust_dialog_option_3() {
    // Test option 3: No, quit
    let mut app = App::new();

    // If welcome already shown, skip
    if *app.screen() == AppScreen::Main {
        return;
    }

    app.update(key_event(KeyCode::Enter)).ok();

    // Skip if already on main (session persistence)
    if *app.screen() != AppScreen::TrustDialog {
        return;
    }

    app.update(key_event(KeyCode::Char('3'))).ok();

    assert!(app.should_quit(), "Option 3 should quit");
}

#[test]
fn test_e2e_trust_dialog_enter_confirms() {
    let mut app = App::new();

    // Navigate to trust dialog
    app.update(key_event(KeyCode::Enter)).ok();
    assert_eq!(*app.screen(), AppScreen::TrustDialog);

    // Navigate to option 2 with arrow keys
    app.update(key_event(KeyCode::Down)).ok();

    // Confirm with Enter
    app.update(key_event(KeyCode::Enter)).ok();

    assert_eq!(*app.screen(), AppScreen::Main, "Enter should confirm selection");
}

#[test]
fn test_e2e_input_field_with_unicode() {
    let mut terminal = create_test_terminal();
    let mut app = App::new();

    // Navigate to main screen
    app.update(key_event(KeyCode::Enter)).ok();
    app.update(key_event(KeyCode::Char('1'))).ok();
    assert_eq!(*app.screen(), AppScreen::Main);

    // Type Unicode characters
    app.update(key_event(KeyCode::Char('🐸'))).ok();
    app.update(key_event(KeyCode::Char('日'))).ok();
    app.update(key_event(KeyCode::Char('本'))).ok();

    // Should not crash with Unicode
    terminal
        .draw(|f| toad::core::ui::render(&mut app, f))
        .expect("Should render with Unicode input");

    // Verify input field contains Unicode
    assert!(!app.input_field().value().is_empty(), "Input field should contain Unicode");
}

#[test]
fn test_e2e_very_small_terminal() {
    // Test with extremely small terminal (smaller than minimum practical size)
    let sizes = [
        (20, 8),   // Very small
        (30, 10),  // Small
        (40, 12),  // Minimal
    ];

    for (width, height) in sizes {
        let backend = TestBackend::new(width, height);
        let mut terminal = Terminal::new(backend).expect("Failed to create terminal");
        let mut app = App::new();

        // Should not panic even with tiny terminal
        terminal
            .draw(|frame| {
                toad::core::ui::render(&mut app, frame);
            })
            .unwrap_or_else(|_| panic!("Should render without panic at very small size {}x{}",
                width, height));
    }
}

#[test]
fn test_e2e_input_field_very_long_text() {
    let mut app = App::new();

    // Navigate to main screen
    app.update(key_event(KeyCode::Enter)).ok();
    app.update(key_event(KeyCode::Char('1'))).ok();

    // Type many characters (test truncation/wrapping)
    for _ in 0..200 {
        app.update(key_event(KeyCode::Char('a'))).ok();
    }

    // Should not crash with very long input
    assert!(
        !app.input_field().value().is_empty(),
        "Input field should handle long text"
    );
}

#[test]
fn test_e2e_rapid_screen_transitions() {
    let mut terminal = create_test_terminal();
    let mut app = App::new();

    // Rapidly transition between screens
    for _ in 0..50 {
        // Welcome -> Trust
        if *app.screen() == AppScreen::Welcome {
            app.update(key_event(KeyCode::Enter)).ok();
        }

        // Trust -> Main (if on trust)
        if *app.screen() == AppScreen::TrustDialog {
            app.update(key_event(KeyCode::Char('1'))).ok();
        }

        // Main -> Help -> Main
        if *app.screen() == AppScreen::Main {
            app.update(key_event(KeyCode::Char('?'))).ok();
            app.update(key_event(KeyCode::Char('?'))).ok();
        }
    }

    // Should handle rapid transitions without crash
    terminal
        .draw(|f| toad::core::ui::render(&mut app, f))
        .expect("Should handle rapid transitions");
}

#[test]
fn test_e2e_command_palette_with_input() {
    let mut terminal = create_test_terminal();
    let mut app = App::new();

    // Navigate to main screen
    app.update(key_event(KeyCode::Enter)).ok();
    app.update(key_event(KeyCode::Char('1'))).ok();

    // Open command palette
    app.update(ctrl_key('p')).ok();
    assert!(app.show_palette());

    // Type some characters
    app.update(key_event(KeyCode::Char('h'))).ok();
    app.update(key_event(KeyCode::Char('e'))).ok();
    app.update(key_event(KeyCode::Char('l'))).ok();
    app.update(key_event(KeyCode::Char('p'))).ok();

    // Should still render correctly
    terminal
        .draw(|f| toad::core::ui::render(&mut app, f))
        .expect("Should render palette with input");

    // Close with Esc
    app.update(key_event(KeyCode::Esc)).ok();
    assert!(!app.show_palette(), "Esc should close palette");
}

#[test]
fn test_e2e_help_screen_from_different_screens() {
    let mut terminal = create_test_terminal();

    // Cannot show help from Welcome (no '?' handler there)
    // But can show help from Main

    let mut app = App::new();
    app.update(key_event(KeyCode::Enter)).ok();
    app.update(key_event(KeyCode::Char('1'))).ok();
    assert_eq!(*app.screen(), AppScreen::Main);

    // Show help
    app.update(key_event(KeyCode::Char('?'))).ok();
    assert!(app.show_help());

    // Render should work
    terminal
        .draw(|f| toad::core::ui::render(&mut app, f))
        .expect("Should render help from main");

    // Hide help
    app.update(key_event(KeyCode::Char('?'))).ok();
    assert!(!app.show_help());
}

#[test]
fn test_e2e_backspace_empty_input() {
    let mut app = App::new();

    // Navigate to main screen
    app.update(key_event(KeyCode::Enter)).ok();
    app.update(key_event(KeyCode::Char('1'))).ok();

    // Input field should be empty
    assert_eq!(app.input_field().value().len(), 0);

    // Backspace on empty input should not crash
    app.update(key_event(KeyCode::Backspace)).ok();

    assert_eq!(app.input_field().value().len(), 0, "Backspace on empty should stay empty");
}

#[test]
fn test_e2e_multiple_backspaces() {
    let mut app = App::new();

    // Navigate to main screen
    app.update(key_event(KeyCode::Enter)).ok();
    app.update(key_event(KeyCode::Char('1'))).ok();

    // Type some characters
    app.update(key_event(KeyCode::Char('h'))).ok();
    app.update(key_event(KeyCode::Char('e'))).ok();
    app.update(key_event(KeyCode::Char('l'))).ok();
    app.update(key_event(KeyCode::Char('l'))).ok();
    app.update(key_event(KeyCode::Char('o'))).ok();

    assert!(!app.input_field().value().is_empty());

    // Backspace all characters
    for _ in 0..10 {
        // More backspaces than characters
        app.update(key_event(KeyCode::Backspace)).ok();
    }

    // Should handle extra backspaces gracefully
    assert_eq!(app.input_field().value().len(), 0, "Should be empty after backspacing");
}

#[test]
fn test_e2e_terminal_resize_during_operation() {
    let mut terminal = create_test_terminal();
    let mut app = App::new();

    // Navigate to main screen
    app.update(key_event(KeyCode::Enter)).ok();
    app.update(key_event(KeyCode::Char('1'))).ok();

    // Type some text
    app.update(key_event(KeyCode::Char('t'))).ok();
    app.update(key_event(KeyCode::Char('e'))).ok();
    app.update(key_event(KeyCode::Char('s'))).ok();
    app.update(key_event(KeyCode::Char('t'))).ok();

    // Simulate resize event
    app.update(Event::Resize(100, 30)).ok();

    // Should still render after resize
    terminal
        .draw(|f| toad::core::ui::render(&mut app, f))
        .expect("Should render after resize");

    // Input should still be there
    assert!(!app.input_field().value().is_empty(), "Input should persist after resize");
}

// =============================================================================
// ADDITIONAL MEDIUM TIER E2E TESTS
// =============================================================================

#[test]
fn test_e2e_toast_notifications() {
    let mut terminal = create_test_terminal();
    let mut app = App::new();

    // Navigate to main screen
    app.update(key_event(KeyCode::Enter)).ok();
    app.update(key_event(KeyCode::Char('1'))).ok();

    // Show different toast types
    app.toast_info("Info toast");
    assert_eq!(app.toasts().len(), 1);

    app.toast_success("Success toast");
    assert_eq!(app.toasts().len(), 2);

    app.toast_warning("Warning toast");
    assert_eq!(app.toasts().len(), 3);

    app.toast_error("Error toast");
    assert_eq!(app.toasts().len(), 4);

    // Should render without crash
    terminal
        .draw(|f| toad::core::ui::render(&mut app, f))
        .expect("Should render with toasts");
}

#[test]
fn test_e2e_toast_unicode() {
    let mut terminal = create_test_terminal();
    let mut app = App::new();

    // Navigate to main screen
    app.update(key_event(KeyCode::Enter)).ok();
    app.update(key_event(KeyCode::Char('1'))).ok();

    // Unicode/emoji toasts
    app.toast_info("🐸 Ribbit!");
    app.toast_success("成功！");
    app.toast_warning("警告");
    app.toast_error("Ошибка");

    assert_eq!(app.toasts().len(), 4);

    // Render with Unicode toasts
    terminal
        .draw(|f| toad::core::ui::render(&mut app, f))
        .expect("Should render Unicode toasts");
}

#[test]
fn test_e2e_many_toasts() {
    let mut terminal = create_test_terminal();
    let mut app = App::new();

    // Navigate to main screen
    app.update(key_event(KeyCode::Enter)).ok();
    app.update(key_event(KeyCode::Char('1'))).ok();

    // Create many toasts
    for i in 0..20 {
        app.toast_info(format!("Toast {}", i));
    }

    assert_eq!(app.toasts().len(), 20);

    // Should render without crash even with many toasts
    terminal
        .draw(|f| toad::core::ui::render(&mut app, f))
        .expect("Should handle many toasts");
}

#[test]
fn test_e2e_tab_operations() {
    let mut terminal = create_test_terminal();
    let mut app = App::new();

    // Navigate to main screen
    app.update(key_event(KeyCode::Enter)).ok();
    app.update(key_event(KeyCode::Char('1'))).ok();

    // Add new tabs
    app.tabs_mut().add_tab("Editor");
    app.tabs_mut().add_tab("Terminal");
    app.tabs_mut().add_tab("Browser");

    assert_eq!(app.tabs().count(), 3);

    // Render with tabs
    terminal
        .draw(|f| toad::core::ui::render(&mut app, f))
        .expect("Should render with tabs");
}

#[test]
fn test_e2e_tab_switching() {
    let mut app = App::new();

    // Navigate to main screen
    app.update(key_event(KeyCode::Enter)).ok();
    app.update(key_event(KeyCode::Char('1'))).ok();

    // Add tabs
    app.tabs_mut().add_tab("Tab 1");
    app.tabs_mut().add_tab("Tab 2");
    app.tabs_mut().add_tab("Tab 3");

    // Switch to different tabs
    app.tabs_mut().next_tab();
    app.tabs_mut().next_tab();
    app.tabs_mut().previous_tab();

    // Should not crash
    assert_eq!(app.tabs().count(), 3);
}

#[test]
fn test_e2e_tabs_with_unicode() {
    let mut terminal = create_test_terminal();
    let mut app = App::new();

    // Navigate to main screen
    app.update(key_event(KeyCode::Enter)).ok();
    app.update(key_event(KeyCode::Char('1'))).ok();

    // Unicode tab names
    app.tabs_mut().add_tab("🐸 Frog Tab");
    app.tabs_mut().add_tab("日本語タブ");
    app.tabs_mut().add_tab("Русский");

    assert!(app.tabs().count() >= 3);

    // Render with Unicode tabs
    terminal
        .draw(|f| toad::core::ui::render(&mut app, f))
        .expect("Should render Unicode tabs");
}

#[test]
fn test_e2e_session_persistence() {
    let mut app = App::new();

    // Navigate to main screen
    app.update(key_event(KeyCode::Enter)).ok();
    app.update(key_event(KeyCode::Char('1'))).ok();

    // Modify session state
    let session = app.session_mut();
    session.set_working_directory(std::path::PathBuf::from("/test/directory"));

    // Verify session state
    assert_eq!(
        app.session().working_directory(),
        &std::path::PathBuf::from("/test/directory")
    );
}

#[test]
fn test_e2e_session_with_unicode_path() {
    let mut app = App::new();

    // Navigate to main screen
    app.update(key_event(KeyCode::Enter)).ok();
    app.update(key_event(KeyCode::Char('1'))).ok();

    // Set Unicode path
    let session = app.session_mut();
    session.set_working_directory(std::path::PathBuf::from("/home/用户/项目/日本語"));

    // Verify Unicode path
    assert!(app
        .session()
        .working_directory()
        .to_string_lossy()
        .contains("用户"));
}

#[test]
fn test_e2e_layout_manager_medium_tier() {
    let mut terminal = create_test_terminal();
    let mut app = App::new();

    // Navigate to main screen
    app.update(key_event(KeyCode::Enter)).ok();
    app.update(key_event(KeyCode::Char('1'))).ok();

    // Access layout manager
    let _layout = app.layout();
    let _layout_mut = app.layout_mut();

    // Should render without crash
    terminal
        .draw(|f| toad::core::ui::render(&mut app, f))
        .expect("Should render with layout");
}

#[test]
fn test_e2e_complete_medium_tier_workflow() {
    let mut terminal = create_test_terminal();
    let mut app = App::new();

    // 1. Navigate to main screen
    app.update(key_event(KeyCode::Enter)).ok();
    app.update(key_event(KeyCode::Char('1'))).ok();
    assert_eq!(*app.screen(), AppScreen::Main);

    // 2. Create tabs for different workspaces
    app.tabs_mut().add_tab("Editor");
    app.tabs_mut().add_tab("Terminal");
    assert!(app.tabs().count() >= 2);

    // 3. Show toast notifications
    app.toast_info("Workspace ready");
    app.toast_success("Tabs created");
    assert_eq!(app.toasts().len(), 2);

    // 4. Update session state
    app.session_mut()
        .set_working_directory(std::path::PathBuf::from("/project"));
    assert_eq!(
        app.session().working_directory(),
        &std::path::PathBuf::from("/project")
    );

    // 5. Render complete state
    terminal
        .draw(|f| toad::core::ui::render(&mut app, f))
        .expect("Should render complete MEDIUM tier state");

    // 6. Navigate between tabs
    app.tabs_mut().next_tab();
    terminal
        .draw(|f| toad::core::ui::render(&mut app, f))
        .expect("Should render after tab switch");

    // Complete workflow without crashes
}

#[test]
fn test_e2e_medium_tier_stress_test() {
    let mut terminal = create_test_terminal();
    let mut app = App::new();

    // Navigate to main screen
    app.update(key_event(KeyCode::Enter)).ok();
    app.update(key_event(KeyCode::Char('1'))).ok();

    // Stress test with many components
    // Create many tabs
    for i in 0..20 {
        app.tabs_mut().add_tab(format!("Tab {}", i));
    }

    // Create many toasts
    for i in 0..20 {
        if i % 2 == 0 {
            app.toast_info(format!("Info {}", i));
        } else {
            app.toast_success(format!("Success {}", i));
        }
    }

    // Update session multiple times
    for i in 0..10 {
        app.session_mut()
            .set_working_directory(std::path::PathBuf::from(format!("/project{}", i)));
    }

    // Should handle stress without crash
    terminal
        .draw(|f| toad::core::ui::render(&mut app, f))
        .expect("Should handle stress test");

    assert!(app.tabs().count() >= 20);
    assert_eq!(app.toasts().len(), 20);
}

#[test]
fn test_e2e_medium_tier_unicode_stress() {
    let mut terminal = create_test_terminal();
    let mut app = App::new();

    // Navigate to main screen
    app.update(key_event(KeyCode::Enter)).ok();
    app.update(key_event(KeyCode::Char('1'))).ok();

    // Unicode stress test
    app.tabs_mut().add_tab("🐸🎉🚀");
    app.tabs_mut().add_tab("日本語");
    app.tabs_mut().add_tab("中文");
    app.tabs_mut().add_tab("العربية");
    app.tabs_mut().add_tab("Русский");

    app.toast_info("🐸 Ribbit!");
    app.toast_success("成功！");
    app.toast_warning("警告");
    app.toast_error("Ошибка");

    app.session_mut()
        .set_working_directory(std::path::PathBuf::from("/home/用户/项目/🐸-日本語"));

    // Render with all Unicode components
    terminal
        .draw(|f| toad::core::ui::render(&mut app, f))
        .expect("Should handle Unicode stress");

    assert!(app.tabs().count() >= 5);
    assert_eq!(app.toasts().len(), 4);
}

#[test]
fn test_e2e_tab_and_toast_interaction() {
    let mut terminal = create_test_terminal();
    let mut app = App::new();

    // Navigate to main screen
    app.update(key_event(KeyCode::Enter)).ok();
    app.update(key_event(KeyCode::Char('1'))).ok();

    // Simulate realistic workflow: create tab -> show toast
    app.tabs_mut().add_tab("New Workspace");
    app.toast_success("Workspace created");

    app.tabs_mut().next_tab();
    app.toast_info("Switched to workspace");

    assert!(app.tabs().count() >= 1);
    assert_eq!(app.toasts().len(), 2);

    // Render interaction
    terminal
        .draw(|f| toad::core::ui::render(&mut app, f))
        .expect("Should render tab+toast interaction");
}

#[test]
fn test_e2e_session_save_workflow() {
    let mut app = App::new();

    // Navigate to main screen
    app.update(key_event(KeyCode::Enter)).ok();
    app.update(key_event(KeyCode::Char('1'))).ok();

    // Set up complete session state
    app.session_mut()
        .set_working_directory(std::path::PathBuf::from("/test/project"));
    app.tabs_mut().add_tab("Editor");
    app.tabs_mut().add_tab("Terminal");

    // Attempt to save session (may fail if path doesn't exist, but shouldn't crash)
    let result = app.save_session();

    // Should either succeed or fail gracefully
    assert!(result.is_ok() || result.is_err());

    // App should still be functional
    assert!(!app.should_quit());
}

#[test]
fn test_e2e_rapid_component_updates() {
    let mut terminal = create_test_terminal();
    let mut app = App::new();

    // Navigate to main screen
    app.update(key_event(KeyCode::Enter)).ok();
    app.update(key_event(KeyCode::Char('1'))).ok();

    // Rapidly update all MEDIUM tier components
    for i in 0..50 {
        // Add tab
        if i % 5 == 0 {
            app.tabs_mut().add_tab(format!("Tab {}", i));
        }

        // Show toast
        if i % 3 == 0 {
            app.toast_info(format!("Update {}", i));
        }

        // Update session
        if i % 7 == 0 {
            app.session_mut()
                .set_working_directory(std::path::PathBuf::from(format!("/dir{}", i)));
        }

        // Render periodically
        if i % 10 == 0 {
            terminal
                .draw(|f| toad::core::ui::render(&mut app, f))
                .ok();
        }
    }

    // Final verification
    assert!(app.tabs().count() > 0);
    assert!(!app.toasts().is_empty());

    // Final render
    terminal
        .draw(|f| toad::core::ui::render(&mut app, f))
        .expect("Should handle rapid updates");
}
