//! UI Screens - Top-level layouts
//!
//! Following Atomic Design methodology, screens are complete layouts that compose
//! organisms, molecules, and atoms into full-page experiences.
//!
//! # Design Principles
//!
//! 1. **Complete Layouts**: Screens represent entire pages/views
//! 2. **Stateful**: Manage screen-level state and interactions
//! 3. **Composable**: Use organisms as building blocks
//! 4. **Interactive**: Handle user input and events
//! 5. **Testable**: Comprehensive test coverage
//!
//! # Screens
//!
//! - [`evaluation`]: Real-time evaluation progress display
//!
//! # Examples
//!
//! ```
//! use toad::ui::screens::evaluation::EvaluationScreen;
//! use toad::ui::organisms::eval_panel::{TaskStatus, TaskState};
//! use ratatui::{buffer::Buffer, layout::Rect};
//!
//! let mut screen = EvaluationScreen::new("M1 Baseline Evaluation");
//! screen.update_progress(5, 10);
//! screen.update_accuracy(85.2);
//! screen.update_cost(0.45);
//! screen.update_duration(120);
//!
//! screen.add_task(TaskStatus {
//!     name: "Task 1".to_string(),
//!     status: TaskState::Completed,
//!     message: Some("Success".to_string()),
//! });
//!
//! let area = Rect::new(0, 0, 80, 24);
//! let mut buf = Buffer::empty(area);
//! screen.render(area, &mut buf);
//! ```

pub mod evaluation;

pub use evaluation::EvaluationScreen;
