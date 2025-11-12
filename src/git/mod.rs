//! Git integration module
//!
//! Provides services for interacting with git repositories, including
//! status checking, commit history, branch management, staging operations,
//! and auto-commit functionality for AI-assisted changes.

pub mod auto_commit;
pub mod graph_service;
pub mod service;

pub use auto_commit::AutoCommitManager;
pub use graph_service::GitGraphService;
pub use service::{BranchInfo, CommitInfo, FileChange, GitService};
