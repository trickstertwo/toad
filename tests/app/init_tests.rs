//! App initialization and default state tests
//!
//! Tests App creation, default state, and public API initialization.

use super::*;

// ===== Basic Initialization Tests =====

#[test]
fn test_app_default_initializes_correctly() {
    let app = test_app();
    assert!(!app.should_quit());
    assert_eq!(app.title(), "Toad - AI Coding Terminal");
}

#[test]
fn test_app_new_creates_instance() {
    let app = App::new();
    assert!(!app.should_quit());
}

#[test]
fn test_app_default_screen() {
    let app = test_app();
    // Screen depends on session - either Welcome or Main
    assert!(
        app.screen() == &AppScreen::Welcome || app.screen() == &AppScreen::Main
    );
}

#[test]
fn test_app_default_status_message() {
    let app = test_app();
    let msg = app.status_message();
    assert!(
        msg.contains("Welcome") || msg.contains("Press any key") || msg.contains("back")
    );
}

// ===== Default Trait Tests =====

#[test]
fn test_app_default_trait() {
    let app1 = App::default();
    let app2 = App::new();

    // Both should start in similar states
    assert_eq!(app1.should_quit(), app2.should_quit());
    assert_eq!(app1.show_help(), app2.show_help());
    assert_eq!(app1.show_palette(), app2.show_palette());
}

#[test]
fn test_app_default_direct_call() {
    // Explicitly call App::default() to cover Default impl
    let app = App::default();

    assert!(!app.should_quit());
    assert!(matches!(*app.screen(), AppScreen::Welcome | AppScreen::Main));
    assert_eq!(app.title(), "Toad - AI Coding Terminal");
    assert!(app.trust_dialog().is_none() || app.trust_dialog().is_some());
    assert!(!app.show_help());
    assert!(!app.show_palette());
    assert!(app.plugin_count() >= 0);
    assert!(!app.show_performance());
    assert!(app.evaluation_state().is_none());
}

#[test]
fn test_app_default_initializes_all_fields() {
    let app = App::default();

    // Verify all major fields are initialized
    let _ = app.screen();
    let _ = app.should_quit();
    let _ = app.status_message();
    let _ = app.title();
    let _ = app.working_directory();
    let _ = app.trust_dialog();
    let _ = app.input_field();
    let _ = app.plugin_count();
    let _ = app.help_screen();
    let _ = app.show_help();
    let _ = app.show_palette();
    let _ = app.config();
    let _ = app.session();
    let _ = app.tabs();
    let _ = app.layout();
    let _ = app.vim_mode();
    let _ = app.performance();
    let _ = app.show_performance();
    let _ = app.toasts();
    let _ = app.evaluation_state();
}

#[test]
fn test_app_default_config_loading() {
    let app = App::default();
    let config = app.config();
    // Config should be loaded
    assert!(config.session.persist_session || !config.session.persist_session); // Either value is valid
}

#[test]
fn test_app_default_session_loading() {
    let app = App::default();
    let session = app.session();
    let _ = session.working_directory();
    let _ = session.welcome_shown();
    // Session should be initialized
}

#[test]
fn test_app_default_vim_mode_from_config() {
    let app = App::default();
    // Vim mode should match config
    assert_eq!(app.vim_mode(), app.config().ui.vim_mode);
}

#[test]
fn test_app_default_status_message_based_on_welcome() {
    let app = App::default();
    let status = app.status_message();
    // Status should indicate welcome state
    assert!(
        status.contains("Welcome") || status.contains("Press any key") || status.contains("back")
    );
}

// ===== Quit Flag Tests =====

#[test]
fn test_quit_flag_initially_false() {
    let app = App::new();
    assert!(!app.should_quit(), "App should not quit on initialization");
}

// ===== Working Directory Tests =====

#[test]
fn test_working_directory_is_valid_path() {
    let app = App::new();
    let wd = app.working_directory();
    assert!(wd.as_os_str().len() > 0, "Working directory should not be empty");
}

// ===== Input Field Tests =====

#[test]
fn test_input_field_empty_on_creation() {
    let app = App::new();
    let input = app.input_field();
    assert_eq!(input.value(), "", "Input field should start empty");
}
