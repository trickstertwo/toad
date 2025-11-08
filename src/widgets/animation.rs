//! Animations and transitions for smooth UI changes
//!
//! Provides easing functions and animation utilities for creating smooth transitions
//! between UI states.
//!
//! # Examples
//!
//! ```
//! use toad::widgets::{Animation, EasingFunction};
//! use std::time::Duration;
//!
//! let mut anim = Animation::new(0.0, 100.0, Duration::from_millis(500))
//!     .with_easing(EasingFunction::EaseInOut);
//!
//! // Update animation
//! anim.start();
//! anim.tick(Duration::from_millis(250));
//! let current = anim.current_value();
//! assert!(current > 0.0 && current < 100.0);
//! ```

use std::time::Duration;

/// Easing functions for smooth animations
///
/// # Examples
///
/// ```
/// use toad::widgets::EasingFunction;
///
/// let linear = EasingFunction::Linear;
/// assert_eq!(linear.apply(0.5), 0.5);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EasingFunction {
    /// Linear interpolation
    #[default]
    Linear,
    /// Ease in (accelerate)
    EaseIn,
    /// Ease out (decelerate)
    EaseOut,
    /// Ease in and out (accelerate then decelerate)
    EaseInOut,
    /// Ease in cubic
    EaseInCubic,
    /// Ease out cubic
    EaseOutCubic,
}

impl EasingFunction {
    /// Apply easing to a normalized value (0.0 to 1.0)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::EasingFunction;
    ///
    /// let ease = EasingFunction::Linear;
    /// assert_eq!(ease.apply(0.0), 0.0);
    /// assert_eq!(ease.apply(0.5), 0.5);
    /// assert_eq!(ease.apply(1.0), 1.0);
    /// ```
    pub fn apply(&self, t: f64) -> f64 {
        let t = t.clamp(0.0, 1.0);
        match self {
            EasingFunction::Linear => t,
            EasingFunction::EaseIn => t * t,
            EasingFunction::EaseOut => t * (2.0 - t),
            EasingFunction::EaseInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    -1.0 + (4.0 - 2.0 * t) * t
                }
            }
            EasingFunction::EaseInCubic => t * t * t,
            EasingFunction::EaseOutCubic => {
                let t = t - 1.0;
                t * t * t + 1.0
            }
        }
    }
}

/// Animation state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationState {
    /// Animation not started
    Idle,
    /// Animation in progress
    Running,
    /// Animation completed
    Complete,
}

/// Animation for smooth value transitions
///
/// # Examples
///
/// ```
/// use toad::widgets::Animation;
/// use std::time::Duration;
///
/// let mut anim = Animation::new(0.0, 100.0, Duration::from_secs(1));
/// assert_eq!(anim.current_value(), 0.0);
///
/// anim.start();
/// anim.tick(Duration::from_millis(500));
/// let mid = anim.current_value();
/// assert!(mid > 0.0 && mid < 100.0);
/// ```
#[derive(Debug, Clone)]
pub struct Animation {
    /// Start value
    start: f64,
    /// End value
    end: f64,
    /// Animation duration
    duration: Duration,
    /// Elapsed time
    elapsed: Duration,
    /// Easing function
    easing: EasingFunction,
    /// Current state
    state: AnimationState,
    /// Whether to loop
    looping: bool,
    /// Whether to reverse on loop
    reverse: bool,
}

impl Animation {
    /// Create a new animation
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Animation;
    /// use std::time::Duration;
    ///
    /// let anim = Animation::new(0.0, 100.0, Duration::from_secs(1));
    /// assert_eq!(anim.start_value(), 0.0);
    /// assert_eq!(anim.end_value(), 100.0);
    /// ```
    pub fn new(start: f64, end: f64, duration: Duration) -> Self {
        Self {
            start,
            end,
            duration,
            elapsed: Duration::ZERO,
            easing: EasingFunction::default(),
            state: AnimationState::Idle,
            looping: false,
            reverse: false,
        }
    }

    /// Set easing function
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{Animation, EasingFunction};
    /// use std::time::Duration;
    ///
    /// let anim = Animation::new(0.0, 100.0, Duration::from_secs(1))
    ///     .with_easing(EasingFunction::EaseInOut);
    /// ```
    pub fn with_easing(mut self, easing: EasingFunction) -> Self {
        self.easing = easing;
        self
    }

    /// Enable looping
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Animation;
    /// use std::time::Duration;
    ///
    /// let anim = Animation::new(0.0, 100.0, Duration::from_secs(1))
    ///     .with_loop(true);
    /// ```
    pub fn with_loop(mut self, looping: bool) -> Self {
        self.looping = looping;
        self
    }

    /// Enable reverse on loop
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Animation;
    /// use std::time::Duration;
    ///
    /// let anim = Animation::new(0.0, 100.0, Duration::from_secs(1))
    ///     .with_loop(true)
    ///     .with_reverse(true);
    /// ```
    pub fn with_reverse(mut self, reverse: bool) -> Self {
        self.reverse = reverse;
        self
    }

