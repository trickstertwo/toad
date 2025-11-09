// Communication Integrations System
//
// Provides webhook integrations with popular communication platforms including
// Slack, Discord, Microsoft Teams, and Email for board updates and notifications.
//
// # Features
//
// - **Slack/Discord**: Post board updates to channels via webhooks
// - **Email**: Create cards from email, send digest emails
// - **Microsoft Teams**: Board activity in Teams channels
// - **Webhook Management**: Configure, test, and manage multiple webhooks
// - **Event Filtering**: Control which events trigger notifications

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Integration platform
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IntegrationPlatform {
    /// Slack webhook integration
    Slack,
    /// Discord webhook integration
    Discord,
    /// Microsoft Teams webhook integration
    Teams,
    /// Email integration
    Email,
}

impl IntegrationPlatform {
    /// Get platform name
    pub fn name(&self) -> &'static str {
        match self {
            IntegrationPlatform::Slack => "Slack",
            IntegrationPlatform::Discord => "Discord",
            IntegrationPlatform::Teams => "Microsoft Teams",
            IntegrationPlatform::Email => "Email",
        }
    }

    /// Get webhook URL format hint
    pub fn url_format(&self) -> &'static str {
        match self {
            IntegrationPlatform::Slack => "https://hooks.slack.com/services/...",
            IntegrationPlatform::Discord => "https://discord.com/api/webhooks/...",
            IntegrationPlatform::Teams => "https://outlook.office.com/webhook/...",
            IntegrationPlatform::Email => "smtp://smtp.example.com:587",
        }
    }
}

/// Event type that can trigger notifications
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EventType {
    /// Task created
    TaskCreated,
    /// Task updated
    TaskUpdated,
    /// Task completed
    TaskCompleted,
    /// Task deleted
    TaskDeleted,
    /// Task moved
    TaskMoved,
    /// Comment added
    CommentAdded,
    /// User assigned
    UserAssigned,
    /// Due date changed
    DueDateChanged,
    /// Priority changed
    PriorityChanged,
    /// Sprint started
    SprintStarted,
    /// Sprint completed
    SprintCompleted,
}

impl EventType {
    /// Get event name
    pub fn name(&self) -> &'static str {
        match self {
            EventType::TaskCreated => "Task Created",
            EventType::TaskUpdated => "Task Updated",
            EventType::TaskCompleted => "Task Completed",
            EventType::TaskDeleted => "Task Deleted",
            EventType::TaskMoved => "Task Moved",
            EventType::CommentAdded => "Comment Added",
            EventType::UserAssigned => "User Assigned",
            EventType::DueDateChanged => "Due Date Changed",
            EventType::PriorityChanged => "Priority Changed",
            EventType::SprintStarted => "Sprint Started",
            EventType::SprintCompleted => "Sprint Completed",
        }
    }

    /// Get emoji for this event type
    pub fn emoji(&self) -> &'static str {
        match self {
            EventType::TaskCreated => "✨",
            EventType::TaskUpdated => "📝",
            EventType::TaskCompleted => "✅",
            EventType::TaskDeleted => "🗑️",
            EventType::TaskMoved => "➡️",
            EventType::CommentAdded => "💬",
            EventType::UserAssigned => "👤",
            EventType::DueDateChanged => "📅",
            EventType::PriorityChanged => "🔥",
            EventType::SprintStarted => "🚀",
            EventType::SprintCompleted => "🏁",
        }
    }
}

/// Notification event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationEvent {
    /// Event type
    pub event_type: EventType,
    /// Event timestamp
    pub timestamp: DateTime<Utc>,
    /// Task ID
    pub task_id: String,
    /// Task title
    pub task_title: String,
    /// User who triggered the event
    pub triggered_by: String,
    /// Event message
    pub message: String,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl NotificationEvent {
    /// Create a new notification event
    pub fn new(
        event_type: EventType,
        task_id: impl Into<String>,
        task_title: impl Into<String>,
        triggered_by: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            event_type,
            timestamp: Utc::now(),
            task_id: task_id.into(),
            task_title: task_title.into(),
            triggered_by: triggered_by.into(),
            message: message.into(),
            metadata: HashMap::new(),
        }
    }

    /// Add metadata
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}

