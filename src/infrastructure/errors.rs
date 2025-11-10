//! Error handling and display system
//!
//! Provides comprehensive error handling with categorization, display,
//! and history tracking. Integrates with the modal system for user-friendly
//! error presentation.
//!
//! # Related Modules
//!
//! For LLM-specific error types (API failures, rate limits, network errors),
//! see [`crate::ai::llm::errors`].
//!
//! # Examples
//!
//! ## Basic Error Handling
//!
//! ```
//! use toad::errors::{ErrorHandler, ErrorSeverity};
//!
//! let mut handler = ErrorHandler::new();
//!
//! // Report an error
//! handler.report_error(
//!     ErrorSeverity::Error,
//!     "Failed to save file",
//!     Some("Check file permissions".to_string())
//! );
//!
//! assert_eq!(handler.error_count(), 1);
//! ```
//!
//! ## Error with Context
//!
//! ```
//! use toad::errors::{ErrorHandler, ErrorSeverity, ErrorEntry};
//!
//! let mut handler = ErrorHandler::new();
//!
//! let error = ErrorEntry::new(
//!     ErrorSeverity::Warning,
//!     "Configuration outdated"
//! ).with_details("Consider updating your config.toml");
//!
//! handler.add_error(error);
//! ```

use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};

/// Error severity levels
///
/// # Examples
///
/// ```
/// use toad::errors::ErrorSeverity;
///
/// let severity = ErrorSeverity::Error;
/// assert_eq!(severity.icon(), "✗");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ErrorSeverity {
    /// Informational message
    Info,
    /// Warning that doesn't prevent operation
    Warning,
    /// Error that prevents operation
    Error,
    /// Critical error requiring immediate attention
    Critical,
}

impl ErrorSeverity {
    /// Get icon for this severity level
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::errors::ErrorSeverity;
    ///
    /// assert_eq!(ErrorSeverity::Info.icon(), "ℹ");
    /// assert_eq!(ErrorSeverity::Warning.icon(), "⚠");
    /// assert_eq!(ErrorSeverity::Error.icon(), "✗");
    /// assert_eq!(ErrorSeverity::Critical.icon(), "⚠");
    /// ```
    pub fn icon(&self) -> &'static str {
        match self {
            ErrorSeverity::Info => "ℹ",
            ErrorSeverity::Warning => "⚠",
            ErrorSeverity::Error => "✗",
            ErrorSeverity::Critical => "⚠",
        }
    }

    /// Get color for this severity level (as u8 for ratatui Color)
    pub fn color_code(&self) -> (u8, u8, u8) {
        match self {
            ErrorSeverity::Info => (100, 181, 246),   // Blue
            ErrorSeverity::Warning => (255, 213, 79), // Yellow
            ErrorSeverity::Error => (239, 83, 80),    // Red
            ErrorSeverity::Critical => (229, 57, 53), // Dark Red
        }
    }

    /// Get display name
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::errors::ErrorSeverity;
    ///
    /// assert_eq!(ErrorSeverity::Error.name(), "Error");
    /// assert_eq!(ErrorSeverity::Warning.name(), "Warning");
    /// ```
    pub fn name(&self) -> &'static str {
        match self {
            ErrorSeverity::Info => "Info",
            ErrorSeverity::Warning => "Warning",
            ErrorSeverity::Error => "Error",
            ErrorSeverity::Critical => "Critical",
        }
    }
}

/// Individual error entry
///
/// # Examples
///
/// ```
/// use toad::errors::{ErrorEntry, ErrorSeverity};
///
/// let error = ErrorEntry::new(ErrorSeverity::Error, "File not found")
///     .with_details("The requested file does not exist")
///     .with_code("E001");
///
/// assert_eq!(error.message(), "File not found");
/// assert_eq!(error.severity(), ErrorSeverity::Error);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorEntry {
    severity: ErrorSeverity,
    message: String,
    details: Option<String>,
    error_code: Option<String>,
    timestamp: SystemTime,
    count: usize,
}

