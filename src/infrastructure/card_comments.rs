//! Card Comments System for threaded discussions on task cards
//!
//! This module provides a comprehensive commenting system for task cards,
//! inspired by Trello, Asana, and Notion. It supports threaded discussions,
//! @mentions, reactions, edit history, and Markdown formatting.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Emoji reaction to a comment
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Reaction {
    /// 👍 Thumbs up
    ThumbsUp,
    /// ❤️ Heart
    Heart,
    /// 🎉 Party popper
    Party,
    /// 😄 Smile
    Smile,
    /// 🚀 Rocket
    Rocket,
    /// 👀 Eyes
    Eyes,
}

impl Reaction {
    /// Returns the emoji representation
    pub fn emoji(&self) -> &'static str {
        match self {
            Reaction::ThumbsUp => "👍",
            Reaction::Heart => "❤️",
            Reaction::Party => "🎉",
            Reaction::Smile => "😄",
            Reaction::Rocket => "🚀",
            Reaction::Eyes => "👀",
        }
    }

    /// Returns all available reactions
    pub fn all() -> Vec<Reaction> {
        vec![
            Reaction::ThumbsUp,
            Reaction::Heart,
            Reaction::Party,
            Reaction::Smile,
            Reaction::Rocket,
            Reaction::Eyes,
        ]
    }
}

/// Reaction entry with user info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReactionEntry {
    /// User ID who reacted
    pub user_id: String,
    /// User display name
    pub user_name: String,
    /// Timestamp of reaction
    pub reacted_at: DateTime<Utc>,
}

/// Edit history entry for a comment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditHistory {
    /// Previous content
    pub previous_content: String,
    /// Edited timestamp
    pub edited_at: DateTime<Utc>,
    /// User who edited (usually same as comment author)
    pub edited_by: String,
}

/// Comment on a task card
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    /// Comment ID
    pub id: String,
    /// Card ID this comment belongs to
    pub card_id: String,
    /// Parent comment ID (for threaded replies)
    pub parent_id: Option<String>,
    /// Comment content (Markdown supported)
    pub content: String,
    /// Author user ID
    pub author_id: String,
    /// Author display name
    pub author_name: String,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
    /// Last updated timestamp
    pub updated_at: DateTime<Utc>,
    /// @mentioned user IDs
    pub mentions: Vec<String>,
    /// Reactions grouped by type
    pub reactions: HashMap<Reaction, Vec<ReactionEntry>>,
    /// Edit history
    pub edit_history: Vec<EditHistory>,
    /// Whether the comment is deleted
    pub deleted: bool,
}

impl Comment {
    /// Creates a new comment
    pub fn new(
        id: String,
        card_id: String,
        parent_id: Option<String>,
        content: String,
        author_id: String,
        author_name: String,
    ) -> Self {
        let now = Utc::now();
        let mentions = Self::extract_mentions(&content);

        Self {
            id,
            card_id,
            parent_id,
            content,
            author_id,
            author_name,
            created_at: now,
            updated_at: now,
            mentions,
            reactions: HashMap::new(),
            edit_history: Vec::new(),
            deleted: false,
        }
    }

    /// Extracts @mentions from content
    fn extract_mentions(content: &str) -> Vec<String> {
        let mut mentions = Vec::new();
        let words: Vec<&str> = content.split_whitespace().collect();

        for word in words {
            if let Some(mention) = word.strip_prefix('@') {
                // Remove trailing punctuation
                let clean_mention = mention.trim_end_matches(|c: char| c.is_ascii_punctuation());
                if !clean_mention.is_empty() {
                    mentions.push(clean_mention.to_string());
                }
            }
        }

        mentions
    }

    /// Edits the comment content
    pub fn edit(&mut self, new_content: String, edited_by: String) {
        let history = EditHistory {
            previous_content: self.content.clone(),
            edited_at: Utc::now(),
            edited_by,
        };

        self.edit_history.push(history);
        self.content = new_content.clone();
        self.mentions = Self::extract_mentions(&new_content);
        self.updated_at = Utc::now();
    }

    /// Adds a reaction to the comment
    pub fn add_reaction(&mut self, reaction: Reaction, user_id: String, user_name: String) {
        let entry = ReactionEntry {
            user_id: user_id.clone(),
            user_name,
            reacted_at: Utc::now(),
        };

        self.reactions
            .entry(reaction)
            .or_insert_with(Vec::new)
            .push(entry);
        self.updated_at = Utc::now();
    }

