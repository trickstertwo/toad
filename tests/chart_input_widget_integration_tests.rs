//! Integration tests for Chart and Input widgets
//!
//! Tests for data visualization (LineChart) and user input (InputField, Dialog) widgets.

use ratatui::style::Color;
use toad::ui::widgets::charts::line_chart::{LineChart, DataSeries};
use toad::ui::widgets::core::dialog::{ConfirmDialog, DialogOption};
use toad::ui::widgets::input::input::{InputField};

// ==================== LineChart Tests ====================

#[test]
fn test_line_chart_creation() {
    let chart = LineChart::new();
    assert_eq!(chart.series_count(), 0);
}

#[test]
fn test_line_chart_builder() {
    let series =
        DataSeries::new("Temperature", vec![10.0, 20.0, 15.0, 25.0, 30.0]).with_color(Color::Red);

    let chart = LineChart::new()
        .add_series(series)
        .with_title("Temperature Chart")
        .with_x_label("Time")
        .with_y_label("°C");

    assert_eq!(chart.series_count(), 1);
}

#[test]
fn test_line_chart_add_series() {
    let series1 = DataSeries::new("Series 1", vec![1.0, 2.0, 3.0]);
    let series2 = DataSeries::new("Series 2", vec![10.0, 20.0, 30.0, 40.0]);

    let chart = LineChart::new().add_series(series1).add_series(series2);

    assert_eq!(chart.series_count(), 2);
}

#[test]
fn test_line_chart_multiple_series() {
    let series1 = DataSeries::new("A", vec![1.0, 2.0, 3.0]);
    let series2 = DataSeries::new("B", vec![4.0, 5.0]);
    let series3 = DataSeries::new("C", vec![6.0, 7.0, 8.0, 9.0]);

    let chart = LineChart::new()
        .add_series(series1)
        .add_series(series2)
        .add_series(series3);

    assert_eq!(chart.series_count(), 3);
}

#[test]
fn test_data_series_creation() {
    let series = DataSeries::new("Test", vec![1.0, 2.0, 3.0]);
    assert_eq!(series.name, "Test");
    assert_eq!(series.data.len(), 3);
}

#[test]
fn test_line_chart_empty_data() {
    let series = DataSeries::new("Empty", vec![]);
    let chart = LineChart::new().add_series(series);
    assert_eq!(chart.series_count(), 1);
}

#[test]
fn test_line_chart_single_point() {
    let series = DataSeries::new("Single", vec![42.0]);
    let chart = LineChart::new().add_series(series);
    assert_eq!(chart.series_count(), 1);
}

#[test]
fn test_line_chart_negative_values() {
    let series = DataSeries::new("Negatives", vec![-10.0, -5.0, 0.0, 5.0, 10.0]);
    let chart = LineChart::new().add_series(series);
    assert_eq!(chart.series_count(), 1);
}

// ==================== InputField Tests ====================

#[test]
fn test_input_field_creation() {
    let input = InputField::new();

    assert_eq!(input.value(), "");
    assert!(!input.is_focused());
}

#[test]
fn test_input_field_set_value() {
    let mut input = InputField::new();

    input.set_value("Hello, World!".to_string());
    assert_eq!(input.value(), "Hello, World!");
}

#[test]
fn test_input_field_insert_char() {
    let mut input = InputField::new();

    input.insert_char('H');
    input.insert_char('e');
    input.insert_char('l');
    input.insert_char('l');
    input.insert_char('o');

    assert_eq!(input.value(), "Hello");
}

#[test]
fn test_input_field_delete_char() {
    let mut input = InputField::new();

    input.set_value("Hello".to_string());
    input.delete_char();

    assert_eq!(input.value(), "Hell");

    input.delete_char();
    assert_eq!(input.value(), "Hel");
}

#[test]
fn test_input_field_cursor_movement() {
    let mut input = InputField::new();
    input.set_value("Hello".to_string());

    // Move to start
    input.move_cursor_start();

    // Insert at start
    input.insert_char('X');
    assert_eq!(input.value(), "XHello");

    // Move to end
    input.move_cursor_end();
    input.insert_char('!');
    assert_eq!(input.value(), "XHello!");
}

#[test]
fn test_input_field_cursor_left_right() {
    let mut input = InputField::new();
    input.set_value("ABC".to_string());

    // Cursor at end
    input.move_cursor_left();
    input.delete_char();
    assert_eq!(input.value(), "AC"); // Deleted 'B'

    input.move_cursor_left();
    input.insert_char('X');
    assert_eq!(input.value(), "XAC"); // Inserted 'X' at start
}

#[test]
fn test_input_field_clear() {
    let mut input = InputField::new();

    input.set_value("Some text".to_string());
    assert_eq!(input.value(), "Some text");

    input.clear();
    assert_eq!(input.value(), "");
}

#[test]
fn test_input_field_focus() {
    let mut input = InputField::new();

    assert!(!input.is_focused());

    input.set_focused(true);
    assert!(input.is_focused());

    input.set_focused(false);
    assert!(!input.is_focused());
}

