//! Performance domain
//!
//! Contains performance optimization modules including lazy rendering,
//! live graphs, and virtual scrolling.

pub mod lazy_render;
pub mod live_graphs;
pub mod performance;
pub mod virtual_scroll;

pub use lazy_render::{LazyRenderManager, LazyRenderState, LazyRenderable};
pub use live_graphs::{DataPoint, GraphType, LiveGraph, LiveGraphManager, UpdateFrequency};
pub use performance::{FrameLimiter, PerformanceMetrics, TargetFPS};
pub use virtual_scroll::VirtualScrollState;
