//! Integration tests for PLATINUM Tier AI Features & File Management
//!
//! Tests for ChatPanel, TokenCounter, ModelSelector, Bookmarks, RecentFiles.

use toad::infrastructure::FileOps;
use toad::navigation::{Bookmark, BookmarkManager, RecentFiles};
use toad::ui::widgets::chat_panel::{ChatMessage, ChatPanel, MessageRole};
use toad::ui::widgets::progress::token_counter::{CostModel, TokenCounter, TokenUsage};
use toad::ui::widgets::selection::model_selector::{ModelInfo, ModelSelector};

// ==================== ChatPanel Tests ====================

#[test]
fn test_chat_panel_creation() {
    let panel = ChatPanel::new();

    assert_eq!(panel.message_count(), 0);
    assert!(panel.is_auto_scrolling());
}

#[test]
fn test_chat_panel_add_messages() {
    let mut panel = ChatPanel::new();

    panel.add_user_message("Hello, AI!");
    panel.add_assistant_message("Hello! How can I help?");
    panel.add_system_message("System initialized");

    assert_eq!(panel.message_count(), 3);
}

#[test]
fn test_chat_panel_conversation() {
    let mut panel = ChatPanel::new();

    panel.add_user_message("What is Rust?");
    panel.add_assistant_message("Rust is a systems programming language...");
    panel.add_user_message("Tell me more");
    panel.add_assistant_message("Rust focuses on safety, speed, and concurrency...");

    assert_eq!(panel.message_count(), 4);
}

#[test]
fn test_chat_panel_streaming() {
    let mut panel = ChatPanel::new();

    let _stream_idx = panel.start_streaming();

    panel.append_streaming("This ");
    panel.append_streaming("is ");
    panel.append_streaming("streaming");

    panel.finish_streaming();

    assert_eq!(panel.message_count(), 1);
}

#[test]
fn test_chat_panel_scroll() {
    let mut panel = ChatPanel::new();

    for i in 0..20 {
        panel.add_user_message(format!("Message {}", i));
    }

    panel.scroll_up(5);
    panel.scroll_down(3);
    panel.scroll_to_top();
    panel.scroll_to_bottom();

    assert_eq!(panel.message_count(), 20);
}

#[test]
fn test_chat_panel_auto_scroll() {
    let mut panel = ChatPanel::new();

    assert!(panel.is_auto_scrolling());

    panel.toggle_auto_scroll();
    assert!(!panel.is_auto_scrolling());

    panel.toggle_auto_scroll();
    assert!(panel.is_auto_scrolling());
}

#[test]
fn test_chat_panel_clear() {
    let mut panel = ChatPanel::new();

    panel.add_user_message("Message 1");
    panel.add_assistant_message("Response 1");
    assert_eq!(panel.message_count(), 2);

    panel.clear();
    assert_eq!(panel.message_count(), 0);
}

#[test]
fn test_chat_message_creation() {
    let msg = ChatMessage::new(MessageRole::User, "Hello");

    assert!(!msg.formatted_time().is_empty());
}

#[test]
fn test_chat_message_streaming() {
    let mut msg = ChatMessage::streaming(MessageRole::Assistant, "Initial");

    msg.append(" more");
    msg.append(" text");
    msg.finish_streaming();
}

// ==================== TokenCounter Tests ====================

#[test]
fn test_token_counter_creation() {
    let counter = TokenCounter::new();

    assert_eq!(counter.session_cost(), 0.0);
    assert_eq!(counter.total_cost(), 0.0);
}

#[test]
fn test_token_counter_add_usage() {
    let mut counter = TokenCounter::new();

    let usage = TokenUsage::new(100, 50);
    counter.add_usage(usage);

    assert!(counter.session_cost() > 0.0);
}

#[test]
fn test_token_counter_reset_session() {
    let mut counter = TokenCounter::new();

    counter.add_usage(TokenUsage::new(100, 50));
    assert!(counter.session_cost() > 0.0);

    counter.reset_session();
    assert_eq!(counter.session_cost(), 0.0);
}

#[test]
fn test_token_counter_reset_total() {
    let mut counter = TokenCounter::new();

    counter.add_usage(TokenUsage::new(100, 50));
    assert!(counter.total_cost() > 0.0);

    counter.reset_total();
    assert_eq!(counter.total_cost(), 0.0);
}

