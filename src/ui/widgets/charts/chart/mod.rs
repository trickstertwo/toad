//! Chart widget for data visualization
//!
//! Provides customizable charts with multiple datasets and axes.

mod state;
#[cfg(test)]
mod tests;

// Re-export all public types
pub use state::{BarChart, BarOrientation, LineChart, LineStyle, ScatterPlot};
