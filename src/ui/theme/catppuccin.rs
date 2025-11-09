/// Catppuccin theme family
///
/// Soothing pastel themes with 4 variants: Latte, Frappe, Macchiato, and Mocha
/// Based on https://github.com/catppuccin/catppuccin
use super::Theme;
use ratatui::style::Color;

/// Catppuccin Mocha (dark)
pub struct CatppuccinMocha;

impl Theme for CatppuccinMocha {
    fn name(&self) -> &str {
        "Catppuccin Mocha"
    }

    fn description(&self) -> &str {
        "Catppuccin Mocha - dark pastel theme"
    }

    fn primary(&self) -> Color {
        Color::Rgb(166, 227, 161) // Green
    }

    fn primary_bright(&self) -> Color {
        Color::Rgb(180, 240, 175)
    }

    fn primary_dark(&self) -> Color {
        Color::Rgb(150, 215, 145)
    }

    fn white(&self) -> Color {
        Color::Rgb(205, 214, 244) // Text
    }

    fn light_gray(&self) -> Color {
        Color::Rgb(186, 194, 222) // Subtext1
    }

    fn gray(&self) -> Color {
        Color::Rgb(166, 173, 200) // Subtext0
    }

    fn dark_gray(&self) -> Color {
        Color::Rgb(147, 153, 178) // Overlay2
    }

    fn darker_gray(&self) -> Color {
        Color::Rgb(108, 112, 134) // Overlay1
    }

    fn black(&self) -> Color {
        Color::Rgb(30, 30, 46) // Base
    }

    fn success(&self) -> Color {
        Color::Rgb(166, 227, 161) // Green
    }

    fn error(&self) -> Color {
        Color::Rgb(243, 139, 168) // Red
    }

    fn warning(&self) -> Color {
        Color::Rgb(249, 226, 175) // Yellow
    }

    fn info(&self) -> Color {
        Color::Rgb(137, 180, 250) // Blue
    }

    fn red(&self) -> Color {
        Color::Rgb(243, 139, 168)
    }

    fn yellow(&self) -> Color {
        Color::Rgb(249, 226, 175)
    }

    fn blue(&self) -> Color {
        Color::Rgb(137, 180, 250)
    }

    fn green(&self) -> Color {
        Color::Rgb(166, 227, 161)
    }

    fn cyan(&self) -> Color {
        Color::Rgb(148, 226, 213) // Teal
    }

    fn magenta(&self) -> Color {
        Color::Rgb(245, 194, 231) // Pink
    }

    fn background(&self) -> Color {
        Color::Rgb(30, 30, 46) // Base
    }

    fn foreground(&self) -> Color {
        Color::Rgb(205, 214, 244) // Text
    }

    fn border(&self) -> Color {
        Color::Rgb(108, 112, 134) // Overlay1
    }

    fn border_focused(&self) -> Color {
        Color::Rgb(166, 227, 161) // Green
    }

    fn title(&self) -> Color {
        Color::Rgb(205, 214, 244) // Text
    }

    fn accent(&self) -> Color {
        Color::Rgb(166, 227, 161) // Green
    }
}

/// Catppuccin Macchiato (dark)
pub struct CatppuccinMacchiato;

impl Theme for CatppuccinMacchiato {
    fn name(&self) -> &str {
        "Catppuccin Macchiato"
    }

    fn description(&self) -> &str {
        "Catppuccin Macchiato - warm dark pastel theme"
    }

    fn primary(&self) -> Color {
        Color::Rgb(166, 218, 149) // Green
    }

    fn primary_bright(&self) -> Color {
        Color::Rgb(180, 230, 165)
    }

    fn primary_dark(&self) -> Color {
        Color::Rgb(150, 205, 135)
    }

    fn white(&self) -> Color {
        Color::Rgb(202, 211, 245) // Text
    }

    fn light_gray(&self) -> Color {
        Color::Rgb(184, 192, 224) // Subtext1
    }

    fn gray(&self) -> Color {
        Color::Rgb(165, 173, 203) // Subtext0
    }

    fn dark_gray(&self) -> Color {
        Color::Rgb(128, 135, 162) // Overlay2
    }

    fn darker_gray(&self) -> Color {
        Color::Rgb(110, 115, 141) // Overlay1
    }

    fn black(&self) -> Color {
        Color::Rgb(36, 39, 58) // Base
    }

    fn success(&self) -> Color {
        Color::Rgb(166, 218, 149)
    }

    fn error(&self) -> Color {
        Color::Rgb(237, 135, 150) // Red
    }

