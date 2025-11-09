//! Toast notification system
//!
//! Non-blocking notification messages that appear temporarily and auto-dismiss.
//! Toasts can be stacked and support different severity levels.
//!
//! # Examples
//!
//! ```
//! use toad::widgets::{Toast, ToastLevel};
//!
//! let toast = Toast::info("Operation completed successfully");
//! assert_eq!(toast.message(), "Operation completed successfully");
//! ```

use crate::ui::theme::ToadTheme;
use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};
use std::time::{Duration, Instant};

/// Toast notification severity level
///
/// # Examples
///
/// ```
/// use toad::widgets::ToastLevel;
///
/// let level = ToastLevel::Success;
/// assert_eq!(level.icon(), "✓");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToastLevel {
    /// Informational message
    Info,
    /// Success message
    Success,
    /// Warning message
    Warning,
    /// Error message
    Error,
}

impl ToastLevel {
    /// Get icon for this toast level
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::ToastLevel;
    ///
    /// assert_eq!(ToastLevel::Info.icon(), "ℹ");
    /// assert_eq!(ToastLevel::Success.icon(), "✓");
    /// assert_eq!(ToastLevel::Warning.icon(), "⚠");
    /// assert_eq!(ToastLevel::Error.icon(), "✗");
    /// ```
    pub fn icon(&self) -> &'static str {
        match self {
            ToastLevel::Info => "ℹ",
            ToastLevel::Success => "✓",
            ToastLevel::Warning => "⚠",
            ToastLevel::Error => "✗",
        }
    }

    /// Get border color for this toast level
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::ToastLevel;
    /// use toad::theme::ToadTheme;
    ///
    /// let color = ToastLevel::Success.border_color();
    /// assert_eq!(color, ToadTheme::TOAD_GREEN);
    /// ```
    pub fn border_color(&self) -> ratatui::style::Color {
        match self {
            ToastLevel::Info => ToadTheme::BLUE,
            ToastLevel::Success => ToadTheme::TOAD_GREEN,
            ToastLevel::Warning => ToadTheme::YELLOW,
            ToastLevel::Error => ToadTheme::RED,
        }
    }
}

/// Individual toast notification
///
/// Toasts are temporary, non-blocking notifications that appear at the
/// top-right of the screen and automatically dismiss after a timeout.
///
/// # Examples
///
/// ```
/// use toad::widgets::Toast;
///
/// let toast = Toast::success("File saved successfully");
/// assert!(toast.is_visible());
/// ```
#[derive(Debug, Clone)]
pub struct Toast {
    level: ToastLevel,
    message: String,
    created_at: Instant,
    duration: Duration,
}

impl Toast {
    /// Create a new toast with custom level and duration
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{Toast, ToastLevel};
    /// use std::time::Duration;
    ///
    /// let toast = Toast::new(
    ///     ToastLevel::Info,
    ///     "Custom message",
    ///     Duration::from_secs(5)
    /// );
    /// assert_eq!(toast.message(), "Custom message");
    /// ```
    pub fn new(level: ToastLevel, message: impl Into<String>, duration: Duration) -> Self {
        Self {
            level,
            message: message.into(),
            created_at: Instant::now(),
            duration,
        }
    }

    /// Create an info toast (3 second duration)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Toast;
    ///
    /// let toast = Toast::info("Information message");
    /// assert!(toast.is_visible());
    /// ```
    pub fn info(message: impl Into<String>) -> Self {
        Self::new(ToastLevel::Info, message, Duration::from_secs(3))
    }

    /// Create a success toast (3 second duration)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Toast;
    ///
    /// let toast = Toast::success("Operation successful");
    /// assert!(toast.is_visible());
    /// ```
    pub fn success(message: impl Into<String>) -> Self {
        Self::new(ToastLevel::Success, message, Duration::from_secs(3))
    }

    /// Create a warning toast (5 second duration)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Toast;
    ///
    /// let toast = Toast::warning("Warning: Check your input");
    /// assert!(toast.is_visible());
    /// ```
    pub fn warning(message: impl Into<String>) -> Self {
        Self::new(ToastLevel::Warning, message, Duration::from_secs(5))
    }

