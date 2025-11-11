//! Integration tests for Platinum TUI widgets
//!
//! Tests cross-widget interactions and workflows specific to platinum features.
//! These tests validate that platinum widgets work correctly when used together.

use toad::ui::widgets::chat_panel::{ChatPanel};
use toad::ui::widgets::git::git_diff_viewer::{GitDiffViewer};
use toad::ui::widgets::progress::spinner::{Spinner, SpinnerStyle};
use toad::ui::widgets::progress::token_counter::{CostModel, TokenCounter, TokenUsage};
use toad::ui::widgets::selection::model_selector::{ModelInfo, ModelSelector};

// =============================================================================
// AI WORKFLOW INTEGRATION TESTS
// =============================================================================

#[test]
fn test_ai_workflow_chat_with_token_tracking() {
    // Simulates AI chat session with cost tracking
    let mut chat = ChatPanel::new();
    let mut counter = TokenCounter::new();

    // User asks a question
    chat.add_user_message("Explain async/await in Rust");
    assert_eq!(chat.message_count(), 1);

    // Simulate AI response with token usage
    chat.start_streaming();
    chat.append_streaming("Async/await in Rust provides...");
    chat.finish_streaming();

    // Track token usage
    counter.add_usage(TokenUsage::new(50, 150)); // Input: 50, Output: 150
    assert!(counter.session_cost() > 0.0);

    // User follows up
    chat.add_user_message("Can you show an example?");
    chat.start_streaming();
    chat.append_streaming("Sure! Here's an example:\n```rust\nasync fn example() {}\n```");
    chat.finish_streaming();

    // Track more usage
    counter.add_usage(TokenUsage::new(30, 200));

    // Verify chat state
    assert_eq!(chat.message_count(), 4);

    // Verify cumulative token tracking
    assert!(counter.session_cost() > 0.0);
    assert!(counter.total_cost() == counter.session_cost());
}

#[test]
fn test_ai_workflow_model_selection_affects_cost() {
    let mut counter1 = TokenCounter::new();
    let mut counter2 = TokenCounter::new();
    let usage = TokenUsage::new(1_000_000, 1_000_000);

    // Same usage with different models
    counter1.set_cost_model(CostModel::claude_haiku_4());
    counter2.set_cost_model(CostModel::claude_opus_4());

    counter1.add_usage(usage.clone());
    counter2.add_usage(usage);

    let haiku_cost = counter1.session_cost();
    let opus_cost = counter2.session_cost();

    // Opus should be significantly more expensive
    assert!(opus_cost > haiku_cost * 10.0);
    assert_eq!(haiku_cost, 1.5); // Haiku: 0.25 + 1.25
    assert_eq!(opus_cost, 90.0); // Opus: 15.0 + 75.0
}

#[test]
fn test_ai_workflow_model_selector_integration() {
    // Use with_models to replace defaults
    let models = vec![
        ModelInfo::new("haiku", "Claude Haiku 4", "Anthropic")
            .with_cost(0.5)
            .with_speed(3.0),
        ModelInfo::new("sonnet", "Claude Sonnet 4.5", "Anthropic")
            .with_cost(2.0)
            .with_speed(2.0),
        ModelInfo::new("opus", "Claude Opus 4", "Anthropic")
            .with_cost(5.0)
            .with_speed(1.0),
    ];

    let mut selector = ModelSelector::new().with_models(models);

    // Select fastest model (Haiku)
    selector.select(0);
    assert_eq!(selector.selected_model().unwrap().id, "haiku");

    // Navigate to slowest model (Opus)
    selector.next();
    selector.next();
    assert_eq!(selector.selected_model().unwrap().id, "opus");

    // Filter models (set to Some)
    selector.set_filter(Some("sonnet".to_string()));
    // Selection still persists
    assert_eq!(selector.selected_model().unwrap().id, "opus");
}

#[test]
fn test_ai_workflow_streaming_response_with_spinner() {
    let mut chat = ChatPanel::new();
    let mut spinner = Spinner::new(SpinnerStyle::Dots);

    // User sends message
    chat.add_user_message("Generate a function");

    // Start streaming response (spinner should be active)
    chat.start_streaming();

    // Simulate streaming chunks with spinner animation
    for chunk in &["Generating", " your", " function", "..."] {
        chat.append_streaming(chunk);
        spinner.tick(); // Spinner animates during streaming
    }

    // Finish streaming
    chat.finish_streaming();

    // Verify message added correctly
    assert_eq!(chat.message_count(), 2);

    // Spinner should have advanced
    assert!(spinner.current_frame() > 0);
}

