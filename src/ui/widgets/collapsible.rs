/// Collapsible sections widget
///
/// Accordion-style panels that can be expanded or collapsed
///
/// # Examples
///
/// ```
/// use toad::widgets::{CollapsibleSection, CollapsibleList};
///
/// let section = CollapsibleSection::new("Section 1", "Content here");
/// assert!(!section.is_expanded());
/// ```
use crate::ui::theme::ToadTheme;
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};
use serde::{Deserialize, Serialize};

/// A single collapsible section
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollapsibleSection {
    /// Section title
    title: String,
    /// Section content (when expanded)
    content: String,
    /// Whether section is expanded
    expanded: bool,
    /// Whether section can be collapsed
    collapsible: bool,
}

impl CollapsibleSection {
    /// Create a new collapsible section
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::CollapsibleSection;
    ///
    /// let section = CollapsibleSection::new("Title", "Content");
    /// assert_eq!(section.title(), "Title");
    /// assert!(!section.is_expanded());
    /// ```
    pub fn new(title: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            content: content.into(),
            expanded: false,
            collapsible: true,
        }
    }

    /// Create an expanded section
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::CollapsibleSection;
    ///
    /// let section = CollapsibleSection::expanded("Title", "Content");
    /// assert!(section.is_expanded());
    /// ```
    pub fn expanded(title: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            content: content.into(),
            expanded: true,
            collapsible: true,
        }
    }

    /// Set collapsible state
    pub fn with_collapsible(mut self, collapsible: bool) -> Self {
        self.collapsible = collapsible;
        self
    }

    /// Get section title
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Get section content
    pub fn content(&self) -> &str {
        &self.content
    }

    /// Set section content
    pub fn set_content(&mut self, content: impl Into<String>) {
        self.content = content.into();
    }

    /// Check if section is expanded
    pub fn is_expanded(&self) -> bool {
        self.expanded
    }

    /// Check if section can be collapsed
    pub fn is_collapsible(&self) -> bool {
        self.collapsible
    }

    /// Toggle expansion state
    pub fn toggle(&mut self) {
        if self.collapsible {
            self.expanded = !self.expanded;
        }
    }

    /// Expand the section
    pub fn expand(&mut self) {
        self.expanded = true;
    }

    /// Collapse the section
    pub fn collapse(&mut self) {
        if self.collapsible {
            self.expanded = false;
        }
    }

    /// Get expansion indicator symbol
    pub fn indicator(&self) -> &'static str {
        if self.expanded { "▼" } else { "▶" }
    }
}

/// List of collapsible sections
#[derive(Debug, Clone)]
pub struct CollapsibleList {
    /// All sections
    sections: Vec<CollapsibleSection>,
    /// Currently focused section index
    focused: Option<usize>,
}

impl CollapsibleList {
    /// Create a new empty collapsible list
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::CollapsibleList;
    ///
    /// let list = CollapsibleList::new();
    /// assert_eq!(list.len(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            sections: Vec::new(),
            focused: None,
        }
    }

    /// Add a section
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{CollapsibleList, CollapsibleSection};
    ///
    /// let mut list = CollapsibleList::new();
    /// list.add(CollapsibleSection::new("Section 1", "Content 1"));
    /// assert_eq!(list.len(), 1);
    /// ```
    pub fn add(&mut self, section: CollapsibleSection) {
        self.sections.push(section);
        if self.sections.len() == 1 {
            self.focused = Some(0);
        }
    }

    /// Get all sections
    pub fn sections(&self) -> &[CollapsibleSection] {
        &self.sections
    }

    /// Get mutable section by index
    pub fn section_mut(&mut self, index: usize) -> Option<&mut CollapsibleSection> {
        self.sections.get_mut(index)
    }

    /// Get number of sections
    pub fn len(&self) -> usize {
        self.sections.len()
    }

    /// Check if list is empty
    pub fn is_empty(&self) -> bool {
        self.sections.is_empty()
    }

    /// Get focused section index
    pub fn focused_index(&self) -> Option<usize> {
        self.focused
    }

    /// Set focused section
    pub fn set_focus(&mut self, index: usize) -> bool {
        if index < self.sections.len() {
            self.focused = Some(index);
            true
        } else {
            false
        }
    }

    /// Focus next section
    pub fn focus_next(&mut self) {
        if self.sections.is_empty() {
            return;
        }

        self.focused = Some(match self.focused {
            Some(idx) if idx + 1 < self.sections.len() => idx + 1,
            _ => 0,
        });
    }

    /// Focus previous section
    pub fn focus_previous(&mut self) {
        if self.sections.is_empty() {
            return;
        }

        self.focused = Some(match self.focused {
            Some(0) | None => self.sections.len() - 1,
            Some(idx) => idx - 1,
        });
    }

    /// Toggle focused section
    pub fn toggle_focused(&mut self) {
        if let Some(idx) = self.focused
            && let Some(section) = self.sections.get_mut(idx) {
                section.toggle();
            }
    }

    /// Expand all sections
    pub fn expand_all(&mut self) {
        for section in &mut self.sections {
            section.expand();
        }
    }

    /// Collapse all sections
    pub fn collapse_all(&mut self) {
        for section in &mut self.sections {
            section.collapse();
        }
    }

    /// Render the collapsible list
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        if self.sections.is_empty() {
            return;
        }

        // Calculate heights for each section
        let mut constraints = Vec::new();
        for section in self.sections.iter() {
            // Header always takes 1 line
            constraints.push(Constraint::Length(3));

            // Content takes space if expanded
            if section.is_expanded() {
                let lines = section.content().lines().count() as u16;
                constraints.push(Constraint::Length(lines + 2)); // +2 for borders
            }
        }

        // If we have more constraints than space, use proportional
        if constraints.len() > area.height as usize {
            constraints = vec![Constraint::Percentage(100)];
        }

        let chunks = Layout::default().constraints(constraints).split(area);

        let mut chunk_idx = 0;
        for (section_idx, section) in self.sections.iter().enumerate() {
            if chunk_idx >= chunks.len() {
                break;
            }

            let is_focused = Some(section_idx) == self.focused;

            // Render header
            let header_style = if is_focused {
                Style::default()
                    .fg(ToadTheme::TOAD_GREEN)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(ToadTheme::FOREGROUND)
            };

            let border_style = if is_focused {
                Style::default().fg(ToadTheme::TOAD_GREEN)
            } else {
                Style::default().fg(ToadTheme::DARK_GRAY)
            };

            let header_text = Line::from(vec![
                Span::styled(section.indicator(), header_style),
                Span::raw(" "),
                Span::styled(&section.title, header_style),
            ]);

            let header_block = Block::default()
                .borders(Borders::ALL)
                .border_style(border_style);

            let header_para = Paragraph::new(header_text).block(header_block);
            frame.render_widget(header_para, chunks[chunk_idx]);
            chunk_idx += 1;

            // Render content if expanded
            if section.is_expanded() && chunk_idx < chunks.len() {
                let content_block = Block::default()
                    .borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM)
                    .border_style(border_style);

                let content_para = Paragraph::new(section.content())
                    .block(content_block)
                    .style(Style::default().fg(ToadTheme::FOREGROUND));

                frame.render_widget(content_para, chunks[chunk_idx]);
                chunk_idx += 1;
            }
        }
    }
}

