/// let chart = LineChart::new(data);
/// assert_eq!(chart.data().len(), 5);
/// ```
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};
use serde::{Deserialize, Serialize};

use crate::ui::theme::ToadTheme;

/// Line chart style
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[derive(Default)]
pub enum LineStyle {
    /// Solid line
    #[default]
    Solid,
    /// Dotted line
    Dotted,
    /// Dashed line
    Dashed,
    /// Stepped line
    Stepped,
}


/// Line chart for time-series data
#[derive(Debug, Clone)]
pub struct LineChart {
    /// Data points
    pub(super) data: Vec<f64>,
    /// Chart title
    pub(super) title: Option<String>,
    /// Line style
    pub(super) line_style: LineStyle,
    /// Line color
    pub(super) color: Color,
    /// Show border
    pub(super) show_border: bool,
    /// Show axes
    pub(super) show_axes: bool,
    /// Show values
    pub(super) show_values: bool,
}

impl LineChart {
    /// Create a new line chart
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::LineChart;
    ///
    /// let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    /// let chart = LineChart::new(data);
    /// assert_eq!(chart.data().len(), 5);
    /// ```
    pub fn new(data: Vec<f64>) -> Self {
        Self {
            data,
            title: None,
            line_style: LineStyle::Solid,
            color: ToadTheme::TOAD_GREEN,
            show_border: true,
            show_axes: true,
            show_values: false,
        }
    }

    /// Set title
    pub fn with_title<S: Into<String>>(mut self, title: S) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set line style
    pub fn with_line_style(mut self, style: LineStyle) -> Self {
        self.line_style = style;
        self
    }

    /// Set color
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Set whether to show border
    pub fn with_border(mut self, show: bool) -> Self {
        self.show_border = show;
        self
    }

    /// Set whether to show axes
    pub fn with_axes(mut self, show: bool) -> Self {
        self.show_axes = show;
        self
    }

    /// Set whether to show values
    pub fn with_values(mut self, show: bool) -> Self {
        self.show_values = show;
        self
    }

    /// Get data
    pub fn data(&self) -> &[f64] {
        &self.data
    }

    /// Set data
    pub fn set_data(&mut self, data: Vec<f64>) {
        self.data = data;
    }

    /// Add data point
    pub fn add_point(&mut self, value: f64) {
        self.data.push(value);
    }

    /// Get min and max values
    pub(super) fn bounds(&self) -> (f64, f64) {
        if self.data.is_empty() {
            return (0.0, 1.0);
        }

        let min = self.data.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max = self.data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

        if (max - min).abs() < 1e-10 {
            (min - 1.0, max + 1.0)
        } else {
            (min, max)
        }
    }

    /// Normalize value to 0.0-1.0 range
    pub(super) fn normalize(&self, value: f64) -> f64 {
        let (min, max) = self.bounds();
        if (max - min).abs() < 1e-10 {
            0.5
        } else {
            (value - min) / (max - min)
        }
    }

    /// Get character for line based on style
    pub(super) fn line_char(&self) -> char {
        match self.line_style {
            LineStyle::Solid => '─',
            LineStyle::Dotted => '·',
            LineStyle::Dashed => '-',
            LineStyle::Stepped => '═',
        }
    }

    /// Render the chart
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        if area.width < 4 || area.height < 4 {
            return;
        }

        let mut lines_to_render: Vec<Line> = Vec::new();

        // Calculate chart dimensions
        let chart_height = if self.show_axes {
            area.height.saturating_sub(3) as usize
        } else {
            area.height.saturating_sub(2) as usize
        };

        let chart_width = if self.show_axes {
            area.width.saturating_sub(6) as usize
        } else {
            area.width.saturating_sub(2) as usize
        };

        if chart_height == 0 || chart_width == 0 {
            return;
        }

        // Create sparkline representation
        let sparkline = self.create_sparkline(chart_width, chart_height);

        // Add chart lines
        for line in sparkline {
            lines_to_render.push(Line::from(Span::styled(
                line,
                Style::default().fg(self.color),
            )));
        }

        // Add min/max labels if showing values
        if self.show_values && !self.data.is_empty() {
            let (min, max) = self.bounds();
            let info = format!(" Min: {:.2}  Max: {:.2} ", min, max);
            lines_to_render.push(Line::from(Span::styled(
                info,
                Style::default().fg(ToadTheme::GRAY),
            )));
        }