#[test]
fn test_input_field_placeholder() {
    let input = InputField::new().with_placeholder("Enter your name");

    // Placeholder doesn't affect value
    assert_eq!(input.value(), "");
}

// ==================== DialogOption Tests ====================

#[test]
fn test_dialog_option_creation() {
    let option = DialogOption::new('y', "Yes");

    assert_eq!(option.label, "Yes");
    assert_eq!(option.key, 'y');
}

// ==================== ConfirmDialog Tests ====================

#[test]
fn test_confirm_dialog_creation() {
    let dialog = ConfirmDialog::new("Confirm Action");

    assert_eq!(dialog.selected(), 0);
}

#[test]
fn test_confirm_dialog_builder() {
    let dialog = ConfirmDialog::new("Delete File?")
        .message("Are you sure you want to delete this file?")
        .message("This action cannot be undone.")
        .option('y', "Yes, delete")
        .option('n', "No, cancel")
        .info_box("Warning: This is a destructive operation");

    assert_eq!(dialog.selected(), 0);
}

#[test]
fn test_confirm_dialog_navigation() {
    let mut dialog = ConfirmDialog::new("Choose Option")
        .option('a', "Option A")
        .option('b', "Option B")
        .option('c', "Option C");

    assert_eq!(dialog.selected(), 0);

    dialog.select_next();
    assert_eq!(dialog.selected(), 1);

    dialog.select_next();
    assert_eq!(dialog.selected(), 2);

    // Can't go beyond last option
    dialog.select_next();
    assert_eq!(dialog.selected(), 2);

    dialog.select_previous();
    assert_eq!(dialog.selected(), 1);

    dialog.select_previous();
    assert_eq!(dialog.selected(), 0);

    // Can't go before first option
    dialog.select_previous();
    assert_eq!(dialog.selected(), 0);
}

#[test]
fn test_confirm_dialog_select_by_key() {
    let mut dialog = ConfirmDialog::new("Menu")
        .option('1', "First")
        .option('2', "Second")
        .option('3', "Third");

    assert_eq!(dialog.selected(), 0);

    let result = dialog.select_by_key('3');
    assert_eq!(result, Some(2));
    assert_eq!(dialog.selected(), 2);

    let result = dialog.select_by_key('1');
    assert_eq!(result, Some(0));
    assert_eq!(dialog.selected(), 0);

    let result = dialog.select_by_key('x');
    assert_eq!(result, None);
    assert_eq!(dialog.selected(), 0); // Unchanged
}

// ==================== Cross-Widget Integration Tests ====================

#[test]
fn test_chart_data_analysis_workflow() {
    // Collect data
    let cpu_data = vec![30.0, 45.0, 60.0, 75.0, 65.0, 50.0, 40.0];

    // Find min/max for analysis
    let cpu_max = cpu_data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    let cpu_min = cpu_data.iter().fold(f64::INFINITY, |a, &b| a.min(b));

    assert_eq!(cpu_max, 75.0);
    assert_eq!(cpu_min, 30.0);

    // Create visualization
    let series = DataSeries::new("CPU Usage", cpu_data).with_color(Color::Red);
    let cpu_chart = LineChart::new()
        .add_series(series)
        .with_title("CPU Usage %")
        .with_x_label("Time")
        .with_y_label("%");

    assert_eq!(cpu_chart.series_count(), 1);
}

#[test]
fn test_input_dialog_workflow() {
    // User types in input field
    let mut input = InputField::new().with_placeholder("Enter filename");

    input.insert_char('t');
    input.insert_char('e');
    input.insert_char('s');
    input.insert_char('t');
    input.insert_char('.');
    input.insert_char('r');
    input.insert_char('s');

    assert_eq!(input.value(), "test.rs");

    // Confirm dialog appears
    let mut dialog = ConfirmDialog::new("File exists")
        .message("The file 'test.rs' already exists.")
        .message("Do you want to overwrite it?")
        .option('y', "Yes, overwrite")
        .option('n', "No, cancel")
        .option('r', "Rename");

    // User selects rename option
    dialog.select_by_key('r');
    assert_eq!(dialog.selected(), 2);

    // User modifies input
    input.move_cursor_end();
    input.delete_char(); // Remove 's'
    input.delete_char(); // Remove 'r'
    input.delete_char(); // Remove '.'
    input.insert_char('2');
    input.insert_char('.');
    input.insert_char('r');
    input.insert_char('s');

    assert_eq!(input.value(), "test2.rs");
}

