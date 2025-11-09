//! Fallback mode for graceful degradation on limited terminals
//!
//! Provides adaptive rendering strategies based on terminal capabilities,
//! ensuring TOAD works on all terminal types from dumb to modern.
//!
//! # Examples
//!
//! ```
//! use toad::infrastructure::{FallbackMode, TerminalCapabilities};
//!
//! let caps = TerminalCapabilities::detect();
//! let fallback = FallbackMode::from_capabilities(&caps);
//! ```

use super::terminal_capabilities::{ColorSupport, FeatureLevel, TerminalCapabilities};
use ratatui::style::Color;

/// Fallback rendering mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FallbackMode {
    /// Full features - all bells and whistles
    Full,
    /// Standard features - 256 colors, Unicode, no gradients
    Standard,
    /// Basic features - 16 colors, ASCII fallbacks
    Basic,
    /// Minimal features - text only, no colors
    Minimal,
}

impl FallbackMode {
    /// Create fallback mode from terminal capabilities
    pub fn from_capabilities(caps: &TerminalCapabilities) -> Self {
        match caps.feature_level() {
            FeatureLevel::Full => FallbackMode::Full,
            FeatureLevel::Standard => FallbackMode::Standard,
            FeatureLevel::Basic => FallbackMode::Basic,
            FeatureLevel::Minimal => FallbackMode::Minimal,
        }
    }

    /// Check if animations should be enabled
    pub fn enable_animations(&self) -> bool {
        matches!(self, FallbackMode::Full | FallbackMode::Standard)
    }

    /// Check if Unicode box-drawing characters should be used
    pub fn use_unicode_borders(&self) -> bool {
        !matches!(self, FallbackMode::Minimal)
    }

    /// Check if colors should be used
    pub fn use_colors(&self) -> bool {
        !matches!(self, FallbackMode::Minimal)
    }

    /// Check if icons should be shown
    pub fn show_icons(&self) -> bool {
        !matches!(self, FallbackMode::Minimal | FallbackMode::Basic)
    }

    /// Check if gradients should be rendered
    pub fn use_gradients(&self) -> bool {
        matches!(self, FallbackMode::Full)
    }

    /// Get fallback color for a given color
    pub fn fallback_color(&self, color: Color, caps: &TerminalCapabilities) -> Color {
        match caps.color_support {
            ColorSupport::None => Color::Reset,
            ColorSupport::Basic16 => Self::to_16_color(color),
            ColorSupport::Colors256 => Self::to_256_color(color),
            ColorSupport::TrueColor => color,
        }
    }

    /// Convert RGB color to 16-color equivalent
    fn to_16_color(color: Color) -> Color {
        match color {
            Color::Rgb(r, g, b) => {
                // Simple brightness-based mapping
                let brightness = (r as u16 + g as u16 + b as u16) / 3;

                // Check which component is dominant
                let max = r.max(g).max(b);

                if brightness < 64 {
                    Color::Black
                } else if brightness > 192 {
                    Color::White
                } else if r == max && r > g + 50 && r > b + 50 {
                    Color::Red
                } else if g == max && g > r + 50 && g > b + 50 {
                    Color::Green
                } else if b == max && b > r + 50 && b > g + 50 {
                    Color::Blue
                } else if r > 128 && g > 128 && b < 128 {
                    Color::Yellow
                } else if r > 128 && b > 128 && g < 128 {
                    Color::Magenta
                } else if g > 128 && b > 128 && r < 128 {
                    Color::Cyan
                } else {
                    Color::Gray
                }
            }
            _ => color, // Already a basic color
        }
    }

    /// Convert RGB color to 256-color equivalent
    fn to_256_color(color: Color) -> Color {
        match color {
            Color::Rgb(r, g, b) => {
                // Use indexed color approximation
                // 256-color palette: 0-15 are standard, 16-231 are 6x6x6 cube, 232-255 are grayscale
                let r_idx = ((r as u16 * 5) / 255) as u8;
                let g_idx = ((g as u16 * 5) / 255) as u8;
                let b_idx = ((b as u16 * 5) / 255) as u8;

                let idx = 16 + 36 * r_idx + 6 * g_idx + b_idx;
                Color::Indexed(idx)
            }
            _ => color,
        }
    }

    /// Get box-drawing characters for this mode
    pub fn box_chars(&self) -> BoxChars {
        if self.use_unicode_borders() {
            BoxChars::unicode()
        } else {
            BoxChars::ascii()
        }
    }

    /// Get spinner frames for this mode
    pub fn spinner_frames(&self) -> &'static [&'static str] {
        if self.show_icons() {
            &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]
        } else if self.use_unicode_borders() {
            &["|", "/", "-", "\\"]
        } else {
            &["-", "\\", "|", "/"]
        }
    }

    /// Get icon fallback
    pub fn icon_fallback<'a>(&self, unicode_icon: &'a str, ascii_fallback: &'a str) -> &'a str {
        if self.show_icons() {
            unicode_icon
        } else {
            ascii_fallback
        }
    }
}

/// Box-drawing character set
#[derive(Debug, Clone, Copy)]
pub struct BoxChars {
    pub horizontal: char,
    pub vertical: char,
    pub top_left: char,
    pub top_right: char,
    pub bottom_left: char,
    pub bottom_right: char,
    pub cross: char,
    pub t_down: char,
    pub t_up: char,
    pub t_left: char,
    pub t_right: char,
}