#[test]
fn test_token_counter_cost_models() {
    let mut counter = TokenCounter::new();

    // Test Claude Sonnet 4.5
    counter.set_cost_model(CostModel::claude_sonnet_4_5());
    counter.add_usage(TokenUsage::new(1000, 1000));
    let sonnet_cost = counter.session_cost();

    counter.reset_session();

    // Test Claude Opus 4
    counter.set_cost_model(CostModel::claude_opus_4());
    counter.add_usage(TokenUsage::new(1000, 1000));
    let opus_cost = counter.session_cost();

    // Opus should be more expensive
    assert!(opus_cost > sonnet_cost);
}

#[test]
fn test_token_counter_with_budget() {
    let counter = TokenCounter::new().with_budget(10.0);

    assert_eq!(counter.session_cost(), 0.0);
}

#[test]
fn test_token_counter_toggle_details() {
    let mut counter = TokenCounter::new();

    counter.toggle_details();
    counter.toggle_details();
}

#[test]
fn test_token_usage() {
    let usage = TokenUsage::new(100, 50);

    assert_eq!(usage.input_tokens, 100);
    assert_eq!(usage.output_tokens, 50);
    assert_eq!(usage.total(), 150);
}

#[test]
fn test_token_usage_add() {
    let mut usage1 = TokenUsage::new(100, 50);
    let usage2 = TokenUsage::new(200, 100);

    usage1.add(&usage2);

    assert_eq!(usage1.input_tokens, 300);
    assert_eq!(usage1.output_tokens, 150);
    assert_eq!(usage1.total(), 450);
}

#[test]
fn test_token_usage_reset() {
    let mut usage = TokenUsage::new(100, 50);

    usage.reset();

    assert_eq!(usage.input_tokens, 0);
    assert_eq!(usage.output_tokens, 0);
    assert_eq!(usage.total(), 0);
}

#[test]
fn test_cost_model_calculate() {
    let model = CostModel::claude_haiku_4();
    let usage = TokenUsage::new(1000, 1000);

    let cost = model.calculate_cost(&usage);

    assert!(cost > 0.0);
}

// ==================== ModelSelector Tests ====================

#[test]
fn test_model_selector_creation() {
    let selector = ModelSelector::new();

    // Should have default models
    assert!(selector.selected_model().is_some());
}

#[test]
fn test_model_selector_navigation() {
    let mut selector = ModelSelector::new();

    selector.next();
    selector.next();
    selector.previous();
}

#[test]
fn test_model_selector_select() {
    let mut selector = ModelSelector::new();

    selector.select(0);
    assert!(selector.selected_model().is_some());
}

#[test]
fn test_model_selector_select_by_id() {
    let mut selector = ModelSelector::new();

    // Try to select a model by ID
    selector.select_by_id("claude-sonnet-4-5");
}

#[test]
fn test_model_selector_add_model() {
    let mut selector = ModelSelector::new();

    let custom_model = ModelInfo::new("custom-model", "Custom Model", "Custom Provider")
        .with_context_window(100000)
        .with_max_output(4000)
        .with_cost(0.01)
        .with_speed(1.5)
        .with_capability("coding")
        .with_available(true);

    selector.add_model(custom_model);
}

#[test]
fn test_model_selector_toggle_details() {
    let mut selector = ModelSelector::new();

    selector.toggle_details();
    selector.toggle_details();
}

#[test]
fn test_model_selector_filter() {
    let mut selector = ModelSelector::new();

    selector.set_filter(Some("coding".to_string()));
    selector.set_filter(None);
}

#[test]
fn test_model_info_creation() {
    let model = ModelInfo::new("test-model", "Test Model", "Test Provider");

    assert_eq!(model.id, "test-model");
    assert_eq!(model.name, "Test Model");
}

#[test]
fn test_model_info_builder() {
    let model = ModelInfo::new("model", "Model", "Provider")
        .with_context_window(128000)
        .with_max_output(8192)
        .with_cost(0.05)
        .with_speed(2.0)
        .with_capability("vision")
        .with_available(true);

    assert_eq!(model.context_window, 128000);
    assert_eq!(model.max_output, 8192);
    assert!(model.available);
}

#[test]
fn test_model_info_indicators() {
    let model = ModelInfo::new("model", "Model", "Provider")
        .with_cost(0.001)
        .with_speed(5.0);

    let cost_indicator = model.cost_indicator();
    let speed_indicator = model.speed_indicator();

    assert!(!cost_indicator.is_empty());
    assert!(!speed_indicator.is_empty());
}

// ==================== FileOps Tests ====================

#[test]
fn test_file_ops_creation() {
    let _ops = FileOps::new();
}

#[test]
fn test_file_ops_builder() {
    let _ops = FileOps::new()
        .with_overwrite(true)
        .with_create_parents(true);
}

// ==================== Bookmarks Tests ====================

