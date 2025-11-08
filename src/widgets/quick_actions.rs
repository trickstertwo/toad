//! Quick Actions panel for frequently used commands
//!
//! Surfaces commonly used actions for quick access with visual indicators.
//!
//! # Examples
//!
//! ```
//! use toad::widgets::QuickActions;
//!
//! let actions = QuickActions::new()
//!     .add_action("Save", "Save current file", "save", Some("Ctrl+S"))
//!     .add_action("Build", "Build project", "build", Some("F5"))
//!     .add_action("Test", "Run tests", "test", Some("F6"));
//!
//! assert_eq!(actions.action_count(), 3);
//! ```

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Widget},
};

/// Quick action item
///
/// Represents a frequently used action with description and keybind.
///
/// # Examples
///
/// ```
/// use toad::widgets::QuickAction;
///
/// let action = QuickAction::new("Save", "Save current file", "save")
///     .with_keybind("Ctrl+S")
///     .with_icon("💾");
///
/// assert_eq!(action.label(), "Save");
/// assert_eq!(action.description(), "Save current file");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuickAction {
    /// Action label
    label: String,
    /// Action description
    description: String,
    /// Action identifier
    action: String,
    /// Optional keybind hint
    keybind: Option<String>,
    /// Optional icon
    icon: Option<String>,
    /// Whether action is enabled
    enabled: bool,
}

impl QuickAction {
    /// Create a new quick action
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::QuickAction;
    ///
    /// let action = QuickAction::new("Save", "Save file", "save");
    /// assert_eq!(action.label(), "Save");
    /// ```
    pub fn new(
        label: impl Into<String>,
        description: impl Into<String>,
        action: impl Into<String>,
    ) -> Self {
        Self {
            label: label.into(),
            description: description.into(),
            action: action.into(),
            keybind: None,
            icon: None,
            enabled: true,
        }
    }

    /// Add keybind hint
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::QuickAction;
    ///
    /// let action = QuickAction::new("Save", "Save file", "save")
    ///     .with_keybind("Ctrl+S");
    /// assert_eq!(action.keybind(), Some("Ctrl+S"));
    /// ```
    pub fn with_keybind(mut self, keybind: impl Into<String>) -> Self {
        self.keybind = Some(keybind.into());
        self
    }

    /// Add icon
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::QuickAction;
    ///
    /// let action = QuickAction::new("Save", "Save file", "save")
    ///     .with_icon("💾");
    /// assert_eq!(action.icon(), Some("💾"));
    /// ```
    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set enabled state
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::QuickAction;
    ///
    /// let action = QuickAction::new("Paste", "Paste text", "paste")
    ///     .with_enabled(false);
    /// assert!(!action.is_enabled());
    /// ```
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Get label
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Get description
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Get action ID
    pub fn action(&self) -> &str {
        &self.action
    }

    /// Get keybind hint
    pub fn keybind(&self) -> Option<&str> {
        self.keybind.as_deref()
    }

    /// Get icon
    pub fn icon(&self) -> Option<&str> {
        self.icon.as_deref()
    }