    /// Create an error toast (7 second duration)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Toast;
    ///
    /// let toast = Toast::error("Error: Operation failed");
    /// assert!(toast.is_visible());
    /// ```
    pub fn error(message: impl Into<String>) -> Self {
        Self::new(ToastLevel::Error, message, Duration::from_secs(7))
    }

    /// Get the toast message
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Toast;
    ///
    /// let toast = Toast::info("Test message");
    /// assert_eq!(toast.message(), "Test message");
    /// ```
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Get the toast level
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{Toast, ToastLevel};
    ///
    /// let toast = Toast::success("Done");
    /// assert_eq!(toast.level(), ToastLevel::Success);
    /// ```
    pub fn level(&self) -> ToastLevel {
        self.level
    }

    /// Check if toast is still visible
    ///
    /// Returns false if the toast has exceeded its duration.
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Toast;
    ///
    /// let toast = Toast::info("Message");
    /// assert!(toast.is_visible());
    /// ```
    pub fn is_visible(&self) -> bool {
        self.created_at.elapsed() < self.duration
    }

    /// Get remaining time before toast disappears
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Toast;
    /// use std::time::Duration;
    ///
    /// let toast = Toast::info("Message");
    /// let remaining = toast.remaining_time();
    /// assert!(remaining <= Duration::from_secs(3));
    /// ```
    pub fn remaining_time(&self) -> Duration {
        self.duration.saturating_sub(self.created_at.elapsed())
    }

    /// Render a single toast
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let icon = self.level.icon();
        let color = self.level.border_color();

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(color))
            .style(Style::default().bg(ToadTheme::BLACK));

        let inner = block.inner(area);
        frame.render_widget(block, area);

        let text = Line::from(vec![
            Span::styled(
                format!("{} ", icon),
                Style::default().fg(color).add_modifier(Modifier::BOLD),
            ),
            Span::styled(&self.message, Style::default().fg(ToadTheme::FOREGROUND)),
        ]);

        let paragraph = Paragraph::new(text).alignment(Alignment::Left);
        frame.render_widget(paragraph, inner);
    }
}

/// Toast notification manager
///
/// Manages a queue of toast notifications, handling auto-dismissal
/// and rendering multiple toasts in a stack.
///
/// # Examples
///
/// ```
/// use toad::widgets::ToastManager;
///
/// let mut manager = ToastManager::new();
/// manager.info("First message");
/// manager.success("Second message");
///
/// assert_eq!(manager.len(), 2);
/// ```
#[derive(Debug, Default)]
pub struct ToastManager {
    toasts: Vec<Toast>,
}

impl ToastManager {
    /// Create a new empty toast manager
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::ToastManager;
    ///
    /// let manager = ToastManager::new();
    /// assert_eq!(manager.len(), 0);
    /// ```
    pub fn new() -> Self {
        Self { toasts: Vec::new() }
    }

    /// Add an info toast
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::ToastManager;
    ///
    /// let mut manager = ToastManager::new();
    /// manager.info("Information");
    /// assert_eq!(manager.len(), 1);
    /// ```
    pub fn info(&mut self, message: impl Into<String>) {
        self.toasts.push(Toast::info(message));
    }

    /// Add a success toast
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::ToastManager;
    ///
    /// let mut manager = ToastManager::new();
    /// manager.success("Success!");
    /// assert_eq!(manager.len(), 1);
    /// ```
    pub fn success(&mut self, message: impl Into<String>) {
        self.toasts.push(Toast::success(message));
    }

    /// Add a warning toast
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::ToastManager;
    ///
    /// let mut manager = ToastManager::new();
    /// manager.warning("Warning!");
    /// assert_eq!(manager.len(), 1);
    /// ```
    pub fn warning(&mut self, message: impl Into<String>) {
        self.toasts.push(Toast::warning(message));
    }

