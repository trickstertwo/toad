//! Interactive tutorial system for onboarding
//!
//! Provides a step-by-step walkthrough for first-time users with
//! interactive demonstrations, progress tracking, and contextual hints.
//!
//! # Examples
//!
//! ```
//! use toad::ui::widgets::InteractiveTutorial;
//!
//! let mut tutorial = InteractiveTutorial::new();
//! tutorial.start();
//! assert_eq!(tutorial.current_step(), 0);
//! ```

use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Wrap, Widget},
};
use serde::{Deserialize, Serialize};

/// Tutorial step definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TutorialStep {
    /// Step title
    pub title: String,
    /// Step description
    pub description: String,
    /// Expected user action
    pub action: String,
    /// Hint text
    pub hint: Option<String>,
    /// Whether this step requires user interaction
    pub interactive: bool,
    /// Highlight area (optional)
    pub highlight_area: Option<HighlightArea>,
}

/// Area to highlight during tutorial
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct HighlightArea {
    /// X position (percentage of screen width, 0.0-1.0)
    pub x: f32,
    /// Y position (percentage of screen height, 0.0-1.0)
    pub y: f32,
    /// Width (percentage of screen width, 0.0-1.0)
    pub width: f32,
    /// Height (percentage of screen height, 0.0-1.0)
    pub height: f32,
}

/// Tutorial state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TutorialState {
    /// Not started
    NotStarted,
    /// In progress
    InProgress,
    /// Completed
    Completed,
    /// Skipped by user
    Skipped,
}

/// Interactive tutorial widget
///
/// Guides users through application features with step-by-step instructions.
///
/// # Features
///
/// - Multiple tutorial steps
/// - Progress tracking
/// - Skip/restart options
/// - Interactive demonstrations
/// - Contextual hints
pub struct InteractiveTutorial {
    /// Tutorial steps
    steps: Vec<TutorialStep>,
    /// Current step index
    current_step: usize,
    /// Tutorial state
    state: TutorialState,
    /// Whether to show hints
    show_hints: bool,
    /// User has completed this step
    step_completed: bool,
}

impl InteractiveTutorial {
    /// Create a new interactive tutorial
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::widgets::InteractiveTutorial;
    ///
    /// let tutorial = InteractiveTutorial::new();
    /// assert_eq!(tutorial.state(), TutorialState::NotStarted);
    /// ```
    pub fn new() -> Self {
        Self {
            steps: Self::default_steps(),
            current_step: 0,
            state: TutorialState::NotStarted,
            show_hints: true,
            step_completed: false,
        }
    }

    /// Create tutorial with custom steps
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::widgets::{InteractiveTutorial, TutorialStep};
    ///
    /// let steps = vec![
    ///     TutorialStep {
    ///         title: "Welcome".to_string(),
    ///         description: "Welcome to TOAD!".to_string(),
    ///         action: "Press Enter to continue".to_string(),
    ///         hint: None,
    ///         interactive: false,
    ///         highlight_area: None,
    ///     },
    /// ];
    /// let tutorial = InteractiveTutorial::with_steps(steps);
    /// assert_eq!(tutorial.step_count(), 1);
    /// ```
    pub fn with_steps(steps: Vec<TutorialStep>) -> Self {
        Self {
            steps,
            current_step: 0,
            state: TutorialState::NotStarted,
            show_hints: true,
            step_completed: false,
        }
    }

    /// Get default tutorial steps
    fn default_steps() -> Vec<TutorialStep> {
        vec![
            TutorialStep {
                title: "Welcome to TOAD!".to_string(),
                description: "TOAD is a Terminal-Oriented Autonomous Developer. This tutorial will guide you through the basics.".to_string(),
                action: "Press Enter to continue".to_string(),
                hint: None,
                interactive: false,
                highlight_area: None,
            },
            TutorialStep {
                title: "Navigation".to_string(),
                description: "Use arrow keys or Vim-style keys (h/j/k/l) to navigate. Press j to move down.".to_string(),
                action: "Press j to continue".to_string(),
                hint: Some("j = down, k = up, h = left, l = right".to_string()),
                interactive: true,
                highlight_area: None,
            },
            TutorialStep {
                title: "Command Palette".to_string(),
                description: "Access all commands quickly with Ctrl+P. This opens the fuzzy command search.".to_string(),
                action: "Press Ctrl+P to open command palette".to_string(),
                hint: Some("You can type to fuzzy search commands".to_string()),
                interactive: true,
                highlight_area: None,
            },
            TutorialStep {
                title: "Search".to_string(),
                description: "Search within content using /. This works like Vim search.".to_string(),
                action: "Press / to start searching".to_string(),
                hint: Some("Use n/N to navigate between results".to_string()),
                interactive: true,
                highlight_area: None,
            },
            TutorialStep {
                title: "Help".to_string(),
                description: "Press ? anytime to see context-specific help and keybindings.".to_string(),
                action: "Press ? to view help".to_string(),
                hint: None,
                interactive: true,
                highlight_area: None,
            },
            TutorialStep {
                title: "Tutorial Complete!".to_string(),
                description: "You've learned the basics! Explore more features using Ctrl+P or ? for help.".to_string(),
                action: "Press Enter to finish".to_string(),
                hint: Some("You can restart this tutorial from the settings".to_string()),
                interactive: false,
                highlight_area: None,
            },
        ]
    }

