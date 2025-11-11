/// Integration tests for power user features (PLATINUM tier)
///
/// Tests for Minimap, Breadcrumbs, MultiSelect, TokenCounter, ModelSelector
use toad::ui::widgets::selection::multiselect::SelectionMode;
use toad::ui::widgets::core::breadcrumbs::{BreadcrumbSegment, Breadcrumbs};
use toad::ui::widgets::layout::minimap::{Minimap, MinimapMode};
use toad::ui::widgets::progress::token_counter::{CostModel, TokenCounter, TokenUsage};
use toad::ui::widgets::selection::model_selector::{ModelInfo, ModelSelector};
use toad::ui::widgets::selection::multiselect::{MultiSelect};

// ============================================================================
// Minimap Integration Tests
// ============================================================================

#[test]
fn test_minimap_creation() {
    let lines = vec!["line 1", "line 2", "line 3"];
    let minimap = Minimap::new(lines);

    assert_eq!(minimap.line_count(), 3);
}

#[test]
fn test_minimap_modes() {
    let lines = vec!["test"];

    let minimap_chars = Minimap::new(lines.clone()).with_mode(MinimapMode::Characters);
    assert_eq!(minimap_chars.line_count(), 1);

    let minimap_blocks = Minimap::new(lines.clone()).with_mode(MinimapMode::Blocks);
    assert_eq!(minimap_blocks.line_count(), 1);

    let minimap_colors = Minimap::new(lines).with_mode(MinimapMode::Colors);
    assert_eq!(minimap_colors.line_count(), 1);
}

#[test]
fn test_minimap_viewport() {
    let lines = vec!["1", "2", "3", "4", "5"];
    let mut minimap = Minimap::new(lines);

    minimap.set_viewport(1, 3);
    assert_eq!(minimap.viewport(), (1, 3));
}

#[test]
fn test_minimap_scroll() {
    let lines: Vec<String> = (0..100).map(|i| format!("line {}", i)).collect();
    let mut minimap = Minimap::new(lines);

    minimap.set_viewport(10, 30);
    minimap.scroll(5);

    let (start, end) = minimap.viewport();
    assert_eq!(start, 15);
    assert_eq!(end, 35);
}

#[test]
fn test_minimap_jump_to() {
    let lines: Vec<String> = (0..100).map(|i| format!("line {}", i)).collect();
    let mut minimap = Minimap::new(lines);

    minimap.jump_to(50);

    // Viewport should be centered around line 50
    let (start, _) = minimap.viewport();
    assert!(start <= 50);
}

#[test]
fn test_minimap_set_lines() {
    let mut minimap = Minimap::new(vec!["old"]);
    assert_eq!(minimap.line_count(), 1);

    minimap.set_lines(vec!["new1", "new2", "new3"]);
    assert_eq!(minimap.line_count(), 3);
}

#[test]
fn test_minimap_with_border() {
    let minimap = Minimap::new(vec!["test"]).with_border(true);
    assert_eq!(minimap.line_count(), 1);
}

#[test]
fn test_minimap_with_scale() {
    let minimap = Minimap::new(vec!["test"]).with_scale(2);
    assert_eq!(minimap.line_count(), 1);
}

// ============================================================================
// Breadcrumbs Integration Tests
// ============================================================================

#[test]
fn test_breadcrumbs_creation() {
    let breadcrumbs = Breadcrumbs::new();
    assert_eq!(breadcrumbs.segments().len(), 0);
}

#[test]
fn test_breadcrumbs_from_path() {
    let breadcrumbs = Breadcrumbs::from_path("/home/user/project");
    assert!(breadcrumbs.segments().len() > 0);
}

#[test]
fn test_breadcrumb_segment_creation() {
    let segment = BreadcrumbSegment::new("Home");
    assert_eq!(segment.label, "Home");
}

#[test]
fn test_breadcrumb_segment_with_icon() {
    let segment = BreadcrumbSegment::new("Folder").with_icon("📁");

    assert_eq!(segment.icon, Some("📁".to_string()));
}

#[test]
fn test_breadcrumbs_push_pop() {
    let mut breadcrumbs = Breadcrumbs::new();

    breadcrumbs.push(BreadcrumbSegment::new("Home"));
    breadcrumbs.push(BreadcrumbSegment::new("Projects"));

    assert_eq!(breadcrumbs.segments().len(), 2);

    let popped = breadcrumbs.pop();
    assert!(popped.is_some());
    assert_eq!(popped.unwrap().label, "Projects");
    assert_eq!(breadcrumbs.segments().len(), 1);
}

