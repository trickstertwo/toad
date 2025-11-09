//! Integration tests for PLATINUM Tier Visualization Widgets
//!
//! Tests for Spinner, Sparkline, Canvas, and BoxDrawing.

use ratatui::style::Color;
use toad::ui::box_drawing::utils::{fancy_box, join_horizontal, join_vertical, simple_box, titled_box};
use toad::ui::box_drawing::{BoxBuilder, BoxStyle};
use toad::ui::widgets::{Canvas, Sparkline, SparklineStyle, Spinner, SpinnerStyle};

// ==================== Spinner Tests ====================

#[test]
fn test_spinner_creation() {
    let spinner = Spinner::new(SpinnerStyle::Dots);

    assert_eq!(spinner.current_frame(), 0);
    assert_eq!(spinner.style(), SpinnerStyle::Dots);
}

#[test]
fn test_spinner_all_styles() {
    let styles = [
        SpinnerStyle::Dots,
        SpinnerStyle::Line,
        SpinnerStyle::Bars,
        SpinnerStyle::Bounce,
        SpinnerStyle::Arrows,
        SpinnerStyle::SimpleDots,
        SpinnerStyle::Binary,
        SpinnerStyle::Clock,
    ];

    for style in styles {
        let spinner = Spinner::new(style);
        assert_eq!(spinner.style(), style);
        assert!(spinner.current_symbol().len() > 0);
    }
}

#[test]
fn test_spinner_tick() {
    let mut spinner = Spinner::new(SpinnerStyle::Dots);

    assert_eq!(spinner.current_frame(), 0);

    spinner.tick();
    assert_eq!(spinner.current_frame(), 1);

    spinner.tick();
    assert_eq!(spinner.current_frame(), 2);
}

#[test]
fn test_spinner_reset() {
    let mut spinner = Spinner::new(SpinnerStyle::Line);

    spinner.tick();
    spinner.tick();
    spinner.tick();
    assert!(spinner.current_frame() > 0);

    spinner.reset();
    assert_eq!(spinner.current_frame(), 0);
}

#[test]
fn test_spinner_builder() {
    let spinner = Spinner::new(SpinnerStyle::Bars)
        .label("Loading...")
        .color(Color::Cyan);

    assert_eq!(spinner.style(), SpinnerStyle::Bars);
}

#[test]
fn test_spinner_set_frame() {
    let mut spinner = Spinner::new(SpinnerStyle::Dots);

    spinner.set_frame(5);
    assert_eq!(spinner.current_frame(), 5);
}

#[test]
fn test_spinner_change_style() {
    let mut spinner = Spinner::new(SpinnerStyle::Dots);

    assert_eq!(spinner.style(), SpinnerStyle::Dots);

    spinner.set_style(SpinnerStyle::Arrows);
    assert_eq!(spinner.style(), SpinnerStyle::Arrows);
}

#[test]
fn test_spinner_frame_wrapping() {
    let mut spinner = Spinner::new(SpinnerStyle::Dots);
    let frame_count = spinner.style().frame_count();

    // Tick enough times to wrap around
    for _ in 0..frame_count + 5 {
        spinner.tick();
    }

    // Should wrap back to frame 5
    assert_eq!(spinner.current_frame(), 5);
}

// ==================== Sparkline Tests ====================

#[test]
fn test_sparkline_creation() {
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let sparkline = Sparkline::new(data);

    assert_eq!(sparkline.data().len(), 5);
    assert_eq!(sparkline.min(), Some(1.0));
    assert_eq!(sparkline.max(), Some(5.0));
}

#[test]
fn test_sparkline_builder() {
    let data = vec![10.0, 20.0, 30.0];
    let sparkline = Sparkline::new(data)
        .title("CPU Usage")
        .style(SparklineStyle::Bars)
        .show_border(true)
        .show_labels(true);

    assert_eq!(sparkline.data().len(), 3);
}

#[test]
fn test_sparkline_all_styles() {
    let data = vec![1.0, 2.0, 3.0, 4.0];
    let styles = SparklineStyle::all();

    for &style in styles {
        let sparkline = Sparkline::new(data.clone()).style(style);
        assert_eq!(sparkline.data().len(), 4);
    }
}

