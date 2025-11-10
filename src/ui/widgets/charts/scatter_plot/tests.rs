//! Scatter plot widget tests

use super::*;
use ratatui::style::Color;

    #[test]
fn test_scatter_series_new() {
    let points = vec![(1.0, 2.0), (3.0, 4.0)];
    let series = ScatterSeries::new("Test", points);
    assert_eq!(series.name, "Test");
    assert_eq!(series.points.len(), 2);
    assert_eq!(series.color, Color::Cyan);
    assert_eq!(series.marker, '●');
}

#[test]
fn test_scatter_series_with_color() {
    let series = ScatterSeries::new("Test", vec![(1.0, 2.0)]).with_color(Color::Red);
    assert_eq!(series.color, Color::Red);
}

#[test]
fn test_scatter_series_with_marker() {
    let series = ScatterSeries::new("Test", vec![(1.0, 2.0)]).with_marker('×');
    assert_eq!(series.marker, '×');
}

#[test]
fn test_scatter_series_x_bounds() {
    let series = ScatterSeries::new("Test", vec![(1.0, 2.0), (3.0, 4.0), (2.0, 3.0)]);
    let (min, max) = series.x_bounds();
    assert_eq!(min, Some(1.0));
    assert_eq!(max, Some(3.0));
}

#[test]
fn test_scatter_series_y_bounds() {
    let series = ScatterSeries::new("Test", vec![(1.0, 2.0), (3.0, 4.0), (2.0, 3.0)]);
    let (min, max) = series.y_bounds();
    assert_eq!(min, Some(2.0));
    assert_eq!(max, Some(4.0));
}

#[test]
fn test_scatter_series_empty_bounds() {
    let series = ScatterSeries::new("Test", vec![]);
    let (min_x, max_x) = series.x_bounds();
    let (min_y, max_y) = series.y_bounds();
    assert_eq!(min_x, None);
    assert_eq!(max_x, None);
    assert_eq!(min_y, None);
    assert_eq!(max_y, None);
}

#[test]
fn test_scatter_plot_new() {
    let plot = ScatterPlot::new();
    assert_eq!(plot.series_count(), 0);
    assert!(plot.show_legend);
    assert!(!plot.show_grid);
}

#[test]
fn test_scatter_plot_add_series() {
    let plot = ScatterPlot::new()
        .add_series(ScatterSeries::new("A", vec![(1.0, 2.0)]))
        .add_series(ScatterSeries::new("B", vec![(3.0, 4.0)]));
    assert_eq!(plot.series_count(), 2);
}

#[test]
fn test_scatter_plot_with_title() {
    let plot = ScatterPlot::new().with_title("My Plot");
    assert_eq!(plot.title, Some("My Plot".to_string()));
}

#[test]
fn test_scatter_plot_with_labels() {
    let plot = ScatterPlot::new().with_x_label("X").with_y_label("Y");
    assert_eq!(plot.x_label, Some("X".to_string()));
    assert_eq!(plot.y_label, Some("Y".to_string()));
}

#[test]
fn test_scatter_plot_with_legend() {
    let plot = ScatterPlot::new().with_legend(false);
    assert!(!plot.show_legend);
}

#[test]
fn test_scatter_plot_with_grid() {
    let plot = ScatterPlot::new().with_grid(true);
    assert!(plot.show_grid);
}

#[test]
fn test_scatter_plot_with_bounds() {
    let plot = ScatterPlot::new()
        .with_x_bounds(0.0, 10.0)
        .with_y_bounds(0.0, 100.0);
    assert_eq!(plot.x_bounds, Some((0.0, 10.0)));
    assert_eq!(plot.y_bounds, Some((0.0, 100.0)));
}

#[test]
fn test_scatter_plot_calculate_x_bounds() {
    let plot = ScatterPlot::new()
        .add_series(ScatterSeries::new("A", vec![(1.0, 2.0), (5.0, 3.0)]))
        .add_series(ScatterSeries::new("B", vec![(2.0, 4.0), (3.0, 5.0)]));

    let (min, max) = plot.calculate_x_bounds();
    assert!(min < 1.0); // With padding
    assert!(max > 5.0); // With padding
}

