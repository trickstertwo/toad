//! UI rendering module (View in Elm Architecture)
//!
//! This module contains the rendering logic that displays
//! the application state to the terminal.

use crate::app::App;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
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

    // Create the main layout: title bar, main area, status bar
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title bar
            Constraint::Min(0),    // Main content area
            Constraint::Length(3), // Status bar
        ])
        .split(area);

    render_title_bar(app, frame, chunks[0]);
    render_main_content(app, frame, chunks[1]);
    render_status_bar(app, frame, chunks[2]);
}

/// Render the title bar
fn render_title_bar(app: &App, frame: &mut Frame, area: Rect) {
    let title_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .style(Style::default().bg(Color::Black));

    let title_text = Paragraph::new(app.title())
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(title_block);

    frame.render_widget(title_text, area);
}

/// Render the main content area
fn render_main_content(_app: &App, frame: &mut Frame, area: Rect) {
    let content_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White))
        .title("Main Content")
        .title_style(Style::default().fg(Color::Green));

    let welcome_text = vec![
        Line::from(vec![
            Span::styled("Welcome to ", Style::default().fg(Color::White)),
            Span::styled(
                "Toad",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from("An AI-powered coding terminal with semi-autonomous agents"),
        Line::from(""),
        Line::from(vec![
            Span::styled("Built with ", Style::default().fg(Color::White)),
            Span::styled(
                "Rust",
                Style::default()
                    .fg(Color::Red)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" + ", Style::default().fg(Color::White)),
            Span::styled(
                "Ratatui",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" + ", Style::default().fg(Color::White)),
            Span::styled(
                "Crossterm",
                Style::default()
                    .fg(Color::Magenta)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
    ];

    let paragraph = Paragraph::new(welcome_text)
        .block(content_block)
        .alignment(Alignment::Center);

    frame.render_widget(paragraph, area);
}

/// Render the status bar
fn render_status_bar(app: &App, frame: &mut Frame, area: Rect) {
    let status_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow))
        .style(Style::default().bg(Color::Black));

    let status_text = Paragraph::new(app.status_message())
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Left)
        .block(status_block);

    frame.render_widget(status_text, area);
}
