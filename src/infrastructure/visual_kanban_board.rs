//! Visual Kanban Board module for flexible column-based workflow
//!
//! This module provides a comprehensive Kanban board system inspired by Trello and Jira.
//! It supports customizable columns, WIP limits, swimlanes, and drag & drop functionality.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Kanban column
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KanbanColumn {
    /// Column ID
    pub id: String,
    /// Column name
    pub name: String,
    /// Column position/order
    pub position: usize,
    /// WIP (Work In Progress) limit (None = unlimited)
    pub wip_limit: Option<usize>,
    /// Card IDs in this column (ordered)
    pub card_ids: Vec<String>,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
    /// Color code for the column
    pub color: Option<String>,
}

impl KanbanColumn {
    /// Creates a new column
    pub fn new(id: String, name: String, position: usize) -> Self {
        Self {
            id,
            name,
            position,
            wip_limit: None,
            card_ids: Vec::new(),
            created_at: Utc::now(),
            color: None,
        }
    }

    /// Checks if the column is over its WIP limit
    pub fn is_over_wip_limit(&self) -> bool {
        if let Some(limit) = self.wip_limit {
            self.card_ids.len() > limit
        } else {
            false
        }
    }

    /// Gets the current card count
    pub fn card_count(&self) -> usize {
        self.card_ids.len()
    }

    /// Checks if adding a card would violate WIP limit
    pub fn would_violate_wip_limit(&self) -> bool {
        if let Some(limit) = self.wip_limit {
            self.card_ids.len() >= limit
        } else {
            false
        }
    }
}

/// Swimlane grouping type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SwimlaneGrouping {
    /// Group by priority
    Priority,
    /// Group by assignee
    Assignee,
    /// Group by project
    Project,
    /// Group by epic
    Epic,
    /// Group by tag
    Tag,
}

/// Swimlane
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Swimlane {
    /// Swimlane ID
    pub id: String,
    /// Swimlane name
    pub name: String,
    /// Grouping type
    pub grouping: SwimlaneGrouping,
    /// Grouping value (e.g., "P0" for Priority::Critical)
    pub value: String,
    /// Position/order
    pub position: usize,
    /// Whether this swimlane is collapsed
    pub collapsed: bool,
}

impl Swimlane {
    /// Creates a new swimlane
    pub fn new(id: String, name: String, grouping: SwimlaneGrouping, value: String, position: usize) -> Self {
        Self {
            id,
            name,
            grouping,
            value,
            position,
            collapsed: false,
        }
    }
}

/// Card position on the board
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardPosition {
    /// Card ID
    pub card_id: String,
    /// Column ID
    pub column_id: String,
    /// Position within column (0-based)
    pub position: usize,
    /// Swimlane ID (if swimlanes are enabled)
    pub swimlane_id: Option<String>,
}

/// Kanban board
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KanbanBoard {
    /// Board ID
    pub id: String,
    /// Board name
    pub name: String,
    /// Board description
    pub description: String,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
    /// Last updated timestamp
    pub updated_at: DateTime<Utc>,
    /// Board owner user ID
    pub owner_id: String,
    /// Whether swimlanes are enabled
    pub swimlanes_enabled: bool,
    /// Swimlane grouping type (if enabled)
    pub swimlane_grouping: Option<SwimlaneGrouping>,
}

impl KanbanBoard {
    /// Creates a new Kanban board
    pub fn new(id: String, name: String, description: String, owner_id: String) -> Self {
        let now = Utc::now();
        Self {
            id,
            name,
            description,
            created_at: now,
            updated_at: now,
            owner_id,
            swimlanes_enabled: false,
            swimlane_grouping: None,
        }
    }

    /// Updates the board
    pub fn touch(&mut self) {
        self.updated_at = Utc::now();
    }
}

/// Board manager
#[derive(Debug)]
pub struct BoardManager {
    boards: HashMap<String, KanbanBoard>,
    columns: HashMap<String, KanbanColumn>,      // column_id -> column
    board_columns: HashMap<String, Vec<String>>, // board_id -> column_ids (ordered)
    swimlanes: HashMap<String, Swimlane>,        // swimlane_id -> swimlane
    board_swimlanes: HashMap<String, Vec<String>>, // board_id -> swimlane_ids (ordered)
    card_positions: HashMap<String, CardPosition>, // card_id -> position
    next_board_id: usize,
    next_column_id: usize,
    next_swimlane_id: usize,
}

