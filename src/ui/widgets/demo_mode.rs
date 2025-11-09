//! Demo mode for showcasing TOAD features
//!
//! Provides an interactive demonstration that automatically showcases
//! various features of TOAD with narration and automatic progression.
//!
//! # Examples
//!
//! ```
//! use toad::ui::widgets::DemoMode;
//!
//! let mut demo = DemoMode::new();
//! demo.start();
//! assert!(demo.is_running());
//! ```

use std::time::{Duration, Instant};

use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Widget, Wrap},
};

/// Demo step with narration
#[derive(Debug, Clone)]
pub struct DemoStep {
    /// Step title
    pub title: String,
    /// Narration text
    pub narration: String,
    /// Duration to show this step (ms)
    pub duration_ms: u64,
    /// Feature being demonstrated
    pub feature: String,
    /// Optional code example
    pub code_example: Option<String>,
}

impl DemoStep {
    /// Create a new demo step
    pub fn new(
        title: impl Into<String>,
        narration: impl Into<String>,
        duration_ms: u64,
    ) -> Self {
        Self {
            title: title.into(),
            narration: narration.into(),
            duration_ms,
            feature: String::new(),
            code_example: None,
        }
    }

    /// Set feature name
    pub fn with_feature(mut self, feature: impl Into<String>) -> Self {
        self.feature = feature.into();
        self
    }

    /// Add code example
    pub fn with_code(mut self, code: impl Into<String>) -> Self {
        self.code_example = Some(code.into());
        self
    }
}

/// Demo mode state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DemoState {
    /// Not started
    Idle,
    /// Running
    Running,
    /// Paused
    Paused,
    /// Completed
    Completed,
}

/// Demo mode widget
pub struct DemoMode {
    /// Demo steps
    steps: Vec<DemoStep>,
    /// Current step index
    current_step: usize,
    /// Demo state
    state: DemoState,
    /// Time when current step started
    step_start_time: Option<Instant>,
    /// Whether to auto-advance
    auto_advance: bool,
    /// Whether to loop demo
    loop_demo: bool,
}

impl DemoMode {
    /// Create a new demo mode
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::widgets::DemoMode;
    ///
    /// let demo = DemoMode::new();
    /// assert!(!demo.is_running());
    /// ```
    pub fn new() -> Self {
        Self {
            steps: Self::default_steps(),
            current_step: 0,
            state: DemoState::Idle,
            step_start_time: None,
            auto_advance: true,
            loop_demo: false,
        }
    }

    /// Get default demo steps
    fn default_steps() -> Vec<DemoStep> {
        vec![
            DemoStep::new(
                "Welcome to TOAD",
                "TOAD is a Terminal-Oriented Autonomous Developer - an AI coding terminal built with Rust and Ratatui.",
                5000,
            )
            .with_feature("Welcome Screen"),

            DemoStep::new(
                "Vim-style Navigation",
                "Navigate efficiently with h/j/k/l keys, just like Vim. Jump to top with 'gg' and bottom with 'G'.",
                4000,
            )
            .with_feature("Vim Motions"),

            DemoStep::new(
                "Command Palette",
                "Press Ctrl+P to access all commands with fuzzy search. It's the fastest way to find any feature!",
                4000,
            )
            .with_feature("Command Palette"),

            DemoStep::new(
                "AI Chat Integration",
                "Interact with AI directly in the terminal. Stream responses in real-time and get code suggestions.",
                5000,
            )
            .with_feature("Chat Panel")
            .with_code("// AI suggests code improvements\nfn optimized_search() {\n    // Use binary search instead\n}"),

            DemoStep::new(
                "Git Integration",
                "Stage files, create commits, view diffs - all without leaving TOAD. Seamless Git workflow.",
                4000,
            )
            .with_feature("Git Integration"),

            DemoStep::new(
                "Split Panes",
                "Work on multiple files simultaneously with split panes. Resize and navigate between them easily.",
                4000,
            )
            .with_feature("Split Panes"),

            DemoStep::new(
                "Syntax Highlighting",
                "Beautiful syntax highlighting for all major languages, powered by tree-sitter.",
                3000,
            )
            .with_feature("Syntax Highlighting")
            .with_code("fn hello() {\n    println!(\"Hello, TOAD!\");\n}"),

            DemoStep::new(
                "Themes",
                "Choose from multiple themes including Catppuccin, Nord, and custom color schemes.",
                3000,
            )
            .with_feature("Theme System"),

            DemoStep::new(
                "Performance",
                "Blazing fast rendering with FPS limiting and incremental updates. Handles large files effortlessly.",
                4000,
            )
            .with_feature("Performance Monitoring"),

            DemoStep::new(
                "Thanks for watching!",
                "Explore TOAD's features yourself. Press '?' anytime for help, or Ctrl+P for the command palette.",
                5000,
            )
            .with_feature("Interactive Tutorial"),
        ]
    }