    /// Removes a reaction from the comment
    pub fn remove_reaction(&mut self, reaction: &Reaction, user_id: &str) -> bool {
        if let Some(entries) = self.reactions.get_mut(reaction) {
            let initial_len = entries.len();
            entries.retain(|e| e.user_id != user_id);
            if entries.len() < initial_len {
                self.updated_at = Utc::now();
                return true;
            }
        }
        false
    }

    /// Gets total reaction count
    pub fn total_reactions(&self) -> usize {
        self.reactions.values().map(|v| v.len()).sum()
    }

    /// Gets reaction count for a specific type
    pub fn reaction_count(&self, reaction: &Reaction) -> usize {
        self.reactions.get(reaction).map(|v| v.len()).unwrap_or(0)
    }

    /// Checks if a user has reacted with a specific reaction
    pub fn user_has_reacted(&self, reaction: &Reaction, user_id: &str) -> bool {
        self.reactions
            .get(reaction)
            .map(|entries| entries.iter().any(|e| e.user_id == user_id))
            .unwrap_or(false)
    }

    /// Marks the comment as deleted (soft delete)
    pub fn delete(&mut self) {
        self.deleted = true;
        self.updated_at = Utc::now();
    }

    /// Checks if the comment has been edited
    pub fn is_edited(&self) -> bool {
        !self.edit_history.is_empty()
    }

    /// Gets the number of edits
    pub fn edit_count(&self) -> usize {
        self.edit_history.len()
    }
}

/// Activity log entry for automated updates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityLogEntry {
    /// Activity ID
    pub id: String,
    /// Card ID this activity belongs to
    pub card_id: String,
    /// Activity type
    pub activity_type: String,
    /// Activity description
    pub description: String,
    /// User who triggered the activity
    pub user_id: Option<String>,
    /// User display name
    pub user_name: Option<String>,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl ActivityLogEntry {
    /// Creates a new activity log entry
    pub fn new(
        id: String,
        card_id: String,
        activity_type: String,
        description: String,
        user_id: Option<String>,
        user_name: Option<String>,
    ) -> Self {
        Self {
            id,
            card_id,
            activity_type,
            description,
            user_id,
            user_name,
            created_at: Utc::now(),
            metadata: HashMap::new(),
        }
    }

    /// Adds metadata to the activity
    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }
}

/// Comment thread manager
#[derive(Debug)]
pub struct CommentManager {
    comments: HashMap<String, Comment>,
    activities: Vec<ActivityLogEntry>,
    next_comment_id: usize,
    next_activity_id: usize,
    max_activities: usize,
}

impl CommentManager {
    /// Creates a new comment manager
    pub fn new() -> Self {
        Self {
            comments: HashMap::new(),
            activities: Vec::new(),
            next_comment_id: 1,
            next_activity_id: 1,
            max_activities: 1000,
        }
    }

    /// Creates a new comment manager with custom activity limit
    pub fn with_max_activities(max_activities: usize) -> Self {
        Self {
            comments: HashMap::new(),
            activities: Vec::new(),
            next_comment_id: 1,
            next_activity_id: 1,
            max_activities,
        }
    }

    /// Creates a new comment
    pub fn create_comment(
        &mut self,
        card_id: String,
        parent_id: Option<String>,
        content: String,
        author_id: String,
        author_name: String,
    ) -> String {
        let id = format!("comment-{}", self.next_comment_id);
        self.next_comment_id += 1;

        let comment = Comment::new(
            id.clone(),
            card_id.clone(),
            parent_id,
            content,
            author_id.clone(),
            author_name.clone(),
        );

        // Log activity
        self.log_activity(
            card_id,
            "comment_added".to_string(),
            format!("{} added a comment", author_name),
            Some(author_id),
            Some(author_name),
        );

        self.comments.insert(id.clone(), comment);
        id
    }

    /// Gets a comment by ID
    pub fn get_comment(&self, comment_id: &str) -> Option<&Comment> {
        self.comments.get(comment_id)
    }

    /// Gets a mutable comment by ID
    pub fn get_comment_mut(&mut self, comment_id: &str) -> Option<&mut Comment> {
        self.comments.get_mut(comment_id)
    }

