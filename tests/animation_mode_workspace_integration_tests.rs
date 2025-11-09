//! Integration tests for Animation, ModeIndicator, MultiStageProgress, and WorkspaceManager
//!
//! Tests PLATINUM tier features for visual polish, modal editing, and workspace management.

use std::path::PathBuf;
use std::time::Duration;
use toad::ui::widgets::{
    Animation, AnimationState, EditorMode, EasingFunction, IndicatorStyle, ModeIndicator,
    MultiStageProgress, ProgressBar, StageStatus,
};
use toad::workspace::{Workspace, WorkspaceManager};

// ============================================================================
// Animation Integration Tests
// ============================================================================

#[test]
fn test_animation_creation() {
    let anim = Animation::new(0.0, 100.0, Duration::from_secs(1));
    assert_eq!(anim.start_value(), 0.0);
    assert_eq!(anim.end_value(), 100.0);
    assert_eq!(anim.state(), AnimationState::Idle);
}

#[test]
fn test_animation_with_easing() {
    let anim = Animation::new(0.0, 100.0, Duration::from_secs(1))
        .with_easing(EasingFunction::EaseInOut);
    assert_eq!(anim.current_value(), 0.0);
}

#[test]
fn test_animation_lifecycle() {
    let mut anim = Animation::new(0.0, 100.0, Duration::from_secs(1));

    // Initial state
    assert_eq!(anim.state(), AnimationState::Idle);
    assert_eq!(anim.current_value(), 0.0);

    // Start animation
    anim.start();
    assert_eq!(anim.state(), AnimationState::Running);

    // Tick halfway
    anim.tick(Duration::from_millis(500));
    let mid_value = anim.current_value();
    assert!(mid_value > 0.0 && mid_value < 100.0);
    assert!((anim.progress() - 0.5).abs() < 0.01);

    // Complete animation
    anim.tick(Duration::from_millis(500));
    assert_eq!(anim.state(), AnimationState::Complete);
    assert_eq!(anim.current_value(), 100.0);
    assert!(anim.is_complete());
}

#[test]
fn test_animation_pause_resume() {
    let mut anim = Animation::new(0.0, 100.0, Duration::from_secs(1));

    anim.start();
    anim.tick(Duration::from_millis(250));
    let paused_value = anim.current_value();

    // Pause
    anim.pause();
    assert_eq!(anim.state(), AnimationState::Idle);

    // Tick while paused should not change value
    anim.tick(Duration::from_millis(250));
    assert_eq!(anim.current_value(), paused_value);
}

#[test]
fn test_animation_reset() {
    let mut anim = Animation::new(0.0, 100.0, Duration::from_secs(1));

    anim.start();
    anim.tick(Duration::from_millis(500));
    assert!(anim.current_value() > 0.0);

    anim.reset();
    assert_eq!(anim.state(), AnimationState::Idle);
    assert_eq!(anim.current_value(), 0.0);
}

#[test]
fn test_animation_loop() {
    let mut anim = Animation::new(0.0, 100.0, Duration::from_millis(100)).with_loop(true);

    anim.start();
    anim.tick(Duration::from_millis(150));

    // Should loop, not complete
    assert!(!anim.is_complete());
    assert_eq!(anim.state(), AnimationState::Running);
}

#[test]
fn test_animation_reverse_loop() {
    let mut anim = Animation::new(0.0, 100.0, Duration::from_millis(100))
        .with_loop(true)
        .with_reverse(true);

    let initial_start = anim.start_value();
    let initial_end = anim.end_value();

    anim.start();
    anim.tick(Duration::from_millis(150)); // Complete one loop

    // Values should be swapped after reverse
    assert_eq!(anim.start_value(), initial_end);
    assert_eq!(anim.end_value(), initial_start);
}

