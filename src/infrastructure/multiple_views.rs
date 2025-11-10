//! Multiple Views module for flexible board visualization
//!
//! This module provides a comprehensive view management system inspired by Asana and Monday.com.
//! It supports 6 different view types: Kanban, List, Calendar, Timeline, Table, and Mind Map.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// View type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ViewType {
    /// Kanban view - Default card-based columns
    Kanban,
    /// List view - Compact task list with sorting/grouping
    List,
    /// Calendar view - Tasks plotted by due date
    Calendar,
    /// Timeline/Gantt view - Visual project timeline with dependencies
    Timeline,
    /// Table/Spreadsheet view - Rows and columns for bulk editing
    Table,
    /// Mind Map view - Hierarchical task relationship visualization
    MindMap,
}

impl ViewType {
    /// Returns all available view types
    pub fn all() -> Vec<ViewType> {
        vec![
            ViewType::Kanban,
            ViewType::List,
            ViewType::Calendar,
            ViewType::Timeline,
            ViewType::Table,
            ViewType::MindMap,
        ]
    }

    /// Returns the view name as a string
    pub fn name(&self) -> &'static str {
        match self {
            ViewType::Kanban => "Kanban",
            ViewType::List => "List",
            ViewType::Calendar => "Calendar",
            ViewType::Timeline => "Timeline",
            ViewType::Table => "Table",
            ViewType::MindMap => "Mind Map",
        }
    }

    /// Returns the keyboard shortcut (Ctrl+N)
    pub fn shortcut(&self) -> &'static str {
        match self {
            ViewType::Kanban => "Ctrl+1",
            ViewType::List => "Ctrl+2",
            ViewType::Calendar => "Ctrl+3",
            ViewType::Timeline => "Ctrl+4",
            ViewType::Table => "Ctrl+5",
            ViewType::MindMap => "Ctrl+6",
        }
    }

    /// Returns the view description
    pub fn description(&self) -> &'static str {
        match self {
            ViewType::Kanban => "Card-based columns with drag & drop",
            ViewType::List => "Compact task list with sorting and grouping",
            ViewType::Calendar => "Tasks plotted by due date with scheduling",
            ViewType::Timeline => "Visual project timeline with dependencies",
            ViewType::Table => "Spreadsheet view for bulk editing",
            ViewType::MindMap => "Hierarchical task relationship tree",
        }
    }
}

/// Sorting options for list view
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SortBy {
    /// Sort by creation date
    CreatedDate,
    /// Sort by due date
    DueDate,
    /// Sort by priority
    Priority,
    /// Sort by title
    Title,
    /// Sort by assignee
    Assignee,
    /// Sort by status
    Status,
    /// Sort by progress
    Progress,
}

/// Sort order
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SortOrder {
    /// Ascending order
    Ascending,
    /// Descending order
    Descending,
}

/// Grouping options for list view
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GroupBy {
    /// No grouping
    None,
    /// Group by status
    Status,
    /// Group by priority
    Priority,
    /// Group by assignee
    Assignee,
    /// Group by tags
    Tags,
    /// Group by due date
    DueDate,
}

/// Calendar display mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CalendarMode {
    /// Month view
    Month,
    /// Week view
    Week,
    /// Day view
    Day,
}

/// Timeline zoom level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimelineZoom {
    /// Show tasks by day
    Days,
    /// Show tasks by week
    Weeks,
    /// Show tasks by month
    Months,
    /// Show tasks by quarter
    Quarters,
}

/// View configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewConfig {
    /// View ID
    pub id: String,
    /// View type
    pub view_type: ViewType,
    /// View name (custom name set by user)
    pub name: String,
    /// Board/Project ID this view belongs to
    pub board_id: String,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
    /// Last accessed timestamp
    pub last_accessed: DateTime<Utc>,
    /// View-specific settings
    pub settings: ViewSettings,
    /// Whether this is the default view for the board
    pub is_default: bool,
}

impl ViewConfig {
    /// Creates a new view configuration
    pub fn new(id: String, view_type: ViewType, name: String, board_id: String) -> Self {
        let now = Utc::now();
        Self {
            id,
            view_type,
            name,
            board_id,
            created_at: now,
            last_accessed: now,
            settings: ViewSettings::default_for_type(view_type),
            is_default: false,
        }
    }

    /// Updates the last accessed timestamp
    pub fn touch(&mut self) {
        self.last_accessed = Utc::now();
    }
}

