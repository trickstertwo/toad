//! ASCII logo for Toad
//!
//! Contains the ASCII art branding for the application

/// The main TOAD ASCII logo
pub const TOAD_LOGO: &str = r#"
        ████████╗ ██████╗   █████╗  ██████╗
        ╚══██╔══╝██╔═══██╗██╔══██╗ ██╔══██╗
           ██║   ██║   ██║███████║ ██║  ██║
           ██║   ██║   ██║██╔══██║ ██║  ██║
           ██║   ╚██████╔╝██║  ██║ ██████╔╝
           ╚═╝    ╚═════╝ ╚═╝  ╚═╝ ╚═════╝
"#;

/// Cute ASCII toad character (for splash screen)
pub const TOAD_CHARACTER: &str = r#"
              ___     ___
             (o o)   (o o)
            (  >  •  <  )
             \   ___   /
              '-_____-'
                |   |
               _|   |_
              (       )
"#;

/// Compact TOAD logo for header
pub const TOAD_COMPACT: &str = r#"
    ████████╗ ██████╗   █████╗  ██████╗
    ╚══██╔══╝██╔═══██╗██╔══██╗ ██╔══██╗
       ██║   ██║   ██║███████║ ██║  ██║
       ██║   ██║   ██║██╔══██║ ██║  ██║
       ██║   ╚██████╔╝██║  ██║ ██████╔╝
       ╚═╝    ╚═════╝ ╚═╝  ╚═╝ ╚═════╝
"#;

/// Minimalist TOAD text logo
pub const TOAD_MINIMAL: &str = "🐸 TOAD";

/// Version info
pub fn version_string() -> String {
    format!("v{}", env!("CARGO_PKG_VERSION"))
}

/// Tagline
pub const TAGLINE: &str = "AI-Powered Coding Terminal";
pub const SUBTITLE: &str = "Semi-Autonomous Coding Agents";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toad_logo_not_empty() {
        assert!(!TOAD_LOGO.is_empty(), "TOAD_LOGO should not be empty");
        assert!(TOAD_LOGO.len() > 100, "TOAD_LOGO should be substantial");
    }

    #[test]
    fn test_toad_logo_contains_box_chars() {
        // Logo uses box drawing characters
        assert!(TOAD_LOGO.contains('█'), "TOAD_LOGO should contain block chars");
        assert!(TOAD_LOGO.contains('╗') || TOAD_LOGO.contains('╔'), "TOAD_LOGO should contain box drawing chars");
    }

    #[test]
    fn test_toad_logo_multiline() {
        let lines: Vec<&str> = TOAD_LOGO.lines().collect();
        assert!(lines.len() >= 6, "TOAD_LOGO should have at least 6 lines");
    }

    #[test]
    fn test_toad_compact_not_empty() {
        assert!(!TOAD_COMPACT.is_empty(), "TOAD_COMPACT should not be empty");
    }

    #[test]
    fn test_toad_compact_contains_box_chars() {
        assert!(TOAD_COMPACT.contains('█'), "TOAD_COMPACT should contain block chars");
    }

    #[test]
    fn test_toad_compact_multiline() {
        let lines: Vec<&str> = TOAD_COMPACT.lines().collect();
        assert!(lines.len() >= 6, "TOAD_COMPACT should have at least 6 lines");
    }

    #[test]
    fn test_toad_compact_shorter_than_full() {
        // Compact should be shorter than full logo
        assert!(
            TOAD_COMPACT.len() < TOAD_LOGO.len(),
            "TOAD_COMPACT should be shorter than TOAD_LOGO"
        );
    }

    #[test]
    fn test_toad_character_not_empty() {
        assert!(!TOAD_CHARACTER.is_empty(), "TOAD_CHARACTER should not be empty");
    }

    #[test]
    fn test_toad_character_has_eyes() {
        assert!(TOAD_CHARACTER.contains("o o"), "TOAD_CHARACTER should have eyes");
    }

    #[test]
    fn test_toad_character_multiline() {
        let lines: Vec<&str> = TOAD_CHARACTER.lines().collect();
        assert!(lines.len() >= 8, "TOAD_CHARACTER should have at least 8 lines");
    }

    #[test]
    fn test_toad_minimal_contains_emoji() {
        assert!(TOAD_MINIMAL.contains('🐸'), "TOAD_MINIMAL should contain frog emoji");
        assert!(TOAD_MINIMAL.contains("TOAD"), "TOAD_MINIMAL should contain 'TOAD' text");
    }

    #[test]
    fn test_toad_minimal_short() {
        assert!(TOAD_MINIMAL.len() < 20, "TOAD_MINIMAL should be very short");
    }

    #[test]
    fn test_version_string_format() {
        let version = version_string();
        assert!(version.starts_with('v'), "Version should start with 'v'");
        assert!(version.len() > 1, "Version should have content after 'v'");
    }

    #[test]
    fn test_version_string_contains_dots() {
        let version = version_string();
        // Semantic versioning has at least one dot (e.g., v0.1 or v1.2.3)
        assert!(version.contains('.'), "Version should contain at least one dot");
    }

    #[test]
    fn test_tagline_not_empty() {
        assert!(!TAGLINE.is_empty(), "TAGLINE should not be empty");
        assert!(TAGLINE.contains("Terminal"), "TAGLINE should mention Terminal");
    }

    #[test]
    fn test_subtitle_not_empty() {
        assert!(!SUBTITLE.is_empty(), "SUBTITLE should not be empty");
        assert!(SUBTITLE.contains("Agent"), "SUBTITLE should mention Agents");
    }

    #[test]
    fn test_all_logos_valid_utf8() {
        // All logos should be valid UTF-8 (this should always pass in Rust)
        assert!(std::str::from_utf8(TOAD_LOGO.as_bytes()).is_ok());
        assert!(std::str::from_utf8(TOAD_COMPACT.as_bytes()).is_ok());
        assert!(std::str::from_utf8(TOAD_CHARACTER.as_bytes()).is_ok());
        assert!(std::str::from_utf8(TOAD_MINIMAL.as_bytes()).is_ok());
    }

    #[test]
    fn test_logo_variants_all_different() {
        // Each logo variant should be unique
        assert_ne!(TOAD_LOGO, TOAD_COMPACT);
        assert_ne!(TOAD_LOGO, TOAD_CHARACTER);
        assert_ne!(TOAD_LOGO, TOAD_MINIMAL);
        assert_ne!(TOAD_COMPACT, TOAD_CHARACTER);
        assert_ne!(TOAD_COMPACT, TOAD_MINIMAL);
        assert_ne!(TOAD_CHARACTER, TOAD_MINIMAL);
    }
}