#[test]
fn test_easing_functions() {
    // Linear
    let linear = EasingFunction::Linear;
    assert_eq!(linear.apply(0.0), 0.0);
    assert_eq!(linear.apply(0.5), 0.5);
    assert_eq!(linear.apply(1.0), 1.0);

    // EaseIn (should be slower at start)
    let ease_in = EasingFunction::EaseIn;
    assert_eq!(ease_in.apply(0.0), 0.0);
    assert!(ease_in.apply(0.5) < 0.5);
    assert_eq!(ease_in.apply(1.0), 1.0);

    // EaseOut (should be slower at end)
    let ease_out = EasingFunction::EaseOut;
    assert_eq!(ease_out.apply(0.0), 0.0);
    assert!(ease_out.apply(0.5) > 0.5);
    assert_eq!(ease_out.apply(1.0), 1.0);
}

#[test]
fn test_animation_builder_pattern() {
    let anim = Animation::new(0.0, 100.0, Duration::from_secs(1))
        .with_easing(EasingFunction::EaseInCubic)
        .with_loop(true)
        .with_reverse(true);

    assert_eq!(anim.start_value(), 0.0);
    assert_eq!(anim.end_value(), 100.0);
}

#[test]
fn test_animation_multiple_easing_types() {
    let easings = vec![
        EasingFunction::Linear,
        EasingFunction::EaseIn,
        EasingFunction::EaseOut,
        EasingFunction::EaseInOut,
        EasingFunction::EaseInCubic,
        EasingFunction::EaseOutCubic,
    ];

    for easing in easings {
        let mut anim =
            Animation::new(0.0, 100.0, Duration::from_millis(100)).with_easing(easing);
        anim.start();
        anim.tick(Duration::from_millis(50));

        let value = anim.current_value();
        assert!(value >= 0.0 && value <= 100.0);
    }
}

// ============================================================================
// ModeIndicator Integration Tests
// ============================================================================

#[test]
fn test_mode_indicator_creation() {
    let indicator = ModeIndicator::new(EditorMode::Normal);
    assert_eq!(indicator.mode(), EditorMode::Normal);
}

#[test]
fn test_mode_indicator_set_mode() {
    let mut indicator = ModeIndicator::new(EditorMode::Normal);
    assert_eq!(indicator.mode(), EditorMode::Normal);

    indicator.set_mode(EditorMode::Insert);
    assert_eq!(indicator.mode(), EditorMode::Insert);

    indicator.set_mode(EditorMode::Visual);
    assert_eq!(indicator.mode(), EditorMode::Visual);
}

#[test]
fn test_mode_indicator_with_style() {
    let full = ModeIndicator::new(EditorMode::Normal).with_style(IndicatorStyle::Full);
    assert_eq!(full.to_string(), "NORMAL");

    let short = ModeIndicator::new(EditorMode::Normal).with_style(IndicatorStyle::Short);
    assert_eq!(short.to_string(), "N");

    let block = ModeIndicator::new(EditorMode::Normal).with_style(IndicatorStyle::Block);
    assert_eq!(block.to_string(), "NORMAL");
}

#[test]
fn test_mode_indicator_with_border() {
    let indicator = ModeIndicator::new(EditorMode::Normal).with_border(true);
    // Border is internal state, can't directly test, but ensures method works
    assert_eq!(indicator.mode(), EditorMode::Normal);
}

#[test]
fn test_editor_mode_names() {
    assert_eq!(EditorMode::Normal.name(), "NORMAL");
    assert_eq!(EditorMode::Insert.name(), "INSERT");
    assert_eq!(EditorMode::Visual.name(), "VISUAL");
    assert_eq!(EditorMode::VisualLine.name(), "VISUAL LINE");
    assert_eq!(EditorMode::VisualBlock.name(), "VISUAL BLOCK");
    assert_eq!(EditorMode::Command.name(), "COMMAND");
    assert_eq!(EditorMode::Replace.name(), "REPLACE");
}

