/// Multi-cursor support for editing multiple locations simultaneously
///
/// Allows editing at multiple positions in a text buffer at the same time
///
/// # Examples
///
/// ```
/// use toad::multicursor::MultiCursor;
///
/// let mut mc = MultiCursor::new();
/// mc.add_cursor(0, 0);
/// mc.add_cursor(1, 0);
/// assert_eq!(mc.cursor_count(), 2);
/// ```

use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

/// Position in a text buffer (line, column)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct CursorPosition {
    /// Line number (0-indexed)
    pub line: usize,
    /// Column number (0-indexed)
    pub col: usize,
}

impl CursorPosition {
    /// Create a new cursor position
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::multicursor::CursorPosition;
    ///
    /// let pos = CursorPosition::new(0, 0);
    /// assert_eq!(pos.line, 0);
    /// assert_eq!(pos.col, 0);
    /// ```
    pub fn new(line: usize, col: usize) -> Self {
        Self { line, col }
    }

    /// Move cursor right by n columns
    pub fn move_right(&mut self, n: usize) {
        self.col += n;
    }

    /// Move cursor left by n columns (clamped at 0)
    pub fn move_left(&mut self, n: usize) {
        self.col = self.col.saturating_sub(n);
    }

    /// Move cursor down by n lines
    pub fn move_down(&mut self, n: usize) {
        self.line += n;
    }

    /// Move cursor up by n lines (clamped at 0)
    pub fn move_up(&mut self, n: usize) {
        self.line = self.line.saturating_sub(n);
    }
}

/// Multi-cursor manager
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiCursor {
    /// All cursor positions (BTreeSet ensures sorted, unique positions)
    cursors: BTreeSet<CursorPosition>,
    /// Primary cursor (last added or main cursor)
    primary: Option<CursorPosition>,
}

