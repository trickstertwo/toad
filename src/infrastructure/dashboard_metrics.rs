// Dashboard & Metrics System
//
// Provides comprehensive analytics and metrics tracking inspired by Jira and Monday.com,
// including cumulative flow diagrams, cycle time, velocity, burndown/burnup charts, and
// team performance metrics.
//
// # Features
//
// - **Cumulative Flow Diagram**: Stacked area chart showing work distribution over time
// - **Cycle Time Chart**: Time from start to completion per task
// - **Lead Time Tracking**: Time from task creation to completion
// - **Velocity Chart**: Tasks completed per week/sprint
// - **WIP Chart**: Current work-in-progress vs. limits
// - **Burndown/Burnup Charts**: Sprint progress visualization
// - **Time in Stage**: How long tasks spend in each column
// - **Blocked Tasks Report**: List of tasks waiting on dependencies
// - **Team Performance**: Individual contributor metrics

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Time period for metrics aggregation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TimePeriod {
    /// Daily metrics
    Daily,
    /// Weekly metrics
    Weekly,
    /// Monthly metrics
    Monthly,
    /// Quarterly metrics
    Quarterly,
    /// Yearly metrics
    Yearly,
    /// Custom date range
    Custom,
}

/// Chart type for visualization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChartType {
    /// Cumulative flow diagram (stacked area)
    CumulativeFlow,
    /// Cycle time chart (line/bar)
    CycleTime,
    /// Lead time chart (line/bar)
    LeadTime,
    /// Velocity chart (bar)
    Velocity,
    /// Work-in-progress chart (line)
    WorkInProgress,
    /// Burndown chart (line)
    Burndown,
    /// Burnup chart (line)
    Burnup,
    /// Time in stage (bar)
    TimeInStage,
}

/// Data point for time series charts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataPoint {
    /// Timestamp of the data point
    pub timestamp: DateTime<Utc>,
    /// Value at this point
    pub value: f64,
    /// Optional label
    pub label: Option<String>,
}

impl DataPoint {
    /// Create a new data point
    pub fn new(timestamp: DateTime<Utc>, value: f64) -> Self {
        Self {
            timestamp,
            value,
            label: None,
        }
    }

    /// Create a data point with a label
    pub fn with_label(timestamp: DateTime<Utc>, value: f64, label: impl Into<String>) -> Self {
        Self {
            timestamp,
            value,
            label: Some(label.into()),
        }
    }
}

/// Cumulative flow diagram data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CumulativeFlowData {
    /// Data points per status/column
    pub series: HashMap<String, Vec<DataPoint>>,
    /// Start date
    pub start_date: DateTime<Utc>,
    /// End date
    pub end_date: DateTime<Utc>,
}

impl CumulativeFlowData {
    /// Create new cumulative flow data
    pub fn new(start_date: DateTime<Utc>, end_date: DateTime<Utc>) -> Self {
        Self {
            series: HashMap::new(),
            start_date,
            end_date,
        }
    }

    /// Add a data point for a specific status
    pub fn add_point(&mut self, status: impl Into<String>, point: DataPoint) {
        self.series.entry(status.into()).or_default().push(point);
    }

    /// Get total tasks at a given time
    pub fn total_at_time(&self, timestamp: DateTime<Utc>) -> usize {
        self.series
            .values()
            .flat_map(|points| {
                points
                    .iter()
                    .filter(|p| p.timestamp <= timestamp)
                    .last()
                    .map(|p| p.value as usize)
            })
            .sum()
    }
}

/// Cycle time metric (time from start to completion)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CycleTimeMetric {
    /// Task ID
    pub task_id: String,
    /// Task title
    pub task_title: String,
    /// Start time
    pub start_time: DateTime<Utc>,
    /// Completion time
    pub completion_time: DateTime<Utc>,
    /// Cycle time in hours
    pub cycle_time_hours: f64,
}

impl CycleTimeMetric {
    /// Calculate cycle time
    pub fn new(
        task_id: impl Into<String>,
        task_title: impl Into<String>,
        start_time: DateTime<Utc>,
        completion_time: DateTime<Utc>,
    ) -> Self {
        let duration = completion_time.signed_duration_since(start_time);
        let cycle_time_hours = duration.num_seconds() as f64 / 3600.0;

        Self {
            task_id: task_id.into(),
            task_title: task_title.into(),
            start_time,
            completion_time,
            cycle_time_hours,
        }
    }

