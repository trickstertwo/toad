//! Git conflict resolution widget
//!
//! Provides a visual interface for resolving merge conflicts with
//! side-by-side comparison and intelligent conflict detection.
//!
//! # Examples
//!
//! ```
//! use toad::ui::widgets::ConflictResolver;
//!
//! let conflict = r#"
//! <<<<<<< HEAD
//! fn hello() {
//!     println!("Hello from main");
//! }
//! =======
//! fn hello() {
//!     println!("Hello from branch");
//! }
//! >>>>>>> feature-branch
//! "#;
//!
//! let mut resolver = ConflictResolver::new();
//! resolver.load_conflict("src/main.rs", conflict);
//! ```

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, StatefulWidget, Widget},
};

/// Conflict region in a file
#[derive(Debug, Clone)]
pub struct ConflictRegion {
    /// File path
    pub file_path: String,
    /// Lines from current branch (ours)
    pub ours: Vec<String>,
    /// Lines from incoming branch (theirs)
    pub theirs: Vec<String>,
    /// Common ancestor lines (if available)
    pub base: Option<Vec<String>>,
    /// Start line in file
    pub start_line: usize,
    /// End line in file
    pub end_line: usize,
    /// Current resolution choice
    pub resolution: ConflictResolution,
}

impl ConflictRegion {
    /// Create a new conflict region
    pub fn new(file_path: impl Into<String>, start_line: usize, end_line: usize) -> Self {
        Self {
            file_path: file_path.into(),
            ours: Vec::new(),
            theirs: Vec::new(),
            base: None,
            start_line,
            end_line,
            resolution: ConflictResolution::Unresolved,
        }
    }

    /// Add a line to "ours" (current branch)
    pub fn add_ours(&mut self, line: impl Into<String>) {
        self.ours.push(line.into());
    }

    /// Add a line to "theirs" (incoming branch)
    pub fn add_theirs(&mut self, line: impl Into<String>) {
        self.theirs.push(line.into());
    }

    /// Set base lines
    pub fn set_base(&mut self, lines: Vec<String>) {
        self.base = Some(lines);
    }

    /// Choose our version
    pub fn choose_ours(&mut self) {
        self.resolution = ConflictResolution::ChooseOurs;
    }

    /// Choose their version
    pub fn choose_theirs(&mut self) {
        self.resolution = ConflictResolution::ChooseTheirs;
    }

    /// Choose both (concatenate)
    pub fn choose_both(&mut self) {
        self.resolution = ConflictResolution::ChooseBoth;
    }

    /// Mark as manually edited
    pub fn mark_manual(&mut self) {
        self.resolution = ConflictResolution::ManualEdit;
    }

    /// Get resolved content
    pub fn get_resolved_content(&self) -> Vec<String> {
        match &self.resolution {
            ConflictResolution::ChooseOurs => self.ours.clone(),
            ConflictResolution::ChooseTheirs => self.theirs.clone(),
            ConflictResolution::ChooseBoth => {
                let mut result = self.ours.clone();
                result.extend(self.theirs.clone());
                result
            }
            ConflictResolution::ManualEdit => {
                // Would be handled by external editor
                self.ours.clone()
            }
            ConflictResolution::Unresolved => Vec::new(),
        }
    }

    /// Check if resolved
    pub fn is_resolved(&self) -> bool {
        !matches!(self.resolution, ConflictResolution::Unresolved)
    }
}

/// Resolution choice for a conflict
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConflictResolution {
    /// Not yet resolved
    Unresolved,
    /// Choose current branch version
    ChooseOurs,
    /// Choose incoming branch version
    ChooseTheirs,
    /// Keep both versions
    ChooseBoth,
    /// Manually edited resolution
    ManualEdit,
}

impl ConflictResolution {
    /// Get color for this resolution
    pub fn color(&self) -> Color {
        match self {
            ConflictResolution::Unresolved => Color::Red,
            ConflictResolution::ChooseOurs => Color::Blue,
            ConflictResolution::ChooseTheirs => Color::Green,
            ConflictResolution::ChooseBoth => Color::Yellow,
            ConflictResolution::ManualEdit => Color::Magenta,
        }
    }

