//! UI rendering module (View in Elm Architecture)
//!
//! This module contains the rendering logic that displays
//! the application state to the terminal.

use crate::core::app::App;
use crate::core::app_state::AppScreen;
use crate::ui::theme::{ToadTheme, ResolvedThemeColors};
use crate::ui::widgets::core::welcome_screen::WelcomeScreen;
use crate::ui::widgets::layout::tabbar::TabBar;
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
    // Create colors from theme
    let colors = ResolvedThemeColors::from_manager(app.theme_manager_mut());

    // Render a semi-transparent background
    let background = Block::default().style(Style::default().bg(colors.background()));
    frame.render_widget(background, area);

    // Render the dialog on top
    if let Some(dialog) = app.trust_dialog() {
        dialog.render(frame, area);
    }
}

/// Render the main interface
fn render_main(app: &mut App, frame: &mut Frame, area: Rect) {
    // Create colors from theme once
    let colors = ResolvedThemeColors::from_manager(app.theme_manager_mut());

    // Create the main layout:
    // 1. Tab bar (if multiple tabs exist)
    // 2. Main content area
    // 3. Metadata line (path + model info)
    // 4. Horizontal separator
    // 5. Input field
    // 6. Horizontal separator
    // 7. Keyboard shortcuts bar
    let show_tabbar = app.tabs().count() > 1;
    let constraints = if show_tabbar {
        vec![
            Constraint::Length(1), // Tab bar
            Constraint::Min(0),    // Main content area
            Constraint::Length(1), // Metadata line
            Constraint::Length(1), // Horizontal separator
            Constraint::Length(1), // Input field
            Constraint::Length(1), // Horizontal separator
            Constraint::Length(1), // Keyboard shortcuts bar
        ]
    } else {
        vec![
            Constraint::Min(0),    // Main content area
            Constraint::Length(1), // Metadata line
            Constraint::Length(1), // Horizontal separator
            Constraint::Length(1), // Input field
            Constraint::Length(1), // Horizontal separator
            Constraint::Length(1), // Keyboard shortcuts bar
        ]
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(area);

    if show_tabbar {
        render_tabbar(app, frame, chunks[0]);
        render_main_content(app, frame, chunks[1]);
        render_metadata_line(app, frame, chunks[2], &colors);
        render_separator(frame, chunks[3], &colors);
        app.input_field().render(frame, chunks[4]);
        render_separator(frame, chunks[5], &colors);
        render_shortcuts_bar(frame, chunks[6], &colors);
    } else {
        render_main_content(app, frame, chunks[0]);
        render_metadata_line(app, frame, chunks[1], &colors);
        render_separator(frame, chunks[2], &colors);
        app.input_field().render(frame, chunks[3]);
        render_separator(frame, chunks[4], &colors);
        render_shortcuts_bar(frame, chunks[5], &colors);
    }

    // Render overlays (help, command palette, settings, and config dialog)
    if app.show_help() {
        app.help_screen().render(frame, area, &colors);
    } else if app.show_palette() {
        app.command_palette_mut().render(frame, area);
    } else if app.show_settings() {
        let current_theme = app.theme_manager_mut().current_theme_name();
        let vim_mode = app.vim_mode();
        app.settings_screen_mut().render(frame, area, current_theme, vim_mode, &colors);
    } else if let Some((milestone, ref config)) = app.show_config_dialog {
        render_config_dialog(frame, area, milestone, config, &colors);
    }
}

/// Render the main content area
fn render_main_content(app: &mut App, frame: &mut Frame, area: Rect) {
    // Check if tool status panel should be shown
    let show_tool_panel = app.tool_status_panel.execution_count() > 0;

    if show_tool_panel {
        // Split horizontally: conversation on left, tool status on right
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(70), // Conversation view
                Constraint::Percentage(30), // Tool status panel
            ])
            .split(area);

        // Render conversation view
        app.conversation_view().render(frame, chunks[0]);

        // Render tool status panel
        app.tool_status_panel.render(frame, chunks[1]);
    } else {
        // No tools executed yet, show full-width conversation
        app.conversation_view().render(frame, area);
    }
}

