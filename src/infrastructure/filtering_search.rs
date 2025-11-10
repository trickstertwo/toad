// Filtering & Search System
//
// Provides comprehensive task filtering and search capabilities including quick filters,
// advanced search, saved filters, and power user search syntax.
//
// # Features
//
// - **Quick Filters**: Pre-built common filters (My Tasks, Due This Week, High Priority)
// - **Advanced Search**: Full-text search across titles, descriptions, comments
// - **Saved Filters**: Bookmark complex filter combinations
// - **Filter by Everything**: Tags, assignee, date range, priority, status, custom fields
// - **Search Syntax**: Power user queries like `assignee:me priority:P0 -tag:blocked`

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Filter operator for comparisons
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FilterOperator {
    /// Equals (exact match)
    Equals,
    /// Not equals
    NotEquals,
    /// Contains substring
    Contains,
    /// Does not contain substring
    NotContains,
    /// Greater than (for dates, numbers)
    GreaterThan,
    /// Less than (for dates, numbers)
    LessThan,
    /// In list of values
    In,
    /// Not in list of values
    NotIn,
    /// Starts with prefix
    StartsWith,
    /// Ends with suffix
    EndsWith,
}

/// Filter field type
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FilterField {
    /// Task title
    Title,
    /// Task description
    Description,
    /// Task tags
    Tags,
    /// Task assignee
    Assignee,
    /// Task priority
    Priority,
    /// Task status
    Status,
    /// Task due date
    DueDate,
    /// Task created date
    CreatedDate,
    /// Task modified date
    ModifiedDate,
    /// Custom field by name
    CustomField(String),
    /// Full-text search across all text fields
    FullText,
}

/// Individual filter condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterCondition {
    /// Field to filter on
    pub field: FilterField,
    /// Comparison operator
    pub operator: FilterOperator,
    /// Value to compare against
    pub value: String,
}

impl FilterCondition {
    /// Create a new filter condition
    pub fn new(field: FilterField, operator: FilterOperator, value: impl Into<String>) -> Self {
        Self {
            field,
            operator,
            value: value.into(),
        }
    }

    /// Create an equals condition
    pub fn equals(field: FilterField, value: impl Into<String>) -> Self {
        Self::new(field, FilterOperator::Equals, value)
    }

    /// Create a contains condition
    pub fn contains(field: FilterField, value: impl Into<String>) -> Self {
        Self::new(field, FilterOperator::Contains, value)
    }

    /// Create a not contains condition (exclusion)
    pub fn not_contains(field: FilterField, value: impl Into<String>) -> Self {
        Self::new(field, FilterOperator::NotContains, value)
    }
}

/// Logical operator for combining filters
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LogicalOperator {
    /// All conditions must match (AND)
    And,
    /// Any condition must match (OR)
    Or,
}

/// Complete filter with multiple conditions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Filter {
    /// Unique filter ID
    pub id: String,
    /// Human-readable filter name
    pub name: String,
    /// Filter conditions
    pub conditions: Vec<FilterCondition>,
    /// Logical operator combining conditions
    pub logic: LogicalOperator,
    /// Whether this is a saved filter
    pub saved: bool,
    /// Filter creation time
    pub created_at: DateTime<Utc>,
}

impl Filter {
    /// Create a new filter
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: format!("filter-{}", Utc::now().timestamp_millis()),
            name: name.into(),
            conditions: Vec::new(),
            logic: LogicalOperator::And,
            saved: false,
            created_at: Utc::now(),
        }
    }

    /// Add a condition to this filter
    pub fn add_condition(&mut self, condition: FilterCondition) {
        self.conditions.push(condition);
    }

    /// Set logical operator
    pub fn with_logic(mut self, logic: LogicalOperator) -> Self {
        self.logic = logic;
        self
    }

    /// Mark this filter as saved
    pub fn mark_saved(&mut self) {
        self.saved = true;
    }

    /// Check if this filter is empty
    pub fn is_empty(&self) -> bool {
        self.conditions.is_empty()
    }
}

