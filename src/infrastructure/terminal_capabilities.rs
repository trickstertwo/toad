//! Terminal capability detection
//!
//! Detects terminal capabilities and features to enable adaptive rendering
//! and graceful degradation on limited terminals.
//!
//! # Examples
//!
//! ```
//! use toad::infrastructure::TerminalCapabilities;
//!
//! let caps = TerminalCapabilities::detect();
//! if caps.supports_truecolor() {
//!     // Use RGB colors
//! }
//! ```

use std::env;

/// Color support level
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ColorSupport {
    /// No color support (1-bit)
    None,
    /// 16 basic colors
    Basic16,
    /// 256 colors (8-bit)
    Colors256,
    /// True color RGB (24-bit)
    TrueColor,
}

impl ColorSupport {
    /// Check if this level supports at least basic colors
    pub fn supports_color(&self) -> bool {
        !matches!(self, ColorSupport::None)
    }

    /// Check if this level supports 256 colors
    pub fn supports_256(&self) -> bool {
        matches!(self, ColorSupport::Colors256 | ColorSupport::TrueColor)
    }

    /// Check if this level supports true color
    pub fn supports_truecolor(&self) -> bool {
        matches!(self, ColorSupport::TrueColor)
    }
}

/// Terminal capability information
#[derive(Debug, Clone)]
pub struct TerminalCapabilities {
    /// Color support level
    pub color_support: ColorSupport,
    /// Whether terminal supports Unicode
    pub unicode_support: bool,
    /// Whether terminal supports mouse events
    pub mouse_support: bool,
    /// Whether terminal supports alternate screen
    pub alternate_screen: bool,
    /// Whether terminal supports styled underlines
    pub styled_underlines: bool,
    /// Whether terminal supports bracketed paste
    pub bracketed_paste: bool,
    /// Terminal name (from $TERM)
    pub term_name: String,
    /// Terminal program (from various env vars)
    pub term_program: Option<String>,
    /// Whether Nerd Fonts are likely supported
    pub nerd_fonts: bool,
}

impl TerminalCapabilities {
    /// Detect terminal capabilities
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::infrastructure::TerminalCapabilities;
    ///
    /// let caps = TerminalCapabilities::detect();
    /// println!("Color support: {:?}", caps.color_support);
    /// ```
    pub fn detect() -> Self {
        let term_name = env::var("TERM").unwrap_or_else(|_| "dumb".to_string());
        let term_program = env::var("TERM_PROGRAM")
            .ok()
            .or_else(|| env::var("TERMINAL_EMULATOR").ok())
            .or_else(|| {
                env::var("WT_SESSION")
                    .ok()
                    .map(|_| "Windows Terminal".to_string())
            });

        let color_support = Self::detect_color_support(&term_name);
        let unicode_support = Self::detect_unicode_support();
        let mouse_support = Self::detect_mouse_support(&term_name);
        let alternate_screen = Self::detect_alternate_screen(&term_name);
        let styled_underlines = Self::detect_styled_underlines(&term_name, term_program.as_deref());
        let bracketed_paste = Self::detect_bracketed_paste(&term_name);
        let nerd_fonts = Self::detect_nerd_fonts(term_program.as_deref());

        Self {
            color_support,
            unicode_support,
            mouse_support,
            alternate_screen,
            styled_underlines,
            bracketed_paste,
            term_name,
            term_program,
            nerd_fonts,
        }
    }

    /// Detect color support level
    fn detect_color_support(term_name: &str) -> ColorSupport {
        // Check COLORTERM for truecolor
        if let Ok(colorterm) = env::var("COLORTERM")
            && (colorterm.contains("truecolor") || colorterm.contains("24bit"))
        {
            return ColorSupport::TrueColor;
        }

        // Check TERM for color hints
        if term_name.contains("truecolor") || term_name.contains("24bit") {
            return ColorSupport::TrueColor;
        }

        if term_name.contains("256color") {
            return ColorSupport::Colors256;
        }

        if term_name.contains("color") {
            return ColorSupport::Basic16;
        }

        // Check for dumb terminal
        if term_name == "dumb" || term_name.is_empty() {
            return ColorSupport::None;
        }

        // Default to 256 color for modern terminals
        if term_name.starts_with("xterm") || term_name.starts_with("screen") {
            return ColorSupport::Colors256;
        }

        ColorSupport::Basic16
    }

