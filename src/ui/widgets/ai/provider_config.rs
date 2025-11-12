//! Provider configuration panel widget
//!
//! Displays status and configuration for all LLM providers including health checks,
//! connection status, and provider switching capabilities.
//!
//! # Features
//!
//! - Multi-provider status display (Anthropic, GitHub, Ollama)
//! - Real-time health checks
//! - Connection status indicators (Connected, Not configured, Rate limited, Error)
//! - Provider switching with fallback support
//! - Credential status (without exposing secrets)
//!
//! # Examples
//!
//! ```no_run
//! use toad::ui::widgets::ai::ProviderConfigPanel;
//!
//! let panel = ProviderConfigPanel::new();
//! ```

use crate::ai::llm::provider::{ProviderConfig, ProviderType};
use crate::ui::atoms::Block;
use crate::ui::theme::ToadTheme;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{List, ListItem, ListState, Paragraph},
    Frame,
};
use std::time::{Duration, Instant};

/// Provider connection status
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProviderStatus {
    /// Connected and operational
    Connected,
    /// Not configured (missing API key or endpoint)
    NotConfigured,
    /// Rate limited (429 errors)
    RateLimited,
    /// Error state (authentication failed, network error, etc.)
    Error,
    /// Health check in progress
    Checking,
}

impl ProviderStatus {
    /// Get color for status
    pub fn color(&self) -> Color {
        match self {
            ProviderStatus::Connected => ToadTheme::TOAD_GREEN,
            ProviderStatus::NotConfigured => ToadTheme::GRAY,
            ProviderStatus::RateLimited => ToadTheme::WARNING,
            ProviderStatus::Error => ToadTheme::ERROR,
            ProviderStatus::Checking => ToadTheme::BLUE,
        }
    }

    /// Get symbol for status
    pub fn symbol(&self) -> &'static str {
        match self {
            ProviderStatus::Connected => "●",
            ProviderStatus::NotConfigured => "○",
            ProviderStatus::RateLimited => "◐",
            ProviderStatus::Error => "✗",
            ProviderStatus::Checking => "⟳",
        }
    }

    /// Get status text
    pub fn text(&self) -> &'static str {
        match self {
            ProviderStatus::Connected => "Connected",
            ProviderStatus::NotConfigured => "Not Configured",
            ProviderStatus::RateLimited => "Rate Limited",
            ProviderStatus::Error => "Error",
            ProviderStatus::Checking => "Checking...",
        }
    }
}

/// Provider entry with status and configuration
#[derive(Debug, Clone)]
pub struct ProviderEntry {
    /// Provider type
    pub provider_type: ProviderType,
    /// Current connection status
    pub status: ProviderStatus,
    /// Whether this is the active provider
    pub is_active: bool,
    /// Available models for this provider
    pub models: Vec<String>,
    /// Current model
    pub current_model: Option<String>,
    /// Last health check time
    pub last_health_check: Option<Instant>,
    /// Error message if status is Error
    pub error_message: Option<String>,
    /// Whether API key is configured
    pub has_api_key: bool,
    /// Base URL (for Ollama)
    pub base_url: Option<String>,
}

impl ProviderEntry {
    /// Create a new provider entry
    pub fn new(provider_type: ProviderType) -> Self {
        Self {
            provider_type,
            status: ProviderStatus::NotConfigured,
            is_active: false,
            models: Vec::new(),
            current_model: None,
            last_health_check: None,
            error_message: None,
            has_api_key: false,
            base_url: None,
        }
    }

    /// Create entry from configuration
    pub fn from_config(config: &ProviderConfig, is_active: bool) -> Self {
        let has_api_key = config.api_key.is_some();
        let models = Self::get_default_models(&config.provider);

        Self {
            provider_type: config.provider.clone(),
            status: if has_api_key || config.provider == ProviderType::Ollama {
                ProviderStatus::Connected
            } else {
                ProviderStatus::NotConfigured
            },
            is_active,
            models,
            current_model: Some(config.model.clone()),
            last_health_check: None,
            error_message: None,
            has_api_key,
            base_url: config.base_url.clone(),
        }
    }

