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

    // ===== DarkTheme Tests =====
    #[test]
    fn test_dark_theme_metadata() {
        let theme = DarkTheme;
        assert_eq!(theme.name(), "Dark");
        assert!(theme.description().contains("Dark"));
    }

    #[test]
    fn test_dark_theme_primary_colors() {
        let theme = DarkTheme;
        assert_eq!(theme.primary(), Color::Rgb(76, 175, 80));
        assert_eq!(theme.primary_bright(), Color::Rgb(129, 199, 132));
        assert_eq!(theme.primary_dark(), Color::Rgb(56, 142, 60));
    }

    #[test]
    fn test_dark_theme_grayscale() {
        let theme = DarkTheme;
        assert_eq!(theme.white(), Color::Rgb(255, 255, 255));
        assert_eq!(theme.light_gray(), Color::Rgb(189, 189, 189));
        assert_eq!(theme.gray(), Color::Rgb(158, 158, 158));
        assert_eq!(theme.dark_gray(), Color::Rgb(97, 97, 97));
        assert_eq!(theme.darker_gray(), Color::Rgb(66, 66, 66));
        assert_eq!(theme.black(), Color::Rgb(33, 33, 33));
    }

    #[test]
    fn test_dark_theme_semantic_colors() {
        let theme = DarkTheme;
        assert_eq!(theme.success(), theme.primary());
        assert_eq!(theme.error(), Color::Rgb(244, 67, 54));
        assert_eq!(theme.warning(), Color::Rgb(255, 152, 0));
        assert_eq!(theme.info(), theme.light_gray());
    }

    #[test]
    fn test_dark_theme_additional_colors() {
        let theme = DarkTheme;
        assert_eq!(theme.red(), Color::Rgb(244, 67, 54));
        assert_eq!(theme.yellow(), Color::Rgb(255, 193, 7));
        assert_eq!(theme.blue(), Color::Rgb(33, 150, 243));
        assert_eq!(theme.green(), theme.primary());
        assert_eq!(theme.cyan(), Color::Rgb(0, 188, 212));
        assert_eq!(theme.magenta(), Color::Rgb(233, 30, 99));
    }

    #[test]
    fn test_dark_theme_ui_colors() {
        let theme = DarkTheme;
        assert_eq!(theme.background(), theme.black());
        assert_eq!(theme.foreground(), theme.light_gray());
        assert_eq!(theme.border(), theme.dark_gray());
        assert_eq!(theme.border_focused(), theme.primary());
        assert_eq!(theme.title(), theme.white());
        assert_eq!(theme.accent(), theme.primary());
    }

    // ===== LightTheme Tests =====
    #[test]
    fn test_light_theme_metadata() {
        let theme = LightTheme;
        assert_eq!(theme.name(), "Light");
        assert!(theme.description().contains("Light"));
    }

    #[test]
    fn test_light_theme_primary_colors() {
        let theme = LightTheme;
        assert_eq!(theme.primary(), Color::Rgb(56, 142, 60));
        assert_eq!(theme.primary_bright(), Color::Rgb(76, 175, 80));
        assert_eq!(theme.primary_dark(), Color::Rgb(46, 125, 50));
    }

    #[test]
    fn test_light_theme_grayscale() {
        let theme = LightTheme;
        assert_eq!(theme.white(), Color::Rgb(33, 33, 33));
        assert_eq!(theme.light_gray(), Color::Rgb(97, 97, 97));
        assert_eq!(theme.gray(), Color::Rgb(158, 158, 158));
        assert_eq!(theme.dark_gray(), Color::Rgb(189, 189, 189));
        assert_eq!(theme.darker_gray(), Color::Rgb(224, 224, 224));
        assert_eq!(theme.black(), Color::Rgb(250, 250, 250));
    }

    #[test]
    fn test_light_theme_semantic_colors() {
        let theme = LightTheme;
        assert_eq!(theme.success(), Color::Rgb(56, 142, 60));
        assert_eq!(theme.error(), Color::Rgb(198, 40, 40));
        assert_eq!(theme.warning(), Color::Rgb(230, 126, 0));
        assert_eq!(theme.info(), Color::Rgb(25, 118, 210));
    }

    #[test]
    fn test_light_theme_additional_colors() {
        let theme = LightTheme;
        assert_eq!(theme.red(), Color::Rgb(198, 40, 40));
        assert_eq!(theme.yellow(), Color::Rgb(245, 166, 35));
        assert_eq!(theme.blue(), Color::Rgb(25, 118, 210));
        assert_eq!(theme.green(), theme.primary());
        assert_eq!(theme.cyan(), Color::Rgb(0, 151, 167));
        assert_eq!(theme.magenta(), Color::Rgb(194, 24, 91));
    }

    #[test]
    fn test_light_theme_ui_colors() {
        let theme = LightTheme;
        assert_eq!(theme.background(), theme.black());
        assert_eq!(theme.foreground(), theme.white());
        assert_eq!(theme.border(), theme.gray());
        assert_eq!(theme.border_focused(), theme.primary());
        assert_eq!(theme.title(), theme.white());
        assert_eq!(theme.accent(), theme.primary());
    }

    // ===== HighContrastTheme Tests =====
    #[test]
    fn test_high_contrast_theme_metadata() {
        let theme = HighContrastTheme;
        assert_eq!(theme.name(), "High Contrast");
        assert!(theme.description().contains("contrast"));
    }

    #[test]
    fn test_high_contrast_theme_primary_colors() {
        let theme = HighContrastTheme;
        assert_eq!(theme.primary(), Color::Rgb(0, 255, 0));
        assert_eq!(theme.primary_bright(), Color::Rgb(127, 255, 127));
        assert_eq!(theme.primary_dark(), Color::Rgb(0, 200, 0));
    }

    #[test]
    fn test_high_contrast_theme_grayscale() {
        let theme = HighContrastTheme;
        assert_eq!(theme.white(), Color::Rgb(255, 255, 255));
        assert_eq!(theme.light_gray(), Color::Rgb(200, 200, 200));
        assert_eq!(theme.gray(), Color::Rgb(128, 128, 128));
        assert_eq!(theme.dark_gray(), Color::Rgb(64, 64, 64));
        assert_eq!(theme.darker_gray(), Color::Rgb(32, 32, 32));
        assert_eq!(theme.black(), Color::Rgb(0, 0, 0));
    }

    #[test]
    fn test_high_contrast_theme_semantic_colors() {
        let theme = HighContrastTheme;
        assert_eq!(theme.success(), Color::Rgb(0, 255, 0));
        assert_eq!(theme.error(), Color::Rgb(255, 0, 0));
        assert_eq!(theme.warning(), Color::Rgb(255, 255, 0));
        assert_eq!(theme.info(), Color::Rgb(0, 255, 255));
    }

    #[test]
    fn test_high_contrast_theme_additional_colors() {
        let theme = HighContrastTheme;
        assert_eq!(theme.red(), Color::Rgb(255, 0, 0));
        assert_eq!(theme.yellow(), Color::Rgb(255, 255, 0));
        assert_eq!(theme.blue(), Color::Rgb(0, 0, 255));
        assert_eq!(theme.green(), theme.primary());
        assert_eq!(theme.cyan(), Color::Rgb(0, 255, 255));
        assert_eq!(theme.magenta(), Color::Rgb(255, 0, 255));
    }

    #[test]
    fn test_high_contrast_theme_ui_colors() {
        let theme = HighContrastTheme;
        assert_eq!(theme.background(), Color::Rgb(0, 0, 0));
        assert_eq!(theme.foreground(), Color::Rgb(255, 255, 255));
        assert_eq!(theme.border(), theme.gray());
        assert_eq!(theme.border_focused(), theme.primary());
        assert_eq!(theme.title(), theme.white());
        assert_eq!(theme.accent(), theme.primary());
    }

    #[test]
    fn test_high_contrast_theme_extreme_contrast() {
        let theme = HighContrastTheme;
        // Verify true black background
        assert_eq!(theme.background(), Color::Rgb(0, 0, 0));
        // Verify true white foreground
        assert_eq!(theme.foreground(), Color::Rgb(255, 255, 255));
        // Verify pure green primary
        assert_eq!(theme.primary(), Color::Rgb(0, 255, 0));
    }
}
