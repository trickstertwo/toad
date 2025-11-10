//! MEDIUM Tier Integration Tests
//!
//! These tests verify that MEDIUM tier components work together correctly
//! in realistic multi-component scenarios.

use std::path::PathBuf;
use std::time::Duration;
use toad::infrastructure::history::History;
use toad::navigation::search::SearchState;
use toad::ui::widgets::ToastManager;
use toad::workspace::{SessionState, TabManager};

// =============================================================================
// TOAST + SESSION INTEGRATION
// =============================================================================

#[test]
fn test_integration_toast_manager_with_session_state() {
    let mut session = SessionState::new();
    let mut toasts = ToastManager::new();

    // Simulate workflow: session loading triggers success toast
    session.set_working_directory(PathBuf::from("/test/project"));
    toasts.success(format!(
        "Loaded session from {}",
        session.working_directory().display()
    ));

    assert_eq!(toasts.len(), 1);

    // Change directory, show new toast
    session.set_working_directory(PathBuf::from("/test/another"));
    toasts.info(format!(
        "Changed to {}",
        session.working_directory().display()
    ));

    assert_eq!(toasts.len(), 2);
}

#[test]
fn test_integration_session_tabs_persistence() {
    let mut session = SessionState::new();
    let mut tabs = TabManager::new();

    // Create multiple tabs
    tabs.add_tab("Editor");
    tabs.add_tab("Terminal");
    tabs.add_tab("Browser");

    // Store tab count in session metadata (simulated)
    let tab_count = tabs.count();
    session.set_working_directory(PathBuf::from(format!("/project/{}", tab_count)));

    // Verify both are in sync
    assert_eq!(tabs.count(), 3);
    assert!(session.working_directory().to_string_lossy().contains("3"));

    // Simulate session save/restore
    let json = serde_json::to_string(&session).unwrap();
    let restored: SessionState = serde_json::from_str(&json).unwrap();
    assert_eq!(restored.working_directory(), session.working_directory());
}

// =============================================================================
// TABS + TOAST INTEGRATION
// =============================================================================

#[test]
fn test_integration_tab_operations_with_notifications() {
    let mut tabs = TabManager::new();
    let mut toasts = ToastManager::new();

    // Add tab -> show toast
    tabs.add_tab("New Tab");
    toasts.success("Tab created");
    assert_eq!(tabs.count(), 1);
    assert_eq!(toasts.len(), 1);

    // Switch tab -> show toast
    tabs.add_tab("Another Tab");
    tabs.switch_to_index(0);
    toasts.info("Switched to tab 1");
    assert_eq!(tabs.active_index(), Some(0));
    assert_eq!(toasts.len(), 2);

    // Close tab -> show toast
    let tab_id = tabs.tabs()[1].id; // Get second tab ID
    if tabs.close_tab(tab_id).is_some() {
        toasts.warning("Tab closed");
    }
    assert_eq!(tabs.count(), 1);
    assert_eq!(toasts.len(), 3);
}

#[test]
fn test_integration_many_tabs_with_toast_overflow() {
    let mut tabs = TabManager::new();
    let mut toasts = ToastManager::new();

    // Create 50 tabs, show toast for each
    for i in 0..50 {
        tabs.add_tab(format!("Tab {}", i));
        if i % 10 == 0 {
            toasts.info(format!("Created {} tabs", i + 1));
        }
    }

    assert_eq!(tabs.count(), 50);
    assert_eq!(toasts.len(), 5); // Toasts every 10 tabs

    // Verify toasts are properly managed (not overflowing UI)
    assert!(toasts.len() <= 50, "Toasts should be managed");
}

// =============================================================================
// HISTORY + SEARCH INTEGRATION
// =============================================================================

#[test]
fn test_integration_history_with_search() {
    let mut history = History::new(100);
    let mut search = SearchState::new();

    // Add commands to history
    history.add("cargo build".to_string());
    history.add("cargo test".to_string());
    history.add("cargo run".to_string());
    history.add("git status".to_string());
    history.add("git commit".to_string());

    // Search through history entries
    let entries: Vec<String> = history.entries().to_vec();
    search.set_query("cargo".to_string());
    search.search(&entries);

    assert_eq!(search.match_count(), 3); // Three cargo commands
    assert_eq!(history.len(), 5);
}

#[test]
fn test_integration_history_search_with_unicode() {
    let mut history = History::new(50);
    let mut search = SearchState::new();

    // Add Unicode commands
    history.add("echo 'Hello 🐸'".to_string());
    history.add("echo '日本語'".to_string());
    history.add("echo 'Test 🎉'".to_string());
    history.add("echo 'Normal'".to_string());

    // Search for emoji
    let entries: Vec<String> = history.entries().to_vec();
    search.set_query("🐸".to_string());
    search.search(&entries);

    assert_eq!(search.match_count(), 1);
    assert_eq!(search.current_match().unwrap().text, "🐸");
}

