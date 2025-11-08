/// Animation and transition system for smooth UI effects
///
/// Provides easing functions and animation state management
///
/// # Examples
///
/// ```
/// use toad::animations::{Animation, EasingFunction};
///
/// let anim = Animation::new(0.0, 100.0, 500);
/// assert_eq!(anim.duration(), 500);
/// ```

use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

/// Easing function type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EasingFunction {
    /// Linear interpolation (constant speed)
    Linear,
    /// Ease in (slow start)
    EaseIn,
    /// Ease out (slow end)
    EaseOut,
    /// Ease in-out (slow start and end)
    EaseInOut,
    /// Cubic ease in
    CubicIn,
    /// Cubic ease out
    CubicOut,
    /// Elastic ease out (bounce)
    ElasticOut,
    /// Bounce ease out
    BounceOut,
}

impl EasingFunction {
    /// Apply easing function to normalized time (0.0 to 1.0)
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
            EasingFunction::CubicIn => t * t * t,
            EasingFunction::CubicOut => {
                let t1 = t - 1.0;
                t1 * t1 * t1 + 1.0
            }
            EasingFunction::ElasticOut => {
                if t == 0.0 || t == 1.0 {
                    t
                } else {
                    let p = 0.3;
                    let s = p / 4.0;
                    2.0f64.powf(-10.0 * t) * ((t - s) * (2.0 * std::f64::consts::PI) / p).sin() + 1.0
                }
            }
            EasingFunction::BounceOut => {
                if t < 1.0 / 2.75 {
                    7.5625 * t * t
                } else if t < 2.0 / 2.75 {
                    let t2 = t - 1.5 / 2.75;
                    7.5625 * t2 * t2 + 0.75
                } else if t < 2.5 / 2.75 {
                    let t2 = t - 2.25 / 2.75;
                    7.5625 * t2 * t2 + 0.9375
                } else {
                    let t2 = t - 2.625 / 2.75;
                    7.5625 * t2 * t2 + 0.984375
                }
            }
        }
    }
}

/// Animation state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnimationState {
    /// Not started
    Idle,
    /// Currently running
    Running,
    /// Completed
    Completed,
    /// Paused
    Paused,
}

/// Animation for smooth value transitions
#[derive(Debug, Clone)]
pub struct Animation {
    /// Start value
    start: f64,
    /// End value
    end: f64,
    /// Duration in milliseconds
    duration_ms: u64,
    /// Easing function
    easing: EasingFunction,
    /// Animation state
    state: AnimationState,
    /// Start time
    #[allow(dead_code)]
    start_time: Option<Instant>,
    /// Elapsed time (for pause/resume)
    elapsed_ms: u64,
}

impl Animation {
    /// Create a new animation
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::animations::Animation;
    ///
    /// let anim = Animation::new(0.0, 100.0, 1000);
    /// assert_eq!(anim.duration(), 1000);
    /// ```
    pub fn new(start: f64, end: f64, duration_ms: u64) -> Self {
        Self {
            start,
            end,
            duration_ms,
            easing: EasingFunction::Linear,
            state: AnimationState::Idle,
            start_time: None,
            elapsed_ms: 0,
        }
    }

    /// Set easing function
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::animations::{Animation, EasingFunction};
    ///
    /// let anim = Animation::new(0.0, 100.0, 1000)
    ///     .with_easing(EasingFunction::EaseInOut);
    /// ```
    pub fn with_easing(mut self, easing: EasingFunction) -> Self {
        self.easing = easing;
        self
    }

    /// Get duration in milliseconds
    pub fn duration(&self) -> u64 {
        self.duration_ms
    }

    /// Get current state
    pub fn state(&self) -> AnimationState {
        self.state
    }

    /// Check if animation is running
    pub fn is_running(&self) -> bool {
        self.state == AnimationState::Running
    }

    /// Check if animation is completed
    pub fn is_completed(&self) -> bool {
        self.state == AnimationState::Completed
    }

    /// Start the animation
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::animations::Animation;
    ///
    /// let mut anim = Animation::new(0.0, 100.0, 1000);
    /// anim.start();
    /// assert!(anim.is_running());
    /// ```
    pub fn start(&mut self) {
        self.state = AnimationState::Running;
        self.start_time = Some(Instant::now());
        self.elapsed_ms = 0;
    }

    /// Pause the animation
    pub fn pause(&mut self) {
        if self.state == AnimationState::Running {
            self.state = AnimationState::Paused;
        }
    }

