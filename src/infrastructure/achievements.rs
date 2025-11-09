//! Achievement and gamification system
//!
//! Provides badges, streaks, leaderboards, and progress tracking to boost
//! engagement and motivation for completing tasks and reaching goals.
//!
//! # Examples
//!
//! ```
//! use toad::infrastructure::achievements::{AchievementSystem, AchievementType};
//!
//! let mut system = AchievementSystem::new();
//! system.record_task_completed("user-1");
//! let unlocked = system.check_achievements("user-1");
//! ```

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Achievement type/category
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AchievementType {
    /// Task completion milestones
    TaskCompletion,
    /// Streak-based achievements
    Streak,
    /// Speed/efficiency achievements
    Speed,
    /// Collaboration achievements
    Collaboration,
    /// Quality achievements
    Quality,
    /// Special/seasonal achievements
    Special,
}

impl AchievementType {
    /// Get all achievement types
    pub fn all() -> &'static [AchievementType] {
        &[
            AchievementType::TaskCompletion,
            AchievementType::Streak,
            AchievementType::Speed,
            AchievementType::Collaboration,
            AchievementType::Quality,
            AchievementType::Special,
        ]
    }

    /// Get display name
    pub fn name(&self) -> &'static str {
        match self {
            AchievementType::TaskCompletion => "Task Completion",
            AchievementType::Streak => "Streak",
            AchievementType::Speed => "Speed",
            AchievementType::Collaboration => "Collaboration",
            AchievementType::Quality => "Quality",
            AchievementType::Special => "Special",
        }
    }
}

/// Achievement tier/rarity
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum AchievementTier {
    /// Common/bronze achievements
    Bronze,
    /// Uncommon/silver achievements
    Silver,
    /// Rare/gold achievements
    Gold,
    /// Epic/platinum achievements
    Platinum,
    /// Legendary/diamond achievements
    Diamond,
}

impl AchievementTier {
    /// Get display name
    pub fn name(&self) -> &'static str {
        match self {
            AchievementTier::Bronze => "Bronze",
            AchievementTier::Silver => "Silver",
            AchievementTier::Gold => "Gold",
            AchievementTier::Platinum => "Platinum",
            AchievementTier::Diamond => "Diamond",
        }
    }

    /// Get color for display
    pub fn color(&self) -> &'static str {
        match self {
            AchievementTier::Bronze => "#CD7F32",
            AchievementTier::Silver => "#C0C0C0",
            AchievementTier::Gold => "#FFD700",
            AchievementTier::Platinum => "#E5E4E2",
            AchievementTier::Diamond => "#B9F2FF",
        }
    }

    /// Get points value
    pub fn points(&self) -> u32 {
        match self {
            AchievementTier::Bronze => 10,
            AchievementTier::Silver => 25,
            AchievementTier::Gold => 50,
            AchievementTier::Platinum => 100,
            AchievementTier::Diamond => 250,
        }
    }
}

/// Achievement definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Achievement {
    /// Unique achievement ID
    pub id: String,
    /// Display name
    pub name: String,
    /// Description
    pub description: String,
    /// Achievement type
    pub achievement_type: AchievementType,
    /// Achievement tier
    pub tier: AchievementTier,
    /// Icon/emoji
    pub icon: String,
    /// Points awarded
    pub points: u32,
    /// Requirement threshold
    pub threshold: u32,
    /// Whether this is hidden until unlocked
    pub hidden: bool,
}

impl Achievement {
    /// Create a new achievement
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        description: impl Into<String>,
        achievement_type: AchievementType,
        tier: AchievementTier,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: description.into(),
            achievement_type,
            tier,
            icon: "🏆".to_string(),
            points: tier.points(),
            threshold: 1,
            hidden: false,
        }
    }

    /// Set icon
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = icon.into();
        self
    }

    /// Set threshold
    pub fn threshold(mut self, threshold: u32) -> Self {
        self.threshold = threshold;
        self
    }

    /// Set custom points
    pub fn points(mut self, points: u32) -> Self {
        self.points = points;
        self
    }

    /// Set as hidden
    pub fn hidden(mut self, hidden: bool) -> Self {
        self.hidden = hidden;
        self
    }
}

/// Unlocked achievement record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnlockedAchievement {
    /// Achievement ID
    pub achievement_id: String,
    /// User ID
    pub user_id: String,
    /// When unlocked
    pub unlocked_at: DateTime<Utc>,
    /// Progress when unlocked
    pub progress: u32,
}

