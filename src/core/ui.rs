//! UI rendering module (View in Elm Architecture)
//!
//! This module contains the rendering logic that displays
//! the application state to the terminal.

use crate::core::app::App;
use crate::core::app_state::AppScreen;
use crate::ui::theme::ToadTheme;
use crate::ui::widgets::core::welcome_screen::WelcomeScreen;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

/// Render the application UI (View in Elm Architecture)
///
/// This function takes the application state and renders it to the terminal frame.
/// Note: Requires mutable reference for stateful widgets (List, CommandPalette).
pub fn render(app: &mut App, frame: &mut Frame) {
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
        AppScreen::Evaluation => {
            render_evaluation(app, frame, area);
        }
    }
}

/// Render the welcome screen
fn render_welcome(_app: &mut App, frame: &mut Frame, area: Rect) {
    let welcome = WelcomeScreen::new().with_tips(true);
    welcome.render(frame, area);
}

/// Render the trust dialog screen
fn render_trust_dialog(app: &mut App, frame: &mut Frame, area: Rect) {
    // Render a semi-transparent background
    let background = Block::default().style(Style::default().bg(ToadTheme::BLACK));
    frame.render_widget(background, area);

    // Render the dialog on top
    if let Some(dialog) = app.trust_dialog() {
        dialog.render(frame, area);
    }
}

/// Render the main interface
fn render_main(app: &mut App, frame: &mut Frame, area: Rect) {
    // Create the main layout:
    // 1. Main content area
    // 2. Metadata line (path + model info)
    // 3. Horizontal separator
    // 4. Input field
    // 5. Horizontal separator
    // 6. Keyboard shortcuts bar
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),    // Main content area
            Constraint::Length(1), // Metadata line
            Constraint::Length(1), // Horizontal separator
            Constraint::Length(1), // Input field
            Constraint::Length(1), // Horizontal separator
            Constraint::Length(1), // Keyboard shortcuts bar
        ])
        .split(area);

    render_main_content(app, frame, chunks[0]);
    render_metadata_line(app, frame, chunks[1]);
    render_separator(frame, chunks[2]);
    app.input_field().render(frame, chunks[3]);
    render_separator(frame, chunks[4]);
    render_shortcuts_bar(frame, chunks[5]);

    // Render overlays (help and command palette)
    if app.show_help() {
        app.help_screen().render(frame, area);
    } else if app.show_palette() {
        app.command_palette_mut().render(frame, area);
    }
}

/// Render the main content area
fn render_main_content(app: &mut App, frame: &mut Frame, area: Rect) {
    // Render conversation view
    app.conversation_view().render(frame, area);
}

/// Render the metadata line (path on left, model info and tokens on right)
fn render_metadata_line(app: &mut App, frame: &mut Frame, area: Rect) {
    let project_path = app.working_directory().to_string_lossy();
    let model_info = "claude-sonnet-4.5 (1x)";

    // Format token usage and cost
    let token_usage = if app.total_input_tokens > 0 || app.total_output_tokens > 0 {
        let cost_str = if app.total_cost_usd >= 0.01 {
            format!(" ${:.2}", app.total_cost_usd)
        } else if app.total_cost_usd > 0.0 {
            format!(" $<0.01")
        } else {
            String::new()
        };

        format!(
            " | {}↓ {}↑{}",
            app.total_input_tokens, app.total_output_tokens, cost_str
        )
    } else {
        String::new()
    };

    // Calculate spacing to push model info to the right
    let path_len = project_path.len();
    let right_side = format!("{}{}", model_info, token_usage);
    let right_len = right_side.len();
    let total_len = path_len + right_len;
    let padding = if total_len < area.width as usize {
        " ".repeat(area.width as usize - total_len)
    } else {
        " ".to_string()
    };

    let mut spans = vec![
        Span::styled(" ", Style::default()),
        Span::styled(
            project_path.to_string(),
            Style::default().fg(ToadTheme::GRAY),
        ),
        Span::styled(padding, Style::default()),
        Span::styled(model_info, Style::default().fg(ToadTheme::GRAY)),
    ];

    // Add token usage if present
    if !token_usage.is_empty() {
        spans.push(Span::styled(
            token_usage,
            Style::default().fg(ToadTheme::TOAD_GREEN),
        ));
    }

    let metadata_line = Line::from(spans);

    let paragraph = Paragraph::new(metadata_line);
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
        ("Ctrl+c", "Exit"),
        ("Ctrl+r", "Expand recent"),
        ("?", "Help"),
        ("/", "Commands"),
        ("Ctrl+p", "Palette"),
    ];

    let mut spans = vec![Span::styled(" ", Style::default())];
    for (i, (key, desc)) in shortcuts.iter().enumerate() {
        if i > 0 {
            spans.push(Span::styled(" · ", Style::default().fg(ToadTheme::GRAY)));
        }
        spans.push(Span::styled(*key, Style::default().fg(ToadTheme::GRAY)));
        spans.push(Span::styled(" ", Style::default()));
        spans.push(Span::styled(*desc, Style::default().fg(ToadTheme::GRAY)));
    }

    let shortcuts_line = Line::from(spans);
    let shortcuts_paragraph = Paragraph::new(shortcuts_line).alignment(Alignment::Left);
    frame.render_widget(shortcuts_paragraph, area);
}

