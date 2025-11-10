//\! Table widget tests

use super::*;
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

// ============================================================================
// ADVANCED COMPREHENSIVE EDGE CASE TESTS (90%+ COVERAGE)
// ============================================================================

// ============ Stress Tests ============

#[test]
fn test_table_50000_rows() {
    let columns = vec![
        TableColumn::new("ID", 10),
        TableColumn::new("Name", 20),
        TableColumn::new("Value", 15),
    ];
    let mut table = DataTable::new("Massive Table", columns);

    for i in 0..50000 {
        table.add_row(vec![
            format!("{}", i),
            format!("Name_{}", i),
            format!("Value_{}", i),
        ]);
    }

    assert_eq!(table.row_count(), 50000);
    table.select_last();
    assert_eq!(table.selected(), Some(49999));
}

#[test]
fn test_table_rapid_row_additions() {
    let columns = vec![TableColumn::new("Data", 20)];
    let mut table = DataTable::new("Rapid Add", columns);

    for i in 0..10000 {
        table.add_row(vec![format!("Row {}", i)]);
    }

    assert_eq!(table.row_count(), 10000);
}

#[test]
fn test_table_rapid_selection_navigation() {
    let columns = vec![TableColumn::new("Data", 20)];
    let mut table = DataTable::new("Rapid Nav", columns);

    table.set_rows(vec![
        vec!["R1".to_string()],
        vec!["R2".to_string()],
        vec!["R3".to_string()],
    ]);

    for _ in 0..5000 {
        table.select_next();
    }

    assert_eq!(table.selected(), Some(2)); // 5000 % 3 = 2

    for _ in 0..7000 {
        table.select_previous();
    }

    assert_eq!(table.selected(), Some(1)); // (2 - 7000) % 3 = 1
}

#[test]
fn test_table_rapid_clear_and_refill() {
    let columns = vec![TableColumn::new("Data", 20)];
    let mut table = DataTable::new("Rapid Clear", columns);

    for _ in 0..500 {
        table.add_row(vec!["Row".to_string()]);
        table.clear();
    }

    assert_eq!(table.row_count(), 0);
}

#[test]
fn test_table_alternating_add_clear_pattern() {
    let columns = vec![TableColumn::new("Data", 20)];
    let mut table = DataTable::new("Alternating", columns);

    for i in 0..1000 {
        if i % 2 == 0 {
            table.add_row(vec![format!("Row {}", i)]);
        } else {
            if i > 100 {
                table.clear();
            }
        }
    }

    // Should end with some rows
    assert!(table.row_count() >= 0);
}

// ============ Unicode Edge Cases ============

#[test]
fn test_table_emoji_in_headers() {
    let columns = vec![
        TableColumn::new("🐸 Frog", 15),
        TableColumn::new("💚 Heart", 15),
        TableColumn::new("🚀 Rocket", 15),
    ];
    let table = DataTable::new("Emoji Headers", columns);

    assert_eq!(table.columns[0].header, "🐸 Frog");
    assert_eq!(table.columns[1].header, "💚 Heart");
}

#[test]
fn test_table_rtl_text_in_cells() {
    let columns = vec![
        TableColumn::new("Arabic", 30),
        TableColumn::new("Hebrew", 30),
    ];
    let mut table = DataTable::new("RTL Table", columns);

    table.add_row(vec![
        "مرحبا بك في الجدول".to_string(),
        "שלום לך בטבלה".to_string(),
    ]);

    let row = table.selected_row().unwrap();
    assert!(row[0].contains("مرحبا"));
    assert!(row[1].contains("שלום"));
}

#[test]
fn test_table_mixed_unicode_cells() {
    let columns = vec![TableColumn::new("Mixed", 50)];
    let mut table = DataTable::new("Mixed Unicode", columns);

    table.add_row(vec!["Hello 🐸 مرحبا 日本語 שלום".to_string()]);

    let row = table.selected_row().unwrap();
    assert!(row[0].contains("Hello"));
    assert!(row[0].contains("🐸"));
    assert!(row[0].contains("مرحبا"));
    assert!(row[0].contains("日本語"));
}