    fn warning(&self) -> Color {
        Color::Rgb(238, 212, 159) // Yellow
    }

    fn info(&self) -> Color {
        Color::Rgb(138, 173, 244) // Blue
    }

    fn red(&self) -> Color {
        Color::Rgb(237, 135, 150)
    }

    fn yellow(&self) -> Color {
        Color::Rgb(238, 212, 159)
    }

    fn blue(&self) -> Color {
        Color::Rgb(138, 173, 244)
    }

    fn green(&self) -> Color {
        Color::Rgb(166, 218, 149)
    }

    fn cyan(&self) -> Color {
        Color::Rgb(139, 213, 202) // Teal
    }

    fn magenta(&self) -> Color {
        Color::Rgb(245, 169, 227) // Pink
    }

    fn background(&self) -> Color {
        Color::Rgb(36, 39, 58)
    }

    fn foreground(&self) -> Color {
        Color::Rgb(202, 211, 245)
    }

    fn border(&self) -> Color {
        Color::Rgb(110, 115, 141)
    }

    fn border_focused(&self) -> Color {
        Color::Rgb(166, 218, 149)
    }

    fn title(&self) -> Color {
        Color::Rgb(202, 211, 245)
    }

    fn accent(&self) -> Color {
        Color::Rgb(166, 218, 149)
    }
}

/// Catppuccin Frappe (dark)
pub struct CatppuccinFrappe;

impl Theme for CatppuccinFrappe {
    fn name(&self) -> &str {
        "Catppuccin Frappe"
    }

    fn description(&self) -> &str {
        "Catppuccin Frappe - cool dark pastel theme"
    }

    fn primary(&self) -> Color {
        Color::Rgb(166, 209, 137) // Green
    }

    fn primary_bright(&self) -> Color {
        Color::Rgb(180, 220, 150)
    }

    fn primary_dark(&self) -> Color {
        Color::Rgb(150, 195, 125)
    }

    fn white(&self) -> Color {
        Color::Rgb(198, 208, 245) // Text
    }

    fn light_gray(&self) -> Color {
        Color::Rgb(181, 191, 226) // Subtext1
    }

    fn gray(&self) -> Color {
        Color::Rgb(165, 173, 206) // Subtext0
    }

    fn dark_gray(&self) -> Color {
        Color::Rgb(131, 139, 167) // Overlay2
    }

    fn darker_gray(&self) -> Color {
        Color::Rgb(115, 121, 148) // Overlay1
    }

    fn black(&self) -> Color {
        Color::Rgb(48, 52, 70) // Base
    }

    fn success(&self) -> Color {
        Color::Rgb(166, 209, 137)
    }

    fn error(&self) -> Color {
        Color::Rgb(231, 130, 132) // Red
    }

    fn warning(&self) -> Color {
        Color::Rgb(229, 200, 144) // Yellow
    }

    fn info(&self) -> Color {
        Color::Rgb(140, 170, 238) // Blue
    }

    fn red(&self) -> Color {
        Color::Rgb(231, 130, 132)
    }

    fn yellow(&self) -> Color {
        Color::Rgb(229, 200, 144)
    }

    fn blue(&self) -> Color {
        Color::Rgb(140, 170, 238)
    }

    fn green(&self) -> Color {
        Color::Rgb(166, 209, 137)
    }

    fn cyan(&self) -> Color {
        Color::Rgb(129, 200, 190) // Teal
    }

    fn magenta(&self) -> Color {
        Color::Rgb(244, 184, 228) // Pink
    }

    fn background(&self) -> Color {
        Color::Rgb(48, 52, 70)
    }

    fn foreground(&self) -> Color {
        Color::Rgb(198, 208, 245)
    }

    fn border(&self) -> Color {
        Color::Rgb(115, 121, 148)
    }

    fn border_focused(&self) -> Color {
        Color::Rgb(166, 209, 137)
    }

    fn title(&self) -> Color {
        Color::Rgb(198, 208, 245)
    }

    fn accent(&self) -> Color {
        Color::Rgb(166, 209, 137)
    }
}

/// Catppuccin Latte (light)
pub struct CatppuccinLatte;

impl Theme for CatppuccinLatte {
    fn name(&self) -> &str {
        "Catppuccin Latte"
    }

    fn description(&self) -> &str {
        "Catppuccin Latte - light pastel theme"
    }

    fn primary(&self) -> Color {
        Color::Rgb(64, 160, 43) // Green
    }