/// Quick filter templates
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum QuickFilter {
    /// Tasks assigned to me
    MyTasks,
    /// Tasks due this week
    DueThisWeek,
    /// Tasks due today
    DueToday,
    /// High priority tasks
    HighPriority,
    /// Overdue tasks
    Overdue,
    /// Unassigned tasks
    Unassigned,
    /// Completed tasks
    Completed,
    /// In progress tasks
    InProgress,
    /// All tasks (no filter)
    All,
}

impl QuickFilter {
    /// Get the display name
    pub fn name(&self) -> &'static str {
        match self {
            QuickFilter::MyTasks => "My Tasks",
            QuickFilter::DueThisWeek => "Due This Week",
            QuickFilter::DueToday => "Due Today",
            QuickFilter::HighPriority => "High Priority",
            QuickFilter::Overdue => "Overdue",
            QuickFilter::Unassigned => "Unassigned",
            QuickFilter::Completed => "Completed",
            QuickFilter::InProgress => "In Progress",
            QuickFilter::All => "All Tasks",
        }
    }

    /// Convert to a Filter
    pub fn to_filter(&self, current_user: &str) -> Filter {
        let mut filter = Filter::new(self.name());
        filter.saved = false;

        match self {
            QuickFilter::MyTasks => {
                filter.add_condition(FilterCondition::equals(FilterField::Assignee, current_user));
            }
            QuickFilter::DueThisWeek => {
                // This would need date logic in real implementation
                filter.add_condition(FilterCondition::new(
                    FilterField::DueDate,
                    FilterOperator::LessThan,
                    "week",
                ));
            }
            QuickFilter::DueToday => {
                filter.add_condition(FilterCondition::new(
                    FilterField::DueDate,
                    FilterOperator::Equals,
                    "today",
                ));
            }
            QuickFilter::HighPriority => {
                filter.add_condition(FilterCondition::equals(FilterField::Priority, "high"));
            }
            QuickFilter::Overdue => {
                filter.add_condition(FilterCondition::new(
                    FilterField::DueDate,
                    FilterOperator::LessThan,
                    "now",
                ));
            }
            QuickFilter::Unassigned => {
                filter.add_condition(FilterCondition::equals(FilterField::Assignee, ""));
            }
            QuickFilter::Completed => {
                filter.add_condition(FilterCondition::equals(FilterField::Status, "completed"));
            }
            QuickFilter::InProgress => {
                filter.add_condition(FilterCondition::equals(FilterField::Status, "in_progress"));
            }
            QuickFilter::All => {
                // No conditions - show everything
            }
        }

        filter
    }
}

/// Search query parser for power user syntax
///
/// Supports syntax like: `assignee:me priority:P0 -tag:blocked status:open`
#[derive(Debug)]
pub struct SearchParser;

impl SearchParser {
    /// Parse a search query into a Filter
    ///
    /// Syntax:
    /// - `field:value` - equals filter
    /// - `-field:value` - not equals filter
    /// - `"quoted text"` - full-text search
    /// - Multiple terms combined with AND by default
    pub fn parse(query: &str) -> Filter {
        let mut filter = Filter::new("Search");
        let tokens = Self::tokenize(query);

        for token in tokens {
            if let Some(condition) = Self::parse_token(&token) {
                filter.add_condition(condition);
            }
        }

        filter
    }

    /// Tokenize query into parts
    fn tokenize(query: &str) -> Vec<String> {
        let mut tokens = Vec::new();
        let mut current = String::new();
        let mut in_quotes = false;

        for ch in query.chars() {
            match ch {
                '"' => {
                    in_quotes = !in_quotes;
                }
                ' ' if !in_quotes => {
                    if !current.is_empty() {
                        tokens.push(current.clone());
                        current.clear();
                    }
                }
                _ => {
                    current.push(ch);
                }
            }
        }

        if !current.is_empty() {
            tokens.push(current);
        }

        tokens
    }