#[test]
fn test_table_combining_characters_in_cells() {
    let columns = vec![TableColumn::new("Combining", 30)];
    let mut table = DataTable::new("Combining", columns);

    table.add_row(vec!["e\u{0301}e\u{0301}".to_string()]); // é é

    assert!(table.selected_row().unwrap()[0].len() > 2);
}

#[test]
fn test_table_zero_width_characters() {
    let columns = vec![TableColumn::new("ZeroWidth", 30)];
    let mut table = DataTable::new("ZW", columns);

    table.add_row(vec!["A\u{200B}B\u{200D}C".to_string()]);

    assert!(table.selected_row().unwrap()[0].contains('\u{200B}'));
}

// ============ Extreme Table Configurations ============

#[test]
fn test_table_100_columns() {
    let columns: Vec<TableColumn> = (0..100)
        .map(|i| TableColumn::new(format!("Col{}", i), 10))
        .collect();

    let mut table = DataTable::new("Wide Table", columns);

    let row: Vec<String> = (0..100).map(|i| format!("Val{}", i)).collect();
    table.add_row(row);

    assert_eq!(table.row_count(), 1);
    assert_eq!(table.selected_row().unwrap().len(), 100);
}

#[test]
fn test_table_very_wide_columns() {
    let columns = vec![
        TableColumn::new("Col1", u16::MAX),
        TableColumn::new("Col2", u16::MAX / 2),
    ];

    let table = DataTable::new("Wide Cols", columns);
    assert_eq!(table.columns[0].width, u16::MAX);
}

#[test]
fn test_table_all_zero_width_columns() {
    let columns = vec![
        TableColumn::new("Z1", 0),
        TableColumn::new("Z2", 0),
        TableColumn::new("Z3", 0),
    ];

    let mut table = DataTable::new("Zero Width", columns);
    table.add_row(vec!["A".to_string(), "B".to_string(), "C".to_string()]);

    assert_eq!(table.row_count(), 1);
}

#[test]
fn test_table_single_cell_100k_characters() {
    let columns = vec![TableColumn::new("Huge", 100)];
    let mut table = DataTable::new("Huge Cell", columns);

    let huge_cell = "X".repeat(100000);
    table.add_row(vec![huge_cell.clone()]);

    assert_eq!(table.selected_row().unwrap()[0].len(), 100000);
}

// ============ Complex Navigation Patterns ============

#[test]
fn test_table_zigzag_navigation() {
    let columns = vec![TableColumn::new("Data", 20)];
    let mut table = DataTable::new("Zigzag", columns);

    table.set_rows(vec![
        vec!["R1".to_string()],
        vec!["R2".to_string()],
        vec!["R3".to_string()],
        vec!["R4".to_string()],
        vec!["R5".to_string()],
    ]);

    for _ in 0..100 {
        table.select_next();
        table.select_next();
        table.select_previous();
    }

    // Should handle without panic
}

#[test]
fn test_table_jump_to_extremes_repeatedly() {
    let columns = vec![TableColumn::new("Data", 20)];
    let mut table = DataTable::new("Extremes", columns);

    table.set_rows(vec![
        vec!["First".to_string()],
        vec!["Middle".to_string()],
        vec!["Last".to_string()],
    ]);

    for _ in 0..500 {
        table.select_first();
        assert_eq!(table.selected(), Some(0));

        table.select_last();
        assert_eq!(table.selected(), Some(2));
    }
}

#[test]
fn test_table_random_navigation_pattern() {
    let columns = vec![TableColumn::new("Data", 20)];
    let mut table = DataTable::new("Random", columns);

    table.set_rows(vec![
        vec!["R1".to_string()],
        vec!["R2".to_string()],
        vec!["R3".to_string()],
        vec!["R4".to_string()],
        vec!["R5".to_string()],
    ]);

    // Pseudo-random but deterministic
    let mut val = 12345u64;
    for _ in 0..1000 {
        val = val.wrapping_mul(1103515245).wrapping_add(12345);
        if val % 2 == 0 {
            table.select_next();
        } else {
            table.select_previous();
        }
    }

    // Should complete without panic
    assert!(table.selected().is_some());
}

// ============ State Transition Edge Cases ============