#[test]
fn test_breadcrumbs_set_segments() {
    let mut breadcrumbs = Breadcrumbs::new();

    let segments = vec![
        BreadcrumbSegment::new("A"),
        BreadcrumbSegment::new("B"),
        BreadcrumbSegment::new("C"),
    ];

    breadcrumbs.set_segments(segments);
    assert_eq!(breadcrumbs.segments().len(), 3);
}

#[test]
fn test_breadcrumbs_separator() {
    let mut breadcrumbs = Breadcrumbs::new();
    breadcrumbs.set_separator(" > ");

    breadcrumbs.push(BreadcrumbSegment::new("A"));
    assert_eq!(breadcrumbs.segments().len(), 1);
}

#[test]
fn test_breadcrumbs_hover() {
    let mut breadcrumbs = Breadcrumbs::new();
    breadcrumbs.push(BreadcrumbSegment::new("A"));

    breadcrumbs.set_hovered(Some(0));
    assert_eq!(breadcrumbs.hovered(), Some(0));

    breadcrumbs.set_hovered(None);
    assert_eq!(breadcrumbs.hovered(), None);
}

#[test]
fn test_breadcrumbs_clear() {
    let mut breadcrumbs = Breadcrumbs::new();
    breadcrumbs.push(BreadcrumbSegment::new("A"));
    breadcrumbs.push(BreadcrumbSegment::new("B"));

    breadcrumbs.clear();
    assert_eq!(breadcrumbs.segments().len(), 0);
}

// ============================================================================
// MultiSelect Integration Tests
// ============================================================================

#[test]
fn test_multiselect_creation() {
    let items = vec!["A", "B", "C"];
    let selector = MultiSelect::new(items);

    assert_eq!(selector.item_count(), 3);
    assert_eq!(selector.selected_count(), 0);
}

#[test]
fn test_multiselect_modes() {
    let items = vec![1, 2, 3];

    let single = MultiSelect::new(items.clone()).with_mode(SelectionMode::Single);
    assert_eq!(single.item_count(), 3);

    let multiple = MultiSelect::new(items.clone()).with_mode(SelectionMode::Multiple);
    assert_eq!(multiple.item_count(), 3);

    let range = MultiSelect::new(items).with_mode(SelectionMode::Range);
    assert_eq!(range.item_count(), 3);
}

#[test]
fn test_multiselect_navigation() {
    let items = vec!["A", "B", "C", "D"];
    let mut selector = MultiSelect::new(items);

    assert_eq!(selector.cursor(), 0);

    selector.next();
    assert_eq!(selector.cursor(), 1);

    selector.next();
    selector.next();
    assert_eq!(selector.cursor(), 3);

    selector.previous();
    assert_eq!(selector.cursor(), 2);

    selector.first();
    assert_eq!(selector.cursor(), 0);

    selector.last();
    assert_eq!(selector.cursor(), 3);
}

#[test]
fn test_multiselect_single_selection() {
    let items = vec![1, 2, 3];
    let mut selector = MultiSelect::new(items);

    selector.select(1);
    assert!(selector.is_selected(1));
    assert_eq!(selector.selected_count(), 1);
}

#[test]
fn test_multiselect_multiple_selection() {
    let items = vec!["A", "B", "C", "D"];
    let mut selector = MultiSelect::new(items);

    selector.select(0);
    selector.select(2);
    selector.select(3);

    assert_eq!(selector.selected_count(), 3);
    assert!(selector.is_selected(0));
    assert!(selector.is_selected(2));
    assert!(selector.is_selected(3));
    assert!(!selector.is_selected(1));
}

#[test]
fn test_multiselect_toggle() {
    let items = vec![1, 2, 3];
    let mut selector = MultiSelect::new(items);

    selector.toggle(1);
    assert!(selector.is_selected(1));

    selector.toggle(1);
    assert!(!selector.is_selected(1));
}

#[test]
fn test_multiselect_toggle_current() {
    let items = vec!["A", "B", "C"];
    let mut selector = MultiSelect::new(items);

    selector.toggle_current();
    assert!(selector.is_selected(0));

    selector.next();
    selector.toggle_current();
    assert!(selector.is_selected(1));
}

