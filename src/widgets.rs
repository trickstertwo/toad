//! Custom widgets for Toad TUI
//!
//! Reusable UI components following Ratatui patterns

pub mod dialog;
pub mod filetree;
pub mod help;
pub mod input;
pub mod modal;
pub mod palette;
pub mod progress;
pub mod table;
pub mod textarea;
pub mod welcome;

pub use dialog::{ConfirmDialog, DialogOption};
pub use filetree::{FileTree, FileTreeNode, FileTreeNodeType};
pub use help::HelpScreen;
pub use input::InputField;
pub use modal::{Modal, ModalType};
pub use palette::{CommandPalette, PaletteCommand};
pub use progress::{MultiStageProgress, ProgressBar};
pub use table::{ColumnAlignment, DataTable, TableColumn};
pub use textarea::Textarea;
pub use welcome::WelcomeScreen;
