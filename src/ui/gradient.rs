//! Gradient rendering for visual polish
//!
//! Provides linear and radial gradients with color interpolation,
//! integrated with terminal capability detection for graceful degradation.
//!
//! # Examples
//!
//! ```
//! use toad::ui::gradient::{Gradient, GradientDirection};
//! use ratatui::style::Color;
//!
//! let gradient = Gradient::linear(
//!     Color::Rgb(0, 255, 0),
//!     Color::Rgb(0, 100, 200),
//!     GradientDirection::Horizontal
//! );
//! let color = gradient.color_at(0.5);
//! ```

use crate::infrastructure::{FallbackMode, TerminalCapabilities};
use ratatui::style::Color;

/// Gradient direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GradientDirection {
    /// Left to right
    Horizontal,
    /// Top to bottom
    Vertical,
    /// Top-left to bottom-right
    DiagonalDown,
    /// Bottom-left to top-right
    DiagonalUp,
}

/// Gradient type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GradientType {
    /// Linear gradient
    Linear,
    /// Radial gradient from center
    Radial,
}

/// Color stop in a gradient
#[derive(Debug, Clone, Copy)]
pub struct ColorStop {
    /// Position from 0.0 to 1.0
    pub position: f32,
    /// Color at this position
    pub color: Color,
}

impl ColorStop {
    /// Create a new color stop
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::gradient::ColorStop;
    /// use ratatui::style::Color;
    ///
    /// let stop = ColorStop::new(0.5, Color::Red);
    /// assert_eq!(stop.position, 0.5);
    /// ```
    pub fn new(position: f32, color: Color) -> Self {
        Self {
            position: position.clamp(0.0, 1.0),
            color,
        }
    }
}

/// Gradient configuration
///
/// Supports linear and radial gradients with multiple color stops.
#[derive(Debug, Clone)]
pub struct Gradient {
    /// Color stops
    stops: Vec<ColorStop>,
    /// Gradient type
    gradient_type: GradientType,
    /// Direction (for linear gradients)
    direction: GradientDirection,
}

impl Gradient {
    /// Create a new linear gradient with two colors
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::gradient::{Gradient, GradientDirection};
    /// use ratatui::style::Color;
    ///
    /// let gradient = Gradient::linear(
    ///     Color::Blue,
    ///     Color::Green,
    ///     GradientDirection::Horizontal
    /// );
    /// ```
    pub fn linear(start: Color, end: Color, direction: GradientDirection) -> Self {
        Self {
            stops: vec![ColorStop::new(0.0, start), ColorStop::new(1.0, end)],
            gradient_type: GradientType::Linear,
            direction,
        }
    }

    /// Create a new radial gradient
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::gradient::Gradient;
    /// use ratatui::style::Color;
    ///
    /// let gradient = Gradient::radial(Color::White, Color::Black);
    /// ```
    pub fn radial(center: Color, edge: Color) -> Self {
        Self {
            stops: vec![ColorStop::new(0.0, center), ColorStop::new(1.0, edge)],
            gradient_type: GradientType::Radial,
            direction: GradientDirection::Horizontal, // Not used for radial
        }
    }