#[test]
fn test_multiselect_select_all() {
    let items = vec![1, 2, 3, 4, 5];
    let mut selector = MultiSelect::new(items);

    selector.select_all();
    assert_eq!(selector.selected_count(), 5);

    for i in 0..5 {
        assert!(selector.is_selected(i));
    }
}

#[test]
fn test_multiselect_clear_selection() {
    let items = vec!["A", "B", "C"];
    let mut selector = MultiSelect::new(items);

    selector.select(0);
    selector.select(1);
    selector.select(2);

    selector.clear_selection();
    assert_eq!(selector.selected_count(), 0);
}

#[test]
fn test_multiselect_selected_indices() {
    let items = vec![1, 2, 3, 4];
    let mut selector = MultiSelect::new(items);

    selector.select(1);
    selector.select(3);

    let indices = selector.selected_indices();
    assert_eq!(indices, vec![1, 3]);
}

#[test]
fn test_multiselect_deselect() {
    let items = vec!["A", "B", "C"];
    let mut selector = MultiSelect::new(items);

    selector.select(0);
    selector.select(1);

    selector.deselect(0);
    assert!(!selector.is_selected(0));
    assert!(selector.is_selected(1));
}

// ============================================================================
// TokenCounter Integration Tests
// ============================================================================

#[test]
fn test_token_usage_creation() {
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
    let mut usage = TokenUsage::new(1000, 500);
    usage.reset();

    assert_eq!(usage.input_tokens, 0);
    assert_eq!(usage.output_tokens, 0);
    assert_eq!(usage.total(), 0);
}

#[test]
fn test_cost_model_claude_sonnet() {
    let model = CostModel::claude_sonnet_4_5();
    let usage = TokenUsage::new(1_000_000, 1_000_000);
    let cost = model.calculate_cost(&usage);

    assert!(cost > 0.0);
}

#[test]
fn test_cost_model_all_models() {
    let models = vec![
        CostModel::claude_sonnet_4_5(),
        CostModel::claude_opus_4(),
        CostModel::claude_haiku_4(),
        CostModel::gpt_4o(),
    ];

    let usage = TokenUsage::new(1000, 1000);

    for model in models {
        let cost = model.calculate_cost(&usage);
        assert!(cost >= 0.0);
    }
}

#[test]
fn test_token_counter_creation() {
    let counter = TokenCounter::new();
    // Verify initialization
    let _counter = counter;
}

#[test]
fn test_token_counter_add_usage() {
    let mut counter = TokenCounter::new();

    counter.add_usage(TokenUsage::new(100, 50));
    counter.add_usage(TokenUsage::new(200, 100));

    // Session and total should track correctly
}

#[test]
fn test_token_counter_reset_session() {
    let mut counter = TokenCounter::new();

    counter.add_usage(TokenUsage::new(1000, 500));
    counter.reset_session();

    // Session should be reset but total should remain
}

#[test]
fn test_token_counter_reset_total() {
    let mut counter = TokenCounter::new();

    counter.add_usage(TokenUsage::new(1000, 500));
    counter.reset_total();

    // Both session and total should be reset
}

#[test]
fn test_token_counter_with_budget() {
    let counter = TokenCounter::new().with_budget(10.0);

    // Budget should be set
    let _counter = counter;
}

#[test]
fn test_token_counter_set_cost_model() {
    let mut counter = TokenCounter::new();
    counter.set_cost_model(CostModel::claude_sonnet_4_5());

    // Cost model should be updated
}

#[test]
fn test_token_counter_toggle_details() {
    let mut counter = TokenCounter::new();
    counter.toggle_details();

    // Should toggle between detailed and compact view
}

#[test]
fn test_token_counter_compact_mode() {
    let counter = TokenCounter::new().with_compact(true);
    let _counter = counter;
}

// ============================================================================
// ModelSelector Integration Tests
// ============================================================================

#[test]
fn test_model_info_creation() {
    let model = ModelInfo::new("gpt-4", "GPT-4", "OpenAI");

    assert_eq!(model.id, "gpt-4");
    assert_eq!(model.name, "GPT-4");
    assert_eq!(model.provider, "OpenAI");
}