#[test]
fn test_scatter_plot_calculate_y_bounds() {
    let plot = ScatterPlot::new()
        .add_series(ScatterSeries::new("A", vec![(1.0, 2.0), (5.0, 6.0)]))
        .add_series(ScatterSeries::new("B", vec![(2.0, 3.0), (3.0, 4.0)]));

    let (min, max) = plot.calculate_y_bounds();
    assert!(min < 2.0); // With padding
    assert!(max > 6.0); // With padding
}

#[test]
fn test_scatter_plot_manual_bounds() {
    let plot = ScatterPlot::new()
        .with_x_bounds(0.0, 10.0)
        .with_y_bounds(0.0, 100.0)
        .add_series(ScatterSeries::new("A", vec![(1.0, 2.0)]));

    let (x_min, x_max) = plot.calculate_x_bounds();
    let (y_min, y_max) = plot.calculate_y_bounds();

    assert_eq!(x_min, 0.0);
    assert_eq!(x_max, 10.0);
    assert_eq!(y_min, 0.0);
    assert_eq!(y_max, 100.0);
}

#[test]
fn test_scatter_plot_empty_bounds() {
    let plot = ScatterPlot::new();
    let (x_min, x_max) = plot.calculate_x_bounds();
    let (y_min, y_max) = plot.calculate_y_bounds();

    assert_eq!(x_min, 0.0);
    assert_eq!(x_max, 1.0);
    assert_eq!(y_min, 0.0);
    assert_eq!(y_max, 1.0);
}

#[test]
fn test_scatter_plot_render_lines() {
    let plot = ScatterPlot::new()
        .with_title("Test Plot")
        .add_series(ScatterSeries::new("Data", vec![(1.0, 2.0), (2.0, 3.0)]));

    let lines = plot.render_lines(40, 20);
    assert!(!lines.is_empty());
}

#[test]
fn test_scatter_plot_render_empty() {
    let plot = ScatterPlot::new();
    let lines = plot.render_lines(40, 20);
    assert!(!lines.is_empty());
}

#[test]
fn test_builder_pattern() {
    let plot = ScatterPlot::new()
        .with_title("Plot")
        .with_x_label("X")
        .with_y_label("Y")
        .with_legend(false)
        .with_grid(true)
        .with_x_bounds(0.0, 10.0)
        .with_y_bounds(0.0, 100.0);

    assert_eq!(plot.title, Some("Plot".to_string()));
    assert_eq!(plot.x_label, Some("X".to_string()));
    assert_eq!(plot.y_label, Some("Y".to_string()));
    assert!(!plot.show_legend);
    assert!(plot.show_grid);
    assert_eq!(plot.x_bounds, Some((0.0, 10.0)));
    assert_eq!(plot.y_bounds, Some((0.0, 100.0)));
}

// ============ COMPREHENSIVE EDGE CASE TESTS ============

#[test]
fn test_scatter_series_with_very_long_name() {
    let long_name = "A".repeat(10000);
    let series = ScatterSeries::new(long_name.clone(), vec![(1.0, 2.0)]);
    assert_eq!(series.name, long_name);
}

#[test]
fn test_scatter_series_with_unicode_name() {
    let series = ScatterSeries::new("📊 相関分析 🎯", vec![(1.0, 2.0)]);
    assert!(series.name.contains("📊"));
    assert!(series.name.contains("相関分析"));
}

#[test]
fn test_scatter_series_with_empty_name() {
    let series = ScatterSeries::new("", vec![(1.0, 2.0)]);
    assert_eq!(series.name, "");
}

#[test]
fn test_scatter_series_with_whitespace_only_name() {
    let series = ScatterSeries::new("     ", vec![(1.0, 2.0)]);
    assert_eq!(series.name, "     ");
}

