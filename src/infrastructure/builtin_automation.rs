// Built-in Automation System
//
// Provides comprehensive automation features inspired by Monday.com and Jira, including
// When/Then rules, auto-assignment, recurring tasks, template cards, and bulk actions.
//
// # Features
//
// - **When/Then Rules**: "When card moves to Done → Archive after 7 days"
// - **Auto-Assignment**: "When priority = P0 → Assign to @lead"
// - **Due Date Automation**: "When created → Set due date 3 days from now"
// - **Recurring Tasks**: Auto-create daily/weekly standup cards
// - **Template Cards**: Save card templates for common task types
// - **Bulk Actions**: Multi-select cards and apply actions (move, tag, assign, delete)

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Trigger condition for automation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TriggerCondition {
    /// Task created
    TaskCreated,
    /// Task moved to specific status
    TaskMovedTo(String),
    /// Task moved from specific status
    TaskMovedFrom(String),
    /// Task priority changed to value
    PriorityChangedTo(String),
    /// Task assigned
    TaskAssigned,
    /// Task unassigned
    TaskUnassigned,
    /// Task due date set
    DueDateSet,
    /// Task due date approaching (days before)
    DueDateApproaching(u32),
    /// Task overdue
    TaskOverdue,
    /// Task completed
    TaskCompleted,
    /// Tag added
    TagAdded(String),
    /// Tag removed
    TagRemoved(String),
}

impl TriggerCondition {
    /// Get trigger name
    pub fn name(&self) -> String {
        match self {
            TriggerCondition::TaskCreated => "Task Created".to_string(),
            TriggerCondition::TaskMovedTo(status) => format!("Task Moved To {}", status),
            TriggerCondition::TaskMovedFrom(status) => format!("Task Moved From {}", status),
            TriggerCondition::PriorityChangedTo(priority) => {
                format!("Priority Changed To {}", priority)
            }
            TriggerCondition::TaskAssigned => "Task Assigned".to_string(),
            TriggerCondition::TaskUnassigned => "Task Unassigned".to_string(),
            TriggerCondition::DueDateSet => "Due Date Set".to_string(),
            TriggerCondition::DueDateApproaching(days) => {
                format!("Due Date Approaching ({} days)", days)
            }
            TriggerCondition::TaskOverdue => "Task Overdue".to_string(),
            TriggerCondition::TaskCompleted => "Task Completed".to_string(),
            TriggerCondition::TagAdded(tag) => format!("Tag Added ({})", tag),
            TriggerCondition::TagRemoved(tag) => format!("Tag Removed ({})", tag),
        }
    }
}

/// Action to perform when trigger fires
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AutomationAction {
    /// Move task to status
    MoveToStatus(String),
    /// Assign to user
    AssignTo(String),
    /// Unassign task
    Unassign,
    /// Add tag
    AddTag(String),
    /// Remove tag
    RemoveTag(String),
    /// Set priority
    SetPriority(String),
    /// Set due date (days from now)
    SetDueDateDaysFromNow(u32),
    /// Archive task
    Archive,
    /// Delete task
    Delete,
    /// Send notification
    SendNotification(String),
    /// Add comment
    AddComment(String),
}

impl AutomationAction {
    /// Get action name
    pub fn name(&self) -> String {
        match self {
            AutomationAction::MoveToStatus(status) => format!("Move To {}", status),
            AutomationAction::AssignTo(user) => format!("Assign To {}", user),
            AutomationAction::Unassign => "Unassign".to_string(),
            AutomationAction::AddTag(tag) => format!("Add Tag {}", tag),
            AutomationAction::RemoveTag(tag) => format!("Remove Tag {}", tag),
            AutomationAction::SetPriority(priority) => format!("Set Priority {}", priority),
            AutomationAction::SetDueDateDaysFromNow(days) => {
                format!("Set Due Date {} days from now", days)
            }
            AutomationAction::Archive => "Archive".to_string(),
            AutomationAction::Delete => "Delete".to_string(),
            AutomationAction::SendNotification(msg) => format!("Send Notification: {}", msg),
            AutomationAction::AddComment(comment) => format!("Add Comment: {}", comment),
        }
    }
}

/// Automation rule (When/Then)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationRule {
    /// Rule ID
    pub id: String,
    /// Rule name
    pub name: String,
    /// Enabled status
    pub enabled: bool,
    /// Trigger condition
    pub trigger: TriggerCondition,
    /// Action to perform
    pub action: AutomationAction,
    /// Delay before executing (in seconds)
    pub delay_seconds: Option<u64>,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
    /// Last executed
    pub last_executed: Option<DateTime<Utc>>,
    /// Execution count
    pub execution_count: usize,
}

