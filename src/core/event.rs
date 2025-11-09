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

    // ===== Missing Event Variant Tests =====
    #[test]
    fn test_event_mouse_variant() {
        use crossterm::event::{MouseButton, MouseEventKind};
        let mouse_event = crossterm::event::MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: 10,
            row: 5,
            modifiers: KeyModifiers::NONE,
        };
        let event = Event::Mouse(mouse_event);

        match event {
            Event::Mouse(m) => {
                assert_eq!(m.column, 10);
                assert_eq!(m.row, 5);
            }
            _ => panic!("Event should be Mouse variant"),
        }
    }

    #[test]
    fn test_event_start_evaluation_variant() {
        use crate::ai::eval_commands::EvalArgs;
        use crate::ai::evaluation::DatasetSource;
        let eval_args = EvalArgs {
            count: Some(10),
            dataset: DatasetSource::Verified,
            milestone: 1,
            output: None,
        };
        let event = Event::StartEvaluation(eval_args.clone());

        match event {
            Event::StartEvaluation(args) => {
                assert_eq!(args.count, Some(10));
                assert_eq!(args.milestone, 1);
            }
            _ => panic!("Event should be StartEvaluation variant"),
        }
    }

    #[test]
    fn test_event_start_comparison_variant() {
        use crate::ai::eval_commands::CompareArgs;
        use crate::ai::evaluation::DatasetSource;
        let compare_args = CompareArgs {
            count: Some(20),
            dataset: DatasetSource::Verified,
            baseline: 1,
            test: 2,
            output: None,
        };
        let event = Event::StartComparison(compare_args.clone());

        match event {
            Event::StartComparison(args) => {
                assert_eq!(args.count, Some(20));
                assert_eq!(args.baseline, 1);
                assert_eq!(args.test, 2);
            }
            _ => panic!("Event should be StartComparison variant"),
        }
    }

    #[test]
    fn test_event_evaluation_progress_variant() {
        let progress = EvaluationProgress {
            current_task: 5,
            total_tasks: 10,
            task_id: "task_789".to_string(),
            current_step: Some(10),
            max_steps: Some(25),
            last_tool: Some("Grep".to_string()),
            total_tokens: 3000,
            total_cost: 0.15,
            message: Some("Searching files".to_string()),
            last_result: None,
        };
        let event = Event::EvaluationProgress(progress.clone());

        match event {
            Event::EvaluationProgress(p) => {
                assert_eq!(p.current_task, 5);
                assert_eq!(p.total_tasks, 10);
                assert_eq!(p.task_id, "task_789");
            }
            _ => panic!("Event should be EvaluationProgress variant"),
        }
    }

    #[test]
    fn test_event_evaluation_complete_variant() {
        use crate::ai::evaluation::EvaluationResults;
        use std::collections::HashMap;
        use chrono::Utc;

        let results = EvaluationResults {
            config_name: "M1".to_string(),
            results: vec![],
            accuracy: 0.8,
            avg_cost_usd: 0.25,
            avg_duration_ms: 1205.0,
            total_tasks: 10,
            tasks_solved: 8,
            by_complexity: HashMap::new(),
            timestamp: Utc::now(),
        };
        let event = Event::EvaluationComplete(results);

        match event {
            Event::EvaluationComplete(r) => {
                assert_eq!(r.total_tasks, 10);
                assert_eq!(r.tasks_solved, 8);
                assert_eq!(r.accuracy, 0.8);
            }
            _ => panic!("Event should be EvaluationComplete variant"),
        }
    }

    // ===== Event Clone Tests for All Variants =====
    #[test]
    fn test_event_key_clone() {
        let key_event = KeyEvent::new(KeyCode::Enter, KeyModifiers::CONTROL);
        let event1 = Event::Key(key_event);
        let event2 = event1.clone();

        match (event1, event2) {
            (Event::Key(k1), Event::Key(k2)) => {
                assert_eq!(k1.code, k2.code);
                assert_eq!(k1.modifiers, k2.modifiers);
            }
            _ => panic!("Both should be Key events"),
        }
    }

    #[test]
    fn test_event_resize_clone() {
        let event1 = Event::Resize(200, 100);
        let event2 = event1.clone();

        match (event1, event2) {
            (Event::Resize(w1, h1), Event::Resize(w2, h2)) => {
                assert_eq!(w1, w2);
                assert_eq!(h1, h2);
            }
            _ => panic!("Both should be Resize events"),
        }
    }

    #[test]
    fn test_event_evaluation_error_clone() {
        let event1 = Event::EvaluationError("Connection failed".to_string());
        let event2 = event1.clone();

        match (event1, event2) {
            (Event::EvaluationError(e1), Event::EvaluationError(e2)) => {
                assert_eq!(e1, e2);
            }
            _ => panic!("Both should be EvaluationError events"),
        }
    }

    // ===== Event Debug Format Tests =====
    #[test]
    fn test_all_event_variants_debug() {
        let events = vec![
            Event::Tick,
            Event::Quit,
            Event::CancelEvaluation,
            Event::Resize(80, 24),
            Event::Key(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE)),
            Event::EvaluationError("Test error".to_string()),
        ];

        for event in events {
            let debug_str = format!("{:?}", event);
            assert!(!debug_str.is_empty());
        }
    }

    // ===== EvaluationProgress Field Combination Tests =====
    #[test]
    fn test_evaluation_progress_all_none_optionals() {
        let progress = EvaluationProgress {
            current_task: 1,
            total_tasks: 1,
            task_id: "minimal".to_string(),
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
        assert!(progress.last_result.is_none());
    }

    #[test]
    fn test_evaluation_progress_all_some_optionals() {
        let progress = EvaluationProgress {
            current_task: 5,
            total_tasks: 10,
            task_id: "full".to_string(),
            current_step: Some(15),
            max_steps: Some(25),
            last_tool: Some("List".to_string()),
            total_tokens: 5000,
            total_cost: 0.25,
            message: Some("All fields populated".to_string()),
            last_result: None,
        };

        assert!(progress.current_step.is_some());
        assert!(progress.max_steps.is_some());
        assert!(progress.last_tool.is_some());
        assert!(progress.message.is_some());
        assert_eq!(progress.current_step.unwrap(), 15);
    }

    #[test]
    fn test_evaluation_progress_zero_cost() {
        let progress = EvaluationProgress {
            current_task: 1,
            total_tasks: 1,
            task_id: "free".to_string(),
            current_step: None,
            max_steps: None,
            last_tool: None,
            total_tokens: 0,
            total_cost: 0.0,
            message: None,
            last_result: None,
        };

        assert_eq!(progress.total_cost, 0.0);
        assert_eq!(progress.total_tokens, 0);
    }

    #[test]
    fn test_evaluation_progress_decimal_cost() {
        let progress = EvaluationProgress {
            current_task: 1,
            total_tasks: 1,
            task_id: "precise".to_string(),
            current_step: None,
            max_steps: None,
            last_tool: None,
            total_tokens: 1234,
            total_cost: 0.123456,
            message: None,
            last_result: None,
        };

        assert_eq!(progress.total_cost, 0.123456);
    }

    // ===== EventHandler Edge Cases =====
    #[test]
    fn test_event_handler_zero_tick_rate() {
        let handler = EventHandler::new(Duration::from_millis(0));
        assert_eq!(handler.tick_rate, Duration::from_millis(0));
    }

    #[test]
    fn test_event_handler_large_tick_rate() {
        let handler = EventHandler::new(Duration::from_secs(3600)); // 1 hour
        assert_eq!(handler.tick_rate, Duration::from_secs(3600));
    }

    // ===== Event Variant Matching Exhaustiveness =====
    #[test]
    fn test_event_match_all_variants() {
        // Ensure all Event variants can be matched
        let events: Vec<Event> = vec![
            Event::Tick,
            Event::Quit,
            Event::CancelEvaluation,
        ];

        for event in events {
            match event {
                Event::Tick => {}
                Event::Key(_) => {}
                Event::Mouse(_) => {}
                Event::Resize(_, _) => {}
                Event::Quit => {}
                Event::StartEvaluation(_) => {}
                Event::StartComparison(_) => {}
                Event::EvaluationProgress(_) => {}
                Event::EvaluationComplete(_) => {}
                Event::EvaluationError(_) => {}
                Event::CancelEvaluation => {}
            }
        }
    }
}
