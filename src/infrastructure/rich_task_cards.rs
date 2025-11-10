//! Rich Task Cards module for comprehensive task management
//!
//! This module provides a sophisticated task card system inspired by Trello, Asana, and Notion.
//! It supports subtasks, multiple assignees, custom fields, effort estimation, and progress tracking.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Priority levels for tasks
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Priority {
    /// P0 - Critical priority
    Critical,
    /// P1 - High priority
    High,
    /// P2 - Medium priority
    Medium,
    /// P3 - Low priority
    Low,
}

impl Priority {
    /// Returns the priority as a numeric value (0 = Critical, 3 = Low)
    pub fn to_value(&self) -> u8 {
        match self {
            Priority::Critical => 0,
            Priority::High => 1,
            Priority::Medium => 2,
            Priority::Low => 3,
        }
    }

    /// Returns the color associated with this priority (for UI rendering)
    pub fn color(&self) -> &'static str {
        match self {
            Priority::Critical => "#FF0000", // Red
            Priority::High => "#FFA500",     // Orange
            Priority::Medium => "#FFFF00",   // Yellow
            Priority::Low => "#00FF00",      // Green
        }
    }
}

/// Effort estimation for tasks
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EffortEstimate {
    /// Story points (e.g., Fibonacci: 1, 2, 3, 5, 8, 13)
    StoryPoints(u32),
    /// Time estimate in hours
    Hours(f32),
    /// Time estimate in days
    Days(f32),
}

impl EffortEstimate {
    /// Convert effort estimate to hours for comparison
    pub fn to_hours(&self) -> f32 {
        match self {
            EffortEstimate::StoryPoints(points) => *points as f32 * 2.0, // Assume 2 hours per point
            EffortEstimate::Hours(h) => *h,
            EffortEstimate::Days(d) => d * 8.0, // Assume 8-hour workday
        }
    }
}

/// Custom field type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CustomFieldType {
    /// Text field
    Text(String),
    /// Number field
    Number(f64),
    /// Dropdown selection
    Dropdown(String, Vec<String>), // (selected, options)
    /// Date field
    Date(DateTime<Utc>),
}

/// Custom field definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomField {
    /// Field name
    pub name: String,
    /// Field value
    pub value: CustomFieldType,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
}

/// Tag/Label for categorization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    /// Tag ID
    pub id: String,
    /// Tag name
    pub name: String,
    /// Tag color (hex code)
    pub color: String,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
}

/// Assignee information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Assignee {
    /// User ID
    pub user_id: String,
    /// Display name
    pub display_name: String,
    /// Avatar URL or emoji
    pub avatar: Option<String>,
    /// Assigned timestamp
    pub assigned_at: DateTime<Utc>,
}

/// Subtask/Checklist item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subtask {
    /// Subtask ID
    pub id: String,
    /// Subtask title
    pub title: String,
    /// Completion status
    pub completed: bool,
    /// Completed timestamp
    pub completed_at: Option<DateTime<Utc>>,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
    /// Assigned user
    pub assignee: Option<String>,
}

/// Rich task card with comprehensive metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichTaskCard {
    /// Task ID
    pub id: String,
    /// Task title
    pub title: String,
    /// Task description (supports Markdown)
    pub description: String,
    /// Current status/column
    pub status: String,
    /// Priority level
    pub priority: Priority,
    /// Multiple assignees
    pub assignees: Vec<Assignee>,
    /// Tags/Labels
    pub tags: Vec<Tag>,
    /// Due date with time
    pub due_date: Option<DateTime<Utc>>,
    /// Recurrence pattern (if recurring)
    pub recurrence: Option<String>, // "daily", "weekly", "monthly"
    /// Effort estimate
    pub effort_estimate: Option<EffortEstimate>,
    /// Progress percentage (0-100)
    pub progress: u8,
    /// Subtasks/Checklist
    pub subtasks: Vec<Subtask>,
    /// Custom fields
    pub custom_fields: HashMap<String, CustomField>,
    /// Cover image URL
    pub cover_image: Option<String>,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
    /// Last updated timestamp
    pub updated_at: DateTime<Utc>,
    /// Created by user ID
    pub created_by: String,
}