    /// Get default models for a provider
    fn get_default_models(provider: &ProviderType) -> Vec<String> {
        match provider {
            ProviderType::Anthropic => vec![
                "claude-sonnet-4-5-20250929".to_string(),
                "claude-3-7-sonnet-20250219".to_string(),
                "claude-3-5-sonnet-20241022".to_string(),
                "claude-3-5-haiku-20241022".to_string(),
                "claude-3-opus-20240229".to_string(),
            ],
            ProviderType::GitHub => vec![
                "gpt-4o".to_string(),
                "gpt-4o-mini".to_string(),
                "o1-preview".to_string(),
                "o1-mini".to_string(),
            ],
            ProviderType::Ollama => vec![
                "llama3.2".to_string(),
                "llama3.1".to_string(),
                "codellama".to_string(),
                "mistral".to_string(),
                "qwen2.5-coder".to_string(),
            ],
        }
    }

    /// Get provider display name
    pub fn name(&self) -> &'static str {
        match self.provider_type {
            ProviderType::Anthropic => "Anthropic (Claude)",
            ProviderType::GitHub => "GitHub Models",
            ProviderType::Ollama => "Ollama (Local)",
        }
    }

    /// Get context window for current model
    pub fn context_window(&self) -> Option<String> {
        if let Some(ref model) = self.current_model {
            match self.provider_type {
                ProviderType::Anthropic => {
                    if model.contains("sonnet-4") || model.contains("3-7") {
                        Some("200K tokens".to_string())
                    } else if model.contains("3-5") || model.contains("3-opus") {
                        Some("200K tokens".to_string())
                    } else if model.contains("haiku") {
                        Some("200K tokens".to_string())
                    } else {
                        Some("200K tokens".to_string())
                    }
                }
                ProviderType::GitHub => {
                    if model.contains("gpt-4o") {
                        Some("128K tokens".to_string())
                    } else if model.contains("o1") {
                        Some("128K tokens".to_string())
                    } else {
                        Some("128K tokens".to_string())
                    }
                }
                ProviderType::Ollama => Some("Varies by model".to_string()),
            }
        } else {
            None
        }
    }
}

/// Provider configuration panel widget
///
/// Displays status, configuration, and health information for all LLM providers.
///
/// # Examples
///
/// ```
/// use toad::ui::widgets::ai::ProviderConfigPanel;
///
/// let panel = ProviderConfigPanel::new();
/// assert_eq!(panel.provider_count(), 3);
/// ```
#[derive(Debug)]
pub struct ProviderConfigPanel {
    /// Provider entries
    providers: Vec<ProviderEntry>,
    /// Selected provider index
    selected_index: usize,
    /// List state for rendering
    list_state: ListState,
    /// Last global health check
    last_health_check: Option<Instant>,
    /// Auto-failover enabled
    auto_failover: bool,
}

impl Default for ProviderConfigPanel {
    fn default() -> Self {
        Self::new()
    }
}

impl ProviderConfigPanel {
    /// Create a new provider configuration panel
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::widgets::ai::ProviderConfigPanel;
    ///
    /// let panel = ProviderConfigPanel::new();
    /// ```
    pub fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        // Initialize with all providers
        let providers = vec![
            ProviderEntry::new(ProviderType::Anthropic),
            ProviderEntry::new(ProviderType::GitHub),
            ProviderEntry::new(ProviderType::Ollama),
        ];