#[test]
fn test_scatter_series_with_special_characters_name() {
    let series = ScatterSeries::new("Test<>&\"'\\|/*?", vec![(1.0, 2.0)]);
    assert!(series.name.contains("<>"));
}

#[test]
fn test_scatter_series_with_very_many_points() {
    let points: Vec<(f64, f64)> = (0..10000).map(|i| (i as f64, i as f64 * 2.0)).collect();
    let series = ScatterSeries::new("Large", points);
    assert_eq!(series.points.len(), 10000);
}

#[test]
fn test_scatter_series_with_extreme_positive_values() {
    let points = vec![(f64::MAX, f64::MAX)];
    let series = ScatterSeries::new("Max", points);
    let (min_x, max_x) = series.x_bounds();
    let (min_y, max_y) = series.y_bounds();
    assert_eq!(min_x, Some(f64::MAX));
    assert_eq!(max_x, Some(f64::MAX));
    assert_eq!(min_y, Some(f64::MAX));
    assert_eq!(max_y, Some(f64::MAX));
}

#[test]
fn test_scatter_series_with_extreme_negative_values() {
    let points = vec![(f64::MIN, f64::MIN)];
    let series = ScatterSeries::new("Min", points);
    let (min_x, max_x) = series.x_bounds();
    let (min_y, max_y) = series.y_bounds();
    assert_eq!(min_x, Some(f64::MIN));
    assert_eq!(max_x, Some(f64::MIN));
    assert_eq!(min_y, Some(f64::MIN));
    assert_eq!(max_y, Some(f64::MIN));
}

#[test]
fn test_scatter_series_with_negative_points() {
    let points = vec![(-10.0, -20.0), (-5.0, -15.0), (-8.0, -12.0)];
    let series = ScatterSeries::new("Negative", points);
    let (min_x, max_x) = series.x_bounds();
    let (min_y, max_y) = series.y_bounds();
    assert_eq!(min_x, Some(-10.0));
    assert_eq!(max_x, Some(-5.0));
    assert_eq!(min_y, Some(-20.0));
    assert_eq!(max_y, Some(-12.0));
}

#[test]
fn test_scatter_series_with_zero_points() {
    let points = vec![(0.0, 0.0), (0.0, 0.0)];
    let series = ScatterSeries::new("Zeros", points);
    let (min_x, max_x) = series.x_bounds();
    let (min_y, max_y) = series.y_bounds();
    assert_eq!(min_x, Some(0.0));
    assert_eq!(max_x, Some(0.0));
    assert_eq!(min_y, Some(0.0));
    assert_eq!(max_y, Some(0.0));
}

#[test]
fn test_scatter_series_with_mixed_values() {
    let points = vec![(-10.0, 20.0), (5.0, -15.0), (0.0, 0.0)];
    let series = ScatterSeries::new("Mixed", points);
    let (min_x, max_x) = series.x_bounds();
    let (min_y, max_y) = series.y_bounds();
    assert_eq!(min_x, Some(-10.0));
    assert_eq!(max_x, Some(5.0));
    assert_eq!(min_y, Some(-15.0));
    assert_eq!(max_y, Some(20.0));
}

#[test]
fn test_scatter_series_with_fractional_values() {
    let points = vec![
        (0.123456789, 0.987654321),
        (3.141592653, 2.718281828),
    ];
    let series = ScatterSeries::new("Fractional", points);
    let (min_x, max_x) = series.x_bounds();
    assert!((min_x.unwrap() - 0.123456789).abs() < 1e-9);
    assert!((max_x.unwrap() - 3.141592653).abs() < 1e-9);
}

#[test]
fn test_scatter_series_with_unicode_marker() {
    let series = ScatterSeries::new("Test", vec![(1.0, 2.0)])
        .with_marker('✕')
        .with_marker('★')
        .with_marker('🔴');
    assert_eq!(series.marker, '🔴');
}