/// Render the metadata line (path on left, model info and tokens on right)
fn render_metadata_line(app: &mut App, frame: &mut Frame, area: Rect, colors: &ResolvedThemeColors) {
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
            Style::default().fg(colors.gray()),
        ),
        Span::styled(padding, Style::default()),
        Span::styled(model_info, Style::default().fg(colors.gray())),
    ];

    // Add token usage if present
    if !token_usage.is_empty() {
        spans.push(Span::styled(
            token_usage,
            Style::default().fg(colors.accent()),
        ));
    }

    let metadata_line = Line::from(spans);

    let paragraph = Paragraph::new(metadata_line);
    frame.render_widget(paragraph, area);
}

/// Render horizontal separator
fn render_separator(frame: &mut Frame, area: Rect, colors: &ResolvedThemeColors) {
    let separator = "─".repeat(area.width as usize);
    let separator_line = Line::from(Span::styled(
        separator,
        Style::default().fg(colors.dark_gray()),
    ));
    let separator_paragraph = Paragraph::new(separator_line);
    frame.render_widget(separator_paragraph, area);
}

/// Render the tab bar
fn render_tabbar(app: &App, frame: &mut Frame, area: Rect) {
    let tabbar = TabBar::new(app.tabs());
    tabbar.render(frame, area);
}

/// Render keyboard shortcuts bar
fn render_shortcuts_bar(frame: &mut Frame, area: Rect, colors: &ResolvedThemeColors) {
    let shortcuts = [
        ("Ctrl+P", "Palette"),
        ("Ctrl+L", "Clear"),
        ("Ctrl+Shift+C", "Copy"),
        ("Shift+Enter", "Newline"),
        ("F9", "Eval"),
        ("F10", "Settings"),
        ("?", "Help"),
    ];

    let mut spans = vec![Span::styled(" ", Style::default())];
    for (i, (key, desc)) in shortcuts.iter().enumerate() {
        if i > 0 {
            spans.push(Span::styled(" · ", Style::default().fg(colors.gray())));
        }
        spans.push(Span::styled(*key, Style::default().fg(colors.gray())));
        spans.push(Span::styled(" ", Style::default()));
        spans.push(Span::styled(*desc, Style::default().fg(colors.gray())));
    }

    let shortcuts_line = Line::from(spans);
    let shortcuts_paragraph = Paragraph::new(shortcuts_line).alignment(Alignment::Left);
    frame.render_widget(shortcuts_paragraph, area);
}

/// Render the evaluation screen with comprehensive real-time visibility
fn render_evaluation(app: &mut App, frame: &mut Frame, area: Rect) {
    // Create colors from theme
    let colors = ResolvedThemeColors::from_manager(app.theme_manager_mut());

    // Get evaluation state
    let eval_state = app.evaluation_state();

    // If no evaluation running or completed, show simple message
    if eval_state.is_none() {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(colors.accent()))
            .title("Evaluation Center")
            .title_style(
                Style::default()
                    .fg(colors.accent())
                    .add_modifier(Modifier::BOLD),
            );
        let inner = block.inner(area);
        frame.render_widget(block, area);

        let msg = Paragraph::new("Starting evaluation...")
            .style(Style::default().fg(colors.gray()));
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
    render_evaluation_header(frame, main_chunks[0], &state, &colors);

    // If evaluation has results or error, show completion screen
    if state.results.is_some() || state.error.is_some() {
        render_evaluation_complete(frame, main_chunks[1], &state, &colors);
        app.toasts_mut().render(frame, area);
        return;
    }

    // Otherwise show running evaluation with multi-panel layout
    if let Some(progress) = &state.progress {
        render_evaluation_running(frame, main_chunks[1], progress, &colors);
    }

    // Render toasts on top
    app.toasts_mut().render(frame, area);
}

/// Render evaluation header with task progress
fn render_evaluation_header(frame: &mut Frame, area: Rect, state: &crate::core::app_state::EvaluationState, colors: &ResolvedThemeColors) {
    if let Some(progress) = &state.progress {
        let header_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(colors.accent()))
            .title(format!(
                " Evaluation: Task {}/{} - Step {}/{} ",
                progress.current_task,
                progress.total_tasks,
                progress.current_step.unwrap_or(0),
                progress.max_steps.unwrap_or(25)
            ))
            .title_style(
                Style::default()
                    .fg(colors.accent())
                    .add_modifier(Modifier::BOLD),
            );

        let inner = header_block.inner(area);
        frame.render_widget(header_block, area);

        // Show task ID and metrics on one line
        let info_line = Line::from(vec![
            Span::styled("Task: ", Style::default().fg(colors.gray())),
            Span::styled(progress.task_id.clone(), Style::default().fg(colors.foreground())),
            Span::styled("  │  Tokens: ", Style::default().fg(colors.gray())),
            Span::styled(
                format!("{}", progress.total_tokens),
                Style::default().fg(colors.info()),
            ),
            Span::styled("  Cost: $", Style::default().fg(colors.gray())),
            Span::styled(
                format!("{:.4}", progress.total_cost),
                Style::default().fg(colors.accent()),
            ),
        ]);

        let info_paragraph = Paragraph::new(info_line);
        frame.render_widget(info_paragraph, inner);
    }
}

