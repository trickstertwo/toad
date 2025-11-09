//! Application state types
//!
//! This module contains type definitions for application state management,
//! following the Elm Architecture pattern (Model-Update-View).
//!
//! # Type Overview
//!
//! - [`AppScreen`]: Enum representing different screens/modes in the TUI
//! - [`EvaluationState`]: State of a running or completed evaluation

/// Different screens/modes the application can be in
///
/// The application can be in one of four states:
/// - Welcome: Initial screen with logo and tips
/// - TrustDialog: Directory trust confirmation
/// - Main: Main application interface
/// - Evaluation: Evaluation running screen with progress
///
/// # Examples
///
/// ```
/// use toad::core::app_state::AppScreen;
///
/// let screen = AppScreen::Welcome;
/// assert_eq!(screen, AppScreen::Welcome);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppScreen {
    /// Initial welcome screen with logo and tips
    Welcome,
    /// Trust confirmation dialog for the current directory
    TrustDialog,
    /// Main application interface
    Main,
    /// Evaluation running screen
    Evaluation,
}

/// State of a running or completed evaluation
///
/// Tracks the lifecycle of an evaluation task from start to completion or failure.
///
/// # Fields
///
/// - `handle`: Handle to the running evaluation task (if still running)
/// - `progress`: Latest progress information (task count, tokens, cost, etc.)
/// - `results`: Final evaluation results (if completed successfully)
/// - `error`: Error message (if evaluation failed)
///
/// # State Transitions
///
/// 1. Running: `handle: Some(_)`, `progress: Some(_)`, `results: None`, `error: None`
/// 2. Completed: `handle: None`, `progress: Some(_)`, `results: Some(_)`, `error: None`
/// 3. Failed: `handle: None`, `progress: Some(_)`, `results: None`, `error: Some(_)`
/// 4. Cancelled: `handle: None`, `progress: Some(_)`, `results: None`, `error: None`
///
/// # Examples
///
/// ```
/// use toad::core::app_state::EvaluationState;
///
/// let state = EvaluationState {
///     handle: None,
///     progress: None,
///     results: None,
///     error: None,
/// };
/// ```
#[derive(Debug)]
pub struct EvaluationState {
    /// Handle to the running evaluation (if still running)
    pub handle: Option<crate::ai::eval_runner::EvaluationHandle>,

    /// Latest progress information
    pub progress: Option<crate::core::event::EvaluationProgress>,

    /// Final results (if completed)
    pub results: Option<crate::ai::evaluation::EvaluationResults>,

    /// Error message (if failed)
    pub error: Option<String>,
}
