// Team Collaboration System
//
// Provides comprehensive team collaboration features including watchers, board sharing,
// activity feeds, and notifications.
//
// # Features
//
// - **Watchers**: Subscribe to card updates without being assigned
// - **Board Sharing**: Share boards with team members (view/edit permissions)
// - **Real-time Updates**: Live board state sync (if multi-user)
// - **Activity Feed**: Global feed of all board changes
// - **Notifications**: Desktop/in-app alerts for mentions, due dates, assignments

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Permission level for board access
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Permission {
    /// Can view board and tasks
    View,
    /// Can comment on tasks
    Comment,
    /// Can edit tasks
    Edit,
    /// Full admin access
    Admin,
}

impl Permission {
    /// Check if this permission includes another permission
    pub fn includes(&self, other: Permission) -> bool {
        match self {
            Permission::Admin => true,
            Permission::Edit => matches!(
                other,
                Permission::View | Permission::Comment | Permission::Edit
            ),
            Permission::Comment => matches!(other, Permission::View | Permission::Comment),
            Permission::View => matches!(other, Permission::View),
        }
    }

    /// Get permission name
    pub fn name(&self) -> &'static str {
        match self {
            Permission::View => "View",
            Permission::Comment => "Comment",
            Permission::Edit => "Edit",
            Permission::Admin => "Admin",
        }
    }
}

/// Board member
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoardMember {
    /// User ID
    pub user_id: String,
    /// Display name
    pub display_name: String,
    /// Permission level
    pub permission: Permission,
    /// When added to board
    pub added_at: DateTime<Utc>,
    /// Added by (user ID)
    pub added_by: String,
}

impl BoardMember {
    /// Create a new board member
    pub fn new(
        user_id: impl Into<String>,
        display_name: impl Into<String>,
        permission: Permission,
        added_by: impl Into<String>,
    ) -> Self {
        Self {
            user_id: user_id.into(),
            display_name: display_name.into(),
            permission,
            added_at: Utc::now(),
            added_by: added_by.into(),
        }
    }

    /// Check if member has permission
    pub fn has_permission(&self, required: Permission) -> bool {
        self.permission.includes(required)
    }
}

/// Watcher subscription for a task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Watcher {
    /// User ID
    pub user_id: String,
    /// Display name
    pub display_name: String,
    /// When subscription started
    pub subscribed_at: DateTime<Utc>,
    /// Notification preferences
    pub notify_on_update: bool,
    /// Notify on comments
    pub notify_on_comment: bool,
    /// Notify on status change
    pub notify_on_status_change: bool,
}

impl Watcher {
    /// Create a new watcher
    pub fn new(user_id: impl Into<String>, display_name: impl Into<String>) -> Self {
        Self {
            user_id: user_id.into(),
            display_name: display_name.into(),
            subscribed_at: Utc::now(),
            notify_on_update: true,
            notify_on_comment: true,
            notify_on_status_change: true,
        }
    }

    /// Enable all notifications
    pub fn notify_all(&mut self) {
        self.notify_on_update = true;
        self.notify_on_comment = true;
        self.notify_on_status_change = true;
    }

    /// Disable all notifications
    pub fn notify_none(&mut self) {
        self.notify_on_update = false;
        self.notify_on_comment = false;
        self.notify_on_status_change = false;
    }
}

/// Activity type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActivityType {
    /// Task created
    TaskCreated,
    /// Task updated
    TaskUpdated,
    /// Task deleted
    TaskDeleted,
    /// Task moved
    TaskMoved,
    /// Comment added
    CommentAdded,
    /// Comment edited
    CommentEdited,
    /// Comment deleted
    CommentDeleted,
    /// User assigned
    UserAssigned,
    /// User unassigned
    UserUnassigned,
    /// Watcher added
    WatcherAdded,
    /// Watcher removed
    WatcherRemoved,
    /// Board shared
    BoardShared,
    /// Member added
    MemberAdded,
    /// Member removed
    MemberRemoved,
    /// Permission changed
    PermissionChanged,
}

