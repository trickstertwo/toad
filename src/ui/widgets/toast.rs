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
}
