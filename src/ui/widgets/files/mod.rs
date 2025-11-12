//! File management widgets
//!
//! This module contains widgets for file navigation, preview, and management
//! including file trees, preview managers, card previews, and context-aware browsing.

pub mod card_preview;
pub mod context_browser;
pub mod preview_manager;
pub mod tree;

pub use context_browser::{ContextBrowser, FileEntry};