    /// Get icon for this resolution
    pub fn icon(&self) -> &'static str {
        match self {
            ConflictResolution::Unresolved => "⚠",
            ConflictResolution::ChooseOurs => "◀",
            ConflictResolution::ChooseTheirs => "▶",
            ConflictResolution::ChooseBoth => "⬌",
            ConflictResolution::ManualEdit => "✎",
        }
    }

    /// Get label for this resolution
    pub fn label(&self) -> &'static str {
        match self {
            ConflictResolution::Unresolved => "UNRESOLVED",
            ConflictResolution::ChooseOurs => "OURS",
            ConflictResolution::ChooseTheirs => "THEIRS",
            ConflictResolution::ChooseBoth => "BOTH",
            ConflictResolution::ManualEdit => "MANUAL",
        }
    }
}

/// View mode for conflict display
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConflictViewMode {
    /// Side-by-side comparison
    SideBySide,
    /// Unified view with markers
    Unified,
    /// Three-way merge (with base)
    ThreeWay,
}

/// Conflict resolver widget
pub struct ConflictResolver {
    /// List of conflicts
    conflicts: Vec<ConflictRegion>,
    /// Current conflict index
    current_conflict: usize,
    /// List state
    list_state: ListState,
    /// View mode
    view_mode: ConflictViewMode,
    /// Branch names
    our_branch: String,
    their_branch: String,
}

impl ConflictResolver {
    /// Create a new conflict resolver
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::widgets::ConflictResolver;
    ///
    /// let resolver = ConflictResolver::new();
    /// assert_eq!(resolver.conflict_count(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            conflicts: Vec::new(),
            current_conflict: 0,
            list_state: ListState::default(),
            view_mode: ConflictViewMode::SideBySide,
            our_branch: "HEAD".to_string(),
            their_branch: "MERGE_HEAD".to_string(),
        }
    }

    /// Set branch names
    pub fn set_branches(&mut self, ours: impl Into<String>, theirs: impl Into<String>) {
        self.our_branch = ours.into();
        self.their_branch = theirs.into();
    }

    /// Load a conflict from text with markers
    pub fn load_conflict(&mut self, file_path: impl Into<String>, content: &str) {
        let path = file_path.into();
        let lines: Vec<&str> = content.lines().collect();

        let mut i = 0;
        while i < lines.len() {
            if lines[i].starts_with("<<<<<<<") {
                let start_line = i;
                let mut ours = Vec::new();
                let mut theirs = Vec::new();

                // Skip conflict marker
                i += 1;

                // Read "ours" section
                while i < lines.len() && !lines[i].starts_with("=======") {
                    ours.push(lines[i].to_string());
                    i += 1;
                }

                // Skip separator
                if i < lines.len() {
                    i += 1;
                }

                // Read "theirs" section
                while i < lines.len() && !lines[i].starts_with(">>>>>>>") {
                    theirs.push(lines[i].to_string());
                    i += 1;
                }

                let end_line = i;

                let mut conflict = ConflictRegion::new(&path, start_line, end_line);
                conflict.ours = ours;
                conflict.theirs = theirs;

                self.conflicts.push(conflict);
            }
            i += 1;
        }

        if !self.conflicts.is_empty() {
            self.list_state.select(Some(0));
        }
    }

    /// Add a conflict region manually
    pub fn add_conflict(&mut self, conflict: ConflictRegion) {
        self.conflicts.push(conflict);
        if self.conflicts.len() == 1 {
            self.list_state.select(Some(0));
        }
    }

    /// Choose "ours" for current conflict
    pub fn choose_ours(&mut self) {
        if let Some(conflict) = self.conflicts.get_mut(self.current_conflict) {
            conflict.choose_ours();
        }
        self.next_conflict();
    }

    /// Choose "theirs" for current conflict
    pub fn choose_theirs(&mut self) {
        if let Some(conflict) = self.conflicts.get_mut(self.current_conflict) {
            conflict.choose_theirs();
        }
        self.next_conflict();
    }

    /// Choose both for current conflict
    pub fn choose_both(&mut self) {
        if let Some(conflict) = self.conflicts.get_mut(self.current_conflict) {
            conflict.choose_both();
        }
        self.next_conflict();
    }

    /// Move to next conflict
    pub fn next_conflict(&mut self) {
        if !self.conflicts.is_empty() {
            self.current_conflict = (self.current_conflict + 1) % self.conflicts.len();
            self.list_state.select(Some(self.current_conflict));
        }
    }

    /// Move to previous conflict
    pub fn previous_conflict(&mut self) {
        if !self.conflicts.is_empty() {
            self.current_conflict = if self.current_conflict == 0 {
                self.conflicts.len() - 1
            } else {
                self.current_conflict - 1
            };
            self.list_state.select(Some(self.current_conflict));
        }
    }

    /// Toggle view mode
    pub fn toggle_view_mode(&mut self) {
        self.view_mode = match self.view_mode {
            ConflictViewMode::SideBySide => ConflictViewMode::Unified,
            ConflictViewMode::Unified => ConflictViewMode::ThreeWay,
            ConflictViewMode::ThreeWay => ConflictViewMode::SideBySide,
        };
    }

    /// Get current conflict
    pub fn current(&self) -> Option<&ConflictRegion> {
        self.conflicts.get(self.current_conflict)
    }

    /// Get conflict count
    pub fn conflict_count(&self) -> usize {
        self.conflicts.len()
    }

    /// Get resolved count
    pub fn resolved_count(&self) -> usize {
        self.conflicts.iter().filter(|c| c.is_resolved()).count()
    }

    /// Check if all conflicts are resolved
    pub fn all_resolved(&self) -> bool {
        !self.conflicts.is_empty() && self.conflicts.iter().all(|c| c.is_resolved())
    }

    /// Get resolved file content
    pub fn get_resolved_file(&self, file_path: &str) -> Option<String> {
        let file_conflicts: Vec<&ConflictRegion> = self
            .conflicts
            .iter()
            .filter(|c| c.file_path == file_path && c.is_resolved())
            .collect();

        if file_conflicts.is_empty() {
            return None;
        }

        let mut result = String::new();
        for conflict in file_conflicts {
            for line in conflict.get_resolved_content() {
                result.push_str(&line);
                result.push('\n');
            }
        }

        Some(result)
    }

    /// Clear all conflicts
    pub fn clear(&mut self) {
        self.conflicts.clear();
        self.current_conflict = 0;
        self.list_state.select(None);
    }
}