impl ActivityType {
    /// Get activity type name
    pub fn name(&self) -> &'static str {
        match self {
            ActivityType::TaskCreated => "Task Created",
            ActivityType::TaskUpdated => "Task Updated",
            ActivityType::TaskDeleted => "Task Deleted",
            ActivityType::TaskMoved => "Task Moved",
            ActivityType::CommentAdded => "Comment Added",
            ActivityType::CommentEdited => "Comment Edited",
            ActivityType::CommentDeleted => "Comment Deleted",
            ActivityType::UserAssigned => "User Assigned",
            ActivityType::UserUnassigned => "User Unassigned",
            ActivityType::WatcherAdded => "Watcher Added",
            ActivityType::WatcherRemoved => "Watcher Removed",
            ActivityType::BoardShared => "Board Shared",
            ActivityType::MemberAdded => "Member Added",
            ActivityType::MemberRemoved => "Member Removed",
            ActivityType::PermissionChanged => "Permission Changed",
        }
    }

    /// Get emoji for activity type
    pub fn emoji(&self) -> &'static str {
        match self {
            ActivityType::TaskCreated => "✨",
            ActivityType::TaskUpdated => "📝",
            ActivityType::TaskDeleted => "🗑️",
            ActivityType::TaskMoved => "➡️",
            ActivityType::CommentAdded => "💬",
            ActivityType::CommentEdited => "✏️",
            ActivityType::CommentDeleted => "🚫",
            ActivityType::UserAssigned => "👤",
            ActivityType::UserUnassigned => "👋",
            ActivityType::WatcherAdded => "👁️",
            ActivityType::WatcherRemoved => "🙈",
            ActivityType::BoardShared => "🔗",
            ActivityType::MemberAdded => "➕",
            ActivityType::MemberRemoved => "➖",
            ActivityType::PermissionChanged => "🔑",
        }
    }
}

/// Activity entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity {
    /// Activity ID
    pub id: String,
    /// Activity type
    pub activity_type: ActivityType,
    /// User who performed the activity
    pub user_id: String,
    /// User display name
    pub user_name: String,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Task ID (if applicable)
    pub task_id: Option<String>,
    /// Task title (if applicable)
    pub task_title: Option<String>,
    /// Activity description
    pub description: String,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl Activity {
    /// Create a new activity
    pub fn new(
        activity_type: ActivityType,
        user_id: impl Into<String>,
        user_name: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            id: format!("activity-{}", Utc::now().timestamp_millis()),
            activity_type,
            user_id: user_id.into(),
            user_name: user_name.into(),
            timestamp: Utc::now(),
            task_id: None,
            task_title: None,
            description: description.into(),
            metadata: HashMap::new(),
        }
    }

    /// Set task info
    pub fn with_task(mut self, task_id: impl Into<String>, task_title: impl Into<String>) -> Self {
        self.task_id = Some(task_id.into());
        self.task_title = Some(task_title.into());
        self
    }

    /// Add metadata
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}

/// Notification type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NotificationType {
    /// Task assigned to you
    TaskAssigned,
    /// Task mentioned you
    Mentioned,
    /// Task due soon
    DueSoon,
    /// Task overdue
    Overdue,
    /// Watched task updated
    WatchedTaskUpdated,
    /// Comment on your task
    CommentAdded,
    /// Reply to your comment
    CommentReply,
}

impl NotificationType {
    /// Get notification type name
    pub fn name(&self) -> &'static str {
        match self {
            NotificationType::TaskAssigned => "Task Assigned",
            NotificationType::Mentioned => "Mentioned",
            NotificationType::DueSoon => "Due Soon",
            NotificationType::Overdue => "Overdue",
            NotificationType::WatchedTaskUpdated => "Watched Task Updated",
            NotificationType::CommentAdded => "Comment Added",
            NotificationType::CommentReply => "Comment Reply",
        }
    }

    /// Get notification priority
    pub fn priority(&self) -> u8 {
        match self {
            NotificationType::Overdue => 5,
            NotificationType::TaskAssigned => 4,
            NotificationType::Mentioned => 4,
            NotificationType::DueSoon => 3,
            NotificationType::CommentReply => 2,
            NotificationType::CommentAdded => 1,
            NotificationType::WatchedTaskUpdated => 1,
        }
    }
}

