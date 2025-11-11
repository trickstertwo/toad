//! Integration tests for File Management and AI Features
//!
//! Tests PLATINUM tier features for bookmarks, recent files, file operations,
//! and chat panel for AI interaction.

use std::path::PathBuf;
use toad::infrastructure::{FileOpResult, FileOps};
use toad::navigation::{Bookmark, BookmarkManager, RecentFile, RecentFiles};
use toad::ui::widgets::chat_panel::{ChatMessage, ChatPanel, MessageRole};

// ============================================================================
// Bookmark Integration Tests
// ============================================================================

#[test]
fn test_bookmark_creation() {
    let bookmark = Bookmark::new("main", "/src/main.rs", 10, 0);
    assert_eq!(bookmark.name, "main");
    assert_eq!(bookmark.path, PathBuf::from("/src/main.rs"));
    assert_eq!(bookmark.line, 10);
    assert_eq!(bookmark.col, 0);
}

#[test]
fn test_bookmark_with_description() {
    let bookmark = Bookmark::new("main", "/src/main.rs", 10, 0).with_description("Entry point");
    assert_eq!(bookmark.description, Some("Entry point".to_string()));
}

#[test]
fn test_bookmark_file_name() {
    let bookmark = Bookmark::new("test", "/path/to/file.rs", 5, 0);
    assert_eq!(bookmark.file_name(), Some("file.rs"));
}

#[test]
fn test_bookmark_display() {
    let bookmark = Bookmark::new("main", "/src/main.rs", 10, 5);
    let display = bookmark.display();
    assert!(display.contains("main"));
    assert!(display.contains("main.rs"));
    assert!(display.contains("10"));
    assert!(display.contains("5"));
}

#[test]
fn test_bookmark_manager_creation() {
    let manager = BookmarkManager::new();
    assert_eq!(manager.count(), 0);
}

#[test]
fn test_bookmark_manager_add_bookmark() {
    let mut manager = BookmarkManager::new();
    manager.add_bookmark("main", "/src/main.rs", 10, 0);
    assert_eq!(manager.count(), 1);
    assert!(manager.has("main"));
}

#[test]
fn test_bookmark_manager_add_with_description() {
    let mut manager = BookmarkManager::new();
    manager.add_bookmark_with_desc("test", "/test.rs", 5, 0, "Test file");
    assert!(manager.has("test"));
    let bookmark = manager.get("test").unwrap();
    assert_eq!(bookmark.description, Some("Test file".to_string()));
}

#[test]
fn test_bookmark_manager_add_object() {
    let mut manager = BookmarkManager::new();
    let bookmark = Bookmark::new("test", "/test.rs", 5, 0);
    manager.add(bookmark);
    assert_eq!(manager.count(), 1);
}

#[test]
fn test_bookmark_manager_get() {
    let mut manager = BookmarkManager::new();
    manager.add_bookmark("main", "/src/main.rs", 10, 0);

    let bookmark = manager.get("main");
    assert!(bookmark.is_some());
    assert_eq!(bookmark.unwrap().line, 10);

    let nonexistent = manager.get("nonexistent");
    assert!(nonexistent.is_none());
}

#[test]
fn test_bookmark_manager_remove() {
    let mut manager = BookmarkManager::new();
    manager.add_bookmark("temp", "/temp.rs", 1, 0);
    assert!(manager.has("temp"));

    manager.remove("temp");
    assert!(!manager.has("temp"));
    assert_eq!(manager.count(), 0);
}

#[test]
fn test_bookmark_manager_update() {
    let mut manager = BookmarkManager::new();
    manager.add_bookmark("test", "/test.rs", 10, 0);

    // Adding with same name replaces
    manager.add_bookmark("test", "/test.rs", 20, 5);
    assert_eq!(manager.count(), 1);

    let bookmark = manager.get("test").unwrap();
    assert_eq!(bookmark.line, 20);
    assert_eq!(bookmark.col, 5);
}

#[test]
fn test_bookmark_manager_list() {
    let mut manager = BookmarkManager::new();
    manager.add_bookmark("a", "/a.rs", 1, 0);
    manager.add_bookmark("b", "/b.rs", 2, 0);
    manager.add_bookmark("c", "/c.rs", 3, 0);

    let bookmarks = manager.all();
    assert_eq!(bookmarks.len(), 3);
}

#[test]
fn test_bookmark_manager_search() {
    let mut manager = BookmarkManager::new();
    manager.add_bookmark("main", "/src/main.rs", 10, 0);
    manager.add_bookmark("test", "/tests/test.rs", 5, 0);
    manager.add_bookmark("lib", "/src/lib.rs", 1, 0);

    let results = manager.search("main");
    assert_eq!(results.len(), 1);

    let results = manager.search("src");
    assert_eq!(results.len(), 2);
}

