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
//!
//! # Examples
//!
//! ```
//! use toad::ui::molecules::metric_card::MetricCard;
//! use toad::ui::molecules::task_item::TaskItem;
//! use toad::ui::atoms::icon::Icon;
//! use toad::ui::nerd_fonts::UiIcon;
//!
//! let card = MetricCard::new("Accuracy", "85.2%")
//!     .icon(Icon::ui(UiIcon::Success));
//!
//! let task = TaskItem::completed("Build project").status("2.3s");
//! ```

pub mod metric_card;
pub mod task_item;

pub use metric_card::MetricCard;
pub use task_item::TaskItem;
