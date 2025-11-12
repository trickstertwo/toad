//! Approval dialog widget for dangerous operations
//!
//! Displays operation details and requests user approval before execution.
//!
//! # Features
//!
//! - Risk level visualization
//! - File diff preview for writes
//! - Command details for bash operations
//! - Keyboard shortcuts (y/n/e/d)
//! - Trust mode toggle
//!
//! # Examples
//!
//! ```no_run
//! use toad::ui::widgets::core::approval_dialog::ApprovalDialog;
//! use toad::core::app_approvals::ApprovalRequest;
//! use std::path::PathBuf;
//!
//! // let request = ApprovalRequest::WriteFile { ... };
//! // let dialog = ApprovalDialog::new(request);
//! ```

use crate::core::app_approvals::{ApprovalRequest, RiskLevel};
use crate::ui::theme::{ResolvedThemeColors, ToadTheme};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

/// Approval dialog widget
#[derive(Debug)]
pub struct ApprovalDialog {
    /// The request being displayed
    request: ApprovalRequest,
    /// Whether to show full details
    show_details: bool,
}

impl ApprovalDialog {
    /// Create a new approval dialog
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use toad::ui::widgets::core::approval_dialog::ApprovalDialog;
    /// use toad::core::app_approvals::ApprovalRequest;
    /// use std::path::PathBuf;
    ///
    /// // let request = ApprovalRequest::BashCommand { ... };
    /// // let dialog = ApprovalDialog::new(request);
    /// ```
    pub fn new(request: ApprovalRequest) -> Self {
        Self {
            request,
            show_details: false,
        }
    }

    /// Toggle detail view
    pub fn toggle_details(&mut self) {
        self.show_details = !self.show_details;
    }

    /// Get the current request
    pub fn request(&self) -> &ApprovalRequest {
        &self.request
    }