    /// Start the tutorial
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::widgets::InteractiveTutorial;
    ///
    /// let mut tutorial = InteractiveTutorial::new();
    /// tutorial.start();
    /// assert_eq!(tutorial.state(), TutorialState::InProgress);
    /// ```
    pub fn start(&mut self) {
        self.state = TutorialState::InProgress;
        self.current_step = 0;
        self.step_completed = false;
    }

    /// Restart the tutorial from the beginning
    pub fn restart(&mut self) {
        self.start();
    }

    /// Skip the tutorial
    pub fn skip(&mut self) {
        self.state = TutorialState::Skipped;
    }

    /// Move to next step
    ///
    /// Returns true if moved to next step, false if already at the end.
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::widgets::InteractiveTutorial;
    ///
    /// let mut tutorial = InteractiveTutorial::new();
    /// tutorial.start();
    ///
    /// let moved = tutorial.next_step();
    /// assert!(moved);
    /// assert_eq!(tutorial.current_step(), 1);
    /// ```
    pub fn next_step(&mut self) -> bool {
        if self.current_step + 1 < self.steps.len() {
            self.current_step += 1;
            self.step_completed = false;
            true
        } else {
            self.state = TutorialState::Completed;
            false
        }
    }

    /// Move to previous step
    ///
    /// Returns true if moved to previous step, false if already at the beginning.
    pub fn previous_step(&mut self) -> bool {
        if self.current_step > 0 {
            self.current_step -= 1;
            self.step_completed = false;
            true
        } else {
            false
        }
    }

    /// Mark current step as completed
    pub fn complete_current_step(&mut self) {
        self.step_completed = true;
    }

    /// Check if current step is completed
    pub fn is_step_completed(&self) -> bool {
        self.step_completed
    }

    /// Get current step index
    pub fn current_step(&self) -> usize {
        self.current_step
    }

    /// Get total step count
    pub fn step_count(&self) -> usize {
        self.steps.len()
    }

    /// Get current step
    pub fn get_current_step(&self) -> Option<&TutorialStep> {
        self.steps.get(self.current_step)
    }

    /// Get tutorial state
    pub fn state(&self) -> TutorialState {
        self.state
    }

    /// Check if tutorial is active
    pub fn is_active(&self) -> bool {
        matches!(self.state, TutorialState::InProgress)
    }

    /// Toggle hint visibility
    pub fn toggle_hints(&mut self) {
        self.show_hints = !self.show_hints;
    }

    /// Get progress as percentage (0.0 to 1.0)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::widgets::InteractiveTutorial;
    ///
    /// let mut tutorial = InteractiveTutorial::new();
    /// tutorial.start();
    /// assert_eq!(tutorial.progress(), 0.0);
    ///
    /// tutorial.next_step();
    /// assert!(tutorial.progress() > 0.0);
    /// ```
    pub fn progress(&self) -> f64 {
        if self.steps.is_empty() {
            1.0
        } else {
            self.current_step as f64 / self.steps.len() as f64
        }
    }
}

impl Default for InteractiveTutorial {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for &InteractiveTutorial {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if !self.is_active() {
            return;
        }

        // Calculate centered dialog area
        let dialog_width = (area.width as f32 * 0.7).min(80.0) as u16;
        let dialog_height = (area.height as f32 * 0.6).min(25.0) as u16;

        let dialog_area = Rect {
            x: (area.width.saturating_sub(dialog_width)) / 2 + area.x,
            y: (area.height.saturating_sub(dialog_height)) / 2 + area.y,
            width: dialog_width,
            height: dialog_height,
        };

        // Clear the dialog area
        Clear.render(dialog_area, buf);