#[test]
fn test_bookmark_manager_clear() {
    let mut manager = BookmarkManager::new();
    manager.add_bookmark("a", "/a.rs", 1, 0);
    manager.add_bookmark("b", "/b.rs", 2, 0);
    assert_eq!(manager.count(), 2);

    manager.clear();
    assert_eq!(manager.count(), 0);
}

// ============================================================================
// RecentFiles Integration Tests
// ============================================================================

#[test]
fn test_recent_file_creation() {
    let file = RecentFile::new(PathBuf::from("/test.rs"));
    assert_eq!(file.path, PathBuf::from("/test.rs"));
    assert_eq!(file.access_count, 1);
    assert!(file.last_accessed > 0);
}

#[test]
fn test_recent_file_touch() {
    let mut file = RecentFile::new(PathBuf::from("/test.rs"));
    let initial_time = file.last_accessed;
    let initial_count = file.access_count;

    std::thread::sleep(std::time::Duration::from_millis(100));
    file.touch();

    assert!(file.last_accessed >= initial_time);
    assert_eq!(file.access_count, initial_count + 1);
}

#[test]
fn test_recent_file_file_name() {
    let file = RecentFile::new(PathBuf::from("/path/to/file.rs"));
    assert_eq!(file.file_name(), Some("file.rs"));
}

#[test]
fn test_recent_file_parent() {
    let file = RecentFile::new(PathBuf::from("/path/to/file.rs"));
    assert_eq!(file.parent(), Some(PathBuf::from("/path/to").as_path()));
}

#[test]
fn test_recent_files_creation() {
    let recent = RecentFiles::new(10);
    assert_eq!(recent.max_size(), 10);
    assert_eq!(recent.len(), 0);
    assert!(recent.is_empty());
}

#[test]
fn test_recent_files_add() {
    let mut recent = RecentFiles::new(5);
    recent.add("/file1.rs".to_string());
    assert_eq!(recent.len(), 1);

    recent.add("/file2.rs".to_string());
    assert_eq!(recent.len(), 2);
}

#[test]
fn test_recent_files_add_duplicate() {
    let mut recent = RecentFiles::new(5);
    recent.add("/file.rs".to_string());
    recent.add("/file.rs".to_string());

    // Should still be 1 file, but access count increased
    assert_eq!(recent.len(), 1);
    let file = recent.get(0).unwrap();
    assert_eq!(file.access_count, 2);
}

#[test]
fn test_recent_files_mru_order() {
    let mut recent = RecentFiles::new(5);
    recent.add("/file1.rs".to_string());
    recent.add("/file2.rs".to_string());
    recent.add("/file3.rs".to_string());

    // Most recent should be first
    assert_eq!(recent.get(0).unwrap().path, PathBuf::from("/file3.rs"));
    assert_eq!(recent.get(1).unwrap().path, PathBuf::from("/file2.rs"));
    assert_eq!(recent.get(2).unwrap().path, PathBuf::from("/file1.rs"));

    // Re-access file1
    recent.add("/file1.rs".to_string());
    assert_eq!(recent.get(0).unwrap().path, PathBuf::from("/file1.rs"));
}

#[test]
fn test_recent_files_max_size() {
    let mut recent = RecentFiles::new(3);

    for i in 0..5 {
        recent.add(format!("/file{}.rs", i));
    }

    // Should only keep 3 most recent
    assert_eq!(recent.len(), 3);
    assert_eq!(recent.get(0).unwrap().path, PathBuf::from("/file4.rs"));
    assert_eq!(recent.get(1).unwrap().path, PathBuf::from("/file3.rs"));
    assert_eq!(recent.get(2).unwrap().path, PathBuf::from("/file2.rs"));
}

#[test]
fn test_recent_files_get() {
    let mut recent = RecentFiles::new(5);
    recent.add("/file1.rs".to_string());
    recent.add("/file2.rs".to_string());

    let file = recent.get(0);
    assert!(file.is_some());

    let out_of_bounds = recent.get(5);
    assert!(out_of_bounds.is_none());
}

#[test]
fn test_recent_files_files() {
    let mut recent = RecentFiles::new(5);
    recent.add("/file1.rs".to_string());
    recent.add("/file2.rs".to_string());

    let files = recent.files();
    assert_eq!(files.len(), 2);
}

#[test]
fn test_recent_files_clear() {
    let mut recent = RecentFiles::new(5);
    recent.add("/file1.rs".to_string());
    recent.add("/file2.rs".to_string());
    assert_eq!(recent.len(), 2);

    recent.clear();
    assert_eq!(recent.len(), 0);
    assert!(recent.is_empty());
}

