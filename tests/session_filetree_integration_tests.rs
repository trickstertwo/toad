//! Integration tests for SessionManager and FileTree widgets
//!
//! Tests session persistence, state management, and file tree navigation.

use std::path::PathBuf;
use toad::ui::widgets::session_manager::{SessionManager};

// ==================== SessionData Tests ====================

#[test]
fn test_session_data_creation() {
    let session = SessionData::new("my-session");

    assert_eq!(session.name(), "my-session");
    assert!(session.is_valid());
    assert_eq!(session.data().len(), 0);
    assert_eq!(session.metadata().len(), 0);
}

#[test]
fn test_session_data_set_get() {
    let mut session = SessionData::new("test");

    session.set_data("key1", "value1");
    session.set_data("key2", "value2");

    assert_eq!(session.get_data("key1"), Some("value1"));
    assert_eq!(session.get_data("key2"), Some("value2"));
    assert_eq!(session.get_data("nonexistent"), None);
}

#[test]
fn test_session_data_remove() {
    let mut session = SessionData::new("test");

    session.set_data("key", "value");
    assert_eq!(session.get_data("key"), Some("value"));

    let removed = session.remove_data("key");
    assert_eq!(removed, Some("value".to_string()));
    assert_eq!(session.get_data("key"), None);

    // Removing again returns None
    assert_eq!(session.remove_data("key"), None);
}

#[test]
fn test_session_data_metadata() {
    let mut session = SessionData::new("test");

    session.set_metadata("project", "toad");
    session.set_metadata("branch", "main");

    assert_eq!(session.get_metadata("project"), Some("toad"));
    assert_eq!(session.get_metadata("branch"), Some("main"));
    assert_eq!(session.get_metadata("nonexistent"), None);
}

#[test]
fn test_session_data_timestamps() {
    let session1 = SessionData::new("test1");
    let created1 = session1.created_at();
    let updated1 = session1.updated_at();

    // Created and updated should be equal initially
    assert_eq!(created1, updated1);

    // Updated timestamp changes when data is modified
    let mut session2 = SessionData::new("test2");
    let created2 = session2.created_at();

    session2.set_data("key", "value");
    let updated2 = session2.updated_at();

    assert_eq!(created2, session2.created_at()); // Created doesn't change
    assert!(updated2 >= created2); // Updated is >= created
}

#[test]
fn test_session_data_validation() {
    let valid_session = SessionData::new("valid");
    assert!(valid_session.is_valid());

    // Empty name would be invalid, but we can't create one through the public API
    // since new() takes impl Into<String>
}

// ==================== SessionManager Tests ====================

#[test]
fn test_session_manager_creation() {
    let manager = SessionManager::new();

    assert_eq!(manager.session_count(), 0);
    assert_eq!(manager.active_session(), None);
    assert!(!manager.auto_save());
}

#[test]
fn test_session_manager_data_operations() {
    let mut manager = SessionManager::new();

    manager.set_data("key1", "value1");
    manager.set_data("key2", "value2");

    assert_eq!(manager.get_data("key1"), Some("value1"));
    assert_eq!(manager.get_data("key2"), Some("value2"));

    manager.remove_data("key1");
    assert_eq!(manager.get_data("key1"), None);
    assert_eq!(manager.get_data("key2"), Some("value2"));

    manager.clear_data();
    assert_eq!(manager.get_data("key2"), None);
}

#[test]
fn test_session_manager_metadata() {
    let mut manager = SessionManager::new();

    manager.set_metadata("workspace", "main");
    manager.set_metadata("layout", "split");

    assert_eq!(manager.get_metadata("workspace"), Some("main"));
    assert_eq!(manager.get_metadata("layout"), Some("split"));
}

#[test]
fn test_session_manager_save_load() {
    let mut manager = SessionManager::new();

    // Set up some data
    manager.set_data("file", "main.rs");
    manager.set_data("line", "42");
    manager.set_metadata("theme", "dark");

    // Save session
    let session = manager.save_session("session1");
    assert!(session.is_some());
    assert_eq!(manager.session_count(), 1);
    assert_eq!(manager.active_session(), Some("session1"));

    // Clear current data
    manager.clear_data();
    assert_eq!(manager.get_data("file"), None);

    // Load session
    assert!(manager.load_session("session1"));
    assert_eq!(manager.get_data("file"), Some("main.rs"));
    assert_eq!(manager.get_data("line"), Some("42"));
    assert_eq!(manager.get_metadata("theme"), Some("dark"));
}