    /// Detect Unicode support
    fn detect_unicode_support() -> bool {
        // Check locale for UTF-8
        if let Ok(lang) = env::var("LANG")
            && (lang.to_uppercase().contains("UTF-8") || lang.to_uppercase().contains("UTF8"))
        {
            return true;
        }

        if let Ok(lc_all) = env::var("LC_ALL")
            && (lc_all.to_uppercase().contains("UTF-8") || lc_all.to_uppercase().contains("UTF8"))
        {
            return true;
        }

        // Default to true for modern systems
        true
    }

    /// Detect mouse support
    fn detect_mouse_support(term_name: &str) -> bool {
        // Most modern terminals support mouse
        !matches!(term_name, "dumb" | "")
    }

    /// Detect alternate screen support
    fn detect_alternate_screen(term_name: &str) -> bool {
        // Dumb terminals don't support alternate screen
        term_name != "dumb" && !term_name.is_empty()
    }

    /// Detect styled underlines (curly, colored, etc.)
    fn detect_styled_underlines(term_name: &str, term_program: Option<&str>) -> bool {
        // Known terminals with styled underline support
        if let Some(program) = term_program
            && (program.contains("iTerm")
                || program.contains("WezTerm")
                || program.contains("kitty")
                || program.contains("Alacritty"))
        {
            return true;
        }

        term_name.contains("kitty") || term_name.contains("wezterm")
    }

    /// Detect bracketed paste support
    fn detect_bracketed_paste(term_name: &str) -> bool {
        // Most modern terminals support bracketed paste
        !matches!(term_name, "dumb" | "")
    }

    /// Detect Nerd Fonts support
    fn detect_nerd_fonts(term_program: Option<&str>) -> bool {
        // Check for terminals known to support Nerd Fonts well
        if let Some(program) = term_program
            && (program.contains("iTerm")
                || program.contains("WezTerm")
                || program.contains("kitty")
                || program.contains("Alacritty")
                || program.contains("Windows Terminal"))
        {
            return true;
        }

        // Check environment variable
        if let Ok(nerd_fonts) = env::var("NERD_FONTS")
            && (nerd_fonts == "1" || nerd_fonts.to_lowercase() == "true")
        {
            return true;
        }

        // Conservative default
        false
    }

    /// Check if terminal supports true color
    pub fn supports_truecolor(&self) -> bool {
        self.color_support.supports_truecolor()
    }

    /// Check if terminal supports 256 colors
    pub fn supports_256_colors(&self) -> bool {
        self.color_support.supports_256()
    }

    /// Check if terminal supports any colors
    pub fn supports_color(&self) -> bool {
        self.color_support.supports_color()
    }

    /// Check if terminal supports Unicode
    pub fn supports_unicode(&self) -> bool {
        self.unicode_support
    }

    /// Check if terminal supports mouse
    pub fn supports_mouse(&self) -> bool {
        self.mouse_support
    }

    /// Check if terminal is "rich" (supports most modern features)
    pub fn is_rich_terminal(&self) -> bool {
        self.supports_256_colors()
            && self.supports_unicode()
            && self.mouse_support
            && self.alternate_screen
    }

    /// Check if terminal is "basic" (limited features)
    pub fn is_basic_terminal(&self) -> bool {
        !self.supports_color() || !self.supports_unicode()
    }