        let paragraph = if self.show_border {
            let title = self.title.as_deref().unwrap_or("Chart");
            Paragraph::new(lines_to_render).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(title)
                    .border_style(Style::default().fg(ToadTheme::BORDER)),
            )
        } else {
            Paragraph::new(lines_to_render)
        };

        frame.render_widget(paragraph, area);
    }

    /// Create ASCII sparkline representation
    pub(super) fn create_sparkline(&self, width: usize, height: usize) -> Vec<String> {
        if self.data.is_empty() {
            return vec![String::new(); height];
        }

        let mut lines = vec![String::from(" ").repeat(width); height];

        // Sample data to fit width
        let step = self.data.len() as f64 / width as f64;
        let line_char = self.line_char();

        for x in 0..width {
            let idx = ((x as f64 * step) as usize).min(self.data.len() - 1);
            let value = self.data[idx];
            let normalized = self.normalize(value);
            let y = ((1.0 - normalized) * (height - 1) as f64) as usize;

            if y < height {
                let mut chars: Vec<char> = lines[y].chars().collect();
                if x < chars.len() {
                    chars[x] = line_char;
                    lines[y] = chars.into_iter().collect();
                }
            }
        }

        lines
    }
}

/// Bar chart orientation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[derive(Default)]
pub enum BarOrientation {
    /// Vertical bars (default)
    #[default]
    Vertical,
    /// Horizontal bars
    Horizontal,
}


/// Bar chart for comparison data
#[derive(Debug, Clone)]
pub struct BarChart {
    /// Data points with labels
    pub(super) data: Vec<(String, f64)>,
    /// Chart title
    pub(super) title: Option<String>,
    /// Bar orientation
    pub(super) orientation: BarOrientation,
    /// Bar color
    pub(super) color: Color,
    /// Show border
    pub(super) show_border: bool,
    /// Show values
    pub(super) show_values: bool,
}

impl BarChart {
    /// Create a new bar chart
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::BarChart;
    ///
    /// let data = vec![
    ///     ("A".to_string(), 10.0),
    ///     ("B".to_string(), 20.0),
    ///     ("C".to_string(), 15.0),
    /// ];
    /// let chart = BarChart::new(data);
    /// assert_eq!(chart.data().len(), 3);
    /// ```
    pub fn new(data: Vec<(String, f64)>) -> Self {
        Self {
            data,
            title: None,
            orientation: BarOrientation::Vertical,
            color: ToadTheme::TOAD_GREEN,
            show_border: true,
            show_values: false,
        }
    }

    /// Set title
    pub fn with_title<S: Into<String>>(mut self, title: S) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set orientation
    pub fn with_orientation(mut self, orientation: BarOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Set color
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Set whether to show border
    pub fn with_border(mut self, show: bool) -> Self {
        self.show_border = show;
        self
    }

    /// Set whether to show values
    pub fn with_values(mut self, show: bool) -> Self {
        self.show_values = show;
        self
    }

    /// Get data
    pub fn data(&self) -> &[(String, f64)] {
        &self.data
    }

    /// Set data
    pub fn set_data(&mut self, data: Vec<(String, f64)>) {
        self.data = data;
    }

    /// Add data point
    pub fn add_bar<S: Into<String>>(&mut self, label: S, value: f64) {
        self.data.push((label.into(), value));
    }

    /// Get max value
    pub(super) fn max_value(&self) -> f64 {
        self.data
            .iter()
            .map(|(_, v)| *v)
            .fold(f64::NEG_INFINITY, |a, b| a.max(b))
            .max(1.0)
    }

    /// Render the bar chart
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        if area.width < 4 || area.height < 4 {
            return;
        }

        let mut lines_to_render: Vec<Line> = Vec::new();

        match self.orientation {
            BarOrientation::Vertical => {
                let chart_height = area.height.saturating_sub(3) as usize;
                let max_value = self.max_value();

                for (label, value) in &self.data {
                    let bar_height = ((value / max_value) * chart_height as f64) as usize;
                    let bar = "█".repeat(bar_height.min(chart_height));
                    let text = if self.show_values {
                        format!("{}: {:.1}", label, value)
                    } else {
                        label.clone()
                    };

                    lines_to_render.push(Line::from(vec![
                        Span::styled(bar, Style::default().fg(self.color)),
                        Span::raw(" "),
                        Span::styled(text, Style::default().fg(ToadTheme::LIGHT_GRAY)),
                    ]));
                }
            }
            BarOrientation::Horizontal => {
                let chart_width = area.width.saturating_sub(15) as usize;
                let max_value = self.max_value();

                for (label, value) in &self.data {
                    let bar_width = ((value / max_value) * chart_width as f64) as usize;
                    let bar = "█".repeat(bar_width.min(chart_width));
                    let text = if self.show_values {
                        format!("{:>10} {:.1} ", label, value)
                    } else {
                        format!("{:>10} ", label)
                    };

                    lines_to_render.push(Line::from(vec![
                        Span::styled(text, Style::default().fg(ToadTheme::LIGHT_GRAY)),
                        Span::styled(bar, Style::default().fg(self.color)),
                    ]));
                }
            }
        }

        let paragraph = if self.show_border {
            let title = self.title.as_deref().unwrap_or("Bar Chart");
            Paragraph::new(lines_to_render).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(title)
                    .border_style(Style::default().fg(ToadTheme::BORDER)),
            )
        } else {
            Paragraph::new(lines_to_render)
        };

