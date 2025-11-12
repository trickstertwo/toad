//! Custom widgets for Toad TUI
//!
//! Reusable UI components following Ratatui patterns
//!
//! # Module Organization
//!
//! - **ai**: AI-powered widgets (diff view, suggestions)
//! - **charts**: Data visualization charts
//! - **chat_panel**: Chat interface components
//! - **context**: Context management (token usage, file list, cost tracking)
//! - **conversation**: Conversation view widgets
//! - **core**: Core UI primitives (dialogs, tables, help, collapsible, undo/redo)
//! - **files**: File management (tree, preview, cards)
//! - **git**: Git operations (branches, commits, diffs, conflicts, staging)
//! - **input**: User input (text areas, vim mode, macros, mode indicators)
//! - **layout**: Layout management (splits, floating, panels, tabs)
//! - **notifications**: Notifications and alerts (toasts, modals, tutorials)
//! - **performance**: Performance monitoring (FPS, profiling, metrics, memory)
//! - **progress**: Progress indicators
//! - **selection**: Selection widgets (context menus, pickers, multiselect)
//! - **session_manager**: Session management
//! - **tools**: Tool execution status and monitoring
//! - **workspace**: Workspace management

pub mod ai;
pub mod charts;
pub mod chat_panel;
pub mod context;
pub mod conversation;
pub mod core;
pub mod files;
pub mod git;
pub mod input;
pub mod layout;
pub mod notifications;
pub mod performance;
pub mod progress;
pub mod selection;
pub mod session_manager;
pub mod tools;
pub mod workspace;