    /// Start the animation
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{Animation, AnimationState};
    /// use std::time::Duration;
    ///
    /// let mut anim = Animation::new(0.0, 100.0, Duration::from_secs(1));
    /// anim.start();
    /// assert_eq!(anim.state(), AnimationState::Running);
    /// ```
    pub fn start(&mut self) {
        self.state = AnimationState::Running;
        self.elapsed = Duration::ZERO;
    }

    /// Pause the animation
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{Animation, AnimationState};
    /// use std::time::Duration;
    ///
    /// let mut anim = Animation::new(0.0, 100.0, Duration::from_secs(1));
    /// anim.start();
    /// anim.pause();
    /// assert_eq!(anim.state(), AnimationState::Idle);
    /// ```
    pub fn pause(&mut self) {
        if self.state == AnimationState::Running {
            self.state = AnimationState::Idle;
        }
    }

    /// Reset the animation
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{Animation, AnimationState};
    /// use std::time::Duration;
    ///
    /// let mut anim = Animation::new(0.0, 100.0, Duration::from_secs(1));
    /// anim.start();
    /// anim.tick(Duration::from_millis(500));
    /// anim.reset();
    /// assert_eq!(anim.state(), AnimationState::Idle);
    /// assert_eq!(anim.current_value(), 0.0);
    /// ```
    pub fn reset(&mut self) {
        self.state = AnimationState::Idle;
        self.elapsed = Duration::ZERO;
    }

    /// Update animation by elapsed time
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Animation;
    /// use std::time::Duration;
    ///
    /// let mut anim = Animation::new(0.0, 100.0, Duration::from_secs(1));
    /// anim.start();
    /// anim.tick(Duration::from_millis(500));
    /// assert!(anim.current_value() > 0.0);
    /// ```
    pub fn tick(&mut self, delta: Duration) {
        if self.state != AnimationState::Running {
            return;
        }

        self.elapsed += delta;

        if self.elapsed >= self.duration {
            if self.looping {
                self.elapsed = Duration::ZERO;
                if self.reverse {
                    std::mem::swap(&mut self.start, &mut self.end);
                }
            } else {
                self.elapsed = self.duration;
                self.state = AnimationState::Complete;
            }
        }
    }

    /// Get current animation value
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Animation;
    /// use std::time::Duration;
    ///
    /// let mut anim = Animation::new(0.0, 100.0, Duration::from_secs(1));
    /// assert_eq!(anim.current_value(), 0.0);
    /// ```
    pub fn current_value(&self) -> f64 {
        let progress = if self.duration.as_secs_f64() > 0.0 {
            (self.elapsed.as_secs_f64() / self.duration.as_secs_f64()).min(1.0)
        } else {
            1.0
        };

        let eased = self.easing.apply(progress);
        self.start + (self.end - self.start) * eased
    }

    /// Get animation progress (0.0 to 1.0)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Animation;
    /// use std::time::Duration;
    ///
    /// let mut anim = Animation::new(0.0, 100.0, Duration::from_secs(1));
    /// anim.start();
    /// anim.tick(Duration::from_millis(500));
    /// assert!((anim.progress() - 0.5).abs() < 0.01);
    /// ```
    pub fn progress(&self) -> f64 {
        if self.duration.as_secs_f64() > 0.0 {
            (self.elapsed.as_secs_f64() / self.duration.as_secs_f64()).min(1.0)
        } else {
            1.0
        }
    }

    /// Get animation state
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{Animation, AnimationState};
    /// use std::time::Duration;
    ///
    /// let anim = Animation::new(0.0, 100.0, Duration::from_secs(1));
    /// assert_eq!(anim.state(), AnimationState::Idle);
    /// ```
    pub fn state(&self) -> AnimationState {
        self.state
    }

    /// Check if animation is complete
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Animation;
    /// use std::time::Duration;
    ///
    /// let mut anim = Animation::new(0.0, 100.0, Duration::from_millis(100));
    /// anim.start();
    /// assert!(!anim.is_complete());
    ///
    /// anim.tick(Duration::from_millis(100));
    /// assert!(anim.is_complete());
    /// ```
    pub fn is_complete(&self) -> bool {
        self.state == AnimationState::Complete
    }

    /// Get start value
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Animation;
    /// use std::time::Duration;
    ///
    /// let anim = Animation::new(10.0, 100.0, Duration::from_secs(1));
    /// assert_eq!(anim.start_value(), 10.0);
    /// ```
    pub fn start_value(&self) -> f64 {
        self.start
    }