/// Render running evaluation with multi-panel layout
fn render_evaluation_running(frame: &mut Frame, area: Rect, progress: &crate::core::event::EvaluationProgress, colors: &ResolvedThemeColors) {
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
    render_conversation_panel(frame, columns[0], progress, colors);

    // Split middle column: Tool log (top) + Files (bottom)
    let middle_split = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(70), // Tool log
            Constraint::Percentage(30), // Files modified
        ])
        .split(columns[1]);

    render_tool_log_panel(frame, middle_split[0], progress, colors);
    render_files_panel(frame, middle_split[1], progress, colors);

    // Render metrics panel
    render_metrics_panel(frame, columns[2], progress, colors);
}

/// Render conversation panel showing LLM messages
fn render_conversation_panel(frame: &mut Frame, area: Rect, progress: &crate::core::event::EvaluationProgress, colors: &ResolvedThemeColors) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(colors.info()))
        .title(" Conversation ")
        .title_style(Style::default().fg(colors.info()).add_modifier(Modifier::BOLD));

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
            crate::ai::llm::Role::User => ("▶ User", colors.accent()),
            crate::ai::llm::Role::Assistant => ("◀ Assistant", colors.info()),
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
                Style::default().fg(colors.foreground()),
            )));
        }
    }

    if lines.is_empty() {
        lines.push(Line::from(Span::styled(
            "Waiting for conversation...",
            Style::default().fg(colors.gray()),
        )));
    }

    let paragraph = Paragraph::new(lines).wrap(ratatui::widgets::Wrap { trim: false });
    frame.render_widget(paragraph, inner);
}

/// Render tool execution log
fn render_tool_log_panel(frame: &mut Frame, area: Rect, progress: &crate::core::event::EvaluationProgress, colors: &ResolvedThemeColors) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(colors.warning()))
        .title(format!(" Tool Executions ({}) ", progress.tool_executions.len()))
        .title_style(Style::default().fg(colors.warning()).add_modifier(Modifier::BOLD));

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
        let status_color = if tool.success { colors.success() } else { colors.error() };

        lines.push(Line::from(vec![
            Span::styled(
                format!("{} ", status_icon),
                Style::default().fg(status_color).add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                tool.tool_name.clone(),
                Style::default().fg(colors.warning()),
            ),
            Span::styled(
                format!(" ({}ms)", tool.duration_ms),
                Style::default().fg(colors.gray()),
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
                Style::default().fg(colors.gray()),
            )));
        }
    }

    if lines.is_empty() {
        lines.push(Line::from(Span::styled(
            "No tools executed yet",
            Style::default().fg(colors.gray()),
        )));
    }

    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, inner);
}

/// Render files modified panel
fn render_files_panel(frame: &mut Frame, area: Rect, progress: &crate::core::event::EvaluationProgress, colors: &ResolvedThemeColors) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(colors.gray()))
        .title(format!(" Files Modified ({}) ", progress.files_modified.len()))
        .title_style(Style::default().fg(colors.gray()));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let mut lines = vec![];

    for file in progress.files_modified.iter().take(inner.height as usize) {
        let file_name = file.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("<unknown>");
        lines.push(Line::from(vec![
            Span::styled("● ", Style::default().fg(colors.success())),
            Span::styled(file_name, Style::default().fg(colors.foreground())),
        ]));
    }

    if lines.is_empty() {
        lines.push(Line::from(Span::styled(
            "No files modified yet",
            Style::default().fg(colors.gray()),
        )));
    }

    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, inner);
}

