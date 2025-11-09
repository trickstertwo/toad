//! Model Selector widget for switching between AI models
//!
//! Displays available AI models with their capabilities, pricing, and performance characteristics.

use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState},
};
use serde::{Deserialize, Serialize};

/// AI model information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    /// Model identifier
    pub id: String,
    /// Display name
    pub name: String,
    /// Model provider (Anthropic, OpenAI, etc.)
    pub provider: String,
    /// Maximum context window (tokens)
    pub context_window: usize,
    /// Output token limit
    pub max_output: usize,
    /// Relative cost (1.0 = baseline)
    pub cost: f64,
    /// Relative speed (1.0 = baseline)
    pub speed: f64,
    /// Capabilities (coding, reasoning, etc.)
    pub capabilities: Vec<String>,
    /// Whether the model is currently available
    pub available: bool,
}

impl ModelInfo {
    /// Create a new model info
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        provider: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            provider: provider.into(),
            context_window: 200_000,
            max_output: 4096,
            cost: 1.0,
            speed: 1.0,
            capabilities: Vec::new(),
            available: true,
        }
    }

    /// Set context window
    pub fn with_context_window(mut self, tokens: usize) -> Self {
        self.context_window = tokens;
        self
    }

    /// Set max output
    pub fn with_max_output(mut self, tokens: usize) -> Self {
        self.max_output = tokens;
        self
    }

    /// Set relative cost
    pub fn with_cost(mut self, cost: f64) -> Self {
        self.cost = cost;
        self
    }

    /// Set relative speed
    pub fn with_speed(mut self, speed: f64) -> Self {
        self.speed = speed;
        self
    }

    /// Add a capability
    pub fn with_capability(mut self, capability: impl Into<String>) -> Self {
        self.capabilities.push(capability.into());
        self
    }

    /// Set availability
    pub fn with_available(mut self, available: bool) -> Self {
        self.available = available;
        self
    }

    /// Format context window as human-readable
    pub fn formatted_context(&self) -> String {
        if self.context_window >= 1_000_000 {
            format!("{}M", self.context_window / 1_000_000)
        } else if self.context_window >= 1_000 {
            format!("{}K", self.context_window / 1_000)
        } else {
            format!("{}", self.context_window)
        }
    }

    /// Get cost indicator ($ symbols)
    pub fn cost_indicator(&self) -> String {
        let level = (self.cost * 4.0).ceil() as usize;
        "$".repeat(level.clamp(1, 5))
    }

    /// Get speed indicator (⚡ symbols)
    pub fn speed_indicator(&self) -> String {
        let level = (self.speed * 3.0).ceil() as usize;
        "⚡".repeat(level.clamp(1, 3))
    }
}

/// Model selector widget
pub struct ModelSelector {
    /// Available models
    models: Vec<ModelInfo>,
    /// Currently selected index
    selected: usize,
    /// List state for rendering
    list_state: ListState,
    /// Show detailed info
    show_details: bool,
    /// Filter by capability
    filter: Option<String>,
}

impl Default for ModelSelector {
    fn default() -> Self {
        Self::new()
    }
}

