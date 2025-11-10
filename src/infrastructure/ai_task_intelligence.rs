//! AI Task Intelligence
//!
//! Smart task prioritization, auto-categorization, effort estimation,
//! bottleneck detection, and burndown forecasting powered by AI/ML.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Task priority suggestion
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SuggestedPriority {
    /// Critical priority
    Critical,
    /// High priority
    High,
    /// Medium priority
    Medium,
    /// Low priority
    Low,
}

impl SuggestedPriority {
    /// Get the display name
    pub fn name(&self) -> &'static str {
        match self {
            SuggestedPriority::Critical => "Critical",
            SuggestedPriority::High => "High",
            SuggestedPriority::Medium => "Medium",
            SuggestedPriority::Low => "Low",
        }
    }

    /// Get numeric score for sorting
    pub fn score(&self) -> u8 {
        match self {
            SuggestedPriority::Critical => 4,
            SuggestedPriority::High => 3,
            SuggestedPriority::Medium => 2,
            SuggestedPriority::Low => 1,
        }
    }
}

/// Priority suggestion with confidence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrioritySuggestion {
    /// Task ID
    pub task_id: String,
    /// Suggested priority
    pub priority: SuggestedPriority,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f32,
    /// Reasoning for the suggestion
    pub reasoning: Vec<String>,
    /// When this suggestion was generated
    pub generated_at: DateTime<Utc>,
}

impl PrioritySuggestion {
    /// Create a new priority suggestion
    pub fn new(task_id: String, priority: SuggestedPriority, confidence: f32) -> Self {
        Self {
            task_id,
            priority,
            confidence: confidence.clamp(0.0, 1.0),
            reasoning: Vec::new(),
            generated_at: Utc::now(),
        }
    }

    /// Add a reason for the suggestion
    pub fn add_reason(&mut self, reason: String) {
        self.reasoning.push(reason);
    }
}

/// Auto-categorization suggestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategorySuggestion {
    /// Task ID
    pub task_id: String,
    /// Suggested tags/labels
    pub suggested_tags: Vec<String>,
    /// Suggested project
    pub suggested_project: Option<String>,
    /// Suggested epic
    pub suggested_epic: Option<String>,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f32,
    /// When this suggestion was generated
    pub generated_at: DateTime<Utc>,
}

impl CategorySuggestion {
    /// Create a new category suggestion
    pub fn new(task_id: String, confidence: f32) -> Self {
        Self {
            task_id,
            suggested_tags: Vec::new(),
            suggested_project: None,
            suggested_epic: None,
            confidence: confidence.clamp(0.0, 1.0),
            generated_at: Utc::now(),
        }
    }

    /// Add a suggested tag
    pub fn add_tag(&mut self, tag: String) {
        if !self.suggested_tags.contains(&tag) {
            self.suggested_tags.push(tag);
        }
    }
}

/// Effort estimation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffortEstimation {
    /// Task ID
    pub task_id: String,
    /// Estimated hours
    pub estimated_hours: f32,
    /// Confidence interval (low, high)
    pub confidence_interval: (f32, f32),
    /// Confidence score (0.0 to 1.0)
    pub confidence: f32,
    /// Similar tasks used for estimation
    pub similar_tasks: Vec<String>,
    /// When this estimation was generated
    pub generated_at: DateTime<Utc>,
}

impl EffortEstimation {
    /// Create a new effort estimation
    pub fn new(task_id: String, estimated_hours: f32, confidence: f32) -> Self {
        let lower = estimated_hours * 0.8;
        let upper = estimated_hours * 1.2;

        Self {
            task_id,
            estimated_hours,
            confidence_interval: (lower, upper),
            confidence: confidence.clamp(0.0, 1.0),
            similar_tasks: Vec::new(),
            generated_at: Utc::now(),
        }
    }

    /// Set a custom confidence interval
    pub fn with_interval(mut self, lower: f32, upper: f32) -> Self {
        self.confidence_interval = (lower, upper);
        self
    }

    /// Add a similar task reference
    pub fn add_similar_task(&mut self, task_id: String) {
        self.similar_tasks.push(task_id);
    }
}

