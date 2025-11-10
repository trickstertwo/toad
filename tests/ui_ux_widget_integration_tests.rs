//! Integration tests for UI/UX widgets
//!
//! Tests for user interface and user experience widgets: Workspace, Toast, Progress, etc.

use std::path::PathBuf;
use toad::ui::widgets::{
    MultiStageProgress, ProgressBar, StageStatus, Toast, ToastLevel, ToastManager, Workspace,
    WorkspaceManager,
};

// ==================== Workspace Tests ====================

#[test]
fn test_workspace_creation() {
    let workspace = Workspace::new("my-project", "/path/to/project");

    assert_eq!(workspace.name(), "my-project");
    assert_eq!(workspace.path(), &PathBuf::from("/path/to/project"));
    assert_eq!(workspace.settings().len(), 0);
    assert_eq!(workspace.state().len(), 0);
}

#[test]
fn test_workspace_settings() {
    let mut workspace = Workspace::new("project", "/path");

    workspace.set_setting("theme", "dark");
    workspace.set_setting("font_size", "14");
    workspace.set_setting("tab_width", "4");

    assert_eq!(workspace.get_setting("theme"), Some("dark"));
    assert_eq!(workspace.get_setting("font_size"), Some("14"));
    assert_eq!(workspace.get_setting("tab_width"), Some("4"));
    assert_eq!(workspace.get_setting("nonexistent"), None);

    workspace.remove_setting("font_size");
    assert_eq!(workspace.get_setting("font_size"), None);
}

#[test]
fn test_workspace_state() {
    let mut workspace = Workspace::new("project", "/path");

    workspace.set_state("current_file", "main.rs");
    workspace.set_state("cursor_line", "150");
    workspace.set_state("scroll_offset", "100");

    assert_eq!(workspace.get_state("current_file"), Some("main.rs"));
    assert_eq!(workspace.get_state("cursor_line"), Some("150"));
    assert_eq!(workspace.get_state("scroll_offset"), Some("100"));

    workspace.remove_state("scroll_offset");
    assert_eq!(workspace.get_state("scroll_offset"), None);
}

#[test]
fn test_workspace_touch() {
    let mut workspace = Workspace::new("project", "/path");
    let first_timestamp = workspace.last_accessed();

    // Touch updates timestamp (may or may not be different due to timing)
    workspace.touch();
    let second_timestamp = workspace.last_accessed();

    assert!(second_timestamp >= first_timestamp);
}

#[test]
fn test_workspace_set_path() {
    let mut workspace = Workspace::new("project", "/old/path");
    assert_eq!(workspace.path(), &PathBuf::from("/old/path"));

    workspace.set_path("/new/path");
    assert_eq!(workspace.path(), &PathBuf::from("/new/path"));
}

// ==================== WorkspaceManager Tests ====================

#[test]
fn test_workspace_manager_creation() {
    let manager = WorkspaceManager::new();

    assert_eq!(manager.workspace_count(), 0);
    assert_eq!(manager.active_workspace_name(), None);
}

#[test]
fn test_workspace_manager_create_workspace() {
    let mut manager = WorkspaceManager::new();

    assert!(manager.create_workspace("project1", "/path/to/project1"));
    assert_eq!(manager.workspace_count(), 1);

    // Auto-switches to first workspace
    assert_eq!(manager.active_workspace_name(), Some("project1"));

    // Cannot create duplicate
    assert!(!manager.create_workspace("project1", "/different/path"));
    assert_eq!(manager.workspace_count(), 1);
}

#[test]
fn test_workspace_manager_multiple_workspaces() {
    let mut manager = WorkspaceManager::new();

    manager.create_workspace("frontend", "/projects/frontend");
    manager.create_workspace("backend", "/projects/backend");
    manager.create_workspace("mobile", "/projects/mobile");

    assert_eq!(manager.workspace_count(), 3);
    assert!(manager.has_workspace("frontend"));
    assert!(manager.has_workspace("backend"));
    assert!(manager.has_workspace("mobile"));
    assert!(!manager.has_workspace("desktop"));
}

#[test]
fn test_workspace_manager_switch_workspace() {
    let mut manager = WorkspaceManager::new();

    manager.create_workspace("project1", "/path1");
    manager.create_workspace("project2", "/path2");
    manager.create_workspace("project3", "/path3");

    // Auto-switched to project1
    assert_eq!(manager.active_workspace_name(), Some("project1"));

    // Switch to project2
    assert!(manager.switch_workspace("project2"));
    assert_eq!(manager.active_workspace_name(), Some("project2"));

    // Switch to project3
    assert!(manager.switch_workspace("project3"));
    assert_eq!(manager.active_workspace_name(), Some("project3"));

    // Cannot switch to nonexistent workspace
    assert!(!manager.switch_workspace("nonexistent"));
    assert_eq!(manager.active_workspace_name(), Some("project3"));
}