// =============================================================================
// GIT WORKFLOW INTEGRATION TESTS
// =============================================================================

#[test]
fn test_git_workflow_diff_viewer_with_multiple_files() {
    let mut viewer = GitDiffViewer::new();

    let diff = r#"diff --git a/src/main.rs b/src/main.rs
+fn main() {
+    println!("Hello");
+}
diff --git a/src/lib.rs b/src/lib.rs
-pub fn old_function() {}
+pub fn new_function() {}
"#;

    viewer.set_diff(diff);

    // Verify multi-file diff parsed
    // Lines: 2 headers + 3 additions in main.rs + 1 deletion + 1 addition in lib.rs = 7
    assert_eq!(viewer.line_count(), 7);

    let (additions, deletions, _) = viewer.stats();
    assert_eq!(additions, 4); // 3 in main.rs + 1 in lib.rs
    assert_eq!(deletions, 1); // 1 in lib.rs
}

#[test]
fn test_git_workflow_diff_filtering_by_file() {
    let mut viewer = GitDiffViewer::new();

    let diff = r#"diff --git a/src/main.rs b/src/main.rs
+fn main() {}
diff --git a/src/lib.rs b/src/lib.rs
+pub fn helper() {}
diff --git a/tests/test.rs b/tests/test.rs
+#[test]
+fn test_it() {}
"#;

    // Filter to only show tests/test.rs
    viewer.set_diff_for_file(diff, "tests/test.rs");

    // Should only have test file changes
    let (additions, _, _) = viewer.stats();
    assert_eq!(additions, 2); // Only test.rs additions

    // Verify correct file shown
    assert_eq!(viewer.line_count(), 3); // header + 2 additions
}

#[test]
fn test_git_workflow_diff_viewer_builder() {
    let viewer = GitDiffViewer::new()
        .with_line_numbers(true)
        .with_syntax_highlighting(true)
        .with_compact(false);

    // Verify configuration applied
    assert_eq!(viewer.line_count(), 0); // No diff yet

    // Can set diff after creation
    let mut viewer = viewer;
    viewer.set_diff("+added line");
    assert_eq!(viewer.line_count(), 1);
}

// =============================================================================
// CROSS-WIDGET STATE MANAGEMENT TESTS
// =============================================================================

#[test]
fn test_chat_panel_state_persistence_across_clears() {
    let mut chat = ChatPanel::new();

    // Build up conversation
    for i in 0..10 {
        chat.add_user_message(format!("Q{}", i));
        chat.add_assistant_message(format!("A{}", i));
    }

    assert_eq!(chat.message_count(), 20);

    // Clear
    chat.clear();
    assert_eq!(chat.message_count(), 0);

    // Can still add messages after clear
    chat.add_user_message("New conversation");
    assert_eq!(chat.message_count(), 1);
}

#[test]
fn test_token_counter_session_vs_total_tracking() {
    let mut counter = TokenCounter::new();

    // Session 1
    counter.add_usage(TokenUsage::new(1000, 500));
    let session1_cost = counter.session_cost();
    let total1_cost = counter.total_cost();
    assert_eq!(session1_cost, total1_cost);

    // Reset session (simulates new session)
    counter.reset_session();
    assert_eq!(counter.session_cost(), 0.0);
    assert_eq!(counter.total_cost(), total1_cost); // Total preserved

    // Session 2
    counter.add_usage(TokenUsage::new(500, 250));
    assert!(counter.session_cost() > 0.0);
    assert!(counter.total_cost() > total1_cost); // Cumulative
}

#[test]
fn test_git_diff_viewer_file_filter_state() {
    let mut viewer = GitDiffViewer::new();

    let diff = r#"diff --git a/a.txt b/a.txt
+a
diff --git a/b.txt b/b.txt
+b
"#;

    // Filter to file
    viewer.set_diff_for_file(diff, "a.txt");
    assert_eq!(viewer.line_count(), 2);

    // Clear removes filter
    viewer.clear();
    assert_eq!(viewer.line_count(), 0);

    // Can set diff again without filter
    viewer.set_diff(diff);
    assert_eq!(viewer.line_count(), 4); // All lines
}

