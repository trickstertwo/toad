/// Canvas drawing for custom graphics (charts, diagrams)
///
/// Provides a grid-based drawing surface with shape primitives
///
/// # Examples
///
/// ```
/// use toad::canvas::Canvas;
///
/// let canvas = Canvas::new(80, 24);
/// assert_eq!(canvas.width(), 80);
/// assert_eq!(canvas.height(), 24);
/// ```

use ratatui::style::Color;
use serde::{Deserialize, Serialize};

/// A pixel on the canvas
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pixel {
    /// Character to display
    pub ch: char,
    /// Foreground color
    pub fg: Color,
    /// Background color
    pub bg: Option<Color>,
}

impl Default for Pixel {
    fn default() -> Self {
        Self {
            ch: ' ',
            fg: Color::White,
            bg: None,
        }
    }
}

/// Drawing canvas
#[derive(Debug, Clone)]
pub struct Canvas {
    /// Width in characters
    width: usize,
    /// Height in characters
    height: usize,
    /// Pixel buffer
    buffer: Vec<Pixel>,
}

impl Canvas {
    /// Create a new canvas
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::canvas::Canvas;
    ///
    /// let canvas = Canvas::new(80, 24);
    /// assert_eq!(canvas.width(), 80);
    /// ```
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buffer: vec![Pixel::default(); width * height],
        }
    }

    /// Get width
    pub fn width(&self) -> usize {
        self.width
    }

    /// Get height
    pub fn height(&self) -> usize {
        self.height
    }

    /// Get pixel at position
    pub fn get(&self, x: usize, y: usize) -> Option<Pixel> {
        if x < self.width && y < self.height {
            Some(self.buffer[y * self.width + x])
        } else {
            None
        }
    }

    /// Set pixel at position
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::canvas::Canvas;
    /// use ratatui::style::Color;
    ///
    /// let mut canvas = Canvas::new(10, 10);
    /// canvas.set_pixel(5, 5, '*', Color::Red, None);
    /// ```
    pub fn set_pixel(&mut self, x: usize, y: usize, ch: char, fg: Color, bg: Option<Color>) {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x] = Pixel { ch, fg, bg };
        }
    }

    /// Clear the canvas
    pub fn clear(&mut self) {
        self.buffer.fill(Pixel::default());
    }

    /// Draw a line (Bresenham's algorithm)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::canvas::Canvas;
    /// use ratatui::style::Color;
    ///
    /// let mut canvas = Canvas::new(80, 24);
    /// canvas.line(0, 0, 79, 23, '─', Color::White);
    /// ```
    pub fn line(&mut self, x0: isize, y0: isize, x1: isize, y1: isize, ch: char, color: Color) {
        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        let mut x = x0;
        let mut y = y0;

        loop {
            if x >= 0 && x < self.width as isize && y >= 0 && y < self.height as isize {
                self.set_pixel(x as usize, y as usize, ch, color, None);
            }

            if x == x1 && y == y1 {
                break;
            }

            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }

    /// Draw a rectangle
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::canvas::Canvas;
    /// use ratatui::style::Color;
    ///
    /// let mut canvas = Canvas::new(80, 24);
    /// canvas.rectangle(10, 5, 30, 15, '█', Color::Blue, false);
    /// ```
    pub fn rectangle(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        ch: char,
        color: Color,
        filled: bool,
    ) {
        if filled {
            for dy in 0..height {
                for dx in 0..width {
                    self.set_pixel(x + dx, y + dy, ch, color, None);
                }
            }
        } else {
            // Top and bottom
            for dx in 0..width {
                self.set_pixel(x + dx, y, ch, color, None);
                self.set_pixel(x + dx, y + height - 1, ch, color, None);
            }
            // Left and right
            for dy in 0..height {
                self.set_pixel(x, y + dy, ch, color, None);
                self.set_pixel(x + width - 1, y + dy, ch, color, None);
            }
        }
    }

    /// Draw a circle (midpoint circle algorithm)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::canvas::Canvas;
    /// use ratatui::style::Color;
    ///
    /// let mut canvas = Canvas::new(80, 24);
    /// canvas.circle(40, 12, 10, '●', Color::Green, false);
    /// ```
    pub fn circle(
        &mut self,
        cx: isize,
        cy: isize,
        radius: isize,
        ch: char,
        color: Color,
        filled: bool,
    ) {
        if filled {
            for y in (cy - radius)..=(cy + radius) {
                for x in (cx - radius)..=(cx + radius) {
                    let dx = x - cx;
                    let dy = y - cy;
                    if dx * dx + dy * dy <= radius * radius {
                        if x >= 0
                            && x < self.width as isize
                            && y >= 0
                            && y < self.height as isize
                        {
                            self.set_pixel(x as usize, y as usize, ch, color, None);
                        }
                    }
                }
            }
        } else {
            let mut x = 0;
            let mut y = radius;
            let mut d = 3 - 2 * radius;

            while x <= y {
                // Draw 8 octants
                self.plot_circle_points(cx, cy, x, y, ch, color);

                x += 1;
                if d < 0 {
                    d = d + 4 * x + 6;
                } else {
                    y -= 1;
                    d = d + 4 * (x - y) + 10;
                }
            }
        }
    }

    /// Helper to plot 8 circle points
    fn plot_circle_points(&mut self, cx: isize, cy: isize, x: isize, y: isize, ch: char, color: Color) {
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
            if px >= 0 && px < self.width as isize && py >= 0 && py < self.height as isize {
                self.set_pixel(px as usize, py as usize, ch, color, None);
            }
        }
    }

    /// Draw text
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::canvas::Canvas;
    /// use ratatui::style::Color;
    ///
    /// let mut canvas = Canvas::new(80, 24);
    /// canvas.text(10, 5, "Hello, World!", Color::White);
    /// ```
    pub fn text(&mut self, x: usize, y: usize, text: &str, color: Color) {
        for (i, ch) in text.chars().enumerate() {
            self.set_pixel(x + i, y, ch, color, None);
        }
    }

    /// Get buffer as lines of strings
    pub fn to_lines(&self) -> Vec<String> {
        let mut lines = Vec::new();

        for y in 0..self.height {
            let mut line = String::new();
            for x in 0..self.width {
                let pixel = self.buffer[y * self.width + x];
                line.push(pixel.ch);
            }
            lines.push(line);
        }

        lines
    }

    /// Get pixel buffer
    pub fn buffer(&self) -> &[Pixel] {
        &self.buffer
    }
}