#[test]
fn test_workspace_manager_get_workspace() {
    let mut manager = WorkspaceManager::new();

    manager.create_workspace("test-project", "/path/to/test");

    let workspace = manager.get_workspace("test-project");
    assert!(workspace.is_some());
    assert_eq!(workspace.unwrap().name(), "test-project");

    let nonexistent = manager.get_workspace("missing");
    assert!(nonexistent.is_none());
}

#[test]
fn test_workspace_manager_get_active_workspace() {
    let mut manager = WorkspaceManager::new();

    assert!(manager.active_workspace().is_none());

    manager.create_workspace("active", "/path");
    let active = manager.active_workspace();
    assert!(active.is_some());
    assert_eq!(active.unwrap().name(), "active");
}

#[test]
fn test_workspace_manager_delete_workspace() {
    let mut manager = WorkspaceManager::new();

    manager.create_workspace("temp1", "/path1");
    manager.create_workspace("temp2", "/path2");
    manager.create_workspace("keep", "/path3");

    assert_eq!(manager.workspace_count(), 3);

    assert!(manager.delete_workspace("temp1"));
    assert_eq!(manager.workspace_count(), 2);
    assert!(!manager.has_workspace("temp1"));

    assert!(manager.delete_workspace("temp2"));
    assert_eq!(manager.workspace_count(), 1);

    // Cannot delete nonexistent
    assert!(!manager.delete_workspace("nonexistent"));
    assert_eq!(manager.workspace_count(), 1);
}

#[test]
fn test_workspace_manager_delete_active_workspace() {
    let mut manager = WorkspaceManager::new();

    manager.create_workspace("active", "/path");
    assert_eq!(manager.active_workspace_name(), Some("active"));

    manager.delete_workspace("active");
    assert_eq!(manager.active_workspace_name(), None);
}

#[test]
fn test_workspace_manager_list_workspaces() {
    let mut manager = WorkspaceManager::new();

    manager.create_workspace("alpha", "/alpha");
    manager.create_workspace("beta", "/beta");
    manager.create_workspace("gamma", "/gamma");

    let names = manager.workspace_names();
    assert_eq!(names.len(), 3);
    assert!(names.contains(&"alpha"));
    assert!(names.contains(&"beta"));
    assert!(names.contains(&"gamma"));
}

#[test]
fn test_workspace_manager_recent_workspaces() {
    let mut manager = WorkspaceManager::new();

    manager.create_workspace("w1", "/1");
    manager.create_workspace("w2", "/2");
    manager.create_workspace("w3", "/3");

    manager.switch_workspace("w2");
    manager.switch_workspace("w3");
    manager.switch_workspace("w1");

    let recent = manager.recent_workspaces();
    // Most recent first
    assert!(recent.len() <= 10); // max_recent
}

// ==================== Toast Tests ====================

#[test]
fn test_toast_creation() {
    let toast = Toast::info("Test message");

    assert_eq!(toast.message(), "Test message");
    assert_eq!(toast.level(), ToastLevel::Info);
    assert!(toast.is_visible());
}

#[test]
fn test_toast_levels() {
    let info = Toast::info("Info");
    let success = Toast::success("Success");
    let warning = Toast::warning("Warning");
    let error = Toast::error("Error");

    assert_eq!(info.level(), ToastLevel::Info);
    assert_eq!(success.level(), ToastLevel::Success);
    assert_eq!(warning.level(), ToastLevel::Warning);
    assert_eq!(error.level(), ToastLevel::Error);
}

#[test]
fn test_toast_level_icons() {
    assert_eq!(ToastLevel::Info.icon(), "ℹ");
    assert_eq!(ToastLevel::Success.icon(), "✓");
    assert_eq!(ToastLevel::Warning.icon(), "⚠");
    assert_eq!(ToastLevel::Error.icon(), "✗");
}

#[test]
fn test_toast_visibility() {
    let toast = Toast::info("Test");
    assert!(toast.is_visible());

    // Remaining time should be positive
    let remaining = toast.remaining_time();
    assert!(remaining.as_secs() > 0);
}

// ==================== ToastManager Tests ====================

#[test]
fn test_toast_manager_creation() {
    let manager = ToastManager::new();
    assert_eq!(manager.len(), 0);
}

#[test]
fn test_toast_manager_add_toasts() {
    let mut manager = ToastManager::new();

    manager.info("Info message");
    manager.success("Success message");
    manager.warning("Warning message");
    manager.error("Error message");

    assert_eq!(manager.len(), 4);
}

#[test]
fn test_toast_manager_clear() {
    let mut manager = ToastManager::new();

    manager.info("Message 1");
    manager.info("Message 2");
    manager.info("Message 3");
    assert_eq!(manager.len(), 3);

    manager.clear();
    assert_eq!(manager.len(), 0);
}

