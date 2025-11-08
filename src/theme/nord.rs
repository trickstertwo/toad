/// Nord theme
///
/// Arctic, north-bluish color palette
/// Based on https://www.nordtheme.com/

use super::Theme;
use ratatui::style::Color;

/// Nord dark theme
pub struct NordTheme;

impl Theme for NordTheme {
    fn name(&self) -> &str {
        "Nord"
    }

    fn description(&self) -> &str {
        "Nord - Arctic, north-bluish dark theme"
    }

    fn primary(&self) -> Color {
        Color::Rgb(143, 188, 187) // Nord8 - Frost cyan
    }

    fn primary_bright(&self) -> Color {
        Color::Rgb(163, 190, 140) // Nord14 - Aurora green
    }

    fn primary_dark(&self) -> Color {
        Color::Rgb(94, 129, 172) // Nord9 - Frost blue
    }

    fn white(&self) -> Color {
        Color::Rgb(236, 239, 244) // Nord6 - Snow storm light
    }

    fn light_gray(&self) -> Color {
        Color::Rgb(229, 233, 240) // Nord5 - Snow storm medium
    }

    fn gray(&self) -> Color {
        Color::Rgb(216, 222, 233) // Nord4 - Snow storm dark
    }

    fn dark_gray(&self) -> Color {
        Color::Rgb(76, 86, 106) // Nord3 - Polar night bright
    }

    fn darker_gray(&self) -> Color {
        Color::Rgb(67, 76, 94) // Nord2 - Polar night medium
    }

    fn black(&self) -> Color {
        Color::Rgb(46, 52, 64) // Nord0 - Polar night dark
    }

    fn success(&self) -> Color {
        Color::Rgb(163, 190, 140) // Nord14 - Aurora green
    }

    fn error(&self) -> Color {
        Color::Rgb(191, 97, 106) // Nord11 - Aurora red
    }

    fn warning(&self) -> Color {
        Color::Rgb(235, 203, 139) // Nord13 - Aurora yellow
    }

    fn info(&self) -> Color {
        Color::Rgb(136, 192, 208) // Nord8 - Frost
    }

    fn red(&self) -> Color {
        Color::Rgb(191, 97, 106) // Nord11
    }

    fn yellow(&self) -> Color {
        Color::Rgb(235, 203, 139) // Nord13
    }

    fn blue(&self) -> Color {
        Color::Rgb(136, 192, 208) // Nord8
    }

    fn green(&self) -> Color {
        Color::Rgb(163, 190, 140) // Nord14
    }

    fn cyan(&self) -> Color {
        Color::Rgb(143, 188, 187) // Nord7
    }

    fn magenta(&self) -> Color {
        Color::Rgb(180, 142, 173) // Nord15 - Aurora purple
    }

    fn background(&self) -> Color {
        Color::Rgb(46, 52, 64) // Nord0
    }

    fn foreground(&self) -> Color {
        Color::Rgb(216, 222, 233) // Nord4
    }

    fn border(&self) -> Color {
        Color::Rgb(67, 76, 94) // Nord2
    }

    fn border_focused(&self) -> Color {
        Color::Rgb(136, 192, 208) // Nord8 - Frost
    }

    fn title(&self) -> Color {
        Color::Rgb(236, 239, 244) // Nord6
    }

    fn accent(&self) -> Color {
        Color::Rgb(136, 192, 208) // Nord8 - Frost
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nord_theme() {
        let theme = NordTheme;
        assert_eq!(theme.name(), "Nord");
        assert!(theme.description().contains("Nord"));
        assert_eq!(theme.background(), Color::Rgb(46, 52, 64));
    }

    #[test]
    fn test_nord_colors() {
        let theme = NordTheme;
        // Verify some key Nord colors
        assert_eq!(theme.error(), Color::Rgb(191, 97, 106)); // Aurora red
        assert_eq!(theme.success(), Color::Rgb(163, 190, 140)); // Aurora green
        assert_eq!(theme.warning(), Color::Rgb(235, 203, 139)); // Aurora yellow
    }
}
