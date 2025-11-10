//\! Vector canvas tests

use super::*;
use ratatui::style::Color;

#[test]
fn test_canvas_new() {
    let canvas = Canvas::new();
    assert_eq!(canvas.shape_count(), 0);
    assert_eq!(canvas.x_bounds, (0.0, 100.0));
    assert_eq!(canvas.y_bounds, (0.0, 100.0));
    assert!(canvas.show_axes);
    assert!(!canvas.show_grid);
}

#[test]
fn test_canvas_default() {
    let canvas = Canvas::default();
    assert_eq!(canvas.shape_count(), 0);
}

#[test]
fn test_canvas_with_title() {
    let canvas = Canvas::new().with_title("Test");
    assert_eq!(canvas.title, Some("Test".to_string()));
}

#[test]
fn test_canvas_with_x_bounds() {
    let canvas = Canvas::new().with_x_bounds(-10.0, 10.0);
    assert_eq!(canvas.x_bounds, (-10.0, 10.0));
}

#[test]
fn test_canvas_with_y_bounds() {
    let canvas = Canvas::new().with_y_bounds(-5.0, 5.0);
    assert_eq!(canvas.y_bounds, (-5.0, 5.0));
}

#[test]
fn test_canvas_with_grid() {
    let canvas = Canvas::new().with_grid(true);
    assert!(canvas.show_grid);
}

#[test]
fn test_canvas_with_axes() {
    let canvas = Canvas::new().with_axes(false);
    assert!(!canvas.show_axes);
}

#[test]
fn test_canvas_line() {
    let mut canvas = Canvas::new();
    canvas.line(0.0, 0.0, 10.0, 10.0, Color::Red);
    assert_eq!(canvas.shape_count(), 1);
}

#[test]
fn test_canvas_rectangle() {
    let mut canvas = Canvas::new();
    canvas.rectangle(5.0, 5.0, 15.0, 15.0, Color::Blue);
    assert_eq!(canvas.shape_count(), 1);
}

#[test]
fn test_canvas_filled_rectangle() {
    let mut canvas = Canvas::new();
    canvas.filled_rectangle(5.0, 5.0, 15.0, 15.0, Color::Green);
    assert_eq!(canvas.shape_count(), 1);
}

#[test]
fn test_canvas_circle() {
    let mut canvas = Canvas::new();
    canvas.circle(50.0, 50.0, 10.0, Color::Yellow);
    assert_eq!(canvas.shape_count(), 1);
}

#[test]
fn test_canvas_filled_circle() {
    let mut canvas = Canvas::new();
    canvas.filled_circle(50.0, 50.0, 10.0, Color::Magenta);
    assert_eq!(canvas.shape_count(), 1);
}

#[test]
fn test_canvas_point() {
    let mut canvas = Canvas::new();
    canvas.point(25.0, 25.0, Color::White, '●');
    assert_eq!(canvas.shape_count(), 1);
}

#[test]
fn test_canvas_clear() {
    let mut canvas = Canvas::new();
    canvas.line(0.0, 0.0, 10.0, 10.0, Color::Red);
    canvas.circle(50.0, 50.0, 10.0, Color::Blue);
    assert_eq!(canvas.shape_count(), 2);
    canvas.clear();
    assert_eq!(canvas.shape_count(), 0);
}

#[test]
fn test_canvas_multiple_shapes() {
    let mut canvas = Canvas::new();
    canvas.line(0.0, 0.0, 10.0, 10.0, Color::Red);
    canvas.rectangle(20.0, 20.0, 40.0, 40.0, Color::Blue);
    canvas.circle(50.0, 50.0, 5.0, Color::Green);
    canvas.point(75.0, 75.0, Color::White, '●');
    assert_eq!(canvas.shape_count(), 4);
}

#[test]
fn test_world_to_screen() {
    let canvas = Canvas::new()
        .with_x_bounds(0.0, 100.0)
        .with_y_bounds(0.0, 100.0);

    let (x, y) = canvas.world_to_screen(50.0, 50.0, 100, 100);
    assert_eq!(x, 50);
    assert_eq!(y, 50);
}