/// Bottleneck detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bottleneck {
    /// Column/status where bottleneck is detected
    pub column_id: String,
    /// Number of tasks in this column
    pub task_count: usize,
    /// WIP limit (if any)
    pub wip_limit: Option<usize>,
    /// Severity (0.0 to 1.0, higher is worse)
    pub severity: f32,
    /// Average time tasks spend in this column (hours)
    pub avg_time_in_column: f32,
    /// Suggested actions
    pub suggested_actions: Vec<String>,
    /// When this was detected
    pub detected_at: DateTime<Utc>,
}

impl Bottleneck {
    /// Create a new bottleneck
    pub fn new(column_id: String, task_count: usize, avg_time_in_column: f32) -> Self {
        let severity = if task_count > 10 {
            0.9
        } else if task_count > 5 {
            0.6
        } else {
            0.3
        };

        Self {
            column_id,
            task_count,
            wip_limit: None,
            severity,
            avg_time_in_column,
            suggested_actions: Vec::new(),
            detected_at: Utc::now(),
        }
    }

    /// Check if WIP limit is exceeded
    pub fn exceeds_wip_limit(&self) -> bool {
        if let Some(limit) = self.wip_limit {
            self.task_count > limit
        } else {
            false
        }
    }

    /// Add a suggested action
    pub fn add_suggestion(&mut self, action: String) {
        self.suggested_actions.push(action);
    }
}

/// Burndown forecast
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BurndownForecast {
    /// Sprint or milestone ID
    pub sprint_id: String,
    /// Target completion date
    pub target_date: DateTime<Utc>,
    /// Forecasted completion date
    pub forecasted_date: DateTime<Utc>,
    /// Confidence in forecast (0.0 to 1.0)
    pub confidence: f32,
    /// Current velocity (tasks per day)
    pub current_velocity: f32,
    /// Remaining work (task count or story points)
    pub remaining_work: f32,
    /// On track status
    pub on_track: bool,
    /// Days ahead or behind schedule (positive = ahead, negative = behind)
    pub days_delta: i32,
    /// When this forecast was generated
    pub generated_at: DateTime<Utc>,
}

impl BurndownForecast {
    /// Create a new burndown forecast
    pub fn new(
        sprint_id: String,
        target_date: DateTime<Utc>,
        remaining_work: f32,
        current_velocity: f32,
    ) -> Self {
        let days_to_complete = if current_velocity > 0.0 {
            (remaining_work / current_velocity).ceil() as i64
        } else {
            365 // Default to 1 year if no velocity
        };

        let forecasted_date = Utc::now() + chrono::Duration::days(days_to_complete);
        let days_delta = (target_date
            .signed_duration_since(forecasted_date)
            .num_days()) as i32;
        let on_track = days_delta >= 0;

        let confidence = if current_velocity > 0.0 { 0.7 } else { 0.3 };

        Self {
            sprint_id,
            target_date,
            forecasted_date,
            confidence,
            current_velocity,
            remaining_work,
            on_track,
            days_delta,
            generated_at: Utc::now(),
        }
    }

    /// Check if sprint is at risk
    pub fn is_at_risk(&self) -> bool {
        self.days_delta < -3 // More than 3 days behind
    }

    /// Get status message
    pub fn status_message(&self) -> String {
        if self.on_track {
            format!("{} days ahead of schedule", self.days_delta)
        } else {
            format!("{} days behind schedule", self.days_delta.abs())
        }
    }
}

/// AI task intelligence system
#[derive(Debug)]
pub struct AITaskIntelligence {
    /// Priority suggestions
    priority_suggestions: HashMap<String, PrioritySuggestion>,
    /// Category suggestions
    category_suggestions: HashMap<String, CategorySuggestion>,
    /// Effort estimations
    effort_estimations: HashMap<String, EffortEstimation>,
    /// Detected bottlenecks
    bottlenecks: HashMap<String, Bottleneck>,
    /// Burndown forecasts
    forecasts: HashMap<String, BurndownForecast>,
}

impl AITaskIntelligence {
    /// Create a new AI task intelligence system
    pub fn new() -> Self {
        Self {
            priority_suggestions: HashMap::new(),
            category_suggestions: HashMap::new(),
            effort_estimations: HashMap::new(),
            bottlenecks: HashMap::new(),
            forecasts: HashMap::new(),
        }
    }

