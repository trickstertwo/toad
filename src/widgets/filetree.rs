//! File tree widget
//!
//! Collapsible directory tree for browsing files

use crate::theme::ToadTheme;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, Borders, List, ListItem, ListState, Scrollbar, ScrollbarOrientation, ScrollbarState,
    },
};
use std::path::{Path, PathBuf};

/// Tree node types
#[derive(Debug, Clone, PartialEq)]
pub enum FileTreeNodeType {
    Directory,
    File,
}

/// A node in the file tree
#[derive(Debug, Clone)]
pub struct FileTreeNode {
    pub path: PathBuf,
    pub name: String,
    pub node_type: FileTreeNodeType,
    pub depth: usize,
    pub is_expanded: bool,
    pub children: Vec<FileTreeNode>,
}

impl FileTreeNode {
    /// Create a new file node
    pub fn file(path: PathBuf, name: String, depth: usize) -> Self {
        Self {
            path,
            name,
            node_type: FileTreeNodeType::File,
            depth,
            is_expanded: false,
            children: Vec::new(),
        }
    }

    /// Create a new directory node
    pub fn directory(path: PathBuf, name: String, depth: usize) -> Self {
        Self {
            path,
            name,
            node_type: FileTreeNodeType::Directory,
            depth,
            is_expanded: false,
            children: Vec::new(),
        }
    }

    /// Toggle expansion state
    pub fn toggle(&mut self) {
        if self.node_type == FileTreeNodeType::Directory {
            self.is_expanded = !self.is_expanded;
        }
    }

    /// Load children from filesystem
    pub fn load_children(&mut self) -> std::io::Result<()> {
        if self.node_type != FileTreeNodeType::Directory {
            return Ok(());
        }

        self.children.clear();

        let entries = std::fs::read_dir(&self.path)?;
        let mut dirs = Vec::new();
        let mut files = Vec::new();

        for entry in entries.flatten() {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();

            // Skip hidden files and common ignore patterns
            if name.starts_with('.') || name == "target" || name == "node_modules" {
                continue;
            }

            if path.is_dir() {
                dirs.push(FileTreeNode::directory(path, name, self.depth + 1));
            } else {
                files.push(FileTreeNode::file(path, name, self.depth + 1));
            }
        }

        // Sort alphabetically
        dirs.sort_by(|a, b| a.name.cmp(&b.name));
        files.sort_by(|a, b| a.name.cmp(&b.name));

        // Directories first, then files
        self.children.extend(dirs);
        self.children.extend(files);

        Ok(())
    }
}

/// File tree widget
pub struct FileTree {
    root: FileTreeNode,
    flattened: Vec<usize>, // Indices into the tree for visible items
    list_state: ListState,
    title: String,
}

impl FileTree {
    /// Create a new file tree rooted at the given path
    pub fn new(root_path: PathBuf) -> std::io::Result<Self> {
        let name = root_path
            .file_name()
            .unwrap_or(root_path.as_os_str())
            .to_string_lossy()
            .to_string();

        let mut root = FileTreeNode::directory(root_path.clone(), name, 0);
        root.is_expanded = true;
        root.load_children()?;

        let mut tree = Self {
            root,
            flattened: Vec::new(),
            list_state: ListState::default(),
            title: "File Tree".to_string(),
        };

        tree.rebuild_flattened();
        if !tree.flattened.is_empty() {
            tree.list_state.select(Some(0));
        }

        Ok(tree)
    }

    /// Set the title
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Get the currently selected path
    pub fn selected_path(&self) -> Option<&Path> {
        self.list_state
            .selected()
            .and_then(|i| self.get_node_by_flat_index(i))
            .map(|node| node.path.as_path())
    }

    /// Toggle the currently selected directory
    pub fn toggle_selected(&mut self) {
        if let Some(i) = self.list_state.selected() {
            self.toggle_node(i);
            self.rebuild_flattened();
        }
    }