#[test]
fn test_world_to_screen_negative() {
    let canvas = Canvas::new()
        .with_x_bounds(-10.0, 10.0)
        .with_y_bounds(-10.0, 10.0);

    let (x, y) = canvas.world_to_screen(0.0, 0.0, 100, 100);
    assert_eq!(x, 50);
    assert_eq!(y, 50);
}

#[test]
fn test_render_lines_empty() {
    let canvas = Canvas::new();
    let lines = canvas.render_lines(40, 20);
    assert!(!lines.is_empty());
}

#[test]
fn test_render_lines_with_shapes() {
    let mut canvas = Canvas::new();
    canvas.line(10.0, 10.0, 90.0, 90.0, Color::Red);
    let lines = canvas.render_lines(40, 20);
    assert!(!lines.is_empty());
}

#[test]
fn test_render_lines_with_title() {
    let canvas = Canvas::new().with_title("Test Canvas");
    let lines = canvas.render_lines(40, 20);
    assert!(!lines.is_empty());
}

#[test]
fn test_builder_pattern() {
    let mut canvas = Canvas::new()
        .with_title("Chart")
        .with_x_bounds(-10.0, 10.0)
        .with_y_bounds(-5.0, 5.0)
        .with_grid(true)
        .with_axes(true);

    canvas.line(-5.0, 0.0, 5.0, 0.0, Color::Red);
    canvas.circle(0.0, 0.0, 3.0, Color::Blue);

    assert_eq!(canvas.title, Some("Chart".to_string()));
    assert_eq!(canvas.x_bounds, (-10.0, 10.0));
    assert_eq!(canvas.y_bounds, (-5.0, 5.0));
    assert!(canvas.show_grid);
    assert!(canvas.show_axes);
    assert_eq!(canvas.shape_count(), 2);
}

// ============ COMPREHENSIVE EDGE CASE TESTS ============

#[test]
fn test_canvas_with_very_long_title() {
    let long_title = "A".repeat(10000);
    let canvas = Canvas::new().with_title(long_title.clone());
    assert_eq!(canvas.title, Some(long_title));
}

#[test]
fn test_canvas_with_unicode_title() {
    let canvas = Canvas::new().with_title("🎨 キャンバス 🖌️");
    assert!(canvas.title.clone().unwrap().contains("🎨"));
    assert!(canvas.title.clone().unwrap().contains("キャンバス"));
}

#[test]
fn test_canvas_with_empty_title() {
    let canvas = Canvas::new().with_title("");
    assert_eq!(canvas.title, Some("".to_string()));
}

#[test]
fn test_canvas_with_extreme_x_bounds() {
    let canvas = Canvas::new().with_x_bounds(f64::MIN, f64::MAX);
    assert_eq!(canvas.x_bounds, (f64::MIN, f64::MAX));
}

#[test]
fn test_canvas_with_extreme_y_bounds() {
    let canvas = Canvas::new().with_y_bounds(f64::MIN, f64::MAX);
    assert_eq!(canvas.y_bounds, (f64::MIN, f64::MAX));
}

#[test]
fn test_canvas_with_negative_bounds() {
    let canvas = Canvas::new()
        .with_x_bounds(-1000.0, -500.0)
        .with_y_bounds(-800.0, -200.0);
    assert_eq!(canvas.x_bounds, (-1000.0, -500.0));
    assert_eq!(canvas.y_bounds, (-800.0, -200.0));
}

#[test]
fn test_canvas_with_zero_sized_bounds() {
    let canvas = Canvas::new()
        .with_x_bounds(5.0, 5.0)
        .with_y_bounds(10.0, 10.0);
    assert_eq!(canvas.x_bounds, (5.0, 5.0));
    assert_eq!(canvas.y_bounds, (10.0, 10.0));
}

#[test]
fn test_canvas_with_inverted_bounds() {
    let canvas = Canvas::new()
        .with_x_bounds(100.0, 0.0)
        .with_y_bounds(100.0, 0.0);
    assert_eq!(canvas.x_bounds, (100.0, 0.0));
    assert_eq!(canvas.y_bounds, (100.0, 0.0));
}

