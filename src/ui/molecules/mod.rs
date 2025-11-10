//! UI Molecules - Composite components
//!
//! Following Atomic Design methodology, molecules are combinations of atoms
//! that form functional UI components.
//!
//! # Design Principles
//!
//! 1. **Composition**: Molecules compose 2+ atoms together
//! 2. **Single Purpose**: Each molecule serves one clear function
//! 3. **Pure Rendering**: No mutable state, pure functions
//! 4. **Reusable**: Can be used in multiple organisms/screens
//! 5. **Testable**: 100% test coverage on all public APIs
//!
//! # Molecules
//!
//! - [`metric_card`]: Labeled metric with optional icon (composes Text + Icon)
//! - [`task_item`]: Task list item with icon and status (composes Icon + Text)
//! - [`progress_bar`]: Progress bar with label and percentage (composes Text atoms)
//! - [`token_counter`]: API token usage display (composes Text + Icon)
//!
//! # Examples
//!
//! ```
//! use toad::ui::molecules::metric_card::MetricCard;
//! use toad::ui::molecules::task_item::TaskItem;
//! use toad::ui::molecules::progress_bar::ProgressBar;
//! use toad::ui::molecules::token_counter::TokenCounter;
//! use toad::ui::atoms::icon::Icon;
//! use toad::ui::nerd_fonts::UiIcon;
//!
//! let card = MetricCard::new("Accuracy", "85.2%")
//!     .icon(Icon::ui(UiIcon::Success));
//!
//! let task = TaskItem::completed("Build project").status("2.3s");
//!
//! let progress = ProgressBar::success("Tasks", 7, 10);
//!
//! let counter = TokenCounter::new(1500, 0.045);
//! ```

pub mod agent_step_item;
pub mod api_call_metrics;
pub mod context_window;
pub mod cost_tracker;
pub mod metric_card;
pub mod model_selector;
pub mod progress_bar;
pub mod task_item;
pub mod token_counter;
pub mod tool_execution_item;

pub use agent_step_item::{AgentStepItem, StepStatus};
pub use api_call_metrics::{APICallMetrics, ThrottleStatus};
pub use context_window::{ContextWindow, UsageState};
pub use cost_tracker::{BudgetStatus, CostTracker};
pub use metric_card::MetricCard;
pub use model_selector::ModelSelector;
pub use progress_bar::ProgressBar;
pub use task_item::TaskItem;
pub use token_counter::TokenCounter;
pub use tool_execution_item::{ExecutionStatus, ToolExecutionItem};