        // Get current step
        if let Some(step) = self.get_current_step() {
            // Split into header, content, action, footer
            let chunks = Layout::vertical([
                Constraint::Length(3),  // Header with progress
                Constraint::Min(5),     // Content
                Constraint::Length(4),  // Action required
                Constraint::Length(3),  // Footer with navigation
            ])
            .split(dialog_area);

            // Render header
            let progress_bar = "=".repeat((self.progress() * 40.0) as usize);
            let header_text = vec![
                Line::from(vec![
                    Span::styled(
                        format!("Step {}/{}: ", self.current_step + 1, self.steps.len()),
                        Style::default().fg(Color::Gray),
                    ),
                    Span::styled(
                        &step.title,
                        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
                    ),
                ]),
                Line::from(vec![
                    Span::styled("[", Style::default().fg(Color::Gray)),
                    Span::styled(&progress_bar, Style::default().fg(Color::Green)),
                    Span::styled(
                        " ".repeat(40 - progress_bar.len()),
                        Style::default().fg(Color::DarkGray),
                    ),
                    Span::styled("]", Style::default().fg(Color::Gray)),
                ]),
            ];

            let header = Paragraph::new(header_text).block(Block::default().borders(Borders::ALL));
            header.render(chunks[0], buf);

            // Render content
            let content = Paragraph::new(step.description.as_str())
                .block(Block::default().borders(Borders::ALL).title("Tutorial"))
                .wrap(Wrap { trim: false })
                .style(Style::default().fg(Color::White));
            content.render(chunks[1], buf);

            // Render action required
            let mut action_lines = vec![Line::from(vec![
                Span::styled("Action: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::raw(&step.action),
            ])];

            if self.show_hints {
                if let Some(hint) = &step.hint {
                    action_lines.push(Line::from(""));
                    action_lines.push(Line::from(vec![
                        Span::styled("💡 Hint: ", Style::default().fg(Color::Cyan)),
                        Span::styled(hint, Style::default().fg(Color::Gray).add_modifier(Modifier::ITALIC)),
                    ]));
                }
            }

            let action_widget = Paragraph::new(action_lines)
                .block(Block::default().borders(Borders::ALL))
                .wrap(Wrap { trim: false });
            action_widget.render(chunks[2], buf);

            // Render footer
            let footer_text = if self.step_completed {
                "✓ Step completed! Press Enter for next step | Ctrl+C to skip tutorial"
            } else {
                "← Previous | → Next | H to toggle hints | Ctrl+C to skip"
            };

            let footer = Paragraph::new(footer_text)
                .block(Block::default().borders(Borders::ALL))
                .style(Style::default().fg(Color::Gray))
                .alignment(Alignment::Center);
            footer.render(chunks[3], buf);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tutorial_new() {
        let tutorial = InteractiveTutorial::new();
        assert_eq!(tutorial.state(), TutorialState::NotStarted);
        assert_eq!(tutorial.current_step(), 0);
        assert!(tutorial.step_count() > 0);
    }

    #[test]
    fn test_tutorial_start() {
        let mut tutorial = InteractiveTutorial::new();
        tutorial.start();

        assert_eq!(tutorial.state(), TutorialState::InProgress);
        assert!(tutorial.is_active());
    }

    #[test]
    fn test_tutorial_navigation() {
        let mut tutorial = InteractiveTutorial::new();
        tutorial.start();

        let step_count = tutorial.step_count();

        // Move forward
        assert!(tutorial.next_step());
        assert_eq!(tutorial.current_step(), 1);

        // Move backward
        assert!(tutorial.previous_step());
        assert_eq!(tutorial.current_step(), 0);

        // Move to last step
        for _ in 0..step_count {
            tutorial.next_step();
        }

        // Should be completed now
        assert_eq!(tutorial.state(), TutorialState::Completed);
    }

    #[test]
    fn test_tutorial_skip() {
        let mut tutorial = InteractiveTutorial::new();
        tutorial.start();
        tutorial.skip();

        assert_eq!(tutorial.state(), TutorialState::Skipped);
        assert!(!tutorial.is_active());
    }

    #[test]
    fn test_tutorial_restart() {
        let mut tutorial = InteractiveTutorial::new();
        tutorial.start();
        tutorial.next_step();
        tutorial.next_step();

        tutorial.restart();

        assert_eq!(tutorial.current_step(), 0);
        assert_eq!(tutorial.state(), TutorialState::InProgress);
    }

    #[test]
    fn test_tutorial_progress() {
        let mut tutorial = InteractiveTutorial::new();
        tutorial.start();

        let initial_progress = tutorial.progress();
        tutorial.next_step();
        let next_progress = tutorial.progress();

        assert!(next_progress > initial_progress);
    }

    #[test]
    fn test_step_completion() {
        let mut tutorial = InteractiveTutorial::new();
        tutorial.start();

        assert!(!tutorial.is_step_completed());

        tutorial.complete_current_step();
        assert!(tutorial.is_step_completed());

        tutorial.next_step();
        assert!(!tutorial.is_step_completed());
    }

    #[test]
    fn test_custom_steps() {
        let steps = vec![
            TutorialStep {
                title: "Step 1".to_string(),
                description: "Description".to_string(),
                action: "Action".to_string(),
                hint: None,
                interactive: false,
                highlight_area: None,
            },
        ];

        let tutorial = InteractiveTutorial::with_steps(steps);
        assert_eq!(tutorial.step_count(), 1);
    }

    #[test]
    fn test_hint_toggle() {
        let mut tutorial = InteractiveTutorial::new();
        let initial = tutorial.show_hints;

        tutorial.toggle_hints();
        assert_eq!(tutorial.show_hints, !initial);

        tutorial.toggle_hints();
        assert_eq!(tutorial.show_hints, initial);
    }
}
