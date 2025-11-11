//! Enhanced border styles for visual polish
//!
//! Provides gradient borders, shadows, rounded corners, and special effects
//! that integrate with terminal capability detection for graceful degradation.
//!
//! # Examples
//!
//! ```
//! use toad::ui::enhanced_borders::{EnhancedBorder, BorderEffect};
//! use toad::ui::gradient::Gradients;
//!
//! let border = EnhancedBorder::new()
//!     .gradient(Gradients::toad_brand())
//!     .rounded(true)
//!     .effect(BorderEffect::Glow);
//! ```

use crate::infrastructure::{FallbackMode, TerminalCapabilities};
use crate::ui::effects::gradient::Gradient;
use ratatui::style::Color;
use ratatui::widgets::BorderType;

/// Border effect
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BorderEffect {
    /// No special effect
    None,
    /// Glowing border
    Glow,
    /// Shadow effect
    Shadow,
    /// Pulsating border
    Pulse,
    /// Double-line border
    Double,
}

/// Border thickness
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BorderThickness {
    /// Thin (single line)
    Thin,
    /// Normal (standard)
    Normal,
    /// Thick (double line or bold)
    Thick,
}

/// Corner style
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CornerStyle {
    /// Sharp 90-degree corners
    Sharp,
    /// Rounded corners
    Rounded,
    /// Beveled corners
    Beveled,
}

/// Enhanced border configuration
///
/// Supports gradients, effects, rounded corners, and various styles
#[derive(Debug, Clone)]
pub struct EnhancedBorder {
    /// Border gradient (if any)
    gradient: Option<Gradient>,
    /// Solid color fallback
    color: Color,
    /// Border effect
    effect: BorderEffect,
    /// Border thickness
    thickness: BorderThickness,
    /// Corner style
    corner_style: CornerStyle,
    /// Whether border is enabled
    enabled: bool,
}

impl EnhancedBorder {
    /// Create a new enhanced border
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::enhanced_borders::EnhancedBorder;
    ///
    /// let border = EnhancedBorder::new();
    /// ```
    pub fn new() -> Self {
        Self {
            gradient: None,
            color: Color::White,
            effect: BorderEffect::None,
            thickness: BorderThickness::Normal,
            corner_style: CornerStyle::Sharp,
            enabled: true,
        }
    }

    /// Set border gradient
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::enhanced_borders::EnhancedBorder;
    /// use toad::ui::gradient::Gradients;
    ///
    /// let border = EnhancedBorder::new().gradient(Gradients::toad_brand());
    /// ```
    pub fn gradient(mut self, gradient: Gradient) -> Self {
        self.gradient = Some(gradient);
        self
    }

    /// Set solid color
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::enhanced_borders::EnhancedBorder;
    /// use ratatui::style::Color;
    ///
    /// let border = EnhancedBorder::new().color(Color::Green);
    /// ```
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Set border effect
    pub fn effect(mut self, effect: BorderEffect) -> Self {
        self.effect = effect;
        self
    }

    /// Set border thickness
    pub fn thickness(mut self, thickness: BorderThickness) -> Self {
        self.thickness = thickness;
        self
    }

    /// Set rounded corners
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::enhanced_borders::EnhancedBorder;
    ///
    /// let border = EnhancedBorder::new().rounded(true);
    /// ```
    pub fn rounded(mut self, rounded: bool) -> Self {
        if rounded {
            self.corner_style = CornerStyle::Rounded;
        } else {
            self.corner_style = CornerStyle::Sharp;
        }
        self
    }

    /// Set corner style
    pub fn corner_style(mut self, style: CornerStyle) -> Self {
        self.corner_style = style;
        self
    }

    /// Enable or disable border
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Get color at position (for gradient borders)
    ///
    /// # Arguments
    ///
    /// * `position` - Position from 0.0 to 1.0 along the border
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::enhanced_borders::EnhancedBorder;
    /// use toad::ui::gradient::Gradients;
    ///
    /// let border = EnhancedBorder::new().gradient(Gradients::toad_brand());
    /// let color = border.color_at(0.5);
    /// ```
    pub fn color_at(&self, position: f32) -> Color {
        if let Some(ref gradient) = self.gradient {
            gradient.color_at(position)
        } else {
            self.color
        }
    }

    /// Get border type for Ratatui
    pub fn border_type(&self) -> BorderType {
        match self.corner_style {
            CornerStyle::Sharp => match self.thickness {
                BorderThickness::Thin => BorderType::Plain,
                BorderThickness::Normal => BorderType::Plain,
                BorderThickness::Thick => BorderType::Thick,
            },
            CornerStyle::Rounded => BorderType::Rounded,
            CornerStyle::Beveled => BorderType::Double,
        }
    }

    /// Check if border has gradient
    pub fn has_gradient(&self) -> bool {
        self.gradient.is_some()
    }

    /// Get effect
    pub fn get_effect(&self) -> BorderEffect {
        self.effect
    }