    /// Add an error toast
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::ToastManager;
    ///
    /// let mut manager = ToastManager::new();
    /// manager.error("Error occurred");
    /// assert_eq!(manager.len(), 1);
    /// ```
    pub fn error(&mut self, message: impl Into<String>) {
        self.toasts.push(Toast::error(message));
    }

    /// Add a custom toast
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{ToastManager, Toast};
    ///
    /// let mut manager = ToastManager::new();
    /// manager.add(Toast::info("Custom"));
    /// assert_eq!(manager.len(), 1);
    /// ```
    pub fn add(&mut self, toast: Toast) {
        self.toasts.push(toast);
    }

    /// Remove expired toasts
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::ToastManager;
    ///
    /// let mut manager = ToastManager::new();
    /// manager.info("Message");
    /// manager.cleanup();
    /// // Toast still visible (just created)
    /// assert_eq!(manager.len(), 1);
    /// ```
    pub fn cleanup(&mut self) {
        self.toasts.retain(|toast| toast.is_visible());
    }

    /// Clear all toasts
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::ToastManager;
    ///
    /// let mut manager = ToastManager::new();
    /// manager.info("Test");
    /// manager.clear();
    /// assert_eq!(manager.len(), 0);
    /// ```
    pub fn clear(&mut self) {
        self.toasts.clear();
    }

