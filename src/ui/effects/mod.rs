//! UI Effects
//!
//! Visual effects and animations for terminal UI.
//!
//! # Modules
//!
//! - [`animations`]: Animation system with easing functions and state management
//! - [`gradient`]: Color gradients for terminal UI

pub mod animations;
pub mod gradient;

// Re-export public types
pub use animations::{Animation, AnimationState, EasingFunction, TransitionManager};
pub use gradient::{ColorStop, Gradient, GradientDirection, GradientType, Gradients};
