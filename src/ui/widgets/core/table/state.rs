//! Table widget
//!
//! Multi-column table with headers, sorting, and selection

use crate::ui::{atoms::block::Block as AtomBlock, theme::ToadTheme};
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    widgets::{
        Borders, Row, Scrollbar, ScrollbarOrientation, ScrollbarState, Table as RatatuiTable,
        TableState,
    },
    Frame,
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
    pub(super) title: String,
    pub(super) columns: Vec<TableColumn>,
    pub(super) rows: Vec<Vec<String>>,
    pub(super) state: TableState,
    pub(super) show_header: bool,
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
        // Use Block atom for table border
        let block_atom = AtomBlock::new()
            .title(&format!(" {} ", self.title))
            .title_style(
                Style::default()
                    .fg(ToadTheme::TOAD_GREEN)
                    .add_modifier(Modifier::BOLD),
            )
            .borders(Borders::ALL)
            .border_style(Style::default().fg(ToadTheme::TOAD_GREEN));

        let block = block_atom.to_ratatui();
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
    fn test_table_column_new() {
        let col = TableColumn::new("Name", 20);
        assert_eq!(col.header, "Name");
        assert_eq!(col.width, 20);
        assert_eq!(col.alignment, ColumnAlignment::Left);
    }

    #[test]
    fn test_table_column_with_alignment() {
        let col = TableColumn::new("Age", 10).with_alignment(ColumnAlignment::Right);
        assert_eq!(col.alignment, ColumnAlignment::Right);
    }

    #[test]
    fn test_data_table_new() {
        let columns = vec![
            TableColumn::new("Name", 20),
            TableColumn::new("Age", 10),
        ];
        let table = DataTable::new("People", columns.clone());
        assert_eq!(table.title, "People");
        assert_eq!(table.columns.len(), 2);
        assert_eq!(table.rows.len(), 0);
        assert_eq!(table.selected(), Some(0));
        assert!(table.show_header);
    }

    #[test]
    fn test_add_row() {
        let columns = vec![TableColumn::new("Name", 20)];
        let mut table = DataTable::new("Test", columns);

        table.add_row(vec!["Alice".to_string()]);
        assert_eq!(table.row_count(), 1);

        table.add_row(vec!["Bob".to_string()]);
        assert_eq!(table.row_count(), 2);
    }

    #[test]
    fn test_set_rows() {
        let columns = vec![TableColumn::new("Name", 20)];
        let mut table = DataTable::new("Test", columns);

        let rows = vec![
            vec!["Alice".to_string()],
            vec!["Bob".to_string()],
            vec!["Charlie".to_string()],
        ];

        table.set_rows(rows);
        assert_eq!(table.row_count(), 3);
        assert_eq!(table.selected(), Some(0));
    }

    #[test]
    fn test_selected_row() {
        let columns = vec![TableColumn::new("Name", 20)];
        let mut table = DataTable::new("Test", columns);

        table.add_row(vec!["Alice".to_string()]);
        table.add_row(vec!["Bob".to_string()]);

        assert_eq!(table.selected_row(), Some(&vec!["Alice".to_string()]));

        table.select_next();
        assert_eq!(table.selected_row(), Some(&vec!["Bob".to_string()]));
    }

    #[test]
    fn test_select_next_wraps_around() {
        let columns = vec![TableColumn::new("Name", 20)];
        let mut table = DataTable::new("Test", columns);

        table.add_row(vec!["Alice".to_string()]);
        table.add_row(vec!["Bob".to_string()]);

        assert_eq!(table.selected(), Some(0));

        table.select_next();
        assert_eq!(table.selected(), Some(1));

        table.select_next(); // Should wrap to 0
        assert_eq!(table.selected(), Some(0));
    }

    #[test]
    fn test_select_previous_wraps_around() {
        let columns = vec![TableColumn::new("Name", 20)];
        let mut table = DataTable::new("Test", columns);

        table.add_row(vec!["Alice".to_string()]);
        table.add_row(vec!["Bob".to_string()]);

        assert_eq!(table.selected(), Some(0));

        table.select_previous(); // Should wrap to last
        assert_eq!(table.selected(), Some(1));

        table.select_previous();
        assert_eq!(table.selected(), Some(0));
    }

    #[test]
    fn test_select_first() {
        let columns = vec![TableColumn::new("Name", 20)];
        let mut table = DataTable::new("Test", columns);

        table.add_row(vec!["Alice".to_string()]);
        table.add_row(vec!["Bob".to_string()]);
        table.add_row(vec!["Charlie".to_string()]);

        table.select_last();
        assert_eq!(table.selected(), Some(2));

        table.select_first();
        assert_eq!(table.selected(), Some(0));
    }

    #[test]
    fn test_select_last() {
        let columns = vec![TableColumn::new("Name", 20)];
        let mut table = DataTable::new("Test", columns);

        table.add_row(vec!["Alice".to_string()]);
        table.add_row(vec!["Bob".to_string()]);
        table.add_row(vec!["Charlie".to_string()]);

        assert_eq!(table.selected(), Some(0));

        table.select_last();
        assert_eq!(table.selected(), Some(2));
    }

    #[test]
    fn test_clear() {
        let columns = vec![TableColumn::new("Name", 20)];
        let mut table = DataTable::new("Test", columns);

        table.add_row(vec!["Alice".to_string()]);
        table.add_row(vec!["Bob".to_string()]);

        assert_eq!(table.row_count(), 2);
        assert!(table.selected().is_some());

        table.clear();
        assert_eq!(table.row_count(), 0);
        assert_eq!(table.selected(), None);
    }

    #[test]
    fn test_select_next_on_empty_table() {
        let columns = vec![TableColumn::new("Name", 20)];
        let mut table = DataTable::new("Test", columns);

        table.select_next(); // Should not panic
        assert_eq!(table.row_count(), 0);
    }

    #[test]
    fn test_select_previous_on_empty_table() {
        let columns = vec![TableColumn::new("Name", 20)];
        let mut table = DataTable::new("Test", columns);

        table.select_previous(); // Should not panic
        assert_eq!(table.row_count(), 0);
    }

    #[test]
    fn test_select_first_on_empty_table() {
        let columns = vec![TableColumn::new("Name", 20)];
        let mut table = DataTable::new("Test", columns);

        table.select_first(); // Should not panic
        assert_eq!(table.selected(), Some(0)); // Initial state
    }

    #[test]
    fn test_select_last_on_empty_table() {
        let columns = vec![TableColumn::new("Name", 20)];
        let mut table = DataTable::new("Test", columns);

        table.select_last(); // Should not panic
        assert_eq!(table.selected(), Some(0)); // Initial state unchanged
    }

    #[test]
    fn test_set_show_header() {
        let columns = vec![TableColumn::new("Name", 20)];
        let mut table = DataTable::new("Test", columns);

        assert!(table.show_header);

        table.set_show_header(false);
        assert!(!table.show_header);

        table.set_show_header(true);
        assert!(table.show_header);
    }

    #[test]
    fn test_selected_row_on_empty_table() {
        let columns = vec![TableColumn::new("Name", 20)];
        let table = DataTable::new("Test", columns);

        assert_eq!(table.selected_row(), None);
    }

    #[test]
    fn test_row_count() {
        let columns = vec![TableColumn::new("Name", 20)];
        let mut table = DataTable::new("Test", columns);

        assert_eq!(table.row_count(), 0);

        table.add_row(vec!["Alice".to_string()]);
        assert_eq!(table.row_count(), 1);

        table.add_row(vec!["Bob".to_string()]);
        assert_eq!(table.row_count(), 2);

        table.clear();
        assert_eq!(table.row_count(), 0);
    }

    #[test]
    fn test_multi_column_table() {
        let columns = vec![
            TableColumn::new("Name", 20),
            TableColumn::new("Age", 10),
            TableColumn::new("City", 15),
        ];
        let mut table = DataTable::new("People", columns);

        table.add_row(vec!["Alice".to_string(), "30".to_string(), "NYC".to_string()]);
        table.add_row(vec!["Bob".to_string(), "25".to_string(), "LA".to_string()]);

        assert_eq!(table.row_count(), 2);
        assert_eq!(table.selected_row().unwrap().len(), 3);
        assert_eq!(table.selected_row().unwrap()[0], "Alice");
        assert_eq!(table.selected_row().unwrap()[1], "30");
        assert_eq!(table.selected_row().unwrap()[2], "NYC");
    }
}
