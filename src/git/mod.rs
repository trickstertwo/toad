//! Git integration module
//!
//! Provides services for interacting with git repositories, including
//! status checking, commit history, branch management, and staging operations.

pub mod graph_service;
pub mod service;

pub use graph_service::GitGraphService;
pub use service::{BranchInfo, CommitInfo, FileChange, GitService};
