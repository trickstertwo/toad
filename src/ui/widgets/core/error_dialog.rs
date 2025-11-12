//! Error dialog widget with recovery options
//!
//! Displays error messages with context and recovery actions.
//!
//! # Features
//!
//! - Error type and message display
//! - Context information
//! - Recovery actions (retry, switch model, check config, view logs)
//! - Keyboard navigation
//! - Preserves conversation state
//!
//! # Examples
//!
//! ```no_run
//! use toad::ui::widgets::core::error_dialog::ErrorDialog;
//!
//! let error = ErrorDialog::new("API Error", "Rate limit exceeded");
//! ```

use crate::ui::atoms::Block;
use crate::ui::theme::{ResolvedThemeColors, ToadTheme};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Clear, Paragraph, Wrap},
    Frame,
};

/// Recovery action that the user can take
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryAction {
    /// Retry the failed operation with the same model
    Retry,
    /// Switch to a different model
    SwitchModel,
    /// Check API key configuration
    CheckConfig,
    /// View detailed error log
    ViewLog,
    /// Dismiss the error dialog
    Dismiss,
}

impl RecoveryAction {
    /// Get the label for this action
    pub fn label(&self) -> &'static str {
        match self {
            RecoveryAction::Retry => "Retry",
            RecoveryAction::SwitchModel => "Switch Model",
            RecoveryAction::CheckConfig => "Check Config",
            RecoveryAction::ViewLog => "View Log",
            RecoveryAction::Dismiss => "Dismiss",
        }
    }

    /// Get the keybinding hint for this action
    pub fn keybind(&self) -> &'static str {
        match self {
            RecoveryAction::Retry => "r",
            RecoveryAction::SwitchModel => "m",
            RecoveryAction::CheckConfig => "c",
            RecoveryAction::ViewLog => "l",
            RecoveryAction::Dismiss => "Esc",
        }
    }
}

/// Error type classification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorType {
    /// Network connectivity error
    Network,
    /// API authentication error
    Authentication,
    /// Rate limiting error
    RateLimit,
    /// Invalid request error
    InvalidRequest,
    /// Server error
    Server,
    /// Unknown error
    Unknown,
}

impl ErrorType {
    /// Get a user-friendly name for this error type
    pub fn display_name(&self) -> &'static str {
        match self {
            ErrorType::Network => "Network Error",
            ErrorType::Authentication => "Authentication Error",
            ErrorType::RateLimit => "Rate Limit",
            ErrorType::InvalidRequest => "Invalid Request",
            ErrorType::Server => "Server Error",
            ErrorType::Unknown => "Error",
        }
    }

    /// Get suggested recovery actions for this error type
    pub fn suggested_actions(&self) -> Vec<RecoveryAction> {
        match self {
            ErrorType::Network => vec![
                RecoveryAction::Retry,
                RecoveryAction::ViewLog,
                RecoveryAction::Dismiss,
            ],
            ErrorType::Authentication => vec![
                RecoveryAction::CheckConfig,
                RecoveryAction::ViewLog,
                RecoveryAction::Dismiss,
            ],
            ErrorType::RateLimit => vec![
                RecoveryAction::SwitchModel,
                RecoveryAction::ViewLog,
                RecoveryAction::Dismiss,
            ],
            ErrorType::InvalidRequest => vec![
                RecoveryAction::ViewLog,
                RecoveryAction::Dismiss,
            ],
            ErrorType::Server => vec![
                RecoveryAction::Retry,
                RecoveryAction::SwitchModel,
                RecoveryAction::ViewLog,
                RecoveryAction::Dismiss,
            ],
            ErrorType::Unknown => vec![
                RecoveryAction::Retry,
                RecoveryAction::ViewLog,
                RecoveryAction::Dismiss,
            ],
        }
    }
}

/// Error dialog widget
///
/// Displays error information with recovery options.
///
/// # Examples
///
/// ```
/// use toad::ui::widgets::core::error_dialog::{ErrorDialog, ErrorType};
///
/// let error = ErrorDialog::new(ErrorType::RateLimit, "Rate limit exceeded");
/// ```
#[derive(Debug, Clone)]
pub struct ErrorDialog {
    /// Error type
    error_type: ErrorType,
    /// Error message
    message: String,
    /// Optional context information
    context: Option<String>,
    /// Currently selected action index
    selected_action: usize,
    /// Available recovery actions
    actions: Vec<RecoveryAction>,
}