    /// Get cycle time as duration
    pub fn cycle_time_duration(&self) -> Duration {
        self.completion_time.signed_duration_since(self.start_time)
    }
}

/// Lead time metric (time from creation to completion)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeadTimeMetric {
    /// Task ID
    pub task_id: String,
    /// Task title
    pub task_title: String,
    /// Creation time
    pub created_at: DateTime<Utc>,
    /// Completion time
    pub completed_at: DateTime<Utc>,
    /// Lead time in hours
    pub lead_time_hours: f64,
}

impl LeadTimeMetric {
    /// Calculate lead time
    pub fn new(
        task_id: impl Into<String>,
        task_title: impl Into<String>,
        created_at: DateTime<Utc>,
        completed_at: DateTime<Utc>,
    ) -> Self {
        let duration = completed_at.signed_duration_since(created_at);
        let lead_time_hours = duration.num_seconds() as f64 / 3600.0;

        Self {
            task_id: task_id.into(),
            task_title: task_title.into(),
            created_at,
            completed_at,
            lead_time_hours,
        }
    }

    /// Get lead time as duration
    pub fn lead_time_duration(&self) -> Duration {
        self.completed_at.signed_duration_since(self.created_at)
    }
}

/// Velocity metric (tasks completed per time period)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VelocityMetric {
    /// Time period
    pub period: TimePeriod,
    /// Start of period
    pub period_start: DateTime<Utc>,
    /// End of period
    pub period_end: DateTime<Utc>,
    /// Tasks completed
    pub tasks_completed: usize,
    /// Story points completed (if applicable)
    pub story_points: Option<usize>,
}

impl VelocityMetric {
    /// Create new velocity metric
    pub fn new(
        period: TimePeriod,
        period_start: DateTime<Utc>,
        period_end: DateTime<Utc>,
        tasks_completed: usize,
    ) -> Self {
        Self {
            period,
            period_start,
            period_end,
            tasks_completed,
            story_points: None,
        }
    }

    /// Set story points
    pub fn with_story_points(mut self, points: usize) -> Self {
        self.story_points = Some(points);
        self
    }

    /// Get average tasks per day
    pub fn tasks_per_day(&self) -> f64 {
        let days = self
            .period_end
            .signed_duration_since(self.period_start)
            .num_days() as f64;
        if days > 0.0 {
            self.tasks_completed as f64 / days
        } else {
            0.0
        }
    }
}

/// Work-in-progress metric
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WipMetric {
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Current WIP count
    pub wip_count: usize,
    /// WIP limit (if set)
    pub wip_limit: Option<usize>,
    /// WIP by status
    pub wip_by_status: HashMap<String, usize>,
}

impl WipMetric {
    /// Create new WIP metric
    pub fn new(timestamp: DateTime<Utc>, wip_count: usize) -> Self {
        Self {
            timestamp,
            wip_count,
            wip_limit: None,
            wip_by_status: HashMap::new(),
        }
    }

    /// Set WIP limit
    pub fn with_limit(mut self, limit: usize) -> Self {
        self.wip_limit = Some(limit);
        self
    }

    /// Check if over limit
    pub fn is_over_limit(&self) -> bool {
        if let Some(limit) = self.wip_limit {
            self.wip_count > limit
        } else {
            false
        }
    }
}

/// Burndown chart data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BurndownData {
    /// Sprint/iteration name
    pub sprint_name: String,
    /// Start date
    pub start_date: DateTime<Utc>,
    /// End date
    pub end_date: DateTime<Utc>,
    /// Total tasks at start
    pub total_tasks: usize,
    /// Ideal burndown line
    pub ideal_line: Vec<DataPoint>,
    /// Actual burndown line
    pub actual_line: Vec<DataPoint>,
}

impl BurndownData {
    /// Create new burndown data
    pub fn new(
        sprint_name: impl Into<String>,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
        total_tasks: usize,
    ) -> Self {
        Self {
            sprint_name: sprint_name.into(),
            start_date,
            end_date,
            total_tasks,
            ideal_line: Vec::new(),
            actual_line: Vec::new(),
        }
    }