/// Render the evaluation screen with comprehensive real-time visibility
fn render_evaluation(app: &mut App, frame: &mut Frame, area: Rect) {
    // Get evaluation state
    let eval_state = app.evaluation_state();

    // If no evaluation running or completed, show simple message
    if eval_state.is_none() {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(ToadTheme::TOAD_GREEN))
            .title("Evaluation Center")
            .title_style(
                Style::default()
                    .fg(ToadTheme::TOAD_GREEN)
                    .add_modifier(Modifier::BOLD),
            );
        let inner = block.inner(area);
        frame.render_widget(block, area);

        let msg = Paragraph::new("Starting evaluation...")
            .style(Style::default().fg(ToadTheme::GRAY));
        frame.render_widget(msg, inner);
        return;
    }

    let state = eval_state.unwrap();

    // Create main layout: Header + Content
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header with task info
            Constraint::Min(0),     // Content area
        ])
        .split(area);

    // Render header with task progress
    render_evaluation_header(frame, main_chunks[0], &state);

    // If evaluation has results or error, show completion screen
    if state.results.is_some() || state.error.is_some() {
        render_evaluation_complete(frame, main_chunks[1], &state);
        app.toasts_mut().render(frame, area);
        return;
    }

    // Otherwise show running evaluation with multi-panel layout
    if let Some(progress) = &state.progress {
        render_evaluation_running(frame, main_chunks[1], progress);
    }

    // Render toasts on top
    app.toasts_mut().render(frame, area);
}

/// Render evaluation header with task progress
fn render_evaluation_header(frame: &mut Frame, area: Rect, state: &crate::core::app_state::EvaluationState) {
    if let Some(progress) = &state.progress {
        let header_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(ToadTheme::TOAD_GREEN))
            .title(format!(
                " Evaluation: Task {}/{} - Step {}/{} ",
                progress.current_task,
                progress.total_tasks,
                progress.current_step.unwrap_or(0),
                progress.max_steps.unwrap_or(25)
            ))
            .title_style(
                Style::default()
                    .fg(ToadTheme::TOAD_GREEN)
                    .add_modifier(Modifier::BOLD),
            );

        let inner = header_block.inner(area);
        frame.render_widget(header_block, area);

        // Show task ID and metrics on one line
        let info_line = Line::from(vec![
            Span::styled("Task: ", Style::default().fg(ToadTheme::GRAY)),
            Span::styled(progress.task_id.clone(), Style::default().fg(ToadTheme::WHITE)),
            Span::styled("  │  Tokens: ", Style::default().fg(ToadTheme::GRAY)),
            Span::styled(
                format!("{}", progress.total_tokens),
                Style::default().fg(ToadTheme::BLUE),
            ),
            Span::styled("  Cost: $", Style::default().fg(ToadTheme::GRAY)),
            Span::styled(
                format!("{:.4}", progress.total_cost),
                Style::default().fg(ToadTheme::TOAD_GREEN),
            ),
        ]);

        let info_paragraph = Paragraph::new(info_line);
        frame.render_widget(info_paragraph, inner);
    }
}