    /// Parse a single token into a FilterCondition
    fn parse_token(token: &str) -> Option<FilterCondition> {
        // Handle negation
        let (negated, token) = if token.starts_with('-') {
            (true, &token[1..])
        } else {
            (false, token)
        };

        // Check for field:value syntax
        if let Some(colon_pos) = token.find(':') {
            let field = &token[..colon_pos];
            let value = &token[colon_pos + 1..];

            let filter_field = match field.to_lowercase().as_str() {
                "title" => FilterField::Title,
                "desc" | "description" => FilterField::Description,
                "tag" | "tags" => FilterField::Tags,
                "assignee" | "assigned" => FilterField::Assignee,
                "priority" | "pri" => FilterField::Priority,
                "status" => FilterField::Status,
                "due" => FilterField::DueDate,
                _ => FilterField::CustomField(field.to_string()),
            };

            let operator = if negated {
                FilterOperator::NotEquals
            } else {
                FilterOperator::Equals
            };

            Some(FilterCondition::new(filter_field, operator, value))
        } else {
            // Plain text - full-text search
            Some(FilterCondition::contains(FilterField::FullText, token))
        }
    }
}

/// Filter and search manager
#[derive(Debug)]
pub struct FilterManager {
    /// All saved filters
    saved_filters: HashMap<String, Filter>,
    /// Currently active filter
    active_filter: Option<Filter>,
    /// Quick filters available
    quick_filters: Vec<QuickFilter>,
    /// Current user for "me" queries
    current_user: String,
    /// Search history
    search_history: Vec<String>,
    /// Maximum search history size
    max_history: usize,
}

impl FilterManager {
    /// Create a new filter manager
    pub fn new(current_user: impl Into<String>) -> Self {
        Self {
            saved_filters: HashMap::new(),
            active_filter: None,
            quick_filters: vec![
                QuickFilter::All,
                QuickFilter::MyTasks,
                QuickFilter::DueToday,
                QuickFilter::DueThisWeek,
                QuickFilter::HighPriority,
                QuickFilter::Overdue,
                QuickFilter::Unassigned,
                QuickFilter::InProgress,
                QuickFilter::Completed,
            ],
            current_user: current_user.into(),
            search_history: Vec::new(),
            max_history: 50,
        }
    }

    /// Apply a quick filter
    pub fn apply_quick_filter(&mut self, quick_filter: QuickFilter) {
        self.active_filter = Some(quick_filter.to_filter(&self.current_user));
    }

    /// Parse and apply a search query
    pub fn search(&mut self, query: impl Into<String>) -> &Filter {
        let query = query.into();
        self.add_to_history(query.clone());
        self.active_filter = Some(SearchParser::parse(&query));
        self.active_filter.as_ref().unwrap()
    }

    /// Save the current filter
    pub fn save_current_filter(&mut self, name: impl Into<String>) -> Option<String> {
        if let Some(mut filter) = self.active_filter.clone() {
            filter.name = name.into();
            filter.mark_saved();
            let id = filter.id.clone();
            self.saved_filters.insert(id.clone(), filter);
            Some(id)
        } else {
            None
        }
    }

    /// Load a saved filter
    pub fn load_filter(&mut self, filter_id: &str) -> bool {
        if let Some(filter) = self.saved_filters.get(filter_id) {
            self.active_filter = Some(filter.clone());
            true
        } else {
            false
        }
    }

    /// Delete a saved filter
    pub fn delete_filter(&mut self, filter_id: &str) -> bool {
        self.saved_filters.remove(filter_id).is_some()
    }

    /// Get all saved filters
    pub fn saved_filters(&self) -> Vec<&Filter> {
        self.saved_filters.values().collect()
    }

    /// Get current active filter
    pub fn active_filter(&self) -> Option<&Filter> {
        self.active_filter.as_ref()
    }

    /// Clear active filter
    pub fn clear_filter(&mut self) {
        self.active_filter = None;
    }

    /// Get available quick filters
    pub fn quick_filters(&self) -> &[QuickFilter] {
        &self.quick_filters
    }