    /// Calculate ideal burndown line
    pub fn calculate_ideal_line(&mut self, days: usize) {
        let tasks_per_day = self.total_tasks as f64 / days as f64;
        for day in 0..=days {
            let timestamp = self.start_date + Duration::days(day as i64);
            let remaining = self.total_tasks as f64 - (day as f64 * tasks_per_day);
            self.ideal_line
                .push(DataPoint::new(timestamp, remaining.max(0.0)));
        }
    }

    /// Add actual data point
    pub fn add_actual_point(&mut self, point: DataPoint) {
        self.actual_line.push(point);
    }
}

/// Burnup chart data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BurnupData {
    /// Sprint/iteration name
    pub sprint_name: String,
    /// Start date
    pub start_date: DateTime<Utc>,
    /// End date
    pub end_date: DateTime<Utc>,
    /// Total scope
    pub total_scope: usize,
    /// Scope line (can change)
    pub scope_line: Vec<DataPoint>,
    /// Completed line
    pub completed_line: Vec<DataPoint>,
}

impl BurnupData {
    /// Create new burnup data
    pub fn new(
        sprint_name: impl Into<String>,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
        total_scope: usize,
    ) -> Self {
        Self {
            sprint_name: sprint_name.into(),
            start_date,
            end_date,
            total_scope,
            scope_line: Vec::new(),
            completed_line: Vec::new(),
        }
    }

    /// Add scope change
    pub fn add_scope_point(&mut self, point: DataPoint) {
        self.scope_line.push(point);
    }

    /// Add completed work
    pub fn add_completed_point(&mut self, point: DataPoint) {
        self.completed_line.push(point);
    }
}

/// Time in stage metric
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeInStageMetric {
    /// Task ID
    pub task_id: String,
    /// Task title
    pub task_title: String,
    /// Stage/column name
    pub stage: String,
    /// Time entered stage
    pub entered_at: DateTime<Utc>,
    /// Time exited stage (if exited)
    pub exited_at: Option<DateTime<Utc>>,
    /// Duration in hours
    pub duration_hours: Option<f64>,
}

impl TimeInStageMetric {
    /// Create new time in stage metric
    pub fn new(
        task_id: impl Into<String>,
        task_title: impl Into<String>,
        stage: impl Into<String>,
        entered_at: DateTime<Utc>,
    ) -> Self {
        Self {
            task_id: task_id.into(),
            task_title: task_title.into(),
            stage: stage.into(),
            entered_at,
            exited_at: None,
            duration_hours: None,
        }
    }

    /// Mark as exited
    pub fn exit(&mut self, exited_at: DateTime<Utc>) {
        self.exited_at = Some(exited_at);
        let duration = exited_at.signed_duration_since(self.entered_at);
        self.duration_hours = Some(duration.num_seconds() as f64 / 3600.0);
    }

    /// Check if still in stage
    pub fn is_active(&self) -> bool {
        self.exited_at.is_none()
    }
}

/// Blocked task information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockedTask {
    /// Task ID
    pub task_id: String,
    /// Task title
    pub task_title: String,
    /// Blocked since
    pub blocked_since: DateTime<Utc>,
    /// Blocking reason
    pub blocking_reason: String,
    /// Dependencies (other task IDs)
    pub dependencies: Vec<String>,
    /// Hours blocked
    pub hours_blocked: f64,
}

impl BlockedTask {
    /// Create new blocked task
    pub fn new(
        task_id: impl Into<String>,
        task_title: impl Into<String>,
        blocked_since: DateTime<Utc>,
        blocking_reason: impl Into<String>,
    ) -> Self {
        let now = Utc::now();
        let duration = now.signed_duration_since(blocked_since);
        let hours_blocked = duration.num_seconds() as f64 / 3600.0;

        Self {
            task_id: task_id.into(),
            task_title: task_title.into(),
            blocked_since,
            blocking_reason: blocking_reason.into(),
            dependencies: Vec::new(),
            hours_blocked,
        }
    }

    /// Add dependency
    pub fn add_dependency(&mut self, task_id: impl Into<String>) {
        self.dependencies.push(task_id.into());
    }

    /// Update hours blocked
    pub fn update_hours_blocked(&mut self) {
        let now = Utc::now();
        let duration = now.signed_duration_since(self.blocked_since);
        self.hours_blocked = duration.num_seconds() as f64 / 3600.0;
    }
}

