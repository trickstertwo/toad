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
}