/// Render metrics panel with per-step details
fn render_metrics_panel(frame: &mut Frame, area: Rect, progress: &crate::core::event::EvaluationProgress, colors: &ResolvedThemeColors) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(colors.accent()))
        .title(" Metrics ")
        .title_style(Style::default().fg(colors.accent()).add_modifier(Modifier::BOLD));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let mut lines = vec![];

    // Current step metrics
    if let Some(step) = progress.current_step {
        lines.push(Line::from(vec![
            Span::styled("Step: ", Style::default().fg(colors.gray())),
            Span::styled(
                format!("{}/{}", step, progress.max_steps.unwrap_or(25)),
                Style::default().fg(colors.foreground()),
            ),
        ]));
    }

    lines.push(Line::from(""));

    // Token metrics
    lines.push(Line::from(vec![
            Span::styled("Total Tokens:", Style::default().fg(colors.gray())),
        ]));
    lines.push(Line::from(vec![
        Span::styled(
            format!("  {}", progress.total_tokens),
            Style::default().fg(colors.info()),
        ),
    ]));

    if let Some(input_tokens) = progress.step_input_tokens {
        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::styled("Step In/Out:", Style::default().fg(colors.gray())),
        ]));
        lines.push(Line::from(vec![
            Span::styled(
                format!("  {} / {}", input_tokens, progress.step_output_tokens.unwrap_or(0)),
                Style::default().fg(colors.info()),
            ),
        ]));
    }

    if let Some(cache_tokens) = progress.cache_read_tokens {
        if cache_tokens > 0 {
            lines.push(Line::from(vec![
                Span::styled("  Cache: ", Style::default().fg(colors.gray())),
                Span::styled(
                    format!("{}", cache_tokens),
                    Style::default().fg(colors.success()),
                ),
            ]));
        }
    }

    // Cost metrics
    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled("Total Cost:", Style::default().fg(colors.gray())),
    ]));
    lines.push(Line::from(vec![
        Span::styled(
            format!("  ${:.4}", progress.total_cost),
            Style::default().fg(colors.accent()),
        ),
    ]));

    // Latency
    if let Some(duration) = progress.step_duration_ms {
        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::styled("Step Time:", Style::default().fg(colors.gray())),
        ]));
        lines.push(Line::from(vec![
            Span::styled(
                format!("  {}ms", duration),
                Style::default().fg(colors.warning()),
            ),
        ]));
    }

    // Current thinking preview
    if let Some(thinking) = &progress.current_thinking {
        if !thinking.is_empty() {
            lines.push(Line::from(""));
            lines.push(Line::from(vec![
                Span::styled("Thinking:", Style::default().fg(colors.gray())),
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
                        Style::default().fg(colors.foreground()),
                    ),
                ]));
            }
        }
    }

    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, inner);
}

/// Render evaluation completion screen
fn render_evaluation_complete(frame: &mut Frame, area: Rect, state: &crate::core::app_state::EvaluationState, colors: &ResolvedThemeColors) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(colors.accent()))
        .title(" Evaluation Complete ")
        .title_style(
            Style::default()
                .fg(colors.accent())
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
                .fg(colors.success())
                .add_modifier(Modifier::BOLD),
        )]));
        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::styled("Accuracy: ", Style::default().fg(colors.gray())),
            Span::styled(
                format!("{:.1}%", results.accuracy),
                Style::default()
                    .fg(colors.success())
                    .add_modifier(Modifier::BOLD),
            ),
        ]));
        lines.push(Line::from(vec![
            Span::styled("Tasks Solved: ", Style::default().fg(colors.gray())),
            Span::styled(
                format!("{}/{}", results.tasks_solved, results.total_tasks),
                Style::default().fg(colors.foreground()),
            ),
        ]));
        lines.push(Line::from(vec![
            Span::styled("Avg Cost: ", Style::default().fg(colors.gray())),
            Span::styled(
                format!("${:.4}", results.avg_cost_usd),
                Style::default().fg(colors.accent()),
            ),
        ]));
        lines.push(Line::from(vec![
            Span::styled("Avg Duration: ", Style::default().fg(colors.gray())),
            Span::styled(
                format!("{}ms", results.avg_duration_ms),
                Style::default().fg(colors.info()),
            ),
        ]));
        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::styled("Press ", Style::default().fg(colors.gray())),
            Span::styled("q", Style::default().fg(colors.accent())),
            Span::styled(" or ", Style::default().fg(colors.gray())),
            Span::styled("Esc", Style::default().fg(colors.accent())),
            Span::styled(
                " to return to main screen",
                Style::default().fg(colors.gray()),
            ),
        ]));
    }

    // Show error if any
    if let Some(error) = &state.error {
        lines.push(Line::from(vec![Span::styled(
            "✗ Evaluation Failed",
            Style::default()
                .fg(colors.error())
                .add_modifier(Modifier::BOLD),
        )]));
        lines.push(Line::from(""));
        lines.push(Line::from(vec![Span::styled(
            error.clone(),
            Style::default().fg(colors.error()),
        )]));
        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::styled("Press ", Style::default().fg(colors.gray())),
            Span::styled("Esc", Style::default().fg(colors.accent())),
            Span::styled(
                " to return to main screen",
                Style::default().fg(colors.gray()),
            ),
        ]));
    }

    let paragraph = Paragraph::new(lines).alignment(Alignment::Left);
    frame.render_widget(paragraph, inner);
}