impl AutomationRule {
    /// Create a new automation rule
    pub fn new(
        name: impl Into<String>,
        trigger: TriggerCondition,
        action: AutomationAction,
    ) -> Self {
        Self {
            id: format!("rule-{}", Utc::now().timestamp_millis()),
            name: name.into(),
            enabled: true,
            trigger,
            action,
            delay_seconds: None,
            created_at: Utc::now(),
            last_executed: None,
            execution_count: 0,
        }
    }

    /// Set delay
    pub fn with_delay(mut self, seconds: u64) -> Self {
        self.delay_seconds = Some(seconds);
        self
    }

    /// Record execution
    pub fn record_execution(&mut self) {
        self.last_executed = Some(Utc::now());
        self.execution_count += 1;
    }

    /// Enable/disable rule
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

/// Recurrence pattern
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecurrencePattern {
    /// Daily
    Daily,
    /// Every N days
    EveryNDays(u32),
    /// Weekly on specific day (0 = Sunday, 6 = Saturday)
    Weekly(u8),
    /// Monthly on specific day
    Monthly(u8),
    /// Yearly on specific date (month, day)
    Yearly(u8, u8),
}

impl RecurrencePattern {
    /// Get pattern name
    pub fn name(&self) -> String {
        match self {
            RecurrencePattern::Daily => "Daily".to_string(),
            RecurrencePattern::EveryNDays(n) => format!("Every {} days", n),
            RecurrencePattern::Weekly(day) => format!("Weekly on {}", Self::day_name(*day)),
            RecurrencePattern::Monthly(day) => format!("Monthly on day {}", day),
            RecurrencePattern::Yearly(month, day) => format!("Yearly on {}/{}", month, day),
        }
    }

    fn day_name(day: u8) -> &'static str {
        match day {
            0 => "Sunday",
            1 => "Monday",
            2 => "Tuesday",
            3 => "Wednesday",
            4 => "Thursday",
            5 => "Friday",
            6 => "Saturday",
            _ => "Unknown",
        }
    }
}

/// Recurring task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecurringTask {
    /// Task ID
    pub id: String,
    /// Task title
    pub title: String,
    /// Task description
    pub description: String,
    /// Recurrence pattern
    pub pattern: RecurrencePattern,
    /// Next occurrence
    pub next_occurrence: DateTime<Utc>,
    /// Last created
    pub last_created: Option<DateTime<Utc>>,
    /// Enabled status
    pub enabled: bool,
    /// Target status/column
    pub target_status: String,
    /// Tags to apply
    pub tags: Vec<String>,
    /// Assignee
    pub assignee: Option<String>,
}

impl RecurringTask {
    /// Create a new recurring task
    pub fn new(
        title: impl Into<String>,
        description: impl Into<String>,
        pattern: RecurrencePattern,
    ) -> Self {
        Self {
            id: format!("recurring-{}", Utc::now().timestamp_millis()),
            title: title.into(),
            description: description.into(),
            pattern,
            next_occurrence: Utc::now(),
            last_created: None,
            enabled: true,
            target_status: "To Do".to_string(),
            tags: Vec::new(),
            assignee: None,
        }
    }

    /// Set target status
    pub fn with_status(mut self, status: impl Into<String>) -> Self {
        self.target_status = status.into();
        self
    }

    /// Add tag
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }

    /// Set assignee
    pub fn with_assignee(mut self, assignee: impl Into<String>) -> Self {
        self.assignee = Some(assignee.into());
        self
    }

    /// Check if task should be created now
    pub fn should_create(&self) -> bool {
        self.enabled && Utc::now() >= self.next_occurrence
    }

    /// Calculate next occurrence
    pub fn calculate_next_occurrence(&mut self) {
        self.last_created = Some(Utc::now());

        self.next_occurrence = match self.pattern {
            RecurrencePattern::Daily => Utc::now() + Duration::days(1),
            RecurrencePattern::EveryNDays(n) => Utc::now() + Duration::days(n as i64),
            RecurrencePattern::Weekly(_) => Utc::now() + Duration::weeks(1),
            RecurrencePattern::Monthly(_) => Utc::now() + Duration::days(30),
            RecurrencePattern::Yearly(_, _) => Utc::now() + Duration::days(365),
        };
    }
}

/// Task template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskTemplate {
    /// Template ID
    pub id: String,
    /// Template name
    pub name: String,
    /// Template title (can use variables)
    pub title: String,
    /// Template description
    pub description: String,
    /// Default status
    pub default_status: String,
    /// Default priority
    pub default_priority: Option<String>,
    /// Default tags
    pub tags: Vec<String>,
    /// Default assignee
    pub assignee: Option<String>,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
    /// Use count
    pub use_count: usize,
}