    fn primary_bright(&self) -> Color {
        Color::Rgb(80, 180, 55)
    }

    fn primary_dark(&self) -> Color {
        Color::Rgb(50, 140, 35)
    }

    fn white(&self) -> Color {
        Color::Rgb(76, 79, 105) // Text
    }

    fn light_gray(&self) -> Color {
        Color::Rgb(92, 95, 119) // Subtext1
    }

    fn gray(&self) -> Color {
        Color::Rgb(108, 111, 133) // Subtext0
    }

    fn dark_gray(&self) -> Color {
        Color::Rgb(156, 160, 176) // Overlay2
    }

    fn darker_gray(&self) -> Color {
        Color::Rgb(172, 176, 190) // Overlay1
    }

    fn black(&self) -> Color {
        Color::Rgb(239, 241, 245) // Base
    }

    fn success(&self) -> Color {
        Color::Rgb(64, 160, 43)
    }

    fn error(&self) -> Color {
        Color::Rgb(210, 15, 57) // Red
    }

    fn warning(&self) -> Color {
        Color::Rgb(223, 142, 29) // Yellow
    }

    fn info(&self) -> Color {
        Color::Rgb(30, 102, 245) // Blue
    }

    fn red(&self) -> Color {
        Color::Rgb(210, 15, 57)
    }

    fn yellow(&self) -> Color {
        Color::Rgb(223, 142, 29)
    }

    fn blue(&self) -> Color {
        Color::Rgb(30, 102, 245)
    }

    fn green(&self) -> Color {
        Color::Rgb(64, 160, 43)
    }

    fn cyan(&self) -> Color {
        Color::Rgb(23, 146, 153) // Teal
    }

    fn magenta(&self) -> Color {
        Color::Rgb(234, 118, 203) // Pink
    }

    fn background(&self) -> Color {
        Color::Rgb(239, 241, 245)
    }

    fn foreground(&self) -> Color {
        Color::Rgb(76, 79, 105)
    }

    fn border(&self) -> Color {
        Color::Rgb(172, 176, 190)
    }

    fn border_focused(&self) -> Color {
        Color::Rgb(64, 160, 43)
    }

    fn title(&self) -> Color {
        Color::Rgb(76, 79, 105)
    }

