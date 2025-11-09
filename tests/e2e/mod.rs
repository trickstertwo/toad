//! End-to-end (E2E) tests for TOAD
//!
//! These tests verify complete workflows by spawning the actual binary
//! and testing real-world usage scenarios.
//!
//! # Test Categories
//!
//! - **CLI Evaluation**: Full evaluation runs via command-line interface
//! - **TUI Simulation**: Terminal UI interaction workflows (requires rexpect)
//! - **Dataset Integration**: Real SWE-bench dataset processing
//!
//! # Running E2E Tests
//!
//! ```bash
//! # Run all e2e tests
//! cargo test --test e2e
//!
//! # Run specific e2e test
//! cargo test --test e2e cli_eval_synthetic
//! ```

mod cli_eval;

// TUI simulation tests require rexpect crate (not yet implemented)
// mod tui_simulation;

// Full dataset tests are expensive, marked with #[ignore]
// mod swebench_e2e;
