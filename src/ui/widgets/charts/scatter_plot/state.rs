//! Scatter plot widget for distribution visualization
//!
//! Displays data points in 2D space, ideal for showing correlations and distributions.
//!
//! # Examples
//!
//! ```
//! use toad::widgets::{ScatterPlot, ScatterSeries};
//! use ratatui::style::Color;
//!
//! let points = vec![(1.0, 2.0), (2.0, 3.5), (3.0, 3.0), (4.0, 5.0)];
//! let series = ScatterSeries::new("Data", points).with_color(Color::Blue);
//!
//! let plot = ScatterPlot::new()
//!     .add_series(series)
//!     .with_title("Correlation Analysis")
//!     .with_x_label("Input")
//!     .with_y_label("Output");
//! ```

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Widget},
};

/// A data series for scatter plots
///
/// # Examples
///
/// ```
/// use toad::widgets::ScatterSeries;
/// use ratatui::style::Color;
///
/// let points = vec![(1.0, 2.0), (2.0, 3.0)];
/// let series = ScatterSeries::new("Sales vs Revenue", points)
///     .with_color(Color::Green);
/// ```
#[derive(Debug, Clone)]
pub struct ScatterSeries {
    /// Series name
    pub name: String,
    /// Data points (x, y)
    pub points: Vec<(f64, f64)>,
    /// Point color
    pub color: Color,
    /// Point marker character
    pub marker: char,
}

impl ScatterSeries {
    /// Create a new scatter series
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::ScatterSeries;
    ///
    /// let points = vec![(1.0, 2.0), (3.0, 4.0)];
    /// let series = ScatterSeries::new("Data", points);
    /// assert_eq!(series.name, "Data");
    /// assert_eq!(series.points.len(), 2);
    /// ```
    pub fn new(name: impl Into<String>, points: Vec<(f64, f64)>) -> Self {
        Self {
            name: name.into(),
            points,
            color: Color::Cyan,
            marker: '●',
        }
    }

    /// Set point color
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::ScatterSeries;
    /// use ratatui::style::Color;
    ///
    /// let series = ScatterSeries::new("Data", vec![(1.0, 2.0)])
    ///     .with_color(Color::Red);
    /// assert_eq!(series.color, Color::Red);
    /// ```
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Set point marker character
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::ScatterSeries;
    ///
    /// let series = ScatterSeries::new("Data", vec![(1.0, 2.0)])
    ///     .with_marker('×');
    /// assert_eq!(series.marker, '×');
    /// ```
    pub fn with_marker(mut self, marker: char) -> Self {
        self.marker = marker;
        self
    }

    /// Get X-axis bounds (min, max)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::ScatterSeries;
    ///
    /// let series = ScatterSeries::new("Data", vec![(1.0, 2.0), (3.0, 4.0)]);
    /// let (min, max) = series.x_bounds();
    /// assert_eq!(min, Some(1.0));
    /// assert_eq!(max, Some(3.0));
    /// ```
    pub fn x_bounds(&self) -> (Option<f64>, Option<f64>) {
        let min = self
            .points
            .iter()
            .map(|(x, _)| *x)
            .min_by(|a, b| a.total_cmp(b));
        let max = self
            .points
            .iter()
            .map(|(x, _)| *x)
            .max_by(|a, b| a.total_cmp(b));
        (min, max)
    }

    /// Get Y-axis bounds (min, max)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::ScatterSeries;
    ///
    /// let series = ScatterSeries::new("Data", vec![(1.0, 2.0), (3.0, 4.0)]);
    /// let (min, max) = series.y_bounds();
    /// assert_eq!(min, Some(2.0));
    /// assert_eq!(max, Some(4.0));
    /// ```
    pub fn y_bounds(&self) -> (Option<f64>, Option<f64>) {
        let min = self
            .points
            .iter()
            .map(|(_, y)| *y)
            .min_by(|a, b| a.total_cmp(b));
        let max = self
            .points
            .iter()
            .map(|(_, y)| *y)
            .max_by(|a, b| a.total_cmp(b));
        (min, max)
    }
}