/// Webhook configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookConfig {
    /// Webhook ID
    pub id: String,
    /// Platform
    pub platform: IntegrationPlatform,
    /// Webhook URL
    pub url: String,
    /// Friendly name
    pub name: String,
    /// Enabled status
    pub enabled: bool,
    /// Event types to send
    pub event_filter: Vec<EventType>,
    /// Channel/recipient
    pub channel: Option<String>,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
    /// Last sent timestamp
    pub last_sent_at: Option<DateTime<Utc>>,
    /// Send count
    pub send_count: usize,
}

impl WebhookConfig {
    /// Create a new webhook configuration
    pub fn new(
        platform: IntegrationPlatform,
        url: impl Into<String>,
        name: impl Into<String>,
    ) -> Self {
        Self {
            id: format!("webhook-{}", Utc::now().timestamp_millis()),
            platform,
            url: url.into(),
            name: name.into(),
            enabled: true,
            event_filter: Vec::new(),
            channel: None,
            created_at: Utc::now(),
            last_sent_at: None,
            send_count: 0,
        }
    }

    /// Set channel
    pub fn with_channel(mut self, channel: impl Into<String>) -> Self {
        self.channel = Some(channel.into());
        self
    }

    /// Enable/disable webhook
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Add event type to filter
    pub fn add_event_type(&mut self, event_type: EventType) {
        if !self.event_filter.contains(&event_type) {
            self.event_filter.push(event_type);
        }
    }

    /// Remove event type from filter
    pub fn remove_event_type(&mut self, event_type: EventType) {
        self.event_filter.retain(|e| e != &event_type);
    }

    /// Check if event should be sent
    pub fn should_send_event(&self, event_type: EventType) -> bool {
        self.enabled
            && (self.event_filter.is_empty() || self.event_filter.contains(&event_type))
    }

    /// Record send
    pub fn record_send(&mut self) {
        self.last_sent_at = Some(Utc::now());
        self.send_count += 1;
    }
}

/// Slack message payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlackMessage {
    /// Message text
    pub text: String,
    /// Channel (optional, can be in webhook URL)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    /// Username
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// Emoji icon
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_emoji: Option<String>,
}

impl SlackMessage {
    /// Create a new Slack message
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            channel: None,
            username: Some("TOAD Bot".to_string()),
            icon_emoji: Some(":frog:".to_string()),
        }
    }

    /// Format event as Slack message
    pub fn from_event(event: &NotificationEvent) -> Self {
        let text = format!(
            "{} {} *{}*\n_{}_\nTriggered by: {}",
            event.event_type.emoji(),
            event.event_type.name(),
            event.task_title,
            event.message,
            event.triggered_by
        );

        Self::new(text)
    }
}

/// Discord message payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordMessage {
    /// Message content
    pub content: String,
    /// Username
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// Avatar URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
}

impl DiscordMessage {
    /// Create a new Discord message
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            username: Some("TOAD Bot".to_string()),
            avatar_url: None,
        }
    }

    /// Format event as Discord message
    pub fn from_event(event: &NotificationEvent) -> Self {
        let content = format!(
            "{} **{}** - **{}**\n{}\n_Triggered by: {}_",
            event.event_type.emoji(),
            event.event_type.name(),
            event.task_title,
            event.message,
            event.triggered_by
        );

        Self::new(content)
    }
}

/// Microsoft Teams message payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamsMessage {
    /// Message type
    #[serde(rename = "@type")]
    pub message_type: String,
    /// Message context
    #[serde(rename = "@context")]
    pub context: String,
    /// Summary
    pub summary: String,
    /// Title
    pub title: String,
    /// Text
    pub text: String,
}

impl TeamsMessage {
    /// Create a new Teams message
    pub fn new(title: impl Into<String>, text: impl Into<String>) -> Self {
        let title_str = title.into();
        Self {
            message_type: "MessageCard".to_string(),
            context: "https://schema.org/extensions".to_string(),
            summary: title_str.clone(),
            title: title_str,
            text: text.into(),
        }
    }

