//! Feature flags visualization and management widget
//!
//! Provides an interactive panel for viewing and toggling feature flags used in
//! A/B testing and experimental features. Displays flags grouped by category with
//! impact indicators and stability levels.
//!
//! # Features
//!
//! - Grouped display by category (Context, Routing, Intelligence, Optimization)
//! - Toggle flags with Space key
//! - Impact indicators (UX, Performance, Memory, Cost)
//! - Stability levels (Essential, Beta, Alpha, Experimental)
//! - Evidence-based descriptions
//! - Save/load from configuration
//!
//! # Examples
//!
//! ```
//! use toad::ui::widgets::core::FeatureFlagsPanel;
//! use toad::config::FeatureFlags;
//!
//! let flags = FeatureFlags::default();
//! let panel = FeatureFlagsPanel::new(flags);
//! ```

use crate::config::FeatureFlags;
use crate::ui::atoms::Block;
use crate::ui::theme::ToadTheme;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{List, ListItem, ListState, Paragraph},
    Frame,
};

/// Feature flag category
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlagCategory {
    /// Context management strategies
    Context,
    /// Model routing strategies
    Routing,
    /// Intelligence features
    Intelligence,
    /// Performance optimizations
    Optimization,
}

impl FlagCategory {
    /// Get category display name
    pub fn name(&self) -> &'static str {
        match self {
            FlagCategory::Context => "Context Strategies",
            FlagCategory::Routing => "Routing Strategies",
            FlagCategory::Intelligence => "Intelligence Features",
            FlagCategory::Optimization => "Performance Optimizations",
        }
    }

    /// Get category description
    pub fn description(&self) -> &'static str {
        match self {
            FlagCategory::Context => "How we gather and present code context to the LLM",
            FlagCategory::Routing => "How we select which model(s) to use",
            FlagCategory::Intelligence => "Smart features that improve accuracy",
            FlagCategory::Optimization => "Performance and cost optimizations",
        }
    }
}

/// Impact level for a feature
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Impact {
    /// No significant impact
    None,
    /// Improves UX
    Ux,
    /// Improves performance
    Performance,
    /// Increases memory usage
    Memory,
    /// Increases cost
    Cost,
    /// Multiple impacts
    Multiple,
}

impl Impact {
    /// Get impact display string
    pub fn display(&self) -> &'static str {
        match self {
            Impact::None => "",
            Impact::Ux => "📊 UX",
            Impact::Performance => "⚡ Perf",
            Impact::Memory => "💾 Mem",
            Impact::Cost => "💰 Cost",
            Impact::Multiple => "🔀 Multi",
        }
    }

    /// Get impact color
    pub fn color(&self) -> Color {
        match self {
            Impact::None => ToadTheme::GRAY,
            Impact::Ux => ToadTheme::TOAD_GREEN,
            Impact::Performance => ToadTheme::BLUE,
            Impact::Memory => ToadTheme::YELLOW,
            Impact::Cost => ToadTheme::WARNING,
            Impact::Multiple => ToadTheme::TOAD_GREEN,
        }
    }
}

/// Stability level for a feature
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stability {
    /// Production-ready, essential feature
    Essential,
    /// Beta quality, tested but not fully proven
    Beta,
    /// Alpha quality, experimental
    Alpha,
    /// Experimental, may not work
    Experimental,
}

impl Stability {
    /// Get stability display string
    pub fn display(&self) -> &'static str {
        match self {
            Stability::Essential => "✓ Essential",
            Stability::Beta => "β Beta",
            Stability::Alpha => "α Alpha",
            Stability::Experimental => "🧪 Exp",
        }
    }

    /// Get stability color
    pub fn color(&self) -> Color {
        match self {
            Stability::Essential => ToadTheme::TOAD_GREEN,
            Stability::Beta => ToadTheme::BLUE,
            Stability::Alpha => ToadTheme::YELLOW,
            Stability::Experimental => ToadTheme::WARNING,
        }
    }
}

