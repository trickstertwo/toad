//! Table widget
//!
//! Multi-column table with headers, sorting, and selection

use crate::ui::theme::ToadTheme;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    widgets::{
        Block, Borders, Row, Scrollbar, ScrollbarOrientation, ScrollbarState,
        Table as RatatuiTable, TableState,
    },
};

/// Column alignment
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColumnAlignment {
    Left,
    Center,
    Right,
}

/// Table column definition
#[derive(Debug, Clone)]
pub struct TableColumn {
    pub header: String,
    pub width: u16,
    pub alignment: ColumnAlignment,
}

impl TableColumn {
    /// Create a new column
    pub fn new(header: impl Into<String>, width: u16) -> Self {
        Self {
            header: header.into(),
            width,
            alignment: ColumnAlignment::Left,
        }
    }

    /// Set alignment
    pub fn with_alignment(mut self, alignment: ColumnAlignment) -> Self {
        self.alignment = alignment;
        self
    }
}

/// Table widget with multi-column support
pub struct DataTable {
    title: String,
    columns: Vec<TableColumn>,
    rows: Vec<Vec<String>>,
    state: TableState,
    show_header: bool,
}

impl DataTable {
    /// Create a new table
    pub fn new(title: impl Into<String>, columns: Vec<TableColumn>) -> Self {
        let mut state = TableState::default();
        state.select(Some(0));

        Self {
            title: title.into(),
            columns,
            rows: Vec::new(),
            state,
            show_header: true,
        }
    }

    /// Add a row to the table
    pub fn add_row(&mut self, row: Vec<String>) {
        self.rows.push(row);
    }

    /// Set all rows at once
    pub fn set_rows(&mut self, rows: Vec<Vec<String>>) {
        self.rows = rows;
        if !self.rows.is_empty() && self.state.selected().is_none() {
            self.state.select(Some(0));
        }
    }

    /// Get the currently selected row index
    pub fn selected(&self) -> Option<usize> {
        self.state.selected()
    }

    /// Get the currently selected row data
    pub fn selected_row(&self) -> Option<&Vec<String>> {
        self.state.selected().and_then(|i| self.rows.get(i))
    }

