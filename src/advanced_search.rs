/// Advanced search and filtering system
///
/// Provides regex search, multi-field filters, filter history, and saved filters

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::path::Path;

/// Search result with field information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdvancedSearchMatch {
    /// Matched text
    pub text: String,
    /// Field name (if multi-field search)
    pub field: Option<String>,
    /// Line number
    pub line: usize,
    /// Column position
    pub column: usize,
    /// Match score/relevance
    pub score: usize,
}

/// Filter operator for multi-field queries
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FilterOperator {
    /// Equals
    Equals,
    /// Contains substring
    Contains,
    /// Starts with
    StartsWith,
    /// Ends with
    EndsWith,
    /// Matches regex
    Regex,
    /// Greater than
    GreaterThan,
    /// Less than
    LessThan,
}

/// A single filter condition
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FilterCondition {
    /// Field name to filter on
    pub field: String,
    /// Operator to apply
    pub operator: FilterOperator,
    /// Value to compare against
    pub value: String,
}

impl FilterCondition {
    /// Create a new filter condition
    pub fn new(field: impl Into<String>, operator: FilterOperator, value: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            operator,
            value: value.into(),
        }
    }

    /// Check if a field value matches this condition
    pub fn matches(&self, field_value: &str) -> bool {
        match self.operator {
            FilterOperator::Equals => field_value == self.value,
            FilterOperator::Contains => field_value.contains(&self.value),
            FilterOperator::StartsWith => field_value.starts_with(&self.value),
            FilterOperator::EndsWith => field_value.ends_with(&self.value),
            FilterOperator::Regex => {
                // Simplified regex - in production, use regex crate
                field_value.contains(&self.value)
            }
            FilterOperator::GreaterThan => {
                // Try to compare as numbers, fall back to string comparison
                match (field_value.parse::<f64>(), self.value.parse::<f64>()) {
                    (Ok(a), Ok(b)) => a > b,
                    _ => field_value > self.value.as_str(),
                }
            }
            FilterOperator::LessThan => {
                match (field_value.parse::<f64>(), self.value.parse::<f64>()) {
                    (Ok(a), Ok(b)) => a < b,
                    _ => field_value < self.value.as_str(),
                }
            }
        }
    }
}

/// Multi-field filter combining multiple conditions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MultiFieldFilter {
    /// Name/description of this filter
    pub name: String,
    /// Filter conditions (AND logic)
    pub conditions: Vec<FilterCondition>,
}

impl MultiFieldFilter {
    /// Create a new multi-field filter
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            conditions: Vec::new(),
        }
    }

    /// Add a condition
    pub fn add_condition(&mut self, condition: FilterCondition) {
        self.conditions.push(condition);
    }

    /// Check if all conditions match
    pub fn matches(&self, fields: &std::collections::HashMap<String, String>) -> bool {
        for condition in &self.conditions {
            if let Some(field_value) = fields.get(&condition.field) {
                if !condition.matches(field_value) {
                    return false;
                }
            } else {
                // Field doesn't exist, condition fails
                return false;
            }
        }
        true // All conditions matched
    }

    /// Get query string representation
    pub fn to_query_string(&self) -> String {
        self.conditions
            .iter()
            .map(|c| format!("{}:{}", c.field, c.value))
            .collect::<Vec<_>>()
            .join(" AND ")
    }
}

/// Filter history entry
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FilterHistoryEntry {
    /// The filter that was used
    pub filter: MultiFieldFilter,
    /// Timestamp when used
    pub timestamp: String,
    /// Number of results found
    pub result_count: usize,
}

/// Filter history manager
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterHistory {
    /// Recent filters (most recent first)
    history: VecDeque<FilterHistoryEntry>,
    /// Maximum history size
    max_size: usize,
}

impl FilterHistory {
    /// Create a new filter history
    pub fn new(max_size: usize) -> Self {
        Self {
            history: VecDeque::with_capacity(max_size),
            max_size,
        }
    }

