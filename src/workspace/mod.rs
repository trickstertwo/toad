//! Workspace domain
//!
//! Contains workspace and layout management functionality including tabs,
//! split panes, session persistence, and resizable panels.

pub mod layout;
pub mod resizable;
pub mod session;
pub mod tabs;
pub mod workspaces;

pub use layout::{LayoutManager, Pane, PanelId, SplitDirection};
pub use resizable::{ResizablePane, ResizablePaneManager, ResizeDirection};
pub use session::SessionState;
pub use tabs::{Tab, TabId, TabManager};
pub use workspaces::{Workspace, WorkspaceManager};