/// Streak tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Streak {
    /// User ID
    pub user_id: String,
    /// Current streak count (consecutive days)
    pub current: u32,
    /// Longest streak ever
    pub longest: u32,
    /// Last activity date
    pub last_activity: DateTime<Utc>,
    /// Streak start date
    pub started_at: DateTime<Utc>,
}

impl Streak {
    /// Create a new streak
    pub fn new(user_id: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            user_id: user_id.into(),
            current: 0,
            longest: 0,
            last_activity: now,
            started_at: now,
        }
    }

    /// Update streak with new activity
    pub fn update(&mut self, now: DateTime<Utc>) {
        let days_diff = (now - self.last_activity).num_days();

        if days_diff == 0 {
            // Same day, no change
            return;
        } else if days_diff == 1 {
            // Consecutive day, increment
            self.current += 1;
            if self.current > self.longest {
                self.longest = self.current;
            }
        } else {
            // Streak broken, restart
            self.current = 1;
            self.started_at = now;
        }

        self.last_activity = now;
    }

    /// Check if streak is active (within last 2 days)
    pub fn is_active(&self) -> bool {
        let days_since = (Utc::now() - self.last_activity).num_days();
        days_since <= 1
    }
}

/// User statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserStats {
    /// Total tasks completed
    pub tasks_completed: u32,
    /// Tasks completed today
    pub tasks_today: u32,
    /// Tasks completed this week
    pub tasks_this_week: u32,
    /// Total points earned
    pub total_points: u32,
    /// Achievements unlocked
    pub achievements_unlocked: u32,
    /// Average tasks per day
    pub avg_tasks_per_day: f32,
}

impl UserStats {
    /// Calculate average tasks per day
    pub fn calculate_average(&mut self, days_active: u32) {
        if days_active > 0 {
            self.avg_tasks_per_day = self.tasks_completed as f32 / days_active as f32;
        }
    }
}

/// Leaderboard entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardEntry {
    /// Rank position
    pub rank: usize,
    /// User ID
    pub user_id: String,
    /// Display name
    pub display_name: String,
    /// Score/points
    pub score: u32,
    /// Tasks completed
    pub tasks_completed: u32,
    /// Current streak
    pub streak: u32,
}

/// Achievement system
///
/// Manages achievements, badges, streaks, and leaderboards for gamification.
#[derive(Debug)]
pub struct AchievementSystem {
    /// All available achievements
    achievements: HashMap<String, Achievement>,
    /// Unlocked achievements by user
    unlocked: HashMap<String, Vec<UnlockedAchievement>>,
    /// User statistics
    stats: HashMap<String, UserStats>,
    /// User streaks
    streaks: HashMap<String, Streak>,
    /// Last activity dates
    last_activity: HashMap<String, DateTime<Utc>>,
}

impl AchievementSystem {
    /// Create a new achievement system
    pub fn new() -> Self {
        let mut system = Self {
            achievements: HashMap::new(),
            unlocked: HashMap::new(),
            stats: HashMap::new(),
            streaks: HashMap::new(),
            last_activity: HashMap::new(),
        };

        system.add_default_achievements();
        system
    }

