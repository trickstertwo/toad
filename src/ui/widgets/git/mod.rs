//! Git-related widgets for version control UI
//!
//! This module contains widgets for Git operations including branch management,
//! commit dialogs, diff viewing, staging UI, and conflict resolution.

pub mod conflict_resolver;
pub mod git_branch_manager;
pub mod git_commit_dialog;
pub mod git_diff_viewer;
pub mod git_graph;
pub mod git_stage_ui;
pub mod git_status_panel;

// Re-export all types for backwards compatibility
pub use conflict_resolver::*;
pub use git_branch_manager::*;
pub use git_commit_dialog::*;
pub use git_diff_viewer::*;
pub use git_graph::*;
pub use git_stage_ui::*;
pub use git_status_panel::*;