/// Render running evaluation with multi-panel layout
fn render_evaluation_running(frame: &mut Frame, area: Rect, progress: &crate::core::event::EvaluationProgress) {
    // Create 3-column layout: Conversation | Tool Log | Metrics
    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(40), // Conversation
            Constraint::Percentage(40), // Tool log + Files
            Constraint::Percentage(20), // Metrics
        ])
        .split(area);

    // Render conversation panel
    render_conversation_panel(frame, columns[0], progress);

    // Split middle column: Tool log (top) + Files (bottom)
    let middle_split = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(70), // Tool log
            Constraint::Percentage(30), // Files modified
        ])
        .split(columns[1]);

    render_tool_log_panel(frame, middle_split[0], progress);
    render_files_panel(frame, middle_split[1], progress);

    // Render metrics panel
    render_metrics_panel(frame, columns[2], progress);
}

/// Render conversation panel showing LLM messages
fn render_conversation_panel(frame: &mut Frame, area: Rect, progress: &crate::core::event::EvaluationProgress) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(ToadTheme::BLUE))
        .title(" Conversation ")
        .title_style(Style::default().fg(ToadTheme::BLUE).add_modifier(Modifier::BOLD));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let mut lines = vec![];

    // Show last N messages (scroll to bottom)
    let max_messages = (inner.height as usize).saturating_sub(2);
    let start_idx = progress.conversation.len().saturating_sub(max_messages);

    for (idx, message) in progress.conversation.iter().skip(start_idx).enumerate() {
        if idx > 0 {
            lines.push(Line::from(""));
        }

        let (prefix, color) = match message.role {
            crate::ai::llm::Role::User => ("▶ User", ToadTheme::TOAD_GREEN),
            crate::ai::llm::Role::Assistant => ("◀ Assistant", ToadTheme::BLUE),
        };

        lines.push(Line::from(Span::styled(
            prefix,
            Style::default().fg(color).add_modifier(Modifier::BOLD),
        )));

        // Show first 200 characters of content (truncate if too long)
        let content = &message.content;
        let truncated_content;
        let display_content = if content.len() > 200 {
            truncated_content = format!("{}...", &content[..200]);
            &truncated_content
        } else {
            content
        };

        for line in display_content.lines().take(3) {
            lines.push(Line::from(Span::styled(
                line.to_string(),
                Style::default().fg(ToadTheme::WHITE),
            )));
        }
    }

    if lines.is_empty() {
        lines.push(Line::from(Span::styled(
            "Waiting for conversation...",
            Style::default().fg(ToadTheme::GRAY),
        )));
    }

    let paragraph = Paragraph::new(lines).wrap(ratatui::widgets::Wrap { trim: false });
    frame.render_widget(paragraph, inner);
}

/// Render tool execution log
fn render_tool_log_panel(frame: &mut Frame, area: Rect, progress: &crate::core::event::EvaluationProgress) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(ToadTheme::YELLOW))
        .title(format!(" Tool Executions ({}) ", progress.tool_executions.len()))
        .title_style(Style::default().fg(ToadTheme::YELLOW).add_modifier(Modifier::BOLD));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let mut lines = vec![];

    // Show last N tool executions
    let max_tools = (inner.height as usize).saturating_sub(2);
    let start_idx = progress.tool_executions.len().saturating_sub(max_tools);

    for (idx, tool) in progress.tool_executions.iter().skip(start_idx).enumerate() {
        if idx > 0 {
            lines.push(Line::from(""));
        }

        let status_icon = if tool.success { "✓" } else { "✗" };
        let status_color = if tool.success { ToadTheme::TOAD_GREEN } else { ToadTheme::RED };

        lines.push(Line::from(vec![
            Span::styled(
                format!("{} ", status_icon),
                Style::default().fg(status_color).add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                tool.tool_name.clone(),
                Style::default().fg(ToadTheme::YELLOW),
            ),
            Span::styled(
                format!(" ({}ms)", tool.duration_ms),
                Style::default().fg(ToadTheme::GRAY),
            ),
        ]));

        // Show output preview (first line only)
        let output_preview = tool.output.lines().next().unwrap_or("");
        if !output_preview.is_empty() {
            let truncated = if output_preview.len() > 60 {
                format!("{}...", &output_preview[..60])
            } else {
                output_preview.to_string()
            };
            lines.push(Line::from(Span::styled(
                format!("  {}", truncated),
                Style::default().fg(ToadTheme::GRAY),
            )));
        }
    }

    if lines.is_empty() {
        lines.push(Line::from(Span::styled(
            "No tools executed yet",
            Style::default().fg(ToadTheme::GRAY),
        )));
    }

    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, inner);
}

