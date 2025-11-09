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

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyCode, KeyModifiers};

    #[test]
    fn test_event_handler_creation() {
        let handler = EventHandler::new(Duration::from_millis(250));
        assert_eq!(handler.tick_rate, Duration::from_millis(250));
    }

    #[test]
    fn test_event_handler_different_tick_rates() {
        let handler_fast = EventHandler::new(Duration::from_millis(16)); // ~60 FPS
        let handler_slow = EventHandler::new(Duration::from_secs(1));

        assert_eq!(handler_fast.tick_rate, Duration::from_millis(16));
        assert_eq!(handler_slow.tick_rate, Duration::from_secs(1));
    }

    #[test]
    fn test_evaluation_progress_creation() {
        let progress = EvaluationProgress {
            current_task: 5,
            total_tasks: 10,
            task_id: "task_123".to_string(),
            current_step: Some(3),
            max_steps: Some(25),
            last_tool: Some("Read".to_string()),
            total_tokens: 1500,
            total_cost: 0.05,
            message: Some("Reading file...".to_string()),
            last_result: None,
        };

        assert_eq!(progress.current_task, 5);
        assert_eq!(progress.total_tasks, 10);
        assert_eq!(progress.task_id, "task_123");
        assert_eq!(progress.current_step, Some(3));
        assert_eq!(progress.last_tool, Some("Read".to_string()));
        assert_eq!(progress.total_tokens, 1500);
        assert_eq!(progress.total_cost, 0.05);
    }

    #[test]
    fn test_evaluation_progress_optional_fields() {
        let progress = EvaluationProgress {
            current_task: 1,
            total_tasks: 1,
            task_id: "simple".to_string(),
            current_step: None,
            max_steps: None,
            last_tool: None,
            total_tokens: 0,
            total_cost: 0.0,
            message: None,
            last_result: None,
        };

        assert!(progress.current_step.is_none());
        assert!(progress.max_steps.is_none());
        assert!(progress.last_tool.is_none());
        assert!(progress.message.is_none());
    }

    #[test]
    fn test_event_tick_variant() {
        let event = Event::Tick;
        match event {
            Event::Tick => {} // Success
            _ => panic!("Event should be Tick variant"),
        }
    }

    #[test]
    fn test_event_key_variant() {
        let key_event = KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE);
        let event = Event::Key(key_event);

        match event {
            Event::Key(k) => {
                assert_eq!(k.code, KeyCode::Char('a'));
                assert_eq!(k.modifiers, KeyModifiers::NONE);
            }
            _ => panic!("Event should be Key variant"),
        }
    }

    #[test]
    fn test_event_resize_variant() {
        let event = Event::Resize(100, 50);

        match event {
            Event::Resize(width, height) => {
                assert_eq!(width, 100);
                assert_eq!(height, 50);
            }
            _ => panic!("Event should be Resize variant"),
        }
    }

    #[test]
    fn test_event_quit_variant() {
        let event = Event::Quit;
        match event {
            Event::Quit => {} // Success
            _ => panic!("Event should be Quit variant"),
        }
    }

    #[test]
    fn test_event_cancel_evaluation_variant() {
        let event = Event::CancelEvaluation;
        match event {
            Event::CancelEvaluation => {} // Success
            _ => panic!("Event should be CancelEvaluation variant"),
        }
    }

    #[test]
    fn test_event_evaluation_error_variant() {
        let error_msg = "Failed to load dataset".to_string();
        let event = Event::EvaluationError(error_msg.clone());

        match event {
            Event::EvaluationError(msg) => {
                assert_eq!(msg, "Failed to load dataset");
            }
            _ => panic!("Event should be EvaluationError variant"),
        }
    }

    #[test]
    fn test_event_debug_format() {
        let event = Event::Tick;
        let debug_str = format!("{:?}", event);
        assert!(debug_str.contains("Tick"));
    }

    #[test]
    fn test_event_clone() {
        let event1 = Event::Quit;
        let event2 = event1.clone();

        match (event1, event2) {
            (Event::Quit, Event::Quit) => {} // Both should be Quit
            _ => panic!("Clone should preserve event type"),
        }
    }

    #[test]
    fn test_evaluation_progress_clone() {
        let progress1 = EvaluationProgress {
            current_task: 1,
            total_tasks: 5,
            task_id: "test".to_string(),
            current_step: Some(2),
            max_steps: Some(10),
            last_tool: Some("Write".to_string()),
            total_tokens: 500,
            total_cost: 0.01,
            message: Some("Testing".to_string()),
            last_result: None,
        };

        let progress2 = progress1.clone();
        assert_eq!(progress1.current_task, progress2.current_task);
        assert_eq!(progress1.task_id, progress2.task_id);
        assert_eq!(progress1.total_tokens, progress2.total_tokens);
    }

    #[test]
    fn test_evaluation_progress_debug_format() {
        let progress = EvaluationProgress {
            current_task: 3,
            total_tasks: 10,
            task_id: "debug_test".to_string(),
            current_step: Some(5),
            max_steps: Some(25),
            last_tool: Some("Bash".to_string()),
            total_tokens: 2000,
            total_cost: 0.10,
            message: Some("Running command".to_string()),
            last_result: None,
        };

        let debug_str = format!("{:?}", progress);
        assert!(debug_str.contains("current_task"));
        assert!(debug_str.contains("debug_test"));
    }

    #[test]
    fn test_evaluation_progress_high_token_count() {
        let progress = EvaluationProgress {
            current_task: 50,
            total_tasks: 100,
            task_id: "large_task".to_string(),
            current_step: Some(20),
            max_steps: Some(25),
            last_tool: Some("Edit".to_string()),
            total_tokens: 1_000_000, // 1 million tokens
            total_cost: 15.50,
            message: Some("Processing large file".to_string()),
            last_result: None,
        };

        assert_eq!(progress.total_tokens, 1_000_000);
        assert_eq!(progress.total_cost, 15.50);
    }

    #[test]
    fn test_evaluation_progress_task_boundaries() {
        // First task
        let first = EvaluationProgress {
            current_task: 1,
            total_tasks: 100,
            task_id: "first".to_string(),
            current_step: Some(1),
            max_steps: Some(25),
            last_tool: None,
            total_tokens: 0,
            total_cost: 0.0,
            message: None,
            last_result: None,
        };

        // Last task
        let last = EvaluationProgress {
            current_task: 100,
            total_tasks: 100,
            task_id: "last".to_string(),
            current_step: Some(25),
            max_steps: Some(25),
            last_tool: Some("Git".to_string()),
            total_tokens: 50000,
            total_cost: 2.50,
            message: Some("Finishing up".to_string()),
            last_result: None,
        };

        assert_eq!(first.current_task, 1);
        assert_eq!(last.current_task, 100);
        assert_eq!(first.current_task < last.current_task, true);
    }
}
