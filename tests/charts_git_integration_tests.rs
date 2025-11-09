//! Integration tests for PLATINUM Tier Charts & Git Widgets
//!
//! Tests for BarChart, ScatterPlot, LiveGraph, GitGraph, GitStatusPanel, GitDiffViewer.

use ratatui::style::Color;
use toad::ui::widgets::{
    BarChartWidget, BarData, BarDirection, DiffLine, DiffLineType, FileStatus, GitCommit, GitDiffViewer,
    GitFile, GitGraph, GitStatusPanel, GraphType, LiveGraph, ScatterPlot, ScatterPlotWidget, ScatterSeries,
};

// ==================== BarChartWidget Tests ====================

#[test]
fn test_bar_chart_creation() {
    let bars = vec![
        BarData::new("Q1", 100.0),
        BarData::new("Q2", 150.0),
        BarData::new("Q3", 120.0),
    ];

    let _chart = BarChartWidget::new(bars);
}

#[test]
fn test_bar_chart_builder() {
    let bars = vec![
        BarData::new("A", 50.0).with_color(Color::Red),
        BarData::new("B", 75.0).with_color(Color::Green),
    ];

    let _chart = BarChartWidget::new(bars)
        .with_title("Sales")
        .with_value_label("Revenue")
        .with_direction(BarDirection::Vertical)
        .with_values(true);
}

#[test]
fn test_bar_direction() {
    let bars = vec![BarData::new("Test", 50.0)];

    let _vertical = BarChartWidget::new(bars.clone()).with_direction(BarDirection::Vertical);
    let _horizontal = BarChartWidget::new(bars).with_direction(BarDirection::Horizontal);
}

// ==================== ScatterPlot Tests (simple from chart.rs) ====================

#[test]
fn test_scatter_plot_simple_creation() {
    let data = vec![(1.0, 2.0), (2.0, 4.0), (3.0, 6.0)];
    let _plot = ScatterPlot::new(data);
}

#[test]
fn test_scatter_plot_simple_builder() {
    let data = vec![(1.0, 1.0), (2.0, 2.0)];
    let _plot = ScatterPlot::new(data).with_title("Analysis");
}

// ==================== ScatterPlotWidget Tests (with series) ====================

#[test]
fn test_scatter_plot_widget_creation() {
    let plot = ScatterPlotWidget::new();

    assert_eq!(plot.series_count(), 0);
}

#[test]
fn test_scatter_plot_widget_add_series() {
    let points = vec![(1.0, 2.0), (2.0, 4.0), (3.0, 6.0)];
    let series = ScatterSeries::new("Linear", points);

    let plot = ScatterPlotWidget::new().add_series(series);

    assert_eq!(plot.series_count(), 1);
}

#[test]
fn test_scatter_plot_widget_builder() {
    let series = ScatterSeries::new("Data", vec![(1.0, 1.0), (2.0, 2.0)])
        .with_color(Color::Red)
        .with_marker('●');

    let plot = ScatterPlotWidget::new()
        .add_series(series)
        .with_title("Analysis")
        .with_x_label("X")
        .with_y_label("Y")
        .with_legend(true)
        .with_grid(true);

    assert_eq!(plot.series_count(), 1);
}

#[test]
fn test_scatter_series_bounds() {
    let points = vec![(5.0, 10.0), (15.0, 25.0), (20.0, 30.0)];
    let series = ScatterSeries::new("Data", points);

    let (x_min, x_max) = series.x_bounds();
    let (y_min, y_max) = series.y_bounds();

    assert_eq!(x_min, Some(5.0));
    assert_eq!(x_max, Some(20.0));
    assert_eq!(y_min, Some(10.0));
    assert_eq!(y_max, Some(30.0));
}

// ==================== LiveGraph Tests ====================

#[test]
fn test_live_graph_creation() {
    let graph = LiveGraph::new(GraphType::Line);

    assert_eq!(graph.data_points(), 0);
}

#[test]
fn test_live_graph_builder() {
    let _graph = LiveGraph::new(GraphType::Line)
        .with_title("CPU")
        .with_y_label("Percent")
        .with_max_points(100)
        .with_color(Color::Green)
        .with_y_bounds(0.0, 100.0)
        .with_auto_scale(false);
}