/// Team member performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamMemberMetrics {
    /// Member name/ID
    pub member_id: String,
    /// Member display name
    pub display_name: String,
    /// Tasks completed
    pub tasks_completed: usize,
    /// Average cycle time (hours)
    pub avg_cycle_time: f64,
    /// Average lead time (hours)
    pub avg_lead_time: f64,
    /// Current WIP
    pub current_wip: usize,
    /// Tasks created
    pub tasks_created: usize,
    /// Tasks reviewed
    pub tasks_reviewed: usize,
}

impl TeamMemberMetrics {
    /// Create new team member metrics
    pub fn new(member_id: impl Into<String>, display_name: impl Into<String>) -> Self {
        Self {
            member_id: member_id.into(),
            display_name: display_name.into(),
            tasks_completed: 0,
            avg_cycle_time: 0.0,
            avg_lead_time: 0.0,
            current_wip: 0,
            tasks_created: 0,
            tasks_reviewed: 0,
        }
    }

    /// Update with completed task
    pub fn record_completion(&mut self, cycle_time: f64, lead_time: f64) {
        let total_cycle = self.avg_cycle_time * self.tasks_completed as f64 + cycle_time;
        let total_lead = self.avg_lead_time * self.tasks_completed as f64 + lead_time;

        self.tasks_completed += 1;
        self.avg_cycle_time = total_cycle / self.tasks_completed as f64;
        self.avg_lead_time = total_lead / self.tasks_completed as f64;
    }

    /// Get productivity score (tasks per day with quality factor)
    pub fn productivity_score(&self) -> f64 {
        if self.avg_cycle_time > 0.0 {
            (self.tasks_completed as f64 / self.avg_cycle_time) * 24.0
        } else {
            0.0
        }
    }
}

/// Dashboard metrics manager
#[derive(Debug)]
pub struct DashboardMetrics {
    /// Cycle time metrics
    cycle_times: Vec<CycleTimeMetric>,
    /// Lead time metrics
    lead_times: Vec<LeadTimeMetric>,
    /// Velocity metrics
    velocities: Vec<VelocityMetric>,
    /// WIP metrics
    wip_history: Vec<WipMetric>,
    /// Time in stage metrics
    time_in_stage: Vec<TimeInStageMetric>,
    /// Blocked tasks
    blocked_tasks: Vec<BlockedTask>,
    /// Team member metrics
    team_metrics: HashMap<String, TeamMemberMetrics>,
}

impl DashboardMetrics {
    /// Create new dashboard metrics
    pub fn new() -> Self {
        Self {
            cycle_times: Vec::new(),
            lead_times: Vec::new(),
            velocities: Vec::new(),
            wip_history: Vec::new(),
            time_in_stage: Vec::new(),
            blocked_tasks: Vec::new(),
            team_metrics: HashMap::new(),
        }
    }

    /// Record cycle time
    pub fn record_cycle_time(&mut self, metric: CycleTimeMetric) {
        self.cycle_times.push(metric);
    }

    /// Record lead time
    pub fn record_lead_time(&mut self, metric: LeadTimeMetric) {
        self.lead_times.push(metric);
    }

    /// Record velocity
    pub fn record_velocity(&mut self, metric: VelocityMetric) {
        self.velocities.push(metric);
    }

    /// Record WIP
    pub fn record_wip(&mut self, metric: WipMetric) {
        self.wip_history.push(metric);
    }

    /// Record time in stage
    pub fn record_time_in_stage(&mut self, metric: TimeInStageMetric) {
        self.time_in_stage.push(metric);
    }

    /// Add blocked task
    pub fn add_blocked_task(&mut self, task: BlockedTask) {
        self.blocked_tasks.push(task);
    }

    /// Remove blocked task
    pub fn remove_blocked_task(&mut self, task_id: &str) -> bool {
        if let Some(pos) = self.blocked_tasks.iter().position(|t| t.task_id == task_id) {
            self.blocked_tasks.remove(pos);
            true
        } else {
            false
        }
    }

    /// Get or create team member metrics
    pub fn get_or_create_member_metrics(
        &mut self,
        member_id: impl Into<String>,
        display_name: impl Into<String>,
    ) -> &mut TeamMemberMetrics {
        let member_id = member_id.into();
        let display_name_clone = display_name.into();
        self.team_metrics
            .entry(member_id.clone())
            .or_insert_with(|| TeamMemberMetrics::new(member_id, display_name_clone))
    }