/// Shape drawing utilities
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Shape {
    /// Line from (x0, y0) to (x1, y1)
    Line { x0: isize, y0: isize, x1: isize, y1: isize },
    /// Rectangle at (x, y) with width and height
    Rectangle { x: usize, y: usize, width: usize, height: usize, filled: bool },
    /// Circle at (cx, cy) with radius
    Circle { cx: isize, cy: isize, radius: isize, filled: bool },
}

impl Shape {
    /// Draw shape on canvas
    pub fn draw(&self, canvas: &mut Canvas, ch: char, color: Color) {
        match self {
            Shape::Line { x0, y0, x1, y1 } => {
                canvas.line(*x0, *y0, *x1, *y1, ch, color);
            }
            Shape::Rectangle { x, y, width, height, filled } => {
                canvas.rectangle(*x, *y, *width, *height, ch, color, *filled);
            }
            Shape::Circle { cx, cy, radius, filled } => {
                canvas.circle(*cx, *cy, *radius, ch, color, *filled);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canvas_creation() {
        let canvas = Canvas::new(80, 24);
        assert_eq!(canvas.width(), 80);
        assert_eq!(canvas.height(), 24);
    }

    #[test]
    fn test_canvas_set_get_pixel() {
        let mut canvas = Canvas::new(10, 10);
        canvas.set_pixel(5, 5, '*', Color::Red, None);

        let pixel = canvas.get(5, 5).unwrap();
        assert_eq!(pixel.ch, '*');
        assert_eq!(pixel.fg, Color::Red);
    }

    #[test]
    fn test_canvas_bounds() {
        let canvas = Canvas::new(10, 10);
        assert!(canvas.get(10, 10).is_none());
        assert!(canvas.get(5, 10).is_none());
        assert!(canvas.get(10, 5).is_none());
    }

    #[test]
    fn test_canvas_clear() {
        let mut canvas = Canvas::new(10, 10);
        canvas.set_pixel(5, 5, '*', Color::Red, None);
        canvas.clear();

        let pixel = canvas.get(5, 5).unwrap();
        assert_eq!(pixel.ch, ' ');
    }

    #[test]
    fn test_canvas_line() {
        let mut canvas = Canvas::new(10, 10);
        canvas.line(0, 0, 9, 0, '-', Color::White);

        // Check horizontal line
        for x in 0..10 {
            let pixel = canvas.get(x, 0).unwrap();
            assert_eq!(pixel.ch, '-');
        }
    }

    #[test]
    fn test_canvas_rectangle_outline() {
        let mut canvas = Canvas::new(20, 20);
        canvas.rectangle(5, 5, 10, 8, '#', Color::Blue, false);

        // Check corners
        assert_eq!(canvas.get(5, 5).unwrap().ch, '#');
        assert_eq!(canvas.get(14, 5).unwrap().ch, '#');
        assert_eq!(canvas.get(5, 12).unwrap().ch, '#');
        assert_eq!(canvas.get(14, 12).unwrap().ch, '#');

        // Interior should be empty
        assert_eq!(canvas.get(10, 10).unwrap().ch, ' ');
    }

    #[test]
    fn test_canvas_rectangle_filled() {
        let mut canvas = Canvas::new(20, 20);
        canvas.rectangle(5, 5, 10, 8, '#', Color::Blue, true);

        // Interior should be filled
        assert_eq!(canvas.get(10, 10).unwrap().ch, '#');
    }

    #[test]
    fn test_canvas_circle() {
        let mut canvas = Canvas::new(40, 40);
        canvas.circle(20, 20, 10, '●', Color::Green, false);

        // Center should be empty (not filled)
        assert_eq!(canvas.get(20, 20).unwrap().ch, ' ');
    }

    #[test]
    fn test_canvas_circle_filled() {
        let mut canvas = Canvas::new(40, 40);
        canvas.circle(20, 20, 10, '●', Color::Green, true);

        // Center should be filled
        assert_eq!(canvas.get(20, 20).unwrap().ch, '●');
    }

    #[test]
    fn test_canvas_text() {
        let mut canvas = Canvas::new(80, 24);
        canvas.text(10, 5, "Hello", Color::White);

        assert_eq!(canvas.get(10, 5).unwrap().ch, 'H');
        assert_eq!(canvas.get(11, 5).unwrap().ch, 'e');
        assert_eq!(canvas.get(12, 5).unwrap().ch, 'l');
        assert_eq!(canvas.get(13, 5).unwrap().ch, 'l');
        assert_eq!(canvas.get(14, 5).unwrap().ch, 'o');
    }

    #[test]
    fn test_canvas_to_lines() {
        let mut canvas = Canvas::new(5, 3);
        canvas.text(0, 1, "Hello", Color::White);

        let lines = canvas.to_lines();
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[1], "Hello");
    }

    #[test]
    fn test_pixel_default() {
        let pixel = Pixel::default();
        assert_eq!(pixel.ch, ' ');
        assert_eq!(pixel.fg, Color::White);
        assert!(pixel.bg.is_none());
    }

    #[test]
    fn test_shape_line_draw() {
        let mut canvas = Canvas::new(20, 20);
        let shape = Shape::Line {
            x0: 0,
            y0: 0,
            x1: 10,
            y1: 10,
        };

        shape.draw(&mut canvas, '-', Color::White);
        assert_eq!(canvas.get(0, 0).unwrap().ch, '-');
    }

    #[test]
    fn test_shape_rectangle_draw() {
        let mut canvas = Canvas::new(20, 20);
        let shape = Shape::Rectangle {
            x: 5,
            y: 5,
            width: 10,
            height: 8,
            filled: false,
        };

        shape.draw(&mut canvas, '#', Color::Blue);
        assert_eq!(canvas.get(5, 5).unwrap().ch, '#');
    }

    #[test]
    fn test_shape_circle_draw() {
        let mut canvas = Canvas::new(40, 40);
        let shape = Shape::Circle {
            cx: 20,
            cy: 20,
            radius: 10,
            filled: false,
        };

        shape.draw(&mut canvas, '●', Color::Green);
        // Center should be empty
        assert_eq!(canvas.get(20, 20).unwrap().ch, ' ');
    }
}
