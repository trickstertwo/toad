// TOAD - Terminal-Oriented Autonomous Developer
// Milestone 0: Infrastructure and Evaluation Framework

pub mod config;
pub mod evaluation;
pub mod metrics;
pub mod stats;

// Re-exports for convenience
pub use config::{FeatureFlags, ToadConfig};
pub use evaluation::{Task, TaskResult, EvaluationHarness};
pub use metrics::{Metrics, MetricsCollector};
pub use stats::{ComparisonResult, StatisticalTest};

/// Current TOAD version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