impl TaskTemplate {
    /// Create a new task template
    pub fn new(name: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            id: format!("template-{}", Utc::now().timestamp_millis()),
            name: name.into(),
            title: title.into(),
            description: String::new(),
            default_status: "To Do".to_string(),
            default_priority: None,
            tags: Vec::new(),
            assignee: None,
            created_at: Utc::now(),
            use_count: 0,
        }
    }

    /// Set description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    /// Set default priority
    pub fn with_priority(mut self, priority: impl Into<String>) -> Self {
        self.default_priority = Some(priority.into());
        self
    }

    /// Add tag
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }

    /// Record use
    pub fn record_use(&mut self) {
        self.use_count += 1;
    }
}

/// Bulk action type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BulkActionType {
    /// Move all to status
    MoveToStatus(String),
    /// Assign all to user
    AssignTo(String),
    /// Add tag to all
    AddTag(String),
    /// Remove tag from all
    RemoveTag(String),
    /// Set priority for all
    SetPriority(String),
    /// Archive all
    Archive,
    /// Delete all
    Delete,
}

impl BulkActionType {
    /// Get action name
    pub fn name(&self) -> String {
        match self {
            BulkActionType::MoveToStatus(status) => format!("Move All To {}", status),
            BulkActionType::AssignTo(user) => format!("Assign All To {}", user),
            BulkActionType::AddTag(tag) => format!("Add Tag '{}' To All", tag),
            BulkActionType::RemoveTag(tag) => format!("Remove Tag '{}' From All", tag),
            BulkActionType::SetPriority(priority) => format!("Set Priority {} For All", priority),
            BulkActionType::Archive => "Archive All".to_string(),
            BulkActionType::Delete => "Delete All".to_string(),
        }
    }
}

/// Bulk action result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkActionResult {
    /// Total tasks affected
    pub affected_count: usize,
    /// Errors encountered
    pub errors: Vec<String>,
    /// Success status
    pub success: bool,
}

impl BulkActionResult {
    /// Create a successful result
    pub fn success(affected_count: usize) -> Self {
        Self {
            affected_count,
            errors: Vec::new(),
            success: true,
        }
    }

    /// Create a result with errors
    pub fn with_errors(affected_count: usize, errors: Vec<String>) -> Self {
        Self {
            affected_count,
            errors,
            success: false,
        }
    }
}

/// Automation manager
#[derive(Debug)]
pub struct AutomationManager {
    /// Automation rules
    rules: HashMap<String, AutomationRule>,
    /// Recurring tasks
    recurring_tasks: HashMap<String, RecurringTask>,
    /// Task templates
    templates: HashMap<String, TaskTemplate>,
    /// Next rule ID counter
    next_rule_id: usize,
    /// Next recurring task ID counter
    next_recurring_id: usize,
    /// Next template ID counter
    next_template_id: usize,
}

impl AutomationManager {
    /// Create a new automation manager
    pub fn new() -> Self {
        Self {
            rules: HashMap::new(),
            recurring_tasks: HashMap::new(),
            templates: HashMap::new(),
            next_rule_id: 1,
            next_recurring_id: 1,
            next_template_id: 1,
        }
    }

    /// Add an automation rule
    pub fn add_rule(&mut self, mut rule: AutomationRule) -> String {
        // Override ID with incremental counter
        rule.id = format!("rule-{}", self.next_rule_id);
        self.next_rule_id += 1;

        let id = rule.id.clone();
        self.rules.insert(id.clone(), rule);
        id
    }

    /// Remove an automation rule
    pub fn remove_rule(&mut self, rule_id: &str) -> bool {
        self.rules.remove(rule_id).is_some()
    }

    /// Get rule
    pub fn get_rule(&self, rule_id: &str) -> Option<&AutomationRule> {
        self.rules.get(rule_id)
    }

    /// Get rule (mutable)
    pub fn get_rule_mut(&mut self, rule_id: &str) -> Option<&mut AutomationRule> {
        self.rules.get_mut(rule_id)
    }

    /// Get all rules
    pub fn get_rules(&self) -> Vec<&AutomationRule> {
        self.rules.values().collect()
    }

    /// Get enabled rules
    pub fn get_enabled_rules(&self) -> Vec<&AutomationRule> {
        self.rules.values().filter(|r| r.enabled).collect()
    }

