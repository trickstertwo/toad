//! Editor domain
//!
//! Contains editor-specific functionality like vim motions, undo/redo,
//! multi-cursor editing, and visual selection.

pub mod macros;
pub mod marks;
pub mod multicursor;
pub mod undo;
pub mod vim_motions;
pub mod visual_selection;

pub use macros::{Macro, MacroAction, MacroManager};
pub use marks::{Mark, MarkType, MarksManager};
pub use multicursor::{CursorPosition, MultiCursor};
pub use undo::{Action, HistoryNavigator, TextDelete, TextInsert, UndoStack};
pub use vim_motions::{Motion, VimMotions};
pub use visual_selection::{Position, SelectionMode, SelectionRange, VisualSelection};
