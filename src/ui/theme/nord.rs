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
    fn test_nord_metadata() {
        let theme = NordTheme;
        assert_eq!(theme.name(), "Nord");
        assert!(theme.description().contains("Nord"));
        assert!(theme.description().contains("Arctic"));
    }

    #[test]
    fn test_nord_primary_colors() {
        let theme = NordTheme;
        assert_eq!(theme.primary(), Color::Rgb(143, 188, 187)); // Nord8 - Frost cyan
        assert_eq!(theme.primary_bright(), Color::Rgb(163, 190, 140)); // Nord14 - Aurora green
        assert_eq!(theme.primary_dark(), Color::Rgb(94, 129, 172)); // Nord9 - Frost blue
    }

    #[test]
    fn test_nord_grayscale() {
        let theme = NordTheme;
        assert_eq!(theme.white(), Color::Rgb(236, 239, 244)); // Nord6
        assert_eq!(theme.light_gray(), Color::Rgb(229, 233, 240)); // Nord5
        assert_eq!(theme.gray(), Color::Rgb(216, 222, 233)); // Nord4
        assert_eq!(theme.dark_gray(), Color::Rgb(76, 86, 106)); // Nord3
        assert_eq!(theme.darker_gray(), Color::Rgb(67, 76, 94)); // Nord2
        assert_eq!(theme.black(), Color::Rgb(46, 52, 64)); // Nord0
    }

    #[test]
    fn test_nord_semantic_colors() {
        let theme = NordTheme;
        assert_eq!(theme.success(), Color::Rgb(163, 190, 140)); // Nord14 - Aurora green
        assert_eq!(theme.error(), Color::Rgb(191, 97, 106)); // Nord11 - Aurora red
        assert_eq!(theme.warning(), Color::Rgb(235, 203, 139)); // Nord13 - Aurora yellow
        assert_eq!(theme.info(), Color::Rgb(136, 192, 208)); // Nord8 - Frost
    }

    #[test]
    fn test_nord_additional_colors() {
        let theme = NordTheme;
        assert_eq!(theme.red(), Color::Rgb(191, 97, 106)); // Nord11
        assert_eq!(theme.yellow(), Color::Rgb(235, 203, 139)); // Nord13
        assert_eq!(theme.blue(), Color::Rgb(136, 192, 208)); // Nord8
        assert_eq!(theme.green(), Color::Rgb(163, 190, 140)); // Nord14
        assert_eq!(theme.cyan(), Color::Rgb(143, 188, 187)); // Nord7
        assert_eq!(theme.magenta(), Color::Rgb(180, 142, 173)); // Nord15
    }

    #[test]
    fn test_nord_ui_colors() {
        let theme = NordTheme;
        assert_eq!(theme.background(), Color::Rgb(46, 52, 64)); // Nord0
        assert_eq!(theme.foreground(), Color::Rgb(216, 222, 233)); // Nord4
        assert_eq!(theme.border(), Color::Rgb(67, 76, 94)); // Nord2
        assert_eq!(theme.border_focused(), Color::Rgb(136, 192, 208)); // Nord8
        assert_eq!(theme.title(), Color::Rgb(236, 239, 244)); // Nord6
        assert_eq!(theme.accent(), Color::Rgb(136, 192, 208)); // Nord8
    }

    #[test]
    fn test_nord_background_is_dark() {
        let theme = NordTheme;
        match theme.background() {
            Color::Rgb(r, g, b) => assert!(r < 100 && g < 100 && b < 100),
            _ => panic!("Expected RGB color"),
        }
    }

    #[test]
    fn test_nord_foreground_is_light() {
        let theme = NordTheme;
        match theme.foreground() {
            Color::Rgb(r, g, b) => assert!(r > 200 && g > 200 && b > 200),
            _ => panic!("Expected RGB color"),
        }
    }

    #[test]
    fn test_nord_frost_colors() {
        let theme = NordTheme;
        // Verify Nord Frost palette (Nord7-10) is used
        assert_eq!(theme.cyan(), Color::Rgb(143, 188, 187)); // Nord7
        assert_eq!(theme.blue(), Color::Rgb(136, 192, 208)); // Nord8
        assert_eq!(theme.primary_dark(), Color::Rgb(94, 129, 172)); // Nord9
    }

    #[test]
    fn test_nord_aurora_colors() {
        let theme = NordTheme;
        // Verify Nord Aurora palette (Nord11-15) is used
        assert_eq!(theme.error(), Color::Rgb(191, 97, 106)); // Nord11 red
        assert_eq!(theme.warning(), Color::Rgb(235, 203, 139)); // Nord13 yellow
        assert_eq!(theme.success(), Color::Rgb(163, 190, 140)); // Nord14 green
        assert_eq!(theme.magenta(), Color::Rgb(180, 142, 173)); // Nord15 purple
    }

    #[test]
    fn test_nord_multiple_instances() {
        let theme1 = NordTheme;
        let theme2 = NordTheme;

        // Multiple instances should have identical colors
        assert_eq!(theme1.primary(), theme2.primary());
        assert_eq!(theme1.background(), theme2.background());
    }

    #[test]
    fn test_nord_all_colors_are_rgb() {
        let theme = NordTheme;

        // Verify all colors are RGB variants
        assert!(matches!(theme.primary(), Color::Rgb(..)));
        assert!(matches!(theme.background(), Color::Rgb(..)));
        assert!(matches!(theme.foreground(), Color::Rgb(..)));
        assert!(matches!(theme.success(), Color::Rgb(..)));
        assert!(matches!(theme.error(), Color::Rgb(..)));
        assert!(matches!(theme.warning(), Color::Rgb(..)));
        assert!(matches!(theme.info(), Color::Rgb(..)));
    }

    #[test]
    fn test_nord_border_colors_distinct() {
        let theme = NordTheme;

        let border = theme.border();
        let border_focused = theme.border_focused();

        // Border and focused border should be different
        assert_ne!(border, border_focused);
    }
}