#[test]
fn test_multi_chart_dashboard() {
    // Create multiple charts for a dashboard

    // Chart 1: Temperature over time
    let temp_series =
        DataSeries::new("Temperature", vec![20.0, 22.0, 25.0, 23.0, 21.0]).with_color(Color::Red);
    let temp_chart = LineChart::new()
        .add_series(temp_series)
        .with_title("Temperature (°C)")
        .with_y_label("°C");

    // Chart 2: Network traffic (download)
    let network_series =
        DataSeries::new("Download", vec![5.0, 6.2, 7.5, 6.8, 5.5]).with_color(Color::Blue);
    let network_chart = LineChart::new()
        .add_series(network_series)
        .with_title("Network Download (MB/s)")
        .with_y_label("MB/s");

    // Chart 3: System load (1 min average)
    let load_series =
        DataSeries::new("Load", vec![1.5, 2.0, 2.5, 2.2, 1.8]).with_color(Color::Green);
    let load_chart = LineChart::new()
        .add_series(load_series)
        .with_title("System Load (1 min)");

    assert_eq!(temp_chart.series_count(), 1);
    assert_eq!(network_chart.series_count(), 1);
    assert_eq!(load_chart.series_count(), 1);
}

// ==================== Real-World Scenario Tests ====================

#[test]
fn test_scenario_performance_monitoring() {
    // Scenario: Monitor application performance metrics

    // Collect performance data
    let response_times = vec![45.0, 52.0, 48.0, 150.0, 55.0, 50.0, 200.0, 53.0];

    // Detect anomalies
    let max_response = response_times
        .iter()
        .fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    assert_eq!(max_response, 200.0); // Detected spike

    // Create visualization
    let series = DataSeries::new("Response Time", response_times).with_color(Color::Blue);
    let chart = LineChart::new()
        .add_series(series)
        .with_title("Response Time (ms)")
        .with_x_label("Request #")
        .with_y_label("ms")
        .with_grid(true);

    assert_eq!(chart.series_count(), 1);
}

#[test]
fn test_scenario_user_input_validation() {
    // Scenario: Validate user input with dialog confirmation

    let mut input = InputField::new().with_placeholder("Enter port number");

    // User types port
    input.insert_char('8');
    input.insert_char('0');
    input.insert_char('8');
    input.insert_char('0');

    assert_eq!(input.value(), "8080");

    // Confirm dialog
    let mut dialog = ConfirmDialog::new("Confirm Port")
        .message("Start server on port 8080?")
        .option('y', "Yes, start server")
        .option('n', "No, change port")
        .option('d', "Use default (3000)");

    // User changes mind
    dialog.select_by_key('n');
    assert_eq!(dialog.selected(), 1);

    // User changes port
    input.clear();
    input.insert_char('3');
    input.insert_char('0');
    input.insert_char('0');
    input.insert_char('0');

    assert_eq!(input.value(), "3000");
}

#[test]
fn test_scenario_data_comparison_charts() {
    // Scenario: Compare multiple datasets

    let week1_sales = vec![100.0, 120.0, 110.0, 130.0, 125.0, 140.0, 135.0];
    let week2_sales = vec![105.0, 125.0, 115.0, 140.0, 135.0, 150.0, 145.0];
    let week3_sales = vec![110.0, 130.0, 120.0, 145.0, 140.0, 155.0, 150.0];

    // Analyze trends
    let max1 = week1_sales.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    let max2 = week2_sales.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    let max3 = week3_sales.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

    assert_eq!(max1, 140.0);
    assert_eq!(max2, 150.0);
    assert_eq!(max3, 155.0); // Upward trend!

    // Create charts for each week
    let series1 = DataSeries::new("Week 1", week1_sales).with_color(Color::Red);
    let chart1 = LineChart::new()
        .add_series(series1)
        .with_title("Week 1 Sales");

    let series2 = DataSeries::new("Week 2", week2_sales).with_color(Color::Green);
    let chart2 = LineChart::new()
        .add_series(series2)
        .with_title("Week 2 Sales");

    let series3 = DataSeries::new("Week 3", week3_sales).with_color(Color::Blue);
    let chart3 = LineChart::new()
        .add_series(series3)
        .with_title("Week 3 Sales");

    assert_eq!(chart1.series_count(), 1);
    assert_eq!(chart2.series_count(), 1);
    assert_eq!(chart3.series_count(), 1);
}

#[test]
fn test_scenario_interactive_data_entry() {
    // Scenario: Interactive form with multiple inputs and confirmation

    let mut name_input = InputField::new().with_placeholder("Name");
    let mut email_input = InputField::new().with_placeholder("Email");
    let mut age_input = InputField::new().with_placeholder("Age");

    // Fill in form
    name_input.set_value("Alice".to_string());
    email_input.set_value("alice@example.com".to_string());
    age_input.set_value("25".to_string());

    assert_eq!(name_input.value(), "Alice");
    assert_eq!(email_input.value(), "alice@example.com");
    assert_eq!(age_input.value(), "25");

    // Confirm submission
    let mut dialog = ConfirmDialog::new("Confirm Registration")
        .message("Name: Alice")
        .message("Email: alice@example.com")
        .message("Age: 25")
        .info_box("Please review your information before submitting")
        .option('s', "Submit")
        .option('e', "Edit")
        .option('c', "Cancel");

    dialog.select_by_key('s');
    assert_eq!(dialog.selected(), 0);
}