    /// Select the next item
    pub fn select_next(&mut self) {
        if self.flattened.is_empty() {
            return;
        }

        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.flattened.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    /// Select the previous item
    pub fn select_previous(&mut self) {
        if self.flattened.is_empty() {
            return;
        }

        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.flattened.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    /// Rebuild the flattened view
    fn rebuild_flattened(&mut self) {
        self.flattened.clear();
        Self::flatten_recursive_helper(&self.root, &mut self.flattened);
    }

    /// Recursively flatten visible nodes (helper function)
    fn flatten_recursive_helper(node: &FileTreeNode, flattened: &mut Vec<usize>) {
        // Add this node
        flattened.push(node.depth);

        // If expanded, add children
        if node.is_expanded {
            for child in &node.children {
                Self::flatten_recursive_helper(child, flattened);
            }
        }
    }

    /// Get a node by flattened index
    fn get_node_by_flat_index(&self, flat_index: usize) -> Option<&FileTreeNode> {
        if flat_index >= self.flattened.len() {
            return None;
        }

        // Rebuild path to node
        let mut nodes_seen = 0;
        Self::find_node_at_flat_index(&self.root, flat_index, &mut nodes_seen)
    }

    /// Recursively find node at flat index
    fn find_node_at_flat_index<'a>(
        node: &'a FileTreeNode,
        target_index: usize,
        nodes_seen: &mut usize,
    ) -> Option<&'a FileTreeNode> {
        if *nodes_seen == target_index {
            return Some(node);
        }
        *nodes_seen += 1;

        if node.is_expanded {
            for child in &node.children {
                if let Some(found) = Self::find_node_at_flat_index(child, target_index, nodes_seen)
                {
                    return Some(found);
                }
            }
        }

        None
    }

    /// Get mutable node by flattened index
    fn get_node_by_flat_index_mut(&mut self, flat_index: usize) -> Option<&mut FileTreeNode> {
        if flat_index >= self.flattened.len() {
            return None;
        }

        let mut nodes_seen = 0;
        Self::find_node_at_flat_index_mut_helper(&mut self.root, flat_index, &mut nodes_seen)
    }

    /// Recursively find mutable node at flat index (helper function)
    fn find_node_at_flat_index_mut_helper<'a>(
        node: &'a mut FileTreeNode,
        target_index: usize,
        nodes_seen: &mut usize,
    ) -> Option<&'a mut FileTreeNode> {
        if *nodes_seen == target_index {
            return Some(node);
        }
        *nodes_seen += 1;

        if node.is_expanded {
            for child in &mut node.children {
                if let Some(found) =
                    Self::find_node_at_flat_index_mut_helper(child, target_index, nodes_seen)
                {
                    return Some(found);
                }
            }
        }

        None
    }

    /// Toggle a node at the given flat index
    fn toggle_node(&mut self, flat_index: usize) {
        if let Some(node) = self.get_node_by_flat_index_mut(flat_index) {
            if node.node_type == FileTreeNodeType::Directory && node.children.is_empty() {
                let _ = node.load_children();
            }
            node.toggle();
        }
    }

    /// Render the file tree
    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let block = Block::default()
            .title(format!(" {} ", self.title))
            .title_style(
                Style::default()
                    .fg(ToadTheme::TOAD_GREEN)
                    .add_modifier(Modifier::BOLD),
            )
            .borders(Borders::ALL)
            .border_style(Style::default().fg(ToadTheme::TOAD_GREEN));

        let inner = block.inner(area);
        frame.render_widget(block, area);

        // Build list items
        let items: Vec<ListItem> = {
            let mut nodes_seen = 0;
            Self::build_list_items(&self.root, &mut nodes_seen)
        };

        let list = List::new(items)
            .highlight_style(
                Style::default()
                    .bg(ToadTheme::TOAD_GREEN_DARK)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("» ");

        frame.render_stateful_widget(list, inner, &mut self.list_state);

        // Render scrollbar
        if !self.flattened.is_empty() {
            let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .style(Style::default().fg(ToadTheme::DARK_GRAY))
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓"));

            let mut scrollbar_state = ScrollbarState::new(self.flattened.len())
                .position(self.list_state.selected().unwrap_or(0));

            frame.render_stateful_widget(scrollbar, inner, &mut scrollbar_state);
        }
    }

    /// Recursively build list items
    fn build_list_items(node: &FileTreeNode, nodes_seen: &mut usize) -> Vec<ListItem<'static>> {
        let mut items = Vec::new();

        // Add this node
        let indent = "  ".repeat(node.depth);
        let icon = match node.node_type {
            FileTreeNodeType::Directory => {
                if node.is_expanded {
                    "▼ "
                } else {
                    "▶ "
                }
            }
            FileTreeNodeType::File => "  ",
        };

        let style = if node.node_type == FileTreeNodeType::Directory {
            Style::default()
                .fg(ToadTheme::TOAD_GREEN_BRIGHT)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(ToadTheme::FOREGROUND)
        };

        // Build line by cloning strings to make it 'static
        let line_text = format!("{}{}{}", indent, icon, node.name);
        let line = Line::from(Span::styled(line_text, style));

        items.push(ListItem::new(line));
        *nodes_seen += 1;

        // Add children if expanded
        if node.is_expanded {
            for child in &node.children {
                items.extend(Self::build_list_items(child, nodes_seen));
            }
        }

        items
    }
}