/// Render the config dialog showing milestone configuration
fn render_config_dialog(
    frame: &mut Frame,
    area: Rect,
    milestone: usize,
    config: &crate::config::ToadConfig,
    colors: &ResolvedThemeColors,
) {
    use ratatui::widgets::{Borders, Clear, Wrap};

    // Create a centered dialog box (60% width, 80% height)
    let dialog_width = (area.width as f32 * 0.6) as u16;
    let dialog_height = (area.height as f32 * 0.8) as u16;
    let dialog_x = (area.width.saturating_sub(dialog_width)) / 2;
    let dialog_y = (area.height.saturating_sub(dialog_height)) / 2;

    let dialog_area = Rect {
        x: area.x + dialog_x,
        y: area.y + dialog_y,
        width: dialog_width,
        height: dialog_height,
    };

    // Clear the area and render background
    frame.render_widget(Clear, dialog_area);

    // Create dialog title
    let title = format!("Milestone {} Configuration", milestone);
    let block = ratatui::widgets::Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(colors.accent()))
        .style(Style::default().bg(colors.background()));

    let inner = block.inner(dialog_area);
    frame.render_widget(block, dialog_area);

    // Build content showing enabled features
    let mut lines = vec![];
    lines.push(Line::from(vec![
        Span::styled("Feature Configuration:", Style::default().fg(colors.foreground()).add_modifier(Modifier::BOLD)),
    ]));
    lines.push(Line::from(""));

    // Context Strategies
    lines.push(Line::from(vec![
        Span::styled("Context Strategies:", Style::default().fg(colors.accent()).add_modifier(Modifier::UNDERLINED)),
    ]));
    lines.push(Line::from(vec![
        Span::raw("  "),
        Span::styled(if config.features.context_ast { "✓" } else { "✗" }, 
            Style::default().fg(if config.features.context_ast { colors.success() } else { colors.gray() })),
        Span::raw(" AST-based context"),
    ]));
    lines.push(Line::from(vec![
        Span::raw("  "),
        Span::styled(if config.features.context_embeddings { "✓" } else { "✗" }, 
            Style::default().fg(if config.features.context_embeddings { colors.success() } else { colors.gray() })),
        Span::raw(" Vector embeddings"),
    ]));
    lines.push(Line::from(vec![
        Span::raw("  "),
        Span::styled(if config.features.context_graph { "✓" } else { "✗" }, 
            Style::default().fg(if config.features.context_graph { colors.success() } else { colors.gray() })),
        Span::raw(" Code graph analysis"),
    ]));
    lines.push(Line::from(vec![
        Span::raw("  "),
        Span::styled(if config.features.context_reranking { "✓" } else { "✗" }, 
            Style::default().fg(if config.features.context_reranking { colors.success() } else { colors.gray() })),
        Span::raw(" Context re-ranking"),
    ]));
    lines.push(Line::from(""));

    // Routing Strategies
    lines.push(Line::from(vec![
        Span::styled("Routing Strategies:", Style::default().fg(colors.accent()).add_modifier(Modifier::UNDERLINED)),
    ]));
    lines.push(Line::from(vec![
        Span::raw("  "),
        Span::styled(if config.features.routing_semantic { "✓" } else { "✗" }, 
            Style::default().fg(if config.features.routing_semantic { colors.success() } else { colors.gray() })),
        Span::raw(" Semantic router"),
    ]));
    lines.push(Line::from(vec![
        Span::raw("  "),
        Span::styled(if config.features.routing_multi_model { "✓" } else { "✗" }, 
            Style::default().fg(if config.features.routing_multi_model { colors.success() } else { colors.gray() })),
        Span::raw(" Multi-model ensemble"),
    ]));
    lines.push(Line::from(vec![
        Span::raw("  "),
        Span::styled(if config.features.routing_cascade { "✓" } else { "✗" }, 
            Style::default().fg(if config.features.routing_cascade { colors.success() } else { colors.gray() })),
        Span::raw(" Cascading routing"),
    ]));
    lines.push(Line::from(vec![
        Span::raw("  "),
        Span::styled(if config.features.routing_speculative { "✓" } else { "✗" }, 
            Style::default().fg(if config.features.routing_speculative { colors.success() } else { colors.gray() })),
        Span::raw(" Speculative execution"),
    ]));
    lines.push(Line::from(""));

    // Intelligence Features
    lines.push(Line::from(vec![
        Span::styled("Intelligence Features:", Style::default().fg(colors.accent()).add_modifier(Modifier::UNDERLINED)),
    ]));
    lines.push(Line::from(vec![
        Span::raw("  "),
        Span::styled(if config.features.smart_test_selection { "✓" } else { "✗" }, 
            Style::default().fg(if config.features.smart_test_selection { colors.success() } else { colors.gray() })),
        Span::raw(" Smart test selection"),
    ]));
    lines.push(Line::from(vec![
        Span::raw("  "),
        Span::styled(if config.features.failure_memory { "✓" } else { "✗" }, 
            Style::default().fg(if config.features.failure_memory { colors.success() } else { colors.gray() })),
        Span::raw(" Failure memory"),
    ]));
    lines.push(Line::from(vec![
        Span::raw("  "),
        Span::styled(if config.features.opportunistic_planning { "✓" } else { "✗" }, 
            Style::default().fg(if config.features.opportunistic_planning { colors.success() } else { colors.gray() })),
        Span::raw(" Opportunistic planning"),
    ]));
    lines.push(Line::from(""));

    // Optimizations
    lines.push(Line::from(vec![
        Span::styled("Optimizations:", Style::default().fg(colors.accent()).add_modifier(Modifier::UNDERLINED)),
    ]));
    lines.push(Line::from(vec![
        Span::raw("  "),
        Span::styled(if config.features.prompt_caching { "✓" } else { "✗" }, 
            Style::default().fg(if config.features.prompt_caching { colors.success() } else { colors.gray() })),
        Span::raw(" Prompt caching"),
    ]));
    lines.push(Line::from(vec![
        Span::raw("  "),
        Span::styled(if config.features.semantic_caching { "✓" } else { "✗" }, 
            Style::default().fg(if config.features.semantic_caching { colors.success() } else { colors.gray() })),
        Span::raw(" Semantic caching"),
    ]));
    lines.push(Line::from(vec![
        Span::raw("  "),
        Span::styled(if config.features.tree_sitter_validation { "✓" } else { "✗" }, 
            Style::default().fg(if config.features.tree_sitter_validation { colors.success() } else { colors.gray() })),
        Span::raw(" Tree-sitter validation"),
    ]));
    lines.push(Line::from(""));

    // Summary
    lines.push(Line::from(vec![
        Span::styled("Total enabled: ", Style::default().fg(colors.foreground()).add_modifier(Modifier::BOLD)),
        Span::styled(
            format!("{}", config.features.enabled_count()),
            Style::default().fg(colors.success()).add_modifier(Modifier::BOLD)
        ),
    ]));
    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled("Press ", Style::default().fg(colors.gray())),
        Span::styled("Esc", Style::default().fg(colors.accent()).add_modifier(Modifier::BOLD)),
        Span::styled(" or ", Style::default().fg(colors.gray())),
        Span::styled("Enter", Style::default().fg(colors.accent()).add_modifier(Modifier::BOLD)),
        Span::styled(" to close", Style::default().fg(colors.gray())),
    ]));

    // Render content
    let paragraph = Paragraph::new(lines)
        .wrap(Wrap { trim: false })
        .alignment(Alignment::Left);

    frame.render_widget(paragraph, inner);
}
