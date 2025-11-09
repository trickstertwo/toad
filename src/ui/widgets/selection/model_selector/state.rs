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