impl BoxChars {
    /// Unicode box-drawing characters (─│┌┐└┘┼)
    pub fn unicode() -> Self {
        Self {
            horizontal: '─',
            vertical: '│',
            top_left: '┌',
            top_right: '┐',
            bottom_left: '└',
            bottom_right: '┘',
            cross: '┼',
            t_down: '┬',
            t_up: '┴',
            t_left: '┤',
            t_right: '├',
        }
    }

    /// ASCII fallback characters (-|+)
    pub fn ascii() -> Self {
        Self {
            horizontal: '-',
            vertical: '|',
            top_left: '+',
            top_right: '+',
            bottom_left: '+',
            bottom_right: '+',
            cross: '+',
            t_down: '+',
            t_up: '+',
            t_left: '+',
            t_right: '+',
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fallback_mode_from_capabilities() {
        let rich_caps = TerminalCapabilities {
            color_support: ColorSupport::TrueColor,
            unicode_support: true,
            mouse_support: true,
            alternate_screen: true,
            styled_underlines: true,
            bracketed_paste: true,
            term_name: "xterm-256color".to_string(),
            term_program: Some("iTerm".to_string()),
            nerd_fonts: true,
        };

        let mode = FallbackMode::from_capabilities(&rich_caps);
        assert_eq!(mode, FallbackMode::Full);

        let basic_caps = TerminalCapabilities {
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

        let mode = FallbackMode::from_capabilities(&basic_caps);
        assert_eq!(mode, FallbackMode::Minimal);
    }

    #[test]
    fn test_enable_animations() {
        assert!(FallbackMode::Full.enable_animations());
        assert!(FallbackMode::Standard.enable_animations());
        assert!(!FallbackMode::Basic.enable_animations());
        assert!(!FallbackMode::Minimal.enable_animations());
    }

    #[test]
    fn test_use_unicode_borders() {
        assert!(FallbackMode::Full.use_unicode_borders());
        assert!(FallbackMode::Standard.use_unicode_borders());
        assert!(FallbackMode::Basic.use_unicode_borders());
        assert!(!FallbackMode::Minimal.use_unicode_borders());
    }

    #[test]
    fn test_show_icons() {
        assert!(FallbackMode::Full.show_icons());
        assert!(FallbackMode::Standard.show_icons());
        assert!(!FallbackMode::Basic.show_icons());
        assert!(!FallbackMode::Minimal.show_icons());
    }

    #[test]
    fn test_use_gradients() {
        assert!(FallbackMode::Full.use_gradients());
        assert!(!FallbackMode::Standard.use_gradients());
        assert!(!FallbackMode::Basic.use_gradients());
        assert!(!FallbackMode::Minimal.use_gradients());
    }

    #[test]
    fn test_fallback_color_none() {
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

        let mode = FallbackMode::Minimal;
        let color = Color::Rgb(255, 0, 0);
        let fallback = mode.fallback_color(color, &caps);

        assert_eq!(fallback, Color::Reset);
    }

    #[test]
    fn test_fallback_color_16() {
        let caps = TerminalCapabilities {
            color_support: ColorSupport::Basic16,
            unicode_support: true,
            mouse_support: true,
            alternate_screen: true,
            styled_underlines: false,
            bracketed_paste: true,
            term_name: "xterm".to_string(),
            term_program: None,
            nerd_fonts: false,
        };

        let mode = FallbackMode::Basic;

        // Red
        let red = Color::Rgb(255, 0, 0);
        let fallback = mode.fallback_color(red, &caps);
        assert_eq!(fallback, Color::Red);

        // Green
        let green = Color::Rgb(0, 255, 0);
        let fallback = mode.fallback_color(green, &caps);
        assert_eq!(fallback, Color::Green);

        // Blue
        let blue = Color::Rgb(0, 0, 255);
        let fallback = mode.fallback_color(blue, &caps);
        assert_eq!(fallback, Color::Blue);
    }

    #[test]
    fn test_box_chars_unicode() {
        let mode = FallbackMode::Full;
        let chars = mode.box_chars();

        assert_eq!(chars.horizontal, '─');
        assert_eq!(chars.vertical, '│');
        assert_eq!(chars.top_left, '┌');
    }

    #[test]
    fn test_box_chars_ascii() {
        let mode = FallbackMode::Minimal;
        let chars = mode.box_chars();

        assert_eq!(chars.horizontal, '-');
        assert_eq!(chars.vertical, '|');
        assert_eq!(chars.top_left, '+');
    }

    #[test]
    fn test_spinner_frames() {
        let full_mode = FallbackMode::Full;
        let frames = full_mode.spinner_frames();
        assert!(frames.len() > 4);
        assert!(frames[0].contains('⠋') || frames[0].contains('⠙'));

        let minimal_mode = FallbackMode::Minimal;
        let frames = minimal_mode.spinner_frames();
        assert_eq!(frames.len(), 4);
        assert_eq!(frames[0], "-");
    }

    #[test]
    fn test_icon_fallback() {
        let full_mode = FallbackMode::Full;
        assert_eq!(full_mode.icon_fallback("✓", "[x]"), "✓");

        let minimal_mode = FallbackMode::Minimal;
        assert_eq!(minimal_mode.icon_fallback("✓", "[x]"), "[x]");
    }

    #[test]
    fn test_use_colors() {
        assert!(FallbackMode::Full.use_colors());
        assert!(FallbackMode::Standard.use_colors());
        assert!(FallbackMode::Basic.use_colors());
        assert!(!FallbackMode::Minimal.use_colors());
    }
}
