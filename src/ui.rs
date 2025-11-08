//! UI rendering module (View in Elm Architecture)
//!
//! This module contains the rendering logic that displays
//! the application state to the terminal.

use crate::app::{App, AppScreen};
use crate::theme::ToadTheme;
use crate::widgets::WelcomeScreen;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

/// Render the application UI (View in Elm Architecture)
///
/// This is a pure function that takes the application state
/// and renders it to the terminal frame.
pub fn render(app: &App, frame: &mut Frame) {
    let area = frame.area();

    match app.screen() {
        AppScreen::Welcome => {
            render_welcome(app, frame, area);
        }
        AppScreen::TrustDialog => {
            render_trust_dialog(app, frame, area);
        }
        AppScreen::Main => {
            render_main(app, frame, area);
        }
    }
}

/// Render the welcome screen
fn render_welcome(_app: &App, frame: &mut Frame, area: Rect) {
    let welcome = WelcomeScreen::new().with_tips(true);
    welcome.render(frame, area);
}

/// Render the trust dialog screen
fn render_trust_dialog(app: &App, frame: &mut Frame, area: Rect) {
    // Render a semi-transparent background
    let background = Block::default().style(Style::default().bg(ToadTheme::BLACK));
    frame.render_widget(background, area);

    // Render the dialog on top
    if let Some(dialog) = app.trust_dialog() {
        dialog.render(frame, area);
    }
}

/// Render the main interface
fn render_main(app: &App, frame: &mut Frame, area: Rect) {
    // Create the main layout following the mockup:
    // 1. System info header (model, plugins, project)
    // 2. Main content area
    // 3. Horizontal separator
    // 4. Input field
    // 5. Keyboard shortcuts bar
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // System info header
            Constraint::Min(0),    // Main content area
            Constraint::Length(1), // Horizontal separator
            Constraint::Length(1), // Input field
            Constraint::Length(1), // Keyboard shortcuts bar
        ])
        .split(area);

    render_system_info(app, frame, chunks[0]);
    render_main_content(app, frame, chunks[1]);
    render_separator(frame, chunks[2]);
    app.input_field().render(frame, chunks[3]);
    render_shortcuts_bar(frame, chunks[4]);
}

/// Render the main content area
fn render_main_content(_app: &App, frame: &mut Frame, area: Rect) {
    let content_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(ToadTheme::BORDER))
        .title("Main Content")
        .title_style(Style::default().fg(ToadTheme::TOAD_GREEN));

    let welcome_text = vec![
        Line::from(vec![
            Span::styled("Welcome to ", Style::default().fg(ToadTheme::FOREGROUND)),
            Span::styled(
                "Toad",
                Style::default()
                    .fg(ToadTheme::TOAD_GREEN)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "An AI-powered coding terminal with semi-autonomous agents",
            Style::default().fg(ToadTheme::GRAY),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("Built with ", Style::default().fg(ToadTheme::FOREGROUND)),
            Span::styled(
                "Rust",
                Style::default()
                    .fg(ToadTheme::TOAD_GREEN)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" + ", Style::default().fg(ToadTheme::FOREGROUND)),
            Span::styled(
                "Ratatui",
                Style::default()
                    .fg(ToadTheme::TOAD_GREEN_BRIGHT)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" + ", Style::default().fg(ToadTheme::FOREGROUND)),
            Span::styled(
                "Crossterm",
                Style::default()
                    .fg(ToadTheme::TOAD_GREEN_DARK)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
    ];

    let paragraph = Paragraph::new(welcome_text)
        .block(content_block)
        .alignment(Alignment::Center);

    frame.render_widget(paragraph, area);
}

/// Render the system info header
fn render_system_info(app: &App, frame: &mut Frame, area: Rect) {
    let info_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(ToadTheme::TOAD_GREEN))
        .style(Style::default().bg(ToadTheme::BLACK));

    // Left side: Model + Runtime
    // Right side: Plugins + Project path
    let model_info = "Sonnet 4.5 · Rust TUI";
    let plugin_info = format!("Active Plugins: {} installed", app.plugin_count());
    let project_path = app.working_directory().to_string_lossy();

    let info_text = vec![
        Line::from(vec![
            Span::styled(
                model_info,
                Style::default()
                    .fg(ToadTheme::TOAD_GREEN_BRIGHT)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("  │  ", Style::default().fg(ToadTheme::DARK_GRAY)),
            Span::styled(&plugin_info, Style::default().fg(ToadTheme::GRAY)),
        ]),
        Line::from(Span::styled(
            format!("Project: {}", project_path),
            Style::default().fg(ToadTheme::GRAY),
        )),
    ];

    let paragraph = Paragraph::new(info_text)
        .block(info_block)
        .alignment(Alignment::Left);

    frame.render_widget(paragraph, area);
}

/// Render horizontal separator
fn render_separator(frame: &mut Frame, area: Rect) {
    let separator = "─".repeat(area.width as usize);
    let separator_line = Line::from(Span::styled(
        separator,
        Style::default().fg(ToadTheme::DARK_GRAY),
    ));
    let separator_paragraph = Paragraph::new(separator_line);
    frame.render_widget(separator_paragraph, area);
}

/// Render keyboard shortcuts bar
fn render_shortcuts_bar(frame: &mut Frame, area: Rect) {
    let shortcuts = [
        ("Ctrl+C", "quit"),
        ("?", "help"),
        ("/", "commands"),
        ("Ctrl+P", "palette"),
        ("Tab", "autocomplete"),
    ];

    let mut spans = Vec::new();
    for (i, (key, desc)) in shortcuts.iter().enumerate() {
        if i > 0 {
            spans.push(Span::styled("  |  ", Style::default().fg(ToadTheme::DARK_GRAY)));
        }
        spans.push(Span::styled(
            *key,
            Style::default()
                .fg(ToadTheme::TOAD_GREEN)
                .add_modifier(Modifier::BOLD),
        ));
        spans.push(Span::styled(" ", Style::default()));
        spans.push(Span::styled(*desc, Style::default().fg(ToadTheme::GRAY)));
    }

    let shortcuts_line = Line::from(spans);
    let shortcuts_paragraph = Paragraph::new(shortcuts_line).alignment(Alignment::Center);
    frame.render_widget(shortcuts_paragraph, area);
}