    /// Start the demo
    pub fn start(&mut self) {
        self.state = DemoState::Running;
        self.current_step = 0;
        self.step_start_time = Some(Instant::now());
    }

    /// Pause the demo
    pub fn pause(&mut self) {
        if self.state == DemoState::Running {
            self.state = DemoState::Paused;
        }
    }

    /// Resume the demo
    pub fn resume(&mut self) {
        if self.state == DemoState::Paused {
            self.state = DemoState::Running;
            self.step_start_time = Some(Instant::now());
        }
    }

    /// Stop the demo
    pub fn stop(&mut self) {
        self.state = DemoState::Idle;
        self.current_step = 0;
        self.step_start_time = None;
    }

    /// Toggle pause/resume
    pub fn toggle_pause(&mut self) {
        match self.state {
            DemoState::Running => self.pause(),
            DemoState::Paused => self.resume(),
            _ => {}
        }
    }

    /// Move to next step
    pub fn next_step(&mut self) {
        if self.current_step < self.steps.len() - 1 {
            self.current_step += 1;
            self.step_start_time = Some(Instant::now());
        } else if self.loop_demo {
            self.current_step = 0;
            self.step_start_time = Some(Instant::now());
        } else {
            self.state = DemoState::Completed;
        }
    }

    /// Move to previous step
    pub fn previous_step(&mut self) {
        if self.current_step > 0 {
            self.current_step -= 1;
            self.step_start_time = Some(Instant::now());
        }
    }

    /// Update demo state (call this regularly in event loop)
    pub fn update(&mut self) {
        if !self.auto_advance || self.state != DemoState::Running {
            return;
        }

        if let Some(start_time) = self.step_start_time {
            if let Some(step) = self.steps.get(self.current_step) {
                let elapsed = start_time.elapsed();
                let duration = Duration::from_millis(step.duration_ms);

                if elapsed >= duration {
                    self.next_step();
                }
            }
        }
    }

    /// Get current step
    pub fn current_step_data(&self) -> Option<&DemoStep> {
        self.steps.get(self.current_step)
    }

    /// Get progress (0.0 to 1.0)
    pub fn progress(&self) -> f32 {
        if let Some(start_time) = self.step_start_time {
            if let Some(step) = self.steps.get(self.current_step) {
                let elapsed = start_time.elapsed().as_millis() as f32;
                let duration = step.duration_ms as f32;
                (elapsed / duration).min(1.0)
            } else {
                0.0
            }
        } else {
            0.0
        }
    }

    /// Check if demo is running
    pub fn is_running(&self) -> bool {
        self.state == DemoState::Running
    }

    /// Check if demo is paused
    pub fn is_paused(&self) -> bool {
        self.state == DemoState::Paused
    }

    /// Check if demo is completed
    pub fn is_completed(&self) -> bool {
        self.state == DemoState::Completed
    }

    /// Set auto-advance
    pub fn set_auto_advance(&mut self, enabled: bool) {
        self.auto_advance = enabled;
    }

    /// Set loop mode
    pub fn set_loop(&mut self, enabled: bool) {
        self.loop_demo = enabled;
    }

    /// Get step count
    pub fn step_count(&self) -> usize {
        self.steps.len()
    }

    /// Get current step index
    pub fn current_step_index(&self) -> usize {
        self.current_step
    }
}

impl Default for DemoMode {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for &DemoMode {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.state == DemoState::Idle {
            // Show start screen
            let start_area = Rect {
                x: (area.width.saturating_sub(60)) / 2 + area.x,
                y: (area.height.saturating_sub(10)) / 2 + area.y,
                width: 60.min(area.width),
                height: 10.min(area.height),
            };

            Clear.render(start_area, buf);

            let text = vec![
                Line::from(""),
                Line::from(vec![Span::styled(
                    "TOAD Demo Mode",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                )]),
                Line::from(""),
                Line::from("Experience TOAD's features in an automated showcase"),
                Line::from(""),
                Line::from(vec![
                    Span::raw("Press "),
                    Span::styled("Enter", Style::default().fg(Color::Green)),
                    Span::raw(" to start"),
                ]),
            ];