impl ErrorEntry {
    /// Create a new error entry
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::errors::{ErrorEntry, ErrorSeverity};
    ///
    /// let error = ErrorEntry::new(ErrorSeverity::Warning, "Low memory");
    /// assert_eq!(error.message(), "Low memory");
    /// ```
    pub fn new(severity: ErrorSeverity, message: impl Into<String>) -> Self {
        Self {
            severity,
            message: message.into(),
            details: None,
            error_code: None,
            timestamp: SystemTime::now(),
            count: 1,
        }
    }

    /// Add detailed error information
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::errors::{ErrorEntry, ErrorSeverity};
    ///
    /// let error = ErrorEntry::new(ErrorSeverity::Error, "Failed")
    ///     .with_details("Network connection timeout");
    ///
    /// assert_eq!(error.details(), Some("Network connection timeout"));
    /// ```
    pub fn with_details(mut self, details: impl Into<String>) -> Self {
        self.details = Some(details.into());
        self
    }

    /// Add error code
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::errors::{ErrorEntry, ErrorSeverity};
    ///
    /// let error = ErrorEntry::new(ErrorSeverity::Error, "Parse error")
    ///     .with_code("E0001");
    ///
    /// assert_eq!(error.error_code(), Some("E0001"));
    /// ```
    pub fn with_code(mut self, code: impl Into<String>) -> Self {
        self.error_code = Some(code.into());
        self
    }

    /// Get error severity
    pub fn severity(&self) -> ErrorSeverity {
        self.severity
    }

    /// Get error message
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Get error details
    pub fn details(&self) -> Option<&str> {
        self.details.as_deref()
    }

    /// Get error code
    pub fn error_code(&self) -> Option<&str> {
        self.error_code.as_deref()
    }

    /// Get timestamp
    pub fn timestamp(&self) -> SystemTime {
        self.timestamp
    }

    /// Get occurrence count
    pub fn count(&self) -> usize {
        self.count
    }

    /// Increment error count (for duplicate errors)
    pub fn increment_count(&mut self) {
        self.count += 1;
        self.timestamp = SystemTime::now(); // Update to latest occurrence
    }

    /// Get age of error
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::errors::{ErrorEntry, ErrorSeverity};
    /// use std::time::Duration;
    ///
    /// let error = ErrorEntry::new(ErrorSeverity::Info, "Test");
    /// let age = error.age();
    /// assert!(age < Duration::from_secs(1));
    /// ```
    pub fn age(&self) -> Duration {
        self.timestamp.elapsed().unwrap_or(Duration::from_secs(0))
    }

    /// Format error for display
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::errors::{ErrorEntry, ErrorSeverity};
    ///
    /// let error = ErrorEntry::new(ErrorSeverity::Error, "Failed")
    ///     .with_code("E001");
    ///
    /// let formatted = error.format();
    /// assert!(formatted.contains("Failed"));
    /// assert!(formatted.contains("E001"));
    /// ```
    pub fn format(&self) -> String {
        let mut result = self.message.clone();

        if let Some(code) = &self.error_code {
            result = format!("[{}] {}", code, result);
        }

        if self.count > 1 {
            result = format!("{} ({}x)", result, self.count);
        }

        result
    }
}

/// Error handler with history and display management
///
/// # Examples
///
/// ```
/// use toad::errors::{ErrorHandler, ErrorSeverity};
///
/// let mut handler = ErrorHandler::new();
///
/// handler.report_error(
///     ErrorSeverity::Warning,
///     "Deprecated API usage",
///     None
/// );
///
/// assert_eq!(handler.total_count(), 1);
/// assert_eq!(handler.warning_count(), 1);
/// ```
#[derive(Debug, Clone, Default)]
pub struct ErrorHandler {
    errors: Vec<ErrorEntry>,
    max_history: usize,
    auto_dismiss_duration: Duration,
}