impl BoardManager {
    /// Creates a new board manager
    pub fn new() -> Self {
        Self {
            boards: HashMap::new(),
            columns: HashMap::new(),
            board_columns: HashMap::new(),
            swimlanes: HashMap::new(),
            board_swimlanes: HashMap::new(),
            card_positions: HashMap::new(),
            next_board_id: 1,
            next_column_id: 1,
            next_swimlane_id: 1,
        }
    }

    /// Creates a new board
    pub fn create_board(&mut self, name: String, description: String, owner_id: String) -> String {
        let id = format!("board-{}", self.next_board_id);
        self.next_board_id += 1;

        let board = KanbanBoard::new(id.clone(), name, description, owner_id);
        self.boards.insert(id.clone(), board);
        self.board_columns.insert(id.clone(), Vec::new());
        self.board_swimlanes.insert(id.clone(), Vec::new());
        id
    }

    /// Gets a board by ID
    pub fn get_board(&self, board_id: &str) -> Option<&KanbanBoard> {
        self.boards.get(board_id)
    }

    /// Gets a mutable board by ID
    pub fn get_board_mut(&mut self, board_id: &str) -> Option<&mut KanbanBoard> {
        self.boards.get_mut(board_id)
    }

    /// Deletes a board
    pub fn delete_board(&mut self, board_id: &str) -> Option<KanbanBoard> {
        let board = self.boards.remove(board_id)?;

        // Remove all columns
        if let Some(column_ids) = self.board_columns.remove(board_id) {
            for column_id in column_ids {
                self.columns.remove(&column_id);
            }
        }

        // Remove all swimlanes
        if let Some(swimlane_ids) = self.board_swimlanes.remove(board_id) {
            for swimlane_id in swimlane_ids {
                self.swimlanes.remove(&swimlane_id);
            }
        }

        Some(board)
    }

    /// Creates a new column for a board
    pub fn create_column(&mut self, board_id: String, name: String) -> Option<String> {
        if !self.boards.contains_key(&board_id) {
            return None;
        }

        let id = format!("column-{}", self.next_column_id);
        self.next_column_id += 1;

        let position = self
            .board_columns
            .get(&board_id)
            .map(|cols| cols.len())
            .unwrap_or(0);

        let column = KanbanColumn::new(id.clone(), name, position);
        self.columns.insert(id.clone(), column);

        self.board_columns
            .entry(board_id.clone())
            .or_default()
            .push(id.clone());

        if let Some(board) = self.boards.get_mut(&board_id) {
            board.touch();
        }

        Some(id)
    }

    /// Gets a column by ID
    pub fn get_column(&self, column_id: &str) -> Option<&KanbanColumn> {
        self.columns.get(column_id)
    }

    /// Gets a mutable column by ID
    pub fn get_column_mut(&mut self, column_id: &str) -> Option<&mut KanbanColumn> {
        self.columns.get_mut(column_id)
    }

    /// Gets all columns for a board (ordered by position)
    pub fn columns_for_board(&self, board_id: &str) -> Vec<&KanbanColumn> {
        self.board_columns
            .get(board_id)
            .map(|column_ids| {
                let mut columns: Vec<&KanbanColumn> = column_ids
                    .iter()
                    .filter_map(|id| self.columns.get(id))
                    .collect();
                columns.sort_by_key(|c| c.position);
                columns
            })
            .unwrap_or_default()
    }

    /// Moves a card to a column
    pub fn move_card_to_column(
        &mut self,
        card_id: String,
        to_column_id: String,
        position: usize,
    ) -> Result<(), String> {
        // Verify column exists
        if !self.columns.contains_key(&to_column_id) {
            return Err(format!("Column {} not found", to_column_id));
        }

        // Remove card from old column if it exists
        if let Some(old_position) = self.card_positions.get(&card_id)
            && let Some(old_column) = self.columns.get_mut(&old_position.column_id) {
                old_column.card_ids.retain(|id| id != &card_id);
            }

        // Add card to new column
        if let Some(column) = self.columns.get_mut(&to_column_id) {
            let insert_pos = position.min(column.card_ids.len());
            column.card_ids.insert(insert_pos, card_id.clone());

            // Update card position
            self.card_positions.insert(
                card_id.clone(),
                CardPosition {
                    card_id,
                    column_id: to_column_id,
                    position: insert_pos,
                    swimlane_id: None,
                },
            );

            Ok(())
        } else {
            Err(format!("Column {} not found", to_column_id))
        }
    }

