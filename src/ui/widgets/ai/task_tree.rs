//! Hierarchical task decomposition tree widget
//!
//! Displays and manages tasks in a tree structure with expand/collapse,
//! status tracking, progress bars, time estimates, and dependencies.
//!
//! # Features
//!
//! - Tree view with expand/collapse (▼ expanded, ▶ collapsed)
//! - Three-level hierarchy: Phase → Task → Subtask
//! - Status indicators: ✓ Complete, ● In Progress, ○ Pending, ⚠ Blocked
//! - Progress bars per phase
//! - Time tracking: estimated vs. actual
//! - Dependency visualization
//! - Manual task management: Space (complete), e (edit), + (add subtask)
//!
//! # Examples
//!
//! ```
//! use toad::ui::widgets::ai::TaskTree;
//!
//! let mut tree = TaskTree::new();
//! let phase_id = tree.add_phase("Backend Implementation");
//! let task_id = tree.add_task(phase_id, "Create JWT module");
//! tree.add_subtask(task_id, "Define TokenClaims struct");
//! ```

use crate::ui::atoms::Block;
use crate::ui::theme::ToadTheme;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{List, ListItem, ListState},
    Frame,
};
use std::time::{Duration, Instant};

/// Task status in the hierarchy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskStatus {
    /// Task is pending, not yet started
    Pending,
    /// Task is currently in progress
    InProgress,
    /// Task is complete
    Complete,
    /// Task is blocked by dependencies
    Blocked,
}

impl TaskStatus {
    /// Get color for status
    pub fn color(&self) -> Color {
        match self {
            TaskStatus::Pending => ToadTheme::GRAY,
            TaskStatus::InProgress => ToadTheme::BLUE,
            TaskStatus::Complete => ToadTheme::TOAD_GREEN,
            TaskStatus::Blocked => ToadTheme::WARNING,
        }
    }

    /// Get symbol for status
    pub fn symbol(&self) -> &'static str {
        match self {
            TaskStatus::Pending => "○",
            TaskStatus::InProgress => "●",
            TaskStatus::Complete => "✓",
            TaskStatus::Blocked => "⚠",
        }
    }

    /// Get text description
    pub fn text(&self) -> &'static str {
        match self {
            TaskStatus::Pending => "Pending",
            TaskStatus::InProgress => "In Progress",
            TaskStatus::Complete => "Complete",
            TaskStatus::Blocked => "Blocked",
        }
    }
}

/// Task node in the tree
#[derive(Debug, Clone)]
pub struct TaskNode {
    /// Unique task ID
    pub id: usize,
    /// Task description
    pub description: String,
    /// Current status
    pub status: TaskStatus,
    /// Parent task ID (None for phases)
    pub parent_id: Option<usize>,
    /// Whether this node is expanded
    pub expanded: bool,
    /// Estimated time in seconds
    pub estimated_time: Option<u64>,
    /// Actual time if completed
    pub actual_time: Option<u64>,
    /// Start time if in progress
    pub start_time: Option<Instant>,
    /// Dependencies (IDs of tasks that must complete first)
    pub dependencies: Vec<usize>,
    /// Depth level (0 = Phase, 1 = Task, 2 = Subtask)
    pub depth: usize,
}

impl TaskNode {
    /// Create a new task node
    pub fn new(
        id: usize,
        description: String,
        parent_id: Option<usize>,
        depth: usize,
    ) -> Self {
        Self {
            id,
            description,
            status: TaskStatus::Pending,
            parent_id,
            expanded: true,
            estimated_time: None,
            actual_time: None,
            start_time: None,
            dependencies: Vec::new(),
            depth,
        }
    }

    /// Start the task
    pub fn start(&mut self) {
        self.status = TaskStatus::InProgress;
        self.start_time = Some(Instant::now());
    }

    /// Complete the task
    pub fn complete(&mut self) {
        self.status = TaskStatus::Complete;
        if let Some(start) = self.start_time {
            self.actual_time = Some(start.elapsed().as_secs());
        }
    }

    /// Block the task
    pub fn block(&mut self) {
        self.status = TaskStatus::Blocked;
    }

    /// Toggle expanded/collapsed
    pub fn toggle_expanded(&mut self) {
        self.expanded = !self.expanded;
    }