    /// Resume the animation
    pub fn resume(&mut self) {
        if self.state == AnimationState::Paused {
            self.state = AnimationState::Running;
            self.start_time = Some(Instant::now());
        }
    }

    /// Reset the animation
    pub fn reset(&mut self) {
        self.state = AnimationState::Idle;
        self.start_time = None;
        self.elapsed_ms = 0;
    }

    /// Update animation and get current value
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::animations::Animation;
    /// use std::time::Duration;
    ///
    /// let mut anim = Animation::new(0.0, 100.0, 1000);
    /// anim.start();
    ///
    /// let value = anim.update(Duration::from_millis(500));
    /// // Value should be around 50.0 (halfway with linear easing)
    /// ```
    pub fn update(&mut self, delta: Duration) -> f64 {
        match self.state {
            AnimationState::Idle => self.start,
            AnimationState::Completed => self.end,
            AnimationState::Paused => {
                // Return current value without updating
                let t = self.elapsed_ms as f64 / self.duration_ms as f64;
                let eased_t = self.easing.apply(t);
                self.start + (self.end - self.start) * eased_t
            }
            AnimationState::Running => {
                self.elapsed_ms += delta.as_millis() as u64;

                if self.elapsed_ms >= self.duration_ms {
                    self.state = AnimationState::Completed;
                    self.end
                } else {
                    let t = self.elapsed_ms as f64 / self.duration_ms as f64;
                    let eased_t = self.easing.apply(t);
                    self.start + (self.end - self.start) * eased_t
                }
            }
        }
    }

    /// Get progress (0.0 to 1.0)
    pub fn progress(&self) -> f64 {
        if self.duration_ms == 0 {
            1.0
        } else {
            (self.elapsed_ms as f64 / self.duration_ms as f64).min(1.0)
        }
    }
}

/// Transition manager for coordinating multiple animations
#[derive(Debug, Clone, Default)]
pub struct TransitionManager {
    /// Active transitions
    transitions: Vec<(String, Animation)>,
}

impl TransitionManager {
    /// Create a new transition manager
    pub fn new() -> Self {
        Self {
            transitions: Vec::new(),
        }
    }

    /// Add a transition
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::animations::{TransitionManager, Animation};
    ///
    /// let mut manager = TransitionManager::new();
    /// let anim = Animation::new(0.0, 100.0, 1000);
    /// manager.add("opacity", anim);
    /// assert_eq!(manager.count(), 1);
    /// ```
    pub fn add<S: Into<String>>(&mut self, name: S, mut animation: Animation) {
        animation.start();
        self.transitions.push((name.into(), animation));
    }

    /// Get a transition by name
    pub fn get(&self, name: &str) -> Option<&Animation> {
        self.transitions
            .iter()
            .find(|(n, _)| n == name)
            .map(|(_, a)| a)
    }

    /// Get a mutable transition by name
    pub fn get_mut(&mut self, name: &str) -> Option<&mut Animation> {
        self.transitions
            .iter_mut()
            .find(|(n, _)| n == name)
            .map(|(_, a)| a)
    }

    /// Update all transitions
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::animations::{TransitionManager, Animation};
    /// use std::time::Duration;
    ///
    /// let mut manager = TransitionManager::new();
    /// manager.add("fade", Animation::new(0.0, 1.0, 500));
    ///
    /// manager.update(Duration::from_millis(100));
    /// ```
    pub fn update(&mut self, delta: Duration) {
        for (_, animation) in &mut self.transitions {
            animation.update(delta);
        }
    }

    /// Remove completed transitions
    pub fn cleanup(&mut self) {
        self.transitions.retain(|(_, a)| !a.is_completed());
    }

    /// Get number of active transitions
    pub fn count(&self) -> usize {
        self.transitions.len()
    }

    /// Check if any transitions are running
    pub fn has_active(&self) -> bool {
        self.transitions.iter().any(|(_, a)| a.is_running())
    }