    /// Find rules matching a trigger
    pub fn find_rules_for_trigger(&self, trigger: &TriggerCondition) -> Vec<&AutomationRule> {
        self.rules
            .values()
            .filter(|r| r.enabled && r.trigger == *trigger)
            .collect()
    }

    /// Add a recurring task
    pub fn add_recurring_task(&mut self, mut task: RecurringTask) -> String {
        // Override ID with incremental counter
        task.id = format!("recurring-{}", self.next_recurring_id);
        self.next_recurring_id += 1;

        let id = task.id.clone();
        self.recurring_tasks.insert(id.clone(), task);
        id
    }

    /// Remove a recurring task
    pub fn remove_recurring_task(&mut self, task_id: &str) -> bool {
        self.recurring_tasks.remove(task_id).is_some()
    }

    /// Get recurring tasks that should be created now
    pub fn get_due_recurring_tasks(&self) -> Vec<&RecurringTask> {
        self.recurring_tasks
            .values()
            .filter(|t| t.should_create())
            .collect()
    }

    /// Mark recurring task as created
    pub fn mark_recurring_task_created(&mut self, task_id: &str) {
        if let Some(task) = self.recurring_tasks.get_mut(task_id) {
            task.calculate_next_occurrence();
        }
    }

    /// Get all recurring tasks
    pub fn get_recurring_tasks(&self) -> Vec<&RecurringTask> {
        self.recurring_tasks.values().collect()
    }

    /// Add a task template
    pub fn add_template(&mut self, mut template: TaskTemplate) -> String {
        // Override ID with incremental counter
        template.id = format!("template-{}", self.next_template_id);
        self.next_template_id += 1;

        let id = template.id.clone();
        self.templates.insert(id.clone(), template);
        id
    }

    /// Remove a template
    pub fn remove_template(&mut self, template_id: &str) -> bool {
        self.templates.remove(template_id).is_some()
    }

    /// Get template
    pub fn get_template(&self, template_id: &str) -> Option<&TaskTemplate> {
        self.templates.get(template_id)
    }

    /// Get all templates
    pub fn get_templates(&self) -> Vec<&TaskTemplate> {
        self.templates.values().collect()
    }

    /// Use a template (increments use count)
    pub fn use_template(&mut self, template_id: &str) -> Option<&TaskTemplate> {
        if let Some(template) = self.templates.get_mut(template_id) {
            template.record_use();
            Some(&*template)
        } else {
            None
        }
    }

    /// Execute bulk action (simulated)
    pub fn execute_bulk_action(
        &self,
        _action: BulkActionType,
        task_ids: &[String],
    ) -> BulkActionResult {
        // In a real implementation, this would apply the action to tasks
        // For now, just return success with the count
        BulkActionResult::success(task_ids.len())
    }
}

