//! Board background system for visual customization
//!
//! Provides gradient backgrounds, pattern backgrounds, and image backgrounds
//! with fallback support for limited terminals.
//!
//! # Examples
//!
//! ```
//! use toad::ui::board_background::{BoardBackground, BackgroundStyle};
//! use toad::ui::gradient::Gradients;
//!
//! let bg = BoardBackground::gradient(Gradients::toad_brand());
//! ```

use crate::infrastructure::{FallbackMode, TerminalCapabilities};
use crate::ui::gradient::Gradient;
use ratatui::style::Color;

/// Background style type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackgroundStyle {
    /// Solid color
    Solid,
    /// Gradient fill
    Gradient,
    /// Pattern fill
    Pattern,
    /// Custom/uploaded image (future)
    Image,
}

/// Pattern type for backgrounds
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PatternType {
    /// Dots pattern
    Dots,
    /// Grid pattern
    Grid,
    /// Diagonal lines
    Diagonal,
    /// Checkerboard
    Checkerboard,
    /// Waves
    Waves,
    /// Hexagons
    Hexagons,
}

/// Board background configuration
///
/// Manages background appearance with support for gradients, patterns,
/// and solid colors with graceful degradation.
#[derive(Debug, Clone)]
pub struct BoardBackground {
    /// Background style
    style: BackgroundStyle,
    /// Primary color
    primary_color: Color,
    /// Secondary color (for gradients/patterns)
    secondary_color: Option<Color>,
    /// Gradient (if style is Gradient)
    gradient: Option<Gradient>,
    /// Pattern type (if style is Pattern)
    pattern: Option<PatternType>,
    /// Pattern density (0.0 to 1.0)
    pattern_density: f32,
    /// Whether background is enabled
    enabled: bool,
    /// Opacity (0.0 to 1.0)
    opacity: f32,
}

impl BoardBackground {
    /// Create a solid color background
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::board_background::BoardBackground;
    /// use ratatui::style::Color;
    ///
    /// let bg = BoardBackground::solid(Color::Black);
    /// ```
    pub fn solid(color: Color) -> Self {
        Self {
            style: BackgroundStyle::Solid,
            primary_color: color,
            secondary_color: None,
            gradient: None,
            pattern: None,
            pattern_density: 0.5,
            enabled: true,
            opacity: 1.0,
        }
    }

    /// Create a gradient background
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::board_background::BoardBackground;
    /// use toad::ui::gradient::Gradients;
    ///
    /// let bg = BoardBackground::gradient(Gradients::ocean());
    /// ```
    pub fn gradient(gradient: Gradient) -> Self {
        Self {
            style: BackgroundStyle::Gradient,
            primary_color: Color::Reset,
            secondary_color: None,
            gradient: Some(gradient),
            pattern: None,
            pattern_density: 0.5,
            enabled: true,
            opacity: 1.0,
        }
    }

    /// Create a pattern background
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::board_background::{BoardBackground, PatternType};
    /// use ratatui::style::Color;
    ///
    /// let bg = BoardBackground::pattern(PatternType::Dots, Color::DarkGray, Color::Black);
    /// ```
    pub fn pattern(pattern: PatternType, fg: Color, bg: Color) -> Self {
        Self {
            style: BackgroundStyle::Pattern,
            primary_color: fg,
            secondary_color: Some(bg),
            gradient: None,
            pattern: Some(pattern),
            pattern_density: 0.5,
            enabled: true,
            opacity: 1.0,
        }
    }

