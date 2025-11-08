/// Visual selection for Vim-style text selection
///
/// Character-wise (v), line-wise (V), and block-wise (Ctrl+v) selection
///
/// # Examples
///
/// ```
/// use toad::visual_selection::{VisualSelection, SelectionMode, Position};
///
/// let mut selection = VisualSelection::new();
/// selection.start(SelectionMode::Character, Position::new(0, 0));
/// selection.update_end(Position::new(0, 5));
/// assert!(selection.is_active());
/// ```

use serde::{Deserialize, Serialize};
use std::cmp::{max, min};

/// Position in text (line, column)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Position {
    pub line: usize,
    pub col: usize,
}

impl Position {
    /// Create a new position
    pub fn new(line: usize, col: usize) -> Self {
        Self { line, col }
    }

    /// Check if this position is before another
    pub fn is_before(&self, other: &Position) -> bool {
        self.line < other.line || (self.line == other.line && self.col < other.col)
    }

    /// Check if this position is after another
    pub fn is_after(&self, other: &Position) -> bool {
        self.line > other.line || (self.line == other.line && self.col > other.col)
    }
}

/// Selection mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SelectionMode {
    /// Character-wise selection (v)
    Character,
    /// Line-wise selection (V)
    Line,
    /// Block-wise selection (Ctrl+v)
    Block,
}

impl SelectionMode {
    /// Get mode name
    pub fn name(&self) -> &'static str {
        match self {
            SelectionMode::Character => "VISUAL",
            SelectionMode::Line => "VISUAL LINE",
            SelectionMode::Block => "VISUAL BLOCK",
        }
    }

    /// Get short name
    pub fn short_name(&self) -> &'static str {
        match self {
            SelectionMode::Character => "v",
            SelectionMode::Line => "V",
            SelectionMode::Block => "^V",
        }
    }
}

/// Visual selection state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualSelection {
    /// Whether selection is active
    active: bool,
    /// Selection mode
    mode: SelectionMode,
    /// Start position (anchor)
    start: Option<Position>,
    /// End position (cursor)
    end: Option<Position>,
}

impl VisualSelection {
    /// Create a new visual selection
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::visual_selection::VisualSelection;
    ///
    /// let selection = VisualSelection::new();
    /// assert!(!selection.is_active());
    /// ```
    pub fn new() -> Self {
        Self {
            active: false,
            mode: SelectionMode::Character,
            start: None,
            end: None,
        }
    }

    /// Start a selection
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::visual_selection::{VisualSelection, SelectionMode, Position};
    ///
    /// let mut selection = VisualSelection::new();
    /// selection.start(SelectionMode::Character, Position::new(0, 0));
    /// assert!(selection.is_active());
    /// ```
    pub fn start(&mut self, mode: SelectionMode, pos: Position) {
        self.active = true;
        self.mode = mode;
        self.start = Some(pos);
        self.end = Some(pos);
    }

    /// Update the end position
    pub fn update_end(&mut self, pos: Position) {
        if self.active {
            self.end = Some(pos);
        }
    }

    /// End the selection and return the range
    pub fn end(&mut self) -> Option<SelectionRange> {
        if self.active {
            let range = self.get_range();
            self.active = false;
            self.start = None;
            self.end = None;
            range
        } else {
            None
        }
    }

    /// Cancel the selection
    pub fn cancel(&mut self) {
        self.active = false;
        self.start = None;
        self.end = None;
    }

    /// Check if selection is active
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Get the selection mode
    pub fn mode(&self) -> SelectionMode {
        self.mode
    }

    /// Change selection mode (toggle between modes)
    pub fn change_mode(&mut self, mode: SelectionMode) {
        if self.active {
            self.mode = mode;
        }
    }

    /// Get the current selection range
    pub fn get_range(&self) -> Option<SelectionRange> {
        if !self.active {
            return None;
        }

        let start = self.start?;
        let end = self.end?;

        Some(SelectionRange {
            mode: self.mode,
            start: min(start, end),
            end: max(start, end),
        })
    }

    /// Check if a position is selected
    pub fn is_selected(&self, pos: Position) -> bool {
        if let Some(range) = self.get_range() {
            range.contains(pos)
        } else {
            false
        }
    }

    /// Get start position
    pub fn start_pos(&self) -> Option<Position> {
        self.start
    }

    /// Get end position
    pub fn end_pos(&self) -> Option<Position> {
        self.end
    }
}

impl Default for VisualSelection {
    fn default() -> Self {
        Self::new()
    }
}

/// A selection range
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SelectionRange {
    pub mode: SelectionMode,
    pub start: Position,
    pub end: Position,
}

impl SelectionRange {
    /// Create a new selection range
    pub fn new(mode: SelectionMode, start: Position, end: Position) -> Self {
        Self {
            mode,
            start: min(start, end),
            end: max(start, end),
        }
    }