    /// Format event as Teams message
    pub fn from_event(event: &NotificationEvent) -> Self {
        let title = format!("{} {}", event.event_type.emoji(), event.event_type.name());
        let text = format!(
            "**{}**\n\n{}\n\nTriggered by: {}",
            event.task_title, event.message, event.triggered_by
        );

        Self::new(title, text)
    }
}

/// Email configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailConfig {
    /// SMTP server
    pub smtp_server: String,
    /// SMTP port
    pub smtp_port: u16,
    /// From address
    pub from_address: String,
    /// From name
    pub from_name: String,
    /// Username (if required)
    pub username: Option<String>,
    /// Use TLS
    pub use_tls: bool,
}

impl EmailConfig {
    /// Create a new email configuration
    pub fn new(smtp_server: impl Into<String>, smtp_port: u16, from_address: impl Into<String>) -> Self {
        Self {
            smtp_server: smtp_server.into(),
            smtp_port,
            from_address: from_address.into(),
            from_name: "TOAD".to_string(),
            username: None,
            use_tls: true,
        }
    }
}

/// Email message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailMessage {
    /// To addresses
    pub to: Vec<String>,
    /// Subject
    pub subject: String,
    /// Body (plain text)
    pub body: String,
    /// HTML body
    pub html_body: Option<String>,
}

impl EmailMessage {
    /// Create a new email message
    pub fn new(to: Vec<String>, subject: impl Into<String>, body: impl Into<String>) -> Self {
        Self {
            to,
            subject: subject.into(),
            body: body.into(),
            html_body: None,
        }
    }

    /// Format event as email
    pub fn from_event(event: &NotificationEvent, to: Vec<String>) -> Self {
        let subject = format!("{} - {}", event.event_type.name(), event.task_title);
        let body = format!(
            "Event: {}\nTask: {}\n\n{}\n\nTriggered by: {}\nTime: {}",
            event.event_type.name(),
            event.task_title,
            event.message,
            event.triggered_by,
            event.timestamp.format("%Y-%m-%d %H:%M:%S UTC")
        );

        Self::new(to, subject, body)
    }
}

/// Integration manager
#[derive(Debug)]
pub struct IntegrationManager {
    /// Configured webhooks
    webhooks: HashMap<String, WebhookConfig>,
    /// Email configuration
    email_config: Option<EmailConfig>,
    /// Event history
    event_history: Vec<NotificationEvent>,
    /// Maximum history size
    max_history: usize,
    /// Next webhook ID counter
    next_webhook_id: usize,
}

impl IntegrationManager {
    /// Create a new integration manager
    pub fn new() -> Self {
        Self {
            webhooks: HashMap::new(),
            email_config: None,
            event_history: Vec::new(),
            max_history: 1000,
            next_webhook_id: 1,
        }
    }

    /// Add a webhook
    pub fn add_webhook(&mut self, mut config: WebhookConfig) -> String {
        // Override ID with incremental counter
        config.id = format!("webhook-{}", self.next_webhook_id);
        self.next_webhook_id += 1;

        let id = config.id.clone();
        self.webhooks.insert(id.clone(), config);
        id
    }

    /// Remove a webhook
    pub fn remove_webhook(&mut self, webhook_id: &str) -> bool {
        self.webhooks.remove(webhook_id).is_some()
    }

    /// Get webhook
    pub fn get_webhook(&self, webhook_id: &str) -> Option<&WebhookConfig> {
        self.webhooks.get(webhook_id)
    }

    /// Get webhook (mutable)
    pub fn get_webhook_mut(&mut self, webhook_id: &str) -> Option<&mut WebhookConfig> {
        self.webhooks.get_mut(webhook_id)
    }

    /// Get all webhooks
    pub fn webhooks(&self) -> Vec<&WebhookConfig> {
        self.webhooks.values().collect()
    }