    /// Set opacity
    pub fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity.clamp(0.0, 1.0);
        self
    }

    /// Set pattern density
    pub fn density(mut self, density: f32) -> Self {
        self.pattern_density = density.clamp(0.0, 1.0);
        self
    }

    /// Enable or disable background
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Get background style
    pub fn style(&self) -> BackgroundStyle {
        self.style
    }

    /// Get primary color
    pub fn primary_color(&self) -> Color {
        self.primary_color
    }

    /// Get secondary color
    pub fn secondary_color(&self) -> Option<Color> {
        self.secondary_color
    }

    /// Get gradient
    pub fn get_gradient(&self) -> Option<&Gradient> {
        self.gradient.as_ref()
    }

    /// Get pattern type
    pub fn pattern_type(&self) -> Option<PatternType> {
        self.pattern
    }

    /// Get pattern density
    pub fn density_value(&self) -> f32 {
        self.pattern_density
    }

    /// Check if enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Get opacity
    pub fn get_opacity(&self) -> f32 {
        self.opacity
    }

    /// Get color at specific position (for gradients)
    ///
    /// # Arguments
    ///
    /// * `x` - X coordinate (0.0 to 1.0)
    /// * `y` - Y coordinate (0.0 to 1.0)
    pub fn color_at(&self, x: f32, y: f32) -> Color {
        match self.style {
            BackgroundStyle::Solid => self.primary_color,
            BackgroundStyle::Gradient => {
                if let Some(ref gradient) = self.gradient {
                    gradient.color_at_2d(x, y)
                } else {
                    self.primary_color
                }
            }
            BackgroundStyle::Pattern => {
                if self.should_draw_pattern(x, y) {
                    self.primary_color
                } else {
                    self.secondary_color.unwrap_or(self.primary_color)
                }
            }
            BackgroundStyle::Image => self.primary_color, // Future: load from image
        }
    }

    /// Get pattern character at position
    pub fn pattern_char_at(&self, x: u16, y: u16) -> Option<char> {
        if !self.enabled || self.style != BackgroundStyle::Pattern {
            return None;
        }

        let pattern = self.pattern?;

        match pattern {
            PatternType::Dots => {
                if x % 4 == 0 && y % 2 == 0 {
                    Some('·')
                } else {
                    Some(' ')
                }
            }
            PatternType::Grid => {
                if x % 8 == 0 || y % 4 == 0 {
                    Some('┼')
                } else {
                    Some(' ')
                }
            }
            PatternType::Diagonal => {
                if (x + y) % 6 == 0 {
                    Some('╱')
                } else if (x + y) % 6 == 3 {
                    Some('╲')
                } else {
                    Some(' ')
                }
            }
            PatternType::Checkerboard => {
                if (x / 4 + y / 2) % 2 == 0 {
                    Some('█')
                } else {
                    Some(' ')
                }
            }
            PatternType::Waves => {
                let wave = (x as f32 * 0.3 + y as f32).sin() * 2.0;
                if wave > 0.0 {
                    Some('~')
                } else {
                    Some(' ')
                }
            }
            PatternType::Hexagons => {
                if (x % 6 == 0 && y % 3 == 0) || (x % 6 == 3 && y % 3 == 1) {
                    Some('⬡')
                } else {
                    Some(' ')
                }
            }
        }
    }

    /// Check if pattern should be drawn at normalized position
    fn should_draw_pattern(&self, x: f32, y: f32) -> bool {
        if let Some(pattern) = self.pattern {
            match pattern {
                PatternType::Dots => {
                    let grid_x = (x * 20.0) as u16;
                    let grid_y = (y * 20.0) as u16;
                    grid_x % 4 == 0 && grid_y % 4 == 0
                }
                PatternType::Grid => {
                    let grid_x = (x * 20.0) as u16;
                    let grid_y = (y * 20.0) as u16;
                    grid_x % 5 == 0 || grid_y % 5 == 0
                }
                PatternType::Diagonal => {
                    let grid_x = (x * 20.0) as u16;
                    let grid_y = (y * 20.0) as u16;
                    (grid_x + grid_y) % 4 == 0
                }
                PatternType::Checkerboard => {
                    let grid_x = (x * 10.0) as u16;
                    let grid_y = (y * 10.0) as u16;
                    (grid_x + grid_y) % 2 == 0
                }
                PatternType::Waves => {
                    ((x * 10.0).sin() + (y * 10.0).sin()) > 0.0
                }
                PatternType::Hexagons => {
                    let grid_x = (x * 15.0) as u16;
                    let grid_y = (y * 15.0) as u16;
                    (grid_x % 3 == 0 && grid_y % 2 == 0) || (grid_x % 3 == 1 && grid_y % 2 == 1)
                }
            }
        } else {
            false
        }
    }

    /// Apply fallback mode
    pub fn with_fallback(&self, mode: &FallbackMode, caps: &TerminalCapabilities) -> BoardBackground {
        let mut result = self.clone();

        // Simplify based on terminal capabilities
        if !mode.use_gradients() && self.style == BackgroundStyle::Gradient {
            // Fallback to solid color
            result.style = BackgroundStyle::Solid;
            result.primary_color = mode.fallback_color(self.primary_color, caps);
        }

        if !mode.use_unicode_borders() && self.style == BackgroundStyle::Pattern {
            // Fallback to solid color
            result.style = BackgroundStyle::Solid;
        }

        if let Some(ref gradient) = result.gradient {
            result.gradient = Some(gradient.with_fallback(mode, caps));
        }

        result.primary_color = mode.fallback_color(result.primary_color, caps);
        if let Some(color) = result.secondary_color {
            result.secondary_color = Some(mode.fallback_color(color, caps));
        }

        result
    }
}