    /// Get average cycle time
    pub fn avg_cycle_time(&self) -> f64 {
        if self.cycle_times.is_empty() {
            0.0
        } else {
            let sum: f64 = self.cycle_times.iter().map(|m| m.cycle_time_hours).sum();
            sum / self.cycle_times.len() as f64
        }
    }

    /// Get average lead time
    pub fn avg_lead_time(&self) -> f64 {
        if self.lead_times.is_empty() {
            0.0
        } else {
            let sum: f64 = self.lead_times.iter().map(|m| m.lead_time_hours).sum();
            sum / self.lead_times.len() as f64
        }
    }

    /// Get current WIP
    pub fn current_wip(&self) -> usize {
        self.wip_history.last().map(|m| m.wip_count).unwrap_or(0)
    }

    /// Get blocked tasks count
    pub fn blocked_tasks_count(&self) -> usize {
        self.blocked_tasks.len()
    }

    /// Get team member count
    pub fn team_member_count(&self) -> usize {
        self.team_metrics.len()
    }

    /// Get all team metrics
    pub fn team_metrics(&self) -> Vec<&TeamMemberMetrics> {
        self.team_metrics.values().collect()
    }

    /// Get blocked tasks
    pub fn blocked_tasks(&self) -> &[BlockedTask] {
        &self.blocked_tasks
    }
}

impl Default for DashboardMetrics {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_point_creation() {
        let now = Utc::now();
        let point = DataPoint::new(now, 10.0);
        assert_eq!(point.value, 10.0);
        assert!(point.label.is_none());
    }

    #[test]
    fn test_data_point_with_label() {
        let now = Utc::now();
        let point = DataPoint::with_label(now, 10.0, "test");
        assert_eq!(point.label, Some("test".to_string()));
    }

    #[test]
    fn test_cumulative_flow_data() {
        let start = Utc::now();
        let end = start + Duration::days(7);
        let mut cfd = CumulativeFlowData::new(start, end);

        cfd.add_point("todo", DataPoint::new(start, 5.0));
        cfd.add_point("in_progress", DataPoint::new(start, 3.0));

        assert_eq!(cfd.series.len(), 2);
        assert_eq!(cfd.total_at_time(start), 8);
    }

    #[test]
    fn test_cycle_time_metric() {
        let start = Utc::now();
        let end = start + Duration::hours(8);
        let metric = CycleTimeMetric::new("task-1", "Test Task", start, end);

        assert_eq!(metric.task_id, "task-1");
        assert_eq!(metric.cycle_time_hours, 8.0);
    }

    #[test]
    fn test_lead_time_metric() {
        let created = Utc::now();
        let completed = created + Duration::days(2);
        let metric = LeadTimeMetric::new("task-1", "Test Task", created, completed);

        assert_eq!(metric.lead_time_hours, 48.0);
    }

    #[test]
    fn test_velocity_metric() {
        let start = Utc::now();
        let end = start + Duration::weeks(1);
        let metric = VelocityMetric::new(TimePeriod::Weekly, start, end, 10);

        assert_eq!(metric.tasks_completed, 10);
        assert!(metric.tasks_per_day() > 0.0);
    }

    #[test]
    fn test_velocity_with_story_points() {
        let start = Utc::now();
        let end = start + Duration::weeks(1);
        let metric = VelocityMetric::new(TimePeriod::Weekly, start, end, 10)
            .with_story_points(50);

        assert_eq!(metric.story_points, Some(50));
    }

    #[test]
    fn test_wip_metric() {
        let now = Utc::now();
        let metric = WipMetric::new(now, 5).with_limit(10);

        assert_eq!(metric.wip_count, 5);
        assert_eq!(metric.wip_limit, Some(10));
        assert!(!metric.is_over_limit());
    }

    #[test]
    fn test_wip_over_limit() {
        let now = Utc::now();
        let metric = WipMetric::new(now, 15).with_limit(10);

        assert!(metric.is_over_limit());
    }

    #[test]
    fn test_burndown_data() {
        let start = Utc::now();
        let end = start + Duration::days(10);
        let mut burndown = BurndownData::new("Sprint 1", start, end, 20);

        burndown.calculate_ideal_line(10);
        assert_eq!(burndown.ideal_line.len(), 11); // 0 to 10 inclusive
        assert_eq!(burndown.ideal_line[0].value, 20.0);
        assert_eq!(burndown.ideal_line[10].value, 0.0);
    }