    /// Get thickness
    pub fn get_thickness(&self) -> BorderThickness {
        self.thickness
    }

    /// Get corner style
    pub fn get_corner_style(&self) -> CornerStyle {
        self.corner_style
    }

    /// Check if border is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Apply fallback mode to border
    pub fn with_fallback(
        &self,
        mode: &FallbackMode,
        caps: &TerminalCapabilities,
    ) -> EnhancedBorder {
        let new_gradient = self.gradient.as_ref().map(|g| g.with_fallback(mode, caps));
        let new_color = mode.fallback_color(self.color, caps);

        // Disable gradients if not supported
        let final_gradient = if mode.use_gradients() {
            new_gradient
        } else {
            None
        };

        // Simplify effects on basic terminals
        let final_effect = if mode.use_colors() {
            self.effect
        } else {
            BorderEffect::None
        };

        // Simplify corners on minimal terminals
        let final_corner_style = if mode.use_unicode_borders() {
            self.corner_style
        } else {
            CornerStyle::Sharp
        };

        EnhancedBorder {
            gradient: final_gradient,
            color: new_color,
            effect: final_effect,
            thickness: self.thickness,
            corner_style: final_corner_style,
            enabled: self.enabled,
        }
    }

    /// Get shadow character based on effect
    pub fn shadow_char(&self) -> Option<char> {
        match self.effect {
            BorderEffect::Shadow => Some('░'),
            _ => None,
        }
    }

    /// Get glow intensity (0.0 to 1.0) for pulsing effects
    ///
    /// # Arguments
    ///
    /// * `time` - Current time for animation
    pub fn glow_intensity(&self, time: f32) -> f32 {
        match self.effect {
            BorderEffect::Glow => 0.8,
            BorderEffect::Pulse => {
                // Pulsate between 0.5 and 1.0
                0.75 + 0.25 * (time * 2.0).sin()
            }
            _ => 1.0,
        }
    }
}

impl Default for EnhancedBorder {
    fn default() -> Self {
        Self::new()
    }
}

/// Predefined enhanced border styles
pub struct BorderStyles;

impl BorderStyles {
    /// TOAD brand border (green-blue gradient)
    pub fn toad_brand() -> EnhancedBorder {
        use crate::ui::effects::gradient::Gradients;
        EnhancedBorder::new()
            .gradient(Gradients::toad_brand())
            .rounded(true)
            .effect(BorderEffect::Glow)
    }

    /// Success border (green with glow)
    pub fn success() -> EnhancedBorder {
        EnhancedBorder::new()
            .color(Color::Green)
            .effect(BorderEffect::Glow)
            .rounded(true)
    }

    /// Error border (red with shadow)
    pub fn error() -> EnhancedBorder {
        EnhancedBorder::new()
            .color(Color::Red)
            .effect(BorderEffect::Shadow)
            .thickness(BorderThickness::Thick)
    }

    /// Warning border (yellow pulse)
    pub fn warning() -> EnhancedBorder {
        EnhancedBorder::new()
            .color(Color::Yellow)
            .effect(BorderEffect::Pulse)
    }

    /// Info border (blue rounded)
    pub fn info() -> EnhancedBorder {
        EnhancedBorder::new().color(Color::Cyan).rounded(true)
    }

    /// Subtle border (gray thin)
    pub fn subtle() -> EnhancedBorder {
        EnhancedBorder::new()
            .color(Color::DarkGray)
            .thickness(BorderThickness::Thin)
    }

    /// Emphasis border (thick double)
    pub fn emphasis() -> EnhancedBorder {
        EnhancedBorder::new()
            .thickness(BorderThickness::Thick)
            .corner_style(CornerStyle::Beveled)
    }

    /// Sunset gradient border
    pub fn sunset() -> EnhancedBorder {
        use crate::ui::effects::gradient::Gradients;
        EnhancedBorder::new()
            .gradient(Gradients::sunset())
            .rounded(true)
    }

