//! Performance monitoring and profiling widgets
//!
//! This module contains widgets for monitoring application performance including
//! FPS counters, render profilers, event metrics, and memory usage.

pub mod event_metrics;
pub mod fps;
pub mod memory;
pub mod render_profiler;

// Re-export all types for backwards compatibility
pub use event_metrics::*;
pub use fps::*;
pub use memory::*;
pub use render_profiler::*;
