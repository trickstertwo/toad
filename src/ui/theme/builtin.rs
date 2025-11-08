/// Built-in themes: Dark, Light, and High-Contrast
///
/// Provides standard color schemes for different preferences
use super::Theme;
use ratatui::style::Color;

/// Dark theme (default) - Toad green with dark grayscale
pub struct DarkTheme;

impl Theme for DarkTheme {
    fn name(&self) -> &str {
        "Dark"
    }

    fn description(&self) -> &str {
        "Dark theme with toad green accents (default)"
    }

    fn primary(&self) -> Color {
        Color::Rgb(76, 175, 80) // Toad green
    }

    fn primary_bright(&self) -> Color {
        Color::Rgb(129, 199, 132)
    }

    fn primary_dark(&self) -> Color {
        Color::Rgb(56, 142, 60)
    }

    fn white(&self) -> Color {
        Color::Rgb(255, 255, 255)
    }

    fn light_gray(&self) -> Color {
        Color::Rgb(189, 189, 189)
    }

    fn gray(&self) -> Color {
        Color::Rgb(158, 158, 158)
    }

    fn dark_gray(&self) -> Color {
        Color::Rgb(97, 97, 97)
    }

    fn darker_gray(&self) -> Color {
        Color::Rgb(66, 66, 66)
    }

    fn black(&self) -> Color {
        Color::Rgb(33, 33, 33)
    }

    fn success(&self) -> Color {
        self.primary()
    }

    fn error(&self) -> Color {
        Color::Rgb(244, 67, 54) // Red
    }

    fn warning(&self) -> Color {
        Color::Rgb(255, 152, 0) // Orange
    }

    fn info(&self) -> Color {
        self.light_gray()
    }

    fn red(&self) -> Color {
        Color::Rgb(244, 67, 54)
    }

    fn yellow(&self) -> Color {
        Color::Rgb(255, 193, 7)
    }

    fn blue(&self) -> Color {
        Color::Rgb(33, 150, 243)
    }

    fn green(&self) -> Color {
        self.primary()
    }

    fn cyan(&self) -> Color {
        Color::Rgb(0, 188, 212)
    }

    fn magenta(&self) -> Color {
        Color::Rgb(233, 30, 99)
    }

    fn background(&self) -> Color {
        self.black()
    }

    fn foreground(&self) -> Color {
        self.light_gray()
    }

    fn border(&self) -> Color {
        self.dark_gray()
    }

    fn border_focused(&self) -> Color {
        self.primary()
    }

    fn title(&self) -> Color {
        self.white()
    }

    fn accent(&self) -> Color {
        self.primary()
    }
}

/// Light theme - bright background with dark text
pub struct LightTheme;

impl Theme for LightTheme {
    fn name(&self) -> &str {
        "Light"
    }

    fn description(&self) -> &str {
        "Light theme with bright background and toad green accents"
    }

    fn primary(&self) -> Color {
        Color::Rgb(56, 142, 60) // Darker toad green for visibility
    }

    fn primary_bright(&self) -> Color {
        Color::Rgb(76, 175, 80)
    }

    fn primary_dark(&self) -> Color {
        Color::Rgb(46, 125, 50)
    }

    fn white(&self) -> Color {
        Color::Rgb(33, 33, 33) // Dark for text on light bg
    }

    fn light_gray(&self) -> Color {
        Color::Rgb(97, 97, 97)
    }

    fn gray(&self) -> Color {
        Color::Rgb(158, 158, 158)
    }

    fn dark_gray(&self) -> Color {
        Color::Rgb(189, 189, 189)
    }

    fn darker_gray(&self) -> Color {
        Color::Rgb(224, 224, 224)
    }

    fn black(&self) -> Color {
        Color::Rgb(250, 250, 250) // Off-white for bg
    }

    fn success(&self) -> Color {
        Color::Rgb(56, 142, 60)
    }

    fn error(&self) -> Color {
        Color::Rgb(198, 40, 40) // Darker red
    }

    fn warning(&self) -> Color {
        Color::Rgb(230, 126, 0) // Darker orange
    }

    fn info(&self) -> Color {
        Color::Rgb(25, 118, 210) // Blue
    }

    fn red(&self) -> Color {
        Color::Rgb(198, 40, 40)
    }

    fn yellow(&self) -> Color {
        Color::Rgb(245, 166, 35)
    }