impl Default for ConflictResolver {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for &ConflictResolver {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Split into list and content
        let chunks = Layout::horizontal([
            Constraint::Percentage(30),
            Constraint::Percentage(70),
        ])
        .split(area);

        // Render conflict list
        let items: Vec<ListItem> = self
            .conflicts
            .iter()
            .enumerate()
            .map(|(idx, conflict)| {
                let icon = conflict.resolution.icon();
                let color = conflict.resolution.color();
                let label = conflict.resolution.label();

                let content = vec![Line::from(vec![
                    Span::styled(format!("{} ", icon), Style::default().fg(color)),
                    Span::styled(
                        format!("[{}] ", label),
                        Style::default().fg(color).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        format!("Conflict {} ", idx + 1),
                        Style::default().fg(Color::White),
                    ),
                    Span::styled(
                        format!("({}:{})", conflict.start_line, conflict.end_line),
                        Style::default().fg(Color::DarkGray),
                    ),
                ])];

                ListItem::new(content)
            })
            .collect();

        let title = format!(
            "Conflicts ({}/{} resolved)",
            self.resolved_count(),
            self.conflict_count()
        );

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(title)
                    .border_style(Style::default().fg(Color::Yellow)),
            )
            .highlight_style(
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            );

        let mut list_state = self.list_state.clone();
        StatefulWidget::render(list, chunks[0], buf, &mut list_state);

