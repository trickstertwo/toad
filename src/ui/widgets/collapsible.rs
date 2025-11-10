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
            && let Some(section) = self.sections.get_mut(idx)
        {
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

    // ============================================================================
    // ADVANCED COMPREHENSIVE EDGE CASE TESTS (90%+ COVERAGE)
    // ============================================================================

    // ============ Stress Tests ============

    #[test]
    fn test_list_10000_sections() {
        let mut list = CollapsibleList::new();

        for i in 0..10000 {
            list.add(CollapsibleSection::new(
                format!("Section {}", i),
                format!("Content {}", i),
            ));
        }

        assert_eq!(list.len(), 10000);
        assert_eq!(list.focused_index(), Some(0));
    }

    #[test]
    fn test_section_rapid_toggles() {
        let mut section = CollapsibleSection::new("Test", "Content");

        for _ in 0..5000 {
            section.toggle();
        }

        assert!(!section.is_expanded()); // Should be collapsed after even number of toggles
    }

    #[test]
    fn test_list_rapid_focus_navigation() {
        let mut list = CollapsibleList::new();
        for i in 0..10 {
            list.add(CollapsibleSection::new(format!("Section {}", i), "Content"));
        }

        for _ in 0..1000 {
            list.focus_next();
        }

        // 1000 % 10 = 0, so should be at index 0
        assert_eq!(list.focused_index(), Some(0));
    }

    #[test]
    fn test_list_rapid_expand_collapse_cycles() {
        let mut list = CollapsibleList::new();
        for i in 0..100 {
            list.add(CollapsibleSection::new(format!("Section {}", i), "Content"));
        }

        for _ in 0..1000 {
            list.expand_all();
            list.collapse_all();
        }

        // All should be collapsed
        for section in list.sections() {
            assert!(!section.is_expanded());
        }
    }

    #[test]
    fn test_list_alternating_toggle_focused() {
        let mut list = CollapsibleList::new();
        for i in 0..5 {
            list.add(CollapsibleSection::new(format!("Section {}", i), "Content"));
        }

        for _ in 0..1000 {
            list.toggle_focused();
            list.focus_next();
        }

        // 1000 toggles on 5 sections = 200 toggles each = all collapsed (even count)
        for section in list.sections() {
            assert!(!section.is_expanded());
        }
    }

    // ============ Unicode Edge Cases ============

    #[test]
    fn test_section_rtl_title_and_content() {
        let section = CollapsibleSection::new("مرحبا بك في TOAD", "שלום לעולם");
        assert!(section.title().contains("مرحبا"));
        assert!(section.content().contains("שלום"));
    }

    #[test]
    fn test_section_mixed_scripts() {
        let section = CollapsibleSection::new("Hello世界Привет안녕하세요", "こんにちは🌍Bonjour");
        assert!(section.title().contains("世界"));
        assert!(section.content().contains("こんにちは"));
    }

    #[test]
    fn test_section_combining_characters() {
        let section = CollapsibleSection::new("Café é̂ñ̃ỹ̀", "naïve résumé");
        assert!(section.title().len() > 5);
        assert!(section.content().len() > 10);
    }

    #[test]
    fn test_section_zero_width_characters() {
        let section = CollapsibleSection::new("Test\u{200B}\u{200C}\u{200D}", "Zero\u{FEFF}Width");
        assert!(section.title().contains("Test"));
        assert!(section.content().contains("Zero"));
    }

    #[test]
    fn test_section_emoji_variations() {
        let section = CollapsibleSection::new("🐸🚀💚", "👍🏻👍🏿🏴󠁧󠁢󠁥󠁮󠁧󠁿");
        assert!(section.title().contains('🐸'));
        assert!(section.content().contains("👍"));
    }

    #[test]
    fn test_list_unicode_sections() {
        let mut list = CollapsibleList::new();
        list.add(CollapsibleSection::new("مرحبا", "Content"));
        list.add(CollapsibleSection::new("שלום", "Content"));
        list.add(CollapsibleSection::new("日本語", "Content"));
        list.add(CollapsibleSection::new("🐸 Toad", "Content"));

        assert_eq!(list.len(), 4);
    }

    // ============ Extreme Sizes ============

    #[test]
    fn test_section_100k_title() {
        let huge_title = "X".repeat(100000);
        let section = CollapsibleSection::new(huge_title.clone(), "Content");
        assert_eq!(section.title().len(), 100000);
    }

    #[test]
    fn test_section_100k_content() {
        let huge_content = "Y".repeat(100000);
        let section = CollapsibleSection::new("Title", huge_content.clone());
        assert_eq!(section.content().len(), 100000);
    }

    #[test]
    fn test_section_multiline_content() {
        let multiline = "Line 1\nLine 2\nLine 3\n".repeat(1000);
        let section = CollapsibleSection::new("Title", multiline.clone());
        assert!(section.content().lines().count() >= 3000);
    }

    #[test]
    fn test_list_1000_sections_with_long_content() {
        let mut list = CollapsibleList::new();

        for i in 0..1000 {
            let content = format!("Content {}", i).repeat(100);
            list.add(CollapsibleSection::new(format!("Section {}", i), content));
        }

        assert_eq!(list.len(), 1000);
    }

    // ============ Complex Workflows ============

    #[test]
    fn test_list_add_toggle_collapse_expand() {
        let mut list = CollapsibleList::new();

        for i in 0..10 {
            list.add(CollapsibleSection::new(format!("Section {}", i), "Content"));
        }

        // Toggle first section
        list.toggle_focused();
        assert!(list.sections()[0].is_expanded());

        // Collapse all
        list.collapse_all();
        assert!(!list.sections()[0].is_expanded());

        // Expand all
        list.expand_all();
        assert!(list.sections()[0].is_expanded());
    }

    #[test]
    fn test_list_mixed_expanded_collapsed() {
        let mut list = CollapsibleList::new();

        for i in 0..10 {
            if i % 2 == 0 {
                list.add(CollapsibleSection::expanded(
                    format!("Section {}", i),
                    "Content",
                ));
            } else {
                list.add(CollapsibleSection::new(format!("Section {}", i), "Content"));
            }
        }

        assert!(list.sections()[0].is_expanded());
        assert!(!list.sections()[1].is_expanded());
        assert!(list.sections()[2].is_expanded());
    }

    #[test]
    fn test_list_focus_and_toggle_pattern() {
        let mut list = CollapsibleList::new();

        for i in 0..5 {
            list.add(CollapsibleSection::new(format!("Section {}", i), "Content"));
        }

        for _ in 0..20 {
            list.toggle_focused();
            list.focus_next();
        }

        // 20 operations, 5 sections = 4 toggles each = all collapsed
        for section in list.sections() {
            assert!(!section.is_expanded());
        }
    }

    #[test]
    fn test_section_content_update_workflow() {
        let mut section = CollapsibleSection::new("Title", "Original");

        section.set_content("Updated 1");
        assert_eq!(section.content(), "Updated 1");

        section.expand();
        section.set_content("Updated 2");
        assert_eq!(section.content(), "Updated 2");
        assert!(section.is_expanded());

        section.collapse();
        section.set_content("Updated 3");
        assert_eq!(section.content(), "Updated 3");
        assert!(!section.is_expanded());
    }

    #[test]
    fn test_list_section_mut_updates() {
        let mut list = CollapsibleList::new();
        list.add(CollapsibleSection::new("Section 1", "Content 1"));
        list.add(CollapsibleSection::new("Section 2", "Content 2"));

        if let Some(section) = list.section_mut(0) {
            section.set_content("Modified Content 1");
            section.expand();
        }

        assert_eq!(list.sections()[0].content(), "Modified Content 1");
        assert!(list.sections()[0].is_expanded());
    }

    // ============ Builder Pattern Edge Cases ============

    #[test]
    fn test_section_with_collapsible_chaining() {
        let section = CollapsibleSection::new("Title", "Content")
            .with_collapsible(false)
            .with_collapsible(true);

        assert!(section.is_collapsible());
    }

    #[test]
    fn test_section_expanded_with_collapsible() {
        let section = CollapsibleSection::expanded("Title", "Content").with_collapsible(false);

        assert!(section.is_expanded());
        assert!(!section.is_collapsible());

        let mut section2 = section.clone();
        section2.collapse();
        assert!(section2.is_expanded()); // Should remain expanded (non-collapsible)
    }

    // ============ Clone/Debug/Serialize Trait Coverage ============

    #[test]
    fn test_section_clone() {
        let section = CollapsibleSection::new("Title", "Content");
        let cloned = section.clone();

        assert_eq!(section.title(), cloned.title());
        assert_eq!(section.content(), cloned.content());
        assert_eq!(section.is_expanded(), cloned.is_expanded());
        assert_eq!(section.is_collapsible(), cloned.is_collapsible());
    }

    #[test]
    fn test_section_debug() {
        let section = CollapsibleSection::new("Title", "Content");
        let debug_str = format!("{:?}", section);

        assert!(debug_str.contains("CollapsibleSection"));
    }

    #[test]
    fn test_section_serialize_deserialize() {
        let section = CollapsibleSection::expanded("Title", "Content").with_collapsible(false);

        let json = serde_json::to_string(&section).unwrap();
        let deserialized: CollapsibleSection = serde_json::from_str(&json).unwrap();

        assert_eq!(section.title(), deserialized.title());
        assert_eq!(section.content(), deserialized.content());
        assert_eq!(section.is_expanded(), deserialized.is_expanded());
        assert_eq!(section.is_collapsible(), deserialized.is_collapsible());
    }

    #[test]
    fn test_list_clone() {
        let mut list = CollapsibleList::new();
        list.add(CollapsibleSection::new("Section 1", "Content 1"));
        list.add(CollapsibleSection::new("Section 2", "Content 2"));

        let cloned = list.clone();

        assert_eq!(list.len(), cloned.len());
        assert_eq!(list.focused_index(), cloned.focused_index());
    }

    #[test]
    fn test_list_debug() {
        let list = CollapsibleList::new();
        let debug_str = format!("{:?}", list);

        assert!(debug_str.contains("CollapsibleList"));
    }

    #[test]
    fn test_list_default() {
        let list = CollapsibleList::default();
        assert_eq!(list.len(), 0);
        assert!(list.is_empty());
    }

    // ============ Focus Edge Cases ============

    #[test]
    fn test_list_set_focus_out_of_bounds() {
        let mut list = CollapsibleList::new();
        list.add(CollapsibleSection::new("Section 1", "Content 1"));

        let result = list.set_focus(10);
        assert!(!result);
        assert_eq!(list.focused_index(), Some(0)); // Should remain unchanged
    }

    #[test]
    fn test_list_focus_empty_list() {
        let mut list = CollapsibleList::new();

        list.focus_next();
        assert_eq!(list.focused_index(), None);

        list.focus_previous();
        assert_eq!(list.focused_index(), None);
    }

    #[test]
    fn test_list_toggle_focused_empty() {
        let mut list = CollapsibleList::new();
        list.toggle_focused(); // Should not panic
    }

    #[test]
    fn test_list_section_mut_out_of_bounds() {
        let mut list = CollapsibleList::new();
        list.add(CollapsibleSection::new("Section 1", "Content 1"));

        assert!(list.section_mut(0).is_some());
        assert!(list.section_mut(10).is_none());
    }

    // ============ Indicator Edge Cases ============

    #[test]
    fn test_section_indicator_all_states() {
        let mut section = CollapsibleSection::new("Test", "Content");

        assert_eq!(section.indicator(), "▶");

        section.expand();
        assert_eq!(section.indicator(), "▼");

        section.collapse();
        assert_eq!(section.indicator(), "▶");

        section.toggle();
        assert_eq!(section.indicator(), "▼");
    }

    // ============ Empty/Whitespace Content ============

    #[test]
    fn test_section_empty_title_and_content() {
        let section = CollapsibleSection::new("", "");
        assert_eq!(section.title(), "");
        assert_eq!(section.content(), "");
    }

    #[test]
    fn test_section_whitespace_only() {
        let section = CollapsibleSection::new("   ", "   \n   ");
        assert_eq!(section.title(), "   ");
        assert!(section.content().contains('\n'));
    }

    #[test]
    fn test_section_special_characters() {
        let section = CollapsibleSection::new("Title<>\"'&", "Content|\\/*?");
        assert!(section.title().contains('<'));
        assert!(section.content().contains('|'));
    }

    // ============ Comprehensive Stress Test ============

    #[test]
    fn test_comprehensive_collapsible_stress() {
        let mut list = CollapsibleList::new();

        // Add sections with varied configurations
        for i in 0..100 {
            let title = match i % 4 {
                0 => format!("ASCII Section {}", i),
                1 => format!("🚀 Emoji Section {}", i),
                2 => format!("日本語 Section {}", i),
                _ => format!("مرحبا Section {}", i),
            };

            let content = match i % 3 {
                0 => "Short content".to_string(),
                1 => "Multi\nLine\nContent\nWith\nMany\nLines".to_string(),
                _ => format!("Long content {}", "X".repeat(100)),
            };

            if i % 2 == 0 {
                list.add(CollapsibleSection::expanded(title, content));
            } else {
                list.add(CollapsibleSection::new(title, content));
            }
        }

        // Verify count
        assert_eq!(list.len(), 100);

        // Navigate and toggle
        for _ in 0..200 {
            list.toggle_focused();
            list.focus_next();
        }

        // Expand all
        list.expand_all();
        for section in list.sections() {
            assert!(section.is_expanded());
        }

        // Collapse all
        list.collapse_all();
        for section in list.sections() {
            assert!(!section.is_expanded());
        }

        // Update some sections
        for i in 0..10 {
            if let Some(section) = list.section_mut(i) {
                section.set_content("Updated content");
                section.expand();
            }
        }

        assert_eq!(list.sections()[0].content(), "Updated content");
        assert!(list.sections()[0].is_expanded());

        // Test focus boundaries
        list.set_focus(0);
        assert_eq!(list.focused_index(), Some(0));

        list.set_focus(99);
        assert_eq!(list.focused_index(), Some(99));

        // Test wrapping
        list.focus_next();
        assert_eq!(list.focused_index(), Some(0));

        list.focus_previous();
        assert_eq!(list.focused_index(), Some(99));

        // Clone and verify
        let cloned = list.clone();
        assert_eq!(list.len(), cloned.len());

        // Final verification
        assert_eq!(list.len(), 100);
        assert!(!list.is_empty());
    }
}
