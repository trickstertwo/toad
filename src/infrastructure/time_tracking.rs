//! Integrated time tracking system for tasks and projects
//!
//! Provides built-in timer functionality, manual time entry, and timesheet views
//! for tracking billable and non-billable hours without external plugins.
//!
//! # Examples
//!
//! ```
//! use toad::infrastructure::time_tracking::{TimeTracker, TimeEntry};
//! use chrono::Utc;
//!
//! let mut tracker = TimeTracker::new();
//! tracker.start_timer("task-123");
//! // ... work on task ...
//! tracker.stop_timer();
//! ```

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Time entry type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TimeEntryType {
    /// Timer-based automatic tracking
    Timer,
    /// Manual time entry
    Manual,
}

/// Billability status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Billability {
    /// Billable hours
    Billable,
    /// Non-billable hours
    NonBillable,
}

/// Time entry record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeEntry {
    /// Unique entry ID
    pub id: String,
    /// Task/card ID
    pub task_id: String,
    /// Entry type (timer or manual)
    pub entry_type: TimeEntryType,
    /// Start time
    pub start: DateTime<Utc>,
    /// End time (None if timer is running)
    pub end: Option<DateTime<Utc>>,
    /// Total duration in seconds
    pub duration_seconds: i64,
    /// Description/notes
    pub description: Option<String>,
    /// Billability status
    pub billability: Billability,
    /// Project/category
    pub project: Option<String>,
    /// Tags for categorization
    pub tags: Vec<String>,
}

impl TimeEntry {
    /// Create a new time entry
    pub fn new(task_id: impl Into<String>, start: DateTime<Utc>) -> Self {
        let task_id = task_id.into();
        let id = format!("entry-{}-{}", task_id, start.timestamp());

        Self {
            id,
            task_id,
            entry_type: TimeEntryType::Timer,
            start,
            end: None,
            duration_seconds: 0,
            description: None,
            billability: Billability::NonBillable,
            project: None,
            tags: Vec::new(),
        }
    }

    /// Create a manual time entry
    pub fn manual(
        task_id: impl Into<String>,
        start: DateTime<Utc>,
        duration: Duration,
    ) -> Self {
        let task_id = task_id.into();
        let id = format!("entry-{}-{}", task_id, start.timestamp());
        let end = start + duration;

        Self {
            id,
            task_id,
            entry_type: TimeEntryType::Manual,
            start,
            end: Some(end),
            duration_seconds: duration.num_seconds(),
            description: None,
            billability: Billability::NonBillable,
            project: None,
            tags: Vec::new(),
        }
    }

    /// Set description
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set billability
    pub fn billable(mut self, billable: bool) -> Self {
        self.billability = if billable {
            Billability::Billable
        } else {
            Billability::NonBillable
        };
        self
    }

    /// Set project
    pub fn project(mut self, project: impl Into<String>) -> Self {
        self.project = Some(project.into());
        self
    }

    /// Add tag
    pub fn add_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }

    /// Stop the timer and calculate duration
    pub fn stop(&mut self, end_time: DateTime<Utc>) {
        self.end = Some(end_time);
        self.duration_seconds = (end_time - self.start).num_seconds();
    }

    /// Get duration as Duration object
    pub fn duration(&self) -> Duration {
        Duration::seconds(self.duration_seconds)
    }

    /// Check if timer is running
    pub fn is_running(&self) -> bool {
        self.end.is_none()
    }

    /// Format duration as HH:MM:SS
    pub fn format_duration(&self) -> String {
        let hours = self.duration_seconds / 3600;
        let minutes = (self.duration_seconds % 3600) / 60;
        let seconds = self.duration_seconds % 60;
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    }
}

/// Active timer state
#[derive(Debug, Clone)]
pub struct ActiveTimer {
    /// Time entry being tracked
    pub entry: TimeEntry,
    /// Start time (for display)
    pub started_at: DateTime<Utc>,
}