    /// Suggest priority for a task
    pub fn suggest_priority(
        &mut self,
        task_id: String,
        priority: SuggestedPriority,
        confidence: f32,
        reasoning: Vec<String>,
    ) -> String {
        let mut suggestion = PrioritySuggestion::new(task_id.clone(), priority, confidence);
        for reason in reasoning {
            suggestion.add_reason(reason);
        }

        self.priority_suggestions
            .insert(task_id.clone(), suggestion);
        task_id
    }

    /// Get priority suggestion for a task
    pub fn get_priority_suggestion(&self, task_id: &str) -> Option<&PrioritySuggestion> {
        self.priority_suggestions.get(task_id)
    }

    /// Get all priority suggestions
    pub fn get_all_priority_suggestions(&self) -> Vec<&PrioritySuggestion> {
        self.priority_suggestions.values().collect()
    }

    /// Get high confidence priority suggestions
    pub fn get_high_confidence_priorities(&self, min_confidence: f32) -> Vec<&PrioritySuggestion> {
        self.priority_suggestions
            .values()
            .filter(|s| s.confidence >= min_confidence)
            .collect()
    }

    /// Suggest categories for a task
    pub fn suggest_categories(
        &mut self,
        task_id: String,
        tags: Vec<String>,
        project: Option<String>,
        epic: Option<String>,
        confidence: f32,
    ) -> String {
        let mut suggestion = CategorySuggestion::new(task_id.clone(), confidence);
        for tag in tags {
            suggestion.add_tag(tag);
        }
        suggestion.suggested_project = project;
        suggestion.suggested_epic = epic;

        self.category_suggestions
            .insert(task_id.clone(), suggestion);
        task_id
    }

    /// Get category suggestion for a task
    pub fn get_category_suggestion(&self, task_id: &str) -> Option<&CategorySuggestion> {
        self.category_suggestions.get(task_id)
    }

    /// Estimate effort for a task
    pub fn estimate_effort(
        &mut self,
        task_id: String,
        hours: f32,
        confidence: f32,
        similar_tasks: Vec<String>,
    ) -> String {
        let mut estimation = EffortEstimation::new(task_id.clone(), hours, confidence);
        for similar_task in similar_tasks {
            estimation.add_similar_task(similar_task);
        }

        self.effort_estimations.insert(task_id.clone(), estimation);
        task_id
    }

    /// Get effort estimation for a task
    pub fn get_effort_estimation(&self, task_id: &str) -> Option<&EffortEstimation> {
        self.effort_estimations.get(task_id)
    }

    /// Get all effort estimations
    pub fn get_all_estimations(&self) -> Vec<&EffortEstimation> {
        self.effort_estimations.values().collect()
    }

    /// Detect a bottleneck
    pub fn detect_bottleneck(
        &mut self,
        column_id: String,
        task_count: usize,
        avg_time: f32,
        wip_limit: Option<usize>,
        suggestions: Vec<String>,
    ) -> String {
        let mut bottleneck = Bottleneck::new(column_id.clone(), task_count, avg_time);
        bottleneck.wip_limit = wip_limit;
        for suggestion in suggestions {
            bottleneck.add_suggestion(suggestion);
        }

        self.bottlenecks.insert(column_id.clone(), bottleneck);
        column_id
    }

    /// Get bottleneck for a column
    pub fn get_bottleneck(&self, column_id: &str) -> Option<&Bottleneck> {
        self.bottlenecks.get(column_id)
    }

    /// Get all bottlenecks
    pub fn get_all_bottlenecks(&self) -> Vec<&Bottleneck> {
        self.bottlenecks.values().collect()
    }

    /// Get severe bottlenecks
    pub fn get_severe_bottlenecks(&self, min_severity: f32) -> Vec<&Bottleneck> {
        self.bottlenecks
            .values()
            .filter(|b| b.severity >= min_severity)
            .collect()
    }

    /// Create a burndown forecast
    pub fn forecast_burndown(
        &mut self,
        sprint_id: String,
        target_date: DateTime<Utc>,
        remaining_work: f32,
        current_velocity: f32,
    ) -> String {
        let forecast = BurndownForecast::new(
            sprint_id.clone(),
            target_date,
            remaining_work,
            current_velocity,
        );
        self.forecasts.insert(sprint_id.clone(), forecast);
        sprint_id
    }

