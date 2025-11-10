//! Canvas drawing widget for custom graphics
//!
//! Provides primitives for drawing custom charts, diagrams, and graphics
//! with precise coordinate control.
//!
//! # Examples
//!
//! ```
//! use toad::widgets::Canvas;
//! use ratatui::style::Color;
//!
//! let mut canvas = Canvas::new()
//!     .with_x_bounds(-10.0, 10.0)
//!     .with_y_bounds(-5.0, 5.0);
//!
//! // Draw a line
//! canvas.line(-5.0, 0.0, 5.0, 0.0, Color::Red);
//!
//! // Draw a rectangle
//! canvas.rectangle(-2.0, -1.0, 2.0, 1.0, Color::Blue);
//!
//! // Draw a circle
//! canvas.circle(0.0, 0.0, 3.0, Color::Green);
//! ```

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Color,
    text::Line,
    widgets::{Block, Borders, Widget},
};

/// Shape to draw on canvas
#[derive(Debug, Clone)]
pub enum Shape {
    /// Line from (x1, y1) to (x2, y2)
    Line {
        /// Start X coordinate
        x1: f64,
        /// Start Y coordinate
        y1: f64,
        /// End X coordinate
        x2: f64,
        /// End Y coordinate
        y2: f64,
        /// Line color
        color: Color,
    },
    /// Rectangle with corners (x1, y1) and (x2, y2)
    Rectangle {
        /// Top-left X coordinate
        x1: f64,
        /// Top-left Y coordinate
        y1: f64,
        /// Bottom-right X coordinate
        x2: f64,
        /// Bottom-right Y coordinate
        y2: f64,
        /// Rectangle color
        color: Color,
        /// Whether to fill the rectangle
        filled: bool,
    },
    /// Circle at (cx, cy) with radius
    Circle {
        /// Center X coordinate
        cx: f64,
        /// Center Y coordinate
        cy: f64,
        /// Circle radius
        radius: f64,
        /// Circle color
        color: Color,
        /// Whether to fill the circle
        filled: bool,
    },
    /// Point at (x, y)
    Point {
        /// X coordinate
        x: f64,
        /// Y coordinate
        y: f64,
        /// Point color
        color: Color,
        /// Point marker character
        marker: char,
    },
}

/// Canvas widget for custom graphics
///
/// Provides a coordinate system and drawing primitives for creating
/// custom charts, diagrams, and visualizations.
///
/// # Examples
///
/// ```
/// use toad::widgets::Canvas;
/// use ratatui::style::Color;
///
/// let mut canvas = Canvas::new()
///     .with_title("My Drawing")
///     .with_x_bounds(0.0, 100.0)
///     .with_y_bounds(0.0, 50.0);
///
/// // Add shapes
/// canvas.line(10.0, 10.0, 90.0, 40.0, Color::Cyan);
/// canvas.circle(50.0, 25.0, 10.0, Color::Yellow);
/// ```
#[derive(Debug, Clone)]
pub struct Canvas {
    /// Shapes to draw
    pub(super) shapes: Vec<Shape>,
    /// Canvas title
    pub(super) title: Option<String>,
    /// X-axis bounds (min, max)
    pub(super) x_bounds: (f64, f64),
    /// Y-axis bounds (min, max)
    pub(super) y_bounds: (f64, f64),
    /// Show coordinate grid
    pub(super) show_grid: bool,
    /// Show axes
    pub(super) show_axes: bool,
}

impl Default for Canvas {
    fn default() -> Self {
        Self::new()
    }
}