    /// Deletes a comment (soft delete)
    pub fn delete_comment(&mut self, comment_id: &str) -> bool {
        if let Some(comment) = self.get_comment_mut(comment_id) {
            comment.delete();
            true
        } else {
            false
        }
    }

    /// Gets all comments for a card
    pub fn comments_for_card(&self, card_id: &str) -> Vec<&Comment> {
        self.comments
            .values()
            .filter(|c| c.card_id == card_id && !c.deleted)
            .collect()
    }

    /// Gets top-level comments for a card (no parent)
    pub fn top_level_comments(&self, card_id: &str) -> Vec<&Comment> {
        self.comments
            .values()
            .filter(|c| c.card_id == card_id && c.parent_id.is_none() && !c.deleted)
            .collect()
    }

    /// Gets replies to a comment
    pub fn replies_to_comment(&self, parent_id: &str) -> Vec<&Comment> {
        self.comments
            .values()
            .filter(|c| c.parent_id.as_deref() == Some(parent_id) && !c.deleted)
            .collect()
    }

    /// Gets comment thread (comment + all nested replies)
    pub fn get_thread(&self, comment_id: &str) -> Vec<&Comment> {
        let mut thread = Vec::new();

        // Add root comment
        if let Some(comment) = self.get_comment(comment_id) {
            if !comment.deleted {
                thread.push(comment);
            }
        }

        // Recursively add replies
        self.collect_replies(comment_id, &mut thread);

        thread
    }

    fn collect_replies<'a>(&'a self, parent_id: &str, thread: &mut Vec<&'a Comment>) {
        let replies = self.replies_to_comment(parent_id);
        for reply in replies {
            thread.push(reply);
            self.collect_replies(&reply.id, thread);
        }
    }

    /// Logs an activity entry
    pub fn log_activity(
        &mut self,
        card_id: String,
        activity_type: String,
        description: String,
        user_id: Option<String>,
        user_name: Option<String>,
    ) -> String {
        let id = format!("activity-{}", self.next_activity_id);
        self.next_activity_id += 1;

        let activity = ActivityLogEntry::new(
            id.clone(),
            card_id,
            activity_type,
            description,
            user_id,
            user_name,
        );

        self.activities.push(activity);

        // Trim old activities
        if self.activities.len() > self.max_activities {
            self.activities.drain(0..self.activities.len() - self.max_activities);
        }

        id
    }

    /// Gets activity log for a card
    pub fn activities_for_card(&self, card_id: &str) -> Vec<&ActivityLogEntry> {
        self.activities
            .iter()
            .filter(|a| a.card_id == card_id)
            .collect()
    }

    /// Gets recent activities (most recent first)
    pub fn recent_activities(&self, limit: usize) -> Vec<&ActivityLogEntry> {
        self.activities.iter().rev().take(limit).collect()
    }

    /// Gets total comment count
    pub fn total_comments(&self) -> usize {
        self.comments.values().filter(|c| !c.deleted).count()
    }

    /// Gets total activity count
    pub fn total_activities(&self) -> usize {
        self.activities.len()
    }

    /// Gets comments mentioning a user
    pub fn comments_mentioning_user(&self, user_id: &str) -> Vec<&Comment> {
        self.comments
            .values()
            .filter(|c| c.mentions.iter().any(|m| m == user_id) && !c.deleted)
            .collect()
    }
}

