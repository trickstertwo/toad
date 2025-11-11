//! End-to-End Test Scenarios
//!
//! Comprehensive tests simulating complete user workflows across multiple widgets.
//! These tests verify that widgets work correctly together in realistic usage scenarios.

use toad::ui::widgets::chat_panel::{ChatPanel};
use toad::ui::widgets::git::git_diff_viewer::{GitDiffViewer};
use toad::ui::widgets::git::git_graph::{GitCommit, GitGraph};
use toad::ui::widgets::git::git_status_panel::{FileStatus, GitStatusPanel};
use toad::ui::widgets::progress::token_counter::{CostModel, TokenCounter, TokenUsage};
use toad::ui::widgets::selection::model_selector::{ModelInfo, ModelSelector};
use toad::ui::widgets::session_manager::{SessionManager};

// ==================== E2E: Complete AI Coding Session ====================

#[test]
fn test_e2e_ai_coding_session_workflow() {
    // Initialize widgets for AI coding session
    let mut chat = ChatPanel::new();
    let mut token_counter = TokenCounter::new();
    let mut model_selector = ModelSelector::new().with_models(vec![
        ModelInfo::new("claude-sonnet-4.5", "Claude Sonnet 4.5", "Anthropic"),
        ModelInfo::new("claude-opus-4", "Claude Opus 4", "Anthropic"),
    ]);
    let mut session = SessionManager::new();

    // Step 1: User selects AI model
    assert_eq!(
        model_selector.selected_model().unwrap().id,
        "claude-sonnet-4.5"
    );
    token_counter.set_cost_model(CostModel::claude_sonnet_4_5());

    // Step 2: User asks first question
    chat.add_user_message("How do I implement a binary search tree in Rust?");
    assert_eq!(chat.message_count(), 1);

    // Step 3: AI responds (streaming)
    chat.start_streaming();
    chat.append_streaming("A binary search tree in Rust can be implemented using...");
    chat.append_streaming("\n\nHere's a basic structure:\n```rust\nstruct Node {");
    chat.finish_streaming();

    // Track token usage for this interaction
    token_counter.add_usage(TokenUsage::new(150, 400)); // 150 input, 400 output
    let cost_1 = token_counter.session_cost();
    assert!(cost_1 > 0.0);
    assert_eq!(chat.message_count(), 2); // User + Assistant

    // Step 4: User follows up
    chat.add_user_message("Can you add a search method?");
    chat.start_streaming();
    chat.append_streaming("Sure! Here's a search method:\n```rust\nimpl Node {");
    chat.finish_streaming();

    token_counter.add_usage(TokenUsage::new(50, 200));
    let cost_2 = token_counter.session_cost();
    assert!(cost_2 > cost_1); // Cost increased
    assert_eq!(chat.message_count(), 4); // 2 users + 2 assistants

    // Step 5: User switches to more powerful model for complex question
    model_selector.next();
    assert_eq!(model_selector.selected_model().unwrap().id, "claude-opus-4");
    token_counter.set_cost_model(CostModel::claude_opus_4());

    chat.add_user_message("How do I handle thread safety with Arc and Mutex?");
    chat.start_streaming();
    chat.append_streaming("Thread safety in Rust requires careful consideration...");
    chat.finish_streaming();

    token_counter.add_usage(TokenUsage::new(100, 500));
    let cost_3 = token_counter.session_cost();
    assert!(cost_3 > cost_2); // Opus is more expensive

    // Step 6: Save session for later
    session.set_data("chat_messages", chat.message_count().to_string());
    session.set_data("total_cost", cost_3.to_string());
    session.set_data("model", "claude-opus-4");
    session.save_session("ai_coding_session");

    // Verify session saved correctly
    assert_eq!(session.session_count(), 1);
    assert_eq!(chat.message_count(), 6); // 3 users + 3 assistants
}

// ==================== E2E: Complete Git Workflow ====================