/// Scatter plot widget
///
/// Displays multiple data series as points in 2D space.
///
/// # Examples
///
/// ```
/// use toad::widgets::{ScatterPlot, ScatterSeries};
///
/// let points = vec![(1.0, 2.0), (2.0, 3.0)];
/// let plot = ScatterPlot::new()
///     .add_series(ScatterSeries::new("Data", points))
///     .with_title("Distribution");
/// ```
#[derive(Debug, Clone)]
pub struct ScatterPlot {
    /// Data series to display
    pub(super) series: Vec<ScatterSeries>,
    /// Plot title
    pub(super) title: Option<String>,
    /// X-axis label
    pub(super) x_label: Option<String>,
    /// Y-axis label
    pub(super) y_label: Option<String>,
    /// Show legend
    pub(super) show_legend: bool,
    /// Show grid
    pub(super) show_grid: bool,
    /// Manual X-axis bounds (min, max)
    pub(super) x_bounds: Option<(f64, f64)>,
    /// Manual Y-axis bounds (min, max)
    pub(super) y_bounds: Option<(f64, f64)>,
}

impl ScatterPlot {
    /// Create a new scatter plot
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::ScatterPlot;
    ///
    /// let plot = ScatterPlot::new();
    /// ```
    pub fn new() -> Self {
        Self {
            series: Vec::new(),
            title: None,
            x_label: None,
            y_label: None,
            show_legend: true,
            show_grid: false,
            x_bounds: None,
            y_bounds: None,
        }
    }

    /// Add a data series
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{ScatterPlot, ScatterSeries};
    ///
    /// let plot = ScatterPlot::new()
    ///     .add_series(ScatterSeries::new("Data", vec![(1.0, 2.0)]));
    /// assert_eq!(plot.series_count(), 1);
    /// ```
    pub fn add_series(mut self, series: ScatterSeries) -> Self {
        self.series.push(series);
        self
    }

    /// Set plot title
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::ScatterPlot;
    ///
    /// let plot = ScatterPlot::new()
    ///     .with_title("My Plot");
    /// ```
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set X-axis label
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::ScatterPlot;
    ///
    /// let plot = ScatterPlot::new()
    ///     .with_x_label("Time");
    /// ```
    pub fn with_x_label(mut self, label: impl Into<String>) -> Self {
        self.x_label = Some(label.into());
        self
    }

    /// Set Y-axis label
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::ScatterPlot;
    ///
    /// let plot = ScatterPlot::new()
    ///     .with_y_label("Value");
    /// ```
    pub fn with_y_label(mut self, label: impl Into<String>) -> Self {
        self.y_label = Some(label.into());
        self
    }

    /// Show or hide legend
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::ScatterPlot;
    ///
    /// let plot = ScatterPlot::new()
    ///     .with_legend(false);
    /// ```
    pub fn with_legend(mut self, show: bool) -> Self {
        self.show_legend = show;
        self
    }

    /// Show or hide grid
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::ScatterPlot;
    ///
    /// let plot = ScatterPlot::new()
    ///     .with_grid(true);
    /// ```
    pub fn with_grid(mut self, show: bool) -> Self {
        self.show_grid = show;
        self
    }

    /// Set X-axis bounds manually
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::ScatterPlot;
    ///
    /// let plot = ScatterPlot::new()
    ///     .with_x_bounds(0.0, 10.0);
    /// ```
    pub fn with_x_bounds(mut self, min: f64, max: f64) -> Self {
        self.x_bounds = Some((min, max));
        self
    }

    /// Set Y-axis bounds manually
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::ScatterPlot;
    ///
    /// let plot = ScatterPlot::new()
    ///     .with_y_bounds(0.0, 100.0);
    /// ```
    pub fn with_y_bounds(mut self, min: f64, max: f64) -> Self {
        self.y_bounds = Some((min, max));
        self
    }

    /// Get number of series
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{ScatterPlot, ScatterSeries};
    ///
    /// let plot = ScatterPlot::new();
    /// assert_eq!(plot.series_count(), 0);
    ///
    /// let plot = plot.add_series(ScatterSeries::new("A", vec![(1.0, 2.0)]));
    /// assert_eq!(plot.series_count(), 1);
    /// ```
    pub fn series_count(&self) -> usize {
        self.series.len()
    }

    /// Get X-axis bounds (min, max)
    pub(super) fn calculate_x_bounds(&self) -> (f64, f64) {
        if let Some(bounds) = self.x_bounds {
            return bounds;
        }

        let mut min = f64::MAX;
        let mut max = f64::MIN;

        for series in &self.series {
            if let (Some(s_min), Some(s_max)) = series.x_bounds() {
                min = min.min(s_min);
                max = max.max(s_max);
            }
        }

        if min == f64::MAX || max == f64::MIN {
            return (0.0, 1.0);
        }

        // Add 10% padding
        let padding = (max - min) * 0.1;
        (min - padding, max + padding)
    }