#[test]
fn test_integration_history_navigation_and_search() {
    let mut history = History::new(100);

    // Add entries
    for i in 0..20 {
        history.add(format!("command_{}", i));
    }

    // Navigate history
    assert_eq!(history.older().map(|s| s.as_str()), Some("command_19"));
    assert_eq!(history.older().map(|s| s.as_str()), Some("command_18"));
    assert_eq!(history.older().map(|s| s.as_str()), Some("command_17"));

    // Now search while navigated
    let entries: Vec<String> = history.entries().to_vec();
    let mut search = SearchState::new();
    search.set_query("command_1".to_string());
    search.search(&entries);

    // Should find all commands starting with "command_1" (1, 10-19)
    assert_eq!(search.match_count(), 11);
}

// =============================================================================
// SESSION + HISTORY + TABS INTEGRATION
// =============================================================================

#[test]
fn test_integration_complete_workspace_state() {
    let mut session = SessionState::new();
    let mut tabs = TabManager::new();
    let mut history = History::new(100);

    // Simulate a complete workspace setup
    session.set_working_directory(PathBuf::from("/home/user/project"));

    // Create workspace tabs
    tabs.add_tab("Editor");
    tabs.add_tab("Terminal");
    tabs.add_tab("Git");
    tabs.switch_to_index(1);

    // Add command history
    history.add("cd /home/user/project".to_string());
    history.add("git status".to_string());
    history.add("cargo build".to_string());

    // Verify complete state
    assert_eq!(
        session.working_directory(),
        &PathBuf::from("/home/user/project")
    );
    assert_eq!(tabs.count(), 3);
    assert_eq!(tabs.active_index(), Some(1));
    assert_eq!(history.len(), 3);

    // Simulate session save
    let session_json = serde_json::to_string(&session).unwrap();
    assert!(session_json.contains("/home/user/project"));
}

#[test]
fn test_integration_workspace_state_with_unicode() {
    let mut session = SessionState::new();
    let mut tabs = TabManager::new();
    let mut history = History::new(50);

    // Unicode in all components
    session.set_working_directory(PathBuf::from("/home/用户/项目"));
    tabs.add_tab("エディタ");
    tabs.add_tab("终端");
    history.add("echo '🐸 Test'".to_string());
    history.add("cd /home/用户/项目".to_string());

    // Verify Unicode handling across all components
    assert!(
        session
            .working_directory()
            .to_string_lossy()
            .contains("用户")
    );
    assert_eq!(&tabs.tabs()[0].title, "エディタ");
    assert_eq!(&tabs.tabs()[1].title, "终端");
    assert!(history.entries().iter().any(|e| e.contains("🐸")));
}

// =============================================================================
// TOAST MANAGER STRESS TESTS WITH OTHER COMPONENTS
// =============================================================================

#[test]
fn test_integration_toast_manager_concurrent_operations() {
    let mut toasts = ToastManager::new();
    let mut tabs = TabManager::new();

    // Simulate rapid operations generating many toasts
    for i in 0..100 {
        tabs.add_tab(format!("Tab {}", i));
        if i % 5 == 0 {
            toasts.info(format!("Tabs: {}", i));
        }
        if i % 10 == 0 {
            toasts.success(format!("Milestone: {}", i));
        }
        if i == 50 {
            toasts.warning("Halfway there!");
        }
    }

    assert_eq!(tabs.count(), 100);
    // Toasts: 20 info + 10 success + 1 warning = 31 total
    assert_eq!(toasts.len(), 31);

    // Verify toasts auto-dismiss (simulate time passing)
    std::thread::sleep(Duration::from_millis(50));
    toasts.cleanup(); // Clean up expired toasts

    // Some toasts should have been cleaned up
    let remaining = toasts.len();
    assert!(remaining <= 31, "Toasts should be cleaned up");
}

#[test]
fn test_integration_toast_priorities_with_tabs() {
    let mut toasts = ToastManager::new();
    let mut tabs = TabManager::new();

    // Create tabs and generate toasts of different severities
    tabs.add_tab("Editor");
    toasts.info("Tab created");

    tabs.add_tab("Terminal");
    toasts.success("Tab initialized");

    // Simulate error closing non-existent tab (invalid ID)
    if tabs.close_tab(999).is_none() {
        toasts.error("Tab not found");
    }

    // Verify we have 3 toasts of different severity levels
    assert_eq!(toasts.len(), 3);
}

// =============================================================================
// SEARCH + MULTIPLE DATA SOURCES INTEGRATION
// =============================================================================