    /// Clear all transitions
    pub fn clear(&mut self) {
        self.transitions.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_easing_linear() {
        let easing = EasingFunction::Linear;
        assert_eq!(easing.apply(0.0), 0.0);
        assert_eq!(easing.apply(0.5), 0.5);
        assert_eq!(easing.apply(1.0), 1.0);
    }

    #[test]
    fn test_easing_ease_in() {
        let easing = EasingFunction::EaseIn;
        assert_eq!(easing.apply(0.0), 0.0);
        assert!(easing.apply(0.5) < 0.5); // Slower at start
        assert_eq!(easing.apply(1.0), 1.0);
    }

    #[test]
    fn test_easing_ease_out() {
        let easing = EasingFunction::EaseOut;
        assert_eq!(easing.apply(0.0), 0.0);
        assert!(easing.apply(0.5) > 0.5); // Faster at start
        assert_eq!(easing.apply(1.0), 1.0);
    }

    #[test]
    fn test_animation_creation() {
        let anim = Animation::new(0.0, 100.0, 1000);
        assert_eq!(anim.duration(), 1000);
        assert_eq!(anim.state(), AnimationState::Idle);
    }

    #[test]
    fn test_animation_with_easing() {
        let anim = Animation::new(0.0, 100.0, 1000)
            .with_easing(EasingFunction::EaseInOut);
        assert_eq!(anim.easing, EasingFunction::EaseInOut);
    }

    #[test]
    fn test_animation_start() {
        let mut anim = Animation::new(0.0, 100.0, 1000);
        anim.start();
        assert!(anim.is_running());
        assert!(!anim.is_completed());
    }

    #[test]
    fn test_animation_pause_resume() {
        let mut anim = Animation::new(0.0, 100.0, 1000);
        anim.start();
        anim.pause();
        assert_eq!(anim.state(), AnimationState::Paused);

        anim.resume();
        assert!(anim.is_running());
    }

    #[test]
    fn test_animation_reset() {
        let mut anim = Animation::new(0.0, 100.0, 1000);
        anim.start();
        anim.reset();
        assert_eq!(anim.state(), AnimationState::Idle);
    }

    #[test]
    fn test_animation_update() {
        let mut anim = Animation::new(0.0, 100.0, 1000);
        anim.start();

        let value = anim.update(Duration::from_millis(500));
        assert!((value - 50.0).abs() < 1.0); // Should be around 50

        let value = anim.update(Duration::from_millis(500));
        assert_eq!(value, 100.0);
        assert!(anim.is_completed());
    }

    #[test]
    fn test_animation_progress() {
        let mut anim = Animation::new(0.0, 100.0, 1000);
        anim.start();

        anim.update(Duration::from_millis(250));
        assert!((anim.progress() - 0.25).abs() < 0.01);

        anim.update(Duration::from_millis(750));
        assert_eq!(anim.progress(), 1.0);
    }

    #[test]
    fn test_transition_manager_creation() {
        let manager = TransitionManager::new();
        assert_eq!(manager.count(), 0);
    }

    #[test]
    fn test_transition_manager_add() {
        let mut manager = TransitionManager::new();
        let anim = Animation::new(0.0, 100.0, 1000);
        manager.add("test", anim);
        assert_eq!(manager.count(), 1);
    }

    #[test]
    fn test_transition_manager_get() {
        let mut manager = TransitionManager::new();
        let anim = Animation::new(0.0, 100.0, 1000);
        manager.add("test", anim);

        let retrieved = manager.get("test");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().duration(), 1000);
    }

    #[test]
    fn test_transition_manager_update() {
        let mut manager = TransitionManager::new();
        manager.add("test", Animation::new(0.0, 100.0, 1000));

        manager.update(Duration::from_millis(500));

        let anim = manager.get("test").unwrap();
        assert!((anim.progress() - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_transition_manager_cleanup() {
        let mut manager = TransitionManager::new();
        manager.add("test", Animation::new(0.0, 100.0, 100));

        manager.update(Duration::from_millis(200));
        manager.cleanup();

        assert_eq!(manager.count(), 0);
    }

    #[test]
    fn test_transition_manager_has_active() {
        let mut manager = TransitionManager::new();
        assert!(!manager.has_active());

        manager.add("test", Animation::new(0.0, 100.0, 1000));
        assert!(manager.has_active());
    }

    #[test]
    fn test_transition_manager_clear() {
        let mut manager = TransitionManager::new();
        manager.add("test1", Animation::new(0.0, 100.0, 1000));
        manager.add("test2", Animation::new(0.0, 100.0, 1000));

        manager.clear();
        assert_eq!(manager.count(), 0);
    }

    #[test]
    fn test_easing_clamp() {
        let easing = EasingFunction::Linear;
        assert_eq!(easing.apply(-0.5), 0.0);
        assert_eq!(easing.apply(1.5), 1.0);
    }
}