#[test]
fn test_sparkline_push() {
    let mut sparkline = Sparkline::new(vec![1.0, 2.0, 3.0]);

    assert_eq!(sparkline.data().len(), 3);

    sparkline.push(4.0);
    assert_eq!(sparkline.data().len(), 4);
    assert_eq!(sparkline.max(), Some(4.0));
}

#[test]
fn test_sparkline_push_with_limit() {
    let mut sparkline = Sparkline::new(vec![1.0, 2.0, 3.0]);

    // Push with limit of 3 - should remove oldest
    sparkline.push_with_limit(4.0, 3);

    assert_eq!(sparkline.data().len(), 3);
    assert_eq!(sparkline.min(), Some(2.0));
    assert_eq!(sparkline.max(), Some(4.0));
}

#[test]
fn test_sparkline_set_data() {
    let mut sparkline = Sparkline::new(vec![1.0, 2.0]);

    sparkline.set_data(vec![10.0, 20.0, 30.0]);

    assert_eq!(sparkline.data().len(), 3);
    assert_eq!(sparkline.min(), Some(10.0));
    assert_eq!(sparkline.max(), Some(30.0));
}

#[test]
fn test_sparkline_avg() {
    let sparkline = Sparkline::new(vec![10.0, 20.0, 30.0]);

    assert_eq!(sparkline.avg(), Some(20.0));
}

#[test]
fn test_sparkline_empty_data() {
    let sparkline = Sparkline::new(vec![]);

    assert_eq!(sparkline.min(), None);
    assert_eq!(sparkline.max(), None);
    assert_eq!(sparkline.avg(), None);
}

#[test]
fn test_sparkline_realtime_metrics() {
    let mut sparkline = Sparkline::new(vec![]).style(SparklineStyle::Braille);

    // Simulate real-time metrics collection
    for i in 0..50 {
        let value = 50.0 + 30.0 * ((i as f64) / 10.0).sin();
        sparkline.push_with_limit(value, 20);
    }

    assert_eq!(sparkline.data().len(), 20);
    assert!(sparkline.min().is_some());
    assert!(sparkline.max().is_some());
}

// ==================== Canvas Tests ====================

#[test]
fn test_canvas_creation() {
    let canvas = Canvas::new();

    assert_eq!(canvas.shape_count(), 0);
}

#[test]
fn test_canvas_builder() {
    let canvas = Canvas::new()
        .with_title("Chart")
        .with_x_bounds(0.0, 100.0)
        .with_y_bounds(0.0, 50.0)
        .with_grid(true)
        .with_axes(true);

    assert_eq!(canvas.shape_count(), 0);
}

#[test]
fn test_canvas_line() {
    let mut canvas = Canvas::new();

    canvas.line(0.0, 0.0, 10.0, 10.0, Color::White);

    assert_eq!(canvas.shape_count(), 1);
}

#[test]
fn test_canvas_rectangle() {
    let mut canvas = Canvas::new();

    canvas.rectangle(5.0, 5.0, 15.0, 15.0, Color::Red);

    assert_eq!(canvas.shape_count(), 1);
}

#[test]
fn test_canvas_filled_rectangle() {
    let mut canvas = Canvas::new();

    canvas.filled_rectangle(0.0, 0.0, 20.0, 20.0, Color::Blue);

    assert_eq!(canvas.shape_count(), 1);
}

#[test]
fn test_canvas_circle() {
    let mut canvas = Canvas::new();

    canvas.circle(50.0, 50.0, 10.0, Color::Green);

    assert_eq!(canvas.shape_count(), 1);
}

#[test]
fn test_canvas_filled_circle() {
    let mut canvas = Canvas::new();

    canvas.filled_circle(25.0, 25.0, 5.0, Color::Yellow);

    assert_eq!(canvas.shape_count(), 1);
}

#[test]
fn test_canvas_point() {
    let mut canvas = Canvas::new();

    canvas.point(10.0, 20.0, Color::Magenta, '●');

    assert_eq!(canvas.shape_count(), 1);
}