#[test]
fn test_live_graph_add_point() {
    let mut graph = LiveGraph::new(GraphType::Line);

    graph.add_point(50.0);
    graph.add_point(60.0);

    assert_eq!(graph.data_points(), 2);
}

#[test]
fn test_live_graph_add_points() {
    let mut graph = LiveGraph::new(GraphType::Bar);

    graph.add_points(&[10.0, 20.0, 30.0]);

    assert_eq!(graph.data_points(), 3);
}

#[test]
fn test_live_graph_max_points() {
    let mut graph = LiveGraph::new(GraphType::Line).with_max_points(3);

    graph.add_point(1.0);
    graph.add_point(2.0);
    graph.add_point(3.0);
    graph.add_point(4.0);

    assert_eq!(graph.data_points(), 3);
}

#[test]
fn test_live_graph_statistics() {
    let mut graph = LiveGraph::new(GraphType::Line);

    graph.add_points(&[10.0, 20.0, 30.0, 40.0, 50.0]);

    assert_eq!(graph.latest(), Some(50.0));
    assert_eq!(graph.average(), Some(30.0));
    assert_eq!(graph.min(), Some(10.0));
    assert_eq!(graph.max(), Some(50.0));
}

#[test]
fn test_live_graph_types() {
    let line = LiveGraph::new(GraphType::Line);
    let bar = LiveGraph::new(GraphType::Bar);
    let scatter = LiveGraph::new(GraphType::Scatter);

    assert_eq!(line.data_points(), 0);
    assert_eq!(bar.data_points(), 0);
    assert_eq!(scatter.data_points(), 0);
}

// ==================== GitGraph Tests ====================

#[test]
fn test_git_graph_creation() {
    let graph = GitGraph::new();

    assert_eq!(graph.commit_count(), 0);
}

#[test]
fn test_git_graph_builder() {
    let _graph = GitGraph::new()
        .with_title("History")
        .with_authors(true)
        .with_branches(true)
        .with_compact(false)
        .with_max_commits(50);
}

#[test]
fn test_git_graph_add_commit() {
    let mut graph = GitGraph::new();

    let commit = GitCommit::new("abc123", "Initial")
        .with_author("Alice")
        .with_branch("main");

    graph.add_commit(commit);

    assert_eq!(graph.commit_count(), 1);
}

#[test]
fn test_git_graph_add_commits() {
    let mut graph = GitGraph::new();

    let commits = vec![
        GitCommit::new("a1", "feat: Add").with_author("Alice"),
        GitCommit::new("a2", "fix: Bug").with_author("Bob"),
    ];

    graph.add_commits(commits);

    assert_eq!(graph.commit_count(), 2);
}

#[test]
fn test_git_commit_builder() {
    let commit = GitCommit::new("abc", "Merge")
        .with_author("Dev")
        .with_branch("main")
        .with_parent("parent")
        .with_merge_parent("merge")
        .with_color(Color::Magenta);

    assert!(commit.is_merge());
}

// ==================== GitStatusPanel Tests ====================

#[test]
fn test_git_status_panel_creation() {
    let panel = GitStatusPanel::new();

    assert_eq!(panel.file_count(), 0);
}

#[test]
fn test_git_status_panel_builder() {
    let _panel = GitStatusPanel::new()
        .with_title("Status")
        .with_summary(true)
        .with_compact(false);
}

#[test]
fn test_git_status_panel_add_file() {
    let mut panel = GitStatusPanel::new();

    panel.add_file("main.rs", FileStatus::Modified);
    panel.add_file("README.md", FileStatus::Untracked);

    assert_eq!(panel.file_count(), 2);
}

#[test]
fn test_git_status_panel_set_files() {
    let mut panel = GitStatusPanel::new();

    let files = vec![
        GitFile::new("file1.txt", FileStatus::Modified),
        GitFile::new("file2.txt", FileStatus::Staged),
    ];

    panel.set_files(files);

    assert_eq!(panel.file_count(), 2);
}

#[test]
fn test_git_status_panel_selection() {
    let mut panel = GitStatusPanel::new();

    panel.add_file("file1.rs", FileStatus::Modified);
    panel.add_file("file2.rs", FileStatus::Staged);

    panel.toggle_selection(0);
    panel.toggle_selection(1);

    let selected = panel.selected_files();
    assert_eq!(selected.len(), 2);
}