#[test]
fn test_e2e_git_complete_workflow() {
    // Initialize git widgets
    let mut status = GitStatusPanel::new();
    let mut diff_viewer = GitDiffViewer::new();
    let mut graph = GitGraph::new();
    let mut session = SessionManager::new();

    // Step 1: Check working directory status
    status.set_branch("main");
    status.set_ahead_behind(0, 0); // In sync with remote
    assert_eq!(status.file_count(), 0); // Clean working directory

    // Step 2: Make changes to files
    status.add_file("src/main.rs", FileStatus::Modified);
    status.add_file("tests/test.rs", FileStatus::Modified);
    status.add_file("README.md", FileStatus::Modified);
    assert_eq!(status.file_count(), 3);

    // Step 3: View diff for main.rs
    let diff_content = r#"diff --git a/src/main.rs b/src/main.rs
index 1234567..abcdefg 100644
--- a/src/main.rs
+++ b/src/main.rs
@@ -1,5 +1,8 @@
 fn main() {
     println!("Hello, world!");
+
+    // New feature
+    println!("Added functionality!");
 }
"#;
    diff_viewer.set_diff(diff_content);
    let (additions, deletions, _) = diff_viewer.stats();
    assert_eq!(additions, 3);
    assert_eq!(deletions, 0);

    // Step 4: Stage files for commit
    status.clear();
    status.add_file("src/main.rs", FileStatus::Staged);
    status.add_file("tests/test.rs", FileStatus::Staged);
    status.add_file("README.md", FileStatus::Staged);

    // Step 5: Create commit
    graph.add_commit(
        GitCommit::new("abc123", "Add new feature with tests and documentation")
            .with_author("Developer")
            .with_branch("main"),
    );
    assert_eq!(graph.commit_count(), 1);

    // Step 6: Working directory clean again
    status.clear();
    assert_eq!(status.file_count(), 0);

    // Step 7: Push to remote (simulated by updating ahead/behind)
    status.set_ahead_behind(1, 0); // 1 commit ahead
    assert_eq!(status.file_count(), 0); // Still clean

    // After push
    status.set_ahead_behind(0, 0); // In sync

    // Step 8: Save git session state
    session.set_data("branch", "main");
    session.set_data("commits", graph.commit_count().to_string());
    session.set_data("clean", "true");
    session.save_session("git_workflow");

    assert!(session.has_session("git_workflow"));
}

// ==================== E2E: Development Session Persistence ====================

#[test]
fn test_e2e_development_session_persistence() {
    // Simulate a complete development session with state persistence

    // Morning session
    let mut session_morning = SessionManager::new();
    let mut chat_morning = ChatPanel::new();
    let mut status_morning = GitStatusPanel::new();

    // Developer starts working on a feature
    session_morning.set_data("current_file", "src/feature.rs");
    session_morning.set_data("cursor_line", "150");
    session_morning.set_metadata("branch", "feature/new-widget");

    // Has conversation with AI about implementation
    chat_morning.add_user_message("How should I structure this widget?");
    chat_morning.add_assistant_message("I recommend using the builder pattern...");
    chat_morning.add_user_message("Can you show an example?");
    chat_morning.add_assistant_message("Sure! Here's an example: ...");

    // Check git status
    status_morning.set_branch("feature/new-widget");
    status_morning.add_file("src/feature.rs", FileStatus::Modified);

    // Save session before lunch
    session_morning.set_data("chat_messages", chat_morning.message_count().to_string());
    session_morning.set_data("uncommitted_files", status_morning.file_count().to_string());
    session_morning.save_session("feature_work");

    // === Lunch break - app closed ===

    // Afternoon session - restore state
    let mut session_afternoon = SessionManager::new();

    // Simulate loading session from disk (here we just copy the session)
    let saved_session = session_morning.get_session("feature_work").unwrap().clone();
    session_afternoon.save_session("feature_work"); // Register the session
    session_afternoon.load_session("feature_work");

    // Manually restore state (in real app, this would be automatic)
    for (k, v) in saved_session.data() {
        session_afternoon.set_data(k, v);
    }

    // Verify state restored
    assert_eq!(
        session_afternoon.get_data("current_file"),
        Some("src/feature.rs")
    );
    assert_eq!(session_afternoon.get_data("cursor_line"), Some("150"));
    assert_eq!(session_afternoon.get_data("chat_messages"), Some("4"));
    assert_eq!(session_afternoon.get_data("uncommitted_files"), Some("1"));
}