impl Default for CollapsibleList {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_section_creation() {
        let section = CollapsibleSection::new("Test", "Content");
        assert_eq!(section.title(), "Test");
        assert_eq!(section.content(), "Content");
        assert!(!section.is_expanded());
        assert!(section.is_collapsible());
    }

    #[test]
    fn test_section_expanded() {
        let section = CollapsibleSection::expanded("Test", "Content");
        assert!(section.is_expanded());
    }

    #[test]
    fn test_section_toggle() {
        let mut section = CollapsibleSection::new("Test", "Content");
        assert!(!section.is_expanded());

        section.toggle();
        assert!(section.is_expanded());

        section.toggle();
        assert!(!section.is_expanded());
    }

    #[test]
    fn test_section_expand_collapse() {
        let mut section = CollapsibleSection::new("Test", "Content");

        section.expand();
        assert!(section.is_expanded());

        section.collapse();
        assert!(!section.is_expanded());
    }

    #[test]
    fn test_section_non_collapsible() {
        let mut section = CollapsibleSection::expanded("Test", "Content").with_collapsible(false);
        assert!(section.is_expanded());
        assert!(!section.is_collapsible());

        section.collapse();
        assert!(section.is_expanded()); // Should remain expanded
    }

    #[test]
    fn test_section_indicator() {
        let mut section = CollapsibleSection::new("Test", "Content");
        assert_eq!(section.indicator(), "▶");

        section.expand();
        assert_eq!(section.indicator(), "▼");
    }

    #[test]
    fn test_list_creation() {
        let list = CollapsibleList::new();
        assert_eq!(list.len(), 0);
        assert!(list.is_empty());
    }

    #[test]
    fn test_list_add() {
        let mut list = CollapsibleList::new();
        list.add(CollapsibleSection::new("Section 1", "Content 1"));
        list.add(CollapsibleSection::new("Section 2", "Content 2"));

        assert_eq!(list.len(), 2);
        assert!(!list.is_empty());
    }

    #[test]
    fn test_list_focus() {
        let mut list = CollapsibleList::new();
        list.add(CollapsibleSection::new("Section 1", "Content 1"));
        list.add(CollapsibleSection::new("Section 2", "Content 2"));

        assert_eq!(list.focused_index(), Some(0));

        list.focus_next();
        assert_eq!(list.focused_index(), Some(1));

        list.focus_previous();
        assert_eq!(list.focused_index(), Some(0));
    }

    #[test]
    fn test_list_focus_wrap() {
        let mut list = CollapsibleList::new();
        list.add(CollapsibleSection::new("Section 1", "Content 1"));
        list.add(CollapsibleSection::new("Section 2", "Content 2"));

        list.set_focus(1);
        list.focus_next();
        assert_eq!(list.focused_index(), Some(0)); // Wrap to beginning

        list.focus_previous();
        assert_eq!(list.focused_index(), Some(1)); // Wrap to end
    }

    #[test]
    fn test_list_toggle_focused() {
        let mut list = CollapsibleList::new();
        list.add(CollapsibleSection::new("Section 1", "Content 1"));

        assert!(!list.sections()[0].is_expanded());

        list.toggle_focused();
        assert!(list.sections()[0].is_expanded());

        list.toggle_focused();
        assert!(!list.sections()[0].is_expanded());
    }

    #[test]
    fn test_list_expand_collapse_all() {
        let mut list = CollapsibleList::new();
        list.add(CollapsibleSection::new("Section 1", "Content 1"));
        list.add(CollapsibleSection::new("Section 2", "Content 2"));

        list.expand_all();
        assert!(list.sections()[0].is_expanded());
        assert!(list.sections()[1].is_expanded());

        list.collapse_all();
        assert!(!list.sections()[0].is_expanded());
        assert!(!list.sections()[1].is_expanded());
    }

    #[test]
    fn test_section_set_content() {
        let mut section = CollapsibleSection::new("Test", "Old content");
        assert_eq!(section.content(), "Old content");

        section.set_content("New content");
        assert_eq!(section.content(), "New content");
    }
}