#[test]
fn test_recent_files_most_frequent() {
    let mut recent = RecentFiles::new(10);
    recent.add("/file1.rs".to_string());
    recent.add("/file2.rs".to_string());
    recent.add("/file1.rs".to_string());
    recent.add("/file1.rs".to_string());
    recent.add("/file2.rs".to_string());

    let frequent = recent.by_frequency();
    assert_eq!(frequent.len(), 2);
    assert_eq!(frequent[0].path, PathBuf::from("/file1.rs"));
    assert_eq!(frequent[0].access_count, 3);
}

// ============================================================================
// FileOps Integration Tests
// ============================================================================

#[test]
fn test_file_ops_creation() {
    let _ops = FileOps::new();
    // Default settings verified through builder
    let _ops = FileOps::new()
        .with_overwrite(true)
        .with_create_parents(false);
}

#[test]
fn test_file_op_result_success() {
    let result = FileOpResult::success(
        "copy".to_string(),
        PathBuf::from("/src.txt"),
        Some(PathBuf::from("/dest.txt")),
    );

    assert!(result.success);
    assert_eq!(result.operation, "copy");
    assert_eq!(result.error, None);
}

#[test]
fn test_file_op_result_failure() {
    let result = FileOpResult::failure(
        "copy".to_string(),
        PathBuf::from("/src.txt"),
        Some(PathBuf::from("/dest.txt")),
        "File not found".to_string(),
    );

    assert!(!result.success);
    assert_eq!(result.error, Some("File not found".to_string()));
}

#[test]
fn test_file_ops_copy_nonexistent_source() {
    let ops = FileOps::new();
    let result = ops.copy("/nonexistent.txt", "/dest.txt").unwrap();

    assert!(!result.success);
    assert!(result.error.is_some());
    assert!(result.error.unwrap().contains("does not exist"));
}

#[test]
fn test_file_ops_builder_pattern() {
    let ops = FileOps::new()
        .with_overwrite(true)
        .with_create_parents(false);

    // Builder pattern works
    let _result = ops.copy("/fake/src.txt", "/fake/dest.txt");
}

// ============================================================================
// ChatPanel Integration Tests
// ============================================================================

#[test]
fn test_chat_message_creation() {
    let msg = ChatMessage::new(MessageRole::User, "Hello");
    assert_eq!(msg.role, MessageRole::User);
    assert_eq!(msg.content, "Hello");
    assert!(!msg.streaming);
    assert!(!msg.has_code);
}

#[test]
fn test_chat_message_with_code() {
    let msg = ChatMessage::new(MessageRole::Assistant, "```rust\nfn main() {}\n```");
    assert!(msg.has_code);
}

#[test]
fn test_chat_message_streaming() {
    let msg = ChatMessage::streaming(MessageRole::Assistant, "Hello");
    assert!(msg.streaming);
}

#[test]
fn test_chat_message_append() {
    let mut msg = ChatMessage::streaming(MessageRole::Assistant, "Hello");
    msg.append(" world");
    assert_eq!(msg.content, "Hello world");
}

#[test]
fn test_chat_message_finish_streaming() {
    let mut msg = ChatMessage::streaming(MessageRole::Assistant, "Hello");
    assert!(msg.streaming);

    msg.finish_streaming();
    assert!(!msg.streaming);
}

#[test]
fn test_chat_message_formatted_time() {
    let msg = ChatMessage::new(MessageRole::User, "Test");
    let time = msg.formatted_time();
    assert!(!time.is_empty());
    assert!(time.contains(":"));
}

#[test]
fn test_chat_message_roles() {
    let user = ChatMessage::new(MessageRole::User, "User message");
    let assistant = ChatMessage::new(MessageRole::Assistant, "Assistant message");
    let system = ChatMessage::new(MessageRole::System, "System message");

    assert_eq!(user.role, MessageRole::User);
    assert_eq!(assistant.role, MessageRole::Assistant);
    assert_eq!(system.role, MessageRole::System);
}

#[test]
fn test_chat_panel_creation() {
    let panel = ChatPanel::new();
    assert_eq!(panel.message_count(), 0);
}

#[test]
fn test_chat_panel_add_message() {
    let mut panel = ChatPanel::new();
    let msg = ChatMessage::new(MessageRole::User, "Hello");
    panel.add_message(msg);
    assert_eq!(panel.message_count(), 1);
}

#[test]
fn test_chat_panel_add_user_message() {
    let mut panel = ChatPanel::new();
    panel.add_user_message("Hello!");
    assert_eq!(panel.message_count(), 1);
}

#[test]
fn test_chat_panel_add_assistant_message() {
    let mut panel = ChatPanel::new();
    panel.add_assistant_message("Hi there!");
    assert_eq!(panel.message_count(), 1);
}