/// Notification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    /// Notification ID
    pub id: String,
    /// Recipient user ID
    pub user_id: String,
    /// Notification type
    pub notification_type: NotificationType,
    /// Notification title
    pub title: String,
    /// Notification message
    pub message: String,
    /// Task ID (if applicable)
    pub task_id: Option<String>,
    /// Read status
    pub read: bool,
    /// Timestamp
    pub created_at: DateTime<Utc>,
    /// Read at (if read)
    pub read_at: Option<DateTime<Utc>>,
}

impl Notification {
    /// Create a new notification
    pub fn new(
        user_id: impl Into<String>,
        notification_type: NotificationType,
        title: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            id: format!("notif-{}", Utc::now().timestamp_millis()),
            user_id: user_id.into(),
            notification_type,
            title: title.into(),
            message: message.into(),
            task_id: None,
            read: false,
            created_at: Utc::now(),
            read_at: None,
        }
    }

    /// Set task ID
    pub fn with_task_id(mut self, task_id: impl Into<String>) -> Self {
        self.task_id = Some(task_id.into());
        self
    }

    /// Mark as read
    pub fn mark_read(&mut self) {
        if !self.read {
            self.read = true;
            self.read_at = Some(Utc::now());
        }
    }

    /// Mark as unread
    pub fn mark_unread(&mut self) {
        self.read = false;
        self.read_at = None;
    }

    /// Check if unread
    pub fn is_unread(&self) -> bool {
        !self.read
    }
}

/// Collaboration manager
#[derive(Debug)]
pub struct CollaborationManager {
    /// Board members by board ID
    board_members: HashMap<String, Vec<BoardMember>>,
    /// Watchers by task ID
    task_watchers: HashMap<String, Vec<Watcher>>,
    /// Activity feed
    activities: Vec<Activity>,
    /// Notifications by user ID
    notifications: HashMap<String, Vec<Notification>>,
    /// Maximum activity history
    max_activities: usize,
    /// Maximum notifications per user
    max_notifications_per_user: usize,
    /// Next activity ID counter
    next_activity_id: usize,
    /// Next notification ID counter
    next_notification_id: usize,
}

impl CollaborationManager {
    /// Create a new collaboration manager
    pub fn new() -> Self {
        Self {
            board_members: HashMap::new(),
            task_watchers: HashMap::new(),
            activities: Vec::new(),
            notifications: HashMap::new(),
            max_activities: 1000,
            max_notifications_per_user: 100,
            next_activity_id: 1,
            next_notification_id: 1,
        }
    }

    /// Add a board member
    pub fn add_board_member(&mut self, board_id: impl Into<String>, member: BoardMember) {
        self.board_members
            .entry(board_id.into())
            .or_default()
            .push(member);
    }

    /// Remove a board member
    pub fn remove_board_member(&mut self, board_id: &str, user_id: &str) -> bool {
        if let Some(members) = self.board_members.get_mut(board_id)
            && let Some(pos) = members.iter().position(|m| m.user_id == user_id)
        {
            members.remove(pos);
            return true;
        }
        false
    }

    /// Get board members
    pub fn get_board_members(&self, board_id: &str) -> Vec<&BoardMember> {
        self.board_members
            .get(board_id)
            .map(|m| m.iter().collect())
            .unwrap_or_default()
    }

    /// Check if user has permission on board
    pub fn has_permission(&self, board_id: &str, user_id: &str, required: Permission) -> bool {
        self.board_members
            .get(board_id)
            .and_then(|members| members.iter().find(|m| m.user_id == user_id))
            .map(|member| member.has_permission(required))
            .unwrap_or(false)
    }