impl ActiveTimer {
    /// Create new active timer
    pub fn new(task_id: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            entry: TimeEntry::new(task_id, now),
            started_at: now,
        }
    }

    /// Get elapsed time
    pub fn elapsed(&self) -> Duration {
        Utc::now() - self.started_at
    }

    /// Get elapsed seconds
    pub fn elapsed_seconds(&self) -> i64 {
        self.elapsed().num_seconds()
    }

    /// Format elapsed time as HH:MM:SS
    pub fn format_elapsed(&self) -> String {
        let seconds = self.elapsed_seconds();
        let hours = seconds / 3600;
        let minutes = (seconds % 3600) / 60;
        let secs = seconds % 60;
        format!("{:02}:{:02}:{:02}", hours, minutes, secs)
    }
}

/// Time tracking statistics
#[derive(Debug, Clone, Default)]
pub struct TimeStats {
    /// Total time tracked (seconds)
    pub total_seconds: i64,
    /// Billable time (seconds)
    pub billable_seconds: i64,
    /// Non-billable time (seconds)
    pub non_billable_seconds: i64,
    /// Number of entries
    pub entry_count: usize,
    /// Average entry duration (seconds)
    pub average_seconds: i64,
}

impl TimeStats {
    /// Calculate from entries
    pub fn from_entries(entries: &[TimeEntry]) -> Self {
        let total_seconds: i64 = entries.iter().map(|e| e.duration_seconds).sum();
        let billable_seconds: i64 = entries
            .iter()
            .filter(|e| e.billability == Billability::Billable)
            .map(|e| e.duration_seconds)
            .sum();
        let non_billable_seconds = total_seconds - billable_seconds;
        let entry_count = entries.len();
        let average_seconds = if entry_count > 0 {
            total_seconds / entry_count as i64
        } else {
            0
        };

        Self {
            total_seconds,
            billable_seconds,
            non_billable_seconds,
            entry_count,
            average_seconds,
        }
    }

    /// Format total time as HH:MM:SS
    pub fn format_total(&self) -> String {
        Self::format_seconds(self.total_seconds)
    }

    /// Format billable time as HH:MM:SS
    pub fn format_billable(&self) -> String {
        Self::format_seconds(self.billable_seconds)
    }

    /// Format non-billable time as HH:MM:SS
    pub fn format_non_billable(&self) -> String {
        Self::format_seconds(self.non_billable_seconds)
    }

    /// Format average time as HH:MM:SS
    pub fn format_average(&self) -> String {
        Self::format_seconds(self.average_seconds)
    }

    /// Format seconds as HH:MM:SS
    fn format_seconds(seconds: i64) -> String {
        let hours = seconds / 3600;
        let minutes = (seconds % 3600) / 60;
        let secs = seconds % 60;
        format!("{:02}:{:02}:{:02}", hours, minutes, secs)
    }
}

/// Time tracker manager
///
/// Manages timers, time entries, and provides timesheet views.
#[derive(Debug)]
pub struct TimeTracker {
    /// All time entries
    entries: Vec<TimeEntry>,
    /// Active timer (if any)
    active_timer: Option<ActiveTimer>,
    /// Entries by task ID
    by_task: HashMap<String, Vec<usize>>,
    /// Next entry ID counter
    next_id: usize,
}