    /// Add default achievements
    fn add_default_achievements(&mut self) {
        // Task completion achievements
        self.add_achievement(
            Achievement::new(
                "first_task",
                "First Steps",
                "Complete your first task",
                AchievementType::TaskCompletion,
                AchievementTier::Bronze,
            )
            .icon("🎯")
            .threshold(1),
        );

        self.add_achievement(
            Achievement::new(
                "task_10",
                "Getting Started",
                "Complete 10 tasks",
                AchievementType::TaskCompletion,
                AchievementTier::Bronze,
            )
            .icon("📝")
            .threshold(10),
        );

        self.add_achievement(
            Achievement::new(
                "task_50",
                "Productive",
                "Complete 50 tasks",
                AchievementType::TaskCompletion,
                AchievementTier::Silver,
            )
            .icon("⭐")
            .threshold(50),
        );

        self.add_achievement(
            Achievement::new(
                "task_100",
                "Centurion",
                "Complete 100 tasks",
                AchievementType::TaskCompletion,
                AchievementTier::Gold,
            )
            .icon("💯")
            .threshold(100),
        );

        // Streak achievements
        self.add_achievement(
            Achievement::new(
                "streak_3",
                "On a Roll",
                "Maintain a 3-day streak",
                AchievementType::Streak,
                AchievementTier::Bronze,
            )
            .icon("🔥")
            .threshold(3),
        );

        self.add_achievement(
            Achievement::new(
                "streak_7",
                "Week Warrior",
                "Maintain a 7-day streak",
                AchievementType::Streak,
                AchievementTier::Silver,
            )
            .icon("🔥")
            .threshold(7),
        );

        self.add_achievement(
            Achievement::new(
                "streak_30",
                "Unstoppable",
                "Maintain a 30-day streak",
                AchievementType::Streak,
                AchievementTier::Gold,
            )
            .icon("🔥")
            .threshold(30),
        );

        // Speed achievements
        self.add_achievement(
            Achievement::new(
                "early_bird",
                "Early Bird",
                "Complete first task before 9 AM",
                AchievementType::Speed,
                AchievementTier::Bronze,
            )
            .icon("🐦")
            .threshold(1),
        );

        self.add_achievement(
            Achievement::new(
                "sprint_champion",
                "Sprint Champion",
                "Complete 5 tasks in one day",
                AchievementType::Speed,
                AchievementTier::Silver,
            )
            .icon("🏃")
            .threshold(5),
        );
    }

    /// Add an achievement
    pub fn add_achievement(&mut self, achievement: Achievement) {
        self.achievements.insert(achievement.id.clone(), achievement);
    }

    /// Get all achievements
    pub fn all_achievements(&self) -> Vec<&Achievement> {
        self.achievements.values().collect()
    }

    /// Get achievements by type
    pub fn achievements_by_type(&self, achievement_type: AchievementType) -> Vec<&Achievement> {
        self.achievements
            .values()
            .filter(|a| a.achievement_type == achievement_type)
            .collect()
    }

    /// Record a task completion
    pub fn record_task_completed(&mut self, user_id: impl Into<String>) {
        let user_id = user_id.into();
        let now = Utc::now();

        // Update stats
        let stats = self.stats.entry(user_id.clone()).or_insert_with(UserStats::default);
        stats.tasks_completed += 1;
        stats.tasks_today += 1;

        // Update streak
        let streak = self
            .streaks
            .entry(user_id.clone())
            .or_insert_with(|| Streak::new(user_id.clone()));
        streak.update(now);

        self.last_activity.insert(user_id, now);
    }

    /// Check for newly unlocked achievements
    pub fn check_achievements(&mut self, user_id: &str) -> Vec<Achievement> {
        let stats = self.stats.get(user_id).cloned().unwrap_or_default();
        let streak = self.streaks.get(user_id).map(|s| s.current).unwrap_or(0);

        // Collect achievements to unlock (to avoid borrow checker issues)
        let to_unlock: Vec<(String, Achievement)> = self
            .achievements
            .values()
            .filter(|achievement| {
                // Skip if already unlocked
                if self.is_unlocked(user_id, &achievement.id) {
                    return false;
                }

                match achievement.achievement_type {
                    AchievementType::TaskCompletion => {
                        stats.tasks_completed >= achievement.threshold
                    }
                    AchievementType::Streak => streak >= achievement.threshold,
                    _ => false, // Other types require specific logic
                }
            })
            .map(|a| (a.id.clone(), a.clone()))
            .collect();

        // Now unlock them
        let mut newly_unlocked = Vec::new();
        for (id, achievement) in to_unlock {
            self.unlock_achievement(user_id, &id, stats.tasks_completed);
            newly_unlocked.push(achievement);
        }

        newly_unlocked
    }

    /// Unlock an achievement for a user
    fn unlock_achievement(&mut self, user_id: &str, achievement_id: &str, progress: u32) {
        let unlocked = UnlockedAchievement {
            achievement_id: achievement_id.to_string(),
            user_id: user_id.to_string(),
            unlocked_at: Utc::now(),
            progress,
        };

        self.unlocked
            .entry(user_id.to_string())
            .or_insert_with(Vec::new)
            .push(unlocked);

        // Update stats
        if let Some(stats) = self.stats.get_mut(user_id) {
            stats.achievements_unlocked += 1;
            if let Some(achievement) = self.achievements.get(achievement_id) {
                stats.total_points += achievement.points;
            }
        }
    }