    /// Add a color stop
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::gradient::{Gradient, GradientDirection};
    /// use ratatui::style::Color;
    ///
    /// let gradient = Gradient::linear(Color::Red, Color::Blue, GradientDirection::Horizontal)
    ///     .with_stop(0.5, Color::Yellow);
    /// ```
    pub fn with_stop(mut self, position: f32, color: Color) -> Self {
        self.stops.push(ColorStop::new(position, color));
        self.stops.sort_by(|a, b| {
            a.position
                .partial_cmp(&b.position)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        self
    }

    /// Get color at a specific position (0.0 to 1.0)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::gradient::{Gradient, GradientDirection};
    /// use ratatui::style::Color;
    ///
    /// let gradient = Gradient::linear(Color::Red, Color::Blue, GradientDirection::Horizontal);
    /// let mid_color = gradient.color_at(0.5);
    /// ```
    pub fn color_at(&self, position: f32) -> Color {
        let position = position.clamp(0.0, 1.0);

        // Find surrounding stops
        if self.stops.is_empty() {
            return Color::Reset;
        }

        if self.stops.len() == 1 {
            return self.stops[0].color;
        }

        // Find the two stops to interpolate between
        let mut before = &self.stops[0];
        let mut after = &self.stops[self.stops.len() - 1];

        for i in 0..self.stops.len() - 1 {
            if position >= self.stops[i].position && position <= self.stops[i + 1].position {
                before = &self.stops[i];
                after = &self.stops[i + 1];
                break;
            }
        }

        // If position is before first stop or after last stop
        if position <= before.position {
            return before.color;
        }
        if position >= after.position {
            return after.color;
        }

        // Interpolate between the two colors
        let range = after.position - before.position;
        let t = (position - before.position) / range;
        Self::interpolate_color(before.color, after.color, t)
    }

    /// Get color at a 2D position for the gradient
    ///
    /// # Arguments
    ///
    /// * `x` - X coordinate (0.0 to 1.0)
    /// * `y` - Y coordinate (0.0 to 1.0)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::gradient::{Gradient, GradientDirection};
    /// use ratatui::style::Color;
    ///
    /// let gradient = Gradient::linear(Color::Red, Color::Blue, GradientDirection::Horizontal);
    /// let color = gradient.color_at_2d(0.5, 0.5);
    /// ```
    pub fn color_at_2d(&self, x: f32, y: f32) -> Color {
        let position = match self.gradient_type {
            GradientType::Linear => match self.direction {
                GradientDirection::Horizontal => x,
                GradientDirection::Vertical => y,
                GradientDirection::DiagonalDown => (x + y) / 2.0,
                GradientDirection::DiagonalUp => (x + (1.0 - y)) / 2.0,
            },
            GradientType::Radial => {
                // Distance from center
                let dx = x - 0.5;
                let dy = y - 0.5;
                (dx * dx + dy * dy).sqrt() * 2.0_f32.sqrt()
            }
        };

        self.color_at(position)
    }

    /// Get direction
    pub fn direction(&self) -> GradientDirection {
        self.direction
    }

    /// Get gradient type
    pub fn gradient_type(&self) -> GradientType {
        self.gradient_type
    }

    /// Apply fallback mode to gradient colors
    pub fn with_fallback(&self, mode: &FallbackMode, caps: &TerminalCapabilities) -> Gradient {
        let new_stops: Vec<ColorStop> = self
            .stops
            .iter()
            .map(|stop| ColorStop {
                position: stop.position,
                color: mode.fallback_color(stop.color, caps),
            })
            .collect();

        Gradient {
            stops: new_stops,
            gradient_type: self.gradient_type,
            direction: self.direction,
        }
    }

    /// Interpolate between two colors
    fn interpolate_color(start: Color, end: Color, t: f32) -> Color {
        let t = t.clamp(0.0, 1.0);

        match (start, end) {
            (Color::Rgb(r1, g1, b1), Color::Rgb(r2, g2, b2)) => {
                let r = (r1 as f32 + (r2 as f32 - r1 as f32) * t) as u8;
                let g = (g1 as f32 + (g2 as f32 - g1 as f32) * t) as u8;
                let b = (b1 as f32 + (b2 as f32 - b1 as f32) * t) as u8;
                Color::Rgb(r, g, b)
            }
            // For indexed colors, try to convert to RGB first
            (Color::Indexed(i1), Color::Indexed(i2)) => {
                // Simple linear interpolation of indices
                let idx = (i1 as f32 + (i2 as f32 - i1 as f32) * t) as u8;
                Color::Indexed(idx)
            }
            // If one is RGB and other is not, prefer the RGB one
            (Color::Rgb(r, g, b), _) => {
                if t < 0.5 {
                    Color::Rgb(r, g, b)
                } else {
                    end
                }
            }
            (_, Color::Rgb(r, g, b)) => {
                if t >= 0.5 {
                    Color::Rgb(r, g, b)
                } else {
                    start
                }
            }
            // For other colors, hard switch at midpoint
            _ => {
                if t < 0.5 {
                    start
                } else {
                    end
                }
            }
        }
    }
}

/// Predefined gradients
pub struct Gradients;

impl Gradients {
    /// TOAD brand gradient (green to blue)
    pub fn toad_brand() -> Gradient {
        Gradient::linear(
            Color::Rgb(0, 255, 136),     // Bright green
            Color::Rgb(0, 204, 255),     // Bright blue
            GradientDirection::Horizontal,
        )
    }

    /// Sunset gradient (orange to purple)
    pub fn sunset() -> Gradient {
        Gradient::linear(
            Color::Rgb(255, 94, 77),     // Orange
            Color::Rgb(139, 76, 255),    // Purple
            GradientDirection::Horizontal,
        )
        .with_stop(0.5, Color::Rgb(255, 138, 101)) // Peach
    }

    /// Ocean gradient (cyan to deep blue)
    pub fn ocean() -> Gradient {
        Gradient::linear(
            Color::Rgb(0, 255, 255),     // Cyan
            Color::Rgb(0, 51, 153),      // Deep blue
            GradientDirection::Vertical,
        )
    }

    /// Fire gradient (yellow to red)
    pub fn fire() -> Gradient {
        Gradient::linear(
            Color::Rgb(255, 255, 0),     // Yellow
            Color::Rgb(255, 0, 0),       // Red
            GradientDirection::Vertical,
        )
        .with_stop(0.5, Color::Rgb(255, 128, 0)) // Orange
    }

    /// Forest gradient (light green to dark green)
    pub fn forest() -> Gradient {
        Gradient::linear(
            Color::Rgb(144, 238, 144),   // Light green
            Color::Rgb(34, 139, 34),     // Dark green
            GradientDirection::Vertical,
        )
    }

    /// Monochrome gradient (white to black)
    pub fn monochrome() -> Gradient {
        Gradient::linear(
            Color::Rgb(255, 255, 255),   // White
            Color::Rgb(0, 0, 0),         // Black
            GradientDirection::Horizontal,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_stop_creation() {
        let stop = ColorStop::new(0.5, Color::Red);
        assert_eq!(stop.position, 0.5);
        assert_eq!(stop.color, Color::Red);
    }

    #[test]
    fn test_color_stop_clamping() {
        let stop1 = ColorStop::new(-0.5, Color::Red);
        assert_eq!(stop1.position, 0.0);

        let stop2 = ColorStop::new(1.5, Color::Blue);
        assert_eq!(stop2.position, 1.0);
    }

    #[test]
    fn test_linear_gradient_creation() {
        let gradient = Gradient::linear(Color::Red, Color::Blue, GradientDirection::Horizontal);
        assert_eq!(gradient.direction(), GradientDirection::Horizontal);
        assert_eq!(gradient.gradient_type(), GradientType::Linear);
    }

    #[test]
    fn test_radial_gradient_creation() {
        let gradient = Gradient::radial(Color::White, Color::Black);
        assert_eq!(gradient.gradient_type(), GradientType::Radial);
    }

    #[test]
    fn test_gradient_with_stop() {
        let gradient = Gradient::linear(Color::Red, Color::Blue, GradientDirection::Horizontal)
            .with_stop(0.5, Color::Green);

        // Should have 3 stops
        assert_eq!(gradient.stops.len(), 3);
    }

    #[test]
    fn test_color_at_endpoints() {
        let gradient = Gradient::linear(Color::Red, Color::Blue, GradientDirection::Horizontal);

        assert_eq!(gradient.color_at(0.0), Color::Red);
        assert_eq!(gradient.color_at(1.0), Color::Blue);
    }

    #[test]
    fn test_color_at_midpoint() {
        let gradient = Gradient::linear(
            Color::Rgb(0, 0, 0),
            Color::Rgb(100, 100, 100),
            GradientDirection::Horizontal,
        );

        let mid_color = gradient.color_at(0.5);
        if let Color::Rgb(r, g, b) = mid_color {
            assert!(r >= 45 && r <= 55); // Allow some rounding error
            assert!(g >= 45 && g <= 55);
            assert!(b >= 45 && b <= 55);
        } else {
            panic!("Expected RGB color");
        }
    }

    #[test]
    fn test_color_interpolation_rgb() {
        let start = Color::Rgb(0, 0, 0);
        let end = Color::Rgb(100, 100, 100);

        let mid = Gradient::interpolate_color(start, end, 0.5);
        if let Color::Rgb(r, g, b) = mid {
            assert!(r >= 45 && r <= 55);
            assert!(g >= 45 && g <= 55);
            assert!(b >= 45 && b <= 55);
        }
    }

    #[test]
    fn test_color_at_2d_horizontal() {
        let gradient = Gradient::linear(Color::Red, Color::Blue, GradientDirection::Horizontal);

        assert_eq!(gradient.color_at_2d(0.0, 0.5), Color::Red);
        assert_eq!(gradient.color_at_2d(1.0, 0.5), Color::Blue);
    }

    #[test]
    fn test_color_at_2d_vertical() {
        let gradient = Gradient::linear(Color::Red, Color::Blue, GradientDirection::Vertical);

        assert_eq!(gradient.color_at_2d(0.5, 0.0), Color::Red);
        assert_eq!(gradient.color_at_2d(0.5, 1.0), Color::Blue);
    }

    #[test]
    fn test_radial_gradient_2d() {
        let gradient = Gradient::radial(Color::White, Color::Black);

        // Center should be white
        assert_eq!(gradient.color_at_2d(0.5, 0.5), Color::White);
    }

    #[test]
    fn test_predefined_toad_brand() {
        let gradient = Gradients::toad_brand();
        assert_eq!(gradient.gradient_type(), GradientType::Linear);
        assert_eq!(gradient.direction(), GradientDirection::Horizontal);
    }

    #[test]
    fn test_predefined_sunset() {
        let gradient = Gradients::sunset();
        assert_eq!(gradient.stops.len(), 3); // Start, middle, end
    }

    #[test]
    fn test_predefined_ocean() {
        let gradient = Gradients::ocean();
        assert_eq!(gradient.direction(), GradientDirection::Vertical);
    }

    #[test]
    fn test_gradient_with_fallback() {
        use crate::infrastructure::{ColorSupport, FeatureLevel};

        let gradient = Gradient::linear(
            Color::Rgb(255, 0, 0),
            Color::Rgb(0, 0, 255),
            GradientDirection::Horizontal,
        );

        let caps = TerminalCapabilities {
            color_support: ColorSupport::Basic16,
            unicode_support: false,
            mouse_support: false,
            alternate_screen: false,
            styled_underlines: false,
            bracketed_paste: false,
            term_name: "dumb".to_string(),
            term_program: None,
            nerd_fonts: false,
        };

        let fallback_mode = FallbackMode::from_capabilities(&caps);
        let fallback_gradient = gradient.with_fallback(&fallback_mode, &caps);

        // Colors should be converted to 16-color palette
        let start_color = fallback_gradient.color_at(0.0);
        assert_eq!(start_color, Color::Red);
    }

    #[test]
    fn test_position_clamping() {
        let gradient = Gradient::linear(Color::Red, Color::Blue, GradientDirection::Horizontal);

        // Values outside 0..1 should be clamped
        assert_eq!(gradient.color_at(-1.0), Color::Red);
        assert_eq!(gradient.color_at(2.0), Color::Blue);
    }
}