        frame.render_widget(paragraph, area);
    }
}

/// Scatter plot for distribution visualization
#[derive(Debug, Clone)]
pub struct ScatterPlot {
    /// Data points (x, y)
    pub(super) data: Vec<(f64, f64)>,
    /// Chart title
    pub(super) title: Option<String>,
    /// Point character
    pub(super) point_char: char,
    /// Point color
    pub(super) color: Color,
    /// Show border
    pub(super) show_border: bool,
    /// Show axes
    pub(super) show_axes: bool,
}

impl ScatterPlot {
    /// Create a new scatter plot
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::ScatterPlot;
    ///
    /// let data = vec![(1.0, 2.0), (2.0, 4.0), (3.0, 6.0)];
    /// let plot = ScatterPlot::new(data);
    /// assert_eq!(plot.data().len(), 3);
    /// ```
    pub fn new(data: Vec<(f64, f64)>) -> Self {
        Self {
            data,
            title: None,
            point_char: '•',
            color: ToadTheme::TOAD_GREEN,
            show_border: true,
            show_axes: true,
        }
    }

    /// Set title
    pub fn with_title<S: Into<String>>(mut self, title: S) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set point character
    pub fn with_point_char(mut self, ch: char) -> Self {
        self.point_char = ch;
        self
    }

    /// Set color
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Set whether to show border
    pub fn with_border(mut self, show: bool) -> Self {
        self.show_border = show;
        self
    }

    /// Set whether to show axes
    pub fn with_axes(mut self, show: bool) -> Self {
        self.show_axes = show;
        self
    }

    /// Get data
    pub fn data(&self) -> &[(f64, f64)] {
        &self.data
    }

    /// Set data
    pub fn set_data(&mut self, data: Vec<(f64, f64)>) {
        self.data = data;
    }

    /// Add point
    pub fn add_point(&mut self, x: f64, y: f64) {
        self.data.push((x, y));
    }

    /// Get bounds
    pub(super) fn bounds(&self) -> (f64, f64, f64, f64) {
        if self.data.is_empty() {
            return (0.0, 1.0, 0.0, 1.0);
        }

        let mut min_x = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_y = f64::NEG_INFINITY;

        for (x, y) in &self.data {
            min_x = min_x.min(*x);
            max_x = max_x.max(*x);
            min_y = min_y.min(*y);
            max_y = max_y.max(*y);
        }

        // Add padding
        if (max_x - min_x).abs() < 1e-10 {
            min_x -= 1.0;
            max_x += 1.0;
        }
        if (max_y - min_y).abs() < 1e-10 {
            min_y -= 1.0;
            max_y += 1.0;
        }

        (min_x, max_x, min_y, max_y)
    }

    /// Normalize point to grid coordinates
    pub(super) fn normalize(&self, x: f64, y: f64, width: usize, height: usize) -> (usize, usize) {
        let (min_x, max_x, min_y, max_y) = self.bounds();

        let norm_x = (x - min_x) / (max_x - min_x);
        let norm_y = 1.0 - (y - min_y) / (max_y - min_y);

        let grid_x = (norm_x * (width - 1) as f64) as usize;
        let grid_y = (norm_y * (height - 1) as f64) as usize;

        (grid_x.min(width - 1), grid_y.min(height - 1))
    }

    /// Render the scatter plot
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        if area.width < 4 || area.height < 4 {
            return;
        }

        let chart_height = area.height.saturating_sub(3) as usize;
        let chart_width = area.width.saturating_sub(3) as usize;

        if chart_height == 0 || chart_width == 0 {
            return;
        }

        // Create grid
        let mut grid = vec![vec![' '; chart_width]; chart_height];

        // Plot points
        for (x, y) in &self.data {
            let (grid_x, grid_y) = self.normalize(*x, *y, chart_width, chart_height);
            grid[grid_y][grid_x] = self.point_char;
        }

        // Render grid
        let mut lines_to_render: Vec<Line> = Vec::new();
        for row in grid {
            let line_str: String = row.into_iter().collect();
            lines_to_render.push(Line::from(Span::styled(
                line_str,
                Style::default().fg(self.color),
            )));
        }

        let paragraph = if self.show_border {
            let title = self.title.as_deref().unwrap_or("Scatter Plot");
            Paragraph::new(lines_to_render).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(title)
                    .border_style(Style::default().fg(ToadTheme::BORDER)),
            )
        } else {
            Paragraph::new(lines_to_render)
        };

        frame.render_widget(paragraph, area);
    }
}