    /// Get burndown forecast
    pub fn get_forecast(&self, sprint_id: &str) -> Option<&BurndownForecast> {
        self.forecasts.get(sprint_id)
    }

    /// Get all forecasts
    pub fn get_all_forecasts(&self) -> Vec<&BurndownForecast> {
        self.forecasts.values().collect()
    }

    /// Get at-risk sprints
    pub fn get_at_risk_sprints(&self) -> Vec<&BurndownForecast> {
        self.forecasts.values().filter(|f| f.is_at_risk()).collect()
    }

    /// Clear old suggestions (older than specified days)
    pub fn clear_old_suggestions(&mut self, days: i64) {
        let cutoff = Utc::now() - chrono::Duration::days(days);

        self.priority_suggestions
            .retain(|_, s| s.generated_at > cutoff);
        self.category_suggestions
            .retain(|_, s| s.generated_at > cutoff);
        self.effort_estimations
            .retain(|_, e| e.generated_at > cutoff);
        self.bottlenecks.retain(|_, b| b.detected_at > cutoff);
        self.forecasts.retain(|_, f| f.generated_at > cutoff);
    }

    /// Get suggestion count
    pub fn suggestion_count(&self) -> usize {
        self.priority_suggestions.len()
            + self.category_suggestions.len()
            + self.effort_estimations.len()
    }

    /// Get bottleneck count
    pub fn bottleneck_count(&self) -> usize {
        self.bottlenecks.len()
    }

    /// Get forecast count
    pub fn forecast_count(&self) -> usize {
        self.forecasts.len()
    }
}

impl Default for AITaskIntelligence {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suggested_priority_name() {
        assert_eq!(SuggestedPriority::Critical.name(), "Critical");
        assert_eq!(SuggestedPriority::High.name(), "High");
        assert_eq!(SuggestedPriority::Medium.name(), "Medium");
        assert_eq!(SuggestedPriority::Low.name(), "Low");
    }

    #[test]
    fn test_suggested_priority_score() {
        assert_eq!(SuggestedPriority::Critical.score(), 4);
        assert_eq!(SuggestedPriority::High.score(), 3);
        assert_eq!(SuggestedPriority::Medium.score(), 2);
        assert_eq!(SuggestedPriority::Low.score(), 1);
    }

    #[test]
    fn test_priority_suggestion_creation() {
        let suggestion =
            PrioritySuggestion::new("task-1".to_string(), SuggestedPriority::High, 0.8);

        assert_eq!(suggestion.task_id, "task-1");
        assert_eq!(suggestion.priority, SuggestedPriority::High);
        assert_eq!(suggestion.confidence, 0.8);
        assert_eq!(suggestion.reasoning.len(), 0);
    }

    #[test]
    fn test_priority_suggestion_add_reason() {
        let mut suggestion =
            PrioritySuggestion::new("task-1".to_string(), SuggestedPriority::High, 0.8);
        suggestion.add_reason("Urgent deadline".to_string());
        suggestion.add_reason("Critical path".to_string());

        assert_eq!(suggestion.reasoning.len(), 2);
        assert_eq!(suggestion.reasoning[0], "Urgent deadline");
    }

    #[test]
    fn test_category_suggestion_creation() {
        let suggestion = CategorySuggestion::new("task-1".to_string(), 0.7);

        assert_eq!(suggestion.task_id, "task-1");
        assert_eq!(suggestion.confidence, 0.7);
        assert!(suggestion.suggested_tags.is_empty());
        assert!(suggestion.suggested_project.is_none());
    }

    #[test]
    fn test_category_suggestion_add_tag() {
        let mut suggestion = CategorySuggestion::new("task-1".to_string(), 0.7);
        suggestion.add_tag("backend".to_string());
        suggestion.add_tag("api".to_string());

        assert_eq!(suggestion.suggested_tags.len(), 2);

        // Adding duplicate should not increase count
        suggestion.add_tag("backend".to_string());
        assert_eq!(suggestion.suggested_tags.len(), 2);
    }