#[test]
fn test_editor_mode_short_names() {
    assert_eq!(EditorMode::Normal.short_name(), "N");
    assert_eq!(EditorMode::Insert.short_name(), "I");
    assert_eq!(EditorMode::Visual.short_name(), "V");
    assert_eq!(EditorMode::VisualLine.short_name(), "VL");
    assert_eq!(EditorMode::VisualBlock.short_name(), "VB");
    assert_eq!(EditorMode::Command.short_name(), "C");
    assert_eq!(EditorMode::Replace.short_name(), "R");
}

#[test]
fn test_editor_mode_all() {
    let modes = EditorMode::all();
    assert_eq!(modes.len(), 7);
    assert!(modes.contains(&EditorMode::Normal));
    assert!(modes.contains(&EditorMode::Insert));
    assert!(modes.contains(&EditorMode::Visual));
}

#[test]
fn test_mode_indicator_all_modes() {
    for mode in EditorMode::all() {
        let indicator = ModeIndicator::new(*mode);
        assert_eq!(indicator.mode(), *mode);
        assert!(!indicator.to_string().is_empty());
    }
}

#[test]
fn test_mode_indicator_style_switching() {
    let mut indicator = ModeIndicator::new(EditorMode::Insert);

    indicator = indicator.with_style(IndicatorStyle::Full);
    assert_eq!(indicator.to_string(), "INSERT");

    indicator = indicator.with_style(IndicatorStyle::Short);
    assert_eq!(indicator.to_string(), "I");
}

#[test]
fn test_mode_indicator_default() {
    let indicator = ModeIndicator::default();
    assert_eq!(indicator.mode(), EditorMode::Normal);
}

// ============================================================================
// MultiStageProgress Integration Tests
// ============================================================================

#[test]
fn test_multi_stage_progress_creation() {
    let stages = vec!["Download".to_string(), "Extract".to_string()];
    let progress = MultiStageProgress::new("Installation", stages);
    assert_eq!(progress.stage_count(), 2);
    assert_eq!(progress.current_stage(), 0);
}

#[test]
fn test_multi_stage_progress_set_stage() {
    let stages = vec![
        "Build".to_string(),
        "Test".to_string(),
        "Deploy".to_string(),
    ];
    let mut progress = MultiStageProgress::new("CI/CD", stages);

    progress.set_stage(0);
    assert_eq!(progress.current_stage(), 0);

    progress.set_stage(1);
    assert_eq!(progress.current_stage(), 1);

    progress.set_stage(2);
    assert_eq!(progress.current_stage(), 2);
}

#[test]
fn test_multi_stage_progress_stage_progress() {
    let stages = vec!["Stage 1".to_string()];
    let mut progress = MultiStageProgress::new("Task", stages);

    progress.set_stage(0);
    progress.set_stage_progress(0.5);

    // With 1 stage, 50% progress in first stage = 50% overall
    assert_eq!(progress.overall_progress(), 0.5);
}

#[test]
fn test_multi_stage_progress_complete_stage() {
    let stages = vec!["A".to_string(), "B".to_string(), "C".to_string()];
    let mut progress = MultiStageProgress::new("Task", stages);

    progress.set_stage(0);
    assert_eq!(progress.completed_stages(), 0);

    progress.complete_stage();
    assert_eq!(progress.completed_stages(), 1);
    assert_eq!(progress.current_stage(), 1);
}

#[test]
fn test_multi_stage_progress_next_stage() {
    let stages = vec!["A".to_string(), "B".to_string()];
    let mut progress = MultiStageProgress::new("Task", stages);

    progress.set_stage(0);
    progress.next_stage();
    assert_eq!(progress.current_stage(), 1);
}

#[test]
fn test_multi_stage_progress_overall_progress() {
    let stages = vec!["A".to_string(), "B".to_string()];
    let mut progress = MultiStageProgress::new("Task", stages);

    // Complete first stage
    progress.set_stage(0);
    progress.set_stage_progress(1.0);
    progress.complete_stage();

    // Should be at 50% (1 of 2 stages complete)
    assert!((progress.overall_progress() - 0.5).abs() < 0.01);

    // Set second stage to 50%
    progress.set_stage_progress(0.5);

    // Should be at 75% (1 complete + 0.5 * 0.5)
    assert!((progress.overall_progress() - 0.75).abs() < 0.01);
}