#[test]
fn test_session_manager_multiple_sessions() {
    let mut manager = SessionManager::new();

    // Session 1
    manager.set_data("project", "toad");
    manager.save_session("session1");

    // Session 2
    manager.clear_data();
    manager.set_data("project", "other");
    manager.save_session("session2");

    // Session 3
    manager.clear_data();
    manager.set_data("project", "third");
    manager.save_session("session3");

    assert_eq!(manager.session_count(), 3);

    // Load session 1
    manager.load_session("session1");
    assert_eq!(manager.get_data("project"), Some("toad"));

    // Load session 2
    manager.load_session("session2");
    assert_eq!(manager.get_data("project"), Some("other"));

    // Load session 3
    manager.load_session("session3");
    assert_eq!(manager.get_data("project"), Some("third"));
}

#[test]
fn test_session_manager_delete() {
    let mut manager = SessionManager::new();

    manager.save_session("session1");
    manager.save_session("session2");
    assert_eq!(manager.session_count(), 2);

    assert!(manager.delete_session("session1"));
    assert_eq!(manager.session_count(), 1);
    assert!(!manager.has_session("session1"));
    assert!(manager.has_session("session2"));

    // Deleting non-existent session returns false
    assert!(!manager.delete_session("nonexistent"));
}

#[test]
fn test_session_manager_clear_all() {
    let mut manager = SessionManager::new();

    manager.save_session("s1");
    manager.save_session("s2");
    manager.save_session("s3");
    assert_eq!(manager.session_count(), 3);

    manager.clear_sessions();
    assert_eq!(manager.session_count(), 0);
    assert_eq!(manager.active_session(), None);
}

#[test]
fn test_session_manager_session_names() {
    let mut manager = SessionManager::new();

    manager.save_session("alpha");
    manager.save_session("beta");
    manager.save_session("gamma");

    let names = manager.session_names();
    assert_eq!(names.len(), 3);
    assert!(names.contains(&"alpha"));
    assert!(names.contains(&"beta"));
    assert!(names.contains(&"gamma"));
}

#[test]
fn test_session_manager_has_session() {
    let mut manager = SessionManager::new();

    assert!(!manager.has_session("test"));

    manager.save_session("test");
    assert!(manager.has_session("test"));

    manager.delete_session("test");
    assert!(!manager.has_session("test"));
}

#[test]
fn test_session_manager_get_session() {
    let mut manager = SessionManager::new();

    manager.set_data("key", "value");
    manager.save_session("test");

    let session = manager.get_session("test");
    assert!(session.is_some());
    assert_eq!(session.unwrap().name(), "test");

    let nonexistent = manager.get_session("nonexistent");
    assert!(nonexistent.is_none());
}

#[test]
fn test_session_manager_rename() {
    let mut manager = SessionManager::new();

    manager.set_data("key", "value");
    manager.save_session("old_name");

    assert!(manager.has_session("old_name"));
    assert!(manager.rename_session("old_name", "new_name"));

    assert!(!manager.has_session("old_name"));
    assert!(manager.has_session("new_name"));
    assert_eq!(manager.active_session(), Some("new_name"));

    // Verify data is preserved
    manager.load_session("new_name");
    assert_eq!(manager.get_data("key"), Some("value"));

    // Renaming nonexistent session returns false
    assert!(!manager.rename_session("nonexistent", "anything"));
}

#[test]
fn test_session_manager_auto_save() {
    let mut manager = SessionManager::new();

    // Enable auto-save
    manager.set_auto_save(true);
    assert!(manager.auto_save());

    // Create and activate a session
    manager.save_session("auto_session");

    // Set data - should auto-save
    manager.set_data("key1", "value1");

    // Load session to verify auto-save worked
    let session = manager.get_session("auto_session");
    assert!(session.is_some());
    assert_eq!(session.unwrap().get_data("key1"), Some("value1"));

    // Disable auto-save
    manager.set_auto_save(false);
    assert!(!manager.auto_save());
}

#[test]
fn test_session_manager_load_nonexistent() {
    let mut manager = SessionManager::new();

    assert!(!manager.load_session("nonexistent"));
    assert_eq!(manager.active_session(), None);
}

// ==================== Cross-Widget Session Workflows ====================