#[test]
fn test_bookmark_creation() {
    let bookmark = Bookmark::new("home", "/home/user", 0, 0);

    assert_eq!(bookmark.name, "home");
}

#[test]
fn test_bookmark_with_description() {
    let bookmark = Bookmark::new("docs", "/docs", 0, 0).with_description("Documentation folder");

    assert_eq!(
        bookmark.description,
        Some("Documentation folder".to_string())
    );
}

#[test]
fn test_bookmark_display() {
    let bookmark = Bookmark::new("src", "/src", 10, 5).with_description("Source code");

    let display = bookmark.display();
    assert!(display.contains("src"));
}

#[test]
fn test_bookmark_manager_creation() {
    let manager = BookmarkManager::new();

    assert_eq!(manager.count(), 0);
    assert!(manager.is_empty());
}

#[test]
fn test_bookmark_manager_add() {
    let mut manager = BookmarkManager::new();

    manager.add_bookmark("test", "/test", 0, 0);

    assert_eq!(manager.count(), 1);
    assert!(!manager.is_empty());
}

#[test]
fn test_bookmark_manager_add_with_desc() {
    let mut manager = BookmarkManager::new();

    manager.add_bookmark_with_desc("code", "/code", 10, 0, "My code directory");

    assert_eq!(manager.count(), 1);
}

#[test]
fn test_bookmark_manager_get() {
    let mut manager = BookmarkManager::new();

    manager.add_bookmark("home", "/home", 0, 0);

    let bookmark = manager.get("home");
    assert!(bookmark.is_some());
    assert_eq!(bookmark.unwrap().name, "home");
}

#[test]
fn test_bookmark_manager_has() {
    let mut manager = BookmarkManager::new();

    manager.add_bookmark("test", "/test", 0, 0);

    assert!(manager.has("test"));
    assert!(!manager.has("nonexistent"));
}

#[test]
fn test_bookmark_manager_remove() {
    let mut manager = BookmarkManager::new();

    manager.add_bookmark("temp", "/tmp", 0, 0);
    assert_eq!(manager.count(), 1);

    let removed = manager.remove("temp");
    assert!(removed);
    assert_eq!(manager.count(), 0);
}

#[test]
fn test_bookmark_manager_all() {
    let mut manager = BookmarkManager::new();

    manager.add_bookmark("a", "/a", 0, 0);
    manager.add_bookmark("b", "/b", 0, 0);
    manager.add_bookmark("c", "/c", 0, 0);

    let all = manager.all();
    assert_eq!(all.len(), 3);
}

#[test]
fn test_bookmark_manager_names() {
    let mut manager = BookmarkManager::new();

    manager.add_bookmark("first", "/first", 0, 0);
    manager.add_bookmark("second", "/second", 0, 0);

    let names = manager.names();
    assert_eq!(names.len(), 2);
    assert!(names.contains(&"first".to_string()));
    assert!(names.contains(&"second".to_string()));
}

#[test]
fn test_bookmark_manager_clear() {
    let mut manager = BookmarkManager::new();

    manager.add_bookmark("a", "/a", 0, 0);
    manager.add_bookmark("b", "/b", 0, 0);
    assert_eq!(manager.count(), 2);

    manager.clear();
    assert_eq!(manager.count(), 0);
    assert!(manager.is_empty());
}

#[test]
fn test_bookmark_manager_search() {
    let mut manager = BookmarkManager::new();

    manager.add_bookmark("project_rust", "/rust", 0, 0);
    manager.add_bookmark("project_go", "/go", 0, 0);
    manager.add_bookmark("docs", "/docs", 0, 0);

    let results = manager.search("project");
    assert_eq!(results.len(), 2);
}

#[test]
fn test_bookmark_manager_sorted() {
    let mut manager = BookmarkManager::new();

    manager.add_bookmark("zebra", "/z", 0, 0);
    manager.add_bookmark("alpha", "/a", 0, 0);
    manager.add_bookmark("beta", "/b", 0, 0);

    let sorted = manager.sorted_by_name();
    assert_eq!(sorted[0].name, "alpha");
    assert_eq!(sorted[2].name, "zebra");
}

// ==================== RecentFiles Tests ====================

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

    recent.add("/file1.txt".to_string());
    recent.add("/file2.txt".to_string());

    assert_eq!(recent.len(), 2);
    assert!(!recent.is_empty());
}

#[test]
fn test_recent_files_max_size() {
    let mut recent = RecentFiles::new(3);

    recent.add("/file1.txt".to_string());
    recent.add("/file2.txt".to_string());
    recent.add("/file3.txt".to_string());
    recent.add("/file4.txt".to_string());

    // Should only keep last 3
    assert_eq!(recent.len(), 3);
}