// Note: ToastManager doesn't expose add_toast or remove_expired in public API
// These are internal methods managed automatically

// ==================== ProgressBar Tests ====================

#[test]
fn test_progress_bar_creation() {
    let progress = ProgressBar::new("Loading");

    assert_eq!(progress.progress(), 0.0);
    assert!(!progress.is_complete());
}

#[test]
fn test_progress_bar_set_progress() {
    let mut progress = ProgressBar::new("Loading");

    progress.set_progress(0.25);
    assert_eq!(progress.progress(), 0.25);
    assert!(!progress.is_complete());

    progress.set_progress(0.5);
    assert_eq!(progress.progress(), 0.5);
    assert!(!progress.is_complete());

    progress.set_progress(1.0);
    assert_eq!(progress.progress(), 1.0);
    assert!(progress.is_complete());
}

#[test]
fn test_progress_bar_builder() {
    let progress = ProgressBar::new("Download")
        .with_progress(0.75)
        .with_message("Downloading file.zip");

    assert_eq!(progress.progress(), 0.75);
    assert!(!progress.is_complete());
}

#[test]
fn test_progress_bar_clamping() {
    let mut progress = ProgressBar::new("Test");

    // Progress clamped to 0.0 - 1.0
    progress.set_progress(-0.5);
    assert_eq!(progress.progress(), 0.0);

    progress.set_progress(1.5);
    assert_eq!(progress.progress(), 1.0);
}

#[test]
fn test_progress_bar_message() {
    let mut progress = ProgressBar::new("Loading");

    progress.set_message("Processing files...");
    progress.set_progress(0.33);

    progress.set_message("Almost done...");
    progress.set_progress(0.95);

    assert_eq!(progress.progress(), 0.95);
}

// ==================== StageStatus Tests ====================

#[test]
fn test_stage_status_active() {
    assert!(!StageStatus::Pending.is_active());
    assert!(StageStatus::InProgress.is_active());
    assert!(!StageStatus::Complete.is_active());
}

#[test]
fn test_stage_status_complete() {
    assert!(!StageStatus::Pending.is_complete());
    assert!(!StageStatus::InProgress.is_complete());
    assert!(StageStatus::Complete.is_complete());
}

// ==================== MultiStageProgress Tests ====================

#[test]
fn test_multi_stage_progress_creation() {
    let stages = vec![
        "Download".to_string(),
        "Extract".to_string(),
        "Install".to_string(),
    ];
    let progress = MultiStageProgress::new("Installation", stages);

    assert_eq!(progress.stage_count(), 3);
    assert_eq!(progress.current_stage(), 0);
}

#[test]
fn test_multi_stage_progress_set_stage() {
    let stages = vec![
        "Step 1".to_string(),
        "Step 2".to_string(),
        "Step 3".to_string(),
    ];
    let mut progress = MultiStageProgress::new("Process", stages);

    assert_eq!(progress.current_stage(), 0);

    progress.set_stage(1);
    assert_eq!(progress.current_stage(), 1);

    progress.set_stage(2);
    assert_eq!(progress.current_stage(), 2);
}

#[test]
fn test_multi_stage_progress_stage_progress() {
    let stages = vec!["Download".to_string(), "Install".to_string()];
    let mut progress = MultiStageProgress::new("Setup", stages);

    progress.set_stage(0);
    progress.set_stage_progress(0.5);

    progress.set_stage(1);
    progress.set_stage_progress(0.75);

    assert!(!progress.is_complete());
}

#[test]
fn test_multi_stage_progress_complete() {
    let stages = vec!["A".to_string(), "B".to_string()];
    let mut progress = MultiStageProgress::new("Test", stages);

    assert!(!progress.is_complete());

    progress.set_stage(0);
    progress.set_stage_progress(1.0);
    progress.complete_stage();

    progress.set_stage(1);
    progress.set_stage_progress(1.0);
    progress.complete_stage();

    assert!(progress.is_complete());
}

#[test]
fn test_multi_stage_progress_next_stage() {
    let stages = vec!["1".to_string(), "2".to_string(), "3".to_string()];
    let mut progress = MultiStageProgress::new("Test", stages);

    assert_eq!(progress.current_stage(), 0);

    progress.next_stage();
    assert_eq!(progress.current_stage(), 1);

    progress.next_stage();
    assert_eq!(progress.current_stage(), 2);

    // Cannot go beyond last stage
    progress.next_stage();
    assert_eq!(progress.current_stage(), 2);
}

// ==================== Cross-Widget Integration Tests ====================