impl ErrorDialog {
    /// Create a new error dialog
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::widgets::core::error_dialog::{ErrorDialog, ErrorType};
    ///
    /// let error = ErrorDialog::new(ErrorType::Network, "Connection failed");
    /// ```
    pub fn new(error_type: ErrorType, message: impl Into<String>) -> Self {
        let actions = error_type.suggested_actions();

        Self {
            error_type,
            message: message.into(),
            context: None,
            selected_action: 0,
            actions,
        }
    }

    /// Create error dialog from error string (infers type from message)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::widgets::core::error_dialog::ErrorDialog;
    ///
    /// let error = ErrorDialog::from_error("Rate limit exceeded");
    /// ```
    pub fn from_error(error: impl Into<String>) -> Self {
        let message = error.into();
        let error_type = Self::infer_error_type(&message);
        Self::new(error_type, message)
    }

    /// Add context information
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::widgets::core::error_dialog::{ErrorDialog, ErrorType};
    ///
    /// let error = ErrorDialog::new(ErrorType::Network, "Connection failed")
    ///     .with_context("While sending message to Claude");
    /// ```
    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.context = Some(context.into());
        self
    }

    /// Infer error type from error message
    fn infer_error_type(message: &str) -> ErrorType {
        let lower = message.to_lowercase();

        if lower.contains("rate limit") || lower.contains("429") {
            ErrorType::RateLimit
        } else if lower.contains("auth") || lower.contains("api key") || lower.contains("401") || lower.contains("403") {
            ErrorType::Authentication
        } else if lower.contains("network") || lower.contains("connection") || lower.contains("timeout") {
            ErrorType::Network
        } else if lower.contains("invalid") || lower.contains("400") {
            ErrorType::InvalidRequest
        } else if lower.contains("server") || lower.contains("500") || lower.contains("502") || lower.contains("503") {
            ErrorType::Server
        } else {
            ErrorType::Unknown
        }
    }

    /// Get the currently selected action
    pub fn selected_action(&self) -> Option<RecoveryAction> {
        self.actions.get(self.selected_action).copied()
    }

    /// Select the next action
    pub fn select_next(&mut self) {
        if !self.actions.is_empty() {
            self.selected_action = (self.selected_action + 1) % self.actions.len();
        }
    }

    /// Select the previous action
    pub fn select_previous(&mut self) {
        if !self.actions.is_empty() {
            self.selected_action = if self.selected_action == 0 {
                self.actions.len() - 1
            } else {
                self.selected_action - 1
            };
        }
    }

    /// Select action by keybind
    pub fn select_by_key(&mut self, key: char) -> Option<RecoveryAction> {
        self.actions
            .iter()
            .find(|action| action.keybind().contains(key))
            .copied()
    }

    /// Render the error dialog
    pub fn render(&self, frame: &mut Frame, area: Rect, colors: &ResolvedThemeColors) {
        // Calculate dialog size (60% width, fit content height)
        let dialog_width = area.width * 60 / 100;
        let dialog_height = 12.min(area.height.saturating_sub(4));

        // Center the dialog
        let horizontal_margin = (area.width.saturating_sub(dialog_width)) / 2;
        let vertical_margin = (area.height.saturating_sub(dialog_height)) / 2;

        let dialog_area = Rect {
            x: area.x + horizontal_margin,
            y: area.y + vertical_margin,
            width: dialog_width,
            height: dialog_height,
        };

        // Clear the area behind the dialog
        frame.render_widget(Clear, dialog_area);

        // Create the dialog layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Title
                Constraint::Min(2),    // Message + context
                Constraint::Length(3), // Actions
            ])
            .split(dialog_area);

        // Render title
        self.render_title(frame, chunks[0], colors);

        // Render message and context
        self.render_message(frame, chunks[1], colors);

        // Render actions
        self.render_actions(frame, chunks[2], colors);
    }

    /// Render the dialog title
    fn render_title(&self, frame: &mut Frame, area: Rect, colors: &ResolvedThemeColors) {
        let title = format!(" {} ", self.error_type.display_name());

        let block = ratatui::widgets::Block::default()
            .borders(ratatui::widgets::Borders::TOP | ratatui::widgets::Borders::LEFT | ratatui::widgets::Borders::RIGHT)
            .title(title)
            .title_style(
                Style::default()
                    .fg(ToadTheme::RED)
                    .add_modifier(Modifier::BOLD),
            )
            .border_style(Style::default().fg(ToadTheme::RED));

        frame.render_widget(block, area);
    }

    /// Render the error message
    fn render_message(&self, frame: &mut Frame, area: Rect, colors: &ResolvedThemeColors) {
        let mut lines = Vec::new();

        // Error message
        lines.push(Line::from(vec![
            Span::styled("Error: ", Style::default().fg(ToadTheme::RED).add_modifier(Modifier::BOLD)),
            Span::styled(&self.message, Style::default().fg(colors.foreground())),
        ]));

        // Context (if provided)
        if let Some(ref context) = self.context {
            lines.push(Line::from(""));
            lines.push(Line::from(vec![
                Span::styled("Context: ", Style::default().fg(colors.gray()).add_modifier(Modifier::BOLD)),
                Span::styled(context, Style::default().fg(colors.gray())),
            ]));
        }

        let paragraph = Paragraph::new(lines)
            .wrap(Wrap { trim: false })
            .block(
                ratatui::widgets::Block::default()
                    .borders(ratatui::widgets::Borders::LEFT | ratatui::widgets::Borders::RIGHT)
                    .border_style(Style::default().fg(ToadTheme::RED))
                    .padding(ratatui::widgets::Padding::horizontal(1)),
            );

        frame.render_widget(paragraph, area);
    }

    /// Render recovery actions
    fn render_actions(&self, frame: &mut Frame, area: Rect, colors: &ResolvedThemeColors) {
        let mut action_spans = Vec::new();

        for (i, action) in self.actions.iter().enumerate() {
            if i > 0 {
                action_spans.push(Span::styled("  ", Style::default()));
            }

            let is_selected = i == self.selected_action;
            let style = if is_selected {
                Style::default()
                    .fg(ToadTheme::TOAD_GREEN_BRIGHT)
                    .add_modifier(Modifier::BOLD | Modifier::REVERSED)
            } else {
                Style::default().fg(colors.gray())
            };

            action_spans.push(Span::styled(
                format!("[{}] {}", action.keybind(), action.label()),
                style,
            ));
        }

        let actions_line = Line::from(action_spans);
        let paragraph = Paragraph::new(actions_line)
            .alignment(Alignment::Center)
            .block(
                ratatui::widgets::Block::default()
                    .borders(ratatui::widgets::Borders::ALL)
                    .border_style(Style::default().fg(ToadTheme::RED))
                    .padding(ratatui::widgets::Padding::horizontal(1)),
            );

        frame.render_widget(paragraph, area);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_dialog_new() {
        let dialog = ErrorDialog::new(ErrorType::Network, "Connection failed");
        assert_eq!(dialog.message, "Connection failed");
        assert_eq!(dialog.error_type, ErrorType::Network);
    }

    #[test]
    fn test_error_dialog_with_context() {
        let dialog = ErrorDialog::new(ErrorType::Authentication, "Invalid API key")
            .with_context("While initializing LLM client");

        assert_eq!(dialog.context, Some("While initializing LLM client".to_string()));
    }

    #[test]
    fn test_infer_error_type() {
        assert_eq!(
            ErrorDialog::infer_error_type("Rate limit exceeded"),
            ErrorType::RateLimit
        );
        assert_eq!(
            ErrorDialog::infer_error_type("Invalid API key"),
            ErrorType::Authentication
        );
        assert_eq!(
            ErrorDialog::infer_error_type("Connection timeout"),
            ErrorType::Network
        );
        assert_eq!(
            ErrorDialog::infer_error_type("Server error 500"),
            ErrorType::Server
        );
    }

    #[test]
    fn test_from_error() {
        let dialog = ErrorDialog::from_error("Rate limit exceeded");
        assert_eq!(dialog.error_type, ErrorType::RateLimit);
    }

    #[test]
    fn test_select_next() {
        let mut dialog = ErrorDialog::new(ErrorType::Network, "Error");
        let initial = dialog.selected_action;

        dialog.select_next();
        assert_ne!(dialog.selected_action, initial);
    }

    #[test]
    fn test_select_previous() {
        let mut dialog = ErrorDialog::new(ErrorType::Network, "Error");
        dialog.select_next();
        dialog.select_next();

        dialog.select_previous();
        // Should have decremented
        assert_eq!(dialog.selected_action, 1);
    }

    #[test]
    fn test_select_by_key() {
        let mut dialog = ErrorDialog::new(ErrorType::RateLimit, "Error");
        let action = dialog.select_by_key('m');
        assert_eq!(action, Some(RecoveryAction::SwitchModel));
    }

    #[test]
    fn test_recovery_action_label() {
        assert_eq!(RecoveryAction::Retry.label(), "Retry");
        assert_eq!(RecoveryAction::SwitchModel.label(), "Switch Model");
        assert_eq!(RecoveryAction::CheckConfig.label(), "Check Config");
    }

    #[test]
    fn test_error_type_suggested_actions() {
        let actions = ErrorType::Authentication.suggested_actions();
        assert!(actions.contains(&RecoveryAction::CheckConfig));
    }
}