    #[test]
    fn test_effort_estimation_creation() {
        let estimation = EffortEstimation::new("task-1".to_string(), 8.0, 0.75);

        assert_eq!(estimation.task_id, "task-1");
        assert_eq!(estimation.estimated_hours, 8.0);
        assert_eq!(estimation.confidence, 0.75);
        assert_eq!(estimation.confidence_interval.0, 6.4); // 80% of 8.0
        assert_eq!(estimation.confidence_interval.1, 9.6); // 120% of 8.0
    }

    #[test]
    fn test_effort_estimation_custom_interval() {
        let estimation =
            EffortEstimation::new("task-1".to_string(), 8.0, 0.75).with_interval(6.0, 10.0);

        assert_eq!(estimation.confidence_interval, (6.0, 10.0));
    }

    #[test]
    fn test_bottleneck_creation() {
        let bottleneck = Bottleneck::new("in-progress".to_string(), 12, 48.0);

        assert_eq!(bottleneck.column_id, "in-progress");
        assert_eq!(bottleneck.task_count, 12);
        assert_eq!(bottleneck.avg_time_in_column, 48.0);
        assert!(bottleneck.severity > 0.8); // High severity for 12 tasks
    }

    #[test]
    fn test_bottleneck_wip_limit() {
        let mut bottleneck = Bottleneck::new("in-progress".to_string(), 8, 24.0);
        bottleneck.wip_limit = Some(5);

        assert!(bottleneck.exceeds_wip_limit());

        bottleneck.wip_limit = Some(10);
        assert!(!bottleneck.exceeds_wip_limit());
    }

    #[test]
    fn test_burndown_forecast_on_track() {
        let target = Utc::now() + chrono::Duration::days(10);
        let forecast = BurndownForecast::new("sprint-1".to_string(), target, 20.0, 3.0);

        // With velocity of 3 tasks/day and 20 tasks remaining, should complete in ~7 days
        // Target is 10 days, so should be on track
        assert!(forecast.on_track);
        assert!(forecast.days_delta > 0);
    }

    #[test]
    fn test_burndown_forecast_behind() {
        let target = Utc::now() + chrono::Duration::days(5);
        let forecast = BurndownForecast::new("sprint-1".to_string(), target, 20.0, 1.0);

        // With velocity of 1 task/day and 20 tasks remaining, need 20 days
        // Target is 5 days, so behind schedule
        assert!(!forecast.on_track);
        assert!(forecast.days_delta < 0);
    }

    #[test]
    fn test_burndown_forecast_at_risk() {
        let target = Utc::now() + chrono::Duration::days(2);
        let forecast = BurndownForecast::new("sprint-1".to_string(), target, 20.0, 1.0);

        assert!(forecast.is_at_risk());
    }

    #[test]
    fn test_ai_suggest_priority() {
        let mut ai = AITaskIntelligence::new();

        ai.suggest_priority(
            "task-1".to_string(),
            SuggestedPriority::High,
            0.85,
            vec!["Urgent".to_string()],
        );

        let suggestion = ai.get_priority_suggestion("task-1").unwrap();
        assert_eq!(suggestion.priority, SuggestedPriority::High);
        assert_eq!(suggestion.confidence, 0.85);
    }

    #[test]
    fn test_ai_high_confidence_priorities() {
        let mut ai = AITaskIntelligence::new();

        ai.suggest_priority("task-1".to_string(), SuggestedPriority::High, 0.9, vec![]);
        ai.suggest_priority("task-2".to_string(), SuggestedPriority::Medium, 0.5, vec![]);
        ai.suggest_priority("task-3".to_string(), SuggestedPriority::Low, 0.8, vec![]);

        let high_conf = ai.get_high_confidence_priorities(0.75);
        assert_eq!(high_conf.len(), 2);
    }

    #[test]
    fn test_ai_suggest_categories() {
        let mut ai = AITaskIntelligence::new();

        ai.suggest_categories(
            "task-1".to_string(),
            vec!["backend".to_string(), "api".to_string()],
            Some("project-1".to_string()),
            None,
            0.75,
        );

        let suggestion = ai.get_category_suggestion("task-1").unwrap();
        assert_eq!(suggestion.suggested_tags.len(), 2);
        assert_eq!(suggestion.suggested_project, Some("project-1".to_string()));
    }