/// Render files modified panel
fn render_files_panel(frame: &mut Frame, area: Rect, progress: &crate::core::event::EvaluationProgress) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(ToadTheme::GRAY))
        .title(format!(" Files Modified ({}) ", progress.files_modified.len()))
        .title_style(Style::default().fg(ToadTheme::GRAY));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let mut lines = vec![];

    for file in progress.files_modified.iter().take(inner.height as usize) {
        let file_name = file.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("<unknown>");
        lines.push(Line::from(vec![
            Span::styled("● ", Style::default().fg(ToadTheme::TOAD_GREEN)),
            Span::styled(file_name, Style::default().fg(ToadTheme::WHITE)),
        ]));
    }

    if lines.is_empty() {
        lines.push(Line::from(Span::styled(
            "No files modified yet",
            Style::default().fg(ToadTheme::GRAY),
        )));
    }

    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, inner);
}

/// Render metrics panel with per-step details
fn render_metrics_panel(frame: &mut Frame, area: Rect, progress: &crate::core::event::EvaluationProgress) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(ToadTheme::TOAD_GREEN))
        .title(" Metrics ")
        .title_style(Style::default().fg(ToadTheme::TOAD_GREEN).add_modifier(Modifier::BOLD));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let mut lines = vec![];

    // Current step metrics
    if let Some(step) = progress.current_step {
        lines.push(Line::from(vec![
            Span::styled("Step: ", Style::default().fg(ToadTheme::GRAY)),
            Span::styled(
                format!("{}/{}", step, progress.max_steps.unwrap_or(25)),
                Style::default().fg(ToadTheme::WHITE),
            ),
        ]));
    }

    lines.push(Line::from(""));

    // Token metrics
    lines.push(Line::from(vec![
            Span::styled("Total Tokens:", Style::default().fg(ToadTheme::GRAY)),
        ]));
    lines.push(Line::from(vec![
        Span::styled(
            format!("  {}", progress.total_tokens),
            Style::default().fg(ToadTheme::BLUE),
        ),
    ]));

    if let Some(input_tokens) = progress.step_input_tokens {
        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::styled("Step In/Out:", Style::default().fg(ToadTheme::GRAY)),
        ]));
        lines.push(Line::from(vec![
            Span::styled(
                format!("  {} / {}", input_tokens, progress.step_output_tokens.unwrap_or(0)),
                Style::default().fg(ToadTheme::BLUE),
            ),
        ]));
    }

    if let Some(cache_tokens) = progress.cache_read_tokens {
        if cache_tokens > 0 {
            lines.push(Line::from(vec![
                Span::styled("  Cache: ", Style::default().fg(ToadTheme::GRAY)),
                Span::styled(
                    format!("{}", cache_tokens),
                    Style::default().fg(ToadTheme::TOAD_GREEN),
                ),
            ]));
        }
    }

    // Cost metrics
    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled("Total Cost:", Style::default().fg(ToadTheme::GRAY)),
    ]));
    lines.push(Line::from(vec![
        Span::styled(
            format!("  ${:.4}", progress.total_cost),
            Style::default().fg(ToadTheme::TOAD_GREEN),
        ),
    ]));

    // Latency
    if let Some(duration) = progress.step_duration_ms {
        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::styled("Step Time:", Style::default().fg(ToadTheme::GRAY)),
        ]));
        lines.push(Line::from(vec![
            Span::styled(
                format!("  {}ms", duration),
                Style::default().fg(ToadTheme::YELLOW),
            ),
        ]));
    }

    // Current thinking preview
    if let Some(thinking) = &progress.current_thinking {
        if !thinking.is_empty() {
            lines.push(Line::from(""));
            lines.push(Line::from(vec![
                Span::styled("Thinking:", Style::default().fg(ToadTheme::GRAY)),
            ]));
            let preview = if thinking.len() > 100 {
                format!("{}...", &thinking[..100])
            } else {
                thinking.clone()
            };
            for line in preview.lines().take(3) {
                lines.push(Line::from(vec![
                    Span::styled(
                        format!("  {}", line),
                        Style::default().fg(ToadTheme::WHITE),
                    ),
                ]));
            }
        }
    }

    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, inner);
}