#[test]
fn test_model_selector_selection_persists_through_filter() {
    let models = vec![
        ModelInfo::new("haiku", "Haiku", "Anthropic"),
        ModelInfo::new("sonnet", "Sonnet", "Anthropic"),
        ModelInfo::new("opus", "Opus", "Anthropic"),
    ];

    let mut selector = ModelSelector::new().with_models(models);

    // Select second model
    selector.select(1);
    assert_eq!(selector.selected_model().unwrap().id, "sonnet");

    // Apply filter
    selector.set_filter(Some("opus".to_string()));

    // Selection still points to sonnet even though it doesn't match filter
    assert_eq!(selector.selected_model().unwrap().id, "sonnet");
}

// =============================================================================
// ERROR HANDLING AND EDGE CASES
// =============================================================================

#[test]
fn test_chat_panel_streaming_edge_cases() {
    let mut chat = ChatPanel::new();

    // Start streaming
    chat.start_streaming();

    // Append empty string
    chat.append_streaming("");

    // Append multiple times
    chat.append_streaming("Part 1");
    chat.append_streaming(" Part 2");
    chat.append_streaming(" Part 3");

    // Finish
    chat.finish_streaming();

    assert_eq!(chat.message_count(), 1);
}

#[test]
fn test_token_counter_budget_warnings() {
    // Test under budget
    let mut counter1 = TokenCounter::new().with_budget(1.0);
    counter1.add_usage(TokenUsage::new(50_000, 25_000));
    let cost1 = counter1.session_cost();
    assert!(cost1 > 0.0 && cost1 < 1.0, "Cost should be under budget");

    // Test at budget
    let mut counter2 = TokenCounter::new().with_budget(1.0);
    counter2.add_usage(TokenUsage::new(200_000, 50_000)); // ~$1.35
    let cost2 = counter2.session_cost();
    assert!(cost2 >= 1.0, "Cost should be at or over budget");

    // Test way over budget
    let mut counter3 = TokenCounter::new().with_budget(1.0);
    counter3.add_usage(TokenUsage::new(1_000_000, 500_000));
    let cost3 = counter3.session_cost();
    assert!(cost3 > 5.0, "Cost should be significantly over budget");
}

#[test]
fn test_git_diff_viewer_empty_and_clear() {
    let mut viewer = GitDiffViewer::new();

    // Empty from start
    assert_eq!(viewer.line_count(), 0);
    let (a, d, c) = viewer.stats();
    assert_eq!((a, d, c), (0, 0, 0));

    // Set diff
    viewer.set_diff("+line");
    assert_eq!(viewer.line_count(), 1);

    // Clear
    viewer.clear();
    assert_eq!(viewer.line_count(), 0);

    // Stats reset
    let (a, d, c) = viewer.stats();
    assert_eq!((a, d, c), (0, 0, 0));
}

#[test]
fn test_spinner_style_changes() {
    let mut spinner = Spinner::new(SpinnerStyle::Dots);

    // Tick a few times
    spinner.tick();
    spinner.tick();
    spinner.tick();
    assert_eq!(spinner.current_frame(), 3);

    // Change style resets frame
    spinner.set_style(SpinnerStyle::Binary);
    assert_eq!(spinner.current_frame(), 0);

    // Verify new style works
    spinner.tick();
    assert_eq!(spinner.current_frame(), 1);
    assert_eq!(spinner.current_symbol(), "1");
}

// =============================================================================
// PERFORMANCE AND STRESS TESTS
// =============================================================================

#[test]
fn test_chat_panel_performance_with_max_history() {
    let mut chat = ChatPanel::new().with_max_history(1000);

    // Add 2000 messages (should trim to 1000)
    for i in 0..2000 {
        chat.add_user_message(format!("Message {}", i));
    }

    // Should be capped
    assert_eq!(chat.message_count(), 1000);
}

#[test]
fn test_git_diff_viewer_performance_with_many_hunks() {
    let mut viewer = GitDiffViewer::new();

    let mut diff = String::new();
    diff.push_str("diff --git a/file.txt b/file.txt\n");

    // Add 100 hunks
    for i in 0..100 {
        diff.push_str(&format!("@@ -{},{} +{},{} @@\n", i * 10, 5, i * 10, 6));
        diff.push_str(" context\n");
        diff.push_str(&format!("+added in hunk {}\n", i));
        diff.push_str(" more context\n");
    }

    viewer.set_diff(&diff);

    // Should parse all hunks
    let (additions, _, _) = viewer.stats();
    assert_eq!(additions, 100);
}