    /// Check if enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Set enabled state (mutable)
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

/// Quick Actions panel widget
///
/// Displays a panel of frequently used actions with descriptions.
///
/// # Examples
///
/// ```
/// use toad::widgets::QuickActions;
///
/// let mut actions = QuickActions::new()
///     .add_action("Save", "Save file", "save", Some("Ctrl+S"))
///     .add_action("Build", "Build project", "build", Some("F5"));
///
/// actions.next();
/// assert_eq!(actions.selected_action(), Some("build"));
/// ```
#[derive(Debug, Clone)]
pub struct QuickActions {
    /// Action items
    actions: Vec<QuickAction>,
    /// Selected action index
    selected: usize,
    /// Show keybind hints
    show_keybinds: bool,
    /// Show icons
    show_icons: bool,
    /// Show descriptions
    show_descriptions: bool,
    /// Title
    title: Option<String>,
    /// Compact mode (one line per action)
    compact: bool,
}

impl Default for QuickActions {
    fn default() -> Self {
        Self::new()
    }
}

impl QuickActions {
    /// Create a new quick actions panel
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::QuickActions;
    ///
    /// let actions = QuickActions::new();
    /// assert_eq!(actions.action_count(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
            selected: 0,
            show_keybinds: true,
            show_icons: true,
            show_descriptions: true,
            title: Some("Quick Actions".to_string()),
            compact: false,
        }
    }

    /// Add a quick action
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::QuickActions;
    ///
    /// let actions = QuickActions::new()
    ///     .add_action("Save", "Save file", "save", Some("Ctrl+S"));
    ///
    /// assert_eq!(actions.action_count(), 1);
    /// ```
    pub fn add_action(
        mut self,
        label: impl Into<String>,
        description: impl Into<String>,
        action: impl Into<String>,
        keybind: Option<impl Into<String>>,
    ) -> Self {
        let mut item = QuickAction::new(label, description, action);
        if let Some(kb) = keybind {
            item = item.with_keybind(kb);
        }
        self.actions.push(item);
        self
    }

    /// Set whether to show keybind hints
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::QuickActions;
    ///
    /// let actions = QuickActions::new()
    ///     .with_keybinds(false);
    /// ```
    pub fn with_keybinds(mut self, show: bool) -> Self {
        self.show_keybinds = show;
        self
    }

    /// Set whether to show icons
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::QuickActions;
    ///
    /// let actions = QuickActions::new()
    ///     .with_icons(false);
    /// ```
    pub fn with_icons(mut self, show: bool) -> Self {
        self.show_icons = show;
        self
    }

    /// Set whether to show descriptions
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::QuickActions;
    ///
    /// let actions = QuickActions::new()
    ///     .with_descriptions(false);
    /// ```
    pub fn with_descriptions(mut self, show: bool) -> Self {
        self.show_descriptions = show;
        self
    }

    /// Set title
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::QuickActions;
    ///
    /// let actions = QuickActions::new()
    ///     .with_title("File Actions");
    /// ```
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set compact mode
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::QuickActions;
    ///
    /// let actions = QuickActions::new()
    ///     .with_compact(true);
    /// ```
    pub fn with_compact(mut self, compact: bool) -> Self {
        self.compact = compact;
        self
    }

    /// Get number of actions
    pub fn action_count(&self) -> usize {
        self.actions.len()
    }

    /// Get all actions
    pub fn actions(&self) -> &[QuickAction] {
        &self.actions
    }

    /// Move selection down
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::QuickActions;
    ///
    /// let mut actions = QuickActions::new()
    ///     .add_action("First", "First action", "first", None::<&str>)
    ///     .add_action("Second", "Second action", "second", None::<&str>);
    ///
    /// actions.next();
    /// assert_eq!(actions.selected_index(), 1);
    /// ```
    pub fn next(&mut self) {
        if self.actions.is_empty() {
            return;
        }

        let start = (self.selected + 1) % self.actions.len();
        self.selected = self.find_next_enabled(start).unwrap_or(self.selected);
    }

    /// Move selection up
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::QuickActions;
    ///
    /// let mut actions = QuickActions::new()
    ///     .add_action("First", "First action", "first", None::<&str>)
    ///     .add_action("Second", "Second action", "second", None::<&str>);
    ///
    /// actions.next();
    /// actions.previous();
    /// assert_eq!(actions.selected_index(), 0);
    /// ```
    pub fn previous(&mut self) {
        if self.actions.is_empty() {
            return;
        }

        let start = if self.selected == 0 {
            self.actions.len() - 1
        } else {
            self.selected - 1
        };

        self.selected = self.find_previous_enabled(start).unwrap_or(self.selected);
    }

    /// Get selected index
    pub fn selected_index(&self) -> usize {
        self.selected
    }

    /// Get selected action
    pub fn selected_item(&self) -> Option<&QuickAction> {
        self.actions.get(self.selected)
    }

    /// Get selected action ID
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::QuickActions;
    ///
    /// let actions = QuickActions::new()
    ///     .add_action("Save", "Save file", "save", None::<&str>);
    ///
    /// assert_eq!(actions.selected_action(), Some("save"));
    /// ```
    pub fn selected_action(&self) -> Option<&str> {
        self.selected_item().map(|item| item.action())
    }

    /// Add action (mutable)
    pub fn push_action(&mut self, action: QuickAction) {
        self.actions.push(action);
    }

    /// Clear all actions
    pub fn clear(&mut self) {
        self.actions.clear();
        self.selected = 0;
    }

    /// Set selected index
    pub fn set_selected(&mut self, index: usize) {
        if index < self.actions.len() {
            self.selected = index;
        }
    }

    /// Find next enabled action starting from index
    fn find_next_enabled(&self, start: usize) -> Option<usize> {
        for i in 0..self.actions.len() {
            let idx = (start + i) % self.actions.len();
            if let Some(action) = self.actions.get(idx)
                && action.is_enabled()
            {
                return Some(idx);
            }
        }
        None
    }

    /// Find previous enabled action starting from index
    fn find_previous_enabled(&self, start: usize) -> Option<usize> {
        for i in 0..self.actions.len() {
            let idx = if start >= i {
                start - i
            } else {
                self.actions.len() + start - i
            };

            if let Some(action) = self.actions.get(idx)
                && action.is_enabled()
            {
                return Some(idx);
            }
        }
        None
    }

    /// Render action lines
    fn render_lines(&self) -> Vec<Line<'static>> {
        let mut lines = Vec::new();

        for (i, action) in self.actions.iter().enumerate() {
            let is_selected = i == self.selected;

            if self.compact {
                // Compact: single line per action
                let mut spans = Vec::new();

                // Selection indicator
                if is_selected {
                    spans.push(Span::styled(
                        "> ".to_string(),
                        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
                    ));
                } else {
                    spans.push(Span::raw("  ".to_string()));
                }

                // Icon
                if self.show_icons
                    && let Some(icon) = action.icon()
                {
                    spans.push(Span::raw(format!("{} ", icon)));
                }

                // Label
                let label_style = if !action.is_enabled() {
                    Style::default().fg(Color::DarkGray)
                } else if is_selected {
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::Gray)
                };

                spans.push(Span::styled(action.label().to_string(), label_style));

                // Keybind
                if self.show_keybinds
                    && let Some(keybind) = action.keybind()
                {
                    spans.push(Span::raw(" ".to_string()));
                    spans.push(Span::styled(
                        format!("[{}]", keybind),
                        Style::default().fg(Color::DarkGray),
                    ));
                }

                lines.push(Line::from(spans));
            } else {
                // Full: multiple lines per action
                let mut label_spans = Vec::new();

                // Selection indicator
                if is_selected {
                    label_spans.push(Span::styled(
                        "> ".to_string(),
                        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
                    ));
                } else {
                    label_spans.push(Span::raw("  ".to_string()));
                }

                // Icon
                if self.show_icons
                    && let Some(icon) = action.icon()
                {
                    label_spans.push(Span::raw(format!("{} ", icon)));
                }

                // Label
                let label_style = if !action.is_enabled() {
                    Style::default().fg(Color::DarkGray)
                } else if is_selected {
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::Cyan)
                };

                label_spans.push(Span::styled(action.label().to_string(), label_style));

                // Keybind on same line
                if self.show_keybinds
                    && let Some(keybind) = action.keybind()
                {
                    label_spans.push(Span::raw(" ".to_string()));
                    label_spans.push(Span::styled(
                        format!("[{}]", keybind),
                        Style::default().fg(Color::DarkGray),
                    ));
                }

                lines.push(Line::from(label_spans));

                // Description on next line
                if self.show_descriptions && !action.description().is_empty() {
                    lines.push(Line::from(vec![
                        Span::raw("    ".to_string()),
                        Span::styled(
                            action.description().to_string(),
                            Style::default().fg(Color::DarkGray),
                        ),
                    ]));
                }
            }
        }

        lines
    }
}