            let para = Paragraph::new(text)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Cyan)),
                )
                .alignment(Alignment::Center);

            para.render(start_area, buf);
            return;
        }

        if let Some(step) = self.current_step_data() {
            // Calculate overlay area
            let width = (area.width as f32 * 0.8).min(80.0) as u16;
            let height = if step.code_example.is_some() { 20 } else { 15 };

            let overlay_area = Rect {
                x: (area.width.saturating_sub(width)) / 2 + area.x,
                y: (area.height.saturating_sub(height)) / 2 + area.y,
                width,
                height: height.min(area.height),
            };

            Clear.render(overlay_area, buf);

            // Split into content and progress bar
            let chunks = Layout::vertical([
                Constraint::Min(0),    // Content
                Constraint::Length(3), // Progress bar
            ])
            .split(overlay_area);

            // Render content
            let mut lines = vec![
                Line::from(vec![
                    Span::styled("🎬 ", Style::default().fg(Color::Yellow)),
                    Span::styled(
                        &step.title,
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::styled("Feature: ", Style::default().fg(Color::Gray)),
                    Span::styled(&step.feature, Style::default().fg(Color::Yellow)),
                ]),
                Line::from(""),
                Line::from(step.narration.as_str()),
            ];

            if let Some(code) = &step.code_example {
                lines.push(Line::from(""));
                lines.push(Line::from(vec![Span::styled(
                    "Example:",
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                )]));
                lines.push(Line::from(""));
                for code_line in code.lines() {
                    lines.push(Line::from(Span::styled(
                        code_line,
                        Style::default().fg(Color::White),
                    )));
                }
            }

            let content = Paragraph::new(lines)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(format!(
                            "Demo Mode ({}/{})",
                            self.current_step + 1,
                            self.steps.len()
                        ))
                        .border_style(Style::default().fg(Color::Cyan)),
                )
                .wrap(Wrap { trim: false });

            content.render(chunks[0], buf);

            // Render progress bar
            let progress = self.progress();
            let progress_width = (chunks[1].width.saturating_sub(4) as f32 * progress) as u16;

            let progress_text = if self.state == DemoState::Paused {
                "⏸ PAUSED"
            } else {
                "▶ PLAYING"
            };

            let mut progress_line = String::new();
            progress_line.push('[');
            for i in 0..chunks[1].width.saturating_sub(4) {
                if i < progress_width {
                    progress_line.push('█');
                } else {
                    progress_line.push('░');
                }
            }
            progress_line.push(']');

            let progress_para = Paragraph::new(vec![
                Line::from(progress_text),
                Line::from(progress_line),
            ])
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::Yellow));

            progress_para.render(chunks[1], buf);

            // Render footer
            let footer_area = Rect {
                x: overlay_area.x,
                y: overlay_area.y + overlay_area.height - 1,
                width: overlay_area.width,
                height: 1,
            };

            let footer_text = "Space: Pause/Resume | →: Next | ←: Previous | q/Esc: Exit";
            let footer = Paragraph::new(footer_text).style(Style::default().fg(Color::DarkGray));

            footer.render(footer_area, buf);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demo_mode_new() {
        let demo = DemoMode::new();
        assert_eq!(demo.state, DemoState::Idle);
        assert!(!demo.is_running());
        assert_eq!(demo.current_step, 0);
    }

    #[test]
    fn test_start_demo() {
        let mut demo = DemoMode::new();
        demo.start();

        assert!(demo.is_running());
        assert_eq!(demo.state, DemoState::Running);
        assert!(demo.step_start_time.is_some());
    }

    #[test]
    fn test_pause_resume() {
        let mut demo = DemoMode::new();
        demo.start();
        demo.pause();

        assert!(demo.is_paused());

        demo.resume();
        assert!(demo.is_running());
    }

    #[test]
    fn test_stop() {
        let mut demo = DemoMode::new();
        demo.start();
        demo.stop();

        assert_eq!(demo.state, DemoState::Idle);
        assert_eq!(demo.current_step, 0);
    }

    #[test]
    fn test_navigation() {
        let mut demo = DemoMode::new();
        demo.start();

        assert_eq!(demo.current_step, 0);

        demo.next_step();
        assert_eq!(demo.current_step, 1);

        demo.previous_step();
        assert_eq!(demo.current_step, 0);
    }

    #[test]
    fn test_completion() {
        let mut demo = DemoMode::new();
        demo.start();

        // Move to last step
        while demo.current_step < demo.steps.len() - 1 {
            demo.next_step();
        }

        demo.next_step(); // Should complete
        assert!(demo.is_completed());
    }

    #[test]
    fn test_loop_mode() {
        let mut demo = DemoMode::new();
        demo.set_loop(true);
        demo.start();

        // Move to last step
        while demo.current_step < demo.steps.len() - 1 {
            demo.next_step();
        }

        demo.next_step(); // Should loop back to 0
        assert_eq!(demo.current_step, 0);
        assert!(demo.is_running());
    }

    #[test]
    fn test_demo_step() {
        let step = DemoStep::new("Title", "Narration", 5000)
            .with_feature("Feature Name")
            .with_code("fn example() {}");

        assert_eq!(step.title, "Title");
        assert_eq!(step.narration, "Narration");
        assert_eq!(step.duration_ms, 5000);
        assert_eq!(step.feature, "Feature Name");
        assert!(step.code_example.is_some());
    }

    #[test]
    fn test_progress() {
        let demo = DemoMode::new();
        assert_eq!(demo.progress(), 0.0);
    }

    #[test]
    fn test_auto_advance() {
        let mut demo = DemoMode::new();
        demo.set_auto_advance(false);
        assert!(!demo.auto_advance);

        demo.set_auto_advance(true);
        assert!(demo.auto_advance);
    }
}
