//! Cross-platform clipboard integration
//!
//! Provides unified clipboard operations (copy/paste) that work across
//! different operating systems and terminal environments.
//!
//! # Examples
//!
//! ```no_run
//! use toad::clipboard::Clipboard;
//!
//! let mut clipboard = Clipboard::new().unwrap();
//! clipboard.copy("Hello, clipboard!").unwrap();
//!
//! let text = clipboard.paste().unwrap();
//! assert_eq!(text, "Hello, clipboard!");
//! ```

use copypasta::{ClipboardContext, ClipboardProvider};
use std::sync::{Arc, Mutex};
use thiserror::Error;

/// Clipboard operation errors
///
/// # Examples
///
/// ```
/// use toad::clipboard::ClipboardError;
///
/// let err = ClipboardError::NotAvailable("System clipboard not accessible".to_string());
/// assert!(err.to_string().contains("not accessible"));
/// ```
#[derive(Debug, Error)]
pub enum ClipboardError {
    /// Clipboard is not available on this platform or environment
    #[error("Clipboard not available: {0}")]
    NotAvailable(String),

    /// Failed to copy text to clipboard
    #[error("Failed to copy to clipboard: {0}")]
    CopyFailed(String),

    /// Failed to paste text from clipboard
    #[error("Failed to paste from clipboard: {0}")]
    PasteFailed(String),
}

/// Cross-platform clipboard interface
///
/// Provides a thread-safe clipboard implementation that works across
/// different operating systems. The clipboard context is wrapped in
/// Arc<Mutex<>> to allow safe concurrent access.
///
/// # Platform Support
///
/// - **Linux**: X11 or Wayland clipboard
/// - **macOS**: Native pasteboard
/// - **Windows**: Native clipboard API
///
/// # Examples
///
/// ```no_run
/// use toad::clipboard::Clipboard;
///
/// let mut clipboard = Clipboard::new().unwrap();
///
/// // Copy text
/// clipboard.copy("Sample text").unwrap();
///
/// // Paste text
/// let content = clipboard.paste().unwrap();
/// assert_eq!(content, "Sample text");
/// ```
pub struct Clipboard {
    context: Arc<Mutex<ClipboardContext>>,
}

impl Clipboard {
    /// Create a new clipboard instance
    ///
    /// # Errors
    ///
    /// Returns `ClipboardError::NotAvailable` if the system clipboard
    /// cannot be accessed (e.g., in headless environments).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use toad::clipboard::Clipboard;
    ///
    /// let clipboard = Clipboard::new().unwrap();
    /// ```
    pub fn new() -> Result<Self, ClipboardError> {
        let context = ClipboardContext::new()
            .map_err(|e| ClipboardError::NotAvailable(e.to_string()))?;

        Ok(Self {
            context: Arc::new(Mutex::new(context)),
        })
    }

    /// Copy text to the clipboard
    ///
    /// # Errors
    ///
    /// Returns `ClipboardError::CopyFailed` if the copy operation fails.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use toad::clipboard::Clipboard;
    ///
    /// let mut clipboard = Clipboard::new().unwrap();
    /// clipboard.copy("Text to copy").unwrap();
    /// ```
    pub fn copy(&mut self, text: &str) -> Result<(), ClipboardError> {
        let mut ctx = self.context.lock()
            .map_err(|e| ClipboardError::CopyFailed(format!("Lock error: {}", e)))?;

        ctx.set_contents(text.to_string())
            .map_err(|e| ClipboardError::CopyFailed(e.to_string()))
    }

    /// Paste text from the clipboard
    ///
    /// # Errors
    ///
    /// Returns `ClipboardError::PasteFailed` if the paste operation fails
    /// or if the clipboard is empty.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use toad::clipboard::Clipboard;
    ///
    /// let mut clipboard = Clipboard::new().unwrap();
    /// clipboard.copy("Hello").unwrap();
    ///
    /// let text = clipboard.paste().unwrap();
    /// assert_eq!(text, "Hello");
    /// ```
    pub fn paste(&mut self) -> Result<String, ClipboardError> {
        let mut ctx = self.context.lock()
            .map_err(|e| ClipboardError::PasteFailed(format!("Lock error: {}", e)))?;

        ctx.get_contents()
            .map_err(|e| ClipboardError::PasteFailed(e.to_string()))
    }

    /// Check if clipboard contains text
    ///
    /// Returns `true` if the clipboard contains non-empty text content.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use toad::clipboard::Clipboard;
    ///
    /// let mut clipboard = Clipboard::new().unwrap();
    /// clipboard.copy("content").unwrap();
    ///
    /// assert!(clipboard.has_content().unwrap());
    /// ```
    pub fn has_content(&mut self) -> Result<bool, ClipboardError> {
        match self.paste() {
            Ok(content) => Ok(!content.is_empty()),
            Err(_) => Ok(false),
        }
    }

    /// Clear the clipboard
    ///
    /// Sets the clipboard to an empty string.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use toad::clipboard::Clipboard;
    ///
    /// let mut clipboard = Clipboard::new().unwrap();
    /// clipboard.copy("test").unwrap();
    /// clipboard.clear().unwrap();
    ///
    /// assert!(!clipboard.has_content().unwrap());
    /// ```
    pub fn clear(&mut self) -> Result<(), ClipboardError> {
        self.copy("")
    }
}

impl Clone for Clipboard {
    fn clone(&self) -> Self {
        Self {
            context: Arc::clone(&self.context),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // Requires system clipboard access
    fn test_clipboard_copy_paste() {
        let mut clipboard = Clipboard::new().unwrap();
        let test_text = "Test clipboard content";

        clipboard.copy(test_text).unwrap();
        let result = clipboard.paste().unwrap();

        assert_eq!(result, test_text);
    }

    #[test]
    #[ignore] // Requires system clipboard access
    fn test_clipboard_clear() {
        let mut clipboard = Clipboard::new().unwrap();

        clipboard.copy("content").unwrap();
        assert!(clipboard.has_content().unwrap());

        clipboard.clear().unwrap();
        let result = clipboard.paste().unwrap();
        assert_eq!(result, "");
    }

    #[test]
    #[ignore] // Requires system clipboard access
    fn test_clipboard_has_content() {
        let mut clipboard = Clipboard::new().unwrap();

        clipboard.clear().unwrap();
        assert!(!clipboard.has_content().unwrap());

        clipboard.copy("data").unwrap();
        assert!(clipboard.has_content().unwrap());
    }

    #[test]
    #[ignore] // Requires system clipboard access
    fn test_clipboard_clone() {
        let mut clipboard1 = Clipboard::new().unwrap();
        let mut clipboard2 = clipboard1.clone();

        clipboard1.copy("shared content").unwrap();
        let result = clipboard2.paste().unwrap();

        assert_eq!(result, "shared content");
    }

    #[test]
    fn test_clipboard_error_display() {
        let err = ClipboardError::NotAvailable("test error".to_string());
        assert!(err.to_string().contains("test error"));

        let err = ClipboardError::CopyFailed("copy error".to_string());
        assert!(err.to_string().contains("copy error"));

        let err = ClipboardError::PasteFailed("paste error".to_string());
        assert!(err.to_string().contains("paste error"));
    }
}