    /// Get elapsed time if in progress
    pub fn elapsed(&self) -> Option<Duration> {
        self.start_time.map(|start| start.elapsed())
    }

    /// Check if dependencies are met
    pub fn dependencies_met(&self, tree: &[TaskNode]) -> bool {
        self.dependencies.iter().all(|dep_id| {
            tree.iter()
                .find(|n| n.id == *dep_id)
                .map(|n| n.status == TaskStatus::Complete)
                .unwrap_or(false)
        })
    }
}

/// Hierarchical task tree widget
///
/// Displays and manages tasks in a three-level hierarchy.
///
/// # Examples
///
/// ```
/// use toad::ui::widgets::ai::TaskTree;
///
/// let mut tree = TaskTree::new();
/// let phase = tree.add_phase("Implementation");
/// assert_eq!(tree.node_count(), 1);
/// ```
#[derive(Debug)]
pub struct TaskTree {
    /// All task nodes
    nodes: Vec<TaskNode>,
    /// Next available ID
    next_id: usize,
    /// Selected node index
    selected_index: usize,
    /// List state for rendering
    list_state: ListState,
    /// Show time estimates
    show_time: bool,
    /// Show dependencies
    show_dependencies: bool,
}

impl Default for TaskTree {
    fn default() -> Self {
        Self::new()
    }
}