#[test]
fn test_multi_stage_progress_is_complete() {
    let stages = vec!["A".to_string(), "B".to_string()];
    let mut progress = MultiStageProgress::new("Task", stages);

    assert!(!progress.is_complete());

    progress.set_stage(0);
    progress.complete_stage();
    assert!(!progress.is_complete());

    progress.complete_stage();
    assert!(progress.is_complete());
}

#[test]
fn test_multi_stage_progress_with_time_tracking() {
    let stages = vec!["Stage 1".to_string()];
    let progress = MultiStageProgress::new("Task", stages).with_time_tracking(true);
    assert_eq!(progress.stage_count(), 1);
}

#[test]
fn test_multi_stage_progress_stage_elapsed() {
    let stages = vec!["A".to_string(), "B".to_string()];
    let mut progress = MultiStageProgress::new("Task", stages);

    progress.set_stage(0);
    std::thread::sleep(Duration::from_millis(10));

    let elapsed = progress.stage_elapsed(0);
    assert!(elapsed.is_some());
    assert!(elapsed.unwrap() >= Duration::from_millis(10));

    // Stage 1 not started
    assert!(progress.stage_elapsed(1).is_none());
}

#[test]
fn test_multi_stage_progress_completed_stages() {
    let stages = vec!["A".to_string(), "B".to_string(), "C".to_string()];
    let mut progress = MultiStageProgress::new("Task", stages);

    assert_eq!(progress.completed_stages(), 0);

    progress.set_stage(0);
    progress.complete_stage();
    assert_eq!(progress.completed_stages(), 1);

    progress.complete_stage();
    assert_eq!(progress.completed_stages(), 2);

    progress.complete_stage();
    assert_eq!(progress.completed_stages(), 3);
}

#[test]
fn test_stage_status() {
    assert!(StageStatus::InProgress.is_active());
    assert!(!StageStatus::Pending.is_active());
    assert!(!StageStatus::Complete.is_active());

    assert!(StageStatus::Complete.is_complete());
    assert!(!StageStatus::Pending.is_complete());
    assert!(!StageStatus::InProgress.is_complete());
}

// ============================================================================
// ProgressBar Integration Tests
// ============================================================================

#[test]
fn test_progress_bar_creation() {
    let progress = ProgressBar::new("Loading");
    assert_eq!(progress.progress(), 0.0);
    assert!(!progress.is_complete());
}

#[test]
fn test_progress_bar_with_progress() {
    let progress = ProgressBar::new("Loading").with_progress(0.75);
    assert_eq!(progress.progress(), 0.75);
    assert!(!progress.is_complete());
}

#[test]
fn test_progress_bar_with_message() {
    let progress = ProgressBar::new("Loading").with_message("Processing files...");
    assert_eq!(progress.progress(), 0.0);
}

#[test]
fn test_progress_bar_set_progress() {
    let mut progress = ProgressBar::new("Loading");
    progress.set_progress(0.5);
    assert_eq!(progress.progress(), 0.5);

    progress.set_progress(1.0);
    assert_eq!(progress.progress(), 1.0);
    assert!(progress.is_complete());
}

#[test]
fn test_progress_bar_set_message() {
    let mut progress = ProgressBar::new("Loading");
    progress.set_message("Step 1");
    // Message is internal, just ensure it doesn't panic
    assert_eq!(progress.progress(), 0.0);
}

#[test]
fn test_progress_bar_completion() {
    let incomplete = ProgressBar::new("Loading").with_progress(0.99);
    assert!(!incomplete.is_complete());

    let complete = ProgressBar::new("Loading").with_progress(1.0);
    assert!(complete.is_complete());
}