    /// Render the approval dialog
    pub fn render(&self, frame: &mut Frame, area: Rect, colors: &ResolvedThemeColors) {
        // Calculate dialog size based on content
        let dialog_width = (area.width * 70 / 100).max(60);
        let dialog_height = if self.show_details {
            (area.height * 80 / 100).max(20)
        } else {
            12.min(area.height.saturating_sub(4))
        };

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
                Constraint::Length(3),     // Title with risk level
                Constraint::Min(2),        // Operation details
                Constraint::Length(1),     // Separator
                Constraint::Length(3),     // Actions
            ])
            .split(dialog_area);

        // Render components
        self.render_title(frame, chunks[0], colors);
        self.render_details(frame, chunks[1], colors);
        self.render_separator(frame, chunks[2], colors);
        self.render_actions(frame, chunks[3], colors);
    }

    /// Render the dialog title with risk indicator
    fn render_title(&self, frame: &mut Frame, area: Rect, _colors: &ResolvedThemeColors) {
        let risk = self.request.risk();
        let risk_color = risk.color();

        let title = format!(" Approval Required - Risk: {} ", risk.label());

        let block = Block::default()
            .borders(Borders::TOP | Borders::LEFT | Borders::RIGHT)
            .title(title)
            .title_style(
                Style::default()
                    .fg(risk_color)
                    .add_modifier(Modifier::BOLD),
            )
            .border_style(Style::default().fg(risk_color));

        frame.render_widget(block, area);
    }

    /// Render operation details
    fn render_details(&self, frame: &mut Frame, area: Rect, colors: &ResolvedThemeColors) {
        let mut lines = Vec::new();

        // Operation summary
        lines.push(Line::from(vec![
            Span::styled("Operation: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::styled(self.request.summary(), Style::default().fg(colors.foreground())),
        ]));
        lines.push(Line::from(""));

        // Operation-specific details
        match &self.request {
            ApprovalRequest::WriteFile {
                path,
                content,
                is_new,
                previous_content,
                ..
            } => {
                if *is_new {
                    lines.push(Line::from(Span::styled(
                        "This will create a new file.",
                        Style::default().fg(colors.info()),
                    )));
                } else {
                    lines.push(Line::from(Span::styled(
                        "This will modify an existing file.",
                        Style::default().fg(colors.warning()),
                    )));
                }
                lines.push(Line::from(""));

                // Show file path
                lines.push(Line::from(vec![
                    Span::styled("File: ", Style::default().fg(colors.gray())),
                    Span::styled(
                        path.display().to_string(),
                        Style::default().fg(colors.foreground()),
                    ),
                ]));

                // Show content preview (first few lines)
                if self.show_details {
                    lines.push(Line::from(""));
                    lines.push(Line::from(Span::styled(
                        "Content preview:",
                        Style::default().fg(colors.gray()).add_modifier(Modifier::BOLD),
                    )));

                    for line in content.lines().take(10) {
                        lines.push(Line::from(Span::styled(
                            format!("  {}", line),
                            Style::default().fg(colors.foreground()),
                        )));
                    }

                    if content.lines().count() > 10 {
                        lines.push(Line::from(Span::styled(
                            "  ... (more lines)",
                            Style::default().fg(colors.gray()),
                        )));
                    }
                } else if let Some(prev) = previous_content {
                    // Show diff stats if not in detail mode
                    let line_count = content.lines().count();
                    let prev_line_count = prev.lines().count();
                    let diff = line_count as i32 - prev_line_count as i32;

                    lines.push(Line::from(vec![
                        Span::styled("Changes: ", Style::default().fg(colors.gray())),
                        Span::styled(
                            format!("{} lines ", line_count),
                            Style::default().fg(colors.foreground()),
                        ),
                        if diff >= 0 {
                            Span::styled(
                                format!("(+{})", diff),
                                Style::default().fg(ToadTheme::TOAD_GREEN),
                            )
                        } else {
                            Span::styled(
                                format!("({})", diff),
                                Style::default().fg(ToadTheme::RED),
                            )
                        },
                    ]));
                }
            }
            ApprovalRequest::BashCommand {
                command,
                working_dir,
                ..
            } => {
                if self.request.is_destructive() {
                    lines.push(Line::from(Span::styled(
                        "⚠ WARNING: This command is potentially destructive!",
                        Style::default()
                            .fg(ToadTheme::RED)
                            .add_modifier(Modifier::BOLD),
                    )));
                    lines.push(Line::from(""));
                }

                lines.push(Line::from(vec![
                    Span::styled("Command: ", Style::default().fg(colors.gray())),
                    Span::styled(command, Style::default().fg(colors.foreground())),
                ]));

                lines.push(Line::from(vec![
                    Span::styled("Directory: ", Style::default().fg(colors.gray())),
                    Span::styled(
                        working_dir.display().to_string(),
                        Style::default().fg(colors.foreground()),
                    ),
                ]));
            }
            ApprovalRequest::GitCommit { message, files } => {
                lines.push(Line::from(vec![
                    Span::styled("Commit: ", Style::default().fg(colors.gray())),
                    Span::styled(
                        message.lines().next().unwrap_or(""),
                        Style::default().fg(colors.foreground()),
                    ),
                ]));

                lines.push(Line::from(vec![
                    Span::styled("Files: ", Style::default().fg(colors.gray())),
                    Span::styled(
                        format!("{}", files.len()),
                        Style::default().fg(colors.foreground()),
                    ),
                ]));

                if self.show_details {
                    lines.push(Line::from(""));
                    for file in files.iter().take(10) {
                        lines.push(Line::from(Span::styled(
                            format!("  - {}", file.display()),
                            Style::default().fg(colors.foreground()),
                        )));
                    }
                    if files.len() > 10 {
                        lines.push(Line::from(Span::styled(
                            format!("  ... and {} more", files.len() - 10),
                            Style::default().fg(colors.gray()),
                        )));
                    }
                }
            }
        }

        let paragraph = Paragraph::new(lines)
            .wrap(Wrap { trim: false })
            .block(
                Block::default()
                    .borders(Borders::LEFT | Borders::RIGHT)
                    .border_style(Style::default().fg(self.request.risk().color()))
                    .padding(ratatui::widgets::Padding::horizontal(1)),
            );

        frame.render_widget(paragraph, area);
    }

    /// Render separator
    fn render_separator(&self, frame: &mut Frame, area: Rect, _colors: &ResolvedThemeColors) {
        let separator = Block::default()
            .borders(Borders::LEFT | Borders::RIGHT)
            .border_style(Style::default().fg(self.request.risk().color()));

        frame.render_widget(separator, area);
    }

    /// Render action buttons
    fn render_actions(&self, frame: &mut Frame, area: Rect, colors: &ResolvedThemeColors) {
        let actions = vec![
            ("y", "Approve", ToadTheme::TOAD_GREEN),
            ("n", "Reject", ToadTheme::RED),
            ("d", "Details", colors.info()),
            ("Esc", "Cancel", colors.gray()),
        ];

        let mut action_spans = Vec::new();

        for (i, (key, label, color)) in actions.iter().enumerate() {
            if i > 0 {
                action_spans.push(Span::styled("  ", Style::default()));
            }

            action_spans.push(Span::styled(
                format!("[{}] ", key),
                Style::default().fg(*color).add_modifier(Modifier::BOLD),
            ));
            action_spans.push(Span::styled(*label, Style::default().fg(colors.foreground())));
        }

        let actions_line = Line::from(action_spans);
        let paragraph = Paragraph::new(actions_line)
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(self.request.risk().color()))
                    .padding(ratatui::widgets::Padding::horizontal(1)),
            );

        frame.render_widget(paragraph, area);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_approval_dialog_new() {
        let request = ApprovalRequest::BashCommand {
            command: "test".to_string(),
            working_dir: PathBuf::from("/tmp"),
            risk: RiskLevel::Low,
        };

        let dialog = ApprovalDialog::new(request);
        assert!(!dialog.show_details);
    }

    #[test]
    fn test_toggle_details() {
        let request = ApprovalRequest::BashCommand {
            command: "test".to_string(),
            working_dir: PathBuf::from("/tmp"),
            risk: RiskLevel::Low,
        };

        let mut dialog = ApprovalDialog::new(request);
        assert!(!dialog.show_details);

        dialog.toggle_details();
        assert!(dialog.show_details);

        dialog.toggle_details();
        assert!(!dialog.show_details);
    }

    #[test]
    fn test_request_access() {
        let request = ApprovalRequest::BashCommand {
            command: "test".to_string(),
            working_dir: PathBuf::from("/tmp"),
            risk: RiskLevel::Low,
        };

        let dialog = ApprovalDialog::new(request.clone());
        assert_eq!(dialog.request().risk(), RiskLevel::Low);
    }
}