#[test]
fn test_canvas_with_many_shapes() {
    let mut canvas = Canvas::new();
    for i in 0..1000 {
        let pos = i as f64;
        canvas.point(pos, pos, Color::White, '●');
    }
    assert_eq!(canvas.shape_count(), 1000);
}

#[test]
fn test_canvas_line_with_same_endpoints() {
    let mut canvas = Canvas::new();
    canvas.line(50.0, 50.0, 50.0, 50.0, Color::Red);
    assert_eq!(canvas.shape_count(), 1);
}

#[test]
fn test_canvas_line_with_extreme_coords() {
    let mut canvas = Canvas::new();
    canvas.line(f64::MIN, f64::MIN, f64::MAX, f64::MAX, Color::Blue);
    assert_eq!(canvas.shape_count(), 1);
}

#[test]
fn test_canvas_line_with_negative_coords() {
    let mut canvas = Canvas::new();
    canvas.line(-100.0, -100.0, -50.0, -50.0, Color::Green);
    assert_eq!(canvas.shape_count(), 1);
}

#[test]
fn test_canvas_rectangle_with_same_corners() {
    let mut canvas = Canvas::new();
    canvas.rectangle(50.0, 50.0, 50.0, 50.0, Color::Red);
    assert_eq!(canvas.shape_count(), 1);
}

#[test]
fn test_canvas_rectangle_with_inverted_corners() {
    let mut canvas = Canvas::new();
    canvas.rectangle(90.0, 90.0, 10.0, 10.0, Color::Blue);
    assert_eq!(canvas.shape_count(), 1);
}

#[test]
fn test_canvas_filled_rectangle_with_extreme_coords() {
    let mut canvas = Canvas::new();
    canvas.filled_rectangle(f64::MIN, f64::MIN, f64::MAX, f64::MAX, Color::Yellow);
    assert_eq!(canvas.shape_count(), 1);
}

#[test]
fn test_canvas_circle_with_zero_radius() {
    let mut canvas = Canvas::new();
    canvas.circle(50.0, 50.0, 0.0, Color::Red);
    assert_eq!(canvas.shape_count(), 1);
}

#[test]
fn test_canvas_circle_with_very_large_radius() {
    let mut canvas = Canvas::new();
    canvas.circle(50.0, 50.0, 10000.0, Color::Blue);
    assert_eq!(canvas.shape_count(), 1);
}

#[test]
fn test_canvas_circle_with_extreme_radius() {
    let mut canvas = Canvas::new();
    canvas.circle(0.0, 0.0, f64::MAX, Color::Green);
    assert_eq!(canvas.shape_count(), 1);
}

#[test]
fn test_canvas_filled_circle_with_negative_center() {
    let mut canvas = Canvas::new();
    canvas.filled_circle(-50.0, -50.0, 10.0, Color::Magenta);
    assert_eq!(canvas.shape_count(), 1);
}

#[test]
fn test_canvas_point_with_unicode_marker() {
    let mut canvas = Canvas::new();
    canvas.point(25.0, 25.0, Color::White, '✕');
    canvas.point(50.0, 50.0, Color::Red, '★');
    canvas.point(75.0, 75.0, Color::Blue, '🔴');
    assert_eq!(canvas.shape_count(), 3);
}

#[test]
fn test_canvas_point_with_extreme_coords() {
    let mut canvas = Canvas::new();
    canvas.point(f64::MAX, f64::MAX, Color::White, '●');
    canvas.point(f64::MIN, f64::MIN, Color::Black, '×');
    assert_eq!(canvas.shape_count(), 2);
}

#[test]
fn test_canvas_clear_after_many_shapes() {
    let mut canvas = Canvas::new();
    for i in 0..100 {
        canvas.line(i as f64, 0.0, i as f64, 100.0, Color::Red);
    }
    assert_eq!(canvas.shape_count(), 100);
    canvas.clear();
    assert_eq!(canvas.shape_count(), 0);
}