impl MultiCursor {
    /// Create a new multi-cursor manager
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::multicursor::MultiCursor;
    ///
    /// let mc = MultiCursor::new();
    /// assert_eq!(mc.cursor_count(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            cursors: BTreeSet::new(),
            primary: None,
        }
    }

    /// Create with a single cursor at position
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::multicursor::MultiCursor;
    ///
    /// let mc = MultiCursor::with_cursor(0, 0);
    /// assert_eq!(mc.cursor_count(), 1);
    /// ```
    pub fn with_cursor(line: usize, col: usize) -> Self {
        let mut mc = Self::new();
        mc.add_cursor(line, col);
        mc
    }

    /// Add a cursor at position
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::multicursor::MultiCursor;
    ///
    /// let mut mc = MultiCursor::new();
    /// mc.add_cursor(0, 0);
    /// mc.add_cursor(1, 5);
    /// assert_eq!(mc.cursor_count(), 2);
    /// ```
    pub fn add_cursor(&mut self, line: usize, col: usize) -> bool {
        let pos = CursorPosition::new(line, col);
        let inserted = self.cursors.insert(pos);
        if inserted {
            self.primary = Some(pos);
        }
        inserted
    }

    /// Remove cursor at position
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::multicursor::MultiCursor;
    ///
    /// let mut mc = MultiCursor::with_cursor(0, 0);
    /// assert_eq!(mc.cursor_count(), 1);
    ///
    /// mc.remove_cursor(0, 0);
    /// assert_eq!(mc.cursor_count(), 0);
    /// ```
    pub fn remove_cursor(&mut self, line: usize, col: usize) -> bool {
        let pos = CursorPosition::new(line, col);
        let removed = self.cursors.remove(&pos);

        // Update primary if we removed it
        if Some(pos) == self.primary {
            self.primary = self.cursors.iter().next().copied();
        }

        removed
    }

    /// Get all cursor positions
    pub fn positions(&self) -> Vec<CursorPosition> {
        self.cursors.iter().copied().collect()
    }

    /// Get cursor count
    pub fn cursor_count(&self) -> usize {
        self.cursors.len()
    }

    /// Check if there are any cursors
    pub fn is_empty(&self) -> bool {
        self.cursors.is_empty()
    }

    /// Clear all cursors
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::multicursor::MultiCursor;
    ///
    /// let mut mc = MultiCursor::with_cursor(0, 0);
    /// mc.add_cursor(1, 0);
    /// assert_eq!(mc.cursor_count(), 2);
    ///
    /// mc.clear();
    /// assert_eq!(mc.cursor_count(), 0);
    /// ```
    pub fn clear(&mut self) {
        self.cursors.clear();
        self.primary = None;
    }

    /// Get primary cursor position
    pub fn primary_position(&self) -> Option<CursorPosition> {
        self.primary
    }

    /// Set primary cursor
    pub fn set_primary(&mut self, line: usize, col: usize) -> bool {
        let pos = CursorPosition::new(line, col);
        if self.cursors.contains(&pos) {
            self.primary = Some(pos);
            true
        } else {
            false
        }
    }

    /// Move all cursors right by n columns
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::multicursor::MultiCursor;
    ///
    /// let mut mc = MultiCursor::with_cursor(0, 0);
    /// mc.move_all_right(5);
    ///
    /// let positions = mc.positions();
    /// assert_eq!(positions[0].col, 5);
    /// ```
    pub fn move_all_right(&mut self, n: usize) {
        let new_cursors: BTreeSet<_> = self
            .cursors
            .iter()
            .map(|&pos| {
                let mut new_pos = pos;
                new_pos.move_right(n);
                new_pos
            })
            .collect();

        self.cursors = new_cursors;
        if let Some(primary) = self.primary {
            let mut new_primary = primary;
            new_primary.move_right(n);
            self.primary = Some(new_primary);
        }
    }

    /// Move all cursors left by n columns
    pub fn move_all_left(&mut self, n: usize) {
        let new_cursors: BTreeSet<_> = self
            .cursors
            .iter()
            .map(|&pos| {
                let mut new_pos = pos;
                new_pos.move_left(n);
                new_pos
            })
            .collect();

        self.cursors = new_cursors;
        if let Some(primary) = self.primary {
            let mut new_primary = primary;
            new_primary.move_left(n);
            self.primary = Some(new_primary);
        }
    }

    /// Move all cursors down by n lines
    pub fn move_all_down(&mut self, n: usize) {
        let new_cursors: BTreeSet<_> = self
            .cursors
            .iter()
            .map(|&pos| {
                let mut new_pos = pos;
                new_pos.move_down(n);
                new_pos
            })
            .collect();

        self.cursors = new_cursors;
        if let Some(primary) = self.primary {
            let mut new_primary = primary;
            new_primary.move_down(n);
            self.primary = Some(new_primary);
        }
    }

    /// Move all cursors up by n lines
    pub fn move_all_up(&mut self, n: usize) {
        let new_cursors: BTreeSet<_> = self
            .cursors
            .iter()
            .map(|&pos| {
                let mut new_pos = pos;
                new_pos.move_up(n);
                new_pos
            })
            .collect();

        self.cursors = new_cursors;
        if let Some(primary) = self.primary {
            let mut new_primary = primary;
            new_primary.move_up(n);
            self.primary = Some(new_primary);
        }
    }

    /// Check if position has a cursor
    pub fn has_cursor_at(&self, line: usize, col: usize) -> bool {
        self.cursors.contains(&CursorPosition::new(line, col))
    }

    /// Merge overlapping cursors (already handled by BTreeSet uniqueness)
    pub fn merge_overlapping(&mut self) {
        // BTreeSet already ensures uniqueness, so this is a no-op
        // Kept for API completeness
    }
}