#[test]
fn test_scatter_series_clone() {
    let original = ScatterSeries::new("Original", vec![(1.0, 2.0), (3.0, 4.0)])
        .with_color(Color::Red)
        .with_marker('×');
    let cloned = original.clone();
    assert_eq!(original.name, cloned.name);
    assert_eq!(original.points.len(), cloned.points.len());
    assert_eq!(original.color, cloned.color);
    assert_eq!(original.marker, cloned.marker);
}

#[test]
fn test_scatter_series_default_marker() {
    let series = ScatterSeries::new("Test", vec![(1.0, 2.0)]);
    assert_eq!(series.marker, '●');
}

#[test]
fn test_scatter_series_default_color() {
    let series = ScatterSeries::new("Test", vec![(1.0, 2.0)]);
    assert_eq!(series.color, Color::Cyan);
}

#[test]
fn test_scatter_plot_with_many_series() {
    let mut plot = ScatterPlot::new();
    for i in 0..50 {
        plot = plot.add_series(ScatterSeries::new(
            format!("Series {}", i),
            vec![(i as f64, i as f64)],
        ));
    }
    assert_eq!(plot.series_count(), 50);
}

#[test]
fn test_scatter_plot_with_single_series() {
    let plot = ScatterPlot::new()
        .add_series(ScatterSeries::new("Only", vec![(1.0, 2.0)]));
    assert_eq!(plot.series_count(), 1);
}

#[test]
fn test_scatter_plot_with_unicode_title() {
    let plot = ScatterPlot::new().with_title("📊 散布図 🎯");
    assert!(plot.title.clone().unwrap().contains("📊"));
    assert!(plot.title.clone().unwrap().contains("散布図"));
}

#[test]
fn test_scatter_plot_with_very_long_title() {
    let long_title = "B".repeat(10000);
    let plot = ScatterPlot::new().with_title(long_title.clone());
    assert_eq!(plot.title, Some(long_title));
}

#[test]
fn test_scatter_plot_with_empty_title() {
    let plot = ScatterPlot::new().with_title("");
    assert_eq!(plot.title, Some("".to_string()));
}

#[test]
fn test_scatter_plot_with_unicode_labels() {
    let plot = ScatterPlot::new()
        .with_x_label("時間 ⏰")
        .with_y_label("値 📈");
    assert!(plot.x_label.clone().unwrap().contains("時間"));
    assert!(plot.y_label.clone().unwrap().contains("値"));
}

#[test]
fn test_scatter_plot_with_very_long_labels() {
    let long_x_label = "X".repeat(1000);
    let long_y_label = "Y".repeat(1000);
    let plot = ScatterPlot::new()
        .with_x_label(long_x_label.clone())
        .with_y_label(long_y_label.clone());
    assert_eq!(plot.x_label, Some(long_x_label));
    assert_eq!(plot.y_label, Some(long_y_label));
}

#[test]
fn test_scatter_plot_with_empty_labels() {
    let plot = ScatterPlot::new()
        .with_x_label("")
        .with_y_label("");
    assert_eq!(plot.x_label, Some("".to_string()));
    assert_eq!(plot.y_label, Some("".to_string()));
}

#[test]
fn test_scatter_plot_clone() {
    let original = ScatterPlot::new()
        .with_title("Title")
        .add_series(ScatterSeries::new("Test", vec![(1.0, 2.0)]))
        .with_grid(true)
        .with_legend(false);
    let cloned = original.clone();
    assert_eq!(original.series_count(), cloned.series_count());
    assert_eq!(original.title, cloned.title);
    assert_eq!(original.show_grid, cloned.show_grid);
    assert_eq!(original.show_legend, cloned.show_legend);
}

#[test]
fn test_scatter_plot_default() {
    let plot = ScatterPlot::default();
    assert_eq!(plot.series_count(), 0);
    assert!(plot.show_legend);
    assert!(!plot.show_grid);
    assert_eq!(plot.title, None);
}

#[test]
fn test_scatter_plot_render_with_zero_dimensions() {
    let plot = ScatterPlot::new()
        .add_series(ScatterSeries::new("Test", vec![(1.0, 2.0)]));
    let _lines = plot.render_lines(0, 0);
    // Just verify it doesn't crash
}

