//! Custom widgets for Toad TUI
//!
//! Reusable UI components following Ratatui patterns

pub mod breadcrumbs;
pub mod collapsible;
pub mod context_menu;
pub mod dialog;
pub mod filetree;
pub mod help;
pub mod input;
pub mod input_prompt;
pub mod modal;
pub mod palette;
pub mod panel;
pub mod progress;
pub mod scrollbar;
pub mod spinner;
pub mod statusline;
pub mod tabbar;
pub mod table;
pub mod textarea;
pub mod toast;
pub mod welcome;

pub use breadcrumbs::{Breadcrumbs, BreadcrumbSegment};
pub use collapsible::{CollapsibleList, CollapsibleSection};
pub use context_menu::{ContextMenu, MenuItem};
pub use dialog::{ConfirmDialog, DialogOption};
pub use filetree::{FileTree, FileTreeNode, FileTreeNodeType};
pub use help::HelpScreen;
pub use input::InputField;
pub use input_prompt::InputPrompt;
pub use modal::{Modal, ModalType};
pub use palette::{CommandPalette, PaletteCommand};
pub use panel::Panel;
pub use progress::{MultiStageProgress, ProgressBar};
pub use scrollbar::{Scrollbar, ScrollbarOrientation, ScrollbarState};
pub use spinner::{Spinner, SpinnerStyle};
pub use statusline::{SectionAlignment, StatusLevel, StatusSection, Statusline};
pub use tabbar::TabBar;
pub use table::{ColumnAlignment, DataTable, TableColumn};
pub use textarea::Textarea;
pub use toast::{Toast, ToastLevel, ToastManager};
pub use welcome::WelcomeScreen;