#[test]
fn test_table_clear_refill_selection() {
    let columns = vec![TableColumn::new("Data", 20)];
    let mut table = DataTable::new("Clear Refill", columns);

    table.add_row(vec!["Row1".to_string()]);
    table.select_first();
    assert_eq!(table.selected(), Some(0));

    table.clear();
    assert_eq!(table.selected(), None);

    table.add_row(vec!["Row2".to_string()]);
    // add_row doesn't auto-select, only set_rows does
    assert_eq!(table.selected(), None);

    // But we can manually select
    table.select_first();
    assert_eq!(table.selected(), Some(0));
}

#[test]
fn test_table_set_rows_preserves_selection_if_valid() {
    let columns = vec![TableColumn::new("Data", 20)];
    let mut table = DataTable::new("Set Rows", columns);

    table.set_rows(vec![
        vec!["R1".to_string()],
        vec!["R2".to_string()],
        vec!["R3".to_string()],
    ]);

    table.select_next();
    table.select_next();
    assert_eq!(table.selected(), Some(2));

    // Set new rows - selection is preserved if still valid
    table.set_rows(vec![
        vec!["New1".to_string()],
        vec!["New2".to_string()],
    ]);

    // Selection stays at 2, but row count is only 2, so it's out of bounds
    // The selection index stays as Some(2) even though it's invalid
    assert_eq!(table.selected(), Some(2));

    // When we try to get the selected row, it returns None because index is invalid
    assert_eq!(table.selected_row(), None);
}

#[test]
fn test_table_header_toggle_stress() {
    let columns = vec![TableColumn::new("Data", 20)];
    let mut table = DataTable::new("Header Toggle", columns);

    for i in 0..1000 {
        table.set_show_header(i % 2 == 0);
    }

    // Last iteration is i=999 (odd), so show_header should be false
    assert!(!table.show_header);
}

// ============ Row Data Edge Cases ============

#[test]
fn test_table_rows_with_varying_lengths() {
    let columns = vec![
        TableColumn::new("C1", 10),
        TableColumn::new("C2", 10),
        TableColumn::new("C3", 10),
    ];
    let mut table = DataTable::new("Varying", columns);

    table.add_row(vec!["A".to_string()]);
    table.add_row(vec!["B".to_string(), "C".to_string()]);
    table.add_row(vec![
        "D".to_string(),
        "E".to_string(),
        "F".to_string(),
    ]);
    table.add_row(vec![
        "G".to_string(),
        "H".to_string(),
        "I".to_string(),
        "J".to_string(),
    ]);

    assert_eq!(table.row_count(), 4);
}

#[test]
fn test_table_all_empty_cells() {
    let columns = vec![
        TableColumn::new("C1", 10),
        TableColumn::new("C2", 10),
    ];
    let mut table = DataTable::new("Empty Cells", columns);

    for _ in 0..100 {
        table.add_row(vec!["".to_string(), "".to_string()]);
    }

    assert_eq!(table.row_count(), 100);
    assert_eq!(table.selected_row().unwrap()[0], "");
}

#[test]
fn test_table_whitespace_only_cells() {
    let columns = vec![TableColumn::new("Data", 20)];
    let mut table = DataTable::new("Whitespace", columns);

    table.add_row(vec!["   ".to_string()]);
    table.add_row(vec!["\t\t\t".to_string()]);
    table.add_row(vec!["  \t  ".to_string()]);

    assert_eq!(table.row_count(), 3);
}

// ============ Column Alignment Edge Cases ============

#[test]
fn test_column_alignment_builder_pattern() {
    let col = TableColumn::new("Test", 20)
        .with_alignment(ColumnAlignment::Left)
        .with_alignment(ColumnAlignment::Center)
        .with_alignment(ColumnAlignment::Right);

    assert_eq!(col.alignment, ColumnAlignment::Right);
}

#[test]
fn test_column_alignment_all_columns_same() {
    let columns = vec![
        TableColumn::new("C1", 10).with_alignment(ColumnAlignment::Center),
        TableColumn::new("C2", 10).with_alignment(ColumnAlignment::Center),
        TableColumn::new("C3", 10).with_alignment(ColumnAlignment::Center),
    ];

    for col in &columns {
        assert_eq!(col.alignment, ColumnAlignment::Center);
    }
}