#[test]
fn test_scatter_plot_render_with_very_small_dimensions() {
    let plot = ScatterPlot::new()
        .add_series(ScatterSeries::new("Test", vec![(1.0, 2.0)]));
    let lines = plot.render_lines(1, 1);
    assert!(!lines.is_empty());
}

#[test]
fn test_scatter_plot_render_with_very_large_dimensions() {
    let plot = ScatterPlot::new()
        .add_series(ScatterSeries::new("Test", vec![(1.0, 2.0), (3.0, 4.0)]));
    let lines = plot.render_lines(1000, 1000);
    assert!(!lines.is_empty());
}

#[test]
fn test_scatter_plot_render_with_grid() {
    let plot = ScatterPlot::new()
        .add_series(ScatterSeries::new("Test", vec![(1.0, 2.0)]))
        .with_grid(true);
    let lines = plot.render_lines(40, 20);
    assert!(!lines.is_empty());
}

#[test]
fn test_scatter_plot_render_without_legend() {
    let plot = ScatterPlot::new()
        .add_series(ScatterSeries::new("Test", vec![(1.0, 2.0)]))
        .with_legend(false);
    let lines = plot.render_lines(40, 20);
    assert!(!lines.is_empty());
}

#[test]
fn test_scatter_plot_render_with_many_series() {
    let mut plot = ScatterPlot::new();
    for i in 0..10 {
        plot = plot.add_series(ScatterSeries::new(
            format!("S{}", i),
            vec![(i as f64, i as f64 * 2.0)],
        ));
    }
    let lines = plot.render_lines(80, 40);
    assert!(!lines.is_empty());
}

#[test]
fn test_scatter_plot_render_with_unicode_data() {
    let plot = ScatterPlot::new()
        .with_title("📊 散布図")
        .with_x_label("時間 ⏰")
        .with_y_label("値 📈")
        .add_series(ScatterSeries::new("データ", vec![(1.0, 2.0), (3.0, 4.0)]));
    let lines = plot.render_lines(60, 30);
    assert!(!lines.is_empty());
}

#[test]
fn test_scatter_plot_builder_pattern_chaining_complete() {
    let series1 = ScatterSeries::new("A", vec![(1.0, 2.0)])
        .with_color(Color::Red)
        .with_marker('×');
    let series2 = ScatterSeries::new("B", vec![(3.0, 4.0)])
        .with_color(Color::Green)
        .with_marker('●');

    let plot = ScatterPlot::new()
        .add_series(series1)
        .add_series(series2)
        .with_title("Test Plot")
        .with_x_label("X Axis")
        .with_y_label("Y Axis")
        .with_legend(true)
        .with_grid(true)
        .with_x_bounds(0.0, 10.0)
        .with_y_bounds(0.0, 10.0);

    assert_eq!(plot.series_count(), 2);
    assert_eq!(plot.title, Some("Test Plot".to_string()));
    assert_eq!(plot.x_label, Some("X Axis".to_string()));
    assert_eq!(plot.y_label, Some("Y Axis".to_string()));
    assert!(plot.show_legend);
    assert!(plot.show_grid);
    assert_eq!(plot.x_bounds, Some((0.0, 10.0)));
    assert_eq!(plot.y_bounds, Some((0.0, 10.0)));
}

#[test]
fn test_scatter_plot_multiple_title_calls() {
    let plot = ScatterPlot::new()
        .with_title("First")
        .with_title("Second")
        .with_title("Third");
    assert_eq!(plot.title, Some("Third".to_string()));
}

#[test]
fn test_scatter_plot_multiple_bounds_calls() {
    let plot = ScatterPlot::new()
        .with_x_bounds(0.0, 10.0)
        .with_x_bounds(5.0, 15.0)
        .with_y_bounds(0.0, 100.0)
        .with_y_bounds(50.0, 150.0);
    assert_eq!(plot.x_bounds, Some((5.0, 15.0)));
    assert_eq!(plot.y_bounds, Some((50.0, 150.0)));
}