impl ErrorHandler {
    /// Create a new error handler
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::errors::ErrorHandler;
    ///
    /// let handler = ErrorHandler::new();
    /// assert_eq!(handler.error_count(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            max_history: 100,
            auto_dismiss_duration: Duration::from_secs(30),
        }
    }

    /// Create handler with custom settings
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::errors::ErrorHandler;
    /// use std::time::Duration;
    ///
    /// let handler = ErrorHandler::with_settings(50, Duration::from_secs(10));
    /// assert_eq!(handler.max_history(), 50);
    /// ```
    pub fn with_settings(max_history: usize, auto_dismiss: Duration) -> Self {
        Self {
            errors: Vec::new(),
            max_history,
            auto_dismiss_duration: auto_dismiss,
        }
    }

    /// Report a new error
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::errors::{ErrorHandler, ErrorSeverity};
    ///
    /// let mut handler = ErrorHandler::new();
    /// handler.report_error(
    ///     ErrorSeverity::Error,
    ///     "Operation failed",
    ///     Some("Retry later".to_string())
    /// );
    ///
    /// assert_eq!(handler.error_count(), 1);
    /// ```
    pub fn report_error(
        &mut self,
        severity: ErrorSeverity,
        message: impl Into<String>,
        details: Option<String>,
    ) {
        let mut error = ErrorEntry::new(severity, message);
        if let Some(d) = details {
            error = error.with_details(d);
        }
        self.add_error(error);
    }

    /// Add an error entry
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::errors::{ErrorHandler, ErrorEntry, ErrorSeverity};
    ///
    /// let mut handler = ErrorHandler::new();
    /// let error = ErrorEntry::new(ErrorSeverity::Info, "Started");
    ///
    /// handler.add_error(error);
    /// assert_eq!(handler.total_count(), 1);
    /// ```
    pub fn add_error(&mut self, error: ErrorEntry) {
        // Check for duplicate errors
        if let Some(existing) = self
            .errors
            .iter_mut()
            .find(|e| e.severity == error.severity && e.message == error.message)
        {
            existing.increment_count();
        } else {
            self.errors.push(error);

            // Limit history size
            if self.errors.len() > self.max_history {
                self.errors.remove(0);
            }
        }
    }

    /// Get all errors
    pub fn errors(&self) -> &[ErrorEntry] {
        &self.errors
    }

    /// Get errors by severity
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::errors::{ErrorHandler, ErrorSeverity};
    ///
    /// let mut handler = ErrorHandler::new();
    /// handler.report_error(ErrorSeverity::Error, "Error 1", None);
    /// handler.report_error(ErrorSeverity::Warning, "Warning 1", None);
    ///
    /// let errors: Vec<_> = handler.errors_by_severity(ErrorSeverity::Error).collect();
    /// assert_eq!(errors.len(), 1);
    /// ```
    pub fn errors_by_severity(&self, severity: ErrorSeverity) -> impl Iterator<Item = &ErrorEntry> {
        self.errors.iter().filter(move |e| e.severity == severity)
    }

    /// Get recent errors (within auto-dismiss duration)
    pub fn recent_errors(&self) -> impl Iterator<Item = &ErrorEntry> {
        let auto_dismiss = self.auto_dismiss_duration;
        self.errors.iter().filter(move |e| e.age() < auto_dismiss)
    }

    /// Clear all errors
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::errors::{ErrorHandler, ErrorSeverity};
    ///
    /// let mut handler = ErrorHandler::new();
    /// handler.report_error(ErrorSeverity::Info, "Test", None);
    /// handler.clear();
    ///
    /// assert_eq!(handler.total_count(), 0);
    /// ```
    pub fn clear(&mut self) {
        self.errors.clear();
    }

    /// Clear errors older than auto-dismiss duration
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::errors::ErrorHandler;
    ///
    /// let mut handler = ErrorHandler::new();
    /// handler.cleanup_old_errors();
    /// ```
    pub fn cleanup_old_errors(&mut self) {
        let cutoff = self.auto_dismiss_duration;
        self.errors.retain(|e| e.age() < cutoff);
    }

    /// Get total error count
    pub fn total_count(&self) -> usize {
        self.errors.len()
    }

    /// Get count by severity
    fn count_by_severity(&self, severity: ErrorSeverity) -> usize {
        self.errors
            .iter()
            .filter(|e| e.severity == severity)
            .count()
    }

    /// Get info count
    pub fn info_count(&self) -> usize {
        self.count_by_severity(ErrorSeverity::Info)
    }

    /// Get warning count
    pub fn warning_count(&self) -> usize {
        self.count_by_severity(ErrorSeverity::Warning)
    }

    /// Get error count
    pub fn error_count(&self) -> usize {
        self.count_by_severity(ErrorSeverity::Error)
    }

    /// Get critical error count
    pub fn critical_count(&self) -> usize {
        self.count_by_severity(ErrorSeverity::Critical)
    }

    /// Check if there are any errors
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    /// Check if there are any critical errors
    pub fn has_critical(&self) -> bool {
        self.errors
            .iter()
            .any(|e| e.severity == ErrorSeverity::Critical)
    }

    /// Get maximum history size
    pub fn max_history(&self) -> usize {
        self.max_history
    }

    /// Get the most recent error
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::errors::{ErrorHandler, ErrorSeverity};
    ///
    /// let mut handler = ErrorHandler::new();
    /// handler.report_error(ErrorSeverity::Info, "First", None);
    /// handler.report_error(ErrorSeverity::Error, "Latest", None);
    ///
    /// let latest = handler.latest_error();
    /// assert_eq!(latest.unwrap().message(), "Latest");
    /// ```
    pub fn latest_error(&self) -> Option<&ErrorEntry> {
        self.errors.last()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_severity_icons() {
        assert_eq!(ErrorSeverity::Info.icon(), "ℹ");
        assert_eq!(ErrorSeverity::Warning.icon(), "⚠");
        assert_eq!(ErrorSeverity::Error.icon(), "✗");
        assert_eq!(ErrorSeverity::Critical.icon(), "⚠");
    }

    #[test]
    fn test_error_severity_names() {
        assert_eq!(ErrorSeverity::Info.name(), "Info");
        assert_eq!(ErrorSeverity::Warning.name(), "Warning");
        assert_eq!(ErrorSeverity::Error.name(), "Error");
        assert_eq!(ErrorSeverity::Critical.name(), "Critical");
    }

    #[test]
    fn test_error_entry_creation() {
        let error = ErrorEntry::new(ErrorSeverity::Error, "Test error");

        assert_eq!(error.severity(), ErrorSeverity::Error);
        assert_eq!(error.message(), "Test error");
        assert_eq!(error.count(), 1);
    }

    #[test]
    fn test_error_entry_with_details() {
        let error =
            ErrorEntry::new(ErrorSeverity::Warning, "Warning").with_details("Additional info");

        assert_eq!(error.details(), Some("Additional info"));
    }

    #[test]
    fn test_error_entry_with_code() {
        let error = ErrorEntry::new(ErrorSeverity::Error, "Error").with_code("E001");

        assert_eq!(error.error_code(), Some("E001"));
    }

    #[test]
    fn test_error_entry_format() {
        let error = ErrorEntry::new(ErrorSeverity::Error, "Failed").with_code("E001");

        let formatted = error.format();
        assert!(formatted.contains("Failed"));
        assert!(formatted.contains("E001"));
    }

    #[test]
    fn test_error_entry_increment_count() {
        let mut error = ErrorEntry::new(ErrorSeverity::Info, "Test");
        assert_eq!(error.count(), 1);

        error.increment_count();
        assert_eq!(error.count(), 2);

        error.increment_count();
        assert_eq!(error.count(), 3);
    }

    #[test]
    fn test_error_handler_creation() {
        let handler = ErrorHandler::new();
        assert_eq!(handler.total_count(), 0);
        assert!(!handler.has_errors());
    }

    #[test]
    fn test_error_handler_report_error() {
        let mut handler = ErrorHandler::new();

        handler.report_error(ErrorSeverity::Error, "Test error", None);

        assert_eq!(handler.total_count(), 1);
        assert_eq!(handler.error_count(), 1);
        assert!(handler.has_errors());
    }

    #[test]
    fn test_error_handler_multiple_errors() {
        let mut handler = ErrorHandler::new();

        handler.report_error(ErrorSeverity::Info, "Info", None);
        handler.report_error(ErrorSeverity::Warning, "Warning", None);
        handler.report_error(ErrorSeverity::Error, "Error", None);
        handler.report_error(ErrorSeverity::Critical, "Critical", None);

        assert_eq!(handler.total_count(), 4);
        assert_eq!(handler.info_count(), 1);
        assert_eq!(handler.warning_count(), 1);
        assert_eq!(handler.error_count(), 1);
        assert_eq!(handler.critical_count(), 1);
    }

    #[test]
    fn test_error_handler_duplicate_errors() {
        let mut handler = ErrorHandler::new();

        handler.report_error(ErrorSeverity::Error, "Same error", None);
        handler.report_error(ErrorSeverity::Error, "Same error", None);
        handler.report_error(ErrorSeverity::Error, "Same error", None);

        // Should only have 1 error entry with count 3
        assert_eq!(handler.total_count(), 1);

        let error = handler.latest_error().unwrap();
        assert_eq!(error.count(), 3);
    }

    #[test]
    fn test_error_handler_clear() {
        let mut handler = ErrorHandler::new();

        handler.report_error(ErrorSeverity::Error, "Error 1", None);
        handler.report_error(ErrorSeverity::Error, "Error 2", None);

        handler.clear();

        assert_eq!(handler.total_count(), 0);
        assert!(!handler.has_errors());
    }

    #[test]
    fn test_error_handler_latest_error() {
        let mut handler = ErrorHandler::new();

        handler.report_error(ErrorSeverity::Info, "First", None);
        handler.report_error(ErrorSeverity::Error, "Latest", None);

        let latest = handler.latest_error();
        assert!(latest.is_some());
        assert_eq!(latest.unwrap().message(), "Latest");
    }

    #[test]
    fn test_error_handler_has_critical() {
        let mut handler = ErrorHandler::new();

        assert!(!handler.has_critical());

        handler.report_error(ErrorSeverity::Error, "Error", None);
        assert!(!handler.has_critical());

        handler.report_error(ErrorSeverity::Critical, "Critical", None);
        assert!(handler.has_critical());
    }

    #[test]
    fn test_error_handler_with_settings() {
        let handler = ErrorHandler::with_settings(50, Duration::from_secs(10));

        assert_eq!(handler.max_history(), 50);
    }

    #[test]
    fn test_error_handler_max_history() {
        let mut handler = ErrorHandler::with_settings(3, Duration::from_secs(30));

        handler.report_error(ErrorSeverity::Info, "Error 1", None);
        handler.report_error(ErrorSeverity::Info, "Error 2", None);
        handler.report_error(ErrorSeverity::Info, "Error 3", None);
        handler.report_error(ErrorSeverity::Info, "Error 4", None);

        // Should only keep last 3
        assert_eq!(handler.total_count(), 3);
    }

    #[test]
    fn test_errors_by_severity() {
        let mut handler = ErrorHandler::new();

        handler.report_error(ErrorSeverity::Error, "Error 1", None);
        handler.report_error(ErrorSeverity::Warning, "Warning 1", None);
        handler.report_error(ErrorSeverity::Error, "Error 2", None);

        let errors: Vec<_> = handler.errors_by_severity(ErrorSeverity::Error).collect();
        assert_eq!(errors.len(), 2);
    }
}