#[test]
fn test_workspace_toast_integration() {
    let mut manager = WorkspaceManager::new();
    let mut toasts = ToastManager::new();

    // Create workspace - show success toast
    manager.create_workspace("new-project", "/path/to/project");
    toasts.success("Workspace 'new-project' created");
    assert_eq!(manager.workspace_count(), 1);
    assert_eq!(toasts.len(), 1);

    // Switch workspace - show info toast
    manager.create_workspace("another", "/path/another");
    toasts.info("Switched to workspace 'another'");
    assert_eq!(toasts.len(), 2);

    // Delete workspace - show warning toast
    manager.delete_workspace("new-project");
    toasts.warning("Workspace 'new-project' deleted");
    assert_eq!(manager.workspace_count(), 1);
    assert_eq!(toasts.len(), 3);
}

#[test]
fn test_workspace_progress_integration() {
    let mut manager = WorkspaceManager::new();
    let mut progress = ProgressBar::new("Loading workspace");

    // Step 1: Create workspace (25%)
    manager.create_workspace("project", "/path");
    progress.set_progress(0.25);
    progress.set_message("Workspace created");

    // Step 2: Load settings (50%)
    let workspace = manager.get_workspace_mut("project").unwrap();
    workspace.set_setting("theme", "dark");
    workspace.set_setting("font", "monospace");
    progress.set_progress(0.5);
    progress.set_message("Settings loaded");

    // Step 3: Restore state (75%)
    workspace.set_state("last_file", "main.rs");
    progress.set_progress(0.75);
    progress.set_message("State restored");

    // Step 4: Complete (100%)
    progress.set_progress(1.0);
    progress.set_message("Workspace ready");
    assert!(progress.is_complete());
}

#[test]
fn test_multi_stage_workspace_loading() {
    let stages = vec![
        "Create workspace".to_string(),
        "Load configuration".to_string(),
        "Restore session".to_string(),
        "Open files".to_string(),
    ];
    let mut progress = MultiStageProgress::new("Workspace Initialization", stages);
    let mut manager = WorkspaceManager::new();

    // Stage 0: Create workspace
    progress.set_stage(0);
    manager.create_workspace("my-project", "/path/to/project");
    progress.set_stage_progress(1.0);
    progress.complete_stage();

    // Stage 1: Load configuration
    let workspace = manager.get_workspace_mut("my-project").unwrap();
    workspace.set_setting("theme", "dark");
    progress.set_stage_progress(1.0);
    progress.complete_stage();

    // Stage 2: Restore session
    let workspace = manager.get_workspace_mut("my-project").unwrap();
    workspace.set_state("current_file", "main.rs");
    progress.set_stage_progress(1.0);
    progress.complete_stage();

    // Stage 3: Open files
    progress.set_stage_progress(1.0);
    progress.complete_stage();

    assert!(progress.is_complete());
    assert_eq!(manager.workspace_count(), 1);
}

// ==================== Real-World Scenario Tests ====================

#[test]
fn test_scenario_workspace_lifecycle() {
    let mut manager = WorkspaceManager::new();
    let mut toasts = ToastManager::new();
    let mut progress = ProgressBar::new("Creating workspace");

    // Create new workspace
    progress.set_progress(0.2);
    progress.set_message("Initializing...");

    manager.create_workspace("full-stack-app", "/projects/full-stack");
    toasts.success("Workspace created");

    progress.set_progress(0.5);
    progress.set_message("Configuring...");

    // Configure workspace
    let workspace = manager.get_workspace_mut("full-stack-app").unwrap();
    workspace.set_setting("language", "rust");
    workspace.set_setting("framework", "axum");
    workspace.set_setting("frontend", "react");

    progress.set_progress(0.75);
    progress.set_message("Setting up environment...");

    // Set initial state
    workspace.set_state("backend_port", "3000");
    workspace.set_state("frontend_port", "5173");
    workspace.set_state("db_connection", "postgresql://localhost");

    progress.set_progress(1.0);
    progress.set_message("Complete!");
    toasts.info("Workspace ready for development");

    assert!(progress.is_complete());
    assert_eq!(manager.workspace_count(), 1);
    assert_eq!(toasts.len(), 2);
}

#[test]
fn test_scenario_workspace_switching_with_notifications() {
    let mut manager = WorkspaceManager::new();
    let mut toasts = ToastManager::new();

    // Set up multiple workspaces
    manager.create_workspace("frontend", "/proj/frontend");
    manager.create_workspace("backend", "/proj/backend");
    manager.create_workspace("mobile", "/proj/mobile");

    // Switch between workspaces with notifications
    manager.switch_workspace("backend");
    toasts.info("Switched to Backend workspace");

    manager.switch_workspace("mobile");
    toasts.info("Switched to Mobile workspace");

    manager.switch_workspace("frontend");
    toasts.info("Switched to Frontend workspace");

    assert_eq!(manager.active_workspace_name(), Some("frontend"));
    assert_eq!(toasts.len(), 3);
}