/// View-specific settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ViewSettings {
    /// Kanban view settings
    Kanban {
        /// Show card covers
        show_covers: bool,
        /// Show card counts per column
        show_counts: bool,
        /// WIP limits enabled
        wip_limits: bool,
    },
    /// List view settings
    List {
        /// Sort by field
        sort_by: SortBy,
        /// Sort order
        sort_order: SortOrder,
        /// Group by field
        group_by: GroupBy,
        /// Show completed tasks
        show_completed: bool,
    },
    /// Calendar view settings
    Calendar {
        /// Calendar display mode
        mode: CalendarMode,
        /// Show weekends
        show_weekends: bool,
        /// Allow drag to reschedule
        allow_reschedule: bool,
    },
    /// Timeline view settings
    Timeline {
        /// Zoom level
        zoom: TimelineZoom,
        /// Show dependencies
        show_dependencies: bool,
        /// Show critical path
        show_critical_path: bool,
    },
    /// Table view settings
    Table {
        /// Visible columns
        visible_columns: Vec<String>,
        /// Allow inline editing
        allow_inline_edit: bool,
        /// Show row numbers
        show_row_numbers: bool,
    },
    /// Mind Map view settings
    MindMap {
        /// Expand all nodes
        expand_all: bool,
        /// Show task details
        show_details: bool,
        /// Layout orientation
        orientation: MindMapOrientation,
    },
}

impl ViewSettings {
    /// Creates default settings for a view type
    pub fn default_for_type(view_type: ViewType) -> Self {
        match view_type {
            ViewType::Kanban => ViewSettings::Kanban {
                show_covers: true,
                show_counts: true,
                wip_limits: false,
            },
            ViewType::List => ViewSettings::List {
                sort_by: SortBy::CreatedDate,
                sort_order: SortOrder::Descending,
                group_by: GroupBy::None,
                show_completed: false,
            },
            ViewType::Calendar => ViewSettings::Calendar {
                mode: CalendarMode::Month,
                show_weekends: true,
                allow_reschedule: true,
            },
            ViewType::Timeline => ViewSettings::Timeline {
                zoom: TimelineZoom::Weeks,
                show_dependencies: true,
                show_critical_path: false,
            },
            ViewType::Table => ViewSettings::Table {
                visible_columns: vec![
                    "title".to_string(),
                    "status".to_string(),
                    "priority".to_string(),
                    "assignee".to_string(),
                    "due_date".to_string(),
                ],
                allow_inline_edit: true,
                show_row_numbers: true,
            },
            ViewType::MindMap => ViewSettings::MindMap {
                expand_all: false,
                show_details: true,
                orientation: MindMapOrientation::TopDown,
            },
        }
    }
}

/// Mind map orientation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MindMapOrientation {
    /// Top to bottom
    TopDown,
    /// Left to right
    LeftRight,
    /// Radial (center outward)
    Radial,
}

/// View manager
#[derive(Debug)]
pub struct ViewManager {
    views: HashMap<String, ViewConfig>,
    board_views: HashMap<String, Vec<String>>, // board_id -> view_ids
    active_views: HashMap<String, String>,     // board_id -> active_view_id
    next_view_id: usize,
}

impl ViewManager {
    /// Creates a new view manager
    pub fn new() -> Self {
        Self {
            views: HashMap::new(),
            board_views: HashMap::new(),
            active_views: HashMap::new(),
            next_view_id: 1,
        }
    }

    /// Creates a new view for a board
    pub fn create_view(&mut self, view_type: ViewType, name: String, board_id: String) -> String {
        let id = format!("view-{}", self.next_view_id);
        self.next_view_id += 1;

        let view = ViewConfig::new(id.clone(), view_type, name, board_id.clone());

        // Add to views
        self.views.insert(id.clone(), view);

        // Add to board_views
        self.board_views
            .entry(board_id.clone())
            .or_default()
            .push(id.clone());

        // Set as active if first view for board
        if self
            .board_views
            .get(&board_id)
            .map(|v| v.len())
            .unwrap_or(0)
            == 1
        {
            self.active_views.insert(board_id, id.clone());
        }

        id
    }

    /// Gets a view by ID
    pub fn get_view(&self, view_id: &str) -> Option<&ViewConfig> {
        self.views.get(view_id)
    }

    /// Gets a mutable view by ID
    pub fn get_view_mut(&mut self, view_id: &str) -> Option<&mut ViewConfig> {
        self.views.get_mut(view_id)
    }