impl ModelSelector {
    /// Create a new model selector
    pub fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Self {
            models: Self::default_models(),
            selected: 0,
            list_state,
            show_details: true,
            filter: None,
        }
    }

    /// Get default model list
    fn default_models() -> Vec<ModelInfo> {
        vec![
            ModelInfo::new("claude-sonnet-4.5", "Claude Sonnet 4.5", "Anthropic")
                .with_context_window(200_000)
                .with_max_output(8192)
                .with_cost(1.0)
                .with_speed(1.5)
                .with_capability("coding")
                .with_capability("reasoning")
                .with_capability("analysis"),
            ModelInfo::new("claude-opus-4", "Claude Opus 4", "Anthropic")
                .with_context_window(200_000)
                .with_max_output(4096)
                .with_cost(3.0)
                .with_speed(0.8)
                .with_capability("coding")
                .with_capability("reasoning")
                .with_capability("deep-analysis"),
            ModelInfo::new("claude-haiku-4", "Claude Haiku 4", "Anthropic")
                .with_context_window(200_000)
                .with_max_output(4096)
                .with_cost(0.2)
                .with_speed(3.0)
                .with_capability("coding")
                .with_capability("fast-responses"),
            ModelInfo::new("gpt-4o", "GPT-4o", "OpenAI")
                .with_context_window(128_000)
                .with_max_output(4096)
                .with_cost(1.5)
                .with_speed(1.2)
                .with_capability("coding")
                .with_capability("vision")
                .with_capability("reasoning"),
            ModelInfo::new("gpt-4o-mini", "GPT-4o Mini", "OpenAI")
                .with_context_window(128_000)
                .with_max_output(4096)
                .with_cost(0.3)
                .with_speed(2.5)
                .with_capability("coding")
                .with_capability("fast-responses"),
            ModelInfo::new("deepseek-coder-v2", "DeepSeek Coder V2", "DeepSeek")
                .with_context_window(128_000)
                .with_max_output(4096)
                .with_cost(0.1)
                .with_speed(2.0)
                .with_capability("coding")
                .with_capability("specialized"),
        ]
    }

    /// Set models
    pub fn with_models(mut self, models: Vec<ModelInfo>) -> Self {
        self.models = models;
        if self.selected >= self.models.len() && !self.models.is_empty() {
            self.selected = self.models.len() - 1;
            self.list_state.select(Some(self.selected));
        }
        self
    }

    /// Add a model
    pub fn add_model(&mut self, model: ModelInfo) {
        self.models.push(model);
    }

    /// Get currently selected model
    pub fn selected_model(&self) -> Option<&ModelInfo> {
        self.models.get(self.selected)
    }

    /// Get selected model ID
    pub fn selected_id(&self) -> Option<&str> {
        self.selected_model().map(|m| m.id.as_str())
    }

    /// Select next model
    pub fn next(&mut self) {
        if !self.models.is_empty() {
            self.selected = (self.selected + 1) % self.models.len();
            self.list_state.select(Some(self.selected));
        }
    }

    /// Select previous model
    pub fn previous(&mut self) {
        if !self.models.is_empty() {
            self.selected = if self.selected == 0 {
                self.models.len() - 1
            } else {
                self.selected - 1
            };
            self.list_state.select(Some(self.selected));
        }
    }

    /// Select model by index
    pub fn select(&mut self, index: usize) {
        if index < self.models.len() {
            self.selected = index;
            self.list_state.select(Some(index));
        }
    }

    /// Select model by ID
    pub fn select_by_id(&mut self, id: &str) -> bool {
        if let Some(index) = self.models.iter().position(|m| m.id == id) {
            self.select(index);
            true
        } else {
            false
        }
    }

    /// Toggle details view
    pub fn toggle_details(&mut self) {
        self.show_details = !self.show_details;
    }

    /// Set filter by capability
    pub fn set_filter(&mut self, capability: Option<String>) {
        self.filter = capability;
    }

    /// Render the model selector
    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        // Get filtered indices before borrowing
        let filtered_indices: Vec<usize> = if let Some(ref filter) = self.filter {
            self.models
                .iter()
                .enumerate()
                .filter(|(_, m)| m.capabilities.contains(filter))
                .map(|(i, _)| i)
                .collect()
        } else {
            (0..self.models.len()).collect()
        };

        // Build list items
        let items: Vec<ListItem> = filtered_indices
            .iter()
            .map(|&idx| {
                let model = &self.models[idx];
                let mut spans = Vec::new();

                // Model name
                spans.push(Span::styled(
                    &model.name,
                    Style::default()
                        .fg(if model.available {
                            Color::White
                        } else {
                            Color::DarkGray
                        })
                        .add_modifier(if idx == self.selected {
                            Modifier::BOLD
                        } else {
                            Modifier::empty()
                        }),
                ));

                spans.push(Span::raw(" "));

                // Provider
                spans.push(Span::styled(
                    format!("({})", model.provider),
                    Style::default().fg(Color::DarkGray),
                ));

                // Context window
                if self.show_details {
                    spans.push(Span::raw(" | "));
                    spans.push(Span::styled(
                        format!("{}tok", model.formatted_context()),
                        Style::default().fg(Color::Cyan),
                    ));

                    // Cost indicator
                    spans.push(Span::raw(" "));
                    spans.push(Span::styled(
                        model.cost_indicator(),
                        Style::default().fg(Color::Yellow),
                    ));

                    // Speed indicator
                    spans.push(Span::raw(" "));
                    spans.push(Span::styled(
                        model.speed_indicator(),
                        Style::default().fg(Color::Green),
                    ));
                }

                // Availability indicator
                if !model.available {
                    spans.push(Span::raw(" "));
                    spans.push(Span::styled(
                        "[unavailable]",
                        Style::default().fg(Color::Red),
                    ));
                }

                ListItem::new(Line::from(spans))
            })
            .collect();

        // Create list widget
        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(if self.show_details {
                        " Model Selector (↹ toggle details) "
                    } else {
                        " Model Selector "
                    })
                    .style(Style::default().fg(Color::White)),
            )
            .highlight_style(
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("▶ ");

        frame.render_stateful_widget(list, area, &mut self.list_state);

        // Render details panel if enabled and model selected
        if self.show_details
            && let Some(model) = self.selected_model() {
                // Show capabilities at the bottom
                let caps_text = format!("Capabilities: {}", model.capabilities.join(", "));
                let caps_line = Line::from(vec![Span::styled(
                    caps_text,
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::ITALIC),
                )]);

                // Render at bottom of area
                if area.height > 2 {
                    let caps_area = Rect {
                        x: area.x + 2,
                        y: area.y + area.height - 2,
                        width: area.width.saturating_sub(4),
                        height: 1,
                    };

                    frame.render_widget(caps_line, caps_area);
                }
            }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_info_creation() {
        let model = ModelInfo::new("test-model", "Test Model", "TestProvider")
            .with_context_window(100_000)
            .with_cost(1.5);

        assert_eq!(model.id, "test-model");
        assert_eq!(model.context_window, 100_000);
        assert_eq!(model.cost, 1.5);
        assert_eq!(model.formatted_context(), "100K");
    }

    #[test]
    fn test_model_selector() {
        let mut selector = ModelSelector::new();
        assert!(selector.selected_model().is_some());

        selector.next();
        assert_eq!(selector.selected, 1);

        selector.previous();
        assert_eq!(selector.selected, 0);
    }

    #[test]
    fn test_model_selection_by_id() {
        let mut selector = ModelSelector::new();
        assert!(selector.select_by_id("claude-opus-4"));
        assert_eq!(selector.selected_id(), Some("claude-opus-4"));
    }

    #[test]
    fn test_cost_indicator() {
        let model = ModelInfo::new("test", "Test", "Provider").with_cost(2.5);
        assert!(!model.cost_indicator().is_empty());
    }

    #[test]
    fn test_speed_indicator() {
        let model = ModelInfo::new("test", "Test", "Provider").with_speed(2.0);
        assert!(!model.speed_indicator().is_empty());
    }

    // ============================================================================
    // COMPREHENSIVE EDGE CASE TESTS (ADVANCED TIER - Advanced Input)
    // ============================================================================

    // ============ Stress Tests ============

    #[test]
    fn test_selector_1000_models() {
        let mut models = Vec::new();
        for i in 0..1000 {
            models.push(
                ModelInfo::new(
                    format!("model-{}", i),
                    format!("Model {}", i),
                    format!("Provider{}", i % 10),
                )
                .with_cost((i as f64) / 100.0),
            );
        }

        let selector = ModelSelector::new().with_models(models);
        assert_eq!(selector.models.len(), 1000);
    }

    #[test]
    fn test_selector_rapid_navigation_1000() {
        let mut selector = ModelSelector::new();
        for _ in 0..1000 {
            selector.next();
        }
        // Default has 6 models, 1000 % 6 = 4
        assert_eq!(selector.selected, 4);
    }

    #[test]
    fn test_selector_alternating_next_previous_1000() {
        let mut selector = ModelSelector::new();
        for _ in 0..1000 {
            selector.next();
            selector.previous();
        }
        // Should end up at starting position
        assert_eq!(selector.selected, 0);
    }

    #[test]
    fn test_model_many_capabilities() {
        let mut model = ModelInfo::new("test", "Test", "Provider");
        for i in 0..1000 {
            model = model.with_capability(format!("capability-{}", i));
        }
        assert_eq!(model.capabilities.len(), 1000);
    }

    #[test]
    fn test_selector_rapid_filter_changes() {
        let mut selector = ModelSelector::new();
        for i in 0..1000 {
            selector.set_filter(Some(format!("filter-{}", i)));
        }
        assert!(selector.filter.is_some());
    }

    // ============ Unicode Edge Cases ============

    #[test]
    fn test_model_unicode_name() {
        let model = ModelInfo::new("test", "日本語モデル 🚀", "Provider");
        assert_eq!(model.name, "日本語モデル 🚀");
    }

    #[test]
    fn test_model_rtl_name() {
        let model = ModelInfo::new("test", "مرحبا بك Model", "Provider");
        assert!(model.name.contains("مرحبا"));
    }

    #[test]
    fn test_model_mixed_scripts_name() {
        let model = ModelInfo::new(
            "test",
            "Hello世界Привет안녕하세요",
            "Provider",
        );
        assert!(model.name.contains("世界"));
    }

    #[test]
    fn test_model_emoji_provider() {
        let model = ModelInfo::new("test", "Test", "🐸 Anthropic 🚀");
        assert!(model.provider.contains('🐸'));
    }

    #[test]
    fn test_model_unicode_capability() {
        let model = ModelInfo::new("test", "Test", "Provider")
            .with_capability("日本語処理")
            .with_capability("🔧 coding")
            .with_capability("مرحبا");

        assert!(model.capabilities.contains(&"日本語処理".to_string()));
        assert!(model.capabilities.contains(&"🔧 coding".to_string()));
    }

    #[test]
    fn test_model_very_long_unicode_name() {
        let long_name = "日本語 ".repeat(1000);
        let model = ModelInfo::new("test", long_name.clone(), "Provider");
        assert_eq!(model.name, long_name);
    }

    // ============ Extreme Values ============

    #[test]
    fn test_model_context_window_max() {
        let model = ModelInfo::new("test", "Test", "Provider")
            .with_context_window(usize::MAX);
        assert_eq!(model.context_window, usize::MAX);
    }

    #[test]
    fn test_model_context_window_zero() {
        let model = ModelInfo::new("test", "Test", "Provider")
            .with_context_window(0);
        assert_eq!(model.context_window, 0);
    }

    #[test]
    fn test_model_max_output_extreme() {
        let model = ModelInfo::new("test", "Test", "Provider")
            .with_max_output(usize::MAX);
        assert_eq!(model.max_output, usize::MAX);
    }

    #[test]
    fn test_model_cost_zero() {
        let model = ModelInfo::new("test", "Test", "Provider").with_cost(0.0);
        assert_eq!(model.cost, 0.0);
        assert_eq!(model.cost_indicator(), "$"); // Clamps to at least 1
    }

    #[test]
    fn test_model_cost_very_high() {
        let model = ModelInfo::new("test", "Test", "Provider").with_cost(100.0);
        assert_eq!(model.cost_indicator(), "$$$$$"); // Clamps to max 5
    }

    #[test]
    fn test_model_speed_zero() {
        let model = ModelInfo::new("test", "Test", "Provider").with_speed(0.0);
        assert_eq!(model.speed, 0.0);
        assert_eq!(model.speed_indicator(), "⚡"); // Clamps to at least 1
    }

    #[test]
    fn test_model_speed_very_high() {
        let model = ModelInfo::new("test", "Test", "Provider").with_speed(100.0);
        assert_eq!(model.speed_indicator(), "⚡⚡⚡"); // Clamps to max 3
    }

    #[test]
    fn test_formatted_context_millions() {
        let model = ModelInfo::new("test", "Test", "Provider")
            .with_context_window(5_000_000);
        assert_eq!(model.formatted_context(), "5M");
    }

    #[test]
    fn test_formatted_context_thousands() {
        let model = ModelInfo::new("test", "Test", "Provider")
            .with_context_window(128_000);
        assert_eq!(model.formatted_context(), "128K");
    }

    #[test]
    fn test_formatted_context_hundreds() {
        let model = ModelInfo::new("test", "Test", "Provider")
            .with_context_window(500);
        assert_eq!(model.formatted_context(), "500");
    }

    // ============ Navigation Edge Cases ============

    #[test]
    fn test_selector_navigation_wrap_forward() {
        let mut selector = ModelSelector::new();
        let model_count = selector.models.len();

        // Navigate to last model
        for _ in 0..model_count - 1 {
            selector.next();
        }
        assert_eq!(selector.selected, model_count - 1);

        // Next should wrap to 0
        selector.next();
        assert_eq!(selector.selected, 0);
    }

    #[test]
    fn test_selector_navigation_wrap_backward() {
        let mut selector = ModelSelector::new();
        let model_count = selector.models.len();

        // At start (0), previous should wrap to last
        selector.previous();
        assert_eq!(selector.selected, model_count - 1);
    }

    #[test]
    fn test_selector_select_out_of_bounds() {
        let mut selector = ModelSelector::new();
        let original = selector.selected;

        selector.select(9999);
        // Should remain unchanged
        assert_eq!(selector.selected, original);
    }

    #[test]
    fn test_selector_select_by_invalid_id() {
        let mut selector = ModelSelector::new();
        let result = selector.select_by_id("nonexistent-model");
        assert!(!result);
    }

    #[test]
    fn test_selector_empty_models() {
        let selector = ModelSelector::new().with_models(vec![]);
        assert!(selector.selected_model().is_none());
        assert!(selector.selected_id().is_none());
    }

    #[test]
    fn test_selector_navigation_empty_models() {
        let mut selector = ModelSelector::new().with_models(vec![]);

        selector.next();
        selector.previous();

        // Should handle gracefully without panicking
        assert!(selector.selected_model().is_none());
    }

    // ============ Filtering Edge Cases ============

    #[test]
    fn test_filter_by_coding_capability() {
        let selector = ModelSelector::new();
        let coding_models = selector
            .models
            .iter()
            .filter(|m| m.capabilities.contains(&"coding".to_string()))
            .count();

        // All default models have coding capability
        assert!(coding_models > 0);
    }

    #[test]
    fn test_filter_nonexistent_capability() {
        let mut selector = ModelSelector::new();
        selector.set_filter(Some("nonexistent-capability".to_string()));

        let filtered = selector
            .models
            .iter()
            .filter(|m| {
                if let Some(ref f) = selector.filter {
                    m.capabilities.contains(f)
                } else {
                    true
                }
            })
            .count();

        assert_eq!(filtered, 0);
    }

    #[test]
    fn test_filter_clear() {
        let mut selector = ModelSelector::new();
        selector.set_filter(Some("coding".to_string()));
        assert!(selector.filter.is_some());

        selector.set_filter(None);
        assert!(selector.filter.is_none());
    }

    // ============ Builder Pattern Edge Cases ============

    #[test]
    fn test_model_info_chained_builders() {
        let model = ModelInfo::new("test", "Test", "Provider")
            .with_context_window(100_000)
            .with_max_output(4096)
            .with_cost(1.5)
            .with_speed(2.0)
            .with_capability("coding")
            .with_capability("reasoning")
            .with_available(false);

        assert_eq!(model.context_window, 100_000);
        assert_eq!(model.max_output, 4096);
        assert_eq!(model.cost, 1.5);
        assert_eq!(model.speed, 2.0);
        assert_eq!(model.capabilities.len(), 2);
        assert!(!model.available);
    }

    #[test]
    fn test_model_info_override_values() {
        let model = ModelInfo::new("test", "Test", "Provider")
            .with_cost(1.0)
            .with_cost(2.0)
            .with_cost(3.0);

        assert_eq!(model.cost, 3.0); // Last value wins
    }

    // ============ Clone/Debug/Serialize Traits ============

    #[test]
    fn test_model_info_clone() {
        let model = ModelInfo::new("test", "Test Model", "Provider")
            .with_cost(1.5)
            .with_capability("coding");

        let cloned = model.clone();
        assert_eq!(model.id, cloned.id);
        assert_eq!(model.name, cloned.name);
        assert_eq!(model.cost, cloned.cost);
        assert_eq!(model.capabilities, cloned.capabilities);
    }

    #[test]
    fn test_model_info_debug() {
        let model = ModelInfo::new("test", "Test", "Provider");
        let debug_str = format!("{:?}", model);
        assert!(debug_str.contains("ModelInfo"));
    }

    #[test]
    fn test_model_info_serialize_deserialize() {
        let model = ModelInfo::new("test-id", "Test Model", "TestProvider")
            .with_context_window(100_000)
            .with_cost(1.5)
            .with_capability("coding");

        let json = serde_json::to_string(&model).unwrap();
        let deserialized: ModelInfo = serde_json::from_str(&json).unwrap();

        assert_eq!(model.id, deserialized.id);
        assert_eq!(model.name, deserialized.name);
        assert_eq!(model.cost, deserialized.cost);
        assert_eq!(model.capabilities, deserialized.capabilities);
    }

    // ============ Complex Workflow Tests ============

    #[test]
    fn test_selector_add_remove_navigate() {
        let mut selector = ModelSelector::new();
        let original_count = selector.models.len();

        // Add a new model
        selector.add_model(
            ModelInfo::new("new-model", "New Model", "Provider")
                .with_capability("test"),
        );
        assert_eq!(selector.models.len(), original_count + 1);

        // Navigate to the new model
        selector.select(original_count);
        assert_eq!(selector.selected_id(), Some("new-model"));
    }

    #[test]
    fn test_selector_toggle_details() {
        let mut selector = ModelSelector::new();
        let initial_state = selector.show_details;

        selector.toggle_details();
        assert_eq!(selector.show_details, !initial_state);

        selector.toggle_details();
        assert_eq!(selector.show_details, initial_state);
    }

    #[test]
    fn test_selector_workflow_navigation_selection() {
        let mut selector = ModelSelector::new();

        // Navigate and select by ID
        selector.next();
        selector.next();
        let id_at_2 = selector.selected_id().unwrap().to_string();

        selector.select(0);
        assert_eq!(selector.selected, 0);

        selector.select_by_id(&id_at_2);
        assert_eq!(selector.selected, 2);
    }

    #[test]
    fn test_selector_with_unavailable_models() {
        let models = vec![
            ModelInfo::new("m1", "Model 1", "P1").with_available(true),
            ModelInfo::new("m2", "Model 2", "P2").with_available(false),
            ModelInfo::new("m3", "Model 3", "P3").with_available(true),
        ];

        let mut selector = ModelSelector::new().with_models(models);

        // Can still select unavailable models
        selector.select(1);
        assert!(selector.selected_model().is_some());
        assert!(!selector.selected_model().unwrap().available);
    }

    // ============ Comprehensive Stress Test ============

    #[test]
    fn test_comprehensive_model_selector_stress() {
        let mut selector = ModelSelector::new();

        // Phase 1: Add many models with varied configurations
        for i in 0..100 {
            let name = match i % 4 {
                0 => format!("ASCII Model {}", i),
                1 => format!("🚀 Emoji Model {}", i),
                2 => format!("日本語 Model {}", i),
                _ => format!("مرحبا Model {}", i),
            };

            let mut model = ModelInfo::new(
                format!("model-{}", i),
                name,
                format!("Provider{}", i % 5),
            )
            .with_context_window(50_000 + (i * 1000))
            .with_max_output(2048 + (i * 10))
            .with_cost((i as f64) / 20.0)
            .with_speed((i as f64) / 30.0);

            // Add capabilities
            for cap_idx in 0..(i % 5) {
                model = model.with_capability(format!("cap-{}", cap_idx));
            }

            // Some unavailable
            if i % 10 == 0 {
                model = model.with_available(false);
            }

            selector.add_model(model);
        }

        let total_models = selector.models.len(); // Default + 100
        assert!(total_models >= 100);

        // Phase 2: Navigation stress
        for _ in 0..200 {
            selector.next();
        }
        assert!(selector.selected_model().is_some());

        for _ in 0..100 {
            selector.previous();
        }
        assert!(selector.selected_model().is_some());

        // Phase 3: Direct selection
        selector.select(0);
        assert_eq!(selector.selected, 0);

        selector.select(total_models - 1);
        assert_eq!(selector.selected, total_models - 1);

        // Phase 4: Select by ID
        assert!(selector.select_by_id("model-50"));
        assert_eq!(selector.selected_id(), Some("model-50"));

        // Phase 5: Toggle details
        selector.toggle_details();
        selector.toggle_details();

        // Phase 6: Filtering
        selector.set_filter(Some("cap-1".to_string()));
        let filtered_count = selector
            .models
            .iter()
            .filter(|m| m.capabilities.contains(&"cap-1".to_string()))
            .count();
        assert!(filtered_count > 0);

        // Phase 7: Clear filter
        selector.set_filter(None);
        assert!(selector.filter.is_none());

        // Final verification
        assert!(selector.selected < total_models);
        assert!(selector.selected_model().is_some());
    }

    // ============ Default Trait Test ============

    #[test]
    fn test_selector_default() {
        let selector = ModelSelector::default();
        assert!(!selector.models.is_empty());
        assert_eq!(selector.selected, 0);
        assert!(selector.show_details);
    }

    // ============ Empty Content Edge Cases ============

    #[test]
    fn test_model_empty_id() {
        let model = ModelInfo::new("", "Name", "Provider");
        assert_eq!(model.id, "");
    }

    #[test]
    fn test_model_empty_name() {
        let model = ModelInfo::new("id", "", "Provider");
        assert_eq!(model.name, "");
    }

    #[test]
    fn test_model_empty_provider() {
        let model = ModelInfo::new("id", "Name", "");
        assert_eq!(model.provider, "");
    }

    #[test]
    fn test_model_no_capabilities() {
        let model = ModelInfo::new("id", "Name", "Provider");
        assert!(model.capabilities.is_empty());
    }

    // ============ Extreme Stress Tests (10k operations) ============

    #[test]
    fn test_selector_10k_next_navigation() {
        let mut selector = ModelSelector::new();
        for _ in 0..10000 {
            selector.next();
        }
        // Should still be functional
        assert!(selector.selected_model().is_some());
    }

    #[test]
    fn test_selector_10k_previous_navigation() {
        let mut selector = ModelSelector::new();
        for _ in 0..10000 {
            selector.previous();
        }
        assert!(selector.selected_model().is_some());
    }

    #[test]
    fn test_selector_10k_mixed_operations() {
        let mut selector = ModelSelector::new();
        for i in 0..10000 {
            match i % 4 {
                0 => selector.next(),
                1 => selector.previous(),
                2 => selector.toggle_details(),
                _ => {
                    selector.select(i % selector.models.len());
                }
            }
        }
        assert!(selector.selected_model().is_some());
    }

    #[test]
    fn test_model_10k_capability_additions() {
        let mut model = ModelInfo::new("test", "Test", "Provider");
        for i in 0..10000 {
            model = model.with_capability(format!("cap{}", i));
        }
        assert_eq!(model.capabilities.len(), 10000);
    }

    // ============ Selection State Preservation ============

    #[test]
    fn test_selector_state_after_model_replacement() {
        let mut selector = ModelSelector::new();
        selector.select(2);
        assert_eq!(selector.selected, 2);

        // Replace with fewer models
        let new_models = vec![
            ModelInfo::new("m1", "Model 1", "P1"),
        ];
        selector = selector.with_models(new_models);

        // Should adjust selection to valid index
        assert_eq!(selector.selected, 0);
    }

    #[test]
    fn test_selector_state_after_model_expansion() {
        let mut selector = ModelSelector::new();
        selector.select(1);

        // Add more models
        for i in 0..50 {
            selector.add_model(ModelInfo::new(format!("m{}", i), format!("Model {}", i), "P"));
        }

        // Selection should remain at 1
        assert_eq!(selector.selected, 1);
    }

    // ============ Capability Filtering Edge Cases ============

    #[test]
    fn test_filter_with_empty_string() {
        let mut selector = ModelSelector::new();
        selector.set_filter(Some("".to_string()));
        assert_eq!(selector.filter, Some("".to_string()));
    }

    #[test]
    fn test_filter_with_unicode_capability() {
        let mut selector = ModelSelector::new();
        selector.add_model(
            ModelInfo::new("test", "Test", "Provider")
                .with_capability("日本語処理")
        );

        selector.set_filter(Some("日本語処理".to_string()));

        let filtered_count = selector
            .models
            .iter()
            .filter(|m| {
                if let Some(ref f) = selector.filter {
                    m.capabilities.contains(f)
                } else {
                    true
                }
            })
            .count();

        assert_eq!(filtered_count, 1);
    }

    #[test]
    fn test_filter_case_sensitivity() {
        let selector = ModelSelector::new();
        // Filter is case-sensitive by default
        let filtered = selector
            .models
            .iter()
            .filter(|m| m.capabilities.contains(&"CODING".to_string()))
            .count();

        // Should be 0 because capabilities use lowercase "coding"
        assert_eq!(filtered, 0);
    }

    // ============ Indicator Edge Cases ============

    #[test]
    fn test_cost_indicator_boundary_values() {
        // Test exact boundary values for cost indicator
        let model1 = ModelInfo::new("t", "T", "P").with_cost(0.25); // Should be $
        let model2 = ModelInfo::new("t", "T", "P").with_cost(0.5);  // Should be $$
        let model3 = ModelInfo::new("t", "T", "P").with_cost(1.0);  // Should be $$$$
        let model4 = ModelInfo::new("t", "T", "P").with_cost(1.25); // Should be $$$$$

        assert_eq!(model1.cost_indicator(), "$");
        assert_eq!(model2.cost_indicator(), "$$");
        assert_eq!(model3.cost_indicator(), "$$$$");
        assert_eq!(model4.cost_indicator(), "$$$$$");
    }

    #[test]
    fn test_speed_indicator_boundary_values() {
        let model1 = ModelInfo::new("t", "T", "P").with_speed(0.33); // ceil(0.33*3) = ceil(0.99) = 1 → ⚡
        let model2 = ModelInfo::new("t", "T", "P").with_speed(0.66); // ceil(0.66*3) = ceil(1.98) = 2 → ⚡⚡
        let model3 = ModelInfo::new("t", "T", "P").with_speed(1.0);  // ceil(1.0*3) = ceil(3.0) = 3 → ⚡⚡⚡

        assert_eq!(model1.speed_indicator(), "⚡");
        assert_eq!(model2.speed_indicator(), "⚡⚡");
        assert_eq!(model3.speed_indicator(), "⚡⚡⚡");
    }

    // ============ Multi-Phase Comprehensive Workflow (10 phases) ============

    #[test]
    fn test_selector_10_phase_comprehensive_workflow() {
        let mut selector = ModelSelector::new();

        // Phase 1: Initial state verification
        assert!(selector.selected_model().is_some());
        assert_eq!(selector.selected, 0);
        assert!(selector.show_details);

        // Phase 2: Add custom models with unicode
        for i in 0..10 {
            selector.add_model(
                ModelInfo::new(format!("custom-{}", i), format!("日本語 Model {}", i), "🚀 Provider")
                    .with_capability("coding")
                    .with_capability(format!("cap-{}", i))
            );
        }
        let total_models = selector.models.len();
        assert!(total_models >= 10);

        // Phase 3: Navigation stress
        for _ in 0..100 {
            selector.next();
        }
        assert!(selector.selected < total_models);

        // Phase 4: Navigate to specific index
        selector.select(total_models / 2);
        assert_eq!(selector.selected, total_models / 2);

        // Phase 5: Toggle details multiple times
        selector.toggle_details();
        assert!(!selector.show_details);
        selector.toggle_details();
        assert!(selector.show_details);

        // Phase 6: Select by ID
        assert!(selector.select_by_id("custom-5"));
        assert!(selector.selected_id().unwrap().contains("custom-5"));

        // Phase 7: Apply filter
        selector.set_filter(Some("cap-3".to_string()));
        let filtered = selector
            .models
            .iter()
            .filter(|m| m.capabilities.contains(&"cap-3".to_string()))
            .count();
        assert_eq!(filtered, 1);

        // Phase 8: Clear filter
        selector.set_filter(None);
        assert!(selector.filter.is_none());

        // Phase 9: Backward navigation
        for _ in 0..50 {
            selector.previous();
        }
        assert!(selector.selected < total_models);

        // Phase 10: Final state verification
        assert!(selector.selected_model().is_some());
        selector.select(0);
        assert_eq!(selector.selected, 0);
    }

    // ============ Context Window Formatting Edge Cases ============

    #[test]
    fn test_formatted_context_exactly_1m() {
        let model = ModelInfo::new("t", "T", "P").with_context_window(1_000_000);
        assert_eq!(model.formatted_context(), "1M");
    }

    #[test]
    fn test_formatted_context_exactly_1k() {
        let model = ModelInfo::new("t", "T", "P").with_context_window(1_000);
        assert_eq!(model.formatted_context(), "1K");
    }

    #[test]
    fn test_formatted_context_999() {
        let model = ModelInfo::new("t", "T", "P").with_context_window(999);
        assert_eq!(model.formatted_context(), "999");
    }

    // ============ Clone Independence ============

    #[test]
    fn test_model_info_clone_independence() {
        let mut model1 = ModelInfo::new("test", "Test", "Provider")
            .with_capability("coding");

        let mut model2 = model1.clone();

        // Modify model2
        model2 = model2.with_capability("reasoning");

        // model1 should be unchanged (capabilities should be different)
        assert_eq!(model1.capabilities.len(), 1);
        assert_eq!(model2.capabilities.len(), 2);
    }
}