#[test]
fn test_session_workflow_save_switch_restore() {
    let mut manager = SessionManager::new();

    // Workflow 1: Working on feature A
    manager.set_data("current_file", "feature_a.rs");
    manager.set_data("cursor_line", "150");
    manager.set_metadata("branch", "feature-a");
    manager.save_session("feature_a_work");

    // Workflow 2: Switch to bugfix B
    manager.clear_data();
    manager.set_data("current_file", "bugfix.rs");
    manager.set_data("cursor_line", "42");
    manager.set_metadata("branch", "hotfix");
    manager.save_session("bugfix_work");

    // Workflow 3: Back to feature A
    manager.load_session("feature_a_work");
    assert_eq!(manager.get_data("current_file"), Some("feature_a.rs"));
    assert_eq!(manager.get_data("cursor_line"), Some("150"));
    assert_eq!(manager.get_metadata("branch"), Some("feature-a"));
}

#[test]
fn test_session_workflow_persistence_across_restarts() {
    // Simulate app session 1
    let mut manager1 = SessionManager::new();

    manager1.set_data("open_files", "main.rs,lib.rs,test.rs");
    manager1.set_data("active_tab", "1");
    manager1.set_metadata("window_size", "80x24");
    manager1.save_session("app_state");

    // Simulate app restart - new manager instance
    let mut manager2 = SessionManager::new();

    // In real app, sessions would be loaded from disk
    // Here we simulate by copying the session
    let session = manager1.get_session("app_state").unwrap().clone();
    manager2.save_session("app_state");

    // Manually restore the data (simulating load from disk)
    manager2.clear_data();
    for (k, v) in session.data() {
        manager2.set_data(k, v);
    }

    assert_eq!(
        manager2.get_data("open_files"),
        Some("main.rs,lib.rs,test.rs")
    );
    assert_eq!(manager2.get_data("active_tab"), Some("1"));
}

#[test]
fn test_session_workflow_multiple_workspaces() {
    let mut manager = SessionManager::new();

    // Workspace 1: Rust project
    manager.set_data("project_type", "rust");
    manager.set_data("build_command", "cargo build");
    manager.set_metadata("language", "rust");
    manager.save_session("rust_workspace");

    // Workspace 2: TypeScript project
    manager.clear_data();
    manager.set_data("project_type", "typescript");
    manager.set_data("build_command", "npm run build");
    manager.set_metadata("language", "typescript");
    manager.save_session("ts_workspace");

    // Switch between workspaces
    manager.load_session("rust_workspace");
    assert_eq!(manager.get_data("build_command"), Some("cargo build"));

    manager.load_session("ts_workspace");
    assert_eq!(manager.get_data("build_command"), Some("npm run build"));
}

// ==================== FileTreeNode Tests ====================

#[test]
fn test_filetree_node_file_creation() {
    let node = FileTreeNode::file(
        PathBuf::from("/home/user/file.txt"),
        "file.txt".to_string(),
        0,
    );

    assert_eq!(node.name, "file.txt");
    assert_eq!(node.node_type, FileTreeNodeType::File);
    assert_eq!(node.depth, 0);
    assert!(!node.is_expanded);
    assert_eq!(node.children.len(), 0);
}

#[test]
fn test_filetree_node_directory_creation() {
    let node = FileTreeNode::directory(PathBuf::from("/home/user/src"), "src".to_string(), 0);

    assert_eq!(node.name, "src");
    assert_eq!(node.node_type, FileTreeNodeType::Directory);
    assert_eq!(node.depth, 0);
    assert!(!node.is_expanded);
    assert_eq!(node.children.len(), 0);
}

#[test]
fn test_filetree_node_toggle_directory() {
    let mut node = FileTreeNode::directory(PathBuf::from("/tmp"), "tmp".to_string(), 0);

    assert!(!node.is_expanded);

    node.toggle();
    assert!(node.is_expanded);

    node.toggle();
    assert!(!node.is_expanded);
}

#[test]
fn test_filetree_node_toggle_file_no_effect() {
    let mut node = FileTreeNode::file(PathBuf::from("/tmp/file"), "file".to_string(), 0);

    assert!(!node.is_expanded);

    node.toggle();
    assert!(!node.is_expanded); // Files don't expand

    node.toggle();
    assert!(!node.is_expanded);
}