    /// Ocean gradient border
    pub fn ocean() -> EnhancedBorder {
        use crate::ui::effects::gradient::Gradients;
        EnhancedBorder::new()
            .gradient(Gradients::ocean())
            .rounded(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::ColorSupport;

    #[test]
    fn test_enhanced_border_creation() {
        let border = EnhancedBorder::new();
        assert_eq!(border.get_effect(), BorderEffect::None);
        assert_eq!(border.get_thickness(), BorderThickness::Normal);
        assert_eq!(border.get_corner_style(), CornerStyle::Sharp);
        assert!(border.is_enabled());
        assert!(!border.has_gradient());
    }

    #[test]
    fn test_border_with_color() {
        let border = EnhancedBorder::new().color(Color::Red);
        assert_eq!(border.color_at(0.5), Color::Red);
    }

    #[test]
    fn test_border_with_gradient() {
        use crate::ui::effects::gradient::{Gradient, GradientDirection};

        let gradient = Gradient::linear(Color::Red, Color::Blue, GradientDirection::Horizontal);
        let border = EnhancedBorder::new().gradient(gradient);

        assert!(border.has_gradient());
        assert_eq!(border.color_at(0.0), Color::Red);
        assert_eq!(border.color_at(1.0), Color::Blue);
    }

    #[test]
    fn test_border_effects() {
        let glow = EnhancedBorder::new().effect(BorderEffect::Glow);
        assert_eq!(glow.get_effect(), BorderEffect::Glow);

        let shadow = EnhancedBorder::new().effect(BorderEffect::Shadow);
        assert_eq!(shadow.get_effect(), BorderEffect::Shadow);
    }

    #[test]
    fn test_border_thickness() {
        let thin = EnhancedBorder::new().thickness(BorderThickness::Thin);
        assert_eq!(thin.get_thickness(), BorderThickness::Thin);

        let thick = EnhancedBorder::new().thickness(BorderThickness::Thick);
        assert_eq!(thick.get_thickness(), BorderThickness::Thick);
    }

    #[test]
    fn test_rounded_corners() {
        let rounded = EnhancedBorder::new().rounded(true);
        assert_eq!(rounded.get_corner_style(), CornerStyle::Rounded);
        assert_eq!(rounded.border_type(), BorderType::Rounded);

        let sharp = EnhancedBorder::new().rounded(false);
        assert_eq!(sharp.get_corner_style(), CornerStyle::Sharp);
    }

    #[test]
    fn test_corner_styles() {
        let rounded = EnhancedBorder::new().corner_style(CornerStyle::Rounded);
        assert_eq!(rounded.border_type(), BorderType::Rounded);

        let beveled = EnhancedBorder::new().corner_style(CornerStyle::Beveled);
        assert_eq!(beveled.border_type(), BorderType::Double);
    }

    #[test]
    fn test_border_enabled() {
        let enabled = EnhancedBorder::new().enabled(true);
        assert!(enabled.is_enabled());

        let disabled = EnhancedBorder::new().enabled(false);
        assert!(!disabled.is_enabled());
    }

    #[test]
    fn test_shadow_char() {
        let shadow = EnhancedBorder::new().effect(BorderEffect::Shadow);
        assert_eq!(shadow.shadow_char(), Some('░'));

        let no_shadow = EnhancedBorder::new().effect(BorderEffect::None);
        assert_eq!(no_shadow.shadow_char(), None);
    }

    #[test]
    fn test_glow_intensity() {
        let glow = EnhancedBorder::new().effect(BorderEffect::Glow);
        assert_eq!(glow.glow_intensity(0.0), 0.8);

        let pulse = EnhancedBorder::new().effect(BorderEffect::Pulse);
        let intensity = pulse.glow_intensity(0.0);
        assert!(intensity >= 0.5 && intensity <= 1.0);
    }

    #[test]
    fn test_with_fallback() {
        use crate::ui::effects::gradient::{Gradient, GradientDirection};

        let gradient = Gradient::linear(
            Color::Rgb(255, 0, 0),
            Color::Rgb(0, 0, 255),
            GradientDirection::Horizontal,
        );

        let border = EnhancedBorder::new()
            .gradient(gradient)
            .effect(BorderEffect::Glow)
            .rounded(true);

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

        let fallback_mode = FallbackMode::from_capabilities(&caps);
        let fallback_border = border.with_fallback(&fallback_mode, &caps);

        // Gradient should be disabled on minimal terminal
        assert!(!fallback_border.has_gradient());
        // Effect should be removed
        assert_eq!(fallback_border.get_effect(), BorderEffect::None);
        // Corners should be simplified
        assert_eq!(fallback_border.get_corner_style(), CornerStyle::Sharp);
    }

    #[test]
    fn test_predefined_toad_brand() {
        let border = BorderStyles::toad_brand();
        assert!(border.has_gradient());
        assert_eq!(border.get_corner_style(), CornerStyle::Rounded);
        assert_eq!(border.get_effect(), BorderEffect::Glow);
    }

    #[test]
    fn test_predefined_success() {
        let border = BorderStyles::success();
        assert_eq!(border.color_at(0.5), Color::Green);
        assert_eq!(border.get_effect(), BorderEffect::Glow);
    }

    #[test]
    fn test_predefined_error() {
        let border = BorderStyles::error();
        assert_eq!(border.color_at(0.5), Color::Red);
        assert_eq!(border.get_effect(), BorderEffect::Shadow);
        assert_eq!(border.get_thickness(), BorderThickness::Thick);
    }

    #[test]
    fn test_predefined_warning() {
        let border = BorderStyles::warning();
        assert_eq!(border.color_at(0.5), Color::Yellow);
        assert_eq!(border.get_effect(), BorderEffect::Pulse);
    }

    #[test]
    fn test_predefined_subtle() {
        let border = BorderStyles::subtle();
        assert_eq!(border.get_thickness(), BorderThickness::Thin);
    }

    #[test]
    fn test_default() {
        let border = EnhancedBorder::default();
        assert!(border.is_enabled());
        assert!(!border.has_gradient());
    }
}