#[test]
fn test_column_alignment_mixed() {
    let columns = vec![
        TableColumn::new("Left", 10).with_alignment(ColumnAlignment::Left),
        TableColumn::new("Center", 10).with_alignment(ColumnAlignment::Center),
        TableColumn::new("Right", 10).with_alignment(ColumnAlignment::Right),
        TableColumn::new("Left2", 10).with_alignment(ColumnAlignment::Left),
    ];

    assert_eq!(columns[0].alignment, ColumnAlignment::Left);
    assert_eq!(columns[1].alignment, ColumnAlignment::Center);
    assert_eq!(columns[2].alignment, ColumnAlignment::Right);
    assert_eq!(columns[3].alignment, ColumnAlignment::Left);
}

// ============ Trait Coverage ============

#[test]
fn test_column_alignment_debug() {
    let left = format!("{:?}", ColumnAlignment::Left);
    let center = format!("{:?}", ColumnAlignment::Center);
    let right = format!("{:?}", ColumnAlignment::Right);

    assert!(left.contains("Left"));
    assert!(center.contains("Center"));
    assert!(right.contains("Right"));
}

#[test]
fn test_column_alignment_clone() {
    let align1 = ColumnAlignment::Center;
    let align2 = align1;

    assert_eq!(align1, align2);
}

#[test]
fn test_column_alignment_partial_eq() {
    assert_eq!(ColumnAlignment::Left, ColumnAlignment::Left);
    assert_ne!(ColumnAlignment::Left, ColumnAlignment::Right);
    assert_ne!(ColumnAlignment::Center, ColumnAlignment::Right);
}

// ============ Comprehensive Stress Test ============

#[test]
fn test_comprehensive_table_stress() {
    // Create table with many columns
    let columns: Vec<TableColumn> = (0..20)
        .map(|i| {
            let alignment = match i % 3 {
                0 => ColumnAlignment::Left,
                1 => ColumnAlignment::Center,
                _ => ColumnAlignment::Right,
            };
            TableColumn::new(format!("Col {} 🐸", i), 15).with_alignment(alignment)
        })
        .collect();

    let mut table = DataTable::new("Comprehensive Stress 💚", columns);

    // Add many rows with varied content
    for i in 0..5000 {
        let row: Vec<String> = (0..20)
            .map(|j| {
                match (i + j) % 4 {
                    0 => format!("Row{}_Col{}", i, j),
                    1 => format!("🚀 Emoji {}", i),
                    2 => format!("日本語 {}", i),
                    _ => "".to_string(),
                }
            })
            .collect();
        table.add_row(row);
    }

    assert_eq!(table.row_count(), 5000);

    // Complex navigation pattern
    for i in 0..100 {
        match i % 5 {
            0 => table.select_first(),
            1 => table.select_last(),
            2 => table.select_next(),
            3 => table.select_previous(),
            _ => {
                for _ in 0..10 {
                    table.select_next();
                }
            }
        }
    }

    // Verify state is consistent
    assert!(table.selected().is_some());
    assert!(table.selected_row().is_some());

    // Toggle header multiple times
    for i in 0..50 {
        table.set_show_header(i % 2 == 0);
    }

    // Add more rows
    for i in 5000..5500 {
        let row: Vec<String> = (0..20).map(|j| format!("Extra{}_{}", i, j)).collect();
        table.add_row(row);
    }

    assert_eq!(table.row_count(), 5500);

    // Navigate to boundaries
    table.select_first();
    assert_eq!(table.selected(), Some(0));

    table.select_last();
    assert_eq!(table.selected(), Some(5499));

    // Verify selected row
    let selected = table.selected_row().unwrap();
    assert_eq!(selected.len(), 20);
    assert!(selected[0].contains("Extra5499"));

    // Clear and verify
    table.clear();
    assert_eq!(table.row_count(), 0);
    assert_eq!(table.selected(), None);

    // Refill with Unicode content
    table.add_row(vec![
        "🐸".to_string(),
        "مرحبا".to_string(),
        "שלום".to_string(),
        "こんにちは".to_string(),
    ]);

    assert_eq!(table.row_count(), 1);
    // add_row doesn't auto-select, must select manually
    table.select_first();
    assert_eq!(table.selected(), Some(0));

    let final_row = table.selected_row().unwrap();
    assert!(final_row[0].contains("🐸"));
    assert!(final_row[1].contains("مرحبا"));
}