#[test]
fn test_model_selector_navigation_wraparound() {
    let models = vec![
        ModelInfo::new("m1", "Model 1", "Provider"),
        ModelInfo::new("m2", "Model 2", "Provider"),
        ModelInfo::new("m3", "Model 3", "Provider"),
    ];

    let mut selector = ModelSelector::new().with_models(models);

    // Start at 0
    selector.select(0);
    assert_eq!(selector.selected_model().unwrap().id, "m1");

    // Go backwards (should wrap to last)
    selector.previous();
    assert_eq!(selector.selected_model().unwrap().id, "m3");

    // Go forward from last (should wrap to first)
    selector.next();
    assert_eq!(selector.selected_model().unwrap().id, "m1");
}

// =============================================================================
// WIDGET BUILDER PATTERN TESTS
// =============================================================================

#[test]
fn test_chat_panel_builder() {
    let chat = ChatPanel::new()
        .with_max_history(100)
        .with_user_color(ratatui::style::Color::Cyan)
        .with_assistant_color(ratatui::style::Color::Green)
        .with_system_color(ratatui::style::Color::Yellow);

    assert_eq!(chat.message_count(), 0);
}

#[test]
fn test_token_counter_builder() {
    let counter = TokenCounter::new().with_budget(10.0).with_compact(true);

    assert_eq!(counter.session_cost(), 0.0);
}

#[test]
fn test_spinner_builder() {
    let spinner = Spinner::new(SpinnerStyle::Arrows)
        .label("Loading...")
        .color(ratatui::style::Color::Green);

    assert_eq!(spinner.current_frame(), 0);
    assert_eq!(spinner.style(), SpinnerStyle::Arrows);
}

#[test]
fn test_git_diff_viewer_builder_chain() {
    let viewer = GitDiffViewer::new()
        .with_title("My Changes")
        .with_line_numbers(false)
        .with_syntax_highlighting(false)
        .with_compact(true);

    assert_eq!(viewer.line_count(), 0);
}

#[test]
fn test_model_selector_builder() {
    let models = vec![
        ModelInfo::new("m1", "Model 1", "Provider"),
        ModelInfo::new("m2", "Model 2", "Provider"),
    ];

    let selector = ModelSelector::new().with_models(models);

    assert_eq!(selector.selected_model().unwrap().id, "m1");
}

// =============================================================================
// INTEGRATION SCENARIO TESTS
// =============================================================================

#[test]
fn test_complete_ai_chat_workflow() {
    // Complete AI chat workflow with cost tracking and model selection
    let mut chat = ChatPanel::new();
    let mut counter = TokenCounter::new().with_budget(1.0);

    let models = vec![
        ModelInfo::new("haiku", "Claude Haiku", "Anthropic")
            .with_cost(0.5)
            .with_speed(3.0),
    ];
    let mut selector = ModelSelector::new().with_models(models);

    // Select model
    selector.select(0);
    let model = selector.selected_model().unwrap();
    assert_eq!(model.id, "haiku");

    // User conversation
    chat.add_user_message("Help me write a function");
    chat.start_streaming();
    chat.append_streaming("Sure! Here's a function...");
    chat.finish_streaming();

    // Track tokens
    counter.add_usage(TokenUsage::new(100, 200));

    // Verify workflow completed
    assert_eq!(chat.message_count(), 2);
    assert!(counter.session_cost() > 0.0);
    assert!(counter.session_cost() < 1.0); // Under budget
}

#[test]
fn test_complete_git_diff_workflow() {
    // Complete git diff workflow with filtering and stats
    let mut viewer = GitDiffViewer::new()
        .with_line_numbers(true)
        .with_syntax_highlighting(true);

    let diff = r#"diff --git a/src/main.rs b/src/main.rs
@@ -1,3 +1,4 @@
 fn main() {
+    println!("Debug");
     println!("Hello");
 }
diff --git a/src/lib.rs b/src/lib.rs
@@ -1,2 +1,3 @@
 pub fn lib() {
+    // New comment
 }
"#;

    // Set full diff
    viewer.set_diff(diff);
    let (total_add, total_del, _) = viewer.stats();
    assert_eq!(total_add, 2);
    assert_eq!(total_del, 0);

    // Filter to specific file
    viewer.set_diff_for_file(diff, "src/main.rs");
    let (file_add, _, _) = viewer.stats();
    assert_eq!(file_add, 1);

    // Clear and verify
    viewer.clear();
    assert_eq!(viewer.line_count(), 0);
}