    /// Check if user has unlocked achievement
    pub fn is_unlocked(&self, user_id: &str, achievement_id: &str) -> bool {
        self.unlocked
            .get(user_id)
            .map(|unlocked| unlocked.iter().any(|u| u.achievement_id == achievement_id))
            .unwrap_or(false)
    }

    /// Get user's unlocked achievements
    pub fn user_achievements(&self, user_id: &str) -> Vec<&UnlockedAchievement> {
        self.unlocked
            .get(user_id)
            .map(|unlocked| unlocked.iter().collect())
            .unwrap_or_default()
    }

    /// Get user stats
    pub fn user_stats(&self, user_id: &str) -> UserStats {
        self.stats.get(user_id).cloned().unwrap_or_default()
    }

    /// Get user streak
    pub fn user_streak(&self, user_id: &str) -> Option<&Streak> {
        self.streaks.get(user_id)
    }

    /// Generate leaderboard
    pub fn leaderboard(&self, limit: usize) -> Vec<LeaderboardEntry> {
        let mut entries: Vec<LeaderboardEntry> = self
            .stats
            .iter()
            .map(|(user_id, stats)| {
                let streak = self
                    .streaks
                    .get(user_id)
                    .map(|s| s.current)
                    .unwrap_or(0);

                LeaderboardEntry {
                    rank: 0,
                    user_id: user_id.clone(),
                    display_name: user_id.clone(),
                    score: stats.total_points,
                    tasks_completed: stats.tasks_completed,
                    streak,
                }
            })
            .collect();

        // Sort by score descending
        entries.sort_by(|a, b| b.score.cmp(&a.score));

        // Assign ranks
        for (i, entry) in entries.iter_mut().enumerate() {
            entry.rank = i + 1;
        }

        // Limit results
        entries.truncate(limit);
        entries
    }

    /// Get achievement progress
    pub fn achievement_progress(&self, user_id: &str, achievement_id: &str) -> Option<f32> {
        let achievement = self.achievements.get(achievement_id)?;
        let stats = self.stats.get(user_id)?;

        let current = match achievement.achievement_type {
            AchievementType::TaskCompletion => stats.tasks_completed,
            AchievementType::Streak => self
                .streaks
                .get(user_id)
                .map(|s| s.current)
                .unwrap_or(0),
            _ => 0,
        };

        Some((current as f32 / achievement.threshold as f32).min(1.0))
    }
}

