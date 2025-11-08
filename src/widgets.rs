//! Custom widgets for Toad TUI
//!
//! Reusable UI components following Ratatui patterns

pub mod borders;
pub mod dialog;
pub mod filetree;
pub mod help;
pub mod input;
pub mod input_dialog;
pub mod input_prompt;
pub mod modal;
pub mod palette;
pub mod progress;
pub mod sparkline;
pub mod spinner;
pub mod split;
pub mod table;
pub mod textarea;
pub mod toast;
pub mod welcome;

pub use borders::{BorderSet, BorderStyle};
pub use dialog::{ConfirmDialog, DialogOption};
pub use filetree::{FileTree, FileTreeNode, FileTreeNodeType};
pub use help::HelpScreen;
pub use input::InputField;
pub use input_dialog::{InputDialog, InputDialogState};
pub use input_prompt::InputPrompt;
pub use modal::{Modal, ModalType};
pub use palette::{CommandPalette, PaletteCommand};
pub use progress::{MultiStageProgress, ProgressBar, StageStatus};
pub use sparkline::{Sparkline, SparklineStyle};
pub use spinner::{Spinner, SpinnerStyle};
pub use split::{PaneBorderStyle, SplitDirection, SplitPane, SplitPaneError, SplitSize};
pub use table::{ColumnAlignment, DataTable, TableColumn};
pub use textarea::Textarea;
pub use toast::{Toast, ToastLevel, ToastManager};
pub use welcome::WelcomeScreen;