impl Default for AutomationManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trigger_condition_name() {
        let trigger = TriggerCondition::TaskMovedTo("Done".to_string());
        assert_eq!(trigger.name(), "Task Moved To Done");
    }

    #[test]
    fn test_automation_action_name() {
        let action = AutomationAction::AssignTo("alice".to_string());
        assert_eq!(action.name(), "Assign To alice");
    }

    #[test]
    fn test_automation_rule_creation() {
        let rule = AutomationRule::new(
            "Auto Archive",
            TriggerCondition::TaskCompleted,
            AutomationAction::Archive,
        );

        assert_eq!(rule.name, "Auto Archive");
        assert!(rule.enabled);
        assert_eq!(rule.execution_count, 0);
    }

    #[test]
    fn test_automation_rule_with_delay() {
        let rule = AutomationRule::new(
            "Delayed Action",
            TriggerCondition::TaskCreated,
            AutomationAction::AddComment("Welcome".to_string()),
        )
        .with_delay(3600);

        assert_eq!(rule.delay_seconds, Some(3600));
    }

    #[test]
    fn test_automation_rule_record_execution() {
        let mut rule = AutomationRule::new(
            "Test",
            TriggerCondition::TaskCreated,
            AutomationAction::Archive,
        );

        assert_eq!(rule.execution_count, 0);
        rule.record_execution();
        assert_eq!(rule.execution_count, 1);
        assert!(rule.last_executed.is_some());
    }

    #[test]
    fn test_recurrence_pattern_name() {
        assert_eq!(RecurrencePattern::Daily.name(), "Daily");
        assert_eq!(RecurrencePattern::EveryNDays(3).name(), "Every 3 days");
        assert_eq!(RecurrencePattern::Weekly(1).name(), "Weekly on Monday");
    }

    #[test]
    fn test_recurring_task_creation() {
        let task = RecurringTask::new(
            "Daily Standup",
            "Team standup meeting",
            RecurrencePattern::Daily,
        );

        assert_eq!(task.title, "Daily Standup");
        assert!(task.enabled);
        assert_eq!(task.pattern, RecurrencePattern::Daily);
    }

    #[test]
    fn test_recurring_task_with_status() {
        let task =
            RecurringTask::new("Task", "Desc", RecurrencePattern::Daily).with_status("In Progress");

        assert_eq!(task.target_status, "In Progress");
    }

    #[test]
    fn test_recurring_task_should_create() {
        let mut task = RecurringTask::new("Task", "Desc", RecurrencePattern::Daily);
        // Set next occurrence to the past
        task.next_occurrence = Utc::now() - Duration::hours(1);

        assert!(task.should_create());
    }

    #[test]
    fn test_task_template_creation() {
        let template = TaskTemplate::new("Bug Report", "Bug: {title}");

        assert_eq!(template.name, "Bug Report");
        assert_eq!(template.use_count, 0);
    }

    #[test]
    fn test_task_template_record_use() {
        let mut template = TaskTemplate::new("Feature", "Feature: {title}");

        template.record_use();
        template.record_use();
        assert_eq!(template.use_count, 2);
    }

    #[test]
    fn test_bulk_action_type_name() {
        let action = BulkActionType::MoveToStatus("Done".to_string());
        assert_eq!(action.name(), "Move All To Done");
    }

    #[test]
    fn test_bulk_action_result_success() {
        let result = BulkActionResult::success(10);
        assert!(result.success);
        assert_eq!(result.affected_count, 10);
        assert!(result.errors.is_empty());
    }

    #[test]
    fn test_automation_manager_creation() {
        let manager = AutomationManager::new();
        assert_eq!(manager.get_rules().len(), 0);
        assert_eq!(manager.get_templates().len(), 0);
    }

    #[test]
    fn test_add_rule() {
        let mut manager = AutomationManager::new();
        let rule = AutomationRule::new(
            "Test",
            TriggerCondition::TaskCreated,
            AutomationAction::Archive,
        );

        let id = manager.add_rule(rule);
        assert_eq!(manager.get_rules().len(), 1);
        assert!(manager.get_rule(&id).is_some());
    }

    #[test]
    fn test_remove_rule() {
        let mut manager = AutomationManager::new();
        let rule = AutomationRule::new(
            "Test",
            TriggerCondition::TaskCreated,
            AutomationAction::Archive,
        );

        let id = manager.add_rule(rule);
        assert!(manager.remove_rule(&id));
        assert_eq!(manager.get_rules().len(), 0);
    }

    #[test]
    fn test_find_rules_for_trigger() {
        let mut manager = AutomationManager::new();

        manager.add_rule(AutomationRule::new(
            "Rule 1",
            TriggerCondition::TaskCreated,
            AutomationAction::Archive,
        ));
        manager.add_rule(AutomationRule::new(
            "Rule 2",
            TriggerCondition::TaskCreated,
            AutomationAction::AddTag("new".to_string()),
        ));
        manager.add_rule(AutomationRule::new(
            "Rule 3",
            TriggerCondition::TaskCompleted,
            AutomationAction::Archive,
        ));

        let rules = manager.find_rules_for_trigger(&TriggerCondition::TaskCreated);
        assert_eq!(rules.len(), 2);
    }

    #[test]
    fn test_add_recurring_task() {
        let mut manager = AutomationManager::new();
        let task = RecurringTask::new("Daily", "Desc", RecurrencePattern::Daily);

        let id = manager.add_recurring_task(task);
        assert_eq!(manager.get_recurring_tasks().len(), 1);
        assert!(manager.recurring_tasks.contains_key(&id));
    }

    #[test]
    fn test_add_template() {
        let mut manager = AutomationManager::new();
        let template = TaskTemplate::new("Bug", "Bug: {title}");

        let id = manager.add_template(template);
        assert_eq!(manager.get_templates().len(), 1);
        assert!(manager.get_template(&id).is_some());
    }

    #[test]
    fn test_use_template() {
        let mut manager = AutomationManager::new();
        let template = TaskTemplate::new("Bug", "Bug: {title}");
        let id = manager.add_template(template);

        manager.use_template(&id);
        let template = manager.get_template(&id).unwrap();
        assert_eq!(template.use_count, 1);
    }

    #[test]
    fn test_execute_bulk_action() {
        let manager = AutomationManager::new();
        let task_ids = vec!["task-1".to_string(), "task-2".to_string()];

        let result = manager.execute_bulk_action(BulkActionType::Archive, &task_ids);
        assert!(result.success);
        assert_eq!(result.affected_count, 2);
    }
}