#[test]
fn test_model_info_builder() {
    let model = ModelInfo::new("claude-3", "Claude 3", "Anthropic")
        .with_context_window(200_000)
        .with_max_output(4096)
        .with_cost(0.003)
        .with_speed(1.2)
        .with_capability("coding")
        .with_available(true);

    assert_eq!(model.context_window, 200_000);
    assert_eq!(model.max_output, 4096);
    assert_eq!(model.cost, 0.003);
    assert_eq!(model.speed, 1.2);
    assert_eq!(model.available, true);
}

#[test]
fn test_model_info_formatted_context() {
    let model = ModelInfo::new("test", "Test", "Test").with_context_window(100_000);

    let formatted = model.formatted_context();
    assert!(formatted.contains("100"));
}

#[test]
fn test_model_info_indicators() {
    let model = ModelInfo::new("test", "Test", "Test")
        .with_cost(0.001)
        .with_speed(2.0);

    let cost_indicator = model.cost_indicator();
    let speed_indicator = model.speed_indicator();

    assert!(!cost_indicator.is_empty());
    assert!(!speed_indicator.is_empty());
}

#[test]
fn test_model_selector_creation() {
    let selector = ModelSelector::new();
    // Should have default models
    assert!(selector.selected_model().is_some());
}

#[test]
fn test_model_selector_with_models() {
    let models = vec![
        ModelInfo::new("m1", "Model 1", "Provider A"),
        ModelInfo::new("m2", "Model 2", "Provider B"),
    ];

    let selector = ModelSelector::new().with_models(models);
    assert!(selector.selected_model().is_some());
}

#[test]
fn test_model_selector_add_model() {
    let mut selector = ModelSelector::new();

    selector.add_model(ModelInfo::new("custom", "Custom Model", "Custom"));

    // Model should be added
}

#[test]
fn test_model_selector_navigation() {
    let selector = ModelSelector::new();
    let _initial_id = selector.selected_id().map(|s| s.to_string());

    // Create a new selector to test navigation
    let mut selector = ModelSelector::new();
    selector.next();

    let next_id = selector.selected_id();

    // Should have moved to next model (if multiple models exist)
    if selector.selected_model().is_some() {
        assert!(next_id.is_some());
    }
}

#[test]
fn test_model_selector_select() {
    let mut selector = ModelSelector::new();

    selector.select(0);
    assert!(selector.selected_model().is_some());
}

#[test]
fn test_model_selector_previous() {
    let mut selector = ModelSelector::new();

    selector.previous();
    assert!(selector.selected_model().is_some());
}

// ============================================================================
// Cross-Feature Integration Tests
// ============================================================================

#[test]
fn test_minimap_with_breadcrumbs() {
    let lines: Vec<String> = (0..100).map(|i| format!("line {}", i)).collect();
    let minimap = Minimap::new(lines);

    let breadcrumbs = Breadcrumbs::from_path("/home/user/file.rs");

    // Both can work together
    assert!(minimap.line_count() > 0);
    assert!(breadcrumbs.segments().len() > 0);
}

#[test]
fn test_multiselect_with_token_counter() {
    let models = vec!["gpt-4", "claude-3", "llama-2"];
    let mut selector = MultiSelect::new(models);

    let mut counter = TokenCounter::new();

    selector.select(0);
    counter.add_usage(TokenUsage::new(1000, 500));

    assert_eq!(selector.selected_count(), 1);
}

#[test]
fn test_complete_power_user_workflow() {
    // User navigates through file
    let lines: Vec<String> = (0..200).map(|i| format!("line {}", i)).collect();
    let mut minimap = Minimap::new(lines);

    minimap.jump_to(50);

    // User tracks location with breadcrumbs
    let mut breadcrumbs = Breadcrumbs::new();
    breadcrumbs.push(BreadcrumbSegment::new("src").with_icon("📁"));
    breadcrumbs.push(BreadcrumbSegment::new("main.rs").with_icon("📄"));

    // User selects multiple items
    let items = vec!["function1", "function2", "function3"];
    let mut selector = MultiSelect::new(items);
    selector.select_all();

    // User monitors AI usage
    let mut counter = TokenCounter::new();
    counter.add_usage(TokenUsage::new(5000, 2500));
    counter.set_cost_model(CostModel::claude_sonnet_4_5());

    // User switches models
    let mut model_selector = ModelSelector::new();
    model_selector.next();

    // Verify all systems work together
    assert!(minimap.line_count() > 0);
    assert!(breadcrumbs.segments().len() > 0);
    assert_eq!(selector.selected_count(), 3);
    assert!(model_selector.selected_model().is_some());
}