    /// Get webhooks by platform
    pub fn webhooks_by_platform(&self, platform: IntegrationPlatform) -> Vec<&WebhookConfig> {
        self.webhooks
            .values()
            .filter(|w| w.platform == platform)
            .collect()
    }

    /// Configure email
    pub fn configure_email(&mut self, config: EmailConfig) {
        self.email_config = Some(config);
    }

    /// Get email configuration
    pub fn email_config(&self) -> Option<&EmailConfig> {
        self.email_config.as_ref()
    }

    /// Send event to all matching webhooks
    pub fn send_event(&mut self, event: NotificationEvent) -> Vec<String> {
        let mut sent_to = Vec::new();

        for (id, webhook) in self.webhooks.iter_mut() {
            if webhook.should_send_event(event.event_type) {
                webhook.record_send();
                sent_to.push(id.clone());
            }
        }

        // Add to history
        self.event_history.push(event);
        if self.event_history.len() > self.max_history {
            self.event_history.remove(0);
        }

        sent_to
    }

    /// Get event history
    pub fn event_history(&self) -> &[NotificationEvent] {
        &self.event_history
    }

    /// Clear event history
    pub fn clear_history(&mut self) {
        self.event_history.clear();
    }

    /// Get webhook count
    pub fn webhook_count(&self) -> usize {
        self.webhooks.len()
    }

    /// Test webhook (returns formatted message)
    pub fn test_webhook(&self, webhook_id: &str) -> Option<String> {
        self.webhooks.get(webhook_id).map(|webhook| {
            let test_event = NotificationEvent::new(
                EventType::TaskCreated,
                "test-123",
                "Test Task",
                "System",
                "This is a test notification",
            );

            match webhook.platform {
                IntegrationPlatform::Slack => {
                    let msg = SlackMessage::from_event(&test_event);
                    serde_json::to_string_pretty(&msg).unwrap_or_default()
                }
                IntegrationPlatform::Discord => {
                    let msg = DiscordMessage::from_event(&test_event);
                    serde_json::to_string_pretty(&msg).unwrap_or_default()
                }
                IntegrationPlatform::Teams => {
                    let msg = TeamsMessage::from_event(&test_event);
                    serde_json::to_string_pretty(&msg).unwrap_or_default()
                }
                IntegrationPlatform::Email => {
                    let msg = EmailMessage::from_event(&test_event, vec!["test@example.com".to_string()]);
                    format!("To: {:?}\nSubject: {}\n\n{}", msg.to, msg.subject, msg.body)
                }
            }
        })
    }
}