    /// Add query to search history
    fn add_to_history(&mut self, query: String) {
        if !query.is_empty() && !self.search_history.contains(&query) {
            self.search_history.insert(0, query);
            if self.search_history.len() > self.max_history {
                self.search_history.truncate(self.max_history);
            }
        }
    }

    /// Get search history
    pub fn search_history(&self) -> &[String] {
        &self.search_history
    }

    /// Clear search history
    pub fn clear_history(&mut self) {
        self.search_history.clear();
    }

    /// Create a custom filter
    pub fn create_filter(&mut self, name: impl Into<String>) -> Filter {
        Filter::new(name)
    }

    /// Apply a custom filter
    pub fn apply_filter(&mut self, filter: Filter) {
        self.active_filter = Some(filter);
    }
}

impl Default for FilterManager {
    fn default() -> Self {
        Self::new("me")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_condition_creation() {
        let condition = FilterCondition::equals(FilterField::Title, "test");
        assert_eq!(condition.field, FilterField::Title);
        assert_eq!(condition.operator, FilterOperator::Equals);
        assert_eq!(condition.value, "test");
    }

    #[test]
    fn test_filter_condition_contains() {
        let condition = FilterCondition::contains(FilterField::Description, "bug");
        assert_eq!(condition.operator, FilterOperator::Contains);
        assert_eq!(condition.value, "bug");
    }

    #[test]
    fn test_filter_condition_not_contains() {
        let condition = FilterCondition::not_contains(FilterField::Tags, "blocked");
        assert_eq!(condition.operator, FilterOperator::NotContains);
        assert_eq!(condition.value, "blocked");
    }

    #[test]
    fn test_filter_creation() {
        let filter = Filter::new("My Filter");
        assert_eq!(filter.name, "My Filter");
        assert!(filter.conditions.is_empty());
        assert_eq!(filter.logic, LogicalOperator::And);
        assert!(!filter.saved);
    }

    #[test]
    fn test_filter_add_condition() {
        let mut filter = Filter::new("Test");
        filter.add_condition(FilterCondition::equals(FilterField::Status, "open"));
        assert_eq!(filter.conditions.len(), 1);
    }

    #[test]
    fn test_filter_with_logic() {
        let filter = Filter::new("Test").with_logic(LogicalOperator::Or);
        assert_eq!(filter.logic, LogicalOperator::Or);
    }

    #[test]
    fn test_filter_mark_saved() {
        let mut filter = Filter::new("Test");
        assert!(!filter.saved);
        filter.mark_saved();
        assert!(filter.saved);
    }

    #[test]
    fn test_filter_is_empty() {
        let filter = Filter::new("Test");
        assert!(filter.is_empty());

        let mut filter = Filter::new("Test");
        filter.add_condition(FilterCondition::equals(FilterField::Title, "test"));
        assert!(!filter.is_empty());
    }

    #[test]
    fn test_quick_filter_names() {
        assert_eq!(QuickFilter::MyTasks.name(), "My Tasks");
        assert_eq!(QuickFilter::DueToday.name(), "Due Today");
        assert_eq!(QuickFilter::HighPriority.name(), "High Priority");
    }

    #[test]
    fn test_quick_filter_my_tasks() {
        let filter = QuickFilter::MyTasks.to_filter("alice");
        assert_eq!(filter.name, "My Tasks");
        assert_eq!(filter.conditions.len(), 1);
        assert_eq!(filter.conditions[0].value, "alice");
    }

    #[test]
    fn test_quick_filter_high_priority() {
        let filter = QuickFilter::HighPriority.to_filter("alice");
        assert_eq!(filter.conditions.len(), 1);
        assert_eq!(filter.conditions[0].value, "high");
    }

    #[test]
    fn test_quick_filter_all() {
        let filter = QuickFilter::All.to_filter("alice");
        assert!(filter.is_empty());
    }

    #[test]
    fn test_search_parser_simple() {
        let filter = SearchParser::parse("assignee:alice");
        assert_eq!(filter.conditions.len(), 1);
        assert_eq!(filter.conditions[0].value, "alice");
    }

    #[test]
    fn test_search_parser_multiple() {
        let filter = SearchParser::parse("assignee:alice priority:high");
        assert_eq!(filter.conditions.len(), 2);
    }

    #[test]
    fn test_search_parser_negation() {
        let filter = SearchParser::parse("-tag:blocked");
        assert_eq!(filter.conditions.len(), 1);
        assert_eq!(filter.conditions[0].operator, FilterOperator::NotEquals);
    }

    #[test]
    fn test_search_parser_full_text() {
        let filter = SearchParser::parse("bug fix");
        assert_eq!(filter.conditions.len(), 2);
        assert!(matches!(filter.conditions[0].field, FilterField::FullText));
    }

    #[test]
    fn test_search_parser_quoted() {
        let filter = SearchParser::parse(r#""urgent bug""#);
        assert_eq!(filter.conditions.len(), 1);
        assert_eq!(filter.conditions[0].value, "urgent bug");
    }

    #[test]
    fn test_filter_manager_creation() {
        let manager = FilterManager::new("alice");
        assert_eq!(manager.current_user, "alice");
        assert!(manager.active_filter.is_none());
        assert_eq!(manager.quick_filters.len(), 9);
    }

    #[test]
    fn test_filter_manager_apply_quick_filter() {
        let mut manager = FilterManager::new("alice");
        manager.apply_quick_filter(QuickFilter::MyTasks);
        assert!(manager.active_filter.is_some());
        assert_eq!(manager.active_filter().unwrap().name, "My Tasks");
    }

    #[test]
    fn test_filter_manager_search() {
        let mut manager = FilterManager::new("alice");
        manager.search("priority:high");
        assert!(manager.active_filter.is_some());
        assert_eq!(manager.search_history.len(), 1);
    }

    #[test]
    fn test_filter_manager_save_filter() {
        let mut manager = FilterManager::new("alice");
        manager.search("assignee:alice status:open");
        let id = manager.save_current_filter("My Open Tasks");
        assert!(id.is_some());
        assert_eq!(manager.saved_filters.len(), 1);
    }

    #[test]
    fn test_filter_manager_load_filter() {
        let mut manager = FilterManager::new("alice");
        manager.search("priority:high");
        let id = manager.save_current_filter("High Priority").unwrap();

        manager.clear_filter();
        assert!(manager.active_filter.is_none());

        let loaded = manager.load_filter(&id);
        assert!(loaded);
        assert!(manager.active_filter.is_some());
    }

    #[test]
    fn test_filter_manager_delete_filter() {
        let mut manager = FilterManager::new("alice");
        manager.search("priority:high");
        let id = manager.save_current_filter("Test").unwrap();

        assert_eq!(manager.saved_filters.len(), 1);
        let deleted = manager.delete_filter(&id);
        assert!(deleted);
        assert_eq!(manager.saved_filters.len(), 0);
    }

    #[test]
    fn test_filter_manager_clear_filter() {
        let mut manager = FilterManager::new("alice");
        manager.search("test");
        assert!(manager.active_filter.is_some());

        manager.clear_filter();
        assert!(manager.active_filter.is_none());
    }

    #[test]
    fn test_filter_manager_search_history() {
        let mut manager = FilterManager::new("alice");
        manager.search("query1");
        manager.search("query2");
        manager.search("query3");

        assert_eq!(manager.search_history().len(), 3);
        assert_eq!(manager.search_history()[0], "query3");
    }

    #[test]
    fn test_filter_manager_clear_history() {
        let mut manager = FilterManager::new("alice");
        manager.search("test");
        assert_eq!(manager.search_history().len(), 1);

        manager.clear_history();
        assert_eq!(manager.search_history().len(), 0);
    }

    #[test]
    fn test_filter_manager_custom_filter() {
        let mut manager = FilterManager::new("alice");
        let mut filter = manager.create_filter("Custom");
        filter.add_condition(FilterCondition::equals(FilterField::Priority, "urgent"));

        manager.apply_filter(filter);
        assert!(manager.active_filter.is_some());
    }
}