#[test]
fn test_canvas_all_shape_types() {
    let mut canvas = Canvas::new();
    canvas.line(10.0, 10.0, 90.0, 10.0, Color::Red);
    canvas.rectangle(10.0, 20.0, 90.0, 40.0, Color::Blue);
    canvas.filled_rectangle(10.0, 50.0, 90.0, 60.0, Color::Green);
    canvas.circle(50.0, 75.0, 10.0, Color::Yellow);
    canvas.filled_circle(50.0, 90.0, 5.0, Color::Magenta);
    canvas.point(50.0, 95.0, Color::White, '●');
    assert_eq!(canvas.shape_count(), 6);
}

#[test]
fn test_canvas_render_with_zero_dimensions() {
    let canvas = Canvas::new();
    let _lines = canvas.render_lines(0, 0);
    // Just verify it doesn't crash
}

#[test]
fn test_canvas_render_with_very_small_dimensions() {
    let mut canvas = Canvas::new();
    canvas.line(0.0, 0.0, 100.0, 100.0, Color::Red);
    let lines = canvas.render_lines(1, 1);
    assert!(!lines.is_empty());
}

#[test]
fn test_canvas_render_with_very_large_dimensions() {
    let mut canvas = Canvas::new();
    canvas.circle(50.0, 50.0, 25.0, Color::Blue);
    let lines = canvas.render_lines(1000, 1000);
    assert!(!lines.is_empty());
}

#[test]
fn test_canvas_render_with_grid_enabled() {
    let mut canvas = Canvas::new().with_grid(true);
    canvas.line(10.0, 10.0, 90.0, 90.0, Color::Red);
    let lines = canvas.render_lines(100, 100);
    assert!(!lines.is_empty());
}

#[test]
fn test_canvas_render_with_axes_disabled() {
    let mut canvas = Canvas::new().with_axes(false);
    canvas.circle(50.0, 50.0, 10.0, Color::Blue);
    let lines = canvas.render_lines(40, 20);
    assert!(!lines.is_empty());
}

#[test]
fn test_canvas_render_with_grid_and_no_axes() {
    let mut canvas = Canvas::new().with_grid(true).with_axes(false);
    canvas.rectangle(20.0, 20.0, 80.0, 80.0, Color::Green);
    let lines = canvas.render_lines(50, 50);
    assert!(!lines.is_empty());
}

#[test]
fn test_world_to_screen_with_zero_range() {
    let canvas = Canvas::new()
        .with_x_bounds(50.0, 50.0)
        .with_y_bounds(50.0, 50.0);
    let (_x, _y) = canvas.world_to_screen(50.0, 50.0, 100, 100);
    // Just verify it doesn't crash with divide by zero
}

#[test]
fn test_world_to_screen_with_extreme_bounds() {
    let canvas = Canvas::new()
        .with_x_bounds(f64::MIN, f64::MAX)
        .with_y_bounds(f64::MIN, f64::MAX);
    let (_x, _y) = canvas.world_to_screen(0.0, 0.0, 100, 100);
    // Just verify it doesn't crash
}

#[test]
fn test_canvas_clone() {
    let mut original = Canvas::new()
        .with_title("Original")
        .with_x_bounds(-10.0, 10.0)
        .with_y_bounds(-5.0, 5.0)
        .with_grid(true);
    original.line(0.0, 0.0, 5.0, 5.0, Color::Red);

    let cloned = original.clone();
    assert_eq!(original.title, cloned.title);
    assert_eq!(original.x_bounds, cloned.x_bounds);
    assert_eq!(original.y_bounds, cloned.y_bounds);
    assert_eq!(original.show_grid, cloned.show_grid);
    assert_eq!(original.shape_count(), cloned.shape_count());
}

#[test]
fn test_canvas_builder_pattern_chaining_complete() {
    let mut canvas = Canvas::new()
        .with_title("Complete Test")
        .with_x_bounds(-100.0, 100.0)
        .with_y_bounds(-50.0, 50.0)
        .with_grid(true)
        .with_axes(true);

    canvas.line(-90.0, 0.0, 90.0, 0.0, Color::Red);
    canvas.rectangle(-50.0, -25.0, 50.0, 25.0, Color::Blue);
    canvas.filled_circle(0.0, 0.0, 10.0, Color::Green);
    canvas.point(0.0, 0.0, Color::White, '●');

    assert_eq!(canvas.title, Some("Complete Test".to_string()));
    assert_eq!(canvas.x_bounds, (-100.0, 100.0));
    assert_eq!(canvas.y_bounds, (-50.0, 50.0));
    assert!(canvas.show_grid);
    assert!(canvas.show_axes);
    assert_eq!(canvas.shape_count(), 4);
}

