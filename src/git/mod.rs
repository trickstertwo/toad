//! Git integration module
//!
//! Provides services for interacting with git repositories, including
//! status checking, commit history, branch management, and staging operations.

pub mod service;

pub use service::{BranchInfo, CommitInfo, FileChange, GitService};
