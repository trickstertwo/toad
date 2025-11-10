//! UI Organisms - Complex compositions
//!
//! Following Atomic Design methodology, organisms are complex components
//! that compose multiple molecules and atoms.
//!
//! # Design Principles
//!
//! 1. **Complex Composition**: Organisms compose multiple molecules
//! 2. **Feature-Complete**: Each organism represents a complete feature section
//! 3. **Stateful**: May hold state for their composed molecules
//! 4. **Reusable**: Can be used in multiple screens/contexts
//! 5. **Testable**: Comprehensive test coverage on all public APIs
//!
//! # Organisms
//!
//! - [`eval_panel`]: Evaluation progress panel (composes ProgressBar, MetricCard, TaskItem)
//!
//! # Examples
//!
//! ```
//! use toad::ui::organisms::eval_panel::{EvalPanel, TaskStatus, TaskState};
//! use ratatui::{buffer::Buffer, layout::Rect};
//!
//! let panel = EvalPanel::new()
//!     .current_task(5)
//!     .total_tasks(10)
//!     .accuracy(85.2)
//!     .cost(0.45)
//!     .duration_secs(120)
//!     .add_task(TaskStatus {
//!         name: "Task 1".to_string(),
//!         status: TaskState::Completed,
//!         message: Some("Success".to_string()),
//!     });
//!
//! let area = Rect::new(0, 0, 80, 24);
//! let mut buf = Buffer::empty(area);
//! panel.render(area, &mut buf);
//! ```

pub mod eval_panel;

pub use eval_panel::{EvalPanel, TaskState, TaskStatus};