impl RichTaskCard {
    /// Creates a new rich task card
    pub fn new(
        id: String,
        title: String,
        description: String,
        status: String,
        priority: Priority,
        created_by: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id,
            title,
            description,
            status,
            priority,
            assignees: Vec::new(),
            tags: Vec::new(),
            due_date: None,
            recurrence: None,
            effort_estimate: None,
            progress: 0,
            subtasks: Vec::new(),
            custom_fields: HashMap::new(),
            cover_image: None,
            created_at: now,
            updated_at: now,
            created_by,
        }
    }

    /// Adds an assignee to the task
    pub fn add_assignee(&mut self, assignee: Assignee) {
        // Check if already assigned
        if !self.assignees.iter().any(|a| a.user_id == assignee.user_id) {
            self.assignees.push(assignee);
            self.updated_at = Utc::now();
        }
    }

    /// Removes an assignee from the task
    pub fn remove_assignee(&mut self, user_id: &str) {
        self.assignees.retain(|a| a.user_id != user_id);
        self.updated_at = Utc::now();
    }

    /// Adds a tag to the task
    pub fn add_tag(&mut self, tag: Tag) {
        // Check if tag already exists
        if !self.tags.iter().any(|t| t.id == tag.id) {
            self.tags.push(tag);
            self.updated_at = Utc::now();
        }
    }

    /// Removes a tag from the task
    pub fn remove_tag(&mut self, tag_id: &str) {
        self.tags.retain(|t| t.id != tag_id);
        self.updated_at = Utc::now();
    }

    /// Adds a subtask to the task
    pub fn add_subtask(&mut self, subtask: Subtask) {
        self.subtasks.push(subtask);
        self.update_progress();
        self.updated_at = Utc::now();
    }

    /// Marks a subtask as completed
    pub fn complete_subtask(&mut self, subtask_id: &str) -> bool {
        if let Some(subtask) = self.subtasks.iter_mut().find(|s| s.id == subtask_id)
            && !subtask.completed
        {
            subtask.completed = true;
            subtask.completed_at = Some(Utc::now());
            self.update_progress();
            self.updated_at = Utc::now();
            return true;
        }
        false
    }

    /// Marks a subtask as incomplete
    pub fn uncomplete_subtask(&mut self, subtask_id: &str) -> bool {
        if let Some(subtask) = self.subtasks.iter_mut().find(|s| s.id == subtask_id)
            && subtask.completed
        {
            subtask.completed = false;
            subtask.completed_at = None;
            self.update_progress();
            self.updated_at = Utc::now();
            return true;
        }
        false
    }

    /// Updates the progress based on completed subtasks
    pub fn update_progress(&mut self) {
        if self.subtasks.is_empty() {
            self.progress = 0;
        } else {
            let completed = self.subtasks.iter().filter(|s| s.completed).count();
            self.progress = ((completed as f32 / self.subtasks.len() as f32) * 100.0) as u8;
        }
    }

    /// Sets a custom field
    pub fn set_custom_field(&mut self, field: CustomField) {
        self.custom_fields.insert(field.name.clone(), field);
        self.updated_at = Utc::now();
    }

    /// Gets a custom field by name
    pub fn get_custom_field(&self, name: &str) -> Option<&CustomField> {
        self.custom_fields.get(name)
    }

    /// Removes a custom field
    pub fn remove_custom_field(&mut self, name: &str) -> Option<CustomField> {
        let field = self.custom_fields.remove(name);
        if field.is_some() {
            self.updated_at = Utc::now();
        }
        field
    }

    /// Calculates completion percentage including manual progress
    pub fn completion_percentage(&self) -> u8 {
        self.progress
    }

    /// Checks if the task is overdue
    pub fn is_overdue(&self) -> bool {
        if let Some(due_date) = self.due_date {
            due_date < Utc::now()
        } else {
            false
        }
    }

    /// Gets the number of completed subtasks
    pub fn completed_subtasks(&self) -> usize {
        self.subtasks.iter().filter(|s| s.completed).count()
    }

    /// Gets the total number of subtasks
    pub fn total_subtasks(&self) -> usize {
        self.subtasks.len()
    }
}