    /// Get recommended feature level
    pub fn feature_level(&self) -> FeatureLevel {
        if self.is_basic_terminal() {
            FeatureLevel::Minimal
        } else if !self.supports_256_colors() {
            FeatureLevel::Basic
        } else if !self.supports_truecolor() {
            FeatureLevel::Standard
        } else {
            FeatureLevel::Full
        }
    }
}

/// Feature level recommendation
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FeatureLevel {
    /// Minimal features (text-only)
    Minimal,
    /// Basic features (16 colors, ASCII)
    Basic,
    /// Standard features (256 colors, Unicode)
    Standard,
    /// Full features (truecolor, all bells and whistles)
    Full,
}

impl FeatureLevel {
    /// Check if animations should be enabled
    pub fn enable_animations(&self) -> bool {
        matches!(self, FeatureLevel::Standard | FeatureLevel::Full)
    }

    /// Check if fancy borders should be used
    pub fn use_fancy_borders(&self) -> bool {
        matches!(self, FeatureLevel::Standard | FeatureLevel::Full)
    }

    /// Check if gradients should be used
    pub fn use_gradients(&self) -> bool {
        matches!(self, FeatureLevel::Full)
    }

    /// Check if icons should be shown
    pub fn show_icons(&self) -> bool {
        !matches!(self, FeatureLevel::Minimal)
    }
}

impl Default for TerminalCapabilities {
    fn default() -> Self {
        Self::detect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_support_levels() {
        assert!(ColorSupport::TrueColor.supports_truecolor());
        assert!(ColorSupport::TrueColor.supports_256());
        assert!(ColorSupport::TrueColor.supports_color());

        assert!(!ColorSupport::Colors256.supports_truecolor());
        assert!(ColorSupport::Colors256.supports_256());
        assert!(ColorSupport::Colors256.supports_color());

        assert!(!ColorSupport::Basic16.supports_truecolor());
        assert!(!ColorSupport::Basic16.supports_256());
        assert!(ColorSupport::Basic16.supports_color());

        assert!(!ColorSupport::None.supports_color());
    }

    #[test]
    fn test_terminal_capabilities_detect() {
        let caps = TerminalCapabilities::detect();
        // Should always have some value
        assert!(!caps.term_name.is_empty() || caps.term_name == "dumb");
    }

    #[test]
    fn test_feature_level_animations() {
        assert!(!FeatureLevel::Minimal.enable_animations());
        assert!(!FeatureLevel::Basic.enable_animations());
        assert!(FeatureLevel::Standard.enable_animations());
        assert!(FeatureLevel::Full.enable_animations());
    }

    #[test]
    fn test_feature_level_gradients() {
        assert!(!FeatureLevel::Minimal.use_gradients());
        assert!(!FeatureLevel::Basic.use_gradients());
        assert!(!FeatureLevel::Standard.use_gradients());
        assert!(FeatureLevel::Full.use_gradients());
    }

    #[test]
    fn test_feature_level_icons() {
        assert!(!FeatureLevel::Minimal.show_icons());
        assert!(FeatureLevel::Basic.show_icons());
        assert!(FeatureLevel::Standard.show_icons());
        assert!(FeatureLevel::Full.show_icons());
    }

    #[test]
    fn test_is_rich_terminal() {
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

        assert!(rich_caps.is_rich_terminal());
    }

    #[test]
    fn test_is_basic_terminal() {
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

        assert!(basic_caps.is_basic_terminal());
    }

    #[test]
    fn test_feature_level_detection() {
        let full_caps = TerminalCapabilities {
            color_support: ColorSupport::TrueColor,
            unicode_support: true,
            mouse_support: true,
            alternate_screen: true,
            styled_underlines: true,
            bracketed_paste: true,
            term_name: "xterm-256color".to_string(),
            term_program: None,
            nerd_fonts: false,
        };

        assert_eq!(full_caps.feature_level(), FeatureLevel::Full);

        let minimal_caps = TerminalCapabilities {
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

        assert_eq!(minimal_caps.feature_level(), FeatureLevel::Minimal);
    }
}