#[test]
fn test_scatter_plot_bounds_with_extreme_values() {
    let plot = ScatterPlot::new()
        .with_x_bounds(f64::MIN, f64::MAX)
        .with_y_bounds(f64::MIN, f64::MAX);
    let (x_min, x_max) = plot.calculate_x_bounds();
    let (y_min, y_max) = plot.calculate_y_bounds();
    assert_eq!(x_min, f64::MIN);
    assert_eq!(x_max, f64::MAX);
    assert_eq!(y_min, f64::MIN);
    assert_eq!(y_max, f64::MAX);
}

#[test]
fn test_scatter_plot_calculate_bounds_with_single_point() {
    let plot = ScatterPlot::new()
        .add_series(ScatterSeries::new("Test", vec![(5.0, 10.0)]));
    let (x_min, x_max) = plot.calculate_x_bounds();
    let (y_min, y_max) = plot.calculate_y_bounds();

    // With single point, padding is 0 (max - min = 0), so bounds == point
    assert_eq!(x_min, 5.0);
    assert_eq!(x_max, 5.0);
    assert_eq!(y_min, 10.0);
    assert_eq!(y_max, 10.0);
}

#[test]
fn test_scatter_plot_calculate_bounds_with_same_points() {
    let plot = ScatterPlot::new()
        .add_series(ScatterSeries::new("Test", vec![(5.0, 10.0), (5.0, 10.0), (5.0, 10.0)]));
    let (x_min, x_max) = plot.calculate_x_bounds();
    let (y_min, y_max) = plot.calculate_y_bounds();

    // With 10% padding (padding is 0 for same points, so min == max)
    assert_eq!(x_min, 5.0);
    assert_eq!(x_max, 5.0);
    assert_eq!(y_min, 10.0);
    assert_eq!(y_max, 10.0);
}

#[test]
fn test_scatter_plot_legend_toggle() {
    let plot1 = ScatterPlot::new().with_legend(true);
    let plot2 = ScatterPlot::new().with_legend(false);
    assert!(plot1.show_legend);
    assert!(!plot2.show_legend);
}

#[test]
fn test_scatter_plot_grid_toggle() {
    let plot1 = ScatterPlot::new().with_grid(true);
    let plot2 = ScatterPlot::new().with_grid(false);
    assert!(plot1.show_grid);
    assert!(!plot2.show_grid);
}

#[test]
fn test_scatter_series_single_point() {
    let series = ScatterSeries::new("Single", vec![(42.0, 100.0)]);
    let (min_x, max_x) = series.x_bounds();
    let (min_y, max_y) = series.y_bounds();
    assert_eq!(min_x, Some(42.0));
    assert_eq!(max_x, Some(42.0));
    assert_eq!(min_y, Some(100.0));
    assert_eq!(max_y, Some(100.0));
}

#[test]
fn test_scatter_plot_render_with_all_features() {
    let plot = ScatterPlot::new()
        .with_title("📊 Complete Test")
        .with_x_label("X 軸")
        .with_y_label("Y 軸")
        .add_series(ScatterSeries::new("Series 1", vec![(1.0, 2.0), (3.0, 4.0)]).with_marker('×'))
        .add_series(ScatterSeries::new("Series 2", vec![(2.0, 3.0), (4.0, 5.0)]).with_marker('●'))
        .with_legend(true)
        .with_grid(true)
        .with_x_bounds(0.0, 5.0)
        .with_y_bounds(0.0, 6.0);

    let lines = plot.render_lines(80, 40);
    assert!(!lines.is_empty());
}

// ============ NaN AND INFINITY HANDLING TESTS ============

