// TOAD - Terminal-Oriented Autonomous Developer
// Milestone 0: Infrastructure and Evaluation Framework

pub mod agent;
pub mod config;
pub mod evaluation;
pub mod llm;
pub mod metrics;
pub mod stats;
pub mod tools;

// Re-exports for convenience
pub use config::{FeatureFlags, ToadConfig};
pub use evaluation::{EvaluationHarness, Task, TaskResult};
pub use metrics::{Metrics, MetricsCollector};
pub use stats::{ComparisonResult, StatisticalTest};

/// Current TOAD version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
