//! Event handling integration tests
//!
//! Tests event processing through public API.

use super::*;

// ===== Basic Event Handler Tests =====

#[test]
fn test_event_quit() {
    let mut app = App::new();
    assert!(!app.should_quit());

    let event = Event::Quit;
    app.update(event).unwrap();

    assert!(app.should_quit());
}

#[test]
fn test_event_resize() {
    let app = App::new();
    let event = Event::Resize(100, 50);
    let _ = app;  // Not actually testing resize handling here, just Event creation

    // Verify the Resize event can be created with proper values
    match event {
        Event::Resize(w, h) => {
            assert_eq!(w, 100);
            assert_eq!(h, 50);
        }
        _ => panic!("Expected Resize event"),
    }
}

#[test]
fn test_event_resize_various_sizes() {
    let mut app = App::new();

    // Test various terminal sizes
    for (width, height) in [(80, 24), (120, 40), (200, 60), (40, 12)] {
        let event = Event::Resize(width, height);
        app.update(event).unwrap();
    }

    // Should handle all sizes without panic
}

#[test]
fn test_event_mouse() {
    use crossterm::event::{MouseEvent, MouseEventKind, KeyModifiers};

    let mut app = App::new();
    let mouse_event = MouseEvent {
        kind: MouseEventKind::Down(crossterm::event::MouseButton::Left),
        column: 10,
        row: 5,
        modifiers: KeyModifiers::NONE,
    };

    let event = Event::Mouse(mouse_event);
    app.update(event).unwrap();

    // Mouse events are currently no-ops but shouldn't panic
}

#[test]
fn test_event_tick() {
    let mut app = App::new();
    let event = Event::Tick;
    app.update(event).unwrap();

    // Tick events should not panic
}

#[test]
fn test_event_tick_multiple() {
    let mut app = App::new();

    // Multiple ticks
    for _ in 0..10 {
        let event = Event::Tick;
        app.update(event).unwrap();
    }

    // Should handle multiple ticks
}