#[test]
fn test_filetree_node_depth_hierarchy() {
    let root = FileTreeNode::directory(PathBuf::from("/"), "root".to_string(), 0);
    let level1 = FileTreeNode::directory(PathBuf::from("/home"), "home".to_string(), 1);
    let level2 = FileTreeNode::directory(PathBuf::from("/home/user"), "user".to_string(), 2);
    let level3 = FileTreeNode::file(
        PathBuf::from("/home/user/file.txt"),
        "file.txt".to_string(),
        3,
    );

    assert_eq!(root.depth, 0);
    assert_eq!(level1.depth, 1);
    assert_eq!(level2.depth, 2);
    assert_eq!(level3.depth, 3);
}

// ==================== Performance & Edge Cases ====================

#[test]
fn test_session_manager_large_dataset() {
    let mut manager = SessionManager::new();

    // Add 1000 data entries
    for i in 0..1000 {
        manager.set_data(format!("key_{}", i), format!("value_{}", i));
    }

    // Save session
    manager.save_session("large_session");

    // Clear and reload
    manager.clear_data();
    assert!(manager.load_session("large_session"));

    // Verify all data is restored
    for i in 0..1000 {
        assert_eq!(
            manager.get_data(&format!("key_{}", i)),
            Some(format!("value_{}", i).as_str())
        );
    }
}

#[test]
fn test_session_manager_many_sessions() {
    let mut manager = SessionManager::new();

    // Create 100 sessions
    for i in 0..100 {
        manager.set_data("session_num", i.to_string());
        manager.save_session(format!("session_{}", i));
    }

    assert_eq!(manager.session_count(), 100);

    // Verify we can load any session
    manager.load_session("session_42");
    assert_eq!(manager.get_data("session_num"), Some("42"));

    manager.load_session("session_99");
    assert_eq!(manager.get_data("session_num"), Some("99"));
}

#[test]
fn test_session_data_clone() {
    let mut session1 = SessionData::new("original");
    session1.set_data("key", "value");
    session1.set_metadata("meta", "data");

    let session2 = session1.clone();

    assert_eq!(session1.name(), session2.name());
    assert_eq!(session1.get_data("key"), session2.get_data("key"));
    assert_eq!(session1.get_metadata("meta"), session2.get_metadata("meta"));

    // Verify they're independent copies
    // (this test just verifies clone works, actual independence would need mutation)
}

#[test]
fn test_session_manager_edge_cases() {
    let mut manager = SessionManager::new();

    // Empty session name is allowed (though not recommended)
    manager.save_session("");
    assert!(manager.has_session(""));

    // Overwriting existing session
    manager.set_data("version", "1");
    manager.save_session("test");

    manager.set_data("version", "2");
    manager.save_session("test"); // Overwrites

    manager.load_session("test");
    assert_eq!(manager.get_data("version"), Some("2"));
}

// ==================== Real-World Scenario Tests ====================

#[test]
fn test_scenario_development_session_management() {
    let mut manager = SessionManager::new();

    // Morning: Start working on feature
    manager.set_data("file", "src/feature.rs");
    manager.set_data("line", "100");
    manager.set_data("scroll", "80");
    manager.set_metadata("branch", "feature/new-widget");
    manager.set_metadata("uncommitted_changes", "true");
    manager.save_session("morning_work");

    // Afternoon: Emergency bugfix
    manager.clear_data();
    manager.set_data("file", "src/bug.rs");
    manager.set_data("line", "42");
    manager.set_metadata("branch", "hotfix/critical");
    manager.save_session("bugfix");

    // Evening: Back to feature work
    manager.load_session("morning_work");
    assert_eq!(manager.get_data("file"), Some("src/feature.rs"));
    assert_eq!(manager.get_data("line"), Some("100"));
    assert_eq!(manager.get_metadata("branch"), Some("feature/new-widget"));
}

#[test]
fn test_scenario_session_cleanup() {
    let mut manager = SessionManager::new();

    // Create multiple sessions
    manager.save_session("old_session_1");
    manager.save_session("old_session_2");
    manager.save_session("current_work");
    manager.save_session("future_work");

    // Clean up old sessions
    manager.delete_session("old_session_1");
    manager.delete_session("old_session_2");

    assert_eq!(manager.session_count(), 2);
    assert!(manager.has_session("current_work"));
    assert!(manager.has_session("future_work"));
}
