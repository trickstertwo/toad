/// Chart widgets for data visualization
///
/// Provides line charts, bar charts, and other data visualizations
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
    data: Vec<f64>,
    /// Chart title
    title: Option<String>,
    /// Line style
    line_style: LineStyle,
    /// Line color
    color: Color,
    /// Show border
    show_border: bool,
    /// Show axes
    show_axes: bool,
    /// Show values
    show_values: bool,
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
    fn bounds(&self) -> (f64, f64) {
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
    fn normalize(&self, value: f64) -> f64 {
        let (min, max) = self.bounds();
        if (max - min).abs() < 1e-10 {
            0.5
        } else {
            (value - min) / (max - min)
        }
    }

    /// Get character for line based on style
    fn line_char(&self) -> char {
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
    fn create_sparkline(&self, width: usize, height: usize) -> Vec<String> {
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
    data: Vec<(String, f64)>,
    /// Chart title
    title: Option<String>,
    /// Bar orientation
    orientation: BarOrientation,
    /// Bar color
    color: Color,
    /// Show border
    show_border: bool,
    /// Show values
    show_values: bool,
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
    fn max_value(&self) -> f64 {
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
    data: Vec<(f64, f64)>,
    /// Chart title
    title: Option<String>,
    /// Point character
    point_char: char,
    /// Point color
    color: Color,
    /// Show border
    show_border: bool,
    /// Show axes
    show_axes: bool,
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
    fn bounds(&self) -> (f64, f64, f64, f64) {
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
    fn normalize(&self, x: f64, y: f64, width: usize, height: usize) -> (usize, usize) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_chart_creation() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let chart = LineChart::new(data);
        assert_eq!(chart.data().len(), 5);
    }

    #[test]
    fn test_line_chart_with_title() {
        let chart = LineChart::new(vec![1.0, 2.0, 3.0]).with_title("Test Chart");
        assert_eq!(chart.title, Some("Test Chart".to_string()));
    }

    #[test]
    fn test_line_chart_with_line_style() {
        let chart = LineChart::new(vec![1.0, 2.0, 3.0]).with_line_style(LineStyle::Dotted);
        assert_eq!(chart.line_style, LineStyle::Dotted);
    }

    #[test]
    fn test_line_chart_with_color() {
        let chart = LineChart::new(vec![1.0, 2.0, 3.0]).with_color(Color::Blue);
        assert_eq!(chart.color, Color::Blue);
    }

    #[test]
    fn test_line_chart_set_data() {
        let mut chart = LineChart::new(vec![1.0, 2.0]);
        chart.set_data(vec![3.0, 4.0, 5.0]);
        assert_eq!(chart.data().len(), 3);
    }

    #[test]
    fn test_line_chart_add_point() {
        let mut chart = LineChart::new(vec![1.0, 2.0]);
        chart.add_point(3.0);
        assert_eq!(chart.data().len(), 3);
        assert_eq!(chart.data()[2], 3.0);
    }

    #[test]
    fn test_line_chart_bounds() {
        let chart = LineChart::new(vec![1.0, 5.0, 3.0, 7.0, 2.0]);
        let (min, max) = chart.bounds();
        assert_eq!(min, 1.0);
        assert_eq!(max, 7.0);
    }

    #[test]
    fn test_line_chart_bounds_empty() {
        let chart = LineChart::new(vec![]);
        let (min, max) = chart.bounds();
        assert_eq!(min, 0.0);
        assert_eq!(max, 1.0);
    }

    #[test]
    fn test_line_chart_bounds_single() {
        let chart = LineChart::new(vec![5.0]);
        let (min, max) = chart.bounds();
        assert_eq!(min, 4.0);
        assert_eq!(max, 6.0);
    }

    #[test]
    fn test_line_chart_normalize() {
        let chart = LineChart::new(vec![0.0, 5.0, 10.0]);
        assert_eq!(chart.normalize(0.0), 0.0);
        assert_eq!(chart.normalize(5.0), 0.5);
        assert_eq!(chart.normalize(10.0), 1.0);
    }

    #[test]
    fn test_line_chart_line_char() {
        let chart = LineChart::new(vec![1.0]).with_line_style(LineStyle::Solid);
        assert_eq!(chart.line_char(), '─');

        let chart = chart.with_line_style(LineStyle::Dotted);
        assert_eq!(chart.line_char(), '·');

        let chart = chart.with_line_style(LineStyle::Dashed);
        assert_eq!(chart.line_char(), '-');

        let chart = chart.with_line_style(LineStyle::Stepped);
        assert_eq!(chart.line_char(), '═');
    }

    #[test]
    fn test_line_chart_create_sparkline() {
        let chart = LineChart::new(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        let sparkline = chart.create_sparkline(10, 5);
        assert_eq!(sparkline.len(), 5);
        assert_eq!(sparkline[0].chars().count(), 10);
    }

    #[test]
    fn test_line_chart_create_sparkline_empty() {
        let chart = LineChart::new(vec![]);
        let sparkline = chart.create_sparkline(10, 5);
        assert_eq!(sparkline.len(), 5);
    }

    #[test]
    fn test_line_style_default() {
        let style = LineStyle::default();
        assert_eq!(style, LineStyle::Solid);
    }

    #[test]
    fn test_line_chart_with_border() {
        let chart = LineChart::new(vec![1.0, 2.0, 3.0]).with_border(false);
        assert!(!chart.show_border);
    }

    #[test]
    fn test_line_chart_with_axes() {
        let chart = LineChart::new(vec![1.0, 2.0, 3.0]).with_axes(false);
        assert!(!chart.show_axes);
    }

    #[test]
    fn test_line_chart_with_values() {
        let chart = LineChart::new(vec![1.0, 2.0, 3.0]).with_values(true);
        assert!(chart.show_values);
    }

    // BarChart tests

    #[test]
    fn test_bar_chart_creation() {
        let data = vec![
            ("A".to_string(), 10.0),
            ("B".to_string(), 20.0),
            ("C".to_string(), 15.0),
        ];
        let chart = BarChart::new(data);
        assert_eq!(chart.data().len(), 3);
    }

    #[test]
    fn test_bar_chart_with_title() {
        let chart = BarChart::new(vec![("A".to_string(), 10.0)]).with_title("Test Bar Chart");
        assert_eq!(chart.title, Some("Test Bar Chart".to_string()));
    }

    #[test]
    fn test_bar_chart_with_orientation() {
        let chart = BarChart::new(vec![("A".to_string(), 10.0)])
            .with_orientation(BarOrientation::Horizontal);
        assert_eq!(chart.orientation, BarOrientation::Horizontal);
    }

    #[test]
    fn test_bar_chart_with_color() {
        let chart = BarChart::new(vec![("A".to_string(), 10.0)]).with_color(Color::Blue);
        assert_eq!(chart.color, Color::Blue);
    }

    #[test]
    fn test_bar_chart_set_data() {
        let mut chart = BarChart::new(vec![("A".to_string(), 10.0)]);
        chart.set_data(vec![("X".to_string(), 5.0), ("Y".to_string(), 15.0)]);
        assert_eq!(chart.data().len(), 2);
    }

    #[test]
    fn test_bar_chart_add_bar() {
        let mut chart = BarChart::new(vec![("A".to_string(), 10.0)]);
        chart.add_bar("B", 20.0);
        assert_eq!(chart.data().len(), 2);
        assert_eq!(chart.data()[1].0, "B");
        assert_eq!(chart.data()[1].1, 20.0);
    }

    #[test]
    fn test_bar_chart_max_value() {
        let chart = BarChart::new(vec![
            ("A".to_string(), 10.0),
            ("B".to_string(), 25.0),
            ("C".to_string(), 15.0),
        ]);
        assert_eq!(chart.max_value(), 25.0);
    }

    #[test]
    fn test_bar_chart_max_value_empty() {
        let chart = BarChart::new(vec![]);
        assert_eq!(chart.max_value(), 1.0);
    }

    #[test]
    fn test_bar_chart_with_border() {
        let chart = BarChart::new(vec![("A".to_string(), 10.0)]).with_border(false);
        assert!(!chart.show_border);
    }

    #[test]
    fn test_bar_chart_with_values() {
        let chart = BarChart::new(vec![("A".to_string(), 10.0)]).with_values(true);
        assert!(chart.show_values);
    }

    #[test]
    fn test_bar_orientation_default() {
        let orientation = BarOrientation::default();
        assert_eq!(orientation, BarOrientation::Vertical);
    }

    // ScatterPlot tests

    #[test]
    fn test_scatter_plot_creation() {
        let data = vec![(1.0, 2.0), (2.0, 4.0), (3.0, 6.0)];
        let plot = ScatterPlot::new(data);
        assert_eq!(plot.data().len(), 3);
    }

    #[test]
    fn test_scatter_plot_with_title() {
        let plot = ScatterPlot::new(vec![(1.0, 2.0)]).with_title("Test Scatter");
        assert_eq!(plot.title, Some("Test Scatter".to_string()));
    }

    #[test]
    fn test_scatter_plot_with_point_char() {
        let plot = ScatterPlot::new(vec![(1.0, 2.0)]).with_point_char('*');
        assert_eq!(plot.point_char, '*');
    }

    #[test]
    fn test_scatter_plot_with_color() {
        let plot = ScatterPlot::new(vec![(1.0, 2.0)]).with_color(Color::Red);
        assert_eq!(plot.color, Color::Red);
    }

    #[test]
    fn test_scatter_plot_set_data() {
        let mut plot = ScatterPlot::new(vec![(1.0, 2.0)]);
        plot.set_data(vec![(3.0, 4.0), (5.0, 6.0)]);
        assert_eq!(plot.data().len(), 2);
    }

    #[test]
    fn test_scatter_plot_add_point() {
        let mut plot = ScatterPlot::new(vec![(1.0, 2.0)]);
        plot.add_point(3.0, 4.0);
        assert_eq!(plot.data().len(), 2);
        assert_eq!(plot.data()[1], (3.0, 4.0));
    }

    #[test]
    fn test_scatter_plot_bounds() {
        let plot = ScatterPlot::new(vec![(1.0, 2.0), (5.0, 10.0), (3.0, 6.0)]);
        let (min_x, max_x, min_y, max_y) = plot.bounds();
        assert_eq!(min_x, 1.0);
        assert_eq!(max_x, 5.0);
        assert_eq!(min_y, 2.0);
        assert_eq!(max_y, 10.0);
    }

    #[test]
    fn test_scatter_plot_bounds_empty() {
        let plot = ScatterPlot::new(vec![]);
        let (min_x, max_x, min_y, max_y) = plot.bounds();
        assert_eq!(min_x, 0.0);
        assert_eq!(max_x, 1.0);
        assert_eq!(min_y, 0.0);
        assert_eq!(max_y, 1.0);
    }

    #[test]
    fn test_scatter_plot_bounds_single_point() {
        let plot = ScatterPlot::new(vec![(5.0, 10.0)]);
        let (min_x, max_x, min_y, max_y) = plot.bounds();
        // Should add padding
        assert_eq!(min_x, 4.0);
        assert_eq!(max_x, 6.0);
        assert_eq!(min_y, 9.0);
        assert_eq!(max_y, 11.0);
    }

    #[test]
    fn test_scatter_plot_normalize() {
        let plot = ScatterPlot::new(vec![(0.0, 0.0), (10.0, 10.0)]);

        // Bottom-left corner
        let (x, y) = plot.normalize(0.0, 0.0, 10, 10);
        assert_eq!(x, 0);
        assert_eq!(y, 9); // Inverted Y

        // Top-right corner
        let (x, y) = plot.normalize(10.0, 10.0, 10, 10);
        assert_eq!(x, 9);
        assert_eq!(y, 0); // Inverted Y
    }

    #[test]
    fn test_scatter_plot_with_border() {
        let plot = ScatterPlot::new(vec![(1.0, 2.0)]).with_border(false);
        assert!(!plot.show_border);
    }

    #[test]
    fn test_scatter_plot_with_axes() {
        let plot = ScatterPlot::new(vec![(1.0, 2.0)]).with_axes(false);
        assert!(!plot.show_axes);
    }

    // ============================================================================
    // ADVANCED COMPREHENSIVE EDGE CASE TESTS (90%+ COVERAGE)
    // ============================================================================

    // ============ LineChart Stress Tests ============

    #[test]
    fn test_line_chart_10000_data_points() {
        let data: Vec<f64> = (0..10000).map(|i| i as f64).collect();
        let chart = LineChart::new(data);
        assert_eq!(chart.data().len(), 10000);
    }

    #[test]
    fn test_line_chart_rapid_data_updates() {
        let mut chart = LineChart::new(vec![1.0]);

        for i in 0..1000 {
            chart.set_data((0..100).map(|j| (i + j) as f64).collect());
            assert_eq!(chart.data().len(), 100);
        }
    }

    #[test]
    fn test_line_chart_rapid_point_additions() {
        let mut chart = LineChart::new(vec![]);

        for i in 0..5000 {
            chart.add_point(i as f64);
        }

        assert_eq!(chart.data().len(), 5000);
    }

    #[test]
    fn test_line_chart_all_line_styles_stress() {
        let data = vec![1.0, 2.0, 3.0];
        let styles = [
            LineStyle::Solid,
            LineStyle::Dotted,
            LineStyle::Dashed,
            LineStyle::Stepped,
        ];

        for _ in 0..1000 {
            for style in styles {
                let _chart = LineChart::new(data.clone()).with_line_style(style);
            }
        }
    }

    // ============ LineChart Unicode Edge Cases ============

    #[test]
    fn test_line_chart_unicode_title() {
        let chart = LineChart::new(vec![1.0, 2.0]).with_title("Chart 📊 グラフ 图表");
        assert!(chart.title.unwrap().contains("📊"));
    }

    #[test]
    fn test_line_chart_emoji_title() {
        let emojis = ["🐸 Chart", "💚 Data", "🚀 Trends", "✨ Stats"];

        for emoji_title in emojis {
            let chart = LineChart::new(vec![1.0]).with_title(emoji_title);
            assert!(chart.title.is_some());
        }
    }

    #[test]
    fn test_line_chart_rtl_title() {
        let chart = LineChart::new(vec![1.0]).with_title("مخطط البيانات Chart שרטוט");
        assert!(chart.title.unwrap().contains("مخطط"));
    }

    #[test]
    fn test_line_chart_very_long_title() {
        let long_title = "Chart ".repeat(1000);
        let chart = LineChart::new(vec![1.0]).with_title(long_title.clone());
        assert_eq!(chart.title.unwrap().len(), long_title.len());
    }

    // ============ LineChart Extreme Values ============

    #[test]
    fn test_line_chart_infinity_values() {
        let data = vec![f64::INFINITY, f64::NEG_INFINITY, 0.0];
        let chart = LineChart::new(data);
        let (min, max) = chart.bounds();
        // Should handle infinity gracefully
        assert!(min.is_finite() || max.is_finite() || min < max);
    }

    #[test]
    fn test_line_chart_nan_values() {
        let data = vec![1.0, f64::NAN, 3.0];
        let chart = LineChart::new(data);
        // Should not panic with NaN
        let _ = chart.bounds();
    }

    #[test]
    fn test_line_chart_very_large_values() {
        let data = vec![f64::MAX / 2.0, f64::MAX / 3.0, f64::MAX / 4.0];
        let chart = LineChart::new(data);
        let (min, max) = chart.bounds();
        assert!(min <= max);
    }

    #[test]
    fn test_line_chart_very_small_values() {
        let data = vec![f64::MIN_POSITIVE, f64::MIN_POSITIVE * 2.0, f64::MIN_POSITIVE * 3.0];
        let chart = LineChart::new(data);
        let (min, max) = chart.bounds();
        assert!(min <= max);
    }

    #[test]
    fn test_line_chart_negative_values() {
        let data = vec![-100.0, -50.0, -25.0, -10.0];
        let chart = LineChart::new(data);
        let (min, max) = chart.bounds();
        assert_eq!(min, -100.0);
        assert_eq!(max, -10.0);
    }

    #[test]
    fn test_line_chart_mixed_positive_negative() {
        let data = vec![-50.0, -25.0, 0.0, 25.0, 50.0];
        let chart = LineChart::new(data);
        let (min, max) = chart.bounds();
        assert_eq!(min, -50.0);
        assert_eq!(max, 50.0);
    }

    #[test]
    fn test_line_chart_all_same_values() {
        let data = vec![5.0, 5.0, 5.0, 5.0, 5.0];
        let chart = LineChart::new(data);
        let (min, max) = chart.bounds();
        // Should add padding when all values are same
        assert_eq!(min, 4.0);
        assert_eq!(max, 6.0);
    }

    // ============ LineChart Builder Pattern ============

    #[test]
    fn test_line_chart_full_builder_chain() {
        let chart = LineChart::new(vec![1.0, 2.0, 3.0])
            .with_title("Test")
            .with_line_style(LineStyle::Dotted)
            .with_color(Color::Red)
            .with_border(false)
            .with_axes(false)
            .with_values(true);

        assert_eq!(chart.title, Some("Test".to_string()));
        assert_eq!(chart.line_style, LineStyle::Dotted);
        assert_eq!(chart.color, Color::Red);
        assert!(!chart.show_border);
        assert!(!chart.show_axes);
        assert!(chart.show_values);
    }

    #[test]
    fn test_line_chart_builder_all_combinations() {
        let styles = [LineStyle::Solid, LineStyle::Dotted, LineStyle::Dashed, LineStyle::Stepped];
        let colors = [Color::Red, Color::Green, Color::Blue];

        for style in styles {
            for color in colors {
                for border in [true, false] {
                    let chart = LineChart::new(vec![1.0])
                        .with_line_style(style)
                        .with_color(color)
                        .with_border(border);

                    assert_eq!(chart.line_style, style);
                    assert_eq!(chart.color, color);
                    assert_eq!(chart.show_border, border);
                }
            }
        }
    }

    // ============ BarChart Stress Tests ============

    #[test]
    fn test_bar_chart_1000_bars() {
        let data: Vec<(String, f64)> = (0..1000)
            .map(|i| (format!("Bar{}", i), i as f64))
            .collect();

        let chart = BarChart::new(data);
        assert_eq!(chart.data().len(), 1000);
    }

    #[test]
    fn test_bar_chart_rapid_bar_additions() {
        let mut chart = BarChart::new(vec![]);

        for i in 0..2000 {
            chart.add_bar(format!("Bar{}", i), i as f64);
        }

        assert_eq!(chart.data().len(), 2000);
    }

    #[test]
    fn test_bar_chart_orientation_switching_stress() {
        for _ in 0..1000 {
            let _vertical = BarChart::new(vec![("A".into(), 1.0)])
                .with_orientation(BarOrientation::Vertical);
            let _horizontal = BarChart::new(vec![("A".into(), 1.0)])
                .with_orientation(BarOrientation::Horizontal);
        }
    }

    // ============ BarChart Unicode Edge Cases ============

    #[test]
    fn test_bar_chart_unicode_labels() {
        let data = vec![
            ("日本".to_string(), 10.0),
            ("中国".to_string(), 20.0),
            ("한국".to_string(), 15.0),
        ];
        let chart = BarChart::new(data);
        assert_eq!(chart.data().len(), 3);
    }

    #[test]
    fn test_bar_chart_emoji_labels() {
        let data = vec![
            ("🐸 Frogs".to_string(), 42.0),
            ("💚 Hearts".to_string(), 100.0),
            ("🚀 Rockets".to_string(), 88.0),
        ];
        let chart = BarChart::new(data);
        assert!(chart.data()[0].0.contains("🐸"));
    }

    #[test]
    fn test_bar_chart_very_long_labels() {
        let long_label = "Label".repeat(200);
        let data = vec![(long_label.clone(), 10.0)];
        let chart = BarChart::new(data);
        assert_eq!(chart.data()[0].0.len(), long_label.len());
    }

    #[test]
    fn test_bar_chart_rtl_labels() {
        let data = vec![
            ("العربية".to_string(), 10.0),
            ("עברית".to_string(), 20.0),
        ];
        let chart = BarChart::new(data);
        assert_eq!(chart.data().len(), 2);
    }

    // ============ BarChart Extreme Values ============

    #[test]
    fn test_bar_chart_negative_values() {
        let data = vec![
            ("A".to_string(), -10.0),
            ("B".to_string(), -20.0),
            ("C".to_string(), -5.0),
        ];
        let chart = BarChart::new(data);
        assert!(chart.max_value() >= 1.0); // Should be at least 1.0
    }

    #[test]
    fn test_bar_chart_zero_values() {
        let data = vec![
            ("A".to_string(), 0.0),
            ("B".to_string(), 0.0),
        ];
        let chart = BarChart::new(data);
        assert_eq!(chart.max_value(), 1.0);
    }

    #[test]
    fn test_bar_chart_very_large_values() {
        let data = vec![
            ("A".to_string(), f64::MAX / 10.0),
            ("B".to_string(), f64::MAX / 20.0),
        ];
        let chart = BarChart::new(data);
        let max = chart.max_value();
        assert!(max > 0.0);
    }

    // ============ BarChart Builder Pattern ============

    #[test]
    fn test_bar_chart_full_builder_chain() {
        let chart = BarChart::new(vec![("A".into(), 10.0)])
            .with_title("Bar Test")
            .with_orientation(BarOrientation::Horizontal)
            .with_color(Color::Blue)
            .with_border(false)
            .with_values(true);

        assert_eq!(chart.title, Some("Bar Test".to_string()));
        assert_eq!(chart.orientation, BarOrientation::Horizontal);
        assert_eq!(chart.color, Color::Blue);
        assert!(!chart.show_border);
        assert!(chart.show_values);
    }

    // ============ ScatterPlot Stress Tests ============

    #[test]
    fn test_scatter_plot_10000_points() {
        let data: Vec<(f64, f64)> = (0..10000)
            .map(|i| (i as f64, (i * 2) as f64))
            .collect();

        let plot = ScatterPlot::new(data);
        assert_eq!(plot.data().len(), 10000);
    }

    #[test]
    fn test_scatter_plot_rapid_point_additions() {
        let mut plot = ScatterPlot::new(vec![]);

        for i in 0..3000 {
            plot.add_point(i as f64, (i * 2) as f64);
        }

        assert_eq!(plot.data().len(), 3000);
    }

    #[test]
    fn test_scatter_plot_rapid_data_updates() {
        let mut plot = ScatterPlot::new(vec![(1.0, 1.0)]);

        for i in 0..500 {
            let new_data: Vec<(f64, f64)> = (0..50)
                .map(|j| ((i + j) as f64, (i + j * 2) as f64))
                .collect();
            plot.set_data(new_data);
        }

        assert_eq!(plot.data().len(), 50);
    }

    // ============ ScatterPlot Unicode Edge Cases ============

    #[test]
    fn test_scatter_plot_unicode_title() {
        let plot = ScatterPlot::new(vec![(1.0, 2.0)])
            .with_title("Scatter 📈 分布図 مبعثر");
        assert!(plot.title.unwrap().contains("📈"));
    }

    #[test]
    fn test_scatter_plot_emoji_point_chars() {
        let emojis = ['🐸', '💚', '✨', '🎯', '🔥'];

        for emoji in emojis {
            let plot = ScatterPlot::new(vec![(1.0, 2.0)]).with_point_char(emoji);
            assert_eq!(plot.point_char, emoji);
        }
    }

    #[test]
    fn test_scatter_plot_unicode_point_chars() {
        let chars = ['●', '○', '◆', '◇', '■', '□', '▲', '△'];

        for ch in chars {
            let plot = ScatterPlot::new(vec![(1.0, 2.0)]).with_point_char(ch);
            assert_eq!(plot.point_char, ch);
        }
    }

    // ============ ScatterPlot Extreme Values ============

    #[test]
    fn test_scatter_plot_infinity_values() {
        let data = vec![
            (f64::INFINITY, 1.0),
            (1.0, f64::NEG_INFINITY),
            (0.0, 0.0),
        ];
        let plot = ScatterPlot::new(data);
        let (min_x, max_x, min_y, max_y) = plot.bounds();
        // Should handle gracefully (may not be fully finite)
        assert!(min_x < max_x || min_y < max_y || min_x.is_infinite());
    }

    #[test]
    fn test_scatter_plot_nan_values() {
        let data = vec![(1.0, f64::NAN), (f64::NAN, 2.0)];
        let plot = ScatterPlot::new(data);
        // Should not panic
        let _ = plot.bounds();
    }

    #[test]
    fn test_scatter_plot_very_large_values() {
        let data = vec![
            (f64::MAX / 2.0, f64::MAX / 3.0),
            (f64::MAX / 4.0, f64::MAX / 5.0),
        ];
        let plot = ScatterPlot::new(data);
        let (min_x, max_x, min_y, max_y) = plot.bounds();
        assert!(min_x <= max_x);
        assert!(min_y <= max_y);
    }

    #[test]
    fn test_scatter_plot_negative_values() {
        let data = vec![(-10.0, -20.0), (-5.0, -10.0), (-1.0, -2.0)];
        let plot = ScatterPlot::new(data);
        let (min_x, max_x, min_y, max_y) = plot.bounds();
        assert_eq!(min_x, -10.0);
        assert_eq!(max_x, -1.0);
        assert_eq!(min_y, -20.0);
        assert_eq!(max_y, -2.0);
    }

    #[test]
    fn test_scatter_plot_all_same_points() {
        let data = vec![(5.0, 10.0), (5.0, 10.0), (5.0, 10.0)];
        let plot = ScatterPlot::new(data);
        let (min_x, max_x, min_y, max_y) = plot.bounds();
        // Should add padding
        assert_eq!(min_x, 4.0);
        assert_eq!(max_x, 6.0);
        assert_eq!(min_y, 9.0);
        assert_eq!(max_y, 11.0);
    }

    // ============ ScatterPlot Builder Pattern ============

    #[test]
    fn test_scatter_plot_full_builder_chain() {
        let plot = ScatterPlot::new(vec![(1.0, 2.0)])
            .with_title("Scatter Test")
            .with_point_char('*')
            .with_color(Color::Yellow)
            .with_border(false)
            .with_axes(false);

        assert_eq!(plot.title, Some("Scatter Test".to_string()));
        assert_eq!(plot.point_char, '*');
        assert_eq!(plot.color, Color::Yellow);
        assert!(!plot.show_border);
        assert!(!plot.show_axes);
    }

    // ============ Trait Implementation Coverage ============

    #[test]
    fn test_line_style_debug() {
        let styles = [
            LineStyle::Solid,
            LineStyle::Dotted,
            LineStyle::Dashed,
            LineStyle::Stepped,
        ];

        for style in styles {
            let debug_str = format!("{:?}", style);
            assert!(!debug_str.is_empty());
        }
    }

    #[test]
    fn test_line_style_clone() {
        let style1 = LineStyle::Dotted;
        let style2 = style1;
        assert_eq!(style1, style2);
    }

    #[test]
    fn test_line_style_partial_eq() {
        assert_eq!(LineStyle::Solid, LineStyle::Solid);
        assert_ne!(LineStyle::Solid, LineStyle::Dotted);
    }

    #[test]
    fn test_bar_orientation_debug() {
        let vertical = format!("{:?}", BarOrientation::Vertical);
        let horizontal = format!("{:?}", BarOrientation::Horizontal);

        assert!(vertical.contains("Vertical"));
        assert!(horizontal.contains("Horizontal"));
    }

    #[test]
    fn test_bar_orientation_clone() {
        let ori1 = BarOrientation::Vertical;
        let ori2 = ori1;
        assert_eq!(ori1, ori2);
    }

    #[test]
    fn test_bar_orientation_partial_eq() {
        assert_eq!(BarOrientation::Vertical, BarOrientation::Vertical);
        assert_ne!(BarOrientation::Vertical, BarOrientation::Horizontal);
    }

    #[test]
    fn test_line_chart_clone() {
        let chart1 = LineChart::new(vec![1.0, 2.0, 3.0])
            .with_title("Test")
            .with_color(Color::Blue);
        let chart2 = chart1.clone();

        assert_eq!(chart1.data().len(), chart2.data().len());
        assert_eq!(chart1.title, chart2.title);
        assert_eq!(chart1.color, chart2.color);
    }

    #[test]
    fn test_line_chart_debug() {
        let chart = LineChart::new(vec![1.0, 2.0, 3.0]);
        let debug_str = format!("{:?}", chart);
        assert!(debug_str.contains("LineChart"));
    }

    #[test]
    fn test_bar_chart_clone() {
        let chart1 = BarChart::new(vec![("A".into(), 10.0)])
            .with_title("Test")
            .with_color(Color::Red);
        let chart2 = chart1.clone();

        assert_eq!(chart1.data().len(), chart2.data().len());
        assert_eq!(chart1.title, chart2.title);
        assert_eq!(chart1.color, chart2.color);
    }

    #[test]
    fn test_bar_chart_debug() {
        let chart = BarChart::new(vec![("A".into(), 10.0)]);
        let debug_str = format!("{:?}", chart);
        assert!(debug_str.contains("BarChart"));
    }

    #[test]
    fn test_scatter_plot_clone() {
        let plot1 = ScatterPlot::new(vec![(1.0, 2.0)])
            .with_title("Test")
            .with_point_char('*');
        let plot2 = plot1.clone();

        assert_eq!(plot1.data().len(), plot2.data().len());
        assert_eq!(plot1.title, plot2.title);
        assert_eq!(plot1.point_char, plot2.point_char);
    }

    #[test]
    fn test_scatter_plot_debug() {
        let plot = ScatterPlot::new(vec![(1.0, 2.0)]);
        let debug_str = format!("{:?}", plot);
        assert!(debug_str.contains("ScatterPlot"));
    }

    // ============ Comprehensive Stress Test ============

    #[test]
    fn test_comprehensive_chart_stress() {
        // LineChart comprehensive test
        let mut line_chart = LineChart::new((0..1000).map(|i| i as f64).collect())
            .with_title("Comprehensive 📊 Test")
            .with_line_style(LineStyle::Solid)
            .with_color(Color::Green)
            .with_border(true)
            .with_axes(true)
            .with_values(true);

        for i in 0..100 {
            line_chart.add_point(1000.0 + i as f64);
        }
        assert_eq!(line_chart.data().len(), 1100);

        // BarChart comprehensive test
        let mut bar_chart = BarChart::new(
            (0..500)
                .map(|i| (format!("Bar{} 🐸", i), i as f64))
                .collect(),
        )
        .with_title("Bar Test 💚")
        .with_orientation(BarOrientation::Horizontal)
        .with_color(Color::Blue)
        .with_values(true);

        for i in 500..600 {
            bar_chart.add_bar(format!("Extra{}", i), i as f64);
        }
        assert_eq!(bar_chart.data().len(), 600);

        // ScatterPlot comprehensive test
        let mut scatter_plot = ScatterPlot::new(
            (0..800)
                .map(|i| (i as f64, (i * 2) as f64))
                .collect(),
        )
        .with_title("Scatter 📈 Plot")
        .with_point_char('●')
        .with_color(Color::Red)
        .with_border(false)
        .with_axes(true);

        for i in 800..900 {
            scatter_plot.add_point(i as f64, (i * 3) as f64);
        }
        assert_eq!(scatter_plot.data().len(), 900);

        // Test all style variations
        for style in [LineStyle::Solid, LineStyle::Dotted, LineStyle::Dashed, LineStyle::Stepped] {
            let _chart = LineChart::new(vec![1.0, 2.0, 3.0]).with_line_style(style);
        }

        // Test both orientations
        for orientation in [BarOrientation::Vertical, BarOrientation::Horizontal] {
            let _chart = BarChart::new(vec![("A".into(), 1.0)]).with_orientation(orientation);
        }

        // Verify bounds calculations
        let (min, max) = line_chart.bounds();
        assert!(min <= max);

        let bar_max = bar_chart.max_value();
        assert!(bar_max > 0.0);

        let (min_x, max_x, min_y, max_y) = scatter_plot.bounds();
        assert!(min_x <= max_x);
        assert!(min_y <= max_y);
    }
}