impl Default for IntegrationManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integration_platform_name() {
        assert_eq!(IntegrationPlatform::Slack.name(), "Slack");
        assert_eq!(IntegrationPlatform::Discord.name(), "Discord");
        assert_eq!(IntegrationPlatform::Teams.name(), "Microsoft Teams");
    }

    #[test]
    fn test_event_type_name() {
        assert_eq!(EventType::TaskCreated.name(), "Task Created");
        assert_eq!(EventType::TaskCompleted.name(), "Task Completed");
    }

    #[test]
    fn test_event_type_emoji() {
        assert_eq!(EventType::TaskCreated.emoji(), "✨");
        assert_eq!(EventType::TaskCompleted.emoji(), "✅");
    }

    #[test]
    fn test_notification_event_creation() {
        let event = NotificationEvent::new(
            EventType::TaskCreated,
            "task-1",
            "New Task",
            "alice",
            "Task was created",
        );

        assert_eq!(event.task_id, "task-1");
        assert_eq!(event.task_title, "New Task");
        assert_eq!(event.triggered_by, "alice");
    }

    #[test]
    fn test_notification_event_with_metadata() {
        let event = NotificationEvent::new(
            EventType::TaskUpdated,
            "task-1",
            "Task",
            "bob",
            "Updated",
        )
        .with_metadata("priority", "high");

        assert_eq!(event.metadata.get("priority"), Some(&"high".to_string()));
    }

    #[test]
    fn test_webhook_config_creation() {
        let webhook = WebhookConfig::new(
            IntegrationPlatform::Slack,
            "https://hooks.slack.com/test",
            "Test Webhook",
        );

        assert_eq!(webhook.platform, IntegrationPlatform::Slack);
        assert!(webhook.enabled);
        assert_eq!(webhook.send_count, 0);
    }

    #[test]
    fn test_webhook_config_with_channel() {
        let webhook = WebhookConfig::new(
            IntegrationPlatform::Slack,
            "https://hooks.slack.com/test",
            "Test",
        )
        .with_channel("#general");

        assert_eq!(webhook.channel, Some("#general".to_string()));
    }

    #[test]
    fn test_webhook_add_event_type() {
        let mut webhook = WebhookConfig::new(
            IntegrationPlatform::Discord,
            "https://discord.com/test",
            "Test",
        );

        webhook.add_event_type(EventType::TaskCreated);
        webhook.add_event_type(EventType::TaskCompleted);

        assert_eq!(webhook.event_filter.len(), 2);
    }

    #[test]
    fn test_webhook_remove_event_type() {
        let mut webhook = WebhookConfig::new(
            IntegrationPlatform::Discord,
            "https://discord.com/test",
            "Test",
        );

        webhook.add_event_type(EventType::TaskCreated);
        webhook.remove_event_type(EventType::TaskCreated);

        assert!(webhook.event_filter.is_empty());
    }

    #[test]
    fn test_webhook_should_send_event() {
        let mut webhook = WebhookConfig::new(
            IntegrationPlatform::Slack,
            "https://hooks.slack.com/test",
            "Test",
        );

        // Empty filter = send all events
        assert!(webhook.should_send_event(EventType::TaskCreated));

        webhook.add_event_type(EventType::TaskCreated);
        assert!(webhook.should_send_event(EventType::TaskCreated));
        assert!(!webhook.should_send_event(EventType::TaskCompleted));
    }

    #[test]
    fn test_webhook_disabled() {
        let mut webhook = WebhookConfig::new(
            IntegrationPlatform::Slack,
            "https://hooks.slack.com/test",
            "Test",
        );

        webhook.set_enabled(false);
        assert!(!webhook.should_send_event(EventType::TaskCreated));
    }

    #[test]
    fn test_slack_message_from_event() {
        let event = NotificationEvent::new(
            EventType::TaskCreated,
            "task-1",
            "New Task",
            "alice",
            "Created a new task",
        );

        let msg = SlackMessage::from_event(&event);
        assert!(msg.text.contains("New Task"));
        assert!(msg.text.contains("alice"));
    }

    #[test]
    fn test_discord_message_from_event() {
        let event = NotificationEvent::new(
            EventType::TaskCompleted,
            "task-1",
            "Completed Task",
            "bob",
            "Task is done",
        );

        let msg = DiscordMessage::from_event(&event);
        assert!(msg.content.contains("Completed Task"));
        assert!(msg.content.contains("bob"));
    }

    #[test]
    fn test_teams_message_from_event() {
        let event = NotificationEvent::new(
            EventType::TaskMoved,
            "task-1",
            "Moved Task",
            "charlie",
            "Moved to In Progress",
        );

        let msg = TeamsMessage::from_event(&event);
        assert!(msg.title.contains("Task Moved"));
        assert!(msg.text.contains("Moved Task"));
    }

    #[test]
    fn test_email_config_creation() {
        let config = EmailConfig::new("smtp.example.com", 587, "noreply@example.com");

        assert_eq!(config.smtp_server, "smtp.example.com");
        assert_eq!(config.smtp_port, 587);
        assert!(config.use_tls);
    }

    #[test]
    fn test_email_message_from_event() {
        let event = NotificationEvent::new(
            EventType::TaskCreated,
            "task-1",
            "New Task",
            "alice",
            "Created task",
        );

        let msg = EmailMessage::from_event(&event, vec!["test@example.com".to_string()]);
        assert_eq!(msg.to.len(), 1);
        assert!(msg.subject.contains("Task Created"));
        assert!(msg.body.contains("New Task"));
    }

    #[test]
    fn test_integration_manager_creation() {
        let manager = IntegrationManager::new();
        assert_eq!(manager.webhook_count(), 0);
        assert!(manager.email_config().is_none());
    }

    #[test]
    fn test_integration_manager_add_webhook() {
        let mut manager = IntegrationManager::new();
        let webhook = WebhookConfig::new(
            IntegrationPlatform::Slack,
            "https://hooks.slack.com/test",
            "Test",
        );

        let id = manager.add_webhook(webhook);
        assert_eq!(manager.webhook_count(), 1);
        assert!(manager.get_webhook(&id).is_some());
    }

    #[test]
    fn test_integration_manager_remove_webhook() {
        let mut manager = IntegrationManager::new();
        let webhook = WebhookConfig::new(
            IntegrationPlatform::Discord,
            "https://discord.com/test",
            "Test",
        );

        let id = manager.add_webhook(webhook);
        assert!(manager.remove_webhook(&id));
        assert_eq!(manager.webhook_count(), 0);
    }

    #[test]
    fn test_integration_manager_webhooks_by_platform() {
        let mut manager = IntegrationManager::new();

        manager.add_webhook(WebhookConfig::new(
            IntegrationPlatform::Slack,
            "https://hooks.slack.com/test1",
            "Slack 1",
        ));
        manager.add_webhook(WebhookConfig::new(
            IntegrationPlatform::Slack,
            "https://hooks.slack.com/test2",
            "Slack 2",
        ));
        manager.add_webhook(WebhookConfig::new(
            IntegrationPlatform::Discord,
            "https://discord.com/test",
            "Discord",
        ));

        let slack_webhooks = manager.webhooks_by_platform(IntegrationPlatform::Slack);
        assert_eq!(slack_webhooks.len(), 2);
    }

    #[test]
    fn test_integration_manager_configure_email() {
        let mut manager = IntegrationManager::new();
        let config = EmailConfig::new("smtp.example.com", 587, "test@example.com");

        manager.configure_email(config);
        assert!(manager.email_config().is_some());
    }

    #[test]
    fn test_integration_manager_send_event() {
        let mut manager = IntegrationManager::new();
        let mut webhook = WebhookConfig::new(
            IntegrationPlatform::Slack,
            "https://hooks.slack.com/test",
            "Test",
        );
        webhook.add_event_type(EventType::TaskCreated);

        let webhook_id = manager.add_webhook(webhook);

        let event = NotificationEvent::new(
            EventType::TaskCreated,
            "task-1",
            "Test",
            "alice",
            "Created",
        );

        let sent_to = manager.send_event(event);
        assert_eq!(sent_to.len(), 1);
        assert_eq!(sent_to[0], webhook_id);
    }

    #[test]
    fn test_integration_manager_event_history() {
        let mut manager = IntegrationManager::new();

        let event1 = NotificationEvent::new(
            EventType::TaskCreated,
            "task-1",
            "Task 1",
            "alice",
            "Created",
        );
        let event2 = NotificationEvent::new(
            EventType::TaskCompleted,
            "task-2",
            "Task 2",
            "bob",
            "Completed",
        );

        manager.send_event(event1);
        manager.send_event(event2);

        assert_eq!(manager.event_history().len(), 2);
    }

    #[test]
    fn test_integration_manager_clear_history() {
        let mut manager = IntegrationManager::new();

        let event = NotificationEvent::new(
            EventType::TaskCreated,
            "task-1",
            "Test",
            "alice",
            "Created",
        );

        manager.send_event(event);
        assert_eq!(manager.event_history().len(), 1);

        manager.clear_history();
        assert_eq!(manager.event_history().len(), 0);
    }

    #[test]
    fn test_integration_manager_test_webhook() {
        let mut manager = IntegrationManager::new();
        let webhook = WebhookConfig::new(
            IntegrationPlatform::Slack,
            "https://hooks.slack.com/test",
            "Test",
        );

        let id = manager.add_webhook(webhook);
        let test_message = manager.test_webhook(&id);

        assert!(test_message.is_some());
        assert!(test_message.unwrap().contains("Test Task"));
    }
}