    fn accent(&self) -> Color {
        Color::Rgb(64, 160, 43)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ===== CatppuccinMocha Tests =====
    #[test]
    fn test_mocha_metadata() {
        let theme = CatppuccinMocha;
        assert_eq!(theme.name(), "Catppuccin Mocha");
        assert!(theme.description().contains("Mocha"));
    }

    #[test]
    fn test_mocha_primary_colors() {
        let theme = CatppuccinMocha;
        assert_eq!(theme.primary(), Color::Rgb(166, 227, 161));
        assert_eq!(theme.primary_bright(), Color::Rgb(180, 240, 175));
        assert_eq!(theme.primary_dark(), Color::Rgb(150, 215, 145));
    }

    #[test]
    fn test_mocha_grayscale() {
        let theme = CatppuccinMocha;
        assert_eq!(theme.white(), Color::Rgb(205, 214, 244));
        assert_eq!(theme.light_gray(), Color::Rgb(186, 194, 222));
        assert_eq!(theme.gray(), Color::Rgb(166, 173, 200));
        assert_eq!(theme.dark_gray(), Color::Rgb(147, 153, 178));
        assert_eq!(theme.darker_gray(), Color::Rgb(108, 112, 134));
        assert_eq!(theme.black(), Color::Rgb(30, 30, 46));
    }

    #[test]
    fn test_mocha_semantic_colors() {
        let theme = CatppuccinMocha;
        assert_eq!(theme.success(), Color::Rgb(166, 227, 161));
        assert_eq!(theme.error(), Color::Rgb(243, 139, 168));
        assert_eq!(theme.warning(), Color::Rgb(249, 226, 175));
        assert_eq!(theme.info(), Color::Rgb(137, 180, 250));
    }

    #[test]
    fn test_mocha_ui_colors() {
        let theme = CatppuccinMocha;
        assert_eq!(theme.background(), Color::Rgb(30, 30, 46));
        assert_eq!(theme.foreground(), Color::Rgb(205, 214, 244));
        assert_eq!(theme.border(), Color::Rgb(108, 112, 134));
        assert_eq!(theme.border_focused(), Color::Rgb(166, 227, 161));
        assert_eq!(theme.title(), Color::Rgb(205, 214, 244));
        assert_eq!(theme.accent(), Color::Rgb(166, 227, 161));
    }

    // ===== CatppuccinMacchiato Tests =====
    #[test]
    fn test_macchiato_metadata() {
        let theme = CatppuccinMacchiato;
        assert_eq!(theme.name(), "Catppuccin Macchiato");
        assert!(theme.description().contains("Macchiato"));
    }

    #[test]
    fn test_macchiato_primary_colors() {
        let theme = CatppuccinMacchiato;
        assert_eq!(theme.primary(), Color::Rgb(166, 218, 149));
        assert_eq!(theme.primary_bright(), Color::Rgb(180, 230, 165));
        assert_eq!(theme.primary_dark(), Color::Rgb(150, 205, 135));
    }

    #[test]
    fn test_macchiato_grayscale() {
        let theme = CatppuccinMacchiato;
        assert_eq!(theme.white(), Color::Rgb(202, 211, 245));
        assert_eq!(theme.light_gray(), Color::Rgb(184, 192, 224));
        assert_eq!(theme.gray(), Color::Rgb(165, 173, 203));
        assert_eq!(theme.dark_gray(), Color::Rgb(128, 135, 162));
        assert_eq!(theme.darker_gray(), Color::Rgb(110, 115, 141));
        assert_eq!(theme.black(), Color::Rgb(36, 39, 58));
    }

    #[test]
    fn test_macchiato_ui_colors() {
        let theme = CatppuccinMacchiato;
        assert_eq!(theme.background(), Color::Rgb(36, 39, 58));
        assert_eq!(theme.foreground(), Color::Rgb(202, 211, 245));
        assert_eq!(theme.border(), Color::Rgb(110, 115, 141));
        assert_eq!(theme.border_focused(), Color::Rgb(166, 218, 149));
    }

    // ===== CatppuccinFrappe Tests =====
    #[test]
    fn test_frappe_metadata() {
        let theme = CatppuccinFrappe;
        assert_eq!(theme.name(), "Catppuccin Frappe");
        assert!(theme.description().contains("Frappe"));
    }

    #[test]
    fn test_frappe_primary_colors() {
        let theme = CatppuccinFrappe;
        assert_eq!(theme.primary(), Color::Rgb(166, 209, 137));
        assert_eq!(theme.primary_bright(), Color::Rgb(180, 220, 150));
        assert_eq!(theme.primary_dark(), Color::Rgb(150, 195, 125));
    }

    #[test]
    fn test_frappe_grayscale() {
        let theme = CatppuccinFrappe;
        assert_eq!(theme.white(), Color::Rgb(198, 208, 245));
        assert_eq!(theme.light_gray(), Color::Rgb(181, 191, 226));
        assert_eq!(theme.gray(), Color::Rgb(165, 173, 206));
        assert_eq!(theme.dark_gray(), Color::Rgb(131, 139, 167));
        assert_eq!(theme.darker_gray(), Color::Rgb(115, 121, 148));
        assert_eq!(theme.black(), Color::Rgb(48, 52, 70));
    }

    #[test]
    fn test_frappe_ui_colors() {
        let theme = CatppuccinFrappe;
        assert_eq!(theme.background(), Color::Rgb(48, 52, 70));
        assert_eq!(theme.foreground(), Color::Rgb(198, 208, 245));
        assert_eq!(theme.border(), Color::Rgb(115, 121, 148));
        assert_eq!(theme.border_focused(), Color::Rgb(166, 209, 137));
    }

    // ===== CatppuccinLatte Tests =====
    #[test]
    fn test_latte_metadata() {
        let theme = CatppuccinLatte;
        assert_eq!(theme.name(), "Catppuccin Latte");
        assert!(theme.description().contains("Latte"));
    }

    #[test]
    fn test_latte_primary_colors() {
        let theme = CatppuccinLatte;
        assert_eq!(theme.primary(), Color::Rgb(64, 160, 43));
        assert_eq!(theme.primary_bright(), Color::Rgb(80, 180, 55));
        assert_eq!(theme.primary_dark(), Color::Rgb(50, 140, 35));
    }

    #[test]
    fn test_latte_grayscale() {
        let theme = CatppuccinLatte;
        assert_eq!(theme.white(), Color::Rgb(76, 79, 105));
        assert_eq!(theme.light_gray(), Color::Rgb(92, 95, 119));
        assert_eq!(theme.gray(), Color::Rgb(108, 111, 133));
        assert_eq!(theme.dark_gray(), Color::Rgb(156, 160, 176));
        assert_eq!(theme.darker_gray(), Color::Rgb(172, 176, 190));
        assert_eq!(theme.black(), Color::Rgb(239, 241, 245));
    }

    #[test]
    fn test_latte_semantic_colors() {
        let theme = CatppuccinLatte;
        assert_eq!(theme.success(), Color::Rgb(64, 160, 43));
        assert_eq!(theme.error(), Color::Rgb(210, 15, 57));
        assert_eq!(theme.warning(), Color::Rgb(223, 142, 29));
        assert_eq!(theme.info(), Color::Rgb(30, 102, 245));
    }

    #[test]
    fn test_latte_ui_colors() {
        let theme = CatppuccinLatte;
        assert_eq!(theme.background(), Color::Rgb(239, 241, 245));
        assert_eq!(theme.foreground(), Color::Rgb(76, 79, 105));
        assert_eq!(theme.border(), Color::Rgb(172, 176, 190));
        assert_eq!(theme.border_focused(), Color::Rgb(64, 160, 43));
    }

    #[test]
    fn test_latte_background_is_bright() {
        let theme = CatppuccinLatte;
        match theme.background() {
            Color::Rgb(r, g, b) => assert!(r > 200 && g > 200 && b > 200),
            _ => panic!("Expected RGB color"),
        }
    }

    // ===== Cross-variant tests =====
    #[test]
    fn test_all_variants_have_unique_names() {
        let mocha = CatppuccinMocha;
        let macchiato = CatppuccinMacchiato;
        let frappe = CatppuccinFrappe;
        let latte = CatppuccinLatte;

        assert_ne!(mocha.name(), macchiato.name());
        assert_ne!(mocha.name(), frappe.name());
        assert_ne!(mocha.name(), latte.name());
        assert_ne!(macchiato.name(), frappe.name());
    }

    #[test]
    fn test_dark_variants_have_dark_backgrounds() {
        let mocha = CatppuccinMocha;
        let macchiato = CatppuccinMacchiato;
        let frappe = CatppuccinFrappe;

        for theme in [&mocha as &dyn Theme, &macchiato, &frappe] {
            match theme.background() {
                Color::Rgb(r, g, b) => assert!(r < 100 && g < 100 && b < 100),
                _ => panic!("Expected RGB color"),
            }
        }
    }

    // ===== Mocha rainbow colors =====
    #[test]
    fn test_mocha_rainbow_colors() {
        let theme = CatppuccinMocha;
        assert_eq!(theme.red(), Color::Rgb(243, 139, 168));
        assert_eq!(theme.yellow(), Color::Rgb(249, 226, 175));
        assert_eq!(theme.blue(), Color::Rgb(137, 180, 250));
        assert_eq!(theme.green(), Color::Rgb(166, 227, 161));
        assert_eq!(theme.cyan(), Color::Rgb(148, 226, 213));
        assert_eq!(theme.magenta(), Color::Rgb(245, 194, 231));
    }

    // ===== Macchiato comprehensive coverage =====
    #[test]
    fn test_macchiato_semantic_colors() {
        let theme = CatppuccinMacchiato;
        assert_eq!(theme.success(), Color::Rgb(166, 218, 149));
        assert_eq!(theme.error(), Color::Rgb(237, 135, 150));
        assert_eq!(theme.warning(), Color::Rgb(238, 212, 159));
        assert_eq!(theme.info(), Color::Rgb(138, 173, 244));
    }

    #[test]
    fn test_macchiato_rainbow_colors() {
        let theme = CatppuccinMacchiato;
        assert_eq!(theme.red(), Color::Rgb(237, 135, 150));
        assert_eq!(theme.yellow(), Color::Rgb(238, 212, 159));
        assert_eq!(theme.blue(), Color::Rgb(138, 173, 244));
        assert_eq!(theme.green(), Color::Rgb(166, 218, 149));
        assert_eq!(theme.cyan(), Color::Rgb(139, 213, 202));
        assert_eq!(theme.magenta(), Color::Rgb(245, 169, 227));
    }

    #[test]
    fn test_macchiato_title_and_accent() {
        let theme = CatppuccinMacchiato;
        assert_eq!(theme.title(), Color::Rgb(202, 211, 245));
        assert_eq!(theme.accent(), Color::Rgb(166, 218, 149));
    }

    // ===== Frappe comprehensive coverage =====
    #[test]
    fn test_frappe_semantic_colors() {
        let theme = CatppuccinFrappe;
        assert_eq!(theme.success(), Color::Rgb(166, 209, 137));
        assert_eq!(theme.error(), Color::Rgb(231, 130, 132));
        assert_eq!(theme.warning(), Color::Rgb(229, 200, 144));
        assert_eq!(theme.info(), Color::Rgb(140, 170, 238));
    }

    #[test]
    fn test_frappe_rainbow_colors() {
        let theme = CatppuccinFrappe;
        assert_eq!(theme.red(), Color::Rgb(231, 130, 132));
        assert_eq!(theme.yellow(), Color::Rgb(229, 200, 144));
        assert_eq!(theme.blue(), Color::Rgb(140, 170, 238));
        assert_eq!(theme.green(), Color::Rgb(166, 209, 137));
        assert_eq!(theme.cyan(), Color::Rgb(129, 200, 190));
        assert_eq!(theme.magenta(), Color::Rgb(244, 184, 228));
    }

    #[test]
    fn test_frappe_title_and_accent() {
        let theme = CatppuccinFrappe;
        assert_eq!(theme.title(), Color::Rgb(198, 208, 245));
        assert_eq!(theme.accent(), Color::Rgb(166, 209, 137));
    }

    // ===== Latte comprehensive coverage =====
    #[test]
    fn test_latte_rainbow_colors() {
        let theme = CatppuccinLatte;
        assert_eq!(theme.red(), Color::Rgb(210, 15, 57));
        assert_eq!(theme.yellow(), Color::Rgb(223, 142, 29));
        assert_eq!(theme.blue(), Color::Rgb(30, 102, 245));
        assert_eq!(theme.green(), Color::Rgb(64, 160, 43));
        assert_eq!(theme.cyan(), Color::Rgb(23, 146, 153));
        assert_eq!(theme.magenta(), Color::Rgb(234, 118, 203));
    }

    #[test]
    fn test_latte_title_and_accent() {
        let theme = CatppuccinLatte;
        assert_eq!(theme.title(), Color::Rgb(76, 79, 105));
        assert_eq!(theme.accent(), Color::Rgb(64, 160, 43));
    }

    // ===== Color consistency tests =====
    #[test]
    fn test_all_themes_primary_matches_accent() {
        let mocha = CatppuccinMocha;
        let macchiato = CatppuccinMacchiato;
        let frappe = CatppuccinFrappe;
        let latte = CatppuccinLatte;

        assert_eq!(mocha.primary(), mocha.accent());
        assert_eq!(macchiato.primary(), macchiato.accent());
        assert_eq!(frappe.primary(), frappe.accent());
        assert_eq!(latte.primary(), latte.accent());
    }

    #[test]
    fn test_all_themes_success_uses_green() {
        let mocha = CatppuccinMocha;
        let macchiato = CatppuccinMacchiato;
        let frappe = CatppuccinFrappe;
        let latte = CatppuccinLatte;

        assert_eq!(mocha.success(), mocha.green());
        assert_eq!(macchiato.success(), macchiato.green());
        assert_eq!(frappe.success(), frappe.green());
        assert_eq!(latte.success(), latte.green());
    }

    #[test]
    fn test_all_themes_error_uses_red() {
        let mocha = CatppuccinMocha;
        let macchiato = CatppuccinMacchiato;
        let frappe = CatppuccinFrappe;
        let latte = CatppuccinLatte;

        assert_eq!(mocha.error(), mocha.red());
        assert_eq!(macchiato.error(), macchiato.red());
        assert_eq!(frappe.error(), frappe.red());
        assert_eq!(latte.error(), latte.red());
    }

    #[test]
    fn test_all_themes_warning_uses_yellow() {
        let mocha = CatppuccinMocha;
        let macchiato = CatppuccinMacchiato;
        let frappe = CatppuccinFrappe;
        let latte = CatppuccinLatte;

        assert_eq!(mocha.warning(), mocha.yellow());
        assert_eq!(macchiato.warning(), macchiato.yellow());
        assert_eq!(frappe.warning(), frappe.yellow());
        assert_eq!(latte.warning(), latte.yellow());
    }

    #[test]
    fn test_all_themes_info_uses_blue() {
        let mocha = CatppuccinMocha;
        let macchiato = CatppuccinMacchiato;
        let frappe = CatppuccinFrappe;
        let latte = CatppuccinLatte;

        assert_eq!(mocha.info(), mocha.blue());
        assert_eq!(macchiato.info(), macchiato.blue());
        assert_eq!(frappe.info(), frappe.blue());
        assert_eq!(latte.info(), latte.blue());
    }
}