#[test]
fn test_progress_bar_clamping() {
    let over = ProgressBar::new("Test").with_progress(1.5);
    assert_eq!(over.progress(), 1.0);

    let under = ProgressBar::new("Test").with_progress(-0.5);
    assert_eq!(under.progress(), 0.0);
}

// ============================================================================
// WorkspaceManager Integration Tests
// ============================================================================

#[test]
fn test_workspace_creation() {
    let ws = Workspace::new("proj1", "Project 1", PathBuf::from("/path/to/proj1"));
    assert_eq!(ws.id, "proj1");
    assert_eq!(ws.name, "Project 1");
    assert_eq!(ws.root_path, PathBuf::from("/path/to/proj1"));
}

#[test]
fn test_workspace_add_recent_file() {
    let mut ws = Workspace::new("proj", "Project", PathBuf::from("/path"));

    ws.add_recent_file(PathBuf::from("/path/file1.rs"));
    assert_eq!(ws.recent_files.len(), 1);

    ws.add_recent_file(PathBuf::from("/path/file2.rs"));
    assert_eq!(ws.recent_files.len(), 2);

    // Adding same file again should move to front
    ws.add_recent_file(PathBuf::from("/path/file1.rs"));
    assert_eq!(ws.recent_files.len(), 2);
    assert_eq!(ws.recent_files[0], PathBuf::from("/path/file1.rs"));
}

#[test]
fn test_workspace_recent_files_limit() {
    let mut ws = Workspace::new("proj", "Project", PathBuf::from("/path"));

    // Add 25 files (limit is 20)
    for i in 0..25 {
        ws.add_recent_file(PathBuf::from(format!("/path/file{}.rs", i)));
    }

    assert_eq!(ws.recent_files.len(), 20);
    assert_eq!(ws.recent_files[0], PathBuf::from("/path/file24.rs"));
}

#[test]
fn test_workspace_settings() {
    let mut ws = Workspace::new("proj", "Project", PathBuf::from("/path"));

    ws.set_setting("theme", "dark");
    ws.set_setting("font_size", "14");

    assert_eq!(ws.get_setting("theme"), Some(&"dark".to_string()));
    assert_eq!(ws.get_setting("font_size"), Some(&"14".to_string()));
    assert_eq!(ws.get_setting("nonexistent"), None);
}

#[test]
fn test_workspace_touch() {
    let mut ws = Workspace::new("proj", "Project", PathBuf::from("/path"));
    let initial = ws.last_accessed;

    std::thread::sleep(Duration::from_secs(1));
    ws.touch();

    assert!(ws.last_accessed > initial);
}

#[test]
fn test_workspace_contains_path() {
    let ws = Workspace::new("proj", "Project", PathBuf::from("/path/to/project"));

    assert!(ws.contains_path(&PathBuf::from("/path/to/project/src/main.rs")));
    assert!(ws.contains_path(&PathBuf::from("/path/to/project")));
    assert!(!ws.contains_path(&PathBuf::from("/other/path")));
}

#[test]
fn test_workspace_manager_creation() {
    let manager = WorkspaceManager::new();
    assert_eq!(manager.count(), 0);
}

#[test]
fn test_workspace_manager_create_workspace() {
    let mut manager = WorkspaceManager::new();

    manager.create_workspace("proj1", "Project 1", "/path/to/proj1");
    assert_eq!(manager.count(), 1);

    manager.create_workspace("proj2", "Project 2", "/path/to/proj2");
    assert_eq!(manager.count(), 2);
}

#[test]
fn test_workspace_manager_get_workspace() {
    let mut manager = WorkspaceManager::new();
    manager.create_workspace("proj1", "Project 1", "/path/to/proj1");

    let ws = manager.get_workspace("proj1");
    assert!(ws.is_some());
    assert_eq!(ws.unwrap().id, "proj1");

    let nonexistent = manager.get_workspace("nonexistent");
    assert!(nonexistent.is_none());
}