    /// Add a watcher to a task
    pub fn add_watcher(&mut self, task_id: impl Into<String>, watcher: Watcher) {
        self.task_watchers
            .entry(task_id.into())
            .or_default()
            .push(watcher);
    }

    /// Remove a watcher from a task
    pub fn remove_watcher(&mut self, task_id: &str, user_id: &str) -> bool {
        if let Some(watchers) = self.task_watchers.get_mut(task_id)
            && let Some(pos) = watchers.iter().position(|w| w.user_id == user_id)
        {
            watchers.remove(pos);
            return true;
        }
        false
    }

    /// Get watchers for a task
    pub fn get_watchers(&self, task_id: &str) -> Vec<&Watcher> {
        self.task_watchers
            .get(task_id)
            .map(|w| w.iter().collect())
            .unwrap_or_default()
    }

    /// Check if user is watching task
    pub fn is_watching(&self, task_id: &str, user_id: &str) -> bool {
        self.task_watchers
            .get(task_id)
            .map(|watchers| watchers.iter().any(|w| w.user_id == user_id))
            .unwrap_or(false)
    }

    /// Record an activity
    pub fn record_activity(&mut self, mut activity: Activity) {
        // Override ID with incremental counter
        activity.id = format!("activity-{}", self.next_activity_id);
        self.next_activity_id += 1;

        self.activities.push(activity);

        // Trim old activities
        if self.activities.len() > self.max_activities {
            self.activities.remove(0);
        }
    }

    /// Get recent activities
    pub fn get_activities(&self, limit: usize) -> Vec<&Activity> {
        let start = if self.activities.len() > limit {
            self.activities.len() - limit
        } else {
            0
        };
        self.activities[start..].iter().rev().collect()
    }

    /// Get activities for a task
    pub fn get_task_activities(&self, task_id: &str) -> Vec<&Activity> {
        self.activities
            .iter()
            .filter(|a| a.task_id.as_deref() == Some(task_id))
            .rev()
            .collect()
    }

    /// Create a notification
    pub fn create_notification(&mut self, mut notification: Notification) -> String {
        // Override ID with incremental counter
        notification.id = format!("notif-{}", self.next_notification_id);
        self.next_notification_id += 1;

        let user_id = notification.user_id.clone();
        let id = notification.id.clone();
        let notifications = self.notifications.entry(user_id).or_default();
        notifications.push(notification);

        // Trim old notifications
        if notifications.len() > self.max_notifications_per_user {
            notifications.remove(0);
        }

        id
    }

    /// Get notifications for a user
    pub fn get_notifications(&self, user_id: &str) -> Vec<&Notification> {
        self.notifications
            .get(user_id)
            .map(|n| n.iter().rev().collect())
            .unwrap_or_default()
    }

    /// Get unread notifications for a user
    pub fn get_unread_notifications(&self, user_id: &str) -> Vec<&Notification> {
        self.notifications
            .get(user_id)
            .map(|n| n.iter().filter(|notif| notif.is_unread()).rev().collect())
            .unwrap_or_default()
    }

    /// Mark notification as read
    pub fn mark_notification_read(&mut self, user_id: &str, notification_id: &str) -> bool {
        if let Some(notifications) = self.notifications.get_mut(user_id)
            && let Some(notif) = notifications.iter_mut().find(|n| n.id == notification_id)
        {
            notif.mark_read();
            return true;
        }
        false
    }

    /// Mark all notifications as read for a user
    pub fn mark_all_read(&mut self, user_id: &str) -> usize {
        let mut count = 0;
        if let Some(notifications) = self.notifications.get_mut(user_id) {
            for notif in notifications.iter_mut() {
                if notif.is_unread() {
                    notif.mark_read();
                    count += 1;
                }
            }
        }
        count
    }

    /// Get unread count for a user
    pub fn unread_count(&self, user_id: &str) -> usize {
        self.notifications
            .get(user_id)
            .map(|n| n.iter().filter(|notif| notif.is_unread()).count())
            .unwrap_or(0)
    }

