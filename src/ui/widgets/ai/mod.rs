//! AI-powered widgets
//!
//! This module contains widgets that leverage AI capabilities including
//! diff views for code changes, smart context-aware suggestions, and
//! provider configuration management.

pub mod diff_view;
pub mod provider_config;
pub mod suggestions;

pub use provider_config::{ProviderConfigPanel, ProviderEntry, ProviderStatus};