/// A single feature flag entry
#[derive(Debug, Clone)]
pub struct FlagEntry {
    /// Flag identifier
    pub id: &'static str,
    /// Display name
    pub name: &'static str,
    /// Description with evidence
    pub description: &'static str,
    /// Category
    pub category: FlagCategory,
    /// Whether enabled
    pub enabled: bool,
    /// Impact level
    pub impact: Impact,
    /// Stability level
    pub stability: Stability,
    /// Warning message if any
    pub warning: Option<&'static str>,
}

impl FlagEntry {
    /// Create a new flag entry
    pub fn new(
        id: &'static str,
        name: &'static str,
        description: &'static str,
        category: FlagCategory,
        enabled: bool,
        impact: Impact,
        stability: Stability,
    ) -> Self {
        Self {
            id,
            name,
            description,
            category,
            enabled,
            impact,
            stability,
            warning: None,
        }
    }

    /// Set warning message
    pub fn with_warning(mut self, warning: &'static str) -> Self {
        self.warning = Some(warning);
        self
    }
}

/// Feature flags panel widget
///
/// Displays all feature flags grouped by category with interactive toggling.
///
/// # Examples
///
/// ```
/// use toad::ui::widgets::core::FeatureFlagsPanel;
/// use toad::config::FeatureFlags;
///
/// let flags = FeatureFlags::default();
/// let mut panel = FeatureFlagsPanel::new(flags);
///
/// // Navigate
/// panel.select_next();
/// panel.select_previous();
///
/// // Toggle selected flag
/// panel.toggle_selected();
///
/// // Get updated flags
/// let updated = panel.to_feature_flags();
/// ```
#[derive(Debug)]
pub struct FeatureFlagsPanel {
    /// Flag entries
    entries: Vec<FlagEntry>,
    /// Selected entry index
    selected_index: usize,
    /// List state for rendering
    list_state: ListState,
    /// Show details panel
    show_details: bool,
    /// Current category filter (None = show all)
    category_filter: Option<FlagCategory>,
}

impl FeatureFlagsPanel {
    /// Create a new feature flags panel
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::widgets::core::FeatureFlagsPanel;
    /// use toad::config::FeatureFlags;
    ///
    /// let flags = FeatureFlags::default();
    /// let panel = FeatureFlagsPanel::new(flags);
    /// assert_eq!(panel.entry_count(), 13);
    /// ```
    pub fn new(flags: FeatureFlags) -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        let entries = Self::build_entries(flags);