    /// Get number of active toasts
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::ToastManager;
    ///
    /// let mut manager = ToastManager::new();
    /// assert_eq!(manager.len(), 0);
    ///
    /// manager.info("One");
    /// manager.info("Two");
    /// assert_eq!(manager.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        self.toasts.len()
    }

    /// Check if there are no active toasts
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::ToastManager;
    ///
    /// let mut manager = ToastManager::new();
    /// assert!(manager.is_empty());
    ///
    /// manager.info("Test");
    /// assert!(!manager.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.toasts.is_empty()
    }

    /// Render all visible toasts
    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        // Auto-cleanup expired toasts
        self.cleanup();

        if self.toasts.is_empty() {
            return;
        }

        // Render toasts from top-right, stacked vertically
        let toast_width: u16 = 40;
        let toast_height: u16 = 3;
        let spacing: u16 = 1;

        for (i, toast) in self.toasts.iter().enumerate() {
            let y_offset = (i as u16) * (toast_height + spacing);

            if y_offset + toast_height > area.height {
                break; // Don't render toasts that don't fit
            }

            let toast_area = Rect {
                x: area.width.saturating_sub(toast_width),
                y: area.y + y_offset,
                width: toast_width.min(area.width),
                height: toast_height,
            };

            toast.render(frame, toast_area);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toast_level_icons() {
        assert_eq!(ToastLevel::Info.icon(), "ℹ");
        assert_eq!(ToastLevel::Success.icon(), "✓");
        assert_eq!(ToastLevel::Warning.icon(), "⚠");
        assert_eq!(ToastLevel::Error.icon(), "✗");
    }

    #[test]
    fn test_toast_creation() {
        let toast = Toast::info("Test message");
        assert_eq!(toast.message(), "Test message");
        assert_eq!(toast.level(), ToastLevel::Info);
        assert!(toast.is_visible());
    }

    #[test]
    fn test_toast_convenience_methods() {
        let info = Toast::info("info");
        assert_eq!(info.level(), ToastLevel::Info);

        let success = Toast::success("success");
        assert_eq!(success.level(), ToastLevel::Success);

        let warning = Toast::warning("warning");
        assert_eq!(warning.level(), ToastLevel::Warning);

        let error = Toast::error("error");
        assert_eq!(error.level(), ToastLevel::Error);
    }

    #[test]
    fn test_toast_manager_operations() {
        let mut manager = ToastManager::new();
        assert_eq!(manager.len(), 0);
        assert!(manager.is_empty());

        manager.info("Message 1");
        assert_eq!(manager.len(), 1);
        assert!(!manager.is_empty());

        manager.success("Message 2");
        assert_eq!(manager.len(), 2);

        manager.clear();
        assert_eq!(manager.len(), 0);
        assert!(manager.is_empty());
    }

    #[test]
    fn test_toast_manager_add_methods() {
        let mut manager = ToastManager::new();

        manager.info("info");
        manager.success("success");
        manager.warning("warning");
        manager.error("error");

        assert_eq!(manager.len(), 4);
    }

    #[test]
    fn test_toast_remaining_time() {
        let toast = Toast::info("Test");
        let remaining = toast.remaining_time();
        assert!(remaining <= Duration::from_secs(3));
        assert!(remaining > Duration::from_secs(2));
    }

    // ============ COMPREHENSIVE EDGE CASE TESTS ============

    #[test]
    fn test_toast_level_border_colors_unique() {
        let info_color = ToastLevel::Info.border_color();
        let success_color = ToastLevel::Success.border_color();
        let warning_color = ToastLevel::Warning.border_color();
        let error_color = ToastLevel::Error.border_color();

        // All colors should be distinct
        assert_ne!(info_color, success_color);
        assert_ne!(success_color, warning_color);
        assert_ne!(warning_color, error_color);
        assert_ne!(info_color, error_color);
    }

    #[test]
    fn test_toast_with_very_long_message() {
        let long_message = "A".repeat(10000);
        let toast = Toast::info(long_message.clone());
        assert_eq!(toast.message(), &long_message);
    }

    #[test]
    fn test_toast_with_unicode_message() {
        let toast = Toast::info("🎉 成功しました！ Operation complete 🚀");
        assert!(toast.message().contains("🎉"));
        assert!(toast.message().contains("成功"));
    }

    #[test]
    fn test_toast_with_empty_message() {
        let toast = Toast::info("");
        assert_eq!(toast.message(), "");
        assert!(toast.is_visible());
    }

    #[test]
    fn test_toast_with_newlines() {
        let toast = Toast::info("Line 1\nLine 2\nLine 3");
        assert!(toast.message().contains("\n"));
    }

    #[test]
    fn test_toast_with_special_characters() {
        let toast = Toast::info("Test<>&\"'\\|/*?");
        assert!(toast.message().contains("<>"));
    }

    #[test]
    fn test_toast_custom_duration() {
        let toast = Toast::new(ToastLevel::Info, "Test", Duration::from_millis(100));
        assert!(toast.is_visible());

        // Can't easily test that it becomes invisible without sleeping
    }

    #[test]
    fn test_toast_duration_differences() {
        let info = Toast::info("Info");
        let success = Toast::success("Success");
        let warning = Toast::warning("Warning");
        let error = Toast::error("Error");

        // Duration is private, but we can verify they're all visible
        assert!(info.is_visible());
        assert!(success.is_visible());
        assert!(warning.is_visible());
        assert!(error.is_visible());
    }

    #[test]
    fn test_toast_manager_add_custom_toast() {
        let mut manager = ToastManager::new();
        let custom_toast = Toast::new(ToastLevel::Success, "Custom", Duration::from_secs(10));

        manager.add(custom_toast);
        assert_eq!(manager.len(), 1);
    }

    #[test]
    fn test_toast_manager_multiple_types() {
        let mut manager = ToastManager::new();

        manager.info("Info 1");
        manager.success("Success 1");
        manager.warning("Warning 1");
        manager.error("Error 1");
        manager.info("Info 2");

        assert_eq!(manager.len(), 5);
    }

    #[test]
    fn test_toast_manager_cleanup_keeps_visible() {
        let mut manager = ToastManager::new();

        manager.info("Test 1");
        manager.info("Test 2");
        manager.info("Test 3");

        manager.cleanup();
        // All should still be visible (just created)
        assert_eq!(manager.len(), 3);
    }

    #[test]
    fn test_toast_manager_clear_removes_all() {
        let mut manager = ToastManager::new();

        manager.info("Test 1");
        manager.success("Test 2");
        manager.warning("Test 3");
        manager.error("Test 4");

        assert_eq!(manager.len(), 4);

        manager.clear();
        assert_eq!(manager.len(), 0);
        assert!(manager.is_empty());
    }

    #[test]
    fn test_toast_manager_is_empty_initially() {
        let manager = ToastManager::new();
        assert!(manager.is_empty());
        assert_eq!(manager.len(), 0);
    }

    #[test]
    fn test_toast_manager_default() {
        let manager = ToastManager::default();
        assert!(manager.is_empty());
    }

    #[test]
    fn test_toast_manager_many_toasts() {
        let mut manager = ToastManager::new();

        for i in 0..100 {
            manager.info(&format!("Toast {}", i));
        }

        assert_eq!(manager.len(), 100);
    }

    #[test]
    fn test_toast_manager_mixed_cleanup() {
        let mut manager = ToastManager::new();

        manager.info("Keep this");
        manager.cleanup();

        assert_eq!(manager.len(), 1);
    }

    #[test]
    fn test_toast_level_equality() {
        assert_eq!(ToastLevel::Info, ToastLevel::Info);
        assert_eq!(ToastLevel::Success, ToastLevel::Success);
        assert_ne!(ToastLevel::Info, ToastLevel::Success);
        assert_ne!(ToastLevel::Warning, ToastLevel::Error);
    }

    #[test]
    fn test_toast_remaining_time_saturating() {
        let toast = Toast::new(ToastLevel::Info, "Test", Duration::from_millis(1));

        // Initially should have some time
        let remaining = toast.remaining_time();
        assert!(remaining <= Duration::from_millis(1));
    }

    #[test]
    fn test_toast_manager_sequential_operations() {
        let mut manager = ToastManager::new();

        manager.info("First");
        assert_eq!(manager.len(), 1);

        manager.success("Second");
        assert_eq!(manager.len(), 2);

        manager.clear();
        assert_eq!(manager.len(), 0);

        manager.error("Third");
        assert_eq!(manager.len(), 1);
    }

    #[test]
    fn test_toast_with_unicode_emoji_only() {
        let toast = Toast::info("🎉🚀🌟💯🔥");
        assert_eq!(toast.message(), "🎉🚀🌟💯🔥");
    }

    #[test]
    fn test_toast_with_whitespace_only() {
        let toast = Toast::info("     ");
        assert_eq!(toast.message(), "     ");
    }

    #[test]
    fn test_toast_manager_alternating_add_clear() {
        let mut manager = ToastManager::new();

        manager.info("Test");
        assert_eq!(manager.len(), 1);

        manager.clear();
        assert_eq!(manager.len(), 0);

        manager.success("Test 2");
        assert_eq!(manager.len(), 1);

        manager.clear();
        assert_eq!(manager.len(), 0);
    }

    #[test]
    fn test_toast_manager_cleanup_multiple_times() {
        let mut manager = ToastManager::new();

        manager.info("Test");
        manager.cleanup();
        manager.cleanup();
        manager.cleanup();

        assert_eq!(manager.len(), 1); // Should still be there
    }

    #[test]
    fn test_toast_clone() {
        let original = Toast::info("Original");
        let cloned = original.clone();

        assert_eq!(original.message(), cloned.message());
        assert_eq!(original.level(), cloned.level());
    }

    #[test]
    fn test_toast_level_copy() {
        let level = ToastLevel::Success;
        let copied = level;

        assert_eq!(level, copied);
    }

    #[test]
    fn test_toast_manager_with_very_long_messages() {
        let mut manager = ToastManager::new();

        let long = "X".repeat(10000);
        manager.info(&long);
        manager.success(&long);
        manager.warning(&long);

        assert_eq!(manager.len(), 3);
    }

    // ============================================================================
    // ADVANCED COMPREHENSIVE EDGE CASE TESTS (90%+ COVERAGE)
    // ============================================================================

    // ============ Stress Tests ============

    #[test]
    fn test_toast_manager_10000_toasts() {
        let mut manager = ToastManager::new();
        for i in 0..10000 {
            manager.info(format!("Toast {}", i));
        }
        assert_eq!(manager.len(), 10000);
    }

    #[test]
    fn test_toast_rapid_creation_all_types() {
        for _ in 0..1000 {
            let _info = Toast::info("Info");
            let _success = Toast::success("Success");
            let _warning = Toast::warning("Warning");
            let _error = Toast::error("Error");
        }
        // Just verify no crashes
    }

    #[test]
    fn test_toast_manager_rapid_add_clear_cycles() {
        let mut manager = ToastManager::new();
        for i in 0..100 {
            manager.info(format!("Message {}", i));
            manager.success(format!("Success {}", i));
            manager.clear();
            assert_eq!(manager.len(), 0);
        }
    }

    #[test]
    fn test_toast_manager_alternating_types_stress() {
        let mut manager = ToastManager::new();
        for i in 0..1000 {
            match i % 4 {
                0 => manager.info(format!("I{}", i)),
                1 => manager.success(format!("S{}", i)),
                2 => manager.warning(format!("W{}", i)),
                _ => manager.error(format!("E{}", i)),
            }
        }
        assert_eq!(manager.len(), 1000);
    }

    // ============ Unicode Edge Cases ============

    #[test]
    fn test_toast_with_rtl_text() {
        let toast = Toast::info("مرحبا العالم Hello שלום");
        assert!(toast.message().contains("مرحبا"));
        assert!(toast.message().contains("שלום"));
    }

    #[test]
    fn test_toast_with_emoji_sequences() {
        let toast = Toast::success("👨‍👩‍👧‍👦 Family emoji 🎉");
        assert!(toast.message().contains("👨‍👩‍👧‍👦"));
    }

    #[test]
    fn test_toast_with_combining_characters() {
        let toast = Toast::warning("Café résumé naïve");
        assert!(toast.message().contains("é"));
    }

    #[test]
    fn test_toast_with_zero_width_characters() {
        let toast = Toast::error("Test\u{200B}with\u{200B}zero\u{200B}width");
        assert!(toast.message().contains("\u{200B}"));
    }

    #[test]
    fn test_toast_with_all_unicode_types() {
        let toast = Toast::info("Latin αβγ 中文 日本語 한글 العربية עברית 🎉🚀");
        assert!(toast.message().contains("αβγ"));
        assert!(toast.message().contains("中文"));
    }

    #[test]
    fn test_toast_with_box_drawing_characters() {
        let toast = Toast::info("┌─┬─┐\n│ │ │\n├─┼─┤");
        assert!(toast.message().contains("┌"));
    }

    // ============ Duration Edge Cases ============

    #[test]
    fn test_toast_zero_duration() {
        let toast = Toast::new(ToastLevel::Info, "Zero", Duration::from_secs(0));
        // Might be immediately invisible
        let _ = toast.is_visible();
    }

    #[test]
    fn test_toast_very_long_duration() {
        let toast = Toast::new(
            ToastLevel::Success,
            "Long",
            Duration::from_secs(3600 * 24 * 365),
        );
        assert!(toast.is_visible());
        let remaining = toast.remaining_time();
        assert!(remaining > Duration::from_secs(3600 * 24 * 364));
    }

    #[test]
    fn test_toast_one_millisecond_duration() {
        let toast = Toast::new(ToastLevel::Warning, "Fast", Duration::from_millis(1));
        let remaining = toast.remaining_time();
        assert!(remaining <= Duration::from_millis(1));
    }

    #[test]
    fn test_toast_remaining_time_decreases() {
        let toast = Toast::info("Test");
        let remaining1 = toast.remaining_time();

        std::thread::sleep(Duration::from_millis(10));

        let remaining2 = toast.remaining_time();
        assert!(remaining2 <= remaining1);
    }

    // ============ ToastLevel Debug and Clone ============

    #[test]
    fn test_toast_level_debug_format() {
        let info = ToastLevel::Info;
        let success = ToastLevel::Success;
        let warning = ToastLevel::Warning;
        let error = ToastLevel::Error;

        assert!(format!("{:?}", info).contains("Info"));
        assert!(format!("{:?}", success).contains("Success"));
        assert!(format!("{:?}", warning).contains("Warning"));
        assert!(format!("{:?}", error).contains("Error"));
    }

    #[test]
    fn test_toast_level_clone() {
        let original = ToastLevel::Warning;
        let cloned = original.clone();
        assert_eq!(original, cloned);
    }

    #[test]
    fn test_toast_level_all_icons_unique() {
        let icons = vec![
            ToastLevel::Info.icon(),
            ToastLevel::Success.icon(),
            ToastLevel::Warning.icon(),
            ToastLevel::Error.icon(),
        ];

        for (i, icon1) in icons.iter().enumerate() {
            for (j, icon2) in icons.iter().enumerate() {
                if i != j {
                    assert_ne!(icon1, icon2);
                }
            }
        }
    }

    #[test]
    fn test_toast_level_all_border_colors() {
        // Just verify all methods work without panicking
        let _info_color = ToastLevel::Info.border_color();
        let _success_color = ToastLevel::Success.border_color();
        let _warning_color = ToastLevel::Warning.border_color();
        let _error_color = ToastLevel::Error.border_color();
    }

    // ============ Toast Clone and Debug ============

    #[test]
    fn test_toast_debug_format() {
        let toast = Toast::info("Debug test");
        let debug_str = format!("{:?}", toast);
        assert!(!debug_str.is_empty());
    }

    #[test]
    fn test_toast_clone_preserves_all_fields() {
        let original = Toast::warning("Original message");
        let cloned = original.clone();

        assert_eq!(original.message(), cloned.message());
        assert_eq!(original.level(), cloned.level());
        // Both should be visible (just created)
        assert!(original.is_visible());
        assert!(cloned.is_visible());
    }

    // ============ ToastManager Edge Cases ============

    #[test]
    fn test_toast_manager_empty_after_default() {
        let manager = ToastManager::default();
        assert!(manager.is_empty());
        assert_eq!(manager.len(), 0);
    }

    #[test]
    fn test_toast_manager_debug_format() {
        let mut manager = ToastManager::new();
        manager.info("Test");
        let debug_str = format!("{:?}", manager);
        assert!(!debug_str.is_empty());
    }

    #[test]
    fn test_toast_manager_multiple_cleanup_calls() {
        let mut manager = ToastManager::new();
        manager.info("Test 1");
        manager.success("Test 2");

        manager.cleanup();
        let count1 = manager.len();

        manager.cleanup();
        let count2 = manager.len();

        manager.cleanup();
        let count3 = manager.len();

        // Should remain stable
        assert_eq!(count1, count2);
        assert_eq!(count2, count3);
    }

    #[test]
    fn test_toast_manager_clear_then_add() {
        let mut manager = ToastManager::new();
        manager.info("First batch");
        manager.success("First batch 2");

        manager.clear();
        assert_eq!(manager.len(), 0);

        manager.warning("Second batch");
        assert_eq!(manager.len(), 1);
    }

    #[test]
    fn test_toast_manager_sequential_adds_preserved() {
        let mut manager = ToastManager::new();

        manager.info("1");
        manager.success("2");
        manager.warning("3");
        manager.error("4");

        assert_eq!(manager.len(), 4);

        // Toasts are stored in order added
        manager.cleanup();
        assert_eq!(manager.len(), 4); // All still visible
    }

    // ============ Message Edge Cases ============

    #[test]
    fn test_toast_extremely_long_message_100k() {
        let long = "M".repeat(100000);
        let toast = Toast::error(long.clone());
        assert_eq!(toast.message().len(), 100000);
    }

    #[test]
    fn test_toast_message_with_tabs() {
        let toast = Toast::info("Col1\tCol2\tCol3");
        assert!(toast.message().contains("\t"));
    }

    #[test]
    fn test_toast_message_with_multiple_newlines() {
        let toast = Toast::warning("Line1\n\n\nLine2");
        assert_eq!(toast.message().matches('\n').count(), 3);
    }

    #[test]
    fn test_toast_message_with_carriage_returns() {
        let toast = Toast::error("Text\rWith\rCR");
        assert!(toast.message().contains("\r"));
    }

    #[test]
    fn test_toast_message_with_ansi_sequences() {
        let toast = Toast::info("\x1b[31mRed\x1b[0m");
        assert!(toast.message().contains("\x1b"));
    }

    #[test]
    fn test_toast_message_only_special_chars() {
        let toast = Toast::success("!@#$%^&*()_+-=[]{}|;:',.<>?/~`");
        assert!(toast.message().contains("!@#"));
    }

    // ============ Complex Manager Operations ============

    #[test]
    fn test_toast_manager_mixed_operations_sequence() {
        let mut manager = ToastManager::new();

        manager.info("1");
        manager.success("2");
        assert_eq!(manager.len(), 2);

        manager.cleanup();
        assert_eq!(manager.len(), 2);

        manager.warning("3");
        assert_eq!(manager.len(), 3);

        manager.clear();
        assert_eq!(manager.len(), 0);

        manager.error("4");
        assert_eq!(manager.len(), 1);
    }

    #[test]
    fn test_toast_manager_add_custom_with_various_durations() {
        let mut manager = ToastManager::new();

        manager.add(Toast::new(
            ToastLevel::Info,
            "1sec",
            Duration::from_secs(1),
        ));
        manager.add(Toast::new(
            ToastLevel::Success,
            "5sec",
            Duration::from_secs(5),
        ));
        manager.add(Toast::new(
            ToastLevel::Warning,
            "10sec",
            Duration::from_secs(10),
        ));

        assert_eq!(manager.len(), 3);
    }

    #[test]
    fn test_toast_manager_with_unicode_messages() {
        let mut manager = ToastManager::new();

        manager.info("日本語");
        manager.success("العربية");
        manager.warning("עברית");
        manager.error("한글");

        assert_eq!(manager.len(), 4);
    }

    // ============ Comprehensive Stress Test ============

    #[test]
    fn test_comprehensive_toast_manager_stress() {
        let mut manager = ToastManager::new();

        // Add 100 toasts with varying types and messages
        for i in 0..100 {
            let message = format!("Message {} with unicode 日本語 🎉", i);

            match i % 4 {
                0 => manager.info(message),
                1 => manager.success(message),
                2 => manager.warning(message),
                _ => manager.error(message),
            }
        }

        assert_eq!(manager.len(), 100);

        // Cleanup shouldn't remove any (all just created)
        manager.cleanup();
        assert_eq!(manager.len(), 100);

        // Clear and verify
        manager.clear();
        assert_eq!(manager.len(), 0);
        assert!(manager.is_empty());

        // Add more after clear
        for i in 0..50 {
            manager.add(Toast::new(
                ToastLevel::Success,
                format!("Custom {}", i),
                Duration::from_secs(i % 10 + 1),
            ));
        }

        assert_eq!(manager.len(), 50);
    }

    #[test]
    fn test_toast_level_coverage_all_methods() {
        let levels = vec![
            ToastLevel::Info,
            ToastLevel::Success,
            ToastLevel::Warning,
            ToastLevel::Error,
        ];

        for level in levels {
            // Call all methods to ensure they don't panic
            let _icon = level.icon();
            let _color = level.border_color();
            let _debug = format!("{:?}", level);
            let _clone = level.clone();
            let _copy = level;
        }
    }

    #[test]
    fn test_toast_all_constructors() {
        let info = Toast::info("Info test");
        let success = Toast::success("Success test");
        let warning = Toast::warning("Warning test");
        let error = Toast::error("Error test");
        let custom = Toast::new(ToastLevel::Info, "Custom", Duration::from_secs(1));

        assert_eq!(info.level(), ToastLevel::Info);
        assert_eq!(success.level(), ToastLevel::Success);
        assert_eq!(warning.level(), ToastLevel::Warning);
        assert_eq!(error.level(), ToastLevel::Error);
        assert_eq!(custom.level(), ToastLevel::Info);
    }

    #[test]
    fn test_toast_manager_len_consistency() {
        let mut manager = ToastManager::new();

        for i in 1..=10 {
            manager.info(format!("Message {}", i));
            assert_eq!(manager.len(), i);
            assert!(!manager.is_empty());
        }

        for i in (0..10).rev() {
            manager.toasts.pop();
            assert_eq!(manager.len(), i);
        }

        assert!(manager.is_empty());
    }
}