impl Default for AchievementSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_achievement_type_all() {
        let types = AchievementType::all();
        assert_eq!(types.len(), 6);
    }

    #[test]
    fn test_achievement_type_name() {
        assert_eq!(AchievementType::TaskCompletion.name(), "Task Completion");
        assert_eq!(AchievementType::Streak.name(), "Streak");
    }

    #[test]
    fn test_achievement_tier_name() {
        assert_eq!(AchievementTier::Bronze.name(), "Bronze");
        assert_eq!(AchievementTier::Gold.name(), "Gold");
    }

    #[test]
    fn test_achievement_tier_points() {
        assert_eq!(AchievementTier::Bronze.points(), 10);
        assert_eq!(AchievementTier::Silver.points(), 25);
        assert_eq!(AchievementTier::Gold.points(), 50);
        assert_eq!(AchievementTier::Platinum.points(), 100);
        assert_eq!(AchievementTier::Diamond.points(), 250);
    }

    #[test]
    fn test_achievement_creation() {
        let achievement = Achievement::new(
            "test",
            "Test Achievement",
            "Test description",
            AchievementType::TaskCompletion,
            AchievementTier::Gold,
        );

        assert_eq!(achievement.id, "test");
        assert_eq!(achievement.name, "Test Achievement");
        assert_eq!(achievement.tier, AchievementTier::Gold);
        assert_eq!(achievement.points, 50);
    }

    #[test]
    fn test_achievement_builder() {
        let achievement = Achievement::new(
            "test",
            "Test",
            "Description",
            AchievementType::Streak,
            AchievementTier::Silver,
        )
        .icon("🔥")
        .threshold(7)
        .points(100)
        .hidden(true);

        assert_eq!(achievement.icon, "🔥");
        assert_eq!(achievement.threshold, 7);
        assert_eq!(achievement.points, 100);
        assert!(achievement.hidden);
    }

    #[test]
    fn test_streak_creation() {
        let streak = Streak::new("user-1");
        assert_eq!(streak.user_id, "user-1");
        assert_eq!(streak.current, 0);
        assert_eq!(streak.longest, 0);
    }

    #[test]
    fn test_streak_update_same_day() {
        let mut streak = Streak::new("user-1");
        let initial_count = streak.current;
        streak.update(Utc::now());
        assert_eq!(streak.current, initial_count);
    }

    #[test]
    fn test_streak_is_active() {
        let streak = Streak::new("user-1");
        assert!(streak.is_active());
    }

    #[test]
    fn test_user_stats_default() {
        let stats = UserStats::default();
        assert_eq!(stats.tasks_completed, 0);
        assert_eq!(stats.total_points, 0);
    }

    #[test]
    fn test_user_stats_calculate_average() {
        let mut stats = UserStats {
            tasks_completed: 30,
            ..Default::default()
        };
        stats.calculate_average(10);
        assert_eq!(stats.avg_tasks_per_day, 3.0);
    }

    #[test]
    fn test_achievement_system_creation() {
        let system = AchievementSystem::new();
        assert!(system.all_achievements().len() > 0);
    }

    #[test]
    fn test_achievement_system_add_achievement() {
        let mut system = AchievementSystem::new();
        let initial_count = system.all_achievements().len();

        let achievement = Achievement::new(
            "custom",
            "Custom",
            "Description",
            AchievementType::Quality,
            AchievementTier::Gold,
        );
        system.add_achievement(achievement);

        assert_eq!(system.all_achievements().len(), initial_count + 1);
    }

    #[test]
    fn test_achievement_system_by_type() {
        let system = AchievementSystem::new();
        let task_achievements = system.achievements_by_type(AchievementType::TaskCompletion);
        assert!(task_achievements.len() > 0);
    }

    #[test]
    fn test_record_task_completed() {
        let mut system = AchievementSystem::new();
        system.record_task_completed("user-1");

        let stats = system.user_stats("user-1");
        assert_eq!(stats.tasks_completed, 1);
    }

    #[test]
    fn test_check_achievements() {
        let mut system = AchievementSystem::new();
        system.record_task_completed("user-1");

        let unlocked = system.check_achievements("user-1");
        assert!(unlocked.len() > 0);
        assert_eq!(unlocked[0].id, "first_task");
    }

    #[test]
    fn test_is_unlocked() {
        let mut system = AchievementSystem::new();
        system.record_task_completed("user-1");
        system.check_achievements("user-1");

        assert!(system.is_unlocked("user-1", "first_task"));
        assert!(!system.is_unlocked("user-1", "task_100"));
    }

    #[test]
    fn test_user_achievements() {
        let mut system = AchievementSystem::new();
        system.record_task_completed("user-1");
        system.check_achievements("user-1");

        let achievements = system.user_achievements("user-1");
        assert_eq!(achievements.len(), 1);
    }

    #[test]
    fn test_user_streak() {
        let mut system = AchievementSystem::new();
        system.record_task_completed("user-1");

        let streak = system.user_streak("user-1");
        assert!(streak.is_some());
    }

    #[test]
    fn test_leaderboard() {
        let mut system = AchievementSystem::new();

        system.record_task_completed("user-1");
        system.check_achievements("user-1");

        system.record_task_completed("user-2");
        system.record_task_completed("user-2");
        system.check_achievements("user-2");

        let leaderboard = system.leaderboard(10);
        assert!(leaderboard.len() >= 2);
        assert_eq!(leaderboard[0].rank, 1);
    }

    #[test]
    fn test_achievement_progress() {
        let mut system = AchievementSystem::new();
        system.record_task_completed("user-1");

        let progress = system.achievement_progress("user-1", "task_10");
        assert!(progress.is_some());
        assert_eq!(progress.unwrap(), 0.1); // 1/10
    }

    #[test]
    fn test_default_system() {
        let system = AchievementSystem::default();
        assert!(system.all_achievements().len() > 0);
    }

    #[test]
    fn test_multiple_task_completions() {
        let mut system = AchievementSystem::new();

        for _ in 0..10 {
            system.record_task_completed("user-1");
        }

        let unlocked = system.check_achievements("user-1");
        assert!(unlocked.iter().any(|a| a.id == "task_10"));
    }

    #[test]
    fn test_points_accumulation() {
        let mut system = AchievementSystem::new();

        system.record_task_completed("user-1");
        system.check_achievements("user-1");

        let stats = system.user_stats("user-1");
        assert!(stats.total_points > 0);
        assert_eq!(stats.achievements_unlocked, 1);
    }
}
