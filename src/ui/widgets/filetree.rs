//! File tree widget
//!
//! Collapsible directory tree for browsing files

use crate::services::FilesystemService;
use crate::ui::{
    atoms::{block::Block as AtomBlock, text::Text as AtomText},
    theme::ToadTheme,
};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    text::Line,
    widgets::{Borders, List, ListItem, ListState, Scrollbar, ScrollbarOrientation, ScrollbarState},
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

    /// Load children from filesystem using FilesystemService
    ///
    /// This method uses the FilesystemService to maintain separation of concerns
    /// between UI widgets and I/O operations.
    pub fn load_children(&mut self, fs_service: &FilesystemService) -> std::io::Result<()> {
        if self.node_type != FileTreeNodeType::Directory {
            return Ok(());
        }

        self.children.clear();

        let entries = fs_service.read_dir(&self.path)?;
        let mut dirs = Vec::new();
        let mut files = Vec::new();

        for entry in entries {
            // Skip hidden files and common ignore patterns
            if entry.file_name.starts_with('.')
                || entry.file_name == "target"
                || entry.file_name == "node_modules"
            {
                continue;
            }

            if entry.is_dir {
                dirs.push(FileTreeNode::directory(
                    entry.path,
                    entry.file_name,
                    self.depth + 1,
                ));
            } else {
                files.push(FileTreeNode::file(
                    entry.path,
                    entry.file_name,
                    self.depth + 1,
                ));
            }
        }

        // FilesystemService already sorts directories first, then alphabetically
        // So we don't need to re-sort here
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
    fs_service: FilesystemService,
}

