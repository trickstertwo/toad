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

// Re-export types for backwards compatibility
// Note: chart module contains generic implementations; specific implementations
// are in dedicated modules (bar_chart, line_chart, scatter_plot)
pub use bar_chart::*;
pub use line_chart::*;
pub use live_graph::*;
pub use scatter_plot::*;
pub use sparkline::*;

// Re-export chart module with explicit namespace to avoid conflicts
pub use chart::{BarOrientation, LineStyle};