#[test]
fn test_workspace_manager_set_active() {
    let mut manager = WorkspaceManager::new();
    manager.create_workspace("proj1", "Project 1", "/path/to/proj1");
    manager.create_workspace("proj2", "Project 2", "/path/to/proj2");

    assert!(manager.set_active("proj1"));
    assert_eq!(manager.active_workspace().unwrap().id, "proj1");

    assert!(manager.set_active("proj2"));
    assert_eq!(manager.active_workspace().unwrap().id, "proj2");

    assert!(!manager.set_active("nonexistent"));
}

#[test]
fn test_workspace_manager_next_previous() {
    let mut manager = WorkspaceManager::new();
    manager.create_workspace("a", "A", "/a");
    manager.create_workspace("b", "B", "/b");
    manager.create_workspace("c", "C", "/c");

    manager.set_active("a");

    let next = manager.next_workspace();
    assert_eq!(next.unwrap().id, "b");

    let next = manager.next_workspace();
    assert_eq!(next.unwrap().id, "c");

    let next = manager.next_workspace();
    assert_eq!(next.unwrap().id, "a"); // Wraps around

    let prev = manager.previous_workspace();
    assert_eq!(prev.unwrap().id, "c");
}

#[test]
fn test_workspace_manager_remove_workspace() {
    let mut manager = WorkspaceManager::new();
    manager.create_workspace("proj1", "Project 1", "/path/to/proj1");
    manager.create_workspace("proj2", "Project 2", "/path/to/proj2");

    assert_eq!(manager.count(), 2);

    manager.remove_workspace("proj1");
    assert_eq!(manager.count(), 1);
    assert!(manager.get_workspace("proj1").is_none());
}

#[test]
fn test_workspace_manager_remove_active() {
    let mut manager = WorkspaceManager::new();
    manager.create_workspace("proj1", "Project 1", "/path/to/proj1");
    manager.set_active("proj1");

    manager.remove_workspace("proj1");
    assert!(manager.active_workspace().is_none());
}

#[test]
fn test_workspace_manager_all_workspaces() {
    let mut manager = WorkspaceManager::new();
    manager.create_workspace("a", "A", "/a");
    manager.create_workspace("b", "B", "/b");

    let all = manager.all_workspaces();
    assert_eq!(all.len(), 2);
}

#[test]
fn test_workspace_manager_recent_workspaces() {
    let mut manager = WorkspaceManager::new();
    manager.create_workspace("a", "A", "/a");
    std::thread::sleep(Duration::from_secs(1));
    manager.create_workspace("b", "B", "/b");

    std::thread::sleep(Duration::from_secs(1));
    manager.set_active("a"); // Touch 'a' to make it most recent

    let recent = manager.recent_workspaces();
    assert_eq!(recent[0].id, "a"); // Most recent first
}

#[test]
fn test_workspace_manager_find_by_path() {
    let mut manager = WorkspaceManager::new();
    manager.create_workspace("proj1", "Project 1", "/path/to/proj1");
    manager.create_workspace("proj2", "Project 2", "/path/to/proj2");

    let found = manager.find_by_path(&PathBuf::from("/path/to/proj1/src/main.rs"));
    assert!(found.is_some());
    assert_eq!(found.unwrap().id, "proj1");

    let not_found = manager.find_by_path(&PathBuf::from("/other/path"));
    assert!(not_found.is_none());
}

#[test]
fn test_workspace_manager_clear() {
    let mut manager = WorkspaceManager::new();
    manager.create_workspace("a", "A", "/a");
    manager.create_workspace("b", "B", "/b");
    manager.set_active("a");

    assert_eq!(manager.count(), 2);

    manager.clear();
    assert_eq!(manager.count(), 0);
    assert!(manager.active_workspace().is_none());
}

// ============================================================================
// Cross-Feature Integration Tests
// ============================================================================

