//! Event handling module
//!
//! This module defines the Event type which represents all possible
//! messages/events in the Elm Architecture pattern.

use crate::ai::eval_commands::{CompareArgs, EvalArgs};
use crate::ai::evaluation::{EvaluationResults, TaskResult};
use crossterm::event::{self, KeyEvent, MouseEvent};
use std::time::Duration;

/// Events that can occur in the application (Message in Elm Architecture)
#[derive(Debug, Clone)]
pub enum Event {
    /// Terminal tick event (for animations, etc.)
    Tick,

    /// Key press event
    Key(KeyEvent),

    /// Mouse event
    Mouse(MouseEvent),

    /// Terminal resize event
    Resize(u16, u16),

    /// Application should quit
    Quit,

    // Evaluation events
    /// Start an evaluation run
    StartEvaluation(EvalArgs),

    /// Start a comparison run
    StartComparison(CompareArgs),

    /// Evaluation progress update
    EvaluationProgress(EvaluationProgress),

    /// Evaluation completed successfully
    EvaluationComplete(EvaluationResults),

    /// Evaluation failed with error
    EvaluationError(String),

    /// Cancel running evaluation
    CancelEvaluation,
}

/// Progress information for a running evaluation
#[derive(Debug, Clone)]
pub struct EvaluationProgress {
    /// Current task number (1-indexed)
    pub current_task: usize,

    /// Total number of tasks
    pub total_tasks: usize,

    /// Current task ID
    pub task_id: String,

    /// Current agent step (1-indexed)
    pub current_step: Option<usize>,

    /// Maximum steps for agent
    pub max_steps: Option<usize>,

    /// Last tool called by agent
    pub last_tool: Option<String>,

    /// Running token count
    pub total_tokens: u64,

    /// Running cost in USD
    pub total_cost: f64,

    /// Latest message from agent
    pub message: Option<String>,

    /// Result of most recently completed task
    pub last_result: Option<TaskResult>,
}

/// Event handler that polls for terminal events
pub struct EventHandler {
    #[allow(dead_code)]
    tick_rate: Duration,
}

impl EventHandler {
    /// Create a new event handler with the given tick rate
    pub fn new(tick_rate: Duration) -> Self {
        Self { tick_rate }
    }

    /// Poll for the next event
    ///
    /// This blocks until an event is available or the timeout is reached.
    pub fn next(&self) -> crate::Result<Event> {
        // Check if there's an event available
        if event::poll(self.tick_rate)? {
            match event::read()? {
                event::Event::Key(key) => Ok(Event::Key(key)),
                event::Event::Mouse(mouse) => Ok(Event::Mouse(mouse)),
                event::Event::Resize(width, height) => Ok(Event::Resize(width, height)),
                _ => Ok(Event::Tick),
            }
        } else {
            Ok(Event::Tick)
        }
    }
}