    /// Check if this range contains a position
    pub fn contains(&self, pos: Position) -> bool {
        match self.mode {
            SelectionMode::Character => {
                // Character-wise: from start to end inclusive
                pos >= self.start && pos <= self.end
            }
            SelectionMode::Line => {
                // Line-wise: entire lines
                pos.line >= self.start.line && pos.line <= self.end.line
            }
            SelectionMode::Block => {
                // Block-wise: rectangular region
                let min_col = min(self.start.col, self.end.col);
                let max_col = max(self.start.col, self.end.col);
                pos.line >= self.start.line
                    && pos.line <= self.end.line
                    && pos.col >= min_col
                    && pos.col <= max_col
            }
        }
    }

    /// Get the line range
    pub fn line_range(&self) -> (usize, usize) {
        (self.start.line, self.end.line)
    }

    /// Get the column range for a given line (for block mode)
    pub fn col_range_for_line(&self, line: usize) -> Option<(usize, usize)> {
        if line < self.start.line || line > self.end.line {
            return None;
        }

        match self.mode {
            SelectionMode::Character => {
                if line == self.start.line && line == self.end.line {
                    Some((self.start.col, self.end.col))
                } else if line == self.start.line {
                    Some((self.start.col, usize::MAX))
                } else if line == self.end.line {
                    Some((0, self.end.col))
                } else {
                    Some((0, usize::MAX))
                }
            }
            SelectionMode::Line => Some((0, usize::MAX)),
            SelectionMode::Block => {
                let min_col = min(self.start.col, self.end.col);
                let max_col = max(self.start.col, self.end.col);
                Some((min_col, max_col))
            }
        }
    }

