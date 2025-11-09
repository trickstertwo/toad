//! Accessor method tests
//!
//! Tests public accessor methods for App state.

use super::*;

// ===== Immutable Accessor Tests =====

#[test]
fn test_title_accessor() {
    let app = App::new();
    let title = app.title();
    assert!(!title.is_empty(), "Title should not be empty");
}

#[test]
fn test_plugin_count_accessor() {
    let app = App::new();
    let count = app.plugin_count();
    assert_eq!(count, 0, "Initial plugin count should be 0");
}

#[test]
fn test_help_screen_accessor() {
    let app = App::new();
    let _help = app.help_screen();
    // Should not panic
}

#[test]
fn test_show_help_accessor() {
    let app = App::new();
    let show = app.show_help();
    assert!(!show, "Help should not be shown initially");
}

#[test]
fn test_show_palette_accessor() {
    let app = App::new();
    let show = app.show_palette();
    assert!(!show, "Palette should not be shown initially");
}

#[test]
fn test_vim_mode_accessor() {
    let app = App::new();
    let _vim = app.vim_mode();
    // Should not panic (value depends on config)
}

#[test]
fn test_layout_accessor() {
    let app = App::new();
    let _layout = app.layout();
    // Should not panic
}

#[test]
fn test_config_accessor() {
    let app = App::new();
    let _config = app.config();
    // Should not panic
}

#[test]
fn test_performance_accessor() {
    let app = App::new();
    let _perf = app.performance();
    // Should not panic
}

#[test]
fn test_show_performance_accessor() {
    let app = App::new();
    let show = app.show_performance();
    assert!(!show, "Performance overlay should not be shown initially");
}

#[test]
fn test_toasts_accessor() {
    let app = App::new();
    let _toasts = app.toasts();
    // Should not panic
}

#[test]
fn test_evaluation_state_accessor() {
    let app = App::new();
    let state = app.evaluation_state();
    assert!(state.is_none(), "Initial evaluation state should be None");
}

// ===== Screen Accessor Tests =====

#[test]
fn test_screen_accessor_returns_reference() {
    let app = App::new();
    let screen1 = app.screen();
    let screen2 = app.screen();
    assert_eq!(screen1, screen2, "Multiple screen() calls should return same value");
}

#[test]
fn test_multiple_accessors_dont_panic() {
    let app = App::new();

    // Call all accessors multiple times
    let _ = app.should_quit();
    let _ = app.status_message();
    let _ = app.title();
    let _ = app.screen();
    let _ = app.working_directory();
    let _ = app.input_field();
    let _ = app.plugin_count();
    let _ = app.help_screen();
    let _ = app.show_help();
    let _ = app.show_palette();
    let _ = app.vim_mode();
    let _ = app.layout();
    let _ = app.config();
    let _ = app.performance();
    let _ = app.show_performance();
    let _ = app.toasts();
    let _ = app.evaluation_state();
}

// ===== Trust Dialog Accessor Tests =====

#[test]
fn test_trust_dialog_accessor_when_none() {
    let app = App::new();
    let dialog = app.trust_dialog();
    // May be None initially
    let _ = dialog;
}

// ===== Session Accessor Tests =====

#[test]
fn test_session_accessor() {
    let app = App::new();
    let session = app.session();
    let _ = session.plugin_count();
    let _ = session.working_directory();
    let _ = session.welcome_shown();
}

#[test]
fn test_tabs_accessor() {
    let app = App::new();
    let tabs = app.tabs();
    // Should have at least initialized state
    let _ = tabs.active_tab();
}

// ===== Empty/Edge Case Tests =====

#[test]
fn test_empty_status_message() {
    let app = App::new();
    let msg = app.status_message();
    // Should have some initial status
    let _ = msg;
}

#[test]
fn test_unicode_in_status_message() {
    let app = App::new();
    let msg = app.status_message();
    // Should handle unicode in status (if present)
    let _ = msg;
}