impl Default for MultiCursor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cursor_position_creation() {
        let pos = CursorPosition::new(5, 10);
        assert_eq!(pos.line, 5);
        assert_eq!(pos.col, 10);
    }

    #[test]
    fn test_cursor_position_movement() {
        let mut pos = CursorPosition::new(5, 10);

        pos.move_right(3);
        assert_eq!(pos.col, 13);

        pos.move_left(5);
        assert_eq!(pos.col, 8);

        pos.move_down(2);
        assert_eq!(pos.line, 7);

        pos.move_up(3);
        assert_eq!(pos.line, 4);
    }

    #[test]
    fn test_cursor_position_clamping() {
        let mut pos = CursorPosition::new(0, 0);

        pos.move_left(10);
        assert_eq!(pos.col, 0); // Clamped

        pos.move_up(10);
        assert_eq!(pos.line, 0); // Clamped
    }

    #[test]
    fn test_multicursor_creation() {
        let mc = MultiCursor::new();
        assert_eq!(mc.cursor_count(), 0);
        assert!(mc.is_empty());
    }

    #[test]
    fn test_multicursor_with_cursor() {
        let mc = MultiCursor::with_cursor(0, 0);
        assert_eq!(mc.cursor_count(), 1);
        assert!(!mc.is_empty());
    }

    #[test]
    fn test_multicursor_add_remove() {
        let mut mc = MultiCursor::new();

        assert!(mc.add_cursor(0, 0));
        assert!(mc.add_cursor(1, 5));
        assert_eq!(mc.cursor_count(), 2);

        // Adding duplicate returns false
        assert!(!mc.add_cursor(0, 0));
        assert_eq!(mc.cursor_count(), 2);

        assert!(mc.remove_cursor(0, 0));
        assert_eq!(mc.cursor_count(), 1);

        assert!(!mc.remove_cursor(5, 5));
        assert_eq!(mc.cursor_count(), 1);
    }

    #[test]
    fn test_multicursor_clear() {
        let mut mc = MultiCursor::new();
        mc.add_cursor(0, 0);
        mc.add_cursor(1, 0);

        assert_eq!(mc.cursor_count(), 2);

        mc.clear();
        assert_eq!(mc.cursor_count(), 0);
        assert!(mc.is_empty());
    }

    #[test]
    fn test_multicursor_positions() {
        let mut mc = MultiCursor::new();
        mc.add_cursor(0, 0);
        mc.add_cursor(1, 5);
        mc.add_cursor(2, 10);

        let positions = mc.positions();
        assert_eq!(positions.len(), 3);
        assert_eq!(positions[0], CursorPosition::new(0, 0));
        assert_eq!(positions[1], CursorPosition::new(1, 5));
        assert_eq!(positions[2], CursorPosition::new(2, 10));
    }

    #[test]
    fn test_multicursor_primary() {
        let mut mc = MultiCursor::new();
        mc.add_cursor(0, 0);
        mc.add_cursor(1, 5);

        // Last added is primary
        assert_eq!(mc.primary_position(), Some(CursorPosition::new(1, 5)));

        mc.set_primary(0, 0);
        assert_eq!(mc.primary_position(), Some(CursorPosition::new(0, 0)));
    }

    #[test]
    fn test_multicursor_move_all() {
        let mut mc = MultiCursor::new();
        mc.add_cursor(0, 0);
        mc.add_cursor(1, 0);

        mc.move_all_right(5);
        let positions = mc.positions();
        assert_eq!(positions[0].col, 5);
        assert_eq!(positions[1].col, 5);

        mc.move_all_down(2);
        let positions = mc.positions();
        assert_eq!(positions[0].line, 2);
        assert_eq!(positions[1].line, 3);
    }

    #[test]
    fn test_multicursor_has_cursor_at() {
        let mut mc = MultiCursor::new();
        mc.add_cursor(0, 0);
        mc.add_cursor(1, 5);

        assert!(mc.has_cursor_at(0, 0));
        assert!(mc.has_cursor_at(1, 5));
        assert!(!mc.has_cursor_at(2, 10));
    }

    #[test]
    fn test_multicursor_sorted_positions() {
        let mut mc = MultiCursor::new();
        mc.add_cursor(2, 10);
        mc.add_cursor(0, 0);
        mc.add_cursor(1, 5);

        let positions = mc.positions();
        // Should be sorted by line, then column
        assert_eq!(positions[0], CursorPosition::new(0, 0));
        assert_eq!(positions[1], CursorPosition::new(1, 5));
        assert_eq!(positions[2], CursorPosition::new(2, 10));
    }
}