    fn blue(&self) -> Color {
        Color::Rgb(25, 118, 210)
    }

    fn green(&self) -> Color {
        self.primary()
    }

    fn cyan(&self) -> Color {
        Color::Rgb(0, 151, 167)
    }

    fn magenta(&self) -> Color {
        Color::Rgb(194, 24, 91)
    }

    fn background(&self) -> Color {
        self.black() // Off-white
    }

    fn foreground(&self) -> Color {
        self.white() // Dark gray/black
    }

    fn border(&self) -> Color {
        self.gray()
    }

    fn border_focused(&self) -> Color {
        self.primary()
    }

    fn title(&self) -> Color {
        self.white() // Dark for visibility
    }

    fn accent(&self) -> Color {
        self.primary()
    }
}

/// High-contrast theme - maximum contrast for accessibility
pub struct HighContrastTheme;

impl Theme for HighContrastTheme {
    fn name(&self) -> &str {
        "High Contrast"
    }

    fn description(&self) -> &str {
        "High-contrast theme for maximum visibility and accessibility"
    }

    fn primary(&self) -> Color {
        Color::Rgb(0, 255, 0) // Bright green
    }

    fn primary_bright(&self) -> Color {
        Color::Rgb(127, 255, 127)
    }

    fn primary_dark(&self) -> Color {
        Color::Rgb(0, 200, 0)
    }

    fn white(&self) -> Color {
        Color::Rgb(255, 255, 255)
    }

    fn light_gray(&self) -> Color {
        Color::Rgb(200, 200, 200)
    }

    fn gray(&self) -> Color {
        Color::Rgb(128, 128, 128)
    }

    fn dark_gray(&self) -> Color {
        Color::Rgb(64, 64, 64)
    }

    fn darker_gray(&self) -> Color {
        Color::Rgb(32, 32, 32)
    }

    fn black(&self) -> Color {
        Color::Rgb(0, 0, 0)
    }

    fn success(&self) -> Color {
        Color::Rgb(0, 255, 0)
    }

    fn error(&self) -> Color {
        Color::Rgb(255, 0, 0)
    }

    fn warning(&self) -> Color {
        Color::Rgb(255, 255, 0)
    }

    fn info(&self) -> Color {
        Color::Rgb(0, 255, 255)
    }

    fn red(&self) -> Color {
        Color::Rgb(255, 0, 0)
    }

    fn yellow(&self) -> Color {
        Color::Rgb(255, 255, 0)
    }

    fn blue(&self) -> Color {
        Color::Rgb(0, 0, 255)
    }

    fn green(&self) -> Color {
        Color::Rgb(0, 255, 0)
    }

    fn cyan(&self) -> Color {
        Color::Rgb(0, 255, 255)
    }

    fn magenta(&self) -> Color {
        Color::Rgb(255, 0, 255)
    }

    fn background(&self) -> Color {
        Color::Rgb(0, 0, 0)
    }

    fn foreground(&self) -> Color {
        Color::Rgb(255, 255, 255)
    }

    fn border(&self) -> Color {
        Color::Rgb(128, 128, 128)
    }

    fn border_focused(&self) -> Color {
        Color::Rgb(0, 255, 0)
    }

    fn title(&self) -> Color {
        Color::Rgb(255, 255, 255)
    }

    fn accent(&self) -> Color {
        Color::Rgb(0, 255, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dark_theme() {
        let theme = DarkTheme;
        assert_eq!(theme.name(), "Dark");
        assert!(theme.description().contains("Dark"));
        assert_eq!(theme.primary(), Color::Rgb(76, 175, 80));
    }

    #[test]
    fn test_light_theme() {
        let theme = LightTheme;
        assert_eq!(theme.name(), "Light");
        assert!(theme.description().contains("Light"));
        // In light theme, background should be bright
        match theme.background() {
            Color::Rgb(r, g, b) => assert!(r > 200 && g > 200 && b > 200),
            _ => panic!("Expected RGB color"),
        }
    }

    #[test]
    fn test_high_contrast_theme() {
        let theme = HighContrastTheme;
        assert_eq!(theme.name(), "High Contrast");
        assert!(theme.description().contains("contrast"));
        assert_eq!(theme.background(), Color::Rgb(0, 0, 0));
        assert_eq!(theme.foreground(), Color::Rgb(255, 255, 255));
    }
}