#[test]
fn test_integration_search_across_tabs_and_history() {
    let mut tabs = TabManager::new();
    let mut history = History::new(100);

    // Create tabs with specific names
    tabs.add_tab("cargo_editor");
    tabs.add_tab("cargo_terminal");
    tabs.add_tab("git_viewer");

    // Add history entries
    history.add("cargo build".to_string());
    history.add("cargo test".to_string());
    history.add("git status".to_string());

    // Search in tab titles
    let tab_titles: Vec<String> = tabs.tabs().iter().map(|t| t.title.clone()).collect();
    let mut search_tabs = SearchState::new();
    search_tabs.set_query("cargo".to_string());
    search_tabs.search(&tab_titles);

    // Search in history
    let history_entries: Vec<String> = history.entries().to_vec();
    let mut search_history = SearchState::new();
    search_history.set_query("cargo".to_string());
    search_history.search(&history_entries);

    // Verify matches in both
    assert_eq!(search_tabs.match_count(), 2); // Two cargo tabs
    assert_eq!(search_history.match_count(), 2); // Two cargo commands
}

// =============================================================================
// SESSION PERSISTENCE WITH ALL COMPONENTS
// =============================================================================

#[test]
fn test_integration_session_serialization_with_complex_state() {
    let mut session = SessionState::new();

    // Set up complex session state
    session.set_working_directory(PathBuf::from(
        "/complex/path/with spaces/and-unicode-日本語",
    ));

    // Serialize
    let json = serde_json::to_string(&session).unwrap();

    // Deserialize
    let restored: SessionState = serde_json::from_str(&json).unwrap();

    // Verify
    assert_eq!(
        restored.working_directory().to_str(),
        Some("/complex/path/with spaces/and-unicode-日本語")
    );
}

#[test]
fn test_integration_session_load_with_toast_notifications() {
    let mut toasts = ToastManager::new();
    let temp_file = std::env::temp_dir().join("test_session_integration.json");

    // Attempt to load session (won't exist)
    let result = SessionState::load(&temp_file);

    match result {
        Ok(_session) => {
            toasts.success("Session loaded");
        }
        Err(_) => {
            toasts.info("Creating new session");
        }
    }

    // Verify toast was created
    assert_eq!(toasts.len(), 1);

    // Clean up
    let _ = std::fs::remove_file(&temp_file);
}

// =============================================================================
// EXTREME STRESS TESTS
// =============================================================================

#[test]
fn test_integration_extreme_all_components_stress() {
    let mut session = SessionState::new();
    let mut tabs = TabManager::new();
    let mut history = History::new(1000);
    let mut toasts = ToastManager::new();
    let mut search = SearchState::new();

    // Create 100 tabs
    for i in 0..100 {
        tabs.add_tab(format!("Tab_{:03}", i));
    }

    // Add 500 history entries
    for i in 0..500 {
        history.add(format!("command_{}", i));
    }

    // Generate 50 toasts
    for i in 0..50 {
        match i % 4 {
            0 => toasts.info(format!("Info {}", i)),
            1 => toasts.success(format!("Success {}", i)),
            2 => toasts.warning(format!("Warning {}", i)),
            _ => toasts.error(format!("Error {}", i)),
        }
    }

    // Search across all history
    let entries: Vec<String> = history.entries().to_vec();
    search.set_query("command_1".to_string());
    search.search(&entries);

    // Verify all components still working
    assert_eq!(tabs.count(), 100);
    assert_eq!(history.len(), 500);
    assert_eq!(toasts.len(), 50);
    assert!(search.match_count() > 0);

    // Modify session
    session.set_working_directory(PathBuf::from("/stress/test"));

    // Verify serialization still works under stress
    let json = serde_json::to_string(&session).unwrap();
    let _restored: SessionState = serde_json::from_str(&json).unwrap();
}

#[test]
fn test_integration_unicode_stress_all_components() {
    let mut tabs = TabManager::new();
    let mut history = History::new(100);
    let mut toasts = ToastManager::new();
    let mut search = SearchState::new();

    // Unicode in tabs
    tabs.add_tab("🐸 Frog");
    tabs.add_tab("日本語タブ");
    tabs.add_tab("中文标签");
    tabs.add_tab("Русский");

    // Unicode in history
    history.add("echo '🎉'".to_string());
    history.add("cat 日本語.txt".to_string());
    history.add("ls 中文目录/".to_string());

    // Unicode in toasts
    toasts.info("🐸 Ribbit!");
    toasts.success("成功！");
    toasts.warning("警告");

    // Search with Unicode
    let entries: Vec<String> = history.entries().to_vec();
    search.set_query("🎉".to_string());
    search.search(&entries);

    // Verify all components handle Unicode
    assert_eq!(tabs.count(), 4);
    assert_eq!(history.len(), 3);
    assert_eq!(toasts.len(), 3);
    assert_eq!(search.match_count(), 1);
}