        Self {
            entries,
            selected_index: 0,
            list_state,
            show_details: true,
            category_filter: None,
        }
    }

    /// Build flag entries from FeatureFlags
    fn build_entries(flags: FeatureFlags) -> Vec<FlagEntry> {
        vec![
            // Context Strategies
            FlagEntry::new(
                "context_ast",
                "AST Context",
                "Use tree-sitter AST parsing for code context. Evidence: Aider proven, +2-5 points",
                FlagCategory::Context,
                flags.context_ast,
                Impact::Ux,
                Stability::Essential,
            ),
            FlagEntry::new(
                "context_embeddings",
                "Vector Embeddings",
                "Add vector embeddings for semantic search. Evidence: Standard RAG technique",
                FlagCategory::Context,
                flags.context_embeddings,
                Impact::Multiple,
                Stability::Beta,
            )
            .with_warning("Increases memory usage and latency"),
            FlagEntry::new(
                "context_graph",
                "Code Graph",
                "Analyze imports, calls, dependencies. Evidence: CodexGraph SIGKDD 2024",
                FlagCategory::Context,
                flags.context_graph,
                Impact::Memory,
                Stability::Alpha,
            ),
            FlagEntry::new(
                "context_reranking",
                "Context Reranking",
                "Re-rank retrieved context by relevance. Evidence: Cohere Rerank improves precision",
                FlagCategory::Context,
                flags.context_reranking,
                Impact::Cost,
                Stability::Beta,
            ),
            // Routing Strategies
            FlagEntry::new(
                "routing_semantic",
                "Semantic Router",
                "Use semantic routing for model selection. Evidence: Aurelio Labs - 50x faster",
                FlagCategory::Routing,
                flags.routing_semantic,
                Impact::Performance,
                Stability::Beta,
            ),
            FlagEntry::new(
                "routing_multi_model",
                "Multi-Model Racing",
                "Race multiple models in parallel. Evidence: TRAE 75.2% vs Warp 71% = +4.2 points PROVEN",
                FlagCategory::Routing,
                flags.routing_multi_model,
                Impact::Cost,
                Stability::Essential,
            )
            .with_warning("Doubles API cost but significantly improves accuracy"),
            FlagEntry::new(
                "routing_cascade",
                "Cascading Routing",
                "Try cheap models first, escalate if needed. Evidence: DavaJ 70% cost reduction",
                FlagCategory::Routing,
                flags.routing_cascade,
                Impact::Cost,
                Stability::Beta,
            ),
            FlagEntry::new(
                "routing_speculative",
                "Speculative Execution",
                "Run fast + premium models in parallel. Evidence: Novel approach, 24% cost savings potential",
                FlagCategory::Routing,
                flags.routing_speculative,
                Impact::Multiple,
                Stability::Experimental,
            ),
            // Intelligence Features
            FlagEntry::new(
                "smart_test_selection",
                "Smart Test Selection",
                "Select tests using coverage + SBFL. Evidence: AutoCodeRover proven, +3-5 points",
                FlagCategory::Intelligence,
                flags.smart_test_selection,
                Impact::Ux,
                Stability::Essential,
            ),
            FlagEntry::new(
                "failure_memory",
                "Failure Memory",
                "Learn from past failures (persistent memory). Evidence: RL experience replay concept",
                FlagCategory::Intelligence,
                flags.failure_memory,
                Impact::Multiple,
                Stability::Alpha,
            ),
            FlagEntry::new(
                "opportunistic_planning",
                "Opportunistic Planning",
                "Fast plan + execute + refine. Evidence: Anytime algorithms",
                FlagCategory::Intelligence,
                flags.opportunistic_planning,
                Impact::Performance,
                Stability::Experimental,
            ),
            // Optimization Features
            FlagEntry::new(
                "prompt_caching",
                "Prompt Caching",
                "Cache prompts at API level. Evidence: 90% cost reduction PROVEN",
                FlagCategory::Optimization,
                flags.prompt_caching,
                Impact::Cost,
                Stability::Essential,
            ),
            FlagEntry::new(
                "semantic_caching",
                "Semantic Caching",
                "Cache by semantic similarity. Evidence: GPTCache 68.8% API reduction",
                FlagCategory::Optimization,
                flags.semantic_caching,
                Impact::Multiple,
                Stability::Beta,
            ),
            FlagEntry::new(
                "tree_sitter_validation",
                "Syntax Validation",
                "Validate syntax before applying. Evidence: Production-proven, prevents errors",
                FlagCategory::Optimization,
                flags.tree_sitter_validation,
                Impact::Ux,
                Stability::Essential,
            ),
        ]
    }

    /// Get number of entries
    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }

    /// Get number of enabled flags
    pub fn enabled_count(&self) -> usize {
        self.entries.iter().filter(|e| e.enabled).count()
    }

    /// Select next entry
    pub fn select_next(&mut self) {
        if !self.entries.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.entries.len();
            self.list_state.select(Some(self.selected_index));
        }
    }

    /// Select previous entry
    pub fn select_previous(&mut self) {
        if !self.entries.is_empty() {
            self.selected_index = if self.selected_index == 0 {
                self.entries.len() - 1
            } else {
                self.selected_index - 1
            };
            self.list_state.select(Some(self.selected_index));
        }
    }

    /// Toggle selected flag
    pub fn toggle_selected(&mut self) {
        if let Some(entry) = self.entries.get_mut(self.selected_index) {
            entry.enabled = !entry.enabled;
        }
    }

    /// Get selected entry
    pub fn selected_entry(&self) -> Option<&FlagEntry> {
        self.entries.get(self.selected_index)
    }

    /// Toggle details panel
    pub fn toggle_details(&mut self) {
        self.show_details = !self.show_details;
    }

    /// Set category filter
    pub fn set_category_filter(&mut self, category: Option<FlagCategory>) {
        self.category_filter = category;
    }

    /// Convert back to FeatureFlags
    pub fn to_feature_flags(&self) -> FeatureFlags {
        let mut flags = FeatureFlags::default();

        for entry in &self.entries {
            match entry.id {
                "context_ast" => flags.context_ast = entry.enabled,
                "context_embeddings" => flags.context_embeddings = entry.enabled,
                "context_graph" => flags.context_graph = entry.enabled,
                "context_reranking" => flags.context_reranking = entry.enabled,
                "routing_semantic" => flags.routing_semantic = entry.enabled,
                "routing_multi_model" => flags.routing_multi_model = entry.enabled,
                "routing_cascade" => flags.routing_cascade = entry.enabled,
                "routing_speculative" => flags.routing_speculative = entry.enabled,
                "smart_test_selection" => flags.smart_test_selection = entry.enabled,
                "failure_memory" => flags.failure_memory = entry.enabled,
                "opportunistic_planning" => flags.opportunistic_planning = entry.enabled,
                "prompt_caching" => flags.prompt_caching = entry.enabled,
                "semantic_caching" => flags.semantic_caching = entry.enabled,
                "tree_sitter_validation" => flags.tree_sitter_validation = entry.enabled,
                _ => {}
            }
        }

        flags
    }

    /// Render the feature flags panel
    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let chunks = if self.show_details {
            Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(60), // Flag list
                    Constraint::Percentage(40), // Details
                ])
                .split(area)
        } else {
            Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(100)])
                .split(area)
        };

        self.render_flag_list(frame, chunks[0]);

        if self.show_details && chunks.len() > 1 {
            self.render_details(frame, chunks[1]);
        }
    }

    /// Render flag list
    fn render_flag_list(&mut self, frame: &mut Frame, area: Rect) {
        let block = Block::themed("Feature Flags").to_ratatui();
        let inner = block.inner(area);
        frame.render_widget(block, area);

        // Group by category
        let mut items = Vec::new();
        let mut current_category: Option<FlagCategory> = None;

        for (_idx, entry) in self.entries.iter().enumerate() {
            // Add category header
            if current_category != Some(entry.category) {
                current_category = Some(entry.category);
                items.push(ListItem::new(Line::from(Span::styled(
                    format!("\n{}", entry.category.name()),
                    Style::default()
                        .fg(ToadTheme::TOAD_GREEN)
                        .add_modifier(Modifier::BOLD),
                ))));
            }

            // Add flag entry
            let mut spans = vec![];

            // Checkbox
            let checkbox = if entry.enabled { "[✓]" } else { "[ ]" };
            spans.push(Span::styled(
                format!("{} ", checkbox),
                Style::default().fg(if entry.enabled {
                    ToadTheme::TOAD_GREEN
                } else {
                    ToadTheme::GRAY
                }),
            ));

            // Name
            spans.push(Span::styled(
                entry.name,
                Style::default().fg(ToadTheme::FOREGROUND),
            ));

            // Stability badge
            spans.push(Span::raw(" "));
            spans.push(Span::styled(
                entry.stability.display(),
                Style::default().fg(entry.stability.color()),
            ));

            // Impact badge
            if entry.impact != Impact::None {
                spans.push(Span::raw(" "));
                spans.push(Span::styled(
                    entry.impact.display(),
                    Style::default().fg(entry.impact.color()),
                ));
            }

            let item = ListItem::new(Line::from(spans));
            items.push(item);
        }

        let list = List::new(items)
            .highlight_style(
                Style::default()
                    .bg(ToadTheme::DARK_GRAY)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("▶ ");

        frame.render_stateful_widget(list, inner, &mut self.list_state);
    }

    /// Render details panel
    fn render_details(&self, frame: &mut Frame, area: Rect) {
        let block = Block::themed("Details").to_ratatui();
        let inner = block.inner(area);
        frame.render_widget(block, area);

        if let Some(entry) = self.selected_entry() {
            let mut lines = vec![];

            // Name
            lines.push(Line::from(Span::styled(
                entry.name,
                Style::default()
                    .fg(ToadTheme::TOAD_GREEN)
                    .add_modifier(Modifier::BOLD),
            )));
            lines.push(Line::from(""));

            // Description
            lines.push(Line::from(Span::styled(
                "Description:",
                Style::default().add_modifier(Modifier::BOLD),
            )));
            lines.push(Line::from(Span::styled(
                entry.description,
                Style::default().fg(ToadTheme::GRAY),
            )));
            lines.push(Line::from(""));

            // Status
            let status = if entry.enabled { "Enabled" } else { "Disabled" };
            lines.push(Line::from(vec![
                Span::styled("Status: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::styled(
                    status,
                    Style::default().fg(if entry.enabled {
                        ToadTheme::TOAD_GREEN
                    } else {
                        ToadTheme::GRAY
                    }),
                ),
            ]));

            // Category
            lines.push(Line::from(vec![
                Span::styled("Category: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::styled(entry.category.name(), Style::default().fg(ToadTheme::GRAY)),
            ]));

            // Stability
            lines.push(Line::from(vec![
                Span::styled(
                    "Stability: ",
                    Style::default().add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    entry.stability.display(),
                    Style::default().fg(entry.stability.color()),
                ),
            ]));

            // Impact
            if entry.impact != Impact::None {
                lines.push(Line::from(vec![
                    Span::styled("Impact: ", Style::default().add_modifier(Modifier::BOLD)),
                    Span::styled(
                        entry.impact.display(),
                        Style::default().fg(entry.impact.color()),
                    ),
                ]));
            }

            // Warning
            if let Some(warning) = entry.warning {
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled(
                    "⚠ Warning:",
                    Style::default()
                        .fg(ToadTheme::WARNING)
                        .add_modifier(Modifier::BOLD),
                )));
                lines.push(Line::from(Span::styled(
                    warning,
                    Style::default().fg(ToadTheme::WARNING),
                )));
            }

            // Keybinds
            lines.push(Line::from(""));
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                "Keybinds:",
                Style::default().add_modifier(Modifier::BOLD),
            )));
            lines.push(Line::from("Space - Toggle flag"));
            lines.push(Line::from("↑/↓ - Navigate"));
            lines.push(Line::from("d - Toggle details"));

            let paragraph = Paragraph::new(lines);
            frame.render_widget(paragraph, inner);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flag_category_name() {
        assert_eq!(FlagCategory::Context.name(), "Context Strategies");
        assert_eq!(FlagCategory::Routing.name(), "Routing Strategies");
        assert_eq!(FlagCategory::Intelligence.name(), "Intelligence Features");
        assert_eq!(
            FlagCategory::Optimization.name(),
            "Performance Optimizations"
        );
    }

    #[test]
    fn test_impact_display() {
        assert_eq!(Impact::None.display(), "");
        assert_eq!(Impact::Ux.display(), "📊 UX");
        assert_eq!(Impact::Performance.display(), "⚡ Perf");
        assert_eq!(Impact::Memory.display(), "💾 Mem");
        assert_eq!(Impact::Cost.display(), "💰 Cost");
    }

    #[test]
    fn test_stability_display() {
        assert_eq!(Stability::Essential.display(), "✓ Essential");
        assert_eq!(Stability::Beta.display(), "β Beta");
        assert_eq!(Stability::Alpha.display(), "α Alpha");
        assert_eq!(Stability::Experimental.display(), "🧪 Exp");
    }

    #[test]
    fn test_panel_creation() {
        let flags = FeatureFlags::default();
        let panel = FeatureFlagsPanel::new(flags);
        assert_eq!(panel.entry_count(), 13);
        assert_eq!(panel.selected_index, 0);
    }

    #[test]
    fn test_navigation() {
        let flags = FeatureFlags::default();
        let mut panel = FeatureFlagsPanel::new(flags);

        assert_eq!(panel.selected_index, 0);

        panel.select_next();
        assert_eq!(panel.selected_index, 1);

        panel.select_previous();
        assert_eq!(panel.selected_index, 0);

        // Wrap around
        panel.select_previous();
        assert_eq!(panel.selected_index, 12);
    }

    #[test]
    fn test_toggle_flag() {
        let mut flags = FeatureFlags::default();
        flags.context_ast = true;

        let mut panel = FeatureFlagsPanel::new(flags);

        // First entry is context_ast
        assert!(panel.entries[0].enabled);

        panel.toggle_selected();
        assert!(!panel.entries[0].enabled);

        panel.toggle_selected();
        assert!(panel.entries[0].enabled);
    }

    #[test]
    fn test_to_feature_flags() {
        let flags = FeatureFlags::default();
        let mut panel = FeatureFlagsPanel::new(flags.clone());

        // Toggle first flag
        panel.toggle_selected();

        let new_flags = panel.to_feature_flags();
        assert_ne!(new_flags.context_ast, flags.context_ast);
    }

    #[test]
    fn test_enabled_count() {
        let flags = FeatureFlags::default();
        let panel = FeatureFlagsPanel::new(flags.clone());
        assert_eq!(panel.enabled_count(), flags.enabled_count());
    }

    #[test]
    fn test_selected_entry() {
        let flags = FeatureFlags::default();
        let panel = FeatureFlagsPanel::new(flags);
        let entry = panel.selected_entry().unwrap();
        assert_eq!(entry.id, "context_ast");
    }

    #[test]
    fn test_toggle_details() {
        let flags = FeatureFlags::default();
        let mut panel = FeatureFlagsPanel::new(flags);
        assert!(panel.show_details);

        panel.toggle_details();
        assert!(!panel.show_details);

        panel.toggle_details();
        assert!(panel.show_details);
    }

    #[test]
    fn test_flag_entry_with_warning() {
        let entry = FlagEntry::new(
            "test",
            "Test Flag",
            "Test description",
            FlagCategory::Context,
            false,
            Impact::None,
            Stability::Beta,
        )
        .with_warning("Test warning");

        assert_eq!(entry.warning, Some("Test warning"));
    }

    #[test]
    fn test_milestone_flags() {
        let m1 = FeatureFlags::milestone_1();
        let panel = FeatureFlagsPanel::new(m1);
        // M1 has minimal flags enabled
        assert!(panel.enabled_count() < 5);

        let m3 = FeatureFlags::milestone_3();
        let panel = FeatureFlagsPanel::new(m3);
        // M3 has more flags enabled
        assert!(panel.enabled_count() > 3);
    }

    #[test]
    fn test_all_flag_ids_handled() {
        let flags = FeatureFlags::default();
        let panel = FeatureFlagsPanel::new(flags.clone());
        let restored = panel.to_feature_flags();

        // Verify round-trip conversion
        assert_eq!(restored.context_ast, flags.context_ast);
        assert_eq!(restored.context_embeddings, flags.context_embeddings);
        assert_eq!(restored.context_graph, flags.context_graph);
        assert_eq!(restored.context_reranking, flags.context_reranking);
        assert_eq!(restored.routing_semantic, flags.routing_semantic);
        assert_eq!(restored.routing_multi_model, flags.routing_multi_model);
        assert_eq!(restored.routing_cascade, flags.routing_cascade);
        assert_eq!(restored.routing_speculative, flags.routing_speculative);
        assert_eq!(
            restored.smart_test_selection,
            flags.smart_test_selection
        );
        assert_eq!(restored.failure_memory, flags.failure_memory);
        assert_eq!(
            restored.opportunistic_planning,
            flags.opportunistic_planning
        );
        assert_eq!(restored.prompt_caching, flags.prompt_caching);
        assert_eq!(restored.semantic_caching, flags.semantic_caching);
        assert_eq!(
            restored.tree_sitter_validation,
            flags.tree_sitter_validation
        );
    }
}