/// Render evaluation completion screen
fn render_evaluation_complete(frame: &mut Frame, area: Rect, state: &crate::core::app_state::EvaluationState) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(ToadTheme::TOAD_GREEN))
        .title(" Evaluation Complete ")
        .title_style(
            Style::default()
                .fg(ToadTheme::TOAD_GREEN)
                .add_modifier(Modifier::BOLD),
        );

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let mut lines = vec![];

    // Show results if complete
    if let Some(results) = &state.results {
        lines.push(Line::from(vec![Span::styled(
            "✓ Evaluation Complete",
            Style::default()
                .fg(ToadTheme::TOAD_GREEN)
                .add_modifier(Modifier::BOLD),
        )]));
        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::styled("Accuracy: ", Style::default().fg(ToadTheme::GRAY)),
            Span::styled(
                format!("{:.1}%", results.accuracy),
                Style::default()
                    .fg(ToadTheme::TOAD_GREEN)
                    .add_modifier(Modifier::BOLD),
            ),
        ]));
        lines.push(Line::from(vec![
            Span::styled("Tasks Solved: ", Style::default().fg(ToadTheme::GRAY)),
            Span::styled(
                format!("{}/{}", results.tasks_solved, results.total_tasks),
                Style::default().fg(ToadTheme::WHITE),
            ),
        ]));
        lines.push(Line::from(vec![
            Span::styled("Avg Cost: ", Style::default().fg(ToadTheme::GRAY)),
            Span::styled(
                format!("${:.4}", results.avg_cost_usd),
                Style::default().fg(ToadTheme::TOAD_GREEN),
            ),
        ]));
        lines.push(Line::from(vec![
            Span::styled("Avg Duration: ", Style::default().fg(ToadTheme::GRAY)),
            Span::styled(
                format!("{}ms", results.avg_duration_ms),
                Style::default().fg(ToadTheme::BLUE),
            ),
        ]));
        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::styled("Press ", Style::default().fg(ToadTheme::GRAY)),
            Span::styled("q", Style::default().fg(ToadTheme::TOAD_GREEN)),
            Span::styled(" or ", Style::default().fg(ToadTheme::GRAY)),
            Span::styled("Esc", Style::default().fg(ToadTheme::TOAD_GREEN)),
            Span::styled(
                " to return to main screen",
                Style::default().fg(ToadTheme::GRAY),
            ),
        ]));
    }

    // Show error if any
    if let Some(error) = &state.error {
        lines.push(Line::from(vec![Span::styled(
            "✗ Evaluation Failed",
            Style::default()
                .fg(ToadTheme::RED)
                .add_modifier(Modifier::BOLD),
        )]));
        lines.push(Line::from(""));
        lines.push(Line::from(vec![Span::styled(
            error.clone(),
            Style::default().fg(ToadTheme::RED),
        )]));
        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::styled("Press ", Style::default().fg(ToadTheme::GRAY)),
            Span::styled("Esc", Style::default().fg(ToadTheme::TOAD_GREEN)),
            Span::styled(
                " to return to main screen",
                Style::default().fg(ToadTheme::GRAY),
            ),
        ]));
    }

    let paragraph = Paragraph::new(lines).alignment(Alignment::Left);
    frame.render_widget(paragraph, inner);
}
