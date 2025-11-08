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

    #[test]
    fn test_catppuccin_mocha() {
        let theme = CatppuccinMocha;
        assert_eq!(theme.name(), "Catppuccin Mocha");
        assert!(theme.description().contains("Mocha"));
    }

    #[test]
    fn test_catppuccin_macchiato() {
        let theme = CatppuccinMacchiato;
        assert_eq!(theme.name(), "Catppuccin Macchiato");
        assert!(theme.description().contains("Macchiato"));
    }

    #[test]
    fn test_catppuccin_frappe() {
        let theme = CatppuccinFrappe;
        assert_eq!(theme.name(), "Catppuccin Frappe");
        assert!(theme.description().contains("Frappe"));
    }

    #[test]
    fn test_catppuccin_latte() {
        let theme = CatppuccinLatte;
        assert_eq!(theme.name(), "Catppuccin Latte");
        assert!(theme.description().contains("Latte"));
        // Light theme should have bright background
        match theme.background() {
            Color::Rgb(r, g, b) => assert!(r > 200 && g > 200 && b > 200),
            _ => panic!("Expected RGB color"),
        }
    }
}