    /// Gets all views for a board
    pub fn views_for_board(&self, board_id: &str) -> Vec<&ViewConfig> {
        self.board_views
            .get(board_id)
            .map(|view_ids| {
                view_ids
                    .iter()
                    .filter_map(|id| self.views.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Sets the active view for a board
    pub fn set_active_view(&mut self, board_id: String, view_id: String) -> bool {
        if let Some(view) = self.views.get_mut(&view_id)
            && view.board_id == board_id
        {
            view.touch();
            self.active_views.insert(board_id, view_id);
            return true;
        }
        false
    }

    /// Gets the active view for a board
    pub fn get_active_view(&self, board_id: &str) -> Option<&ViewConfig> {
        self.active_views
            .get(board_id)
            .and_then(|view_id| self.views.get(view_id))
    }

    /// Gets the active view ID for a board
    pub fn get_active_view_id(&self, board_id: &str) -> Option<&String> {
        self.active_views.get(board_id)
    }

    /// Switches to the next view for a board
    pub fn next_view(&mut self, board_id: &str) -> Option<String> {
        let view_ids = self.board_views.get(board_id)?;
        let current_id = self.active_views.get(board_id)?;

        let current_idx = view_ids.iter().position(|id| id == current_id)?;
        let next_idx = (current_idx + 1) % view_ids.len();
        let next_id = view_ids[next_idx].clone();

        self.set_active_view(board_id.to_string(), next_id.clone());
        Some(next_id)
    }

    /// Switches to the previous view for a board
    pub fn previous_view(&mut self, board_id: &str) -> Option<String> {
        let view_ids = self.board_views.get(board_id)?;
        let current_id = self.active_views.get(board_id)?;

        let current_idx = view_ids.iter().position(|id| id == current_id)?;
        let prev_idx = if current_idx == 0 {
            view_ids.len() - 1
        } else {
            current_idx - 1
        };
        let prev_id = view_ids[prev_idx].clone();

        self.set_active_view(board_id.to_string(), prev_id.clone());
        Some(prev_id)
    }

    /// Switches to a view by type (first view of that type for the board)
    pub fn switch_to_type(&mut self, board_id: &str, view_type: ViewType) -> Option<String> {
        let views = self.views_for_board(board_id);
        let view = views.into_iter().find(|v| v.view_type == view_type)?;
        let view_id = view.id.clone();

        self.set_active_view(board_id.to_string(), view_id.clone());
        Some(view_id)
    }

    /// Deletes a view
    pub fn delete_view(&mut self, view_id: &str) -> Option<ViewConfig> {
        let view = self.views.remove(view_id)?;

        // Remove from board_views
        if let Some(view_ids) = self.board_views.get_mut(&view.board_id) {
            view_ids.retain(|id| id != view_id);
        }

        // If this was the active view, switch to another
        if let Some(active_id) = self.active_views.get(&view.board_id)
            && active_id == view_id
            && let Some(view_ids) = self.board_views.get(&view.board_id)
        {
            if let Some(first_id) = view_ids.first() {
                self.active_views
                    .insert(view.board_id.clone(), first_id.clone());
            } else {
                self.active_views.remove(&view.board_id);
            }
        }

        Some(view)
    }

    /// Sets a view as the default for its board
    pub fn set_default_view(&mut self, view_id: &str) -> bool {
        if let Some(view) = self.views.get(view_id) {
            let board_id = view.board_id.clone();

            // Unset all other defaults for this board
            for view in self.views.values_mut() {
                if view.board_id == board_id {
                    view.is_default = view.id == view_id;
                }
            }

            true
        } else {
            false
        }
    }

    /// Gets the default view for a board
    pub fn get_default_view(&self, board_id: &str) -> Option<&ViewConfig> {
        self.views_for_board(board_id)
            .into_iter()
            .find(|v| v.is_default)
    }

    /// Gets total view count
    pub fn total_views(&self) -> usize {
        self.views.len()
    }

    /// Gets view count for a board
    pub fn view_count_for_board(&self, board_id: &str) -> usize {
        self.board_views.get(board_id).map(|v| v.len()).unwrap_or(0)
    }
}

impl Default for ViewManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_view_type_all() {
        let types = ViewType::all();
        assert_eq!(types.len(), 6);
        assert!(types.contains(&ViewType::Kanban));
        assert!(types.contains(&ViewType::MindMap));
    }

    #[test]
    fn test_view_type_name() {
        assert_eq!(ViewType::Kanban.name(), "Kanban");
        assert_eq!(ViewType::Timeline.name(), "Timeline");
    }

    #[test]
    fn test_view_type_shortcut() {
        assert_eq!(ViewType::Kanban.shortcut(), "Ctrl+1");
        assert_eq!(ViewType::List.shortcut(), "Ctrl+2");
        assert_eq!(ViewType::MindMap.shortcut(), "Ctrl+6");
    }

    #[test]
    fn test_create_view() {
        let mut manager = ViewManager::new();
        let view_id = manager.create_view(
            ViewType::Kanban,
            "My Kanban".to_string(),
            "board-1".to_string(),
        );

        assert_eq!(view_id, "view-1");
        let view = manager.get_view(&view_id).unwrap();
        assert_eq!(view.view_type, ViewType::Kanban);
        assert_eq!(view.name, "My Kanban");
        assert_eq!(view.board_id, "board-1");
    }

    #[test]
    fn test_views_for_board() {
        let mut manager = ViewManager::new();
        manager.create_view(
            ViewType::Kanban,
            "Kanban".to_string(),
            "board-1".to_string(),
        );
        manager.create_view(ViewType::List, "List".to_string(), "board-1".to_string());
        manager.create_view(
            ViewType::Calendar,
            "Calendar".to_string(),
            "board-2".to_string(),
        );

        let board1_views = manager.views_for_board("board-1");
        assert_eq!(board1_views.len(), 2);

        let board2_views = manager.views_for_board("board-2");
        assert_eq!(board2_views.len(), 1);
    }

    #[test]
    fn test_set_active_view() {
        let mut manager = ViewManager::new();
        let view_id = manager.create_view(
            ViewType::Kanban,
            "Kanban".to_string(),
            "board-1".to_string(),
        );

        // Should be automatically set as active (first view)
        assert_eq!(manager.get_active_view_id("board-1"), Some(&view_id));

        let view_id2 =
            manager.create_view(ViewType::List, "List".to_string(), "board-1".to_string());
        manager.set_active_view("board-1".to_string(), view_id2.clone());

        assert_eq!(manager.get_active_view_id("board-1"), Some(&view_id2));
    }

    #[test]
    fn test_get_active_view() {
        let mut manager = ViewManager::new();
        let view_id = manager.create_view(
            ViewType::Kanban,
            "Kanban".to_string(),
            "board-1".to_string(),
        );

        let active = manager.get_active_view("board-1").unwrap();
        assert_eq!(active.id, view_id);
        assert_eq!(active.view_type, ViewType::Kanban);
    }

    #[test]
    fn test_next_view() {
        let mut manager = ViewManager::new();
        let view1 = manager.create_view(
            ViewType::Kanban,
            "Kanban".to_string(),
            "board-1".to_string(),
        );
        let view2 = manager.create_view(ViewType::List, "List".to_string(), "board-1".to_string());
        let view3 = manager.create_view(
            ViewType::Calendar,
            "Calendar".to_string(),
            "board-1".to_string(),
        );

        assert_eq!(manager.get_active_view_id("board-1"), Some(&view1));

        let next = manager.next_view("board-1").unwrap();
        assert_eq!(next, view2);

        let next = manager.next_view("board-1").unwrap();
        assert_eq!(next, view3);

        let next = manager.next_view("board-1").unwrap();
        assert_eq!(next, view1); // Wrap around
    }

    #[test]
    fn test_previous_view() {
        let mut manager = ViewManager::new();
        let view1 = manager.create_view(
            ViewType::Kanban,
            "Kanban".to_string(),
            "board-1".to_string(),
        );
        let view2 = manager.create_view(ViewType::List, "List".to_string(), "board-1".to_string());
        let view3 = manager.create_view(
            ViewType::Calendar,
            "Calendar".to_string(),
            "board-1".to_string(),
        );

        assert_eq!(manager.get_active_view_id("board-1"), Some(&view1));

        let prev = manager.previous_view("board-1").unwrap();
        assert_eq!(prev, view3); // Wrap around

        let prev = manager.previous_view("board-1").unwrap();
        assert_eq!(prev, view2);
    }

    #[test]
    fn test_switch_to_type() {
        let mut manager = ViewManager::new();
        manager.create_view(
            ViewType::Kanban,
            "Kanban".to_string(),
            "board-1".to_string(),
        );
        let list_id =
            manager.create_view(ViewType::List, "List".to_string(), "board-1".to_string());

        let switched = manager.switch_to_type("board-1", ViewType::List).unwrap();
        assert_eq!(switched, list_id);
        assert_eq!(manager.get_active_view_id("board-1"), Some(&list_id));
    }

    #[test]
    fn test_delete_view() {
        let mut manager = ViewManager::new();
        let view1 = manager.create_view(
            ViewType::Kanban,
            "Kanban".to_string(),
            "board-1".to_string(),
        );
        let view2 = manager.create_view(ViewType::List, "List".to_string(), "board-1".to_string());

        assert_eq!(manager.total_views(), 2);

        let deleted = manager.delete_view(&view1);
        assert!(deleted.is_some());
        assert_eq!(manager.total_views(), 1);

        // Active view should switch to remaining view
        assert_eq!(manager.get_active_view_id("board-1"), Some(&view2));
    }

    #[test]
    fn test_set_default_view() {
        let mut manager = ViewManager::new();
        let view1 = manager.create_view(
            ViewType::Kanban,
            "Kanban".to_string(),
            "board-1".to_string(),
        );
        let view2 = manager.create_view(ViewType::List, "List".to_string(), "board-1".to_string());

        manager.set_default_view(&view2);

        let default = manager.get_default_view("board-1").unwrap();
        assert_eq!(default.id, view2);

        // view1 should not be default
        let view1_ref = manager.get_view(&view1).unwrap();
        assert!(!view1_ref.is_default);
    }

    #[test]
    fn test_view_settings_default() {
        let kanban_settings = ViewSettings::default_for_type(ViewType::Kanban);
        match kanban_settings {
            ViewSettings::Kanban { show_covers, .. } => assert!(show_covers),
            _ => panic!("Expected Kanban settings"),
        }

        let list_settings = ViewSettings::default_for_type(ViewType::List);
        match list_settings {
            ViewSettings::List {
                sort_by,
                sort_order,
                ..
            } => {
                assert_eq!(sort_by, SortBy::CreatedDate);
                assert_eq!(sort_order, SortOrder::Descending);
            }
            _ => panic!("Expected List settings"),
        }
    }

    #[test]
    fn test_view_touch() {
        let mut manager = ViewManager::new();
        let view_id = manager.create_view(
            ViewType::Kanban,
            "Kanban".to_string(),
            "board-1".to_string(),
        );

        let view = manager.get_view(&view_id).unwrap();
        let initial_time = view.last_accessed;

        std::thread::sleep(std::time::Duration::from_millis(10));

        let view = manager.get_view_mut(&view_id).unwrap();
        view.touch();

        assert!(view.last_accessed > initial_time);
    }

    #[test]
    fn test_view_count_for_board() {
        let mut manager = ViewManager::new();
        manager.create_view(
            ViewType::Kanban,
            "Kanban".to_string(),
            "board-1".to_string(),
        );
        manager.create_view(ViewType::List, "List".to_string(), "board-1".to_string());
        manager.create_view(
            ViewType::Calendar,
            "Calendar".to_string(),
            "board-2".to_string(),
        );

        assert_eq!(manager.view_count_for_board("board-1"), 2);
        assert_eq!(manager.view_count_for_board("board-2"), 1);
        assert_eq!(manager.view_count_for_board("board-3"), 0);
    }

    #[test]
    fn test_delete_last_view_clears_active() {
        let mut manager = ViewManager::new();
        let view_id = manager.create_view(
            ViewType::Kanban,
            "Kanban".to_string(),
            "board-1".to_string(),
        );

        assert_eq!(manager.get_active_view_id("board-1"), Some(&view_id));

        manager.delete_view(&view_id);

        assert_eq!(manager.get_active_view_id("board-1"), None);
    }

    #[test]
    fn test_first_view_auto_active() {
        let mut manager = ViewManager::new();
        let view_id = manager.create_view(
            ViewType::Kanban,
            "Kanban".to_string(),
            "board-1".to_string(),
        );

        // First view should automatically be set as active
        assert_eq!(manager.get_active_view_id("board-1"), Some(&view_id));
    }

    #[test]
    fn test_multiple_boards_independent() {
        let mut manager = ViewManager::new();
        let board1_view = manager.create_view(
            ViewType::Kanban,
            "Kanban".to_string(),
            "board-1".to_string(),
        );
        let board2_view =
            manager.create_view(ViewType::List, "List".to_string(), "board-2".to_string());

        assert_eq!(manager.get_active_view_id("board-1"), Some(&board1_view));
        assert_eq!(manager.get_active_view_id("board-2"), Some(&board2_view));
    }
}