    #[test]
    fn test_burnup_data() {
        let start = Utc::now();
        let end = start + Duration::days(10);
        let mut burnup = BurnupData::new("Sprint 1", start, end, 20);

        burnup.add_scope_point(DataPoint::new(start, 20.0));
        burnup.add_completed_point(DataPoint::new(start, 0.0));

        assert_eq!(burnup.scope_line.len(), 1);
        assert_eq!(burnup.completed_line.len(), 1);
    }

    #[test]
    fn test_time_in_stage_metric() {
        let entered = Utc::now();
        let mut metric = TimeInStageMetric::new("task-1", "Test", "In Progress", entered);

        assert!(metric.is_active());
        assert!(metric.duration_hours.is_none());

        let exited = entered + Duration::hours(4);
        metric.exit(exited);

        assert!(!metric.is_active());
        assert_eq!(metric.duration_hours, Some(4.0));
    }

    #[test]
    fn test_blocked_task() {
        let blocked_since = Utc::now() - Duration::hours(2);
        let mut task = BlockedTask::new("task-1", "Test", blocked_since, "Waiting for API");

        task.add_dependency("task-2");
        task.add_dependency("task-3");

        assert_eq!(task.dependencies.len(), 2);
        assert!(task.hours_blocked >= 2.0);
    }

    #[test]
    fn test_team_member_metrics() {
        let mut metrics = TeamMemberMetrics::new("alice", "Alice");

        metrics.record_completion(8.0, 24.0);
        metrics.record_completion(4.0, 16.0);

        assert_eq!(metrics.tasks_completed, 2);
        assert_eq!(metrics.avg_cycle_time, 6.0);
        assert_eq!(metrics.avg_lead_time, 20.0);
    }

    #[test]
    fn test_team_member_productivity_score() {
        let mut metrics = TeamMemberMetrics::new("alice", "Alice");
        metrics.record_completion(8.0, 24.0);

        let score = metrics.productivity_score();
        assert!(score > 0.0);
    }

    #[test]
    fn test_dashboard_metrics_creation() {
        let dashboard = DashboardMetrics::new();
        assert_eq!(dashboard.avg_cycle_time(), 0.0);
        assert_eq!(dashboard.avg_lead_time(), 0.0);
        assert_eq!(dashboard.current_wip(), 0);
    }

    #[test]
    fn test_dashboard_record_cycle_time() {
        let mut dashboard = DashboardMetrics::new();
        let start = Utc::now();
        let end = start + Duration::hours(8);

        dashboard.record_cycle_time(CycleTimeMetric::new("task-1", "Test", start, end));

        assert_eq!(dashboard.avg_cycle_time(), 8.0);
    }

    #[test]
    fn test_dashboard_record_lead_time() {
        let mut dashboard = DashboardMetrics::new();
        let created = Utc::now();
        let completed = created + Duration::days(1);

        dashboard.record_lead_time(LeadTimeMetric::new("task-1", "Test", created, completed));

        assert_eq!(dashboard.avg_lead_time(), 24.0);
    }

    #[test]
    fn test_dashboard_record_wip() {
        let mut dashboard = DashboardMetrics::new();
        let now = Utc::now();

        dashboard.record_wip(WipMetric::new(now, 5));
        dashboard.record_wip(WipMetric::new(now + Duration::hours(1), 7));

        assert_eq!(dashboard.current_wip(), 7);
    }

    #[test]
    fn test_dashboard_blocked_tasks() {
        let mut dashboard = DashboardMetrics::new();
        let now = Utc::now();

        dashboard.add_blocked_task(BlockedTask::new("task-1", "Test", now, "Waiting"));

        assert_eq!(dashboard.blocked_tasks_count(), 1);

        let removed = dashboard.remove_blocked_task("task-1");
        assert!(removed);
        assert_eq!(dashboard.blocked_tasks_count(), 0);
    }

    #[test]
    fn test_dashboard_team_metrics() {
        let mut dashboard = DashboardMetrics::new();

        let metrics = dashboard.get_or_create_member_metrics("alice", "Alice");
        metrics.tasks_completed = 5;

        assert_eq!(dashboard.team_member_count(), 1);
        assert_eq!(dashboard.team_metrics().len(), 1);
    }
}