        // Render current conflict
        if let Some(conflict) = self.current() {
            match self.view_mode {
                ConflictViewMode::SideBySide => {
                    let content_chunks = Layout::horizontal([
                        Constraint::Percentage(50),
                        Constraint::Percentage(50),
                    ])
                    .split(chunks[1]);


                    // Left: Ours
                    let ours_text: Vec<Line> = conflict
                        .ours
                        .iter()
                        .map(|line| {
                            Line::from(Span::styled(
                                line,
                                Style::default().fg(Color::Blue),
                            ))
                        })
                        .collect();

                    let ours_para = Paragraph::new(ours_text)
                        .block(
                            Block::default()
                                .borders(Borders::ALL)
                                .title(format!("◀ {} (Ours)", self.our_branch))
                                .border_style(Style::default().fg(Color::Blue)),
                        )
                        .wrap(ratatui::widgets::Wrap { trim: false });

                    ours_para.render(content_chunks[0], buf);

                    // Right: Theirs
                    let theirs_text: Vec<Line> = conflict
                        .theirs
                        .iter()
                        .map(|line| {
                            Line::from(Span::styled(
                                line,
                                Style::default().fg(Color::Green),
                            ))
                        })
                        .collect();

                    let theirs_para = Paragraph::new(theirs_text)
                        .block(
                            Block::default()
                                .borders(Borders::ALL)
                                .title(format!("▶ {} (Theirs)", self.their_branch))
                                .border_style(Style::default().fg(Color::Green)),
                        )
                        .wrap(ratatui::widgets::Wrap { trim: false });

                    theirs_para.render(content_chunks[1], buf);
                }
                ConflictViewMode::Unified | ConflictViewMode::ThreeWay => {
                    let content_area = chunks[1];

                    if self.view_mode == ConflictViewMode::Unified {
                    let mut lines = vec![
                        Line::from(Span::styled(
                            format!("<<<<<<< {}", self.our_branch),
                            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
                        )),
                    ];

                    for line in &conflict.ours {
                        lines.push(Line::from(Span::styled(
                            line,
                            Style::default().fg(Color::Blue),
                        )));
                    }

                    lines.push(Line::from(Span::styled(
                        "=======",
                        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
                    )));

                    for line in &conflict.theirs {
                        lines.push(Line::from(Span::styled(
                            line,
                            Style::default().fg(Color::Green),
                        )));
                    }

                    lines.push(Line::from(Span::styled(
                        format!(">>>>>>> {}", self.their_branch),
                        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
                    )));

                        let para = Paragraph::new(lines)
                            .block(
                                Block::default()
                                    .borders(Borders::ALL)
                                    .title("Unified View")
                                    .border_style(Style::default().fg(Color::Cyan)),
                            )
                            .wrap(ratatui::widgets::Wrap { trim: false });

                        para.render(content_area, buf);
                    } else {
                        // Three-way view
                        let mut lines = Vec::new();

                        if let Some(base) = &conflict.base {
                            lines.push(Line::from(Span::styled(
                                "BASE:",
                                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
                            )));
                            for line in base {
                                lines.push(Line::from(Span::styled(
                                    line,
                                    Style::default().fg(Color::Yellow),
                                )));
                            }
                            lines.push(Line::from(""));
                        }

                        lines.push(Line::from(Span::styled(
                            format!("OURS ({}):", self.our_branch),
                            Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD),
                        )));
                        for line in &conflict.ours {
                            lines.push(Line::from(Span::styled(
                                line,
                                Style::default().fg(Color::Blue),
                            )));
                        }

                        lines.push(Line::from(""));
                        lines.push(Line::from(Span::styled(
                            format!("THEIRS ({}):", self.their_branch),
                            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
                        )));
                        for line in &conflict.theirs {
                            lines.push(Line::from(Span::styled(
                                line,
                                Style::default().fg(Color::Green),
                            )));
                        }

                        let para = Paragraph::new(lines)
                            .block(
                                Block::default()
                                    .borders(Borders::ALL)
                                    .title("Three-Way View")
                                    .border_style(Style::default().fg(Color::Magenta)),
                            )
                            .wrap(ratatui::widgets::Wrap { trim: false });

                        para.render(content_area, buf);
                    }
                }
            }
        }

        // Render footer
        if area.height > 2 {
            let footer_area = Rect {
                x: area.x,
                y: area.y + area.height - 1,
                width: area.width,
                height: 1,
            };

            let footer_text = "o: Choose Ours | t: Choose Theirs | b: Both | v: Toggle View | ↑↓: Navigate";
            let footer = Paragraph::new(footer_text).style(Style::default().fg(Color::DarkGray));

            footer.render(footer_area, buf);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conflict_resolver_new() {
        let resolver = ConflictResolver::new();
        assert_eq!(resolver.conflict_count(), 0);
        assert_eq!(resolver.resolved_count(), 0);
    }

    #[test]
    fn test_load_conflict() {
        let mut resolver = ConflictResolver::new();
        let conflict_text = r#"
<<<<<<< HEAD
fn hello() {
    println!("Hello from main");
}
=======
fn hello() {
    println!("Hello from branch");
}
>>>>>>> feature-branch
"#;

        resolver.load_conflict("src/main.rs", conflict_text);
        assert_eq!(resolver.conflict_count(), 1);
    }

    #[test]
    fn test_choose_ours() {
        let mut resolver = ConflictResolver::new();
        let mut conflict = ConflictRegion::new("test.rs", 1, 5);
        conflict.add_ours("line from ours");
        conflict.add_theirs("line from theirs");
        resolver.add_conflict(conflict);

        resolver.choose_ours();
        assert_eq!(resolver.resolved_count(), 1);
    }

    #[test]
    fn test_choose_theirs() {
        let mut resolver = ConflictResolver::new();
        let mut conflict = ConflictRegion::new("test.rs", 1, 5);
        conflict.add_ours("line from ours");
        conflict.add_theirs("line from theirs");
        resolver.add_conflict(conflict);

        resolver.choose_theirs();
        assert_eq!(resolver.resolved_count(), 1);
    }

    #[test]
    fn test_choose_both() {
        let mut resolver = ConflictResolver::new();
        let mut conflict = ConflictRegion::new("test.rs", 1, 5);
        conflict.add_ours("ours");
        conflict.add_theirs("theirs");
        resolver.add_conflict(conflict);

        resolver.choose_both();

        let resolved = resolver.conflicts[0].get_resolved_content();
        assert_eq!(resolved.len(), 2);
        assert_eq!(resolved[0], "ours");
        assert_eq!(resolved[1], "theirs");
    }

    #[test]
    fn test_navigation() {
        let mut resolver = ConflictResolver::new();
        resolver.add_conflict(ConflictRegion::new("file1.rs", 1, 5));
        resolver.add_conflict(ConflictRegion::new("file2.rs", 10, 15));
        resolver.add_conflict(ConflictRegion::new("file3.rs", 20, 25));

        assert_eq!(resolver.current_conflict, 0);

        resolver.next_conflict();
        assert_eq!(resolver.current_conflict, 1);

        resolver.next_conflict();
        assert_eq!(resolver.current_conflict, 2);

        resolver.next_conflict(); // Wrap around
        assert_eq!(resolver.current_conflict, 0);

        resolver.previous_conflict();
        assert_eq!(resolver.current_conflict, 2);
    }

    #[test]
    fn test_all_resolved() {
        let mut resolver = ConflictResolver::new();
        let mut c1 = ConflictRegion::new("file1.rs", 1, 5);
        c1.choose_ours();
        let mut c2 = ConflictRegion::new("file2.rs", 10, 15);
        c2.choose_theirs();

        resolver.add_conflict(c1);
        resolver.add_conflict(c2);

        assert!(resolver.all_resolved());
    }

    #[test]
    fn test_conflict_region() {
        let mut region = ConflictRegion::new("test.rs", 1, 10);
        region.add_ours("our line 1");
        region.add_ours("our line 2");
        region.add_theirs("their line 1");

        assert!(!region.is_resolved());

        region.choose_ours();
        assert!(region.is_resolved());

        let content = region.get_resolved_content();
        assert_eq!(content.len(), 2);
    }

    #[test]
    fn test_toggle_view_mode() {
        let mut resolver = ConflictResolver::new();
        assert_eq!(resolver.view_mode, ConflictViewMode::SideBySide);

        resolver.toggle_view_mode();
        assert_eq!(resolver.view_mode, ConflictViewMode::Unified);

        resolver.toggle_view_mode();
        assert_eq!(resolver.view_mode, ConflictViewMode::ThreeWay);

        resolver.toggle_view_mode();
        assert_eq!(resolver.view_mode, ConflictViewMode::SideBySide);
    }

    #[test]
    fn test_set_branches() {
        let mut resolver = ConflictResolver::new();
        resolver.set_branches("main", "feature");

        assert_eq!(resolver.our_branch, "main");
        assert_eq!(resolver.their_branch, "feature");
    }
}