impl Canvas {
    /// Create a new canvas
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Canvas;
    ///
    /// let canvas = Canvas::new();
    /// assert_eq!(canvas.shape_count(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            shapes: Vec::new(),
            title: None,
            x_bounds: (0.0, 100.0),
            y_bounds: (0.0, 100.0),
            show_grid: false,
            show_axes: true,
        }
    }

    /// Set canvas title
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Canvas;
    ///
    /// let canvas = Canvas::new().with_title("Chart");
    /// ```
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set X-axis bounds
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Canvas;
    ///
    /// let canvas = Canvas::new().with_x_bounds(-10.0, 10.0);
    /// ```
    pub fn with_x_bounds(mut self, min: f64, max: f64) -> Self {
        self.x_bounds = (min, max);
        self
    }

    /// Set Y-axis bounds
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Canvas;
    ///
    /// let canvas = Canvas::new().with_y_bounds(0.0, 50.0);
    /// ```
    pub fn with_y_bounds(mut self, min: f64, max: f64) -> Self {
        self.y_bounds = (min, max);
        self
    }

    /// Show or hide coordinate grid
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Canvas;
    ///
    /// let canvas = Canvas::new().with_grid(true);
    /// ```
    pub fn with_grid(mut self, show: bool) -> Self {
        self.show_grid = show;
        self
    }

    /// Show or hide axes
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Canvas;
    ///
    /// let canvas = Canvas::new().with_axes(false);
    /// ```
    pub fn with_axes(mut self, show: bool) -> Self {
        self.show_axes = show;
        self
    }

    /// Draw a line
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Canvas;
    /// use ratatui::style::Color;
    ///
    /// let mut canvas = Canvas::new();
    /// canvas.line(0.0, 0.0, 10.0, 10.0, Color::Red);
    /// assert_eq!(canvas.shape_count(), 1);
    /// ```
    pub fn line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, color: Color) {
        self.shapes.push(Shape::Line {
            x1,
            y1,
            x2,
            y2,
            color,
        });
    }

    /// Draw a rectangle
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Canvas;
    /// use ratatui::style::Color;
    ///
    /// let mut canvas = Canvas::new();
    /// canvas.rectangle(5.0, 5.0, 15.0, 15.0, Color::Blue);
    /// assert_eq!(canvas.shape_count(), 1);
    /// ```
    pub fn rectangle(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, color: Color) {
        self.shapes.push(Shape::Rectangle {
            x1,
            y1,
            x2,
            y2,
            color,
            filled: false,
        });
    }

    /// Draw a filled rectangle
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Canvas;
    /// use ratatui::style::Color;
    ///
    /// let mut canvas = Canvas::new();
    /// canvas.filled_rectangle(5.0, 5.0, 15.0, 15.0, Color::Green);
    /// ```
    pub fn filled_rectangle(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, color: Color) {
        self.shapes.push(Shape::Rectangle {
            x1,
            y1,
            x2,
            y2,
            color,
            filled: true,
        });
    }

    /// Draw a circle
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Canvas;
    /// use ratatui::style::Color;
    ///
    /// let mut canvas = Canvas::new();
    /// canvas.circle(50.0, 50.0, 10.0, Color::Yellow);
    /// assert_eq!(canvas.shape_count(), 1);
    /// ```
    pub fn circle(&mut self, cx: f64, cy: f64, radius: f64, color: Color) {
        self.shapes.push(Shape::Circle {
            cx,
            cy,
            radius,
            color,
            filled: false,
        });
    }

    /// Draw a filled circle
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Canvas;
    /// use ratatui::style::Color;
    ///
    /// let mut canvas = Canvas::new();
    /// canvas.filled_circle(50.0, 50.0, 10.0, Color::Magenta);
    /// ```
    pub fn filled_circle(&mut self, cx: f64, cy: f64, radius: f64, color: Color) {
        self.shapes.push(Shape::Circle {
            cx,
            cy,
            radius,
            color,
            filled: true,
        });
    }

    /// Draw a point
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Canvas;
    /// use ratatui::style::Color;
    ///
    /// let mut canvas = Canvas::new();
    /// canvas.point(25.0, 25.0, Color::White, '●');
    /// assert_eq!(canvas.shape_count(), 1);
    /// ```
    pub fn point(&mut self, x: f64, y: f64, color: Color, marker: char) {
        self.shapes.push(Shape::Point {
            x,
            y,
            color,
            marker,
        });
    }

    /// Clear all shapes
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Canvas;
    /// use ratatui::style::Color;
    ///
    /// let mut canvas = Canvas::new();
    /// canvas.line(0.0, 0.0, 10.0, 10.0, Color::Red);
    /// assert_eq!(canvas.shape_count(), 1);
    /// canvas.clear();
    /// assert_eq!(canvas.shape_count(), 0);
    /// ```
    pub fn clear(&mut self) {
        self.shapes.clear();
    }

    /// Get number of shapes
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Canvas;
    /// use ratatui::style::Color;
    ///
    /// let mut canvas = Canvas::new();
    /// assert_eq!(canvas.shape_count(), 0);
    /// canvas.circle(0.0, 0.0, 5.0, Color::Red);
    /// assert_eq!(canvas.shape_count(), 1);
    /// ```
    pub fn shape_count(&self) -> usize {
        self.shapes.len()
    }

    /// Convert world coordinates to screen coordinates
    pub(super) fn world_to_screen(&self, x: f64, y: f64, width: u16, height: u16) -> (u16, u16) {
        let x_range = self.x_bounds.1 - self.x_bounds.0;
        let y_range = self.y_bounds.1 - self.y_bounds.0;

        let screen_x = ((x - self.x_bounds.0) / x_range * width as f64) as u16;
        let screen_y = ((self.y_bounds.1 - y) / y_range * height as f64) as u16;

        (
            screen_x.min(width.saturating_sub(1)),
            screen_y.min(height.saturating_sub(1)),
        )
    }

    /// Render canvas to text lines
    pub fn render_lines(&self, width: u16, height: u16) -> Vec<Line<'static>> {
        let mut lines = Vec::new();

        // Title
        if let Some(title) = &self.title {
            lines.push(Line::from(title.clone()));
        }

        // Create a character grid for rendering
        let mut grid = vec![vec![' '; width as usize]; height as usize];

        // Draw grid if enabled
        #[allow(clippy::needless_range_loop)]
        if self.show_grid {
            for y in 0..height as usize {
                for x in 0..width as usize {
                    if x % 10 == 0 || y % 5 == 0 {
                        grid[y][x] = '·';
                    }
                }
            }
        }

        // Draw axes if enabled
        if self.show_axes {
            // Y-axis
            if self.x_bounds.0 <= 0.0 && self.x_bounds.1 >= 0.0 {
                let (x, _) = self.world_to_screen(0.0, 0.0, width, height);
                let x = x as usize;
                for row in grid.iter_mut().take(height as usize) {
                    row[x] = '│';
                }
            }

            // X-axis
            if self.y_bounds.0 <= 0.0 && self.y_bounds.1 >= 0.0 {
                let (_, y) = self.world_to_screen(0.0, 0.0, width, height);
                let y = y as usize;
                if y < grid.len() {
                    for cell in grid[y].iter_mut().take(width as usize) {
                        *cell = '─';
                    }
                }
            }
        }

        // Draw shapes
        for shape in &self.shapes {
            match shape {
                Shape::Line { x1, y1, x2, y2, .. } => {
                    self.draw_line(&mut grid, *x1, *y1, *x2, *y2, width, height);
                }
                Shape::Rectangle {
                    x1,
                    y1,
                    x2,
                    y2,
                    filled,
                    ..
                } => {
                    if *filled {
                        self.draw_filled_rectangle(&mut grid, *x1, *y1, *x2, *y2, width, height);
                    } else {
                        self.draw_rectangle(&mut grid, *x1, *y1, *x2, *y2, width, height);
                    }
                }
                Shape::Circle {
                    cx,
                    cy,
                    radius,
                    filled,
                    ..
                } => {
                    if *filled {
                        self.draw_filled_circle(&mut grid, *cx, *cy, *radius, width, height);
                    } else {
                        self.draw_circle(&mut grid, *cx, *cy, *radius, width, height);
                    }
                }
                Shape::Point { x, y, marker, .. } => {
                    let (sx, sy) = self.world_to_screen(*x, *y, width, height);
                    if sy < height && sx < width {
                        grid[sy as usize][sx as usize] = *marker;
                    }
                }
            }
        }

        // Convert grid to lines
        for row in grid {
            lines.push(Line::from(row.iter().collect::<String>()));
        }

        lines
    }

    /// Draw a line using Bresenham's algorithm
    #[allow(clippy::too_many_arguments)]
    fn draw_line(
        &self,
        grid: &mut [Vec<char>],
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        width: u16,
        height: u16,
    ) {
        let (mut sx1, mut sy1) = self.world_to_screen(x1, y1, width, height);
        let (sx2, sy2) = self.world_to_screen(x2, y2, width, height);

        let dx = (sx2 as i32 - sx1 as i32).abs();
        let dy = -(sy2 as i32 - sy1 as i32).abs();
        let sx = if sx1 < sx2 { 1 } else { -1 };
        let sy = if sy1 < sy2 { 1 } else { -1 };
        let mut err = dx + dy;

        loop {
            if sy1 < height && sx1 < width {
                grid[sy1 as usize][sx1 as usize] = '●';
            }

            if sx1 == sx2 && sy1 == sy2 {
                break;
            }

            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                sx1 = (sx1 as i32 + sx) as u16;
            }
            if e2 <= dx {
                err += dx;
                sy1 = (sy1 as i32 + sy) as u16;
            }
        }
    }

    /// Draw a rectangle outline
    #[allow(clippy::too_many_arguments)]
    fn draw_rectangle(
        &self,
        grid: &mut [Vec<char>],
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        width: u16,
        height: u16,
    ) {
        let (sx1, sy1) = self.world_to_screen(x1, y1, width, height);
        let (sx2, sy2) = self.world_to_screen(x2, y2, width, height);

        let min_x = sx1.min(sx2) as usize;
        let max_x = sx1.max(sx2) as usize;
        let min_y = sy1.min(sy2) as usize;
        let max_y = sy1.max(sy2) as usize;

        // Top and bottom
        #[allow(clippy::needless_range_loop)]
        for x in min_x..=max_x.min(width.saturating_sub(1) as usize) {
            if min_y < height as usize {
                grid[min_y][x] = '─';
            }
            if max_y < height as usize {
                grid[max_y][x] = '─';
            }
        }

        // Left and right
        #[allow(clippy::needless_range_loop)]
        for y in min_y..=max_y.min(height.saturating_sub(1) as usize) {
            if min_x < width as usize {
                grid[y][min_x] = '│';
            }
            if max_x < width as usize {
                grid[y][max_x] = '│';
            }
        }

        // Corners
        if min_y < height as usize && min_x < width as usize {
            grid[min_y][min_x] = '┌';
        }
        if min_y < height as usize && max_x < width as usize {
            grid[min_y][max_x] = '┐';
        }
        if max_y < height as usize && min_x < width as usize {
            grid[max_y][min_x] = '└';
        }
        if max_y < height as usize && max_x < width as usize {
            grid[max_y][max_x] = '┘';
        }
    }

    /// Draw a filled rectangle
    #[allow(clippy::too_many_arguments)]
    fn draw_filled_rectangle(
        &self,
        grid: &mut [Vec<char>],
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        width: u16,
        height: u16,
    ) {
        let (sx1, sy1) = self.world_to_screen(x1, y1, width, height);
        let (sx2, sy2) = self.world_to_screen(x2, y2, width, height);

        let min_x = sx1.min(sx2) as usize;
        let max_x = sx1.max(sx2) as usize;
        let min_y = sy1.min(sy2) as usize;
        let max_y = sy1.max(sy2) as usize;

        #[allow(clippy::needless_range_loop)]
        for y in min_y..=max_y.min(height.saturating_sub(1) as usize) {
            #[allow(clippy::needless_range_loop)]
            for x in min_x..=max_x.min(width.saturating_sub(1) as usize) {
                grid[y][x] = '█';
            }
        }
    }

    /// Draw a circle outline using midpoint circle algorithm
    #[allow(clippy::too_many_arguments)]
    fn draw_circle(
        &self,
        grid: &mut [Vec<char>],
        cx: f64,
        cy: f64,
        radius: f64,
        width: u16,
        height: u16,
    ) {
        let (scx, scy) = self.world_to_screen(cx, cy, width, height);
        let x_scale = width as f64 / (self.x_bounds.1 - self.x_bounds.0);
        let y_scale = height as f64 / (self.y_bounds.1 - self.y_bounds.0);
        let sr = ((radius * x_scale + radius * y_scale) / 2.0) as i32;

        let mut x = sr;
        let mut y = 0;
        let mut err = 0;

        while x >= y {
            self.plot_circle_points(grid, scx as i32, scy as i32, x, y, width, height);
            y += 1;
            err += 1 + 2 * y;
            if 2 * (err - x) + 1 > 0 {
                x -= 1;
                err += 1 - 2 * x;
            }
        }
    }

    /// Plot 8 symmetric points for circle
    #[allow(clippy::too_many_arguments)]
    fn plot_circle_points(
        &self,
        grid: &mut [Vec<char>],
        cx: i32,
        cy: i32,
        x: i32,
        y: i32,
        width: u16,
        height: u16,
    ) {
        let points = [
            (cx + x, cy + y),
            (cx - x, cy + y),
            (cx + x, cy - y),
            (cx - x, cy - y),
            (cx + y, cy + x),
            (cx - y, cy + x),
            (cx + y, cy - x),
            (cx - y, cy - x),
        ];

        for (px, py) in points {
            if px >= 0 && py >= 0 && (px as u16) < width && (py as u16) < height {
                grid[py as usize][px as usize] = '○';
            }
        }
    }

    /// Draw a filled circle
    #[allow(clippy::too_many_arguments)]
    fn draw_filled_circle(
        &self,
        grid: &mut [Vec<char>],
        cx: f64,
        cy: f64,
        radius: f64,
        width: u16,
        height: u16,
    ) {
        let (scx, scy) = self.world_to_screen(cx, cy, width, height);
        let x_scale = width as f64 / (self.x_bounds.1 - self.x_bounds.0);
        let y_scale = height as f64 / (self.y_bounds.1 - self.y_bounds.0);
        let sr = ((radius * x_scale + radius * y_scale) / 2.0) as i32;

        for dy in -sr..=sr {
            for dx in -sr..=sr {
                if dx * dx + dy * dy <= sr * sr {
                    let px = scx as i32 + dx;
                    let py = scy as i32 + dy;
                    if px >= 0 && py >= 0 && (px as u16) < width && (py as u16) < height {
                        grid[py as usize][px as usize] = '●';
                    }
                }
            }
        }
    }
}

impl Widget for &Canvas {
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