impl Widget for &QuickActions {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default());

        if let Some(title) = &self.title {
            block = block.title(title.clone());
        }

        let inner = block.inner(area);
        block.render(area, buf);

        let lines = self.render_lines();
        for (i, line) in lines.iter().enumerate() {
            if i >= inner.height as usize {
                break;
            }
            let y = inner.y + i as u16;
            buf.set_line(inner.x, y, line, inner.width);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_action_new() {
        let action = QuickAction::new("Save", "Save file", "save");
        assert_eq!(action.label(), "Save");
        assert_eq!(action.description(), "Save file");
        assert_eq!(action.action(), "save");
        assert_eq!(action.keybind(), None);
        assert_eq!(action.icon(), None);
        assert!(action.is_enabled());
    }

    #[test]
    fn test_quick_action_with_keybind() {
        let action = QuickAction::new("Save", "Save file", "save")
            .with_keybind("Ctrl+S");
        assert_eq!(action.keybind(), Some("Ctrl+S"));
    }

    #[test]
    fn test_quick_action_with_icon() {
        let action = QuickAction::new("Save", "Save file", "save")
            .with_icon("💾");
        assert_eq!(action.icon(), Some("💾"));
    }

    #[test]
    fn test_quick_action_with_enabled() {
        let action = QuickAction::new("Paste", "Paste text", "paste")
            .with_enabled(false);
        assert!(!action.is_enabled());
    }

    #[test]
    fn test_quick_action_set_enabled() {
        let mut action = QuickAction::new("Save", "Save file", "save");
        assert!(action.is_enabled());
        action.set_enabled(false);
        assert!(!action.is_enabled());
    }

    #[test]
    fn test_quick_actions_new() {
        let actions = QuickActions::new();
        assert_eq!(actions.action_count(), 0);
        assert_eq!(actions.selected_index(), 0);
        assert!(actions.show_keybinds);
        assert!(actions.show_icons);
        assert!(actions.show_descriptions);
    }

    #[test]
    fn test_quick_actions_default() {
        let actions = QuickActions::default();
        assert_eq!(actions.action_count(), 0);
    }

    #[test]
    fn test_quick_actions_add_action() {
        let actions = QuickActions::new()
            .add_action("Save", "Save file", "save", Some("Ctrl+S"))
            .add_action("Build", "Build project", "build", None::<&str>);

        assert_eq!(actions.action_count(), 2);
        assert_eq!(actions.actions()[0].label(), "Save");
        assert_eq!(actions.actions()[1].label(), "Build");
    }

    #[test]
    fn test_quick_actions_with_keybinds() {
        let actions = QuickActions::new()
            .with_keybinds(false);
        assert!(!actions.show_keybinds);
    }

    #[test]
    fn test_quick_actions_with_icons() {
        let actions = QuickActions::new()
            .with_icons(false);
        assert!(!actions.show_icons);
    }

    #[test]
    fn test_quick_actions_with_descriptions() {
        let actions = QuickActions::new()
            .with_descriptions(false);
        assert!(!actions.show_descriptions);
    }

    #[test]
    fn test_quick_actions_with_title() {
        let actions = QuickActions::new()
            .with_title("File Actions");
        assert_eq!(actions.title, Some("File Actions".to_string()));
    }

    #[test]
    fn test_quick_actions_with_compact() {
        let actions = QuickActions::new()
            .with_compact(true);
        assert!(actions.compact);
    }

    #[test]
    fn test_quick_actions_navigation() {
        let mut actions = QuickActions::new()
            .add_action("First", "First action", "first", None::<&str>)
            .add_action("Second", "Second action", "second", None::<&str>)
            .add_action("Third", "Third action", "third", None::<&str>);

        assert_eq!(actions.selected_index(), 0);

        actions.next();
        assert_eq!(actions.selected_index(), 1);

        actions.next();
        assert_eq!(actions.selected_index(), 2);

        actions.next();
        assert_eq!(actions.selected_index(), 0); // Wrap around

        actions.previous();
        assert_eq!(actions.selected_index(), 2); // Wrap around

        actions.previous();
        assert_eq!(actions.selected_index(), 1);
    }

    #[test]
    fn test_quick_actions_skip_disabled() {
        let mut actions = QuickActions::new()
            .add_action("First", "First action", "first", None::<&str>)
            .add_action("Second", "Second action", "second", None::<&str>)
            .add_action("Third", "Third action", "third", None::<&str>);

        actions.actions[1].set_enabled(false);

        assert_eq!(actions.selected_index(), 0);

        actions.next();
        assert_eq!(actions.selected_index(), 2); // Skip disabled
    }

    #[test]
    fn test_quick_actions_selected_action() {
        let actions = QuickActions::new()
            .add_action("Save", "Save file", "save", None::<&str>)
            .add_action("Build", "Build project", "build", None::<&str>);

        assert_eq!(actions.selected_action(), Some("save"));
    }

    #[test]
    fn test_quick_actions_selected_item() {
        let actions = QuickActions::new()
            .add_action("Save", "Save file", "save", None::<&str>);

        let item = actions.selected_item();
        assert!(item.is_some());
        assert_eq!(item.unwrap().label(), "Save");
    }

    #[test]
    fn test_quick_actions_push_action() {
        let mut actions = QuickActions::new();
        assert_eq!(actions.action_count(), 0);

        actions.push_action(QuickAction::new("Test", "Test action", "test"));
        assert_eq!(actions.action_count(), 1);
    }

    #[test]
    fn test_quick_actions_clear() {
        let mut actions = QuickActions::new()
            .add_action("First", "First action", "first", None::<&str>)
            .add_action("Second", "Second action", "second", None::<&str>);

        assert_eq!(actions.action_count(), 2);
        actions.clear();
        assert_eq!(actions.action_count(), 0);
        assert_eq!(actions.selected_index(), 0);
    }

    #[test]
    fn test_quick_actions_set_selected() {
        let mut actions = QuickActions::new()
            .add_action("First", "First action", "first", None::<&str>)
            .add_action("Second", "Second action", "second", None::<&str>);

        actions.set_selected(1);
        assert_eq!(actions.selected_index(), 1);
    }

    #[test]
    fn test_quick_actions_render_lines_compact() {
        let actions = QuickActions::new()
            .add_action("Save", "Save file", "save", Some("Ctrl+S"))
            .with_compact(true);

        let lines = actions.render_lines();
        assert_eq!(lines.len(), 1);
    }

    #[test]
    fn test_quick_actions_render_lines_full() {
        let actions = QuickActions::new()
            .add_action("Save", "Save file", "save", Some("Ctrl+S"))
            .with_compact(false);

        let lines = actions.render_lines();
        assert_eq!(lines.len(), 2); // Label + description
    }

    #[test]
    fn test_quick_actions_builder_pattern() {
        let actions = QuickActions::new()
            .add_action("Save", "Save file", "save", Some("Ctrl+S"))
            .add_action("Build", "Build project", "build", Some("F5"))
            .with_keybinds(false)
            .with_icons(true)
            .with_descriptions(false)
            .with_title("Actions")
            .with_compact(true);

        assert_eq!(actions.action_count(), 2);
        assert!(!actions.show_keybinds);
        assert!(actions.show_icons);
        assert!(!actions.show_descriptions);
        assert_eq!(actions.title, Some("Actions".to_string()));
        assert!(actions.compact);
    }
}