    /// Get end value
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Animation;
    /// use std::time::Duration;
    ///
    /// let anim = Animation::new(0.0, 100.0, Duration::from_secs(1));
    /// assert_eq!(anim.end_value(), 100.0);
    /// ```
    pub fn end_value(&self) -> f64 {
        self.end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_easing_linear() {
        let ease = EasingFunction::Linear;
        assert_eq!(ease.apply(0.0), 0.0);
        assert_eq!(ease.apply(0.5), 0.5);
        assert_eq!(ease.apply(1.0), 1.0);
    }

    #[test]
    fn test_easing_ease_in() {
        let ease = EasingFunction::EaseIn;
        assert_eq!(ease.apply(0.0), 0.0);
        assert!(ease.apply(0.5) < 0.5);
        assert_eq!(ease.apply(1.0), 1.0);
    }

    #[test]
    fn test_easing_ease_out() {
        let ease = EasingFunction::EaseOut;
        assert_eq!(ease.apply(0.0), 0.0);
        assert!(ease.apply(0.5) > 0.5);
        assert_eq!(ease.apply(1.0), 1.0);
    }

    #[test]
    fn test_easing_clamp() {
        let ease = EasingFunction::Linear;
        assert_eq!(ease.apply(-0.5), 0.0);
        assert_eq!(ease.apply(1.5), 1.0);
    }

    #[test]
    fn test_animation_new() {
        let anim = Animation::new(0.0, 100.0, Duration::from_secs(1));
        assert_eq!(anim.start_value(), 0.0);
        assert_eq!(anim.end_value(), 100.0);
        assert_eq!(anim.state(), AnimationState::Idle);
    }

    #[test]
    fn test_animation_with_easing() {
        let anim = Animation::new(0.0, 100.0, Duration::from_secs(1))
            .with_easing(EasingFunction::EaseIn);
        assert_eq!(anim.easing, EasingFunction::EaseIn);
    }

    #[test]
    fn test_animation_with_loop() {
        let anim = Animation::new(0.0, 100.0, Duration::from_secs(1))
            .with_loop(true);
        assert!(anim.looping);
    }

    #[test]
    fn test_animation_start() {
        let mut anim = Animation::new(0.0, 100.0, Duration::from_secs(1));
        anim.start();
        assert_eq!(anim.state(), AnimationState::Running);
    }

    #[test]
    fn test_animation_pause() {
        let mut anim = Animation::new(0.0, 100.0, Duration::from_secs(1));
        anim.start();
        anim.pause();
        assert_eq!(anim.state(), AnimationState::Idle);
    }

    #[test]
    fn test_animation_reset() {
        let mut anim = Animation::new(0.0, 100.0, Duration::from_secs(1));
        anim.start();
        anim.tick(Duration::from_millis(500));
        anim.reset();
        assert_eq!(anim.state(), AnimationState::Idle);
        assert_eq!(anim.current_value(), 0.0);
    }

    #[test]
    fn test_animation_tick() {
        let mut anim = Animation::new(0.0, 100.0, Duration::from_secs(1));
        anim.start();
        anim.tick(Duration::from_millis(500));

        let value = anim.current_value();
        assert!(value > 0.0 && value < 100.0);
    }

    #[test]
    fn test_animation_complete() {
        let mut anim = Animation::new(0.0, 100.0, Duration::from_millis(100));
        anim.start();
        anim.tick(Duration::from_millis(100));

        assert!(anim.is_complete());
        assert_eq!(anim.state(), AnimationState::Complete);
        assert_eq!(anim.current_value(), 100.0);
    }

    #[test]
    fn test_animation_loop() {
        let mut anim = Animation::new(0.0, 100.0, Duration::from_millis(100))
            .with_loop(true);
        anim.start();
        anim.tick(Duration::from_millis(150));

        assert!(!anim.is_complete());
        assert_eq!(anim.state(), AnimationState::Running);
    }

    #[test]
    fn test_animation_progress() {
        let mut anim = Animation::new(0.0, 100.0, Duration::from_secs(1));
        anim.start();
        anim.tick(Duration::from_millis(500));

        assert!((anim.progress() - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_animation_no_tick_when_idle() {
        let mut anim = Animation::new(0.0, 100.0, Duration::from_secs(1));
        anim.tick(Duration::from_millis(500));

        assert_eq!(anim.current_value(), 0.0);
    }

    #[test]
    fn test_animation_zero_duration() {
        let mut anim = Animation::new(0.0, 100.0, Duration::ZERO);
        anim.start();
        anim.tick(Duration::from_millis(1));

        assert_eq!(anim.current_value(), 100.0);
    }

    #[test]
    fn test_animation_builder_pattern() {
        let anim = Animation::new(0.0, 100.0, Duration::from_secs(1))
            .with_easing(EasingFunction::EaseInOut)
            .with_loop(true)
            .with_reverse(true);

        assert_eq!(anim.easing, EasingFunction::EaseInOut);
        assert!(anim.looping);
        assert!(anim.reverse);
    }

    #[test]
    fn test_easing_default() {
        let ease = EasingFunction::default();
        assert_eq!(ease, EasingFunction::Linear);
    }
}