impl Default for BoardBackground {
    fn default() -> Self {
        Self::solid(Color::Reset)
    }
}

/// Predefined board backgrounds
pub struct BoardBackgrounds;

impl BoardBackgrounds {
    /// TOAD brand gradient background
    pub fn toad_brand() -> BoardBackground {
        use crate::ui::gradient::Gradients;
        BoardBackground::gradient(Gradients::toad_brand())
    }

    /// Subtle dots pattern
    pub fn subtle_dots() -> BoardBackground {
        BoardBackground::pattern(
            PatternType::Dots,
            Color::DarkGray,
            Color::Black,
        )
        .density(0.3)
    }

    /// Grid pattern
    pub fn grid() -> BoardBackground {
        BoardBackground::pattern(
            PatternType::Grid,
            Color::DarkGray,
            Color::Reset,
        )
        .density(0.5)
    }

    /// Ocean gradient
    pub fn ocean() -> BoardBackground {
        use crate::ui::gradient::Gradients;
        BoardBackground::gradient(Gradients::ocean())
    }

    /// Sunset gradient
    pub fn sunset() -> BoardBackground {
        use crate::ui::gradient::Gradients;
        BoardBackground::gradient(Gradients::sunset())
    }

    /// Dark solid
    pub fn dark() -> BoardBackground {
        BoardBackground::solid(Color::Black)
    }