// ==================== E2E: Bug Investigation and Fix Workflow ====================

#[test]
fn test_e2e_bug_investigation_workflow() {
    let mut chat = ChatPanel::new();
    let mut diff_viewer = GitDiffViewer::new();
    let mut status = GitStatusPanel::new();
    let mut graph = GitGraph::new();
    let mut token_counter = TokenCounter::new();

    // Step 1: Bug reported - view current code via git diff
    status.set_branch("main");
    let current_code_diff = r#"diff --git a/src/calculator.rs b/src/calculator.rs
index aaa..bbb 100644
--- a/src/calculator.rs
+++ b/src/calculator.rs
@@ -5,7 +5,7 @@
 pub fn divide(a: i32, b: i32) -> i32 {
-    a / b
+    a / b  // BUG: No zero check!
 }
"#;
    diff_viewer.set_diff(current_code_diff);

    // Step 2: Ask AI about the bug
    chat.add_user_message("I have a divide function that crashes on zero. How should I fix it?");
    chat.add_assistant_message(
        "You should check for zero division and return a Result instead. Here's how...",
    );

    token_counter.add_usage(TokenUsage::new(50, 100));

    // Step 3: Implement fix
    status.add_file("src/calculator.rs", FileStatus::Modified);

    let fix_diff = r#"diff --git a/src/calculator.rs b/src/calculator.rs
index aaa..bbb 100644
--- a/src/calculator.rs
+++ b/src/calculator.rs
@@ -5,7 +5,11 @@
-pub fn divide(a: i32, b: i32) -> i32 {
-    a / b
+pub fn divide(a: i32, b: i32) -> Result<i32, String> {
+    if b == 0 {
+        return Err("Division by zero".to_string());
+    }
+    Ok(a / b)
 }
"#;
    diff_viewer.set_diff(fix_diff);
    let (additions, deletions, _) = diff_viewer.stats();
    assert!(additions > 0); // Added error handling
    assert!(deletions > 0); // Removed old code

    // Step 4: Ask AI about tests
    chat.add_user_message("What tests should I add for this?");
    chat.add_assistant_message("You should test: normal division, division by zero, ...");

    token_counter.add_usage(TokenUsage::new(40, 120));

    // Step 5: Add tests
    status.add_file("tests/calculator_test.rs", FileStatus::Untracked);

    // Step 6: Stage and commit
    status.clear();
    status.add_file("src/calculator.rs", FileStatus::Staged);
    status.add_file("tests/calculator_test.rs", FileStatus::Staged);

    graph.add_commit(
        GitCommit::new("fix123", "Fix division by zero bug")
            .with_author("Developer")
            .with_branch("main"),
    );

    // Verify workflow completed
    assert_eq!(chat.message_count(), 4);
    assert!(token_counter.session_cost() > 0.0);
    assert_eq!(graph.commit_count(), 1);
}

// ==================== E2E: Multi-Workspace Development ====================

#[test]
fn test_e2e_multi_workspace_workflow() {
    let mut session = SessionManager::new();

    // Workspace 1: Backend API work
    let mut backend_chat = ChatPanel::new();
    let mut backend_status = GitStatusPanel::new();

    backend_chat.add_user_message("How do I implement REST endpoints in Rust?");
    backend_chat.add_assistant_message("You can use axum or actix-web...");

    backend_status.set_branch("backend/api");
    backend_status.add_file("src/api/routes.rs", FileStatus::Modified);
    backend_status.add_file("src/api/handlers.rs", FileStatus::Modified);

    session.set_data("workspace", "backend");
    session.set_data("branch", "backend/api");
    session.set_data("files_modified", backend_status.file_count().to_string());
    session.set_data("chat_active", "true");
    session.save_session("backend_workspace");

    // Workspace 2: Frontend work
    let mut frontend_chat = ChatPanel::new();
    let mut frontend_status = GitStatusPanel::new();

    session.clear_data();

    frontend_chat.add_user_message("Best way to handle forms in React?");
    frontend_chat.add_assistant_message("I recommend using React Hook Form...");

    frontend_status.set_branch("frontend/ui");
    frontend_status.add_file("src/components/Form.tsx", FileStatus::Modified);

    session.set_data("workspace", "frontend");
    session.set_data("branch", "frontend/ui");
    session.set_data("files_modified", frontend_status.file_count().to_string());
    session.save_session("frontend_workspace");

    // Switch back to backend
    assert!(session.load_session("backend_workspace"));
    assert_eq!(session.get_data("workspace"), Some("backend"));
    assert_eq!(session.get_data("branch"), Some("backend/api"));
    assert_eq!(session.get_data("files_modified"), Some("2"));

    // Switch to frontend
    assert!(session.load_session("frontend_workspace"));
    assert_eq!(session.get_data("workspace"), Some("frontend"));
    assert_eq!(session.get_data("branch"), Some("frontend/ui"));
    assert_eq!(session.get_data("files_modified"), Some("1"));

    assert_eq!(session.session_count(), 2);
}

// ==================== E2E: Feature Branch Workflow ====================

#[test]
fn test_e2e_feature_branch_complete_workflow() {
    let mut status = GitStatusPanel::new();
    let mut graph = GitGraph::new();
    let mut diff_viewer = GitDiffViewer::new();
    let mut chat = ChatPanel::new();
    let mut session = SessionManager::new();

    // Step 1: Start from main branch
    status.set_branch("main");
    graph.add_commit(GitCommit::new("main1", "Latest main commit").with_branch("main"));

    // Step 2: Create feature branch
    status.set_branch("feature/add-logging");

    // Step 3: Ask AI about implementation
    chat.add_user_message("What's the best logging library for Rust?");
    chat.add_assistant_message("I recommend using `tracing` because...");

    // Step 4: Implement feature
    status.add_file("Cargo.toml", FileStatus::Modified);
    status.add_file("src/logging.rs", FileStatus::Untracked);

    let logging_diff = r#"diff --git a/src/logging.rs b/src/logging.rs
new file mode 100644
index 0000000..1234567
--- /dev/null
+++ b/src/logging.rs
@@ -0,0 +1,5 @@
+use tracing::info;
+
+pub fn setup_logging() {
+    info!("Logging initialized");
+}
"#;
    diff_viewer.set_diff(logging_diff);

    // Step 5: Commit on feature branch
    status.clear();
    status.add_file("Cargo.toml", FileStatus::Staged);
    status.add_file("src/logging.rs", FileStatus::Staged);

    graph.add_commit(
        GitCommit::new("feat1", "Add logging infrastructure")
            .with_branch("feature/add-logging")
            .with_parent("main1")
            .with_author("Developer"),
    );

    // Step 6: More feature work
    chat.add_user_message("How do I add log levels?");
    chat.add_assistant_message("You can use info!, warn!, error! macros...");

    status.add_file("src/logging.rs", FileStatus::Modified);
    graph.add_commit(
        GitCommit::new("feat2", "Add log level support")
            .with_branch("feature/add-logging")
            .with_parent("feat1"),
    );

    // Step 7: Merge to main
    status.set_branch("main");
    status.clear();

    graph.add_commit(
        GitCommit::new("merge1", "Merge feature/add-logging into main")
            .with_branch("main")
            .with_parent("main1")
            .with_merge_parent("feat2"),
    );

    // Verify complete workflow
    assert_eq!(graph.commit_count(), 4); // main1, feat1, feat2, merge1
    assert_eq!(chat.message_count(), 4);

    // Save session
    session.set_data("feature_completed", "true");
    session.set_data("commits_created", "3");
    session.save_session("feature_add_logging");

    assert!(session.has_session("feature_add_logging"));
}

// ==================== E2E: Cost-Aware AI Interaction ====================

#[test]
fn test_e2e_cost_aware_ai_workflow() {
    let mut chat = ChatPanel::new();
    let mut token_counter = TokenCounter::new().with_budget(5.0);
    let mut model_selector = ModelSelector::new().with_models(vec![
        ModelInfo::new("claude-haiku-4", "Claude Haiku 4", "Anthropic"),
        ModelInfo::new("claude-sonnet-4.5", "Claude Sonnet 4.5", "Anthropic"),
    ]);

    // Start with cheaper model for simple questions
    assert_eq!(
        model_selector.selected_model().unwrap().id,
        "claude-haiku-4"
    );
    token_counter.set_cost_model(CostModel::claude_haiku_4());

    // Simple questions with Haiku
    chat.add_user_message("What is a hashmap?");
    chat.add_assistant_message("A hashmap is a data structure...");
    token_counter.add_usage(TokenUsage::new(20, 50));

    chat.add_user_message("How do I use it?");
    chat.add_assistant_message("You can use HashMap::new()...");
    token_counter.add_usage(TokenUsage::new(15, 40));

    let haiku_cost = token_counter.session_cost();
    assert!(haiku_cost < 1.0); // Should be very cheap

    // Complex question - switch to Sonnet
    model_selector.next();
    assert_eq!(
        model_selector.selected_model().unwrap().id,
        "claude-sonnet-4.5"
    );
    token_counter.set_cost_model(CostModel::claude_sonnet_4_5());

    chat.add_user_message("Explain advanced async patterns with Pin and Unpin");
    chat.add_assistant_message("Advanced async in Rust involves understanding...");
    token_counter.add_usage(TokenUsage::new(100, 500));

    let total_cost = token_counter.session_cost();
    assert!(total_cost > haiku_cost);
    assert!(total_cost < 5.0); // Still under budget

    // Verify cost tracking
    assert_eq!(chat.message_count(), 6);
}

// ==================== E2E: Refactoring Workflow ====================

#[test]
fn test_e2e_refactoring_workflow() {
    let mut chat = ChatPanel::new();
    let mut status = GitStatusPanel::new();
    let mut diff_viewer = GitDiffViewer::new();
    let mut graph = GitGraph::new();

    // Step 1: Identify code that needs refactoring
    chat.add_user_message("This function is too complex. How should I refactor it?");
    chat.add_assistant_message(
        "Consider extracting helper methods and using the builder pattern...",
    );

    // Step 2: Make refactoring changes
    status.set_branch("refactor/simplify-api");
    status.add_file("src/api.rs", FileStatus::Modified);
    status.add_file("src/api/helpers.rs", FileStatus::Untracked);

    // Step 3: View diff to review changes
    let refactor_diff = r#"diff --git a/src/api.rs b/src/api.rs
index aaa..bbb 100644
--- a/src/api.rs
+++ b/src/api.rs
@@ -10,15 +10,7 @@
-pub fn complex_function(a: i32, b: i32, c: i32) -> Result<i32> {
-    // 50 lines of complex logic
-}
+pub fn simple_function(a: i32, b: i32, c: i32) -> Result<i32> {
+    helpers::validate(a, b, c)?;
+    helpers::process(a, b, c)
+}
"#;
    diff_viewer.set_diff(refactor_diff);
    let (additions, deletions, _) = diff_viewer.stats();
    assert!(additions > 0 && deletions > 0); // Refactoring changed code

    // Step 4: Ask AI about test updates
    chat.add_user_message("Do I need to update tests after this refactoring?");
    chat.add_assistant_message(
        "Yes, you should update tests to verify the new helper functions...",
    );

    // Step 5: Update tests
    status.add_file("tests/api_test.rs", FileStatus::Modified);
    status.add_file("tests/helpers_test.rs", FileStatus::Untracked);

    // Step 6: Commit refactoring
    status.clear();
    status.add_file("src/api.rs", FileStatus::Staged);
    status.add_file("src/api/helpers.rs", FileStatus::Staged);
    status.add_file("tests/api_test.rs", FileStatus::Staged);
    status.add_file("tests/helpers_test.rs", FileStatus::Staged);

    graph.add_commit(
        GitCommit::new("refactor1", "Refactor: Simplify API with helper functions")
            .with_author("Developer")
            .with_branch("refactor/simplify-api"),
    );

    assert_eq!(chat.message_count(), 4);
    assert_eq!(graph.commit_count(), 1);
}