/// Manager for rich task cards
#[derive(Debug)]
pub struct RichTaskCardManager {
    cards: HashMap<String, RichTaskCard>,
    next_card_id: usize,
    next_subtask_id: usize,
    next_tag_id: usize,
}

impl RichTaskCardManager {
    /// Creates a new rich task card manager
    pub fn new() -> Self {
        Self {
            cards: HashMap::new(),
            next_card_id: 1,
            next_subtask_id: 1,
            next_tag_id: 1,
        }
    }

    /// Creates a new task card
    pub fn create_card(
        &mut self,
        title: String,
        description: String,
        status: String,
        priority: Priority,
        created_by: String,
    ) -> String {
        let id = format!("card-{}", self.next_card_id);
        self.next_card_id += 1;

        let card = RichTaskCard::new(id.clone(), title, description, status, priority, created_by);
        self.cards.insert(id.clone(), card);
        id
    }

    /// Gets a task card by ID
    pub fn get_card(&self, card_id: &str) -> Option<&RichTaskCard> {
        self.cards.get(card_id)
    }

    /// Gets a mutable task card by ID
    pub fn get_card_mut(&mut self, card_id: &str) -> Option<&mut RichTaskCard> {
        self.cards.get_mut(card_id)
    }

    /// Deletes a task card
    pub fn delete_card(&mut self, card_id: &str) -> Option<RichTaskCard> {
        self.cards.remove(card_id)
    }

    /// Creates a new subtask for a card
    pub fn create_subtask(&mut self, card_id: &str, title: String) -> Option<String> {
        let subtask_id = format!("subtask-{}", self.next_subtask_id);
        self.next_subtask_id += 1;

        let subtask = Subtask {
            id: subtask_id.clone(),
            title,
            completed: false,
            completed_at: None,
            created_at: Utc::now(),
            assignee: None,
        };

        if let Some(card) = self.get_card_mut(card_id) {
            card.add_subtask(subtask);
            Some(subtask_id)
        } else {
            None
        }
    }

    /// Creates a new tag
    pub fn create_tag(&mut self, name: String, color: String) -> Tag {
        let id = format!("tag-{}", self.next_tag_id);
        self.next_tag_id += 1;

        Tag {
            id,
            name,
            color,
            created_at: Utc::now(),
        }
    }

    /// Gets all cards with a specific tag
    pub fn cards_with_tag(&self, tag_id: &str) -> Vec<&RichTaskCard> {
        self.cards
            .values()
            .filter(|card| card.tags.iter().any(|t| t.id == tag_id))
            .collect()
    }

    /// Gets all cards assigned to a user
    pub fn cards_assigned_to(&self, user_id: &str) -> Vec<&RichTaskCard> {
        self.cards
            .values()
            .filter(|card| card.assignees.iter().any(|a| a.user_id == user_id))
            .collect()
    }

    /// Gets all overdue cards
    pub fn overdue_cards(&self) -> Vec<&RichTaskCard> {
        self.cards
            .values()
            .filter(|card| card.is_overdue())
            .collect()
    }

    /// Gets all cards by status
    pub fn cards_by_status(&self, status: &str) -> Vec<&RichTaskCard> {
        self.cards
            .values()
            .filter(|card| card.status == status)
            .collect()
    }

    /// Gets all cards by priority
    pub fn cards_by_priority(&self, priority: Priority) -> Vec<&RichTaskCard> {
        self.cards
            .values()
            .filter(|card| card.priority == priority)
            .collect()
    }

    /// Gets total number of cards
    pub fn total_cards(&self) -> usize {
        self.cards.len()
    }
}