impl FileTree {
    /// Create a new file tree rooted at the given path
    ///
    /// Uses FilesystemService for I/O operations (Separation of Concerns).
    pub fn new(root_path: PathBuf) -> std::io::Result<Self> {
        let fs_service = FilesystemService::new();

        let name = root_path
            .file_name()
            .unwrap_or(root_path.as_os_str())
            .to_string_lossy()
            .to_string();

        let mut root = FileTreeNode::directory(root_path.clone(), name, 0);
        root.is_expanded = true;
        root.load_children(&fs_service)?;

        let mut tree = Self {
            root,
            flattened: Vec::new(),
            list_state: ListState::default(),
            title: "File Tree".to_string(),
            fs_service,
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
        // First, check if we need to load children (immutable borrow)
        let needs_load = if let Some(node) = self.get_node_by_flat_index(flat_index) {
            node.node_type == FileTreeNodeType::Directory && node.children.is_empty()
        } else {
            false
        };

        // Load children if needed
        // Note: We create a new FilesystemService here to avoid borrow checker issues.
        // FilesystemService::new() is cheap (zero-cost) since it's just an empty struct.
        if needs_load {
            let fs_service = FilesystemService::new();
            if let Some(node) = self.get_node_by_flat_index_mut(flat_index) {
                let _ = node.load_children(&fs_service);
            }
        }

        // Toggle the node (mutable borrow, separate scope)
        if let Some(node) = self.get_node_by_flat_index_mut(flat_index) {
            node.toggle();
        }
    }

    /// Render the file tree
    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let block = AtomBlock::new()
            .title(format!(" {} ", self.title))
            .title_style(
                Style::default()
                    .fg(ToadTheme::TOAD_GREEN)
                    .add_modifier(Modifier::BOLD),
            )
            .borders(Borders::ALL)
            .border_style(Style::default().fg(ToadTheme::TOAD_GREEN))
            .to_ratatui();

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
        let line = Line::from(AtomText::new(line_text).style(style).to_span());

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_file_node_creation() {
        let path = PathBuf::from("/test/file.txt");
        let node = FileTreeNode::file(path.clone(), "file.txt".to_string(), 1);

        assert_eq!(node.path, path);
        assert_eq!(node.name, "file.txt");
        assert_eq!(node.node_type, FileTreeNodeType::File);
        assert_eq!(node.depth, 1);
        assert!(!node.is_expanded);
        assert!(node.children.is_empty());
    }

    #[test]
    fn test_directory_node_creation() {
        let path = PathBuf::from("/test/dir");
        let node = FileTreeNode::directory(path.clone(), "dir".to_string(), 0);

        assert_eq!(node.path, path);
        assert_eq!(node.name, "dir");
        assert_eq!(node.node_type, FileTreeNodeType::Directory);
        assert_eq!(node.depth, 0);
        assert!(!node.is_expanded);
        assert!(node.children.is_empty());
    }

    #[test]
    fn test_directory_toggle() {
        let mut node = FileTreeNode::directory(PathBuf::from("/test"), "test".to_string(), 0);

        assert!(!node.is_expanded);
        node.toggle();
        assert!(node.is_expanded);
        node.toggle();
        assert!(!node.is_expanded);
    }

    #[test]
    fn test_file_toggle_does_nothing() {
        let mut node = FileTreeNode::file(PathBuf::from("/test.txt"), "test.txt".to_string(), 0);

        assert!(!node.is_expanded);
        node.toggle();
        assert!(!node.is_expanded); // Should remain false for files
    }

    #[test]
    fn test_load_children_on_file_returns_ok() {
        let fs_service = FilesystemService::new();
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let file_path = temp_dir.path().join("test.txt");
        fs::write(&file_path, "content").expect("Failed to write test file");

        let mut node = FileTreeNode::file(file_path, "test.txt".to_string(), 0);
        let result = node.load_children(&fs_service);

        assert!(result.is_ok());
        assert!(node.children.is_empty()); // Files don't have children
    }

    #[test]
    fn test_load_children_empty_directory() {
        let fs_service = FilesystemService::new();
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let mut node =
            FileTreeNode::directory(temp_dir.path().to_path_buf(), "test".to_string(), 0);

        let result = node.load_children(&fs_service);
        assert!(result.is_ok());
        assert!(node.children.is_empty());
    }

    #[test]
    fn test_load_children_with_files() {
        let fs_service = FilesystemService::new();
        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        // Create some test files
        fs::write(temp_dir.path().join("file1.txt"), "content").expect("Failed to write file");
        fs::write(temp_dir.path().join("file2.txt"), "content").expect("Failed to write file");

        let mut node =
            FileTreeNode::directory(temp_dir.path().to_path_buf(), "test".to_string(), 0);
        node.load_children(&fs_service)
            .expect("Failed to load children");

        assert_eq!(node.children.len(), 2);
        assert!(
            node.children
                .iter()
                .all(|c| c.node_type == FileTreeNodeType::File)
        );
        assert!(node.children.iter().all(|c| c.depth == 1));
    }

    #[test]
    fn test_load_children_with_directories() {
        let fs_service = FilesystemService::new();
        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        // Create subdirectories
        fs::create_dir(temp_dir.path().join("dir1")).expect("Failed to create dir");
        fs::create_dir(temp_dir.path().join("dir2")).expect("Failed to create dir");

        let mut node =
            FileTreeNode::directory(temp_dir.path().to_path_buf(), "test".to_string(), 0);
        node.load_children(&fs_service)
            .expect("Failed to load children");

        assert_eq!(node.children.len(), 2);
        assert!(
            node.children
                .iter()
                .all(|c| c.node_type == FileTreeNodeType::Directory)
        );
    }

    #[test]
    fn test_load_children_mixed_content() {
        let fs_service = FilesystemService::new();
        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        // Create mixed content
        fs::create_dir(temp_dir.path().join("dir1")).expect("Failed to create dir");
        fs::write(temp_dir.path().join("file1.txt"), "content").expect("Failed to write file");
        fs::create_dir(temp_dir.path().join("dir2")).expect("Failed to create dir");
        fs::write(temp_dir.path().join("file2.txt"), "content").expect("Failed to write file");

        let mut node =
            FileTreeNode::directory(temp_dir.path().to_path_buf(), "test".to_string(), 0);
        node.load_children(&fs_service)
            .expect("Failed to load children");

        assert_eq!(node.children.len(), 4);
        // Directories should come first
        assert_eq!(node.children[0].node_type, FileTreeNodeType::Directory);
        assert_eq!(node.children[1].node_type, FileTreeNodeType::Directory);
        assert_eq!(node.children[2].node_type, FileTreeNodeType::File);
        assert_eq!(node.children[3].node_type, FileTreeNodeType::File);
    }

    #[test]
    fn test_load_children_alphabetical_sorting() {
        let fs_service = FilesystemService::new();
        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        // Create files in non-alphabetical order
        fs::write(temp_dir.path().join("zebra.txt"), "content").expect("Failed to write file");
        fs::write(temp_dir.path().join("apple.txt"), "content").expect("Failed to write file");
        fs::write(temp_dir.path().join("mango.txt"), "content").expect("Failed to write file");

        let mut node =
            FileTreeNode::directory(temp_dir.path().to_path_buf(), "test".to_string(), 0);
        node.load_children(&fs_service)
            .expect("Failed to load children");

        assert_eq!(node.children[0].name, "apple.txt");
        assert_eq!(node.children[1].name, "mango.txt");
        assert_eq!(node.children[2].name, "zebra.txt");
    }

    #[test]
    fn test_load_children_skips_hidden_files() {
        let fs_service = FilesystemService::new();
        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        // Create hidden and visible files
        fs::write(temp_dir.path().join(".hidden"), "content").expect("Failed to write file");
        fs::write(temp_dir.path().join("visible.txt"), "content").expect("Failed to write file");

        let mut node =
            FileTreeNode::directory(temp_dir.path().to_path_buf(), "test".to_string(), 0);
        node.load_children(&fs_service)
            .expect("Failed to load children");

        assert_eq!(node.children.len(), 1);
        assert_eq!(node.children[0].name, "visible.txt");
    }

    #[test]
    fn test_load_children_skips_target_directory() {
        let fs_service = FilesystemService::new();
        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        // Create target and regular directories
        fs::create_dir(temp_dir.path().join("target")).expect("Failed to create dir");
        fs::create_dir(temp_dir.path().join("src")).expect("Failed to create dir");

        let mut node =
            FileTreeNode::directory(temp_dir.path().to_path_buf(), "test".to_string(), 0);
        node.load_children(&fs_service)
            .expect("Failed to load children");

        assert_eq!(node.children.len(), 1);
        assert_eq!(node.children[0].name, "src");
    }

    #[test]
    fn test_load_children_skips_node_modules() {
        let fs_service = FilesystemService::new();
        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        // Create node_modules and regular directory
        fs::create_dir(temp_dir.path().join("node_modules")).expect("Failed to create dir");
        fs::create_dir(temp_dir.path().join("lib")).expect("Failed to create dir");

        let mut node =
            FileTreeNode::directory(temp_dir.path().to_path_buf(), "test".to_string(), 0);
        node.load_children(&fs_service)
            .expect("Failed to load children");

        assert_eq!(node.children.len(), 1);
        assert_eq!(node.children[0].name, "lib");
    }

    #[test]
    fn test_file_tree_with_title() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let tree = FileTree::new(temp_dir.path().to_path_buf())
            .expect("Failed to create tree")
            .with_title("Custom Title");

        assert_eq!(tree.title, "Custom Title");
    }

    #[test]
    fn test_file_tree_default_title() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let tree = FileTree::new(temp_dir.path().to_path_buf()).expect("Failed to create tree");

        assert_eq!(tree.title, "File Tree");
    }
}