    #[test]
    fn test_ai_estimate_effort() {
        let mut ai = AITaskIntelligence::new();

        ai.estimate_effort(
            "task-1".to_string(),
            8.0,
            0.7,
            vec!["task-old-1".to_string(), "task-old-2".to_string()],
        );

        let estimation = ai.get_effort_estimation("task-1").unwrap();
        assert_eq!(estimation.estimated_hours, 8.0);
        assert_eq!(estimation.similar_tasks.len(), 2);
    }

    #[test]
    fn test_ai_detect_bottleneck() {
        let mut ai = AITaskIntelligence::new();

        ai.detect_bottleneck(
            "in-progress".to_string(),
            15,
            72.0,
            Some(10),
            vec!["Add more reviewers".to_string()],
        );

        let bottleneck = ai.get_bottleneck("in-progress").unwrap();
        assert_eq!(bottleneck.task_count, 15);
        assert!(bottleneck.exceeds_wip_limit());
        assert_eq!(bottleneck.suggested_actions.len(), 1);
    }

    #[test]
    fn test_ai_severe_bottlenecks() {
        let mut ai = AITaskIntelligence::new();

        ai.detect_bottleneck("col-1".to_string(), 12, 48.0, None, vec![]);
        ai.detect_bottleneck("col-2".to_string(), 3, 12.0, None, vec![]);
        ai.detect_bottleneck("col-3".to_string(), 8, 36.0, None, vec![]);

        let severe = ai.get_severe_bottlenecks(0.5);
        assert_eq!(severe.len(), 2); // col-1 and col-3
    }

    #[test]
    fn test_ai_forecast_burndown() {
        let mut ai = AITaskIntelligence::new();
        let target = Utc::now() + chrono::Duration::days(10);

        ai.forecast_burndown("sprint-1".to_string(), target, 20.0, 3.0);

        let forecast = ai.get_forecast("sprint-1").unwrap();
        assert_eq!(forecast.remaining_work, 20.0);
        assert_eq!(forecast.current_velocity, 3.0);
        assert!(forecast.on_track);
    }

    #[test]
    fn test_ai_at_risk_sprints() {
        let mut ai = AITaskIntelligence::new();

        let target1 = Utc::now() + chrono::Duration::days(20);
        let target2 = Utc::now() + chrono::Duration::days(5);

        ai.forecast_burndown("sprint-1".to_string(), target1, 20.0, 2.0);
        ai.forecast_burndown("sprint-2".to_string(), target2, 30.0, 1.0);

        let at_risk = ai.get_at_risk_sprints();
        assert_eq!(at_risk.len(), 1); // Only sprint-2 is at risk
    }

    #[test]
    fn test_ai_counts() {
        let mut ai = AITaskIntelligence::new();

        ai.suggest_priority("task-1".to_string(), SuggestedPriority::High, 0.8, vec![]);
        ai.suggest_categories("task-2".to_string(), vec![], None, None, 0.7);
        ai.estimate_effort("task-3".to_string(), 4.0, 0.6, vec![]);

        assert_eq!(ai.suggestion_count(), 3);

        ai.detect_bottleneck("col-1".to_string(), 10, 24.0, None, vec![]);
        assert_eq!(ai.bottleneck_count(), 1);

        let target = Utc::now() + chrono::Duration::days(10);
        ai.forecast_burndown("sprint-1".to_string(), target, 15.0, 2.0);
        assert_eq!(ai.forecast_count(), 1);
    }

    #[test]
    fn test_ai_clear_old_suggestions() {
        let mut ai = AITaskIntelligence::new();

        ai.suggest_priority("task-1".to_string(), SuggestedPriority::High, 0.8, vec![]);
        ai.suggest_categories("task-2".to_string(), vec![], None, None, 0.7);

        // Clear suggestions older than 0 days (should clear all since they were just created)
        // But since they were created "now", they won't be cleared with days=0
        assert_eq!(ai.suggestion_count(), 2);

        // Clear with negative cutoff won't clear anything created now
        ai.clear_old_suggestions(1);
        assert_eq!(ai.suggestion_count(), 2);
    }
}