#[test]
fn test_recent_files_remove() {
    let mut recent = RecentFiles::new(5);

    recent.add("/file1.txt".to_string());
    recent.add("/file2.txt".to_string());

    let removed = recent.remove("/file1.txt");
    assert!(removed);
    assert_eq!(recent.len(), 1);
}

#[test]
fn test_recent_files_clear() {
    let mut recent = RecentFiles::new(5);

    recent.add("/file1.txt".to_string());
    recent.add("/file2.txt".to_string());
    assert_eq!(recent.len(), 2);

    recent.clear();
    assert_eq!(recent.len(), 0);
    assert!(recent.is_empty());
}

#[test]
fn test_recent_files_get() {
    let mut recent = RecentFiles::new(5);

    recent.add("/file1.txt".to_string());
    recent.add("/file2.txt".to_string());

    let file = recent.get(0);
    assert!(file.is_some());
}

#[test]
fn test_recent_files_files() {
    let mut recent = RecentFiles::new(5);

    recent.add("/file1.txt".to_string());
    recent.add("/file2.txt".to_string());

    let files = recent.files();
    assert_eq!(files.len(), 2);
}

#[test]
fn test_recent_files_search() {
    let mut recent = RecentFiles::new(10);

    recent.add("/project/src/main.rs".to_string());
    recent.add("/project/tests/test.rs".to_string());
    recent.add("/docs/README.md".to_string());

    let results = recent.search("project");
    assert_eq!(results.len(), 2);
}

#[test]
fn test_recent_files_cleanup() {
    let mut recent = RecentFiles::new(10);

    recent.add("/nonexistent1.txt".to_string());
    recent.add("/nonexistent2.txt".to_string());

    recent.cleanup();

    // Nonexistent files should be removed
    assert_eq!(recent.len(), 0);
}

// ==================== Cross-Feature Integration Tests ====================

#[test]
fn test_ai_coding_workflow() {
    let mut chat = ChatPanel::new();
    let mut tokens = TokenCounter::new().with_budget(5.0);
    let selector = ModelSelector::new();

    // User asks a question
    chat.add_user_message("How do I implement a linked list in Rust?");

    // AI responds
    chat.add_assistant_message("Here's how to implement a linked list in Rust using Box<T>...");

    // Track token usage
    tokens.add_usage(TokenUsage::new(50, 200));

    // Check conversation state
    assert_eq!(chat.message_count(), 2);
    assert!(tokens.session_cost() > 0.0);
    assert!(selector.selected_model().is_some());
}

#[test]
fn test_file_management_workflow() {
    let mut bookmarks = BookmarkManager::new();
    let mut recent = RecentFiles::new(10);

    // Add project bookmarks with line/column positions
    bookmarks.add_bookmark_with_desc(
        "toad_main",
        "/home/user/toad/src/main.rs",
        1,
        0,
        "Entry point",
    );
    bookmarks.add_bookmark_with_desc("docs", "/home/user/docs/README.md", 0, 0, "Documentation");

    // Work with files
    recent.add("/home/user/toad/src/main.rs".to_string());
    recent.add("/home/user/toad/README.md".to_string());
    recent.add("/home/user/docs/guide.md".to_string());

    // Verify state
    assert_eq!(bookmarks.count(), 2);
    assert_eq!(recent.len(), 3);

    // Search bookmarks
    let toad_bookmarks = bookmarks.search("toad");
    assert_eq!(toad_bookmarks.len(), 1);

    // Search recent files
    let toad_files = recent.search("toad");
    assert_eq!(toad_files.len(), 2);
}

#[test]
fn test_complete_ai_session() {
    let mut chat = ChatPanel::new();
    let mut tokens = TokenCounter::new();
    let mut recent = RecentFiles::new(20);

    // Initialize conversation
    chat.add_system_message("Claude AI initialized. Ready to assist with Rust development.");

    // User opens a file
    recent.add("/project/src/lib.rs".to_string());

    // User asks about the file
    chat.add_user_message("Analyze this file and suggest improvements");

    // AI starts streaming response
    let _stream_idx = chat.start_streaming();
    chat.append_streaming("Based on the code, here are some suggestions:\n");
    chat.append_streaming("1. Add error handling\n");
    chat.append_streaming("2. Improve documentation\n");
    chat.finish_streaming();

    // Track usage
    tokens.add_usage(TokenUsage::new(1000, 500));

    // Verify state
    assert_eq!(chat.message_count(), 3);
    assert!(tokens.session_cost() > 0.0);
    assert_eq!(recent.len(), 1);
}