#[test]
fn test_scatter_series_x_bounds_with_nan() {
    // NaN values should be handled gracefully with total_cmp
    let series = ScatterSeries::new("Test", vec![(f64::NAN, 2.0), (3.0, 4.0), (2.0, 3.0)]);
    let (min, max) = series.x_bounds();
    // total_cmp treats NaN as less than all other values
    assert!(min.is_some());
    assert!(max.is_some());
}

#[test]
fn test_scatter_series_y_bounds_with_nan() {
    let series = ScatterSeries::new("Test", vec![(1.0, f64::NAN), (3.0, 4.0), (2.0, 3.0)]);
    let (min, max) = series.y_bounds();
    // Should not panic, NaN is handled by total_cmp
    assert!(min.is_some());
    assert!(max.is_some());
}

#[test]
fn test_scatter_series_x_bounds_with_infinity() {
    let series = ScatterSeries::new(
        "Test",
        vec![(f64::INFINITY, 2.0), (3.0, 4.0), (f64::NEG_INFINITY, 3.0)],
    );
    let (min, max) = series.x_bounds();
    assert_eq!(min, Some(f64::NEG_INFINITY));
    assert_eq!(max, Some(f64::INFINITY));
}

#[test]
fn test_scatter_series_y_bounds_with_infinity() {
    let series = ScatterSeries::new(
        "Test",
        vec![(1.0, f64::INFINITY), (3.0, 4.0), (2.0, f64::NEG_INFINITY)],
    );
    let (min, max) = series.y_bounds();
    assert_eq!(min, Some(f64::NEG_INFINITY));
    assert_eq!(max, Some(f64::INFINITY));
}

#[test]
fn test_scatter_series_all_nan() {
    // All NaN values should still return Some
    let series = ScatterSeries::new("Test", vec![(f64::NAN, f64::NAN), (f64::NAN, f64::NAN)]);
    let (min_x, max_x) = series.x_bounds();
    let (min_y, max_y) = series.y_bounds();
    assert!(min_x.is_some());
    assert!(max_x.is_some());
    assert!(min_y.is_some());
    assert!(max_y.is_some());
}

#[test]
fn test_scatter_series_mixed_valid_and_nan() {
    // Mix of valid and NaN values
    let series = ScatterSeries::new(
        "Test",
        vec![(1.0, 2.0), (f64::NAN, 3.0), (3.0, f64::NAN), (2.0, 4.0)],
    );
    let (min_x, max_x) = series.x_bounds();
    let (min_y, max_y) = series.y_bounds();
    // total_cmp orders NaN as less than all finite values
    assert!(min_x.is_some());
    assert!(max_x.is_some());
    assert!(min_y.is_some());
    assert!(max_y.is_some());
}

#[test]
fn test_scatter_series_single_point_with_nan() {
    let series = ScatterSeries::new("Test", vec![(f64::NAN, f64::NAN)]);
    let (min_x, max_x) = series.x_bounds();
    let (min_y, max_y) = series.y_bounds();
    assert!(min_x.is_some());
    assert!(max_x.is_some());
    assert!(min_y.is_some());
    assert!(max_y.is_some());
}

#[test]
fn test_scatter_series_zero_values() {
    // Test with zero values (regression test)
    let series = ScatterSeries::new("Test", vec![(0.0, 0.0), (1.0, 1.0)]);
    let (min_x, max_x) = series.x_bounds();
    let (min_y, max_y) = series.y_bounds();
    assert_eq!(min_x, Some(0.0));
    assert_eq!(max_x, Some(1.0));
    assert_eq!(min_y, Some(0.0));
    assert_eq!(max_y, Some(1.0));
}

#[test]
fn test_scatter_series_negative_values() {
    // Test with negative values
    let series = ScatterSeries::new("Test", vec![(-5.0, -10.0), (5.0, 10.0), (0.0, 0.0)]);
    let (min_x, max_x) = series.x_bounds();
    let (min_y, max_y) = series.y_bounds();
    assert_eq!(min_x, Some(-5.0));
    assert_eq!(max_x, Some(5.0));
    assert_eq!(min_y, Some(-10.0));
    assert_eq!(max_y, Some(10.0));
}