impl TimeTracker {
    /// Create a new time tracker
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            active_timer: None,
            by_task: HashMap::new(),
            next_id: 0,
        }
    }

    /// Start a timer for a task
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::infrastructure::time_tracking::TimeTracker;
    ///
    /// let mut tracker = TimeTracker::new();
    /// tracker.start_timer("task-123");
    /// ```
    pub fn start_timer(&mut self, task_id: impl Into<String>) -> bool {
        if self.active_timer.is_some() {
            return false; // Timer already running
        }

        self.active_timer = Some(ActiveTimer::new(task_id));
        true
    }

    /// Stop the active timer
    pub fn stop_timer(&mut self) -> Option<TimeEntry> {
        if let Some(mut timer) = self.active_timer.take() {
            let now = Utc::now();
            timer.entry.stop(now);
            self.add_entry(timer.entry.clone());
            Some(timer.entry)
        } else {
            None
        }
    }

    /// Get active timer
    pub fn active_timer(&self) -> Option<&ActiveTimer> {
        self.active_timer.as_ref()
    }

    /// Check if a timer is running
    pub fn is_timer_running(&self) -> bool {
        self.active_timer.is_some()
    }

    /// Add a manual time entry
    pub fn add_entry(&mut self, entry: TimeEntry) -> usize {
        let index = self.entries.len();
        let task_id = entry.task_id.clone();

        self.entries.push(entry);
        self.by_task
            .entry(task_id)
            .or_default()
            .push(index);

        self.next_id += 1;
        index
    }

    /// Add manual time
    pub fn add_manual_time(
        &mut self,
        task_id: impl Into<String>,
        start: DateTime<Utc>,
        duration: Duration,
    ) -> usize {
        let entry = TimeEntry::manual(task_id, start, duration);
        self.add_entry(entry)
    }

    /// Get all entries
    pub fn entries(&self) -> &[TimeEntry] {
        &self.entries
    }

    /// Get entries for a specific task
    pub fn entries_for_task(&self, task_id: &str) -> Vec<&TimeEntry> {
        self.by_task
            .get(task_id)
            .map(|indices| {
                indices
                    .iter()
                    .filter_map(|&idx| self.entries.get(idx))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get entries within a date range
    pub fn entries_in_range(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Vec<&TimeEntry> {
        self.entries
            .iter()
            .filter(|e| e.start >= start && e.start < end)
            .collect()
    }

    /// Get total time for a task
    pub fn total_time_for_task(&self, task_id: &str) -> Duration {
        let seconds: i64 = self
            .entries_for_task(task_id)
            .iter()
            .map(|e| e.duration_seconds)
            .sum();
        Duration::seconds(seconds)
    }

    /// Get statistics for all entries
    pub fn stats(&self) -> TimeStats {
        TimeStats::from_entries(&self.entries)
    }

    /// Get statistics for a specific task
    pub fn stats_for_task(&self, task_id: &str) -> TimeStats {
        let entries = self.entries_for_task(task_id);
        let owned_entries: Vec<TimeEntry> = entries.iter().map(|&e| e.clone()).collect();
        TimeStats::from_entries(&owned_entries)
    }

    /// Get statistics for a date range
    pub fn stats_for_range(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> TimeStats {
        let entries = self.entries_in_range(start, end);
        let owned_entries: Vec<TimeEntry> = entries.iter().map(|&e| e.clone()).collect();
        TimeStats::from_entries(&owned_entries)
    }

    /// Clear all entries
    pub fn clear(&mut self) {
        self.entries.clear();
        self.by_task.clear();
        self.active_timer = None;
        self.next_id = 0;
    }

    /// Remove entry by index
    pub fn remove_entry(&mut self, index: usize) -> bool {
        if index < self.entries.len() {
            let entry = self.entries.remove(index);

            // Update by_task indices
            if let Some(indices) = self.by_task.get_mut(&entry.task_id) {
                indices.retain(|&i| i != index);
                // Decrement indices > index
                for idx in indices.iter_mut() {
                    if *idx > index {
                        *idx -= 1;
                    }
                }
            }

            true
        } else {
            false
        }
    }
}

impl Default for TimeTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn make_datetime(year: i32, month: u32, day: u32, hour: u32, min: u32) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(year, month, day, hour, min, 0).unwrap()
    }

    #[test]
    fn test_time_entry_creation() {
        let start = make_datetime(2025, 11, 9, 10, 0);
        let entry = TimeEntry::new("task-123", start);

        assert_eq!(entry.task_id, "task-123");
        assert_eq!(entry.start, start);
        assert_eq!(entry.entry_type, TimeEntryType::Timer);
        assert!(entry.is_running());
        assert_eq!(entry.duration_seconds, 0);
    }

    #[test]
    fn test_time_entry_manual() {
        let start = make_datetime(2025, 11, 9, 10, 0);
        let duration = Duration::hours(2);
        let entry = TimeEntry::manual("task-123", start, duration);

        assert_eq!(entry.task_id, "task-123");
        assert_eq!(entry.entry_type, TimeEntryType::Manual);
        assert!(!entry.is_running());
        assert_eq!(entry.duration_seconds, 7200);
    }

    #[test]
    fn test_time_entry_builder() {
        let start = make_datetime(2025, 11, 9, 10, 0);
        let entry = TimeEntry::new("task-123", start)
            .description("Working on feature")
            .billable(true)
            .project("Project A")
            .add_tag("development")
            .add_tag("frontend");

        assert_eq!(entry.description, Some("Working on feature".to_string()));
        assert_eq!(entry.billability, Billability::Billable);
        assert_eq!(entry.project, Some("Project A".to_string()));
        assert_eq!(entry.tags, vec!["development", "frontend"]);
    }

    #[test]
    fn test_time_entry_stop() {
        let start = make_datetime(2025, 11, 9, 10, 0);
        let end = make_datetime(2025, 11, 9, 12, 30);
        let mut entry = TimeEntry::new("task-123", start);

        entry.stop(end);

        assert!(!entry.is_running());
        assert_eq!(entry.end, Some(end));
        assert_eq!(entry.duration_seconds, 9000); // 2.5 hours
    }

    #[test]
    fn test_time_entry_format_duration() {
        let start = make_datetime(2025, 11, 9, 10, 0);
        let mut entry = TimeEntry::new("task-123", start);
        entry.duration_seconds = 3665; // 1h 1m 5s

        assert_eq!(entry.format_duration(), "01:01:05");
    }

    #[test]
    fn test_active_timer_creation() {
        let timer = ActiveTimer::new("task-123");
        assert_eq!(timer.entry.task_id, "task-123");
    }

    #[test]
    fn test_time_stats_calculation() {
        let start = make_datetime(2025, 11, 9, 10, 0);
        let entries = vec![
            TimeEntry::manual("task-1", start, Duration::hours(2)).billable(true),
            TimeEntry::manual("task-2", start, Duration::hours(3)).billable(false),
            TimeEntry::manual("task-3", start, Duration::hours(1)).billable(true),
        ];

        let stats = TimeStats::from_entries(&entries);

        assert_eq!(stats.total_seconds, 21600); // 6 hours
        assert_eq!(stats.billable_seconds, 10800); // 3 hours
        assert_eq!(stats.non_billable_seconds, 10800); // 3 hours
        assert_eq!(stats.entry_count, 3);
        assert_eq!(stats.average_seconds, 7200); // 2 hours
    }

    #[test]
    fn test_time_stats_format() {
        let stats = TimeStats {
            total_seconds: 3665,
            billable_seconds: 1800,
            non_billable_seconds: 1865,
            entry_count: 2,
            average_seconds: 1832,
        };

        assert_eq!(stats.format_total(), "01:01:05");
        assert_eq!(stats.format_billable(), "00:30:00");
        assert_eq!(stats.format_average(), "00:30:32");
    }

    #[test]
    fn test_tracker_creation() {
        let tracker = TimeTracker::new();
        assert_eq!(tracker.entries().len(), 0);
        assert!(!tracker.is_timer_running());
    }

    #[test]
    fn test_tracker_start_timer() {
        let mut tracker = TimeTracker::new();
        assert!(tracker.start_timer("task-123"));
        assert!(tracker.is_timer_running());
        assert!(tracker.active_timer().is_some());
    }

    #[test]
    fn test_tracker_start_timer_already_running() {
        let mut tracker = TimeTracker::new();
        tracker.start_timer("task-123");
        assert!(!tracker.start_timer("task-456")); // Should fail
    }

    #[test]
    fn test_tracker_stop_timer() {
        let mut tracker = TimeTracker::new();
        tracker.start_timer("task-123");

        let entry = tracker.stop_timer();
        assert!(entry.is_some());
        assert!(!tracker.is_timer_running());
        assert_eq!(tracker.entries().len(), 1);
    }

    #[test]
    fn test_tracker_add_manual_time() {
        let mut tracker = TimeTracker::new();
        let start = make_datetime(2025, 11, 9, 10, 0);

        tracker.add_manual_time("task-123", start, Duration::hours(2));

        assert_eq!(tracker.entries().len(), 1);
        assert_eq!(tracker.entries()[0].duration_seconds, 7200);
    }

    #[test]
    fn test_tracker_entries_for_task() {
        let mut tracker = TimeTracker::new();
        let start = make_datetime(2025, 11, 9, 10, 0);

        tracker.add_manual_time("task-123", start, Duration::hours(1));
        tracker.add_manual_time("task-456", start, Duration::hours(2));
        tracker.add_manual_time("task-123", start, Duration::hours(3));

        let entries = tracker.entries_for_task("task-123");
        assert_eq!(entries.len(), 2);
    }

    #[test]
    fn test_tracker_total_time_for_task() {
        let mut tracker = TimeTracker::new();
        let start = make_datetime(2025, 11, 9, 10, 0);

        tracker.add_manual_time("task-123", start, Duration::hours(1));
        tracker.add_manual_time("task-123", start, Duration::hours(2));

        let total = tracker.total_time_for_task("task-123");
        assert_eq!(total.num_hours(), 3);
    }

    #[test]
    fn test_tracker_entries_in_range() {
        let mut tracker = TimeTracker::new();

        tracker.add_manual_time(
            "task-1",
            make_datetime(2025, 11, 9, 10, 0),
            Duration::hours(1),
        );
        tracker.add_manual_time(
            "task-2",
            make_datetime(2025, 11, 10, 10, 0),
            Duration::hours(2),
        );
        tracker.add_manual_time(
            "task-3",
            make_datetime(2025, 11, 11, 10, 0),
            Duration::hours(3),
        );

        let entries = tracker.entries_in_range(
            make_datetime(2025, 11, 9, 0, 0),
            make_datetime(2025, 11, 11, 0, 0),
        );

        assert_eq!(entries.len(), 2);
    }

    #[test]
    fn test_tracker_stats() {
        let mut tracker = TimeTracker::new();
        let start = make_datetime(2025, 11, 9, 10, 0);

        let entry1 = TimeEntry::manual("task-1", start, Duration::hours(2)).billable(true);
        let entry2 = TimeEntry::manual("task-2", start, Duration::hours(3)).billable(false);

        tracker.add_entry(entry1);
        tracker.add_entry(entry2);

        let stats = tracker.stats();
        assert_eq!(stats.total_seconds, 18000); // 5 hours
        assert_eq!(stats.billable_seconds, 7200); // 2 hours
        assert_eq!(stats.entry_count, 2);
    }

    #[test]
    fn test_tracker_clear() {
        let mut tracker = TimeTracker::new();
        let start = make_datetime(2025, 11, 9, 10, 0);

        tracker.add_manual_time("task-123", start, Duration::hours(1));
        tracker.start_timer("task-456");

        tracker.clear();

        assert_eq!(tracker.entries().len(), 0);
        assert!(!tracker.is_timer_running());
    }

    #[test]
    fn test_tracker_remove_entry() {
        let mut tracker = TimeTracker::new();
        let start = make_datetime(2025, 11, 9, 10, 0);

        tracker.add_manual_time("task-123", start, Duration::hours(1));
        tracker.add_manual_time("task-456", start, Duration::hours(2));

        assert!(tracker.remove_entry(0));
        assert_eq!(tracker.entries().len(), 1);
        assert_eq!(tracker.entries()[0].task_id, "task-456");
    }

    #[test]
    fn test_default_tracker() {
        let tracker = TimeTracker::default();
        assert_eq!(tracker.entries().len(), 0);
    }
}