#[test]
fn test_animation_with_progress_bar() {
    let mut anim = Animation::new(0.0, 1.0, Duration::from_secs(1));
    let mut progress = ProgressBar::new("Animated Loading");

    anim.start();

    // Simulate several animation frames
    for _ in 0..10 {
        anim.tick(Duration::from_millis(100));
        progress.set_progress(anim.current_value());
    }

    assert!(progress.is_complete());
}

#[test]
fn test_multi_stage_with_mode_indicator() {
    let stages = vec![
        "Normal Mode".to_string(),
        "Insert Mode".to_string(),
        "Visual Mode".to_string(),
    ];
    let mut progress = MultiStageProgress::new("Editor Setup", stages);
    let mut mode = ModeIndicator::new(EditorMode::Normal);

    // Stage 0: Normal mode
    progress.set_stage(0);
    mode.set_mode(EditorMode::Normal);
    assert_eq!(progress.current_stage(), 0);
    assert_eq!(mode.mode(), EditorMode::Normal);

    // Stage 1: Insert mode
    progress.complete_stage();
    mode.set_mode(EditorMode::Insert);
    assert_eq!(progress.current_stage(), 1);
    assert_eq!(mode.mode(), EditorMode::Insert);

    // Stage 2: Visual mode
    progress.complete_stage();
    mode.set_mode(EditorMode::Visual);
    assert_eq!(progress.current_stage(), 2);
    assert_eq!(mode.mode(), EditorMode::Visual);
}

#[test]
fn test_workspace_with_settings_per_mode() {
    let mut ws = Workspace::new("proj", "Project", PathBuf::from("/path"));

    // Store settings for different editor modes
    ws.set_setting("normal_color", "green");
    ws.set_setting("insert_color", "blue");
    ws.set_setting("visual_color", "yellow");

    let _normal = ModeIndicator::new(EditorMode::Normal);
    assert_eq!(ws.get_setting("normal_color"), Some(&"green".to_string()));

    let _insert = ModeIndicator::new(EditorMode::Insert);
    assert_eq!(ws.get_setting("insert_color"), Some(&"blue".to_string()));

    let _visual = ModeIndicator::new(EditorMode::Visual);
    assert_eq!(ws.get_setting("visual_color"), Some(&"yellow".to_string()));
}

#[test]
fn test_complete_workflow_with_all_features() {
    // Setup workspace manager
    let mut manager = WorkspaceManager::new();
    manager.create_workspace("myapp", "My App", "/path/to/myapp");
    manager.set_active("myapp");

    // Configure workspace settings
    if let Some(ws) = manager.get_workspace_mut("myapp") {
        ws.set_setting("default_mode", "Normal");
        ws.add_recent_file(PathBuf::from("/path/to/myapp/src/main.rs"));
    }

    // Setup mode indicator
    let mut mode = ModeIndicator::new(EditorMode::Normal)
        .with_style(IndicatorStyle::Full)
        .with_border(true);

    // Setup multi-stage progress
    let stages = vec![
        "Load Workspace".to_string(),
        "Initialize Editor".to_string(),
        "Ready".to_string(),
    ];
    let mut progress = MultiStageProgress::new("Startup", stages).with_time_tracking(true);

    // Simulate startup workflow
    // Stage 0: Load Workspace
    progress.set_stage(0);
    progress.set_stage_progress(1.0);
    progress.complete_stage();

    // Stage 1: Initialize Editor
    mode.set_mode(EditorMode::Insert);
    progress.set_stage_progress(1.0);
    progress.complete_stage();

    // Stage 2: Ready - complete the final stage
    progress.set_stage_progress(1.0);
    progress.complete_stage();

    // Setup animation for smooth transitions
    let mut anim = Animation::new(0.0, 100.0, Duration::from_millis(300))
        .with_easing(EasingFunction::EaseInOut);
    anim.start();
    anim.tick(Duration::from_millis(300));

    // Verify complete workflow
    assert!(progress.is_complete());
    assert_eq!(mode.mode(), EditorMode::Insert);
    assert_eq!(manager.active_workspace().unwrap().id, "myapp");
    assert!(anim.is_complete());
}