    /// Get Y-axis bounds (min, max)
    pub(super) fn calculate_y_bounds(&self) -> (f64, f64) {
        if let Some(bounds) = self.y_bounds {
            return bounds;
        }

        let mut min = f64::MAX;
        let mut max = f64::MIN;

        for series in &self.series {
            if let (Some(s_min), Some(s_max)) = series.y_bounds() {
                min = min.min(s_min);
                max = max.max(s_max);
            }
        }

        if min == f64::MAX || max == f64::MIN {
            return (0.0, 1.0);
        }

        // Add 10% padding
        let padding = (max - min) * 0.1;
        (min - padding, max + padding)
    }

    /// Render as text lines
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{ScatterPlot, ScatterSeries};
    ///
    /// let plot = ScatterPlot::new()
    ///     .add_series(ScatterSeries::new("Data", vec![(1.0, 2.0)]));
    ///
    /// let lines = plot.render_lines(40, 20);
    /// assert!(!lines.is_empty());
    /// ```
    pub fn render_lines(&self, width: u16, height: u16) -> Vec<Line<'static>> {
        let mut lines = Vec::new();

        // Title
        if let Some(title) = &self.title {
            lines.push(Line::from(Span::styled(
                title.clone(),
                Style::default().fg(Color::Cyan),
            )));
        }

        if self.series.is_empty() {
            lines.push(Line::from("No data"));
            return lines;
        }

        // Y-axis label
        if let Some(label) = &self.y_label {
            lines.push(Line::from(format!("Y: {}", label)));
        }

        let (x_min, x_max) = self.calculate_x_bounds();
        let (y_min, y_max) = self.calculate_y_bounds();
        let chart_height = height.saturating_sub(4);
        let chart_width = width.saturating_sub(10);

        // Render plot area
        for h in 0..chart_height {
            let y_value = y_max - (h as f64 / chart_height as f64) * (y_max - y_min);
            let mut line_str = format!("{:>6.1} |", y_value);

            for w in 0..chart_width {
                let x_value = x_min + (w as f64 / chart_width as f64) * (x_max - x_min);
                let mut found = false;

                // Check if any point is near this position
                for series in &self.series {
                    for (px, py) in &series.points {
                        let x_diff = ((px - x_value) / (x_max - x_min) * chart_width as f64).abs();
                        let y_diff = ((py - y_value) / (y_max - y_min) * chart_height as f64).abs();

                        if x_diff < 0.5 && y_diff < 0.5 {
                            line_str.push(series.marker);
                            found = true;
                            break;
                        }
                    }
                    if found {
                        break;
                    }
                }

                if !found {
                    line_str.push(if self.show_grid && (h % 5 == 0 || w % 5 == 0) {
                        '·'
                    } else {
                        ' '
                    });
                }
            }

            lines.push(Line::from(line_str));
        }

        // X-axis
        let mut x_axis = String::from("       +");
        for _ in 0..chart_width {
            x_axis.push('─');
        }
        lines.push(Line::from(x_axis));

        // X-axis label
        if let Some(label) = &self.x_label {
            lines.push(Line::from(format!("        X: {}", label)));
        }

        // Legend
        if self.show_legend && !self.series.is_empty() {
            lines.push(Line::from(""));
            lines.push(Line::from("Legend:"));
            for series in &self.series {
                lines.push(Line::from(vec![
                    Span::raw("  "),
                    Span::styled(series.marker.to_string(), Style::default().fg(series.color)),
                    Span::raw(format!(" {} ({} points)", series.name, series.points.len())),
                ]));
            }
        }

        lines
    }
}

impl Default for ScatterPlot {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for &ScatterPlot {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let lines = self.render_lines(area.width, area.height);
        let block = Block::default().borders(Borders::ALL);
        let inner = block.inner(area);

        block.render(area, buf);

        for (i, line) in lines.iter().enumerate() {
            if i >= inner.height as usize {
                break;
            }
            let y = inner.y + i as u16;
            buf.set_line(inner.x, y, line, inner.width);
        }
    }
}