        Self {
            providers,
            selected_index: 0,
            list_state,
            last_health_check: None,
            auto_failover: true,
        }
    }

    /// Initialize from provider configuration
    pub fn from_config(config: &ProviderConfig) -> Self {
        let mut panel = Self::new();
        panel.update_from_config(config);
        panel
    }

    /// Update panel from provider configuration
    pub fn update_from_config(&mut self, config: &ProviderConfig) {
        for provider in &mut self.providers {
            if provider.provider_type == config.provider {
                *provider = ProviderEntry::from_config(config, true);
            } else {
                provider.is_active = false;
            }
        }
    }

    /// Get number of providers
    pub fn provider_count(&self) -> usize {
        self.providers.len()
    }

    /// Select next provider
    pub fn select_next(&mut self) {
        if !self.providers.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.providers.len();
            self.list_state.select(Some(self.selected_index));
        }
    }

    /// Select previous provider
    pub fn select_previous(&mut self) {
        if !self.providers.is_empty() {
            self.selected_index = if self.selected_index == 0 {
                self.providers.len() - 1
            } else {
                self.selected_index - 1
            };
            self.list_state.select(Some(self.selected_index));
        }
    }

    /// Get selected provider
    pub fn selected_provider(&self) -> Option<&ProviderEntry> {
        self.providers.get(self.selected_index)
    }

    /// Get selected provider (mutable)
    pub fn selected_provider_mut(&mut self) -> Option<&mut ProviderEntry> {
        self.providers.get_mut(self.selected_index)
    }

    /// Toggle auto-failover
    pub fn toggle_auto_failover(&mut self) {
        self.auto_failover = !self.auto_failover;
    }

    /// Mark provider as active
    pub fn set_active_provider(&mut self, provider_type: ProviderType) {
        for provider in &mut self.providers {
            provider.is_active = provider.provider_type == provider_type;
        }
    }

    /// Update provider status
    pub fn update_provider_status(
        &mut self,
        provider_type: ProviderType,
        status: ProviderStatus,
        error: Option<String>,
    ) {
        if let Some(provider) = self
            .providers
            .iter_mut()
            .find(|p| p.provider_type == provider_type)
        {
            provider.status = status;
            provider.error_message = error;
            provider.last_health_check = Some(Instant::now());
        }
    }

    /// Run health check on all providers
    pub fn health_check_all(&mut self) {
        for provider in &mut self.providers {
            provider.status = ProviderStatus::Checking;
        }
        self.last_health_check = Some(Instant::now());
    }

    /// Get active provider type
    pub fn active_provider(&self) -> Option<ProviderType> {
        self.providers
            .iter()
            .find(|p| p.is_active)
            .map(|p| p.provider_type.clone())
    }

    /// Render the provider configuration panel
    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        // Split into header and content
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Min(0),    // Provider list
                Constraint::Length(4), // Footer with actions
            ])
            .split(area);

        self.render_header(frame, chunks[0]);
        self.render_provider_list(frame, chunks[1]);
        self.render_footer(frame, chunks[2]);
    }

    /// Render header
    fn render_header(&self, frame: &mut Frame, area: Rect) {
        let block = Block::themed("Provider Configuration").to_ratatui();
        let inner = block.inner(area);
        frame.render_widget(block, area);

        let status_text = if self.auto_failover {
            "Auto-failover: ON"
        } else {
            "Auto-failover: OFF"
        };

        let health_check_text = if let Some(last_check) = self.last_health_check {
            let elapsed = last_check.elapsed();
            if elapsed < Duration::from_secs(60) {
                format!("Last check: {}s ago", elapsed.as_secs())
            } else {
                format!("Last check: {}m ago", elapsed.as_secs() / 60)
            }
        } else {
            "Not checked".to_string()
        };

        let line = Line::from(vec![
            Span::styled(status_text, Style::default().fg(ToadTheme::TOAD_GREEN)),
            Span::raw(" | "),
            Span::styled(health_check_text, Style::default().fg(ToadTheme::GRAY)),
        ]);

        let paragraph = Paragraph::new(line);
        frame.render_widget(paragraph, inner);
    }

    /// Render provider list
    fn render_provider_list(&mut self, frame: &mut Frame, area: Rect) {
        let block = Block::themed("Providers").to_ratatui();
        let inner = block.inner(area);
        frame.render_widget(block, area);

        if self.providers.is_empty() {
            let text = Paragraph::new("No providers configured");
            frame.render_widget(text, inner);
            return;
        }

        let items: Vec<ListItem> = self
            .providers
            .iter()
            .map(|provider| {
                let mut spans = vec![];

                // Active indicator
                if provider.is_active {
                    spans.push(Span::styled(
                        "[ACTIVE] ",
                        Style::default()
                            .fg(ToadTheme::TOAD_GREEN)
                            .add_modifier(Modifier::BOLD),
                    ));
                } else {
                    spans.push(Span::raw("         "));
                }

                // Status indicator
                spans.push(Span::styled(
                    format!("{} ", provider.status.symbol()),
                    Style::default().fg(provider.status.color()),
                ));

                // Provider name
                spans.push(Span::styled(
                    provider.name(),
                    Style::default()
                        .fg(ToadTheme::FOREGROUND)
                        .add_modifier(Modifier::BOLD),
                ));

                // Model
                if let Some(ref model) = provider.current_model {
                    spans.push(Span::styled(
                        format!(" - {}", model),
                        Style::default().fg(ToadTheme::GRAY),
                    ));
                }

                // Context window
                if let Some(context) = provider.context_window() {
                    spans.push(Span::styled(
                        format!(" ({})", context),
                        Style::default().fg(ToadTheme::GRAY),
                    ));
                }

                ListItem::new(Line::from(spans))
            })
            .collect();

        let list = List::new(items)
            .highlight_style(
                Style::default()
                    .bg(ToadTheme::DARK_GRAY)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("▶ ");

        frame.render_stateful_widget(list, inner, &mut self.list_state);
    }

    /// Render footer with actions
    fn render_footer(&self, frame: &mut Frame, area: Rect) {
        let block = Block::themed("Actions").to_ratatui();
        let inner = block.inner(area);
        frame.render_widget(block, area);

        let mut lines = vec![];

        // Key bindings
        lines.push(Line::from(vec![
            Span::styled("↑/↓", Style::default().fg(ToadTheme::BLUE)),
            Span::raw(" Navigate | "),
            Span::styled("Enter", Style::default().fg(ToadTheme::BLUE)),
            Span::raw(" Activate | "),
            Span::styled("h", Style::default().fg(ToadTheme::BLUE)),
            Span::raw(" Health Check | "),
            Span::styled("f", Style::default().fg(ToadTheme::BLUE)),
            Span::raw(" Toggle Failover"),
        ]));

        // Selected provider details
        if let Some(provider) = self.selected_provider() {
            let status_line = format!(
                "Status: {} | API Key: {}",
                provider.status.text(),
                if provider.has_api_key { "✓" } else { "✗" }
            );

            lines.push(Line::from(Span::styled(
                status_line,
                Style::default().fg(ToadTheme::GRAY),
            )));
        }

        let paragraph = Paragraph::new(lines);
        frame.render_widget(paragraph, inner);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_status_color() {
        assert_eq!(ProviderStatus::Connected.color(), ToadTheme::TOAD_GREEN);
        assert_eq!(ProviderStatus::Error.color(), ToadTheme::ERROR);
    }

    #[test]
    fn test_provider_entry_new() {
        let entry = ProviderEntry::new(ProviderType::Anthropic);
        assert_eq!(entry.provider_type, ProviderType::Anthropic);
        assert_eq!(entry.status, ProviderStatus::NotConfigured);
        assert!(!entry.is_active);
    }

    #[test]
    fn test_provider_panel_new() {
        let panel = ProviderConfigPanel::new();
        assert_eq!(panel.provider_count(), 3);
        assert_eq!(panel.selected_index, 0);
    }

    #[test]
    fn test_select_next_previous() {
        let mut panel = ProviderConfigPanel::new();
        assert_eq!(panel.selected_index, 0);

        panel.select_next();
        assert_eq!(panel.selected_index, 1);

        panel.select_next();
        assert_eq!(panel.selected_index, 2);

        panel.select_next(); // Wraps around
        assert_eq!(panel.selected_index, 0);

        panel.select_previous();
        assert_eq!(panel.selected_index, 2);
    }

    #[test]
    fn test_set_active_provider() {
        let mut panel = ProviderConfigPanel::new();
        panel.set_active_provider(ProviderType::GitHub);

        let active = panel.active_provider();
        assert_eq!(active, Some(ProviderType::GitHub));
    }

    #[test]
    fn test_toggle_auto_failover() {
        let mut panel = ProviderConfigPanel::new();
        assert!(panel.auto_failover);

        panel.toggle_auto_failover();
        assert!(!panel.auto_failover);

        panel.toggle_auto_failover();
        assert!(panel.auto_failover);
    }

    #[test]
    fn test_update_provider_status() {
        let mut panel = ProviderConfigPanel::new();
        panel.update_provider_status(
            ProviderType::Anthropic,
            ProviderStatus::Error,
            Some("Test error".to_string()),
        );

        let provider = panel
            .providers
            .iter()
            .find(|p| p.provider_type == ProviderType::Anthropic)
            .unwrap();

        assert_eq!(provider.status, ProviderStatus::Error);
        assert_eq!(provider.error_message, Some("Test error".to_string()));
        assert!(provider.last_health_check.is_some());
    }

    #[test]
    fn test_from_config() {
        let config = ProviderConfig::anthropic("claude-sonnet-4-5-20250929")
            .with_api_key("test-key");
        let panel = ProviderConfigPanel::from_config(&config);

        let anthropic = panel
            .providers
            .iter()
            .find(|p| p.provider_type == ProviderType::Anthropic)
            .unwrap();

        assert!(anthropic.is_active);
        assert_eq!(
            anthropic.current_model,
            Some("claude-sonnet-4-5-20250929".to_string())
        );
        assert!(anthropic.has_api_key);
    }
}