impl Default for CommentManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reaction_emoji() {
        assert_eq!(Reaction::ThumbsUp.emoji(), "👍");
        assert_eq!(Reaction::Heart.emoji(), "❤️");
        assert_eq!(Reaction::Party.emoji(), "🎉");
    }

    #[test]
    fn test_all_reactions() {
        let reactions = Reaction::all();
        assert_eq!(reactions.len(), 6);
        assert!(reactions.contains(&Reaction::ThumbsUp));
        assert!(reactions.contains(&Reaction::Rocket));
    }

    #[test]
    fn test_extract_mentions() {
        let content = "Hey @alice and @bob, please review this!";
        let mentions = Comment::extract_mentions(content);
        assert_eq!(mentions.len(), 2);
        assert!(mentions.contains(&"alice".to_string()));
        assert!(mentions.contains(&"bob".to_string()));
    }

    #[test]
    fn test_extract_mentions_with_punctuation() {
        let content = "Thanks @alice! CC @bob, @charlie.";
        let mentions = Comment::extract_mentions(content);
        assert_eq!(mentions.len(), 3);
        assert!(mentions.contains(&"alice".to_string()));
        assert!(mentions.contains(&"bob".to_string()));
        assert!(mentions.contains(&"charlie".to_string()));
    }

    #[test]
    fn test_create_comment() {
        let mut manager = CommentManager::new();
        let comment_id = manager.create_comment(
            "card-1".to_string(),
            None,
            "Test comment".to_string(),
            "user-1".to_string(),
            "Alice".to_string(),
        );

        assert_eq!(comment_id, "comment-1");
        let comment = manager.get_comment(&comment_id).unwrap();
        assert_eq!(comment.content, "Test comment");
        assert_eq!(comment.author_id, "user-1");
        assert_eq!(comment.parent_id, None);
    }

    #[test]
    fn test_create_threaded_reply() {
        let mut manager = CommentManager::new();
        let parent_id = manager.create_comment(
            "card-1".to_string(),
            None,
            "Parent comment".to_string(),
            "user-1".to_string(),
            "Alice".to_string(),
        );

        let reply_id = manager.create_comment(
            "card-1".to_string(),
            Some(parent_id.clone()),
            "Reply comment".to_string(),
            "user-2".to_string(),
            "Bob".to_string(),
        );

        let reply = manager.get_comment(&reply_id).unwrap();
        assert_eq!(reply.parent_id, Some(parent_id));
    }

    #[test]
    fn test_edit_comment() {
        let mut manager = CommentManager::new();
        let comment_id = manager.create_comment(
            "card-1".to_string(),
            None,
            "Original content".to_string(),
            "user-1".to_string(),
            "Alice".to_string(),
        );

        let comment = manager.get_comment_mut(&comment_id).unwrap();
        comment.edit("Updated content".to_string(), "user-1".to_string());

        assert_eq!(comment.content, "Updated content");
        assert!(comment.is_edited());
        assert_eq!(comment.edit_count(), 1);
        assert_eq!(comment.edit_history[0].previous_content, "Original content");
    }

    #[test]
    fn test_add_reaction() {
        let mut manager = CommentManager::new();
        let comment_id = manager.create_comment(
            "card-1".to_string(),
            None,
            "Test comment".to_string(),
            "user-1".to_string(),
            "Alice".to_string(),
        );

        let comment = manager.get_comment_mut(&comment_id).unwrap();
        comment.add_reaction(
            Reaction::ThumbsUp,
            "user-2".to_string(),
            "Bob".to_string(),
        );
        comment.add_reaction(
            Reaction::Heart,
            "user-3".to_string(),
            "Charlie".to_string(),
        );

        assert_eq!(comment.total_reactions(), 2);
        assert_eq!(comment.reaction_count(&Reaction::ThumbsUp), 1);
        assert_eq!(comment.reaction_count(&Reaction::Heart), 1);
    }

    #[test]
    fn test_remove_reaction() {
        let mut manager = CommentManager::new();
        let comment_id = manager.create_comment(
            "card-1".to_string(),
            None,
            "Test comment".to_string(),
            "user-1".to_string(),
            "Alice".to_string(),
        );

        let comment = manager.get_comment_mut(&comment_id).unwrap();
        comment.add_reaction(
            Reaction::ThumbsUp,
            "user-2".to_string(),
            "Bob".to_string(),
        );

        assert_eq!(comment.reaction_count(&Reaction::ThumbsUp), 1);

        comment.remove_reaction(&Reaction::ThumbsUp, "user-2");
        assert_eq!(comment.reaction_count(&Reaction::ThumbsUp), 0);
    }

    #[test]
    fn test_user_has_reacted() {
        let mut manager = CommentManager::new();
        let comment_id = manager.create_comment(
            "card-1".to_string(),
            None,
            "Test comment".to_string(),
            "user-1".to_string(),
            "Alice".to_string(),
        );

        let comment = manager.get_comment_mut(&comment_id).unwrap();
        comment.add_reaction(
            Reaction::ThumbsUp,
            "user-2".to_string(),
            "Bob".to_string(),
        );

        assert!(comment.user_has_reacted(&Reaction::ThumbsUp, "user-2"));
        assert!(!comment.user_has_reacted(&Reaction::ThumbsUp, "user-3"));
        assert!(!comment.user_has_reacted(&Reaction::Heart, "user-2"));
    }

    #[test]
    fn test_delete_comment() {
        let mut manager = CommentManager::new();
        let comment_id = manager.create_comment(
            "card-1".to_string(),
            None,
            "Test comment".to_string(),
            "user-1".to_string(),
            "Alice".to_string(),
        );

        assert!(manager.delete_comment(&comment_id));

        let comment = manager.get_comment(&comment_id).unwrap();
        assert!(comment.deleted);
    }

    #[test]
    fn test_comments_for_card() {
        let mut manager = CommentManager::new();
        manager.create_comment(
            "card-1".to_string(),
            None,
            "Comment 1".to_string(),
            "user-1".to_string(),
            "Alice".to_string(),
        );
        manager.create_comment(
            "card-1".to_string(),
            None,
            "Comment 2".to_string(),
            "user-2".to_string(),
            "Bob".to_string(),
        );
        manager.create_comment(
            "card-2".to_string(),
            None,
            "Comment 3".to_string(),
            "user-1".to_string(),
            "Alice".to_string(),
        );

        let card1_comments = manager.comments_for_card("card-1");
        assert_eq!(card1_comments.len(), 2);

        let card2_comments = manager.comments_for_card("card-2");
        assert_eq!(card2_comments.len(), 1);
    }

    #[test]
    fn test_top_level_comments() {
        let mut manager = CommentManager::new();
        let parent_id = manager.create_comment(
            "card-1".to_string(),
            None,
            "Parent".to_string(),
            "user-1".to_string(),
            "Alice".to_string(),
        );
        manager.create_comment(
            "card-1".to_string(),
            Some(parent_id),
            "Reply".to_string(),
            "user-2".to_string(),
            "Bob".to_string(),
        );

        let top_level = manager.top_level_comments("card-1");
        assert_eq!(top_level.len(), 1);
    }

    #[test]
    fn test_replies_to_comment() {
        let mut manager = CommentManager::new();
        let parent_id = manager.create_comment(
            "card-1".to_string(),
            None,
            "Parent".to_string(),
            "user-1".to_string(),
            "Alice".to_string(),
        );
        manager.create_comment(
            "card-1".to_string(),
            Some(parent_id.clone()),
            "Reply 1".to_string(),
            "user-2".to_string(),
            "Bob".to_string(),
        );
        manager.create_comment(
            "card-1".to_string(),
            Some(parent_id.clone()),
            "Reply 2".to_string(),
            "user-3".to_string(),
            "Charlie".to_string(),
        );

        let replies = manager.replies_to_comment(&parent_id);
        assert_eq!(replies.len(), 2);
    }

    #[test]
    fn test_get_thread() {
        let mut manager = CommentManager::new();
        let parent_id = manager.create_comment(
            "card-1".to_string(),
            None,
            "Parent".to_string(),
            "user-1".to_string(),
            "Alice".to_string(),
        );
        let reply1_id = manager.create_comment(
            "card-1".to_string(),
            Some(parent_id.clone()),
            "Reply 1".to_string(),
            "user-2".to_string(),
            "Bob".to_string(),
        );
        manager.create_comment(
            "card-1".to_string(),
            Some(reply1_id),
            "Nested reply".to_string(),
            "user-3".to_string(),
            "Charlie".to_string(),
        );

        let thread = manager.get_thread(&parent_id);
        assert_eq!(thread.len(), 3); // Parent + Reply 1 + Nested reply
    }

    #[test]
    fn test_log_activity() {
        let mut manager = CommentManager::new();
        let activity_id = manager.log_activity(
            "card-1".to_string(),
            "card_moved".to_string(),
            "Card moved to In Progress".to_string(),
            Some("user-1".to_string()),
            Some("Alice".to_string()),
        );

        assert_eq!(activity_id, "activity-1");
        assert_eq!(manager.total_activities(), 1);
    }

    #[test]
    fn test_activities_for_card() {
        let mut manager = CommentManager::new();
        manager.log_activity(
            "card-1".to_string(),
            "card_moved".to_string(),
            "Moved".to_string(),
            None,
            None,
        );
        manager.log_activity(
            "card-1".to_string(),
            "assignee_changed".to_string(),
            "Assigned".to_string(),
            None,
            None,
        );
        manager.log_activity(
            "card-2".to_string(),
            "card_created".to_string(),
            "Created".to_string(),
            None,
            None,
        );

        let card1_activities = manager.activities_for_card("card-1");
        assert_eq!(card1_activities.len(), 2);
    }

    #[test]
    fn test_recent_activities() {
        let mut manager = CommentManager::new();
        manager.log_activity(
            "card-1".to_string(),
            "activity1".to_string(),
            "First".to_string(),
            None,
            None,
        );
        manager.log_activity(
            "card-1".to_string(),
            "activity2".to_string(),
            "Second".to_string(),
            None,
            None,
        );
        manager.log_activity(
            "card-1".to_string(),
            "activity3".to_string(),
            "Third".to_string(),
            None,
            None,
        );

        let recent = manager.recent_activities(2);
        assert_eq!(recent.len(), 2);
        // Most recent first
        assert_eq!(recent[0].activity_type, "activity3");
        assert_eq!(recent[1].activity_type, "activity2");
    }

    #[test]
    fn test_activity_max_limit() {
        let mut manager = CommentManager::with_max_activities(5);
        for i in 1..=10 {
            manager.log_activity(
                "card-1".to_string(),
                format!("activity{}", i),
                format!("Activity {}", i),
                None,
                None,
            );
        }

        assert_eq!(manager.total_activities(), 5);
    }

    #[test]
    fn test_comments_mentioning_user() {
        let mut manager = CommentManager::new();
        manager.create_comment(
            "card-1".to_string(),
            None,
            "Hey @alice, check this out!".to_string(),
            "user-1".to_string(),
            "Bob".to_string(),
        );
        manager.create_comment(
            "card-1".to_string(),
            None,
            "This is a comment".to_string(),
            "user-2".to_string(),
            "Charlie".to_string(),
        );
        manager.create_comment(
            "card-2".to_string(),
            None,
            "Thanks @alice!".to_string(),
            "user-3".to_string(),
            "Dave".to_string(),
        );

        let mentions = manager.comments_mentioning_user("alice");
        assert_eq!(mentions.len(), 2);
    }

    #[test]
    fn test_activity_metadata() {
        let mut activity = ActivityLogEntry::new(
            "activity-1".to_string(),
            "card-1".to_string(),
            "card_moved".to_string(),
            "Card moved".to_string(),
            Some("user-1".to_string()),
            Some("Alice".to_string()),
        );

        activity.add_metadata("from_column".to_string(), "Todo".to_string());
        activity.add_metadata("to_column".to_string(), "In Progress".to_string());

        assert_eq!(activity.metadata.len(), 2);
        assert_eq!(activity.metadata.get("from_column").unwrap(), "Todo");
        assert_eq!(activity.metadata.get("to_column").unwrap(), "In Progress");
    }

    #[test]
    fn test_total_comments_excludes_deleted() {
        let mut manager = CommentManager::new();
        let comment1_id = manager.create_comment(
            "card-1".to_string(),
            None,
            "Comment 1".to_string(),
            "user-1".to_string(),
            "Alice".to_string(),
        );
        manager.create_comment(
            "card-1".to_string(),
            None,
            "Comment 2".to_string(),
            "user-2".to_string(),
            "Bob".to_string(),
        );

        assert_eq!(manager.total_comments(), 2);

        manager.delete_comment(&comment1_id);
        assert_eq!(manager.total_comments(), 1);
    }

    #[test]
    fn test_deleted_comments_not_in_thread() {
        let mut manager = CommentManager::new();
        let parent_id = manager.create_comment(
            "card-1".to_string(),
            None,
            "Parent".to_string(),
            "user-1".to_string(),
            "Alice".to_string(),
        );
        let reply_id = manager.create_comment(
            "card-1".to_string(),
            Some(parent_id.clone()),
            "Reply".to_string(),
            "user-2".to_string(),
            "Bob".to_string(),
        );

        let thread_before = manager.get_thread(&parent_id);
        assert_eq!(thread_before.len(), 2);

        manager.delete_comment(&reply_id);

        let thread_after = manager.get_thread(&parent_id);
        assert_eq!(thread_after.len(), 1); // Only parent, reply is deleted
    }
}