impl Default for RichTaskCardManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority_values() {
        assert_eq!(Priority::Critical.to_value(), 0);
        assert_eq!(Priority::High.to_value(), 1);
        assert_eq!(Priority::Medium.to_value(), 2);
        assert_eq!(Priority::Low.to_value(), 3);
    }

    #[test]
    fn test_priority_colors() {
        assert_eq!(Priority::Critical.color(), "#FF0000");
        assert_eq!(Priority::High.color(), "#FFA500");
        assert_eq!(Priority::Medium.color(), "#FFFF00");
        assert_eq!(Priority::Low.color(), "#00FF00");
    }

    #[test]
    fn test_effort_estimate_to_hours() {
        let points = EffortEstimate::StoryPoints(5);
        assert_eq!(points.to_hours(), 10.0); // 5 points * 2 hours

        let hours = EffortEstimate::Hours(4.5);
        assert_eq!(hours.to_hours(), 4.5);

        let days = EffortEstimate::Days(2.0);
        assert_eq!(days.to_hours(), 16.0); // 2 days * 8 hours
    }

    #[test]
    fn test_create_card() {
        let mut manager = RichTaskCardManager::new();
        let card_id = manager.create_card(
            "Test Task".to_string(),
            "Description".to_string(),
            "Todo".to_string(),
            Priority::High,
            "user-1".to_string(),
        );

        assert_eq!(card_id, "card-1");
        let card = manager.get_card(&card_id).unwrap();
        assert_eq!(card.title, "Test Task");
        assert_eq!(card.priority, Priority::High);
        assert_eq!(card.progress, 0);
    }

    #[test]
    fn test_add_remove_assignee() {
        let mut manager = RichTaskCardManager::new();
        let card_id = manager.create_card(
            "Test".to_string(),
            "Desc".to_string(),
            "Todo".to_string(),
            Priority::Medium,
            "user-1".to_string(),
        );

        let assignee = Assignee {
            user_id: "user-2".to_string(),
            display_name: "Alice".to_string(),
            avatar: None,
            assigned_at: Utc::now(),
        };

        let card = manager.get_card_mut(&card_id).unwrap();
        card.add_assignee(assignee);
        assert_eq!(card.assignees.len(), 1);

        card.remove_assignee("user-2");
        assert_eq!(card.assignees.len(), 0);
    }

    #[test]
    fn test_add_duplicate_assignee() {
        let mut card = RichTaskCard::new(
            "card-1".to_string(),
            "Test".to_string(),
            "Desc".to_string(),
            "Todo".to_string(),
            Priority::Medium,
            "user-1".to_string(),
        );

        let assignee = Assignee {
            user_id: "user-2".to_string(),
            display_name: "Alice".to_string(),
            avatar: None,
            assigned_at: Utc::now(),
        };

        card.add_assignee(assignee.clone());
        card.add_assignee(assignee);
        assert_eq!(card.assignees.len(), 1); // Should not add duplicate
    }

    #[test]
    fn test_add_remove_tags() {
        let mut manager = RichTaskCardManager::new();
        let card_id = manager.create_card(
            "Test".to_string(),
            "Desc".to_string(),
            "Todo".to_string(),
            Priority::Medium,
            "user-1".to_string(),
        );

        let tag = manager.create_tag("Bug".to_string(), "#FF0000".to_string());
        let tag_id = tag.id.clone();

        let card = manager.get_card_mut(&card_id).unwrap();
        card.add_tag(tag);
        assert_eq!(card.tags.len(), 1);

        card.remove_tag(&tag_id);
        assert_eq!(card.tags.len(), 0);
    }

    #[test]
    fn test_subtask_completion() {
        let mut manager = RichTaskCardManager::new();
        let card_id = manager.create_card(
            "Test".to_string(),
            "Desc".to_string(),
            "Todo".to_string(),
            Priority::Medium,
            "user-1".to_string(),
        );

        let subtask_id = manager
            .create_subtask(&card_id, "Subtask 1".to_string())
            .unwrap();

        let card = manager.get_card_mut(&card_id).unwrap();
        assert_eq!(card.total_subtasks(), 1);
        assert_eq!(card.completed_subtasks(), 0);
        assert_eq!(card.progress, 0);

        card.complete_subtask(&subtask_id);
        assert_eq!(card.completed_subtasks(), 1);
        assert_eq!(card.progress, 100);

        card.uncomplete_subtask(&subtask_id);
        assert_eq!(card.completed_subtasks(), 0);
        assert_eq!(card.progress, 0);
    }

    #[test]
    fn test_multiple_subtasks_progress() {
        let mut manager = RichTaskCardManager::new();
        let card_id = manager.create_card(
            "Test".to_string(),
            "Desc".to_string(),
            "Todo".to_string(),
            Priority::Medium,
            "user-1".to_string(),
        );

        let subtask1 = manager
            .create_subtask(&card_id, "Subtask 1".to_string())
            .unwrap();
        let subtask2 = manager
            .create_subtask(&card_id, "Subtask 2".to_string())
            .unwrap();
        manager.create_subtask(&card_id, "Subtask 3".to_string());

        let card = manager.get_card_mut(&card_id).unwrap();
        assert_eq!(card.total_subtasks(), 3);
        assert_eq!(card.progress, 0);

        card.complete_subtask(&subtask1);
        assert_eq!(card.progress, 33); // 1/3 = 33%

        card.complete_subtask(&subtask2);
        assert_eq!(card.progress, 66); // 2/3 = 66%
    }

    #[test]
    fn test_custom_fields() {
        let mut card = RichTaskCard::new(
            "card-1".to_string(),
            "Test".to_string(),
            "Desc".to_string(),
            "Todo".to_string(),
            Priority::Medium,
            "user-1".to_string(),
        );

        let field = CustomField {
            name: "Environment".to_string(),
            value: CustomFieldType::Dropdown(
                "Production".to_string(),
                vec![
                    "Development".to_string(),
                    "Staging".to_string(),
                    "Production".to_string(),
                ],
            ),
            created_at: Utc::now(),
        };

        card.set_custom_field(field);
        assert_eq!(card.custom_fields.len(), 1);

        let retrieved = card.get_custom_field("Environment").unwrap();
        match &retrieved.value {
            CustomFieldType::Dropdown(selected, _) => assert_eq!(selected, "Production"),
            _ => panic!("Expected dropdown field"),
        }

        card.remove_custom_field("Environment");
        assert_eq!(card.custom_fields.len(), 0);
    }

    #[test]
    fn test_is_overdue() {
        let mut card = RichTaskCard::new(
            "card-1".to_string(),
            "Test".to_string(),
            "Desc".to_string(),
            "Todo".to_string(),
            Priority::Medium,
            "user-1".to_string(),
        );

        assert!(!card.is_overdue()); // No due date

        // Set due date in the past
        card.due_date = Some(Utc::now() - chrono::Duration::days(1));
        assert!(card.is_overdue());

        // Set due date in the future
        card.due_date = Some(Utc::now() + chrono::Duration::days(1));
        assert!(!card.is_overdue());
    }

    #[test]
    fn test_cards_with_tag() {
        let mut manager = RichTaskCardManager::new();
        let card1_id = manager.create_card(
            "Card 1".to_string(),
            "Desc".to_string(),
            "Todo".to_string(),
            Priority::High,
            "user-1".to_string(),
        );
        let card2_id = manager.create_card(
            "Card 2".to_string(),
            "Desc".to_string(),
            "Todo".to_string(),
            Priority::Medium,
            "user-1".to_string(),
        );

        let tag = manager.create_tag("Bug".to_string(), "#FF0000".to_string());
        let tag_id = tag.id.clone();

        manager
            .get_card_mut(&card1_id)
            .unwrap()
            .add_tag(tag.clone());
        manager.get_card_mut(&card2_id).unwrap().add_tag(tag);

        let cards = manager.cards_with_tag(&tag_id);
        assert_eq!(cards.len(), 2);
    }

    #[test]
    fn test_cards_assigned_to() {
        let mut manager = RichTaskCardManager::new();
        let card_id = manager.create_card(
            "Card 1".to_string(),
            "Desc".to_string(),
            "Todo".to_string(),
            Priority::High,
            "user-1".to_string(),
        );

        let assignee = Assignee {
            user_id: "user-2".to_string(),
            display_name: "Alice".to_string(),
            avatar: None,
            assigned_at: Utc::now(),
        };

        manager
            .get_card_mut(&card_id)
            .unwrap()
            .add_assignee(assignee);

        let cards = manager.cards_assigned_to("user-2");
        assert_eq!(cards.len(), 1);
        assert_eq!(cards[0].id, card_id);
    }

    #[test]
    fn test_overdue_cards() {
        let mut manager = RichTaskCardManager::new();
        let card1_id = manager.create_card(
            "Card 1".to_string(),
            "Desc".to_string(),
            "Todo".to_string(),
            Priority::High,
            "user-1".to_string(),
        );
        let card2_id = manager.create_card(
            "Card 2".to_string(),
            "Desc".to_string(),
            "Todo".to_string(),
            Priority::Medium,
            "user-1".to_string(),
        );

        // Set card1 overdue
        manager.get_card_mut(&card1_id).unwrap().due_date =
            Some(Utc::now() - chrono::Duration::days(1));

        // Set card2 not overdue
        manager.get_card_mut(&card2_id).unwrap().due_date =
            Some(Utc::now() + chrono::Duration::days(1));

        let overdue = manager.overdue_cards();
        assert_eq!(overdue.len(), 1);
        assert_eq!(overdue[0].id, card1_id);
    }

    #[test]
    fn test_cards_by_status() {
        let mut manager = RichTaskCardManager::new();
        manager.create_card(
            "Card 1".to_string(),
            "Desc".to_string(),
            "Todo".to_string(),
            Priority::High,
            "user-1".to_string(),
        );
        manager.create_card(
            "Card 2".to_string(),
            "Desc".to_string(),
            "In Progress".to_string(),
            Priority::Medium,
            "user-1".to_string(),
        );
        manager.create_card(
            "Card 3".to_string(),
            "Desc".to_string(),
            "Todo".to_string(),
            Priority::Low,
            "user-1".to_string(),
        );

        let todo_cards = manager.cards_by_status("Todo");
        assert_eq!(todo_cards.len(), 2);

        let in_progress_cards = manager.cards_by_status("In Progress");
        assert_eq!(in_progress_cards.len(), 1);
    }

    #[test]
    fn test_cards_by_priority() {
        let mut manager = RichTaskCardManager::new();
        manager.create_card(
            "Card 1".to_string(),
            "Desc".to_string(),
            "Todo".to_string(),
            Priority::High,
            "user-1".to_string(),
        );
        manager.create_card(
            "Card 2".to_string(),
            "Desc".to_string(),
            "Todo".to_string(),
            Priority::High,
            "user-1".to_string(),
        );
        manager.create_card(
            "Card 3".to_string(),
            "Desc".to_string(),
            "Todo".to_string(),
            Priority::Low,
            "user-1".to_string(),
        );

        let high_priority = manager.cards_by_priority(Priority::High);
        assert_eq!(high_priority.len(), 2);

        let low_priority = manager.cards_by_priority(Priority::Low);
        assert_eq!(low_priority.len(), 1);
    }

    #[test]
    fn test_delete_card() {
        let mut manager = RichTaskCardManager::new();
        let card_id = manager.create_card(
            "Test".to_string(),
            "Desc".to_string(),
            "Todo".to_string(),
            Priority::Medium,
            "user-1".to_string(),
        );

        assert_eq!(manager.total_cards(), 1);

        let deleted = manager.delete_card(&card_id);
        assert!(deleted.is_some());
        assert_eq!(manager.total_cards(), 0);
    }

    #[test]
    fn test_create_tag() {
        let mut manager = RichTaskCardManager::new();
        let tag1 = manager.create_tag("Bug".to_string(), "#FF0000".to_string());
        let tag2 = manager.create_tag("Feature".to_string(), "#00FF00".to_string());

        assert_eq!(tag1.id, "tag-1");
        assert_eq!(tag2.id, "tag-2");
        assert_eq!(tag1.name, "Bug");
        assert_eq!(tag2.color, "#00FF00");
    }

    #[test]
    fn test_subtask_with_assignee() {
        let mut subtask = Subtask {
            id: "subtask-1".to_string(),
            title: "Test Subtask".to_string(),
            completed: false,
            completed_at: None,
            created_at: Utc::now(),
            assignee: Some("user-1".to_string()),
        };

        assert_eq!(subtask.assignee, Some("user-1".to_string()));

        subtask.assignee = None;
        assert_eq!(subtask.assignee, None);
    }

    #[test]
    fn test_completion_percentage() {
        let mut card = RichTaskCard::new(
            "card-1".to_string(),
            "Test".to_string(),
            "Desc".to_string(),
            "Todo".to_string(),
            Priority::Medium,
            "user-1".to_string(),
        );

        assert_eq!(card.completion_percentage(), 0);

        card.progress = 50;
        assert_eq!(card.completion_percentage(), 50);
    }
}
