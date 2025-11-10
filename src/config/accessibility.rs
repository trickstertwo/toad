//! Accessibility configuration
//!
//! Provides settings for users who need accessibility features such as
//! high contrast mode, reduced motion, large text, and keyboard-only navigation.
//!
//! # Examples
//!
//! ```
//! use toad::config::AccessibilityConfig;
//!
//! let config = AccessibilityConfig::default();
//! assert!(!config.high_contrast_mode);
//! ```

use serde::{Deserialize, Serialize};

/// Accessibility configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AccessibilityConfig {
    /// Enable high contrast colors for better visibility
    pub high_contrast_mode: bool,

    /// Disable animations and transitions
    pub reduced_motion: bool,

    /// Increase text size throughout the UI
    pub large_text_mode: bool,

    /// Show keyboard-only navigation indicators
    pub keyboard_only_mode: bool,

    /// Enable screen reader support (descriptive labels)
    pub screen_reader_support: bool,

    /// Slow down automatic transitions (ms delay)
    pub slow_transitions: bool,

    /// Flash on visual bell instead of beep
    pub visual_bell: bool,

    /// Enable focus indicators for all interactive elements
    pub focus_indicators: bool,
}

impl AccessibilityConfig {
    /// Create a new accessibility config with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Create config optimized for low vision users
    pub fn high_visibility() -> Self {
        Self {
            high_contrast_mode: true,
            reduced_motion: false,
            large_text_mode: true,
            keyboard_only_mode: false,
            screen_reader_support: false,
            slow_transitions: false,
            visual_bell: true,
            focus_indicators: true,
        }
    }

    /// Create config optimized for motion sensitivity
    pub fn reduced_motion_preset() -> Self {
        Self {
            high_contrast_mode: false,
            reduced_motion: true,
            large_text_mode: false,
            keyboard_only_mode: false,
            screen_reader_support: false,
            slow_transitions: true,
            visual_bell: true,
            focus_indicators: false,
        }
    }

    /// Create config for keyboard-only users
    pub fn keyboard_only() -> Self {
        Self {
            high_contrast_mode: false,
            reduced_motion: false,
            large_text_mode: false,
            keyboard_only_mode: true,
            screen_reader_support: false,
            slow_transitions: false,
            visual_bell: false,
            focus_indicators: true,
        }
    }

    /// Create config for screen reader users
    pub fn screen_reader() -> Self {
        Self {
            high_contrast_mode: false,
            reduced_motion: true,
            large_text_mode: false,
            keyboard_only_mode: true,
            screen_reader_support: true,
            slow_transitions: true,
            visual_bell: false,
            focus_indicators: true,
        }
    }

    /// Check if any accessibility features are enabled
    pub fn has_any_enabled(&self) -> bool {
        self.high_contrast_mode
            || self.reduced_motion
            || self.large_text_mode
            || self.keyboard_only_mode
            || self.screen_reader_support
            || self.slow_transitions
            || self.visual_bell
            || self.focus_indicators
    }

    /// Get text size multiplier based on large text setting
    pub fn text_size_multiplier(&self) -> f32 {
        if self.large_text_mode { 1.5 } else { 1.0 }
    }

    /// Get transition duration in milliseconds
    pub fn transition_duration(&self) -> u64 {
        if self.slow_transitions {
            500 // Slower transitions
        } else if self.reduced_motion {
            0 // No transitions
        } else {
            200 // Normal speed
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accessibility_config_default() {
        let config = AccessibilityConfig::default();
        assert!(!config.has_any_enabled());
    }

    #[test]
    fn test_accessibility_config_high_visibility() {
        let config = AccessibilityConfig::high_visibility();
        assert!(config.high_contrast_mode);
        assert!(config.large_text_mode);
        assert!(config.has_any_enabled());
    }

    #[test]
    fn test_accessibility_config_reduced_motion() {
        let config = AccessibilityConfig::reduced_motion_preset();
        assert!(config.reduced_motion);
        assert!(config.slow_transitions);
        assert_eq!(config.transition_duration(), 500);
    }

    #[test]
    fn test_accessibility_config_keyboard_only() {
        let config = AccessibilityConfig::keyboard_only();
        assert!(config.keyboard_only_mode);
        assert!(config.focus_indicators);
    }

    #[test]
    fn test_text_size_multiplier() {
        let mut config = AccessibilityConfig::default();
        assert_eq!(config.text_size_multiplier(), 1.0);

        config.large_text_mode = true;
        assert_eq!(config.text_size_multiplier(), 1.5);
    }

    #[test]
    fn test_transition_duration() {
        let mut config = AccessibilityConfig::default();
        assert_eq!(config.transition_duration(), 200);

        config.reduced_motion = true;
        assert_eq!(config.transition_duration(), 0);

        config.reduced_motion = false;
        config.slow_transitions = true;
        assert_eq!(config.transition_duration(), 500);
    }
}