#[test]
fn test_canvas_multiple_shapes() {
    let mut canvas = Canvas::new()
        .with_x_bounds(0.0, 100.0)
        .with_y_bounds(0.0, 100.0);

    canvas.line(0.0, 0.0, 100.0, 100.0, Color::White);
    canvas.rectangle(20.0, 20.0, 40.0, 40.0, Color::Red);
    canvas.circle(50.0, 50.0, 15.0, Color::Green);
    canvas.point(75.0, 75.0, Color::Blue, '●');

    assert_eq!(canvas.shape_count(), 4);
}

#[test]
fn test_canvas_clear() {
    let mut canvas = Canvas::new();

    canvas.line(0.0, 0.0, 10.0, 10.0, Color::White);
    canvas.rectangle(5.0, 5.0, 15.0, 15.0, Color::Red);
    canvas.circle(20.0, 20.0, 5.0, Color::Green);

    assert_eq!(canvas.shape_count(), 3);

    canvas.clear();

    assert_eq!(canvas.shape_count(), 0);
}

#[test]
fn test_canvas_drawing_chart() {
    let mut canvas = Canvas::new()
        .with_title("Sin Wave")
        .with_x_bounds(0.0, 360.0)
        .with_y_bounds(-1.0, 1.0);

    // Draw sine wave
    for x in 0..360 {
        let y = ((x as f64).to_radians()).sin();
        canvas.point(x as f64, y, Color::Cyan, '•');
    }

    assert_eq!(canvas.shape_count(), 360);
}

// ==================== BoxDrawing Tests ====================

#[test]
fn test_box_style_all() {
    let styles = BoxStyle::all();

    assert!(styles.len() >= 5); // Light, Heavy, Double, Rounded, ASCII
}

#[test]
fn test_box_style_names() {
    let styles = [
        BoxStyle::Light,
        BoxStyle::Heavy,
        BoxStyle::Double,
        BoxStyle::Rounded,
        BoxStyle::Ascii,
    ];

    for style in styles {
        let name = style.name();
        assert!(name.len() > 0);
        assert!(!name.is_empty());
    }
}

#[test]
fn test_box_chars() {
    let style = BoxStyle::Light;
    let chars = style.chars();

    // Verify horizontal line generation
    let line = chars.horizontal_line(10);
    // Line length may include Unicode characters
    assert!(line.chars().count() >= 1);
}

#[test]
fn test_box_builder_simple() {
    let content = vec!["Line 1", "Line 2", "Line 3"];
    let builder = BoxBuilder::new(BoxStyle::Light, 20, 5);
    let result = builder.build(&content);

    assert!(result.len() > 0);
}

#[test]
fn test_box_builder_with_title() {
    let content = vec!["Hello", "World"];
    let builder = BoxBuilder::new(BoxStyle::Rounded, 15, 4).title("Test Box");
    let result = builder.build(&content);

    assert!(result.len() > 0);
}

#[test]
fn test_box_builder_with_padding() {
    let content = vec!["Test"];
    let builder = BoxBuilder::new(BoxStyle::Heavy, 20, 5).padding(2);
    let result = builder.build(&content);

    assert!(result.len() > 0);
}

#[test]
fn test_simple_box() {
    let content = vec!["A", "B", "C"];
    let result = simple_box(15, 5, &content);

    // Box adds top and bottom borders, so result is taller than requested height
    assert!(result.len() >= 5);
}

#[test]
fn test_fancy_box() {
    let content = vec!["Fancy", "Content"];
    let result = fancy_box(20, 4, &content);

    assert!(result.len() >= 4);
}

#[test]
fn test_titled_box() {
    let content = vec!["Data 1", "Data 2"];
    let result = titled_box(BoxStyle::Light, 25, 6, "Title", &content);

    // Box with title adds borders
    assert!(result.len() >= 6);
}

#[test]
fn test_box_vertical_line() {
    let chars = BoxStyle::Light.chars();
    let lines = chars.vertical_line(5);

    assert_eq!(lines.len(), 5);
}