#[test]
fn test_chat_panel_add_system_message() {
    let mut panel = ChatPanel::new();
    panel.add_system_message("System initialized");
    assert_eq!(panel.message_count(), 1);
}

#[test]
fn test_chat_panel_streaming() {
    let mut panel = ChatPanel::new();

    let _idx = panel.start_streaming();
    assert_eq!(panel.message_count(), 1);

    panel.append_streaming("Hello");
    panel.append_streaming(" world");
    panel.finish_streaming();

    // Verify streaming worked
    assert_eq!(panel.message_count(), 1);
}

#[test]
fn test_chat_panel_scroll() {
    let mut panel = ChatPanel::new();
    for i in 0..10 {
        panel.add_user_message(format!("Message {}", i));
    }

    panel.scroll_up(5);
    panel.scroll_down(2);
    panel.scroll_to_top();
    panel.scroll_to_bottom();
}

#[test]
fn test_chat_panel_max_history() {
    let mut panel = ChatPanel::new();

    // Add more than max_history messages
    for i in 0..1500 {
        panel.add_user_message(format!("Message {}", i));
    }

    // Should be trimmed to max_history (default 1000)
    assert!(panel.message_count() <= 1000);
}

#[test]
fn test_chat_panel_clear() {
    let mut panel = ChatPanel::new();
    panel.add_user_message("Test");
    panel.add_assistant_message("Response");
    assert_eq!(panel.message_count(), 2);

    panel.clear();
    assert_eq!(panel.message_count(), 0);
}

#[test]
fn test_chat_panel_toggle_timestamps() {
    let mut panel = ChatPanel::new();
    panel.toggle_timestamps();
    panel.toggle_timestamps();
}

// ============================================================================
// Cross-Feature Integration Tests
// ============================================================================

#[test]
fn test_bookmarks_with_recent_files() {
    let mut bookmarks = BookmarkManager::new();
    let mut recent = RecentFiles::new(10);

    // Add some bookmarks
    bookmarks.add_bookmark("main", "/src/main.rs", 1, 0);
    bookmarks.add_bookmark("test", "/tests/test.rs", 5, 0);

    // Access bookmarked files
    recent.add("/src/main.rs".to_string());
    recent.add("/tests/test.rs".to_string());

    assert_eq!(bookmarks.count(), 2);
    assert_eq!(recent.len(), 2);

    // Most recently accessed should be test
    assert_eq!(recent.get(0).unwrap().path, PathBuf::from("/tests/test.rs"));
}

#[test]
fn test_chat_panel_with_file_references() {
    let mut panel = ChatPanel::new();
    let mut bookmarks = BookmarkManager::new();

    // User asks about a file
    panel.add_user_message("Show me the main function");

    // Add bookmark to referenced location
    bookmarks.add_bookmark("main_func", "/src/main.rs", 10, 0);

    // Assistant responds with file reference
    panel.add_assistant_message("The main function is at /src/main.rs:10");

    assert_eq!(panel.message_count(), 2);
    assert!(bookmarks.has("main_func"));
}

#[test]
fn test_complete_file_workflow() {
    let mut bookmarks = BookmarkManager::new();
    let mut recent = RecentFiles::new(20);
    let _ops = FileOps::new().with_overwrite(true);

    // User opens a file
    recent.add("/src/main.rs".to_string());

    // User bookmarks important location
    bookmarks.add_bookmark("entry", "/src/main.rs", 1, 0);

    // User navigates to other files
    recent.add("/src/lib.rs".to_string());
    recent.add("/tests/integration.rs".to_string());

    // User bookmarks test location
    bookmarks.add_bookmark("test_suite", "/tests/integration.rs", 50, 0);

    // User returns to main via bookmark
    let bookmark = bookmarks.get("entry").unwrap();
    recent.add(bookmark.path.to_string_lossy().to_string());

    // Verify state
    assert_eq!(bookmarks.count(), 2);
    assert_eq!(recent.len(), 3);
    assert_eq!(recent.get(0).unwrap().path, PathBuf::from("/src/main.rs"));
}

#[test]
fn test_chat_panel_ai_workflow() {
    let mut panel = ChatPanel::new();

    // User asks question
    panel.add_user_message("How do I implement a binary tree?");

    // AI starts streaming response
    panel.start_streaming();
    panel.append_streaming("A binary tree can be implemented with");
    panel.append_streaming(" a recursive structure: ");
    panel.append_streaming("```rust\nstruct Node {\n    value: i32,\n    left: Option<Box<Node>>,\n    right: Option<Box<Node>>,\n}\n```");
    panel.finish_streaming();

    // User follows up
    panel.add_user_message("Can you show traversal methods?");

    // AI responds
    panel.add_assistant_message(
        "Here are the main traversal methods: in-order, pre-order, and post-order.",
    );

    assert_eq!(panel.message_count(), 4);
}