    /// Add a filter to history
    pub fn add(&mut self, filter: MultiFieldFilter, result_count: usize) {
        let entry = FilterHistoryEntry {
            filter,
            timestamp: chrono::Utc::now().to_rfc3339(),
            result_count,
        };

        // Remove duplicates (same filter)
        self.history.retain(|e| e.filter != entry.filter);

        // Add to front
        self.history.push_front(entry);

        // Maintain max size
        while self.history.len() > self.max_size {
            self.history.pop_back();
        }
    }

    /// Get recent filters
    pub fn recent(&self, count: usize) -> Vec<&FilterHistoryEntry> {
        self.history.iter().take(count).collect()
    }

    /// Get all history entries
    pub fn all(&self) -> &VecDeque<FilterHistoryEntry> {
        &self.history
    }

    /// Clear history
    pub fn clear(&mut self) {
        self.history.clear();
    }

    /// Get entry by index
    pub fn get(&self, index: usize) -> Option<&FilterHistoryEntry> {
        self.history.get(index)
    }
}

impl Default for FilterHistory {
    fn default() -> Self {
        Self::new(50) // Default to 50 entries
    }
}

/// Saved filter collection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedFilters {
    /// Named saved filters
    filters: Vec<MultiFieldFilter>,
}

impl SavedFilters {
    /// Create a new saved filters collection
    pub fn new() -> Self {
        Self {
            filters: Vec::new(),
        }
    }

    /// Add a filter
    pub fn add(&mut self, filter: MultiFieldFilter) {
        // Remove existing filter with same name
        self.filters.retain(|f| f.name != filter.name);
        self.filters.push(filter);
    }

    /// Remove a filter by name
    pub fn remove(&mut self, name: &str) -> bool {
        let len = self.filters.len();
        self.filters.retain(|f| f.name != name);
        self.filters.len() < len
    }

    /// Get a filter by name
    pub fn get(&self, name: &str) -> Option<&MultiFieldFilter> {
        self.filters.iter().find(|f| f.name == name)
    }

    /// Get all filters
    pub fn all(&self) -> &[MultiFieldFilter] {
        &self.filters
    }

    /// Get filter names
    pub fn names(&self) -> Vec<String> {
        self.filters.iter().map(|f| f.name.clone()).collect()
    }

    /// Save to file
    pub fn save_to_file(&self, path: impl AsRef<Path>) -> std::io::Result<()> {
        let content = toml::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        std::fs::write(path, content)
    }

    /// Load from file
    pub fn load_from_file(path: impl AsRef<Path>) -> std::io::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        toml::from_str(&content)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }
}

impl Default for SavedFilters {
    fn default() -> Self {
        Self::new()
    }
}

/// Advanced search manager
pub struct AdvancedSearchManager {
    /// Filter history
    history: FilterHistory,
    /// Saved filters
    saved: SavedFilters,
    /// Current active filter
    current_filter: Option<MultiFieldFilter>,
}

impl AdvancedSearchManager {
    /// Create a new advanced search manager
    pub fn new() -> Self {
        Self {
            history: FilterHistory::default(),
            saved: SavedFilters::default(),
            current_filter: None,
        }
    }

    /// Set current filter
    pub fn set_filter(&mut self, filter: MultiFieldFilter) {
        self.current_filter = Some(filter);
    }

    /// Get current filter
    pub fn current_filter(&self) -> Option<&MultiFieldFilter> {
        self.current_filter.as_ref()
    }

    /// Execute current filter and add to history
    pub fn execute_filter(&mut self, result_count: usize) {
        if let Some(filter) = &self.current_filter {
            self.history.add(filter.clone(), result_count);
        }
    }

    /// Get filter history
    pub fn history(&self) -> &FilterHistory {
        &self.history
    }

    /// Get saved filters
    pub fn saved(&self) -> &SavedFilters {
        &self.saved
    }