    /// Select the next row
    pub fn select_next(&mut self) {
        if self.rows.is_empty() {
            return;
        }

        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.rows.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    /// Select the previous row
    pub fn select_previous(&mut self) {
        if self.rows.is_empty() {
            return;
        }

        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.rows.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    /// Select first row
    pub fn select_first(&mut self) {
        if !self.rows.is_empty() {
            self.state.select(Some(0));
        }
    }

    /// Select last row
    pub fn select_last(&mut self) {
        if !self.rows.is_empty() {
            self.state.select(Some(self.rows.len() - 1));
        }
    }

    /// Get row count
    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    /// Clear all rows
    pub fn clear(&mut self) {
        self.rows.clear();
        self.state.select(None);
    }

    /// Toggle header visibility
    pub fn set_show_header(&mut self, show: bool) {
        self.show_header = show;
    }

    /// Render the table
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

        // Build header
        let header_cells = self
            .columns
            .iter()
            .map(|col| col.header.as_str())
            .collect::<Vec<_>>();

        let header = Row::new(header_cells).style(
            Style::default()
                .fg(ToadTheme::TOAD_GREEN_BRIGHT)
                .add_modifier(Modifier::BOLD),
        );

        // Build rows
        let rows: Vec<Row> = self
            .rows
            .iter()
            .map(|row_data| {
                let cells = row_data
                    .iter()
                    .map(|cell| cell.as_str())
                    .collect::<Vec<_>>();
                Row::new(cells).style(Style::default().fg(ToadTheme::FOREGROUND))
            })
            .collect();

        // Build column widths
        let widths: Vec<ratatui::layout::Constraint> = self
            .columns
            .iter()
            .map(|col| ratatui::layout::Constraint::Length(col.width))
            .collect();

        let table = RatatuiTable::new(rows, widths)
            .header(header)
            .block(block)
            .row_highlight_style(
                Style::default()
                    .bg(ToadTheme::TOAD_GREEN_DARK)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("» ");

        frame.render_stateful_widget(table, area, &mut self.state);

        // Render scrollbar if there are rows
        if !self.rows.is_empty() {
            let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .style(Style::default().fg(ToadTheme::DARK_GRAY))
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓"));

            let mut scrollbar_state =
                ScrollbarState::new(self.rows.len()).position(self.state.selected().unwrap_or(0));

            frame.render_stateful_widget(scrollbar, inner, &mut scrollbar_state);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_creation() {
        let columns = vec![
            TableColumn::new("Name", 20),
            TableColumn::new("Age", 10),
            TableColumn::new("City", 30),
        ];

        let table = DataTable::new("Test Table", columns);
        assert_eq!(table.row_count(), 0);
        assert_eq!(table.selected(), Some(0));
    }

    #[test]
    fn test_table_add_rows() {
        let columns = vec![TableColumn::new("Name", 20), TableColumn::new("Age", 10)];

        let mut table = DataTable::new("Test", columns);

        table.add_row(vec!["Alice".to_string(), "30".to_string()]);
        table.add_row(vec!["Bob".to_string(), "25".to_string()]);

        assert_eq!(table.row_count(), 2);
        assert_eq!(
            table.selected_row(),
            Some(&vec!["Alice".to_string(), "30".to_string()])
        );
    }

    #[test]
    fn test_table_navigation() {
        let columns = vec![TableColumn::new("Data", 20)];
        let mut table = DataTable::new("Test", columns);

        table.set_rows(vec![
            vec!["Row 1".to_string()],
            vec!["Row 2".to_string()],
            vec!["Row 3".to_string()],
        ]);

        assert_eq!(table.selected(), Some(0));

        table.select_next();
        assert_eq!(table.selected(), Some(1));

        table.select_next();
        assert_eq!(table.selected(), Some(2));

        table.select_next(); // Should wrap to 0
        assert_eq!(table.selected(), Some(0));

        table.select_previous(); // Should wrap to 2
        assert_eq!(table.selected(), Some(2));
    }

    #[test]
    fn test_table_first_last() {
        let columns = vec![TableColumn::new("Data", 20)];
        let mut table = DataTable::new("Test", columns);

        table.set_rows(vec![
            vec!["Row 1".to_string()],
            vec!["Row 2".to_string()],
            vec!["Row 3".to_string()],
        ]);

        table.select_last();
        assert_eq!(table.selected(), Some(2));

        table.select_first();
        assert_eq!(table.selected(), Some(0));
    }

    #[test]
    fn test_table_clear() {
        let columns = vec![TableColumn::new("Data", 20)];
        let mut table = DataTable::new("Test", columns);

        table.set_rows(vec![vec!["Row 1".to_string()]]);
        assert_eq!(table.row_count(), 1);

        table.clear();
        assert_eq!(table.row_count(), 0);
        assert_eq!(table.selected(), None);
    }

    // ========================================================================
    // COMPREHENSIVE EDGE CASE TESTS (MEDIUM TIER)
    // ========================================================================

    #[test]
    fn test_table_empty_no_columns() {
        let table = DataTable::new("Empty Table", vec![]);
        assert_eq!(table.row_count(), 0);
        assert_eq!(table.selected(), Some(0)); // Default selection
    }

    #[test]
    fn test_table_single_column() {
        let columns = vec![TableColumn::new("Only Column", 50)];
        let mut table = DataTable::new("Single Column", columns);

        table.add_row(vec!["Value 1".to_string()]);
        table.add_row(vec!["Value 2".to_string()]);

        assert_eq!(table.row_count(), 2);
        assert_eq!(table.selected_row(), Some(&vec!["Value 1".to_string()]));
    }

    #[test]
    fn test_table_single_row() {
        let columns = vec![
            TableColumn::new("Col1", 10),
            TableColumn::new("Col2", 10),
            TableColumn::new("Col3", 10),
        ];
        let mut table = DataTable::new("Single Row", columns);

        table.add_row(vec!["A".to_string(), "B".to_string(), "C".to_string()]);

        assert_eq!(table.row_count(), 1);

        // Wrap-around with single row
        table.select_next();
        assert_eq!(table.selected(), Some(0), "Should wrap to first row");

        table.select_previous();
        assert_eq!(table.selected(), Some(0), "Should stay on only row");
    }

    #[test]
    fn test_table_very_large_dataset() {
        let columns = vec![TableColumn::new("ID", 10), TableColumn::new("Data", 50)];
        let mut table = DataTable::new("Large Table", columns);

        // Add 10,000 rows
        for i in 0..10_000 {
            table.add_row(vec![format!("{}", i), format!("Data row {}", i)]);
        }

        assert_eq!(table.row_count(), 10_000);

        // Navigate to last row
        table.select_last();
        assert_eq!(table.selected(), Some(9_999));

        // Navigate to first row
        table.select_first();
        assert_eq!(table.selected(), Some(0));
    }

    #[test]
    fn test_table_with_unicode_cells() {
        let columns = vec![
            TableColumn::new("🐸 Emoji", 20),
            TableColumn::new("日本語", 20),
        ];
        let mut table = DataTable::new("Unicode Table", columns);

        table.add_row(vec!["🎉 Party".to_string(), "こんにちは".to_string()]);
        table.add_row(vec!["👨‍💻 Dev".to_string(), "標題".to_string()]);
        table.add_row(vec!["🐸 Frog".to_string(), "カエル".to_string()]);

        assert_eq!(table.row_count(), 3);

        // Verify Unicode is preserved
        let first_row = table.selected_row().unwrap();
        assert!(first_row[0].contains("🎉"));
        assert!(first_row[1].contains("こんにちは"));
    }

    #[test]
    fn test_table_very_long_cell_content() {
        let columns = vec![
            TableColumn::new("Short", 10),
            TableColumn::new("Long Content", 50),
        ];
        let mut table = DataTable::new("Long Cell Table", columns);

        let long_text = "x".repeat(1000);
        table.add_row(vec!["Short".to_string(), long_text.clone()]);

        assert_eq!(table.selected_row().unwrap()[1].len(), 1000);
        assert_eq!(table.selected_row().unwrap()[1], long_text);
    }

    #[test]
    fn test_table_column_alignment() {
        let columns = vec![
            TableColumn::new("Left", 15).with_alignment(ColumnAlignment::Left),
            TableColumn::new("Center", 15).with_alignment(ColumnAlignment::Center),
            TableColumn::new("Right", 15).with_alignment(ColumnAlignment::Right),
        ];

        assert_eq!(columns[0].alignment, ColumnAlignment::Left);
        assert_eq!(columns[1].alignment, ColumnAlignment::Center);
        assert_eq!(columns[2].alignment, ColumnAlignment::Right);
    }

    #[test]
    fn test_table_column_width_extremes() {
        // Zero width column
        let col_zero = TableColumn::new("Zero", 0);
        assert_eq!(col_zero.width, 0);

        // Very large width
        let col_large = TableColumn::new("Large", u16::MAX);
        assert_eq!(col_large.width, u16::MAX);
    }

    #[test]
    fn test_table_empty_navigation() {
        let columns = vec![TableColumn::new("Data", 20)];
        let mut table = DataTable::new("Empty Nav Test", columns);

        // Navigation on empty table should be no-op
        table.select_next();
        assert_eq!(table.selected(), Some(0)); // Still default

        table.select_previous();
        assert_eq!(table.selected(), Some(0));

        table.select_first();
        assert_eq!(table.selected(), Some(0));

        table.select_last();
        assert_eq!(table.selected(), Some(0));
    }

    #[test]
    fn test_table_selected_row_none() {
        let columns = vec![TableColumn::new("Data", 20)];
        let mut table = DataTable::new("Test", columns);

        table.add_row(vec!["Row 1".to_string()]);
        table.clear();

        // After clear, selection should be None
        assert!(table.selected_row().is_none());
    }

    #[test]
    fn test_table_set_rows_with_selection() {
        let columns = vec![TableColumn::new("Data", 20)];
        let mut table = DataTable::new("Test", columns);

        // Initially empty, no selection
        table.clear();
        assert_eq!(table.selected(), None);

        // Set rows should restore selection to 0
        table.set_rows(vec![
            vec!["Row 1".to_string()],
            vec!["Row 2".to_string()],
        ]);

        assert_eq!(table.selected(), Some(0));
    }

    #[test]
    fn test_table_header_toggle() {
        let columns = vec![TableColumn::new("Header", 20)];
        let mut table = DataTable::new("Toggle Test", columns);

        // Default should show header
        assert!(table.show_header);

        table.set_show_header(false);
        assert!(!table.show_header);

        table.set_show_header(true);
        assert!(table.show_header);
    }

    #[test]
    fn test_table_mismatched_column_count() {
        let columns = vec![
            TableColumn::new("Col1", 10),
            TableColumn::new("Col2", 10),
            TableColumn::new("Col3", 10),
        ];
        let mut table = DataTable::new("Mismatch Test", columns);

        // Add row with fewer columns than defined
        table.add_row(vec!["A".to_string(), "B".to_string()]);

        // Add row with more columns than defined
        table.add_row(vec![
            "X".to_string(),
            "Y".to_string(),
            "Z".to_string(),
            "Extra".to_string(),
        ]);

        assert_eq!(table.row_count(), 2);
        // Table should handle mismatches gracefully
    }

    #[test]
    fn test_table_empty_strings_in_cells() {
        let columns = vec![TableColumn::new("Data", 20)];
        let mut table = DataTable::new("Empty Cells", columns);

        table.add_row(vec!["".to_string()]);
        table.add_row(vec!["Non-empty".to_string()]);
        table.add_row(vec!["".to_string()]);

        assert_eq!(table.row_count(), 3);
        assert_eq!(table.selected_row().unwrap()[0], "");
    }

    #[test]
    fn test_table_newlines_in_cells() {
        let columns = vec![TableColumn::new("Data", 20)];
        let mut table = DataTable::new("Newline Test", columns);

        table.add_row(vec!["Line 1\nLine 2\nLine 3".to_string()]);

        assert!(table.selected_row().unwrap()[0].contains('\n'));
    }

    #[test]
    fn test_table_rapid_selection_changes() {
        let columns = vec![TableColumn::new("Data", 20)];
        let mut table = DataTable::new("Rapid Test", columns);

        table.set_rows(vec![
            vec!["Row 1".to_string()],
            vec!["Row 2".to_string()],
            vec!["Row 3".to_string()],
            vec!["Row 4".to_string()],
            vec!["Row 5".to_string()],
        ]);

        // Rapid next
        for _ in 0..100 {
            table.select_next();
        }

        // Should wrap around correctly
        assert_eq!(table.selected(), Some(0)); // 100 % 5 = 0

        // Rapid previous
        for _ in 0..50 {
            table.select_previous();
        }

        assert_eq!(table.selected(), Some(0)); // 50 % 5 = 0
    }

    #[test]
    fn test_table_column_alignment_all_variants() {
        // Ensure all alignment variants work
        let left = ColumnAlignment::Left;
        let center = ColumnAlignment::Center;
        let right = ColumnAlignment::Right;

        assert_eq!(left, ColumnAlignment::Left);
        assert_eq!(center, ColumnAlignment::Center);
        assert_eq!(right, ColumnAlignment::Right);

        // Test PartialEq
        assert_ne!(left, center);
        assert_ne!(center, right);
        assert_ne!(left, right);
    }

    #[test]
    fn test_table_column_clone() {
        let col1 = TableColumn::new("Test", 20)
            .with_alignment(ColumnAlignment::Center);

        let col2 = col1.clone();

        assert_eq!(col1.header, col2.header);
        assert_eq!(col1.width, col2.width);
        assert_eq!(col1.alignment, col2.alignment);
    }

    #[test]
    fn test_table_column_debug() {
        let col = TableColumn::new("Debug Test", 15)
            .with_alignment(ColumnAlignment::Right);

        let debug_str = format!("{:?}", col);
        assert!(debug_str.contains("Debug Test"));
        assert!(debug_str.contains("15"));
    }

    #[test]
    fn test_table_add_row_multiple_times() {
        let columns = vec![TableColumn::new("Data", 20)];
        let mut table = DataTable::new("Multi Add Test", columns);

        // Add same row multiple times
        for i in 0..5 {
            table.add_row(vec![format!("Row {}", i)]);
        }

        assert_eq!(table.row_count(), 5);

        // Select each row and verify content
        for i in 0..5 {
            table.select_first();
            for _ in 0..i {
                table.select_next();
            }
            assert_eq!(
                table.selected_row().unwrap()[0],
                format!("Row {}", i)
            );
        }
    }

    #[test]
    fn test_table_special_characters_in_cells() {
        let columns = vec![TableColumn::new("Special", 30)];
        let mut table = DataTable::new("Special Chars", columns);

        table.add_row(vec!["<>&\"'".to_string()]);
        table.add_row(vec!["\t\r\n".to_string()]);
        table.add_row(vec!["!@#$%^&*()".to_string()]);

        assert_eq!(table.row_count(), 3);
    }

    #[test]
    fn test_table_selection_persistence_after_add_row() {
        let columns = vec![TableColumn::new("Data", 20)];
        let mut table = DataTable::new("Persistence Test", columns);

        table.set_rows(vec![
            vec!["Row 1".to_string()],
            vec!["Row 2".to_string()],
        ]);

        // Select second row
        table.select_next();
        assert_eq!(table.selected(), Some(1));

        // Add new row
        table.add_row(vec!["Row 3".to_string()]);

        // Selection should remain on index 1
        assert_eq!(table.selected(), Some(1));
        assert_eq!(table.row_count(), 3);
    }

    #[test]
    fn test_table_boundary_navigation_wrap_around() {
        let columns = vec![TableColumn::new("Data", 20)];
        let mut table = DataTable::new("Wrap Test", columns);

        table.set_rows(vec![
            vec!["First".to_string()],
            vec!["Middle".to_string()],
            vec!["Last".to_string()],
        ]);

        // Start at first
        table.select_first();
        assert_eq!(table.selected(), Some(0));

        // Previous from first wraps to last
        table.select_previous();
        assert_eq!(table.selected(), Some(2));

        // Next from last wraps to first
        table.select_next();
        assert_eq!(table.selected(), Some(0));
    }
}