#[test]
fn test_file_status_variants() {
    let _modified = FileStatus::Modified;
    let _staged = FileStatus::Staged;
    let _untracked = FileStatus::Untracked;
    let _deleted = FileStatus::Deleted;
    let _renamed = FileStatus::Renamed;
    let _conflicted = FileStatus::Conflicted;
    let _mod_staged = FileStatus::ModifiedStaged;
}

// ==================== GitDiffViewer Tests ====================

#[test]
fn test_git_diff_viewer_creation() {
    let viewer = GitDiffViewer::new();

    assert_eq!(viewer.line_count(), 0);
}

#[test]
fn test_git_diff_viewer_builder() {
    let _viewer = GitDiffViewer::new()
        .with_title("Diff")
        .with_line_numbers(true)
        .with_syntax_highlighting(true)
        .with_compact(false);
}

#[test]
fn test_git_diff_viewer_set_diff() {
    let mut viewer = GitDiffViewer::new();

    let diff = r#"@@ -10,3 +10,4 @@
 fn main() {
-    println!("Hello");
+    println!("Hello, World!");
 }
"#;

    viewer.set_diff(diff);

    assert!(viewer.line_count() > 0);
}

#[test]
fn test_git_diff_viewer_stats() {
    let mut viewer = GitDiffViewer::new();

    let diff = r#"@@ -5,5 +5,6 @@
-removed 1
-removed 2
+added 1
+added 2
+added 3
"#;

    viewer.set_diff(diff);

    let (additions, deletions, _) = viewer.stats();
    assert_eq!(additions, 3);
    assert_eq!(deletions, 2);
}

#[test]
fn test_diff_line_creation() {
    let _addition = DiffLine::new("+new", DiffLineType::Addition, None, Some(2));
    let _deletion = DiffLine::new("-old", DiffLineType::Deletion, Some(1), None);
    let _context = DiffLine::new(" same", DiffLineType::Context, Some(5), Some(5));
    let _hunk = DiffLine::new("@@ -1,1 +1,1 @@", DiffLineType::Hunk, None, None);
}

#[test]
fn test_diff_line_types() {
    let _addition = DiffLineType::Addition;
    let _deletion = DiffLineType::Deletion;
    let _context = DiffLineType::Context;
    let _hunk = DiffLineType::Hunk;
    let _file_header = DiffLineType::FileHeader;
}

// ==================== Cross-Feature Integration Tests ====================

#[test]
fn test_git_workflow() {
    let mut status = GitStatusPanel::new();
    status.set_branch("main");
    status.add_file("main.rs", FileStatus::Modified);

    let mut graph = GitGraph::new();
    graph.add_commit(GitCommit::new("a1", "Initial").with_branch("main"));

    let mut diff = GitDiffViewer::new();
    diff.set_diff("@@ -1,1 +1,1 @@\n-old\n+new\n");

    assert_eq!(status.file_count(), 1);
    assert_eq!(graph.commit_count(), 1);
    assert!(diff.line_count() > 0);
}

#[test]
fn test_performance_monitoring() {
    let mut cpu = LiveGraph::new(GraphType::Line).with_max_points(60);
    let mut memory = LiveGraph::new(GraphType::Bar).with_max_points(60);

    for i in 0..60 {
        cpu.add_point(40.0 + 30.0 * ((i as f64) / 10.0).sin());
        memory.add_point(50.0 + (i as f64) * 0.3);
    }

    assert_eq!(cpu.data_points(), 60);
    assert_eq!(memory.data_points(), 60);
}

#[test]
fn test_charts_dashboard() {
    let bars = vec![
        BarData::new("A", 100.0),
        BarData::new("B", 150.0),
    ];
    let _bar_chart = BarChartWidget::new(bars).with_title("Sales");

    let series = ScatterSeries::new("Data", vec![(1.0, 100.0), (2.0, 200.0)]);
    let scatter = ScatterPlotWidget::new().add_series(series);

    let mut live = LiveGraph::new(GraphType::Line).with_max_points(30);
    for i in 0..30 {
        live.add_point(50.0 + 10.0 * ((i as f64) / 5.0).sin());
    }

    assert_eq!(scatter.series_count(), 1);
    assert_eq!(live.data_points(), 30);
}