    /// Get the number of lines in the selection
    pub fn line_count(&self) -> usize {
        self.end.line - self.start.line + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_creation() {
        let pos = Position::new(5, 10);
        assert_eq!(pos.line, 5);
        assert_eq!(pos.col, 10);
    }

    #[test]
    fn test_position_is_before() {
        let pos1 = Position::new(0, 0);
        let pos2 = Position::new(0, 5);
        let pos3 = Position::new(1, 0);

        assert!(pos1.is_before(&pos2));
        assert!(pos1.is_before(&pos3));
        assert!(pos2.is_before(&pos3));
        assert!(!pos2.is_before(&pos1));
    }

    #[test]
    fn test_position_is_after() {
        let pos1 = Position::new(0, 0);
        let pos2 = Position::new(0, 5);

        assert!(!pos1.is_after(&pos2));
        assert!(pos2.is_after(&pos1));
    }

    #[test]
    fn test_selection_mode_name() {
        assert_eq!(SelectionMode::Character.name(), "VISUAL");
        assert_eq!(SelectionMode::Line.name(), "VISUAL LINE");
        assert_eq!(SelectionMode::Block.name(), "VISUAL BLOCK");
    }

    #[test]
    fn test_selection_mode_short_name() {
        assert_eq!(SelectionMode::Character.short_name(), "v");
        assert_eq!(SelectionMode::Line.short_name(), "V");
        assert_eq!(SelectionMode::Block.short_name(), "^V");
    }

    #[test]
    fn test_visual_selection_new() {
        let selection = VisualSelection::new();
        assert!(!selection.is_active());
        assert_eq!(selection.mode(), SelectionMode::Character);
    }

    #[test]
    fn test_visual_selection_start() {
        let mut selection = VisualSelection::new();
        selection.start(SelectionMode::Character, Position::new(0, 0));

        assert!(selection.is_active());
        assert_eq!(selection.mode(), SelectionMode::Character);
        assert_eq!(selection.start_pos(), Some(Position::new(0, 0)));
        assert_eq!(selection.end_pos(), Some(Position::new(0, 0)));
    }

    #[test]
    fn test_visual_selection_update_end() {
        let mut selection = VisualSelection::new();
        selection.start(SelectionMode::Character, Position::new(0, 0));
        selection.update_end(Position::new(0, 5));

        assert_eq!(selection.end_pos(), Some(Position::new(0, 5)));
    }

    #[test]
    fn test_visual_selection_cancel() {
        let mut selection = VisualSelection::new();
        selection.start(SelectionMode::Character, Position::new(0, 0));
        assert!(selection.is_active());

        selection.cancel();
        assert!(!selection.is_active());
        assert_eq!(selection.start_pos(), None);
        assert_eq!(selection.end_pos(), None);
    }

    #[test]
    fn test_visual_selection_end() {
        let mut selection = VisualSelection::new();
        selection.start(SelectionMode::Character, Position::new(0, 0));
        selection.update_end(Position::new(0, 5));

        let range = selection.end();
        assert!(range.is_some());
        assert!(!selection.is_active());
    }

    #[test]
    fn test_visual_selection_change_mode() {
        let mut selection = VisualSelection::new();
        selection.start(SelectionMode::Character, Position::new(0, 0));
        assert_eq!(selection.mode(), SelectionMode::Character);

        selection.change_mode(SelectionMode::Line);
        assert_eq!(selection.mode(), SelectionMode::Line);
    }

    #[test]
    fn test_selection_range_character() {
        let range = SelectionRange::new(
            SelectionMode::Character,
            Position::new(0, 0),
            Position::new(0, 5),
        );

        assert!(range.contains(Position::new(0, 0)));
        assert!(range.contains(Position::new(0, 3)));
        assert!(range.contains(Position::new(0, 5)));
        assert!(!range.contains(Position::new(0, 6)));
        assert!(!range.contains(Position::new(1, 0)));
    }

    #[test]
    fn test_selection_range_line() {
        let range = SelectionRange::new(
            SelectionMode::Line,
            Position::new(0, 0),
            Position::new(2, 5),
        );

        assert!(range.contains(Position::new(0, 0)));
        assert!(range.contains(Position::new(0, 100)));
        assert!(range.contains(Position::new(1, 0)));
        assert!(range.contains(Position::new(2, 0)));
        assert!(!range.contains(Position::new(3, 0)));
    }

    #[test]
    fn test_selection_range_block() {
        let range = SelectionRange::new(
            SelectionMode::Block,
            Position::new(0, 2),
            Position::new(2, 5),
        );

        assert!(range.contains(Position::new(0, 2)));
        assert!(range.contains(Position::new(1, 3)));
        assert!(range.contains(Position::new(2, 5)));
        assert!(!range.contains(Position::new(0, 1)));
        assert!(!range.contains(Position::new(0, 6)));
        assert!(!range.contains(Position::new(3, 3)));
    }

    #[test]
    fn test_selection_range_line_range() {
        let range = SelectionRange::new(
            SelectionMode::Character,
            Position::new(1, 5),
            Position::new(3, 10),
        );

        assert_eq!(range.line_range(), (1, 3));
    }

    #[test]
    fn test_selection_range_col_range_character() {
        let range = SelectionRange::new(
            SelectionMode::Character,
            Position::new(0, 5),
            Position::new(2, 10),
        );

        assert_eq!(range.col_range_for_line(0), Some((5, usize::MAX)));
        assert_eq!(range.col_range_for_line(1), Some((0, usize::MAX)));
        assert_eq!(range.col_range_for_line(2), Some((0, 10)));
        assert_eq!(range.col_range_for_line(3), None);
    }

    #[test]
    fn test_selection_range_col_range_line() {
        let range = SelectionRange::new(
            SelectionMode::Line,
            Position::new(0, 5),
            Position::new(2, 10),
        );

        assert_eq!(range.col_range_for_line(0), Some((0, usize::MAX)));
        assert_eq!(range.col_range_for_line(1), Some((0, usize::MAX)));
        assert_eq!(range.col_range_for_line(2), Some((0, usize::MAX)));
    }

    #[test]
    fn test_selection_range_col_range_block() {
        let range = SelectionRange::new(
            SelectionMode::Block,
            Position::new(0, 5),
            Position::new(2, 10),
        );

        assert_eq!(range.col_range_for_line(0), Some((5, 10)));
        assert_eq!(range.col_range_for_line(1), Some((5, 10)));
        assert_eq!(range.col_range_for_line(2), Some((5, 10)));
        assert_eq!(range.col_range_for_line(3), None);
    }

    #[test]
    fn test_selection_range_line_count() {
        let range = SelectionRange::new(
            SelectionMode::Character,
            Position::new(1, 0),
            Position::new(3, 10),
        );

        assert_eq!(range.line_count(), 3);
    }

    #[test]
    fn test_is_selected() {
        let mut selection = VisualSelection::new();
        selection.start(SelectionMode::Character, Position::new(0, 0));
        selection.update_end(Position::new(0, 5));

        assert!(selection.is_selected(Position::new(0, 0)));
        assert!(selection.is_selected(Position::new(0, 3)));
        assert!(selection.is_selected(Position::new(0, 5)));
        assert!(!selection.is_selected(Position::new(0, 6)));
    }

    #[test]
    fn test_get_range() {
        let mut selection = VisualSelection::new();
        selection.start(SelectionMode::Line, Position::new(1, 5));
        selection.update_end(Position::new(3, 10));

        let range = selection.get_range().unwrap();
        assert_eq!(range.mode, SelectionMode::Line);
        assert_eq!(range.start, Position::new(1, 5));
        assert_eq!(range.end, Position::new(3, 10));
    }

    #[test]
    fn test_get_range_reversed() {
        let mut selection = VisualSelection::new();
        selection.start(SelectionMode::Character, Position::new(3, 10));
        selection.update_end(Position::new(1, 5));

        let range = selection.get_range().unwrap();
        // Should be normalized (start < end)
        assert_eq!(range.start, Position::new(1, 5));
        assert_eq!(range.end, Position::new(3, 10));
    }

    #[test]
    fn test_default() {
        let selection = VisualSelection::default();
        assert!(!selection.is_active());
    }

    #[test]
    fn test_single_line_selection() {
        let range = SelectionRange::new(
            SelectionMode::Character,
            Position::new(0, 5),
            Position::new(0, 10),
        );

        assert_eq!(range.line_count(), 1);
        assert_eq!(range.col_range_for_line(0), Some((5, 10)));
    }
}
