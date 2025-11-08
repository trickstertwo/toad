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
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use serde::{Deserialize, Serialize};

use crate::theme::ToadTheme;

/// Line chart style
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LineStyle {
    /// Solid line
    Solid,
    /// Dotted line
    Dotted,
    /// Dashed line
    Dashed,
    /// Stepped line
    Stepped,
}

impl Default for LineStyle {
    fn default() -> Self {
        LineStyle::Solid
    }
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
            lines_to_render.push(Line::from(Span::styled(line, Style::default().fg(self.color))));
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
        let chart = LineChart::new(vec![1.0, 2.0, 3.0])
            .with_title("Test Chart");
        assert_eq!(chart.title, Some("Test Chart".to_string()));
    }

    #[test]
    fn test_line_chart_with_line_style() {
        let chart = LineChart::new(vec![1.0, 2.0, 3.0])
            .with_line_style(LineStyle::Dotted);
        assert_eq!(chart.line_style, LineStyle::Dotted);
    }

    #[test]
    fn test_line_chart_with_color() {
        let chart = LineChart::new(vec![1.0, 2.0, 3.0])
            .with_color(Color::Blue);
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
        let chart = LineChart::new(vec![1.0])
            .with_line_style(LineStyle::Solid);
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
        let chart = LineChart::new(vec![1.0, 2.0, 3.0])
            .with_border(false);
        assert!(!chart.show_border);
    }

    #[test]
    fn test_line_chart_with_axes() {
        let chart = LineChart::new(vec![1.0, 2.0, 3.0])
            .with_axes(false);
        assert!(!chart.show_axes);
    }

    #[test]
    fn test_line_chart_with_values() {
        let chart = LineChart::new(vec![1.0, 2.0, 3.0])
            .with_values(true);
        assert!(chart.show_values);
    }
}