    /// Removes a card from the board
    pub fn remove_card(&mut self, card_id: &str) -> Option<CardPosition> {
        let position = self.card_positions.remove(card_id)?;

        if let Some(column) = self.columns.get_mut(&position.column_id) {
            column.card_ids.retain(|id| id != card_id);
        }

        Some(position)
    }

    /// Gets the position of a card
    pub fn get_card_position(&self, card_id: &str) -> Option<&CardPosition> {
        self.card_positions.get(card_id)
    }

    /// Creates a swimlane for a board
    pub fn create_swimlane(
        &mut self,
        board_id: String,
        name: String,
        grouping: SwimlaneGrouping,
        value: String,
    ) -> Option<String> {
        if !self.boards.contains_key(&board_id) {
            return None;
        }

        let id = format!("swimlane-{}", self.next_swimlane_id);
        self.next_swimlane_id += 1;

        let position = self
            .board_swimlanes
            .get(&board_id)
            .map(|lanes| lanes.len())
            .unwrap_or(0);

        let swimlane = Swimlane::new(id.clone(), name, grouping, value, position);
        self.swimlanes.insert(id.clone(), swimlane);

        self.board_swimlanes
            .entry(board_id.clone())
            .or_default()
            .push(id.clone());

        if let Some(board) = self.boards.get_mut(&board_id) {
            board.swimlanes_enabled = true;
            board.swimlane_grouping = Some(grouping);
            board.touch();
        }

        Some(id)
    }

    /// Gets swimlanes for a board (ordered by position)
    pub fn swimlanes_for_board(&self, board_id: &str) -> Vec<&Swimlane> {
        self.board_swimlanes
            .get(board_id)
            .map(|swimlane_ids| {
                let mut swimlanes: Vec<&Swimlane> = swimlane_ids
                    .iter()
                    .filter_map(|id| self.swimlanes.get(id))
                    .collect();
                swimlanes.sort_by_key(|s| s.position);
                swimlanes
            })
            .unwrap_or_default()
    }

    /// Sets WIP limit for a column
    pub fn set_wip_limit(&mut self, column_id: &str, limit: Option<usize>) -> bool {
        if let Some(column) = self.columns.get_mut(column_id) {
            column.wip_limit = limit;
            true
        } else {
            false
        }
    }

