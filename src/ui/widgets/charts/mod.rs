//! Chart visualization widgets
//!
//! This module contains widgets for data visualization including bar charts,
//! line charts, scatter plots, and sparklines.

pub mod bar_chart;
pub mod chart;
pub mod line_chart;
pub mod live_graph;
pub mod scatter_plot;
pub mod sparkline;

// Re-export all types for backwards compatibility
pub use bar_chart::*;
pub use chart::*;
pub use line_chart::*;
pub use live_graph::*;
pub use scatter_plot::*;
pub use sparkline::*;