#[test]
fn test_canvas_multiple_title_calls() {
    let canvas = Canvas::new()
        .with_title("First")
        .with_title("Second")
        .with_title("Third");
    assert_eq!(canvas.title, Some("Third".to_string()));
}

#[test]
fn test_canvas_multiple_bounds_calls() {
    let canvas = Canvas::new()
        .with_x_bounds(0.0, 100.0)
        .with_x_bounds(50.0, 150.0)
        .with_y_bounds(0.0, 50.0)
        .with_y_bounds(25.0, 75.0);
    assert_eq!(canvas.x_bounds, (50.0, 150.0));
    assert_eq!(canvas.y_bounds, (25.0, 75.0));
}

#[test]
fn test_canvas_grid_toggle() {
    let canvas1 = Canvas::new().with_grid(true);
    let canvas2 = Canvas::new().with_grid(false);
    assert!(canvas1.show_grid);
    assert!(!canvas2.show_grid);
}

#[test]
fn test_canvas_axes_toggle() {
    let canvas1 = Canvas::new().with_axes(true);
    let canvas2 = Canvas::new().with_axes(false);
    assert!(canvas1.show_axes);
    assert!(!canvas2.show_axes);
}

#[test]
fn test_canvas_render_with_unicode_title() {
    let canvas = Canvas::new().with_title("🎨 Drawing 🖌️");
    let lines = canvas.render_lines(40, 20);
    assert!(!lines.is_empty());
}

#[test]
fn test_canvas_shapes_with_all_features() {
    let mut canvas = Canvas::new()
        .with_title("📊 Complete Canvas Test")
        .with_x_bounds(-10.0, 10.0)
        .with_y_bounds(-10.0, 10.0)
        .with_grid(true)
        .with_axes(true);

    canvas.line(-5.0, 0.0, 5.0, 0.0, Color::Red);
    canvas.line(0.0, -5.0, 0.0, 5.0, Color::Blue);
    canvas.rectangle(-3.0, -3.0, 3.0, 3.0, Color::Green);
    canvas.filled_rectangle(-1.0, -1.0, 1.0, 1.0, Color::Yellow);
    canvas.circle(0.0, 0.0, 7.0, Color::Magenta);
    canvas.filled_circle(0.0, 0.0, 2.0, Color::Cyan);
    canvas.point(0.0, 0.0, Color::White, '×');

    assert_eq!(canvas.shape_count(), 7);
    let lines = canvas.render_lines(80, 80);
    assert!(!lines.is_empty());
}

#[test]
fn test_shape_enum_clone() {
    let line = Shape::Line {
        x1: 0.0,
        y1: 0.0,
        x2: 10.0,
        y2: 10.0,
        color: Color::Red,
    };
    let _cloned = line.clone();
}

#[test]
fn test_canvas_fractional_coordinates() {
    let mut canvas = Canvas::new();
    canvas.line(
        0.123456789,
        0.987654321,
        3.141592653,
        2.718281828,
        Color::Red,
    );
    canvas.circle(1.414213562, 1.732050808, 0.618033989, Color::Blue);
    canvas.point(2.236067977, 1.618033989, Color::Green, '●');
    assert_eq!(canvas.shape_count(), 3);
}

#[test]
fn test_canvas_default_configuration() {
    let canvas = Canvas::default();
    assert_eq!(canvas.x_bounds, (0.0, 100.0));
    assert_eq!(canvas.y_bounds, (0.0, 100.0));
    assert!(!canvas.show_grid);
    assert!(canvas.show_axes);
    assert_eq!(canvas.title, None);
    assert_eq!(canvas.shape_count(), 0);
}