    /// Reorders columns for a board
    pub fn reorder_columns(&mut self, board_id: &str, new_order: Vec<String>) -> bool {
        if let Some(board_columns) = self.board_columns.get_mut(board_id) {
            // Verify all column IDs are valid
            if new_order.iter().all(|id| board_columns.contains(id))
                && new_order.len() == board_columns.len()
            {
                *board_columns = new_order.clone();

                // Update positions
                for (pos, column_id) in new_order.iter().enumerate() {
                    if let Some(column) = self.columns.get_mut(column_id) {
                        column.position = pos;
                    }
                }

                if let Some(board) = self.boards.get_mut(board_id) {
                    board.touch();
                }

                true
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Gets total board count
    pub fn total_boards(&self) -> usize {
        self.boards.len()
    }

    /// Gets total column count
    pub fn total_columns(&self) -> usize {
        self.columns.len()
    }

    /// Gets columns that are over WIP limit
    pub fn columns_over_wip_limit(&self) -> Vec<&KanbanColumn> {
        self.columns
            .values()
            .filter(|col| col.is_over_wip_limit())
            .collect()
    }
}

impl Default for BoardManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_board() {
        let mut manager = BoardManager::new();
        let board_id = manager.create_board(
            "My Board".to_string(),
            "Test board".to_string(),
            "user-1".to_string(),
        );

        assert_eq!(board_id, "board-1");
        let board = manager.get_board(&board_id).unwrap();
        assert_eq!(board.name, "My Board");
        assert_eq!(board.owner_id, "user-1");
    }

    #[test]
    fn test_create_column() {
        let mut manager = BoardManager::new();
        let board_id = manager.create_board(
            "Board".to_string(),
            "Desc".to_string(),
            "user-1".to_string(),
        );

        let column_id = manager
            .create_column(board_id.clone(), "Todo".to_string())
            .unwrap();

        assert_eq!(column_id, "column-1");
        let column = manager.get_column(&column_id).unwrap();
        assert_eq!(column.name, "Todo");
        assert_eq!(column.position, 0);
    }

    #[test]
    fn test_columns_for_board() {
        let mut manager = BoardManager::new();
        let board_id = manager.create_board(
            "Board".to_string(),
            "Desc".to_string(),
            "user-1".to_string(),
        );

        manager.create_column(board_id.clone(), "Todo".to_string());
        manager.create_column(board_id.clone(), "In Progress".to_string());
        manager.create_column(board_id.clone(), "Done".to_string());

        let columns = manager.columns_for_board(&board_id);
        assert_eq!(columns.len(), 3);
        assert_eq!(columns[0].name, "Todo");
        assert_eq!(columns[1].name, "In Progress");
        assert_eq!(columns[2].name, "Done");
    }

    #[test]
    fn test_move_card_to_column() {
        let mut manager = BoardManager::new();
        let board_id = manager.create_board(
            "Board".to_string(),
            "Desc".to_string(),
            "user-1".to_string(),
        );

        let col1 = manager
            .create_column(board_id.clone(), "Todo".to_string())
            .unwrap();
        let col2 = manager
            .create_column(board_id, "Done".to_string())
            .unwrap();

        manager
            .move_card_to_column("card-1".to_string(), col1.clone(), 0)
            .unwrap();

        let position = manager.get_card_position("card-1").unwrap();
        assert_eq!(position.column_id, col1);

        // Move to different column
        manager
            .move_card_to_column("card-1".to_string(), col2.clone(), 0)
            .unwrap();

        let position = manager.get_card_position("card-1").unwrap();
        assert_eq!(position.column_id, col2);

        // Old column should be empty
        let col1_ref = manager.get_column(&col1).unwrap();
        assert_eq!(col1_ref.card_ids.len(), 0);
    }

    #[test]
    fn test_remove_card() {
        let mut manager = BoardManager::new();
        let board_id = manager.create_board(
            "Board".to_string(),
            "Desc".to_string(),
            "user-1".to_string(),
        );

        let col_id = manager
            .create_column(board_id, "Todo".to_string())
            .unwrap();

        manager
            .move_card_to_column("card-1".to_string(), col_id.clone(), 0)
            .unwrap();

        let removed = manager.remove_card("card-1");
        assert!(removed.is_some());

        let position = manager.get_card_position("card-1");
        assert!(position.is_none());

        let column = manager.get_column(&col_id).unwrap();
        assert_eq!(column.card_ids.len(), 0);
    }

    #[test]
    fn test_wip_limit() {
        let mut manager = BoardManager::new();
        let board_id = manager.create_board(
            "Board".to_string(),
            "Desc".to_string(),
            "user-1".to_string(),
        );

        let col_id = manager
            .create_column(board_id, "In Progress".to_string())
            .unwrap();

        manager.set_wip_limit(&col_id, Some(2));

        let column = manager.get_column(&col_id).unwrap();
        assert_eq!(column.wip_limit, Some(2));
        assert!(!column.is_over_wip_limit());

        // Add cards
        manager
            .move_card_to_column("card-1".to_string(), col_id.clone(), 0)
            .unwrap();
        manager
            .move_card_to_column("card-2".to_string(), col_id.clone(), 1)
            .unwrap();

        let column = manager.get_column(&col_id).unwrap();
        assert!(!column.is_over_wip_limit());
        assert!(column.would_violate_wip_limit());

        // Add one more
        manager
            .move_card_to_column("card-3".to_string(), col_id.clone(), 2)
            .unwrap();

        let column = manager.get_column(&col_id).unwrap();
        assert!(column.is_over_wip_limit());
    }

    #[test]
    fn test_create_swimlane() {
        let mut manager = BoardManager::new();
        let board_id = manager.create_board(
            "Board".to_string(),
            "Desc".to_string(),
            "user-1".to_string(),
        );

        let swimlane_id = manager
            .create_swimlane(
                board_id.clone(),
                "High Priority".to_string(),
                SwimlaneGrouping::Priority,
                "High".to_string(),
            )
            .unwrap();

        assert_eq!(swimlane_id, "swimlane-1");

        let swimlanes = manager.swimlanes_for_board(&board_id);
        assert_eq!(swimlanes.len(), 1);
        assert_eq!(swimlanes[0].name, "High Priority");

        let board = manager.get_board(&board_id).unwrap();
        assert!(board.swimlanes_enabled);
        assert_eq!(board.swimlane_grouping, Some(SwimlaneGrouping::Priority));
    }

    #[test]
    fn test_reorder_columns() {
        let mut manager = BoardManager::new();
        let board_id = manager.create_board(
            "Board".to_string(),
            "Desc".to_string(),
            "user-1".to_string(),
        );

        let col1 = manager
            .create_column(board_id.clone(), "Todo".to_string())
            .unwrap();
        let col2 = manager
            .create_column(board_id.clone(), "In Progress".to_string())
            .unwrap();
        let col3 = manager
            .create_column(board_id.clone(), "Done".to_string())
            .unwrap();

        // Reorder: swap col1 and col3
        let success = manager.reorder_columns(&board_id, vec![col3.clone(), col2.clone(), col1.clone()]);
        assert!(success);

        let columns = manager.columns_for_board(&board_id);
        assert_eq!(columns[0].id, col3);
        assert_eq!(columns[1].id, col2);
        assert_eq!(columns[2].id, col1);
    }

    #[test]
    fn test_delete_board() {
        let mut manager = BoardManager::new();
        let board_id = manager.create_board(
            "Board".to_string(),
            "Desc".to_string(),
            "user-1".to_string(),
        );

        manager.create_column(board_id.clone(), "Todo".to_string());
        manager.create_column(board_id.clone(), "Done".to_string());

        assert_eq!(manager.total_boards(), 1);
        assert_eq!(manager.total_columns(), 2);

        let deleted = manager.delete_board(&board_id);
        assert!(deleted.is_some());

        assert_eq!(manager.total_boards(), 0);
        assert_eq!(manager.total_columns(), 0);
    }

    #[test]
    fn test_columns_over_wip_limit() {
        let mut manager = BoardManager::new();
        let board_id = manager.create_board(
            "Board".to_string(),
            "Desc".to_string(),
            "user-1".to_string(),
        );

        let col1 = manager
            .create_column(board_id.clone(), "Todo".to_string())
            .unwrap();
        let col2 = manager
            .create_column(board_id, "In Progress".to_string())
            .unwrap();

        manager.set_wip_limit(&col2, Some(1));

        manager
            .move_card_to_column("card-1".to_string(), col2.clone(), 0)
            .unwrap();
        manager
            .move_card_to_column("card-2".to_string(), col2, 1)
            .unwrap();

        let over_limit = manager.columns_over_wip_limit();
        assert_eq!(over_limit.len(), 1);
        assert_eq!(over_limit[0].name, "In Progress");
    }

    #[test]
    fn test_card_position_tracking() {
        let mut manager = BoardManager::new();
        let board_id = manager.create_board(
            "Board".to_string(),
            "Desc".to_string(),
            "user-1".to_string(),
        );

        let col_id = manager
            .create_column(board_id, "Todo".to_string())
            .unwrap();

        manager
            .move_card_to_column("card-1".to_string(), col_id.clone(), 0)
            .unwrap();
        manager
            .move_card_to_column("card-2".to_string(), col_id.clone(), 1)
            .unwrap();

        let pos1 = manager.get_card_position("card-1").unwrap();
        assert_eq!(pos1.position, 0);

        let pos2 = manager.get_card_position("card-2").unwrap();
        assert_eq!(pos2.position, 1);
    }

    #[test]
    fn test_column_card_count() {
        let column = KanbanColumn::new("col-1".to_string(), "Todo".to_string(), 0);
        assert_eq!(column.card_count(), 0);

        let mut column = column;
        column.card_ids.push("card-1".to_string());
        column.card_ids.push("card-2".to_string());
        assert_eq!(column.card_count(), 2);
    }

    #[test]
    fn test_board_touch() {
        let mut board = KanbanBoard::new(
            "board-1".to_string(),
            "Board".to_string(),
            "Desc".to_string(),
            "user-1".to_string(),
        );

        let original_time = board.updated_at;
        std::thread::sleep(std::time::Duration::from_millis(10));

        board.touch();
        assert!(board.updated_at > original_time);
    }

    #[test]
    fn test_swimlane_collapse() {
        let mut swimlane = Swimlane::new(
            "lane-1".to_string(),
            "High Priority".to_string(),
            SwimlaneGrouping::Priority,
            "High".to_string(),
            0,
        );

        assert!(!swimlane.collapsed);
        swimlane.collapsed = true;
        assert!(swimlane.collapsed);
    }

    #[test]
    fn test_invalid_column_for_move() {
        let mut manager = BoardManager::new();

        let result = manager.move_card_to_column(
            "card-1".to_string(),
            "nonexistent".to_string(),
            0,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_create_column_invalid_board() {
        let mut manager = BoardManager::new();

        let result = manager.create_column("nonexistent".to_string(), "Todo".to_string());

        assert!(result.is_none());
    }
}