impl TaskTree {
    /// Create a new task tree
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::widgets::ai::TaskTree;
    ///
    /// let tree = TaskTree::new();
    /// assert_eq!(tree.node_count(), 0);
    /// ```
    pub fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Self {
            nodes: Vec::new(),
            next_id: 0,
            selected_index: 0,
            list_state,
            show_time: true,
            show_dependencies: true,
        }
    }

    /// Get next ID and increment
    fn get_next_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    /// Add a phase (top-level)
    pub fn add_phase(&mut self, description: impl Into<String>) -> usize {
        let id = self.get_next_id();
        let node = TaskNode::new(id, description.into(), None, 0);
        self.nodes.push(node);
        id
    }

    /// Add a task under a phase
    pub fn add_task(&mut self, phase_id: usize, description: impl Into<String>) -> usize {
        let id = self.get_next_id();
        let node = TaskNode::new(id, description.into(), Some(phase_id), 1);
        self.nodes.push(node);
        id
    }

    /// Add a subtask under a task
    pub fn add_subtask(&mut self, task_id: usize, description: impl Into<String>) -> usize {
        let id = self.get_next_id();
        let node = TaskNode::new(id, description.into(), Some(task_id), 2);
        self.nodes.push(node);
        id
    }

    /// Get node count
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Get node by ID
    pub fn get_node(&self, id: usize) -> Option<&TaskNode> {
        self.nodes.iter().find(|n| n.id == id)
    }

    /// Get node by ID (mutable)
    pub fn get_node_mut(&mut self, id: usize) -> Option<&mut TaskNode> {
        self.nodes.iter_mut().find(|n| n.id == id)
    }

    /// Get selected node
    pub fn selected_node(&self) -> Option<&TaskNode> {
        self.get_visible_nodes().get(self.selected_index).copied()
    }

    /// Get selected node (mutable)
    pub fn selected_node_mut(&mut self) -> Option<&mut TaskNode> {
        let visible = self.get_visible_nodes();
        if let Some(node) = visible.get(self.selected_index) {
            let id = node.id;
            self.get_node_mut(id)
        } else {
            None
        }
    }

    /// Get visible nodes (respecting expanded/collapsed)
    fn get_visible_nodes(&self) -> Vec<&TaskNode> {
        let mut visible = Vec::new();

        for node in &self.nodes {
            // Always show phases
            if node.depth == 0 {
                visible.push(node);
            } else {
                // Check if parent is expanded
                if let Some(parent_id) = node.parent_id {
                    if let Some(parent) = self.get_node(parent_id) {
                        if parent.expanded {
                            // For subtasks, also check grandparent
                            if node.depth == 2 {
                                if let Some(grandparent_id) = parent.parent_id {
                                    if let Some(grandparent) = self.get_node(grandparent_id) {
                                        if grandparent.expanded {
                                            visible.push(node);
                                        }
                                    }
                                }
                            } else {
                                visible.push(node);
                            }
                        }
                    }
                }
            }
        }

        visible
    }

    /// Select next node
    pub fn select_next(&mut self) {
        let visible = self.get_visible_nodes();
        if !visible.is_empty() {
            self.selected_index = (self.selected_index + 1) % visible.len();
            self.list_state.select(Some(self.selected_index));
        }
    }

    /// Select previous node
    pub fn select_previous(&mut self) {
        let visible = self.get_visible_nodes();
        if !visible.is_empty() {
            self.selected_index = if self.selected_index == 0 {
                visible.len() - 1
            } else {
                self.selected_index - 1
            };
            self.list_state.select(Some(self.selected_index));
        }
    }

    /// Toggle selected node expanded/collapsed
    pub fn toggle_selected_expanded(&mut self) {
        if let Some(node) = self.selected_node_mut() {
            node.toggle_expanded();
        }
    }

    /// Complete selected task
    pub fn complete_selected(&mut self) {
        if let Some(node) = self.selected_node_mut() {
            node.complete();
        }
    }

    /// Start selected task
    pub fn start_selected(&mut self) {
        if let Some(node) = self.selected_node_mut() {
            node.start();
        }
    }

    /// Get progress for a phase
    fn phase_progress(&self, phase_id: usize) -> f64 {
        let children: Vec<_> = self
            .nodes
            .iter()
            .filter(|n| n.parent_id == Some(phase_id))
            .collect();

        if children.is_empty() {
            return 0.0;
        }

        let completed = children
            .iter()
            .filter(|n| n.status == TaskStatus::Complete)
            .count();

        (completed as f64 / children.len() as f64) * 100.0
    }

    /// Render the task tree
    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Min(0),    // Tree
                Constraint::Length(2), // Footer
            ])
            .split(area);

        self.render_header(frame, chunks[0]);
        self.render_tree(frame, chunks[1]);
        self.render_footer(frame, chunks[2]);
    }

    /// Render header
    fn render_header(&self, frame: &mut Frame, area: Rect) {
        let block = Block::themed("Task Decomposition").to_ratatui();
        let inner = block.inner(area);
        frame.render_widget(block, area);

        let total = self.nodes.len();
        let completed = self
            .nodes
            .iter()
            .filter(|n| n.status == TaskStatus::Complete)
            .count();
        let in_progress = self
            .nodes
            .iter()
            .filter(|n| n.status == TaskStatus::InProgress)
            .count();

        let info = format!(
            "Total: {} | Completed: {} | In Progress: {}",
            total, completed, in_progress
        );

        let paragraph = ratatui::widgets::Paragraph::new(info);
        frame.render_widget(paragraph, inner);
    }

    /// Render tree
    fn render_tree(&mut self, frame: &mut Frame, area: Rect) {
        let block = Block::themed("Tasks").to_ratatui();
        let inner = block.inner(area);
        frame.render_widget(block, area);

        let visible_nodes = self.get_visible_nodes();
        let show_time = self.show_time;

        // Copy all node data to avoid holding references to self
        struct NodeRenderData {
            depth: usize,
            expanded: bool,
            status: TaskStatus,
            description: String,
            actual_time: Option<u64>,
            estimated_time: Option<u64>,
            elapsed_secs: Option<u64>,
            has_children: bool,
            progress: f64,
        }

        let node_data: Vec<NodeRenderData> = visible_nodes
            .iter()
            .map(|node| {
                let has_children = self
                    .nodes
                    .iter()
                    .any(|n| n.parent_id == Some(node.id));
                let progress = if node.depth == 0 {
                    self.phase_progress(node.id)
                } else {
                    0.0
                };
                let elapsed_secs = node.elapsed().map(|d| d.as_secs());

                NodeRenderData {
                    depth: node.depth,
                    expanded: node.expanded,
                    status: node.status,
                    description: node.description.clone(),
                    actual_time: node.actual_time,
                    estimated_time: node.estimated_time,
                    elapsed_secs,
                    has_children,
                    progress,
                }
            })
            .collect();

        let items: Vec<ListItem> = node_data
            .iter()
            .map(|data| {
                let mut spans = vec![];

                // Indentation
                let indent = "  ".repeat(data.depth);
                spans.push(Span::raw(indent));

                // Expand/collapse indicator for phases and tasks
                if data.depth < 2 {
                    if data.has_children {
                        let symbol = if data.expanded { "▼" } else { "▶" };
                        spans.push(Span::styled(
                            format!("{} ", symbol),
                            Style::default().fg(ToadTheme::GRAY),
                        ));
                    } else {
                        spans.push(Span::raw("  "));
                    }
                }

                // Status indicator
                spans.push(Span::styled(
                    format!("{} ", data.status.symbol()),
                    Style::default().fg(data.status.color()),
                ));

                // Description
                let style = if data.depth == 0 {
                    Style::default()
                        .fg(ToadTheme::TOAD_GREEN)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(ToadTheme::FOREGROUND)
                };
                spans.push(Span::styled(&data.description, style));

                // Time info
                if show_time {
                    if let Some(actual) = data.actual_time {
                        spans.push(Span::styled(
                            format!(" ({}s)", actual),
                            Style::default().fg(ToadTheme::GRAY),
                        ));
                    } else if let Some(elapsed) = data.elapsed_secs {
                        spans.push(Span::styled(
                            format!(" ({}s)", elapsed),
                            Style::default().fg(ToadTheme::BLUE),
                        ));
                    } else if let Some(estimated) = data.estimated_time {
                        spans.push(Span::styled(
                            format!(" (~{}s)", estimated),
                            Style::default().fg(ToadTheme::GRAY),
                        ));
                    }
                }

                // Progress bar for phases
                if data.depth == 0 && data.progress > 0.0 {
                    spans.push(Span::styled(
                        format!(" [{:.0}%]", data.progress),
                        Style::default().fg(ToadTheme::TOAD_GREEN),
                    ));
                }

                ListItem::new(Line::from(spans))
            })
            .collect();

        let list = List::new(items).highlight_style(
            Style::default()
                .bg(ToadTheme::DARK_GRAY)
                .add_modifier(Modifier::BOLD),
        );

        frame.render_stateful_widget(list, inner, &mut self.list_state);
    }

    /// Render footer
    fn render_footer(&self, frame: &mut Frame, area: Rect) {
        let block = Block::themed("Controls").to_ratatui();
        let inner = block.inner(area);
        frame.render_widget(block, area);

        let controls = Line::from(vec![
            Span::styled("↑/↓", Style::default().fg(ToadTheme::BLUE)),
            Span::raw(" Navigate | "),
            Span::styled("Space", Style::default().fg(ToadTheme::BLUE)),
            Span::raw(" Complete | "),
            Span::styled("Enter", Style::default().fg(ToadTheme::BLUE)),
            Span::raw(" Expand/Collapse | "),
            Span::styled("s", Style::default().fg(ToadTheme::BLUE)),
            Span::raw(" Start"),
        ]);

        let paragraph = ratatui::widgets::Paragraph::new(controls);
        frame.render_widget(paragraph, inner);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_status_symbols() {
        assert_eq!(TaskStatus::Pending.symbol(), "○");
        assert_eq!(TaskStatus::InProgress.symbol(), "●");
        assert_eq!(TaskStatus::Complete.symbol(), "✓");
        assert_eq!(TaskStatus::Blocked.symbol(), "⚠");
    }

    #[test]
    fn test_task_node_creation() {
        let node = TaskNode::new(0, "Test task".to_string(), None, 0);
        assert_eq!(node.id, 0);
        assert_eq!(node.description, "Test task");
        assert_eq!(node.status, TaskStatus::Pending);
        assert!(node.expanded);
    }

    #[test]
    fn test_task_node_lifecycle() {
        let mut node = TaskNode::new(0, "Test".to_string(), None, 0);

        node.start();
        assert_eq!(node.status, TaskStatus::InProgress);
        assert!(node.start_time.is_some());

        node.complete();
        assert_eq!(node.status, TaskStatus::Complete);
        assert!(node.actual_time.is_some());
    }

    #[test]
    fn test_task_tree_creation() {
        let tree = TaskTree::new();
        assert_eq!(tree.node_count(), 0);
        assert_eq!(tree.next_id, 0);
    }

    #[test]
    fn test_add_phase() {
        let mut tree = TaskTree::new();
        let id = tree.add_phase("Phase 1");

        assert_eq!(tree.node_count(), 1);
        assert_eq!(id, 0);

        let node = tree.get_node(id).unwrap();
        assert_eq!(node.description, "Phase 1");
        assert_eq!(node.depth, 0);
        assert_eq!(node.parent_id, None);
    }

    #[test]
    fn test_add_task() {
        let mut tree = TaskTree::new();
        let phase_id = tree.add_phase("Phase 1");
        let task_id = tree.add_task(phase_id, "Task 1");

        assert_eq!(tree.node_count(), 2);

        let task = tree.get_node(task_id).unwrap();
        assert_eq!(task.description, "Task 1");
        assert_eq!(task.depth, 1);
        assert_eq!(task.parent_id, Some(phase_id));
    }

    #[test]
    fn test_add_subtask() {
        let mut tree = TaskTree::new();
        let phase_id = tree.add_phase("Phase");
        let task_id = tree.add_task(phase_id, "Task");
        let subtask_id = tree.add_subtask(task_id, "Subtask");

        assert_eq!(tree.node_count(), 3);

        let subtask = tree.get_node(subtask_id).unwrap();
        assert_eq!(subtask.description, "Subtask");
        assert_eq!(subtask.depth, 2);
        assert_eq!(subtask.parent_id, Some(task_id));
    }

    #[test]
    fn test_phase_progress() {
        let mut tree = TaskTree::new();
        let phase_id = tree.add_phase("Phase");
        let task1 = tree.add_task(phase_id, "Task 1");
        let task2 = tree.add_task(phase_id, "Task 2");

        assert_eq!(tree.phase_progress(phase_id), 0.0);

        tree.get_node_mut(task1).unwrap().complete();
        assert_eq!(tree.phase_progress(phase_id), 50.0);

        tree.get_node_mut(task2).unwrap().complete();
        assert_eq!(tree.phase_progress(phase_id), 100.0);
    }

    #[test]
    fn test_toggle_expanded() {
        let mut node = TaskNode::new(0, "Test".to_string(), None, 0);
        assert!(node.expanded);

        node.toggle_expanded();
        assert!(!node.expanded);

        node.toggle_expanded();
        assert!(node.expanded);
    }

    #[test]
    fn test_visible_nodes_collapsed() {
        let mut tree = TaskTree::new();
        let phase_id = tree.add_phase("Phase");
        tree.add_task(phase_id, "Task 1");
        tree.add_task(phase_id, "Task 2");

        // All visible when expanded
        assert_eq!(tree.get_visible_nodes().len(), 3);

        // Collapse phase
        tree.get_node_mut(phase_id).unwrap().expanded = false;
        assert_eq!(tree.get_visible_nodes().len(), 1); // Only phase visible
    }

    #[test]
    fn test_navigation() {
        let mut tree = TaskTree::new();
        tree.add_phase("Phase 1");
        tree.add_phase("Phase 2");

        assert_eq!(tree.selected_index, 0);

        tree.select_next();
        assert_eq!(tree.selected_index, 1);

        tree.select_next(); // Wraps around
        assert_eq!(tree.selected_index, 0);

        tree.select_previous();
        assert_eq!(tree.selected_index, 1);
    }

    #[test]
    fn test_get_next_id() {
        let mut tree = TaskTree::new();
        assert_eq!(tree.get_next_id(), 0);
        assert_eq!(tree.get_next_id(), 1);
        assert_eq!(tree.get_next_id(), 2);
    }

    #[test]
    fn test_dependencies_met() {
        let mut tree = TaskTree::new();
        let phase = tree.add_phase("Phase");
        let task1 = tree.add_task(phase, "Task 1");
        let task2 = tree.add_task(phase, "Task 2");

        // Task 2 depends on Task 1
        tree.get_node_mut(task2).unwrap().dependencies.push(task1);

        // Dependencies not met initially
        let task2_node = tree.get_node(task2).unwrap().clone();
        assert!(!task2_node.dependencies_met(&tree.nodes));

        // Complete Task 1
        tree.get_node_mut(task1).unwrap().complete();

        // Now dependencies are met
        let task2_node = tree.get_node(task2).unwrap().clone();
        assert!(task2_node.dependencies_met(&tree.nodes));
    }
}