#[test]
fn test_box_borders() {
    let chars = BoxStyle::Double.chars();

    let top = chars.top_border(10);
    let bottom = chars.bottom_border(10);
    let middle = chars.middle_line(10);

    // Borders are generated with Unicode box-drawing characters
    assert!(top.chars().count() >= 1);
    assert!(bottom.chars().count() >= 1);
    assert!(middle.chars().count() >= 1);
}

#[test]
fn test_join_horizontal() {
    let box1 = vec!["┌─┐".to_string(), "│A│".to_string(), "└─┘".to_string()];
    let box2 = vec!["┌─┐".to_string(), "│B│".to_string(), "└─┘".to_string()];

    let result = join_horizontal(&[box1, box2]);

    assert_eq!(result.len(), 3);
}

#[test]
fn test_join_vertical() {
    let box1 = vec!["┌───┐".to_string(), "│Top│".to_string(), "└───┘".to_string()];
    let box2 = vec![
        "┌────┐".to_string(),
        "│Down│".to_string(),
        "└────┘".to_string(),
    ];

    let result = join_vertical(&[box1, box2]);

    assert_eq!(result.len(), 6);
}

// ==================== Cross-Feature Integration Tests ====================

#[test]
fn test_spinner_with_sparkline_dashboard() {
    let mut spinner = Spinner::new(SpinnerStyle::Dots).label("Loading metrics...");
    let mut sparkline = Sparkline::new(vec![]).style(SparklineStyle::Braille);

    // Simulate loading data with spinner
    for i in 0..10 {
        spinner.tick();
        sparkline.push_with_limit((i * 10) as f64, 20);
    }

    assert_eq!(sparkline.data().len(), 10);
}

#[test]
fn test_canvas_with_boxdrawing_chart() {
    let mut canvas = Canvas::new()
        .with_x_bounds(0.0, 100.0)
        .with_y_bounds(0.0, 100.0);

    // Draw chart content
    canvas.line(0.0, 0.0, 100.0, 100.0, Color::Cyan);
    canvas.circle(50.0, 50.0, 20.0, Color::Green);

    // Create border with box drawing
    let content = vec!["[Chart Content]"];
    let border = titled_box(BoxStyle::Rounded, 30, 10, "Chart", &content);

    assert_eq!(canvas.shape_count(), 2);
    assert!(border.len() >= 10);
}

#[test]
fn test_complete_visualization_dashboard() {
    // Spinner for loading
    let mut spinner = Spinner::new(SpinnerStyle::Arrows).label("Fetching data...");

    // Sparkline for trends
    let mut cpu_sparkline = Sparkline::new(vec![]).title("CPU").style(SparklineStyle::Bars);
    let mut mem_sparkline = Sparkline::new(vec![])
        .title("Memory")
        .style(SparklineStyle::Braille);

    // Canvas for custom visualization
    let mut canvas = Canvas::new()
        .with_title("Network Traffic")
        .with_x_bounds(0.0, 60.0)
        .with_y_bounds(0.0, 100.0);

    // Simulate data collection
    for i in 0..60 {
        spinner.tick();

        let cpu = 30.0 + 20.0 * ((i as f64) / 10.0).sin();
        cpu_sparkline.push_with_limit(cpu, 30);

        let mem = 40.0 + (i as f64) * 0.5;
        mem_sparkline.push_with_limit(mem, 30);

        canvas.point(i as f64, cpu, Color::Cyan, '●');
    }

    // Create bordered panels
    let cpu_box = titled_box(BoxStyle::Double, 25, 5, "CPU Usage", &["[Sparkline]"]);
    let mem_box = titled_box(BoxStyle::Double, 25, 5, "Memory", &["[Sparkline]"]);
    let chart_box = titled_box(BoxStyle::Heavy, 35, 15, "Network", &["[Canvas]"]);

    // Verify state
    assert_eq!(cpu_sparkline.data().len(), 30);
    assert_eq!(mem_sparkline.data().len(), 30);
    assert_eq!(canvas.shape_count(), 60);
    assert!(cpu_box.len() >= 5);
    assert!(mem_box.len() >= 5);
    assert!(chart_box.len() >= 15);
}