    /// Transparent
    pub fn transparent() -> BoardBackground {
        BoardBackground::solid(Color::Reset).opacity(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solid_background() {
        let bg = BoardBackground::solid(Color::Blue);
        assert_eq!(bg.style(), BackgroundStyle::Solid);
        assert_eq!(bg.primary_color(), Color::Blue);
        assert!(bg.is_enabled());
    }

    #[test]
    fn test_gradient_background() {
        use crate::ui::gradient::{Gradient, GradientDirection};

        let gradient = Gradient::linear(Color::Red, Color::Blue, GradientDirection::Horizontal);
        let bg = BoardBackground::gradient(gradient);

        assert_eq!(bg.style(), BackgroundStyle::Gradient);
        assert!(bg.get_gradient().is_some());
    }

    #[test]
    fn test_pattern_background() {
        let bg = BoardBackground::pattern(PatternType::Dots, Color::White, Color::Black);
        assert_eq!(bg.style(), BackgroundStyle::Pattern);
        assert_eq!(bg.pattern_type(), Some(PatternType::Dots));
        assert_eq!(bg.secondary_color(), Some(Color::Black));
    }

    #[test]
    fn test_opacity() {
        let bg = BoardBackground::solid(Color::Red).opacity(0.5);
        assert_eq!(bg.get_opacity(), 0.5);

        // Test clamping
        let bg = BoardBackground::solid(Color::Red).opacity(1.5);
        assert_eq!(bg.get_opacity(), 1.0);

        let bg = BoardBackground::solid(Color::Red).opacity(-0.5);
        assert_eq!(bg.get_opacity(), 0.0);
    }

    #[test]
    fn test_density() {
        let bg = BoardBackground::pattern(PatternType::Grid, Color::White, Color::Black)
            .density(0.7);
        assert_eq!(bg.density_value(), 0.7);
    }

    #[test]
    fn test_enabled() {
        let bg = BoardBackground::solid(Color::Red).enabled(false);
        assert!(!bg.is_enabled());
    }

    #[test]
    fn test_color_at_solid() {
        let bg = BoardBackground::solid(Color::Red);
        assert_eq!(bg.color_at(0.5, 0.5), Color::Red);
    }

    #[test]
    fn test_color_at_gradient() {
        use crate::ui::gradient::{Gradient, GradientDirection};

        let gradient = Gradient::linear(Color::Red, Color::Blue, GradientDirection::Horizontal);
        let bg = BoardBackground::gradient(gradient);

        assert_eq!(bg.color_at(0.0, 0.5), Color::Red);
        assert_eq!(bg.color_at(1.0, 0.5), Color::Blue);
    }

    #[test]
    fn test_pattern_char_at() {
        let bg = BoardBackground::pattern(PatternType::Dots, Color::White, Color::Black);

        // Should return Some character for enabled pattern
        let char = bg.pattern_char_at(0, 0);
        assert!(char.is_some());
    }

    #[test]
    fn test_with_fallback() {
        use crate::infrastructure::ColorSupport;
        use crate::ui::gradient::{Gradient, GradientDirection};

        let gradient = Gradient::linear(
            Color::Rgb(255, 0, 0),
            Color::Rgb(0, 0, 255),
            GradientDirection::Horizontal,
        );
        let bg = BoardBackground::gradient(gradient);

        let caps = TerminalCapabilities {
            color_support: ColorSupport::None,
            unicode_support: false,
            mouse_support: false,
            alternate_screen: false,
            styled_underlines: false,
            bracketed_paste: false,
            term_name: "dumb".to_string(),
            term_program: None,
            nerd_fonts: false,
        };

        let mode = FallbackMode::from_capabilities(&caps);
        let fallback_bg = bg.with_fallback(&mode, &caps);

        // Should fallback to solid on minimal terminal
        assert_eq!(fallback_bg.style(), BackgroundStyle::Solid);
    }

    #[test]
    fn test_predefined_toad_brand() {
        let bg = BoardBackgrounds::toad_brand();
        assert_eq!(bg.style(), BackgroundStyle::Gradient);
    }

    #[test]
    fn test_predefined_subtle_dots() {
        let bg = BoardBackgrounds::subtle_dots();
        assert_eq!(bg.style(), BackgroundStyle::Pattern);
        assert_eq!(bg.pattern_type(), Some(PatternType::Dots));
    }

    #[test]
    fn test_predefined_grid() {
        let bg = BoardBackgrounds::grid();
        assert_eq!(bg.pattern_type(), Some(PatternType::Grid));
    }

    #[test]
    fn test_predefined_dark() {
        let bg = BoardBackgrounds::dark();
        assert_eq!(bg.style(), BackgroundStyle::Solid);
        assert_eq!(bg.primary_color(), Color::Black);
    }

    #[test]
    fn test_predefined_transparent() {
        let bg = BoardBackgrounds::transparent();
        assert_eq!(bg.get_opacity(), 0.0);
    }

    #[test]
    fn test_default() {
        let bg = BoardBackground::default();
        assert_eq!(bg.style(), BackgroundStyle::Solid);
        assert!(bg.is_enabled());
    }
}