    /// Clear all notifications for a user
    pub fn clear_notifications(&mut self, user_id: &str) {
        self.notifications.remove(user_id);
    }
}

impl Default for CollaborationManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permission_includes() {
        assert!(Permission::Admin.includes(Permission::View));
        assert!(Permission::Admin.includes(Permission::Edit));
        assert!(Permission::Edit.includes(Permission::View));
        assert!(Permission::Edit.includes(Permission::Comment));
        assert!(!Permission::View.includes(Permission::Edit));
    }

    #[test]
    fn test_board_member_creation() {
        let member = BoardMember::new("alice", "Alice", Permission::Edit, "admin");
        assert_eq!(member.user_id, "alice");
        assert_eq!(member.permission, Permission::Edit);
        assert!(member.has_permission(Permission::View));
        assert!(member.has_permission(Permission::Edit));
        assert!(!member.has_permission(Permission::Admin));
    }

    #[test]
    fn test_watcher_creation() {
        let watcher = Watcher::new("bob", "Bob");
        assert_eq!(watcher.user_id, "bob");
        assert!(watcher.notify_on_update);
        assert!(watcher.notify_on_comment);
    }

    #[test]
    fn test_watcher_notifications() {
        let mut watcher = Watcher::new("bob", "Bob");
        watcher.notify_none();
        assert!(!watcher.notify_on_update);
        assert!(!watcher.notify_on_comment);

        watcher.notify_all();
        assert!(watcher.notify_on_update);
        assert!(watcher.notify_on_comment);
    }

    #[test]
    fn test_activity_creation() {
        let activity = Activity::new(ActivityType::TaskCreated, "alice", "Alice", "Created task");
        assert_eq!(activity.activity_type, ActivityType::TaskCreated);
        assert_eq!(activity.user_id, "alice");
        assert!(activity.task_id.is_none());
    }

    #[test]
    fn test_activity_with_task() {
        let activity = Activity::new(ActivityType::TaskUpdated, "bob", "Bob", "Updated task")
            .with_task("task-1", "Test Task");
        assert_eq!(activity.task_id, Some("task-1".to_string()));
        assert_eq!(activity.task_title, Some("Test Task".to_string()));
    }

    #[test]
    fn test_notification_creation() {
        let notification = Notification::new(
            "alice",
            NotificationType::TaskAssigned,
            "New Task",
            "You were assigned",
        );
        assert_eq!(notification.user_id, "alice");
        assert!(notification.is_unread());
        assert!(notification.read_at.is_none());
    }

    #[test]
    fn test_notification_mark_read() {
        let mut notification =
            Notification::new("alice", NotificationType::TaskAssigned, "Test", "Message");
        assert!(notification.is_unread());

        notification.mark_read();
        assert!(!notification.is_unread());
        assert!(notification.read_at.is_some());
    }

    #[test]
    fn test_notification_priority() {
        assert_eq!(NotificationType::Overdue.priority(), 5);
        assert_eq!(NotificationType::TaskAssigned.priority(), 4);
        assert_eq!(NotificationType::WatchedTaskUpdated.priority(), 1);
    }

    #[test]
    fn test_collaboration_manager_creation() {
        let manager = CollaborationManager::new();
        assert_eq!(manager.max_activities, 1000);
        assert_eq!(manager.max_notifications_per_user, 100);
    }

    #[test]
    fn test_add_board_member() {
        let mut manager = CollaborationManager::new();
        let member = BoardMember::new("alice", "Alice", Permission::Edit, "admin");

        manager.add_board_member("board-1", member);
        assert_eq!(manager.get_board_members("board-1").len(), 1);
    }

    #[test]
    fn test_remove_board_member() {
        let mut manager = CollaborationManager::new();
        manager.add_board_member(
            "board-1",
            BoardMember::new("alice", "Alice", Permission::Edit, "admin"),
        );

        assert!(manager.remove_board_member("board-1", "alice"));
        assert_eq!(manager.get_board_members("board-1").len(), 0);
    }

    #[test]
    fn test_has_permission() {
        let mut manager = CollaborationManager::new();
        manager.add_board_member(
            "board-1",
            BoardMember::new("alice", "Alice", Permission::Edit, "admin"),
        );

        assert!(manager.has_permission("board-1", "alice", Permission::View));
        assert!(manager.has_permission("board-1", "alice", Permission::Edit));
        assert!(!manager.has_permission("board-1", "alice", Permission::Admin));
    }

    #[test]
    fn test_add_watcher() {
        let mut manager = CollaborationManager::new();
        let watcher = Watcher::new("bob", "Bob");

        manager.add_watcher("task-1", watcher);
        assert_eq!(manager.get_watchers("task-1").len(), 1);
        assert!(manager.is_watching("task-1", "bob"));
    }

    #[test]
    fn test_remove_watcher() {
        let mut manager = CollaborationManager::new();
        manager.add_watcher("task-1", Watcher::new("bob", "Bob"));

        assert!(manager.remove_watcher("task-1", "bob"));
        assert!(!manager.is_watching("task-1", "bob"));
    }

    #[test]
    fn test_record_activity() {
        let mut manager = CollaborationManager::new();
        let activity = Activity::new(ActivityType::TaskCreated, "alice", "Alice", "Created task");

        manager.record_activity(activity);
        assert_eq!(manager.get_activities(10).len(), 1);
    }

    #[test]
    fn test_get_task_activities() {
        let mut manager = CollaborationManager::new();

        manager.record_activity(
            Activity::new(ActivityType::TaskCreated, "alice", "Alice", "Created")
                .with_task("task-1", "Task 1"),
        );
        manager.record_activity(
            Activity::new(ActivityType::TaskUpdated, "bob", "Bob", "Updated")
                .with_task("task-1", "Task 1"),
        );
        manager.record_activity(
            Activity::new(ActivityType::TaskCreated, "charlie", "Charlie", "Created")
                .with_task("task-2", "Task 2"),
        );

        assert_eq!(manager.get_task_activities("task-1").len(), 2);
    }

    #[test]
    fn test_create_notification() {
        let mut manager = CollaborationManager::new();
        let notification =
            Notification::new("alice", NotificationType::TaskAssigned, "Test", "Message");

        manager.create_notification(notification);
        assert_eq!(manager.get_notifications("alice").len(), 1);
        assert_eq!(manager.unread_count("alice"), 1);
    }

    #[test]
    fn test_mark_notification_read() {
        let mut manager = CollaborationManager::new();
        let notification =
            Notification::new("alice", NotificationType::TaskAssigned, "Test", "Message");

        let notif_id = manager.create_notification(notification);
        assert_eq!(manager.unread_count("alice"), 1);

        manager.mark_notification_read("alice", &notif_id);
        assert_eq!(manager.unread_count("alice"), 0);
    }

    #[test]
    fn test_mark_all_read() {
        let mut manager = CollaborationManager::new();

        manager.create_notification(Notification::new(
            "alice",
            NotificationType::TaskAssigned,
            "Test 1",
            "Message",
        ));
        manager.create_notification(Notification::new(
            "alice",
            NotificationType::Mentioned,
            "Test 2",
            "Message",
        ));

        assert_eq!(manager.unread_count("alice"), 2);
        let marked = manager.mark_all_read("alice");
        assert_eq!(marked, 2);
        assert_eq!(manager.unread_count("alice"), 0);
    }

    #[test]
    fn test_clear_notifications() {
        let mut manager = CollaborationManager::new();
        manager.create_notification(Notification::new(
            "alice",
            NotificationType::TaskAssigned,
            "Test",
            "Message",
        ));

        assert_eq!(manager.get_notifications("alice").len(), 1);
        manager.clear_notifications("alice");
        assert_eq!(manager.get_notifications("alice").len(), 0);
    }
}