    /// Get mutable saved filters
    pub fn saved_mut(&mut self) -> &mut SavedFilters {
        &mut self.saved
    }

    /// Save current filter
    pub fn save_current(&mut self, name: impl Into<String>) {
        if let Some(mut filter) = self.current_filter.clone() {
            filter.name = name.into();
            self.saved.add(filter);
        }
    }

    /// Load a saved filter by name
    pub fn load_saved(&mut self, name: &str) -> bool {
        if let Some(filter) = self.saved.get(name) {
            self.current_filter = Some(filter.clone());
            true
        } else {
            false
        }
    }
}

impl Default for AdvancedSearchManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_filter_condition_equals() {
        let condition = FilterCondition::new("status", FilterOperator::Equals, "active");
        assert!(condition.matches("active"));
        assert!(!condition.matches("inactive"));
    }

    #[test]
    fn test_filter_condition_contains() {
        let condition = FilterCondition::new("name", FilterOperator::Contains, "test");
        assert!(condition.matches("test123"));
        assert!(condition.matches("123test"));
        assert!(!condition.matches("hello"));
    }

    #[test]
    fn test_filter_condition_numbers() {
        let condition = FilterCondition::new("count", FilterOperator::GreaterThan, "10");
        assert!(condition.matches("15"));
        assert!(!condition.matches("5"));
    }

    #[test]
    fn test_multi_field_filter() {
        let mut filter = MultiFieldFilter::new("Active users");
        filter.add_condition(FilterCondition::new("status", FilterOperator::Equals, "active"));
        filter.add_condition(FilterCondition::new("age", FilterOperator::GreaterThan, "18"));

        let mut fields = HashMap::new();
        fields.insert("status".to_string(), "active".to_string());
        fields.insert("age".to_string(), "25".to_string());
        assert!(filter.matches(&fields));

        fields.insert("age".to_string(), "15".to_string());
        assert!(!filter.matches(&fields));
    }

    #[test]
    fn test_filter_history() {
        let mut history = FilterHistory::new(5);

        let filter1 = MultiFieldFilter::new("Filter 1");
        history.add(filter1.clone(), 10);

        let filter2 = MultiFieldFilter::new("Filter 2");
        history.add(filter2, 20);

        assert_eq!(history.all().len(), 2);
        assert_eq!(history.recent(1).len(), 1);

        // Adding same filter should deduplicate
        history.add(filter1, 15);
        assert_eq!(history.all().len(), 2);
    }

    #[test]
    fn test_saved_filters() {
        let mut saved = SavedFilters::new();

        let filter = MultiFieldFilter::new("My Filter");
        saved.add(filter.clone());

        assert_eq!(saved.all().len(), 1);
        assert!(saved.get("My Filter").is_some());
        assert!(saved.names().contains(&"My Filter".to_string()));

        assert!(saved.remove("My Filter"));
        assert_eq!(saved.all().len(), 0);
    }

    #[test]
    fn test_advanced_search_manager() {
        let mut manager = AdvancedSearchManager::new();

        let filter = MultiFieldFilter::new("Test");
        manager.set_filter(filter.clone());

        assert!(manager.current_filter().is_some());

        manager.execute_filter(5);
        assert_eq!(manager.history().all().len(), 1);

        manager.save_current("Saved Test");
        assert_eq!(manager.saved().all().len(), 1);

        assert!(manager.load_saved("Saved Test"));
    }

    #[test]
    fn test_filter_query_string() {
        let mut filter = MultiFieldFilter::new("Test");
        filter.add_condition(FilterCondition::new("field1", FilterOperator::Equals, "value1"));
        filter.add_condition(FilterCondition::new("field2", FilterOperator::Contains, "value2"));

        let query = filter.to_query_string();
        assert!(query.contains("field1:value1"));
        assert!(query.contains("field2:value2"));
    }
}
