//! Cross-Window Context
//!
//! Enables context sharing and communication between multiple TOAD window instances.
//! Supports shared clipboard, drag & drop, context referencing, and agent context sharing.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Type of content stored in cross-window clipboard
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClipboardContentType {
    /// Plain text content
    Text,
    /// JSON-formatted data
    Json,
    /// Task data (from cards)
    Task,
    /// File paths
    FilePaths,
    /// Agent context
    AgentContext,
}

impl ClipboardContentType {
    /// Get the display name for the content type
    pub fn name(&self) -> &'static str {
        match self {
            ClipboardContentType::Text => "Text",
            ClipboardContentType::Json => "JSON",
            ClipboardContentType::Task => "Task",
            ClipboardContentType::FilePaths => "File Paths",
            ClipboardContentType::AgentContext => "Agent Context",
        }
    }
}

/// Entry in the shared clipboard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipboardEntry {
    /// Unique identifier for the entry
    pub id: String,
    /// Window ID that created this entry
    pub source_window_id: String,
    /// Type of content
    pub content_type: ClipboardContentType,
    /// The actual content
    pub content: String,
    /// Optional metadata (JSON)
    pub metadata: Option<String>,
    /// When this entry was created
    pub created_at: DateTime<Utc>,
    /// Size in bytes
    pub size_bytes: usize,
}

impl ClipboardEntry {
    /// Create a new clipboard entry
    pub fn new(
        id: String,
        source_window_id: String,
        content_type: ClipboardContentType,
        content: String,
    ) -> Self {
        let size_bytes = content.len();
        Self {
            id,
            source_window_id,
            content_type,
            content,
            metadata: None,
            created_at: Utc::now(),
            size_bytes,
        }
    }

    /// Set metadata for this entry
    pub fn with_metadata(mut self, metadata: String) -> Self {
        self.metadata = Some(metadata);
        self
    }

    /// Get a human-readable size string
    pub fn human_readable_size(&self) -> String {
        if self.size_bytes < 1024 {
            format!("{} B", self.size_bytes)
        } else if self.size_bytes < 1024 * 1024 {
            format!("{:.2} KB", self.size_bytes as f64 / 1024.0)
        } else {
            format!("{:.2} MB", self.size_bytes as f64 / (1024.0 * 1024.0))
        }
    }
}

/// Drag and drop operation between windows
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragDropOperation {
    /// Unique identifier for the operation
    pub id: String,
    /// Source window ID
    pub source_window_id: String,
    /// Target window ID (None if still in progress)
    pub target_window_id: Option<String>,
    /// Type of item being dragged
    pub item_type: String,
    /// Item ID being dragged
    pub item_id: String,
    /// Optional payload data
    pub payload: Option<String>,
    /// Operation status
    pub status: DragDropStatus,
    /// When the operation started
    pub started_at: DateTime<Utc>,
    /// When the operation completed (if finished)
    pub completed_at: Option<DateTime<Utc>>,
}

/// Status of a drag and drop operation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DragDropStatus {
    /// Operation is in progress
    InProgress,
    /// Operation completed successfully
    Completed,
    /// Operation was cancelled
    Cancelled,
    /// Operation failed
    Failed,
}

impl DragDropOperation {
    /// Create a new drag and drop operation
    pub fn new(source_window_id: String, item_type: String, item_id: String) -> Self {
        let id = format!("dnd-{}", Utc::now().timestamp_millis());
        Self {
            id,
            source_window_id,
            target_window_id: None,
            item_type,
            item_id,
            payload: None,
            status: DragDropStatus::InProgress,
            started_at: Utc::now(),
            completed_at: None,
        }
    }

    /// Complete the operation
    pub fn complete(&mut self, target_window_id: String) {
        self.target_window_id = Some(target_window_id);
        self.status = DragDropStatus::Completed;
        self.completed_at = Some(Utc::now());
    }

    /// Cancel the operation
    pub fn cancel(&mut self) {
        self.status = DragDropStatus::Cancelled;
        self.completed_at = Some(Utc::now());
    }

    /// Mark the operation as failed
    pub fn fail(&mut self) {
        self.status = DragDropStatus::Failed;
        self.completed_at = Some(Utc::now());
    }
}

/// Reference to another window's context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowContextReference {
    /// Window ID being referenced
    pub window_id: String,
    /// Type of context being referenced
    pub context_type: String,
    /// Context key or identifier
    pub context_key: String,
    /// When this reference was created
    pub created_at: DateTime<Utc>,
}

/// Shared agent context across windows
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedAgentContext {
    /// Unique identifier for the context
    pub id: String,
    /// Window ID that owns this context
    pub owner_window_id: String,
    /// Agent type or name
    pub agent_type: String,
    /// Context data (JSON)
    pub context_data: String,
    /// Last updated timestamp
    pub updated_at: DateTime<Utc>,
    /// Windows that have subscribed to this context
    pub subscribers: Vec<String>,
}

impl SharedAgentContext {
    /// Create a new shared agent context
    pub fn new(owner_window_id: String, agent_type: String, context_data: String) -> Self {
        let id = format!("ctx-{}-{}", agent_type, Utc::now().timestamp_millis());
        Self {
            id,
            owner_window_id,
            agent_type,
            context_data,
            updated_at: Utc::now(),
            subscribers: Vec::new(),
        }
    }

    /// Subscribe a window to this context
    pub fn subscribe(&mut self, window_id: String) {
        if !self.subscribers.contains(&window_id) {
            self.subscribers.push(window_id);
        }
    }

    /// Unsubscribe a window from this context
    pub fn unsubscribe(&mut self, window_id: &str) {
        self.subscribers.retain(|id| id != window_id);
    }

    /// Update the context data
    pub fn update(&mut self, context_data: String) {
        self.context_data = context_data;
        self.updated_at = Utc::now();
    }
}

/// Manager for cross-window context and communication
#[derive(Debug)]
pub struct CrossWindowContextManager {
    /// Shared clipboard entries
    clipboard: Vec<ClipboardEntry>,
    /// Maximum clipboard entries to keep
    max_clipboard_entries: usize,
    /// Active drag and drop operations
    drag_drop_operations: HashMap<String, DragDropOperation>,
    /// Window context references
    context_references: HashMap<String, Vec<WindowContextReference>>,
    /// Shared agent contexts
    agent_contexts: HashMap<String, SharedAgentContext>,
    /// Next clipboard entry ID
    next_clipboard_id: usize,
}

impl CrossWindowContextManager {
    /// Create a new cross-window context manager
    pub fn new() -> Self {
        Self {
            clipboard: Vec::new(),
            max_clipboard_entries: 100,
            drag_drop_operations: HashMap::new(),
            context_references: HashMap::new(),
            agent_contexts: HashMap::new(),
            next_clipboard_id: 1,
        }
    }

    /// Set the maximum number of clipboard entries to keep
    pub fn set_max_clipboard_entries(&mut self, max: usize) {
        self.max_clipboard_entries = max;
        self.trim_clipboard();
    }

    /// Add an entry to the shared clipboard
    pub fn add_to_clipboard(
        &mut self,
        source_window_id: String,
        content_type: ClipboardContentType,
        content: String,
        metadata: Option<String>,
    ) -> String {
        let id = format!("clip-{}", self.next_clipboard_id);
        self.next_clipboard_id += 1;

        let mut entry = ClipboardEntry::new(id.clone(), source_window_id, content_type, content);
        if let Some(meta) = metadata {
            entry = entry.with_metadata(meta);
        }

        self.clipboard.insert(0, entry);
        self.trim_clipboard();

        id
    }

    /// Get the most recent clipboard entry
    pub fn get_latest_clipboard(&self) -> Option<&ClipboardEntry> {
        self.clipboard.first()
    }

    /// Get a specific clipboard entry by ID
    pub fn get_clipboard_entry(&self, id: &str) -> Option<&ClipboardEntry> {
        self.clipboard.iter().find(|entry| entry.id == id)
    }

    /// Get all clipboard entries
    pub fn get_clipboard_history(&self) -> &[ClipboardEntry] {
        &self.clipboard
    }

    /// Get clipboard entries of a specific type
    pub fn get_clipboard_by_type(
        &self,
        content_type: ClipboardContentType,
    ) -> Vec<&ClipboardEntry> {
        self.clipboard
            .iter()
            .filter(|entry| entry.content_type == content_type)
            .collect()
    }

    /// Clear the clipboard
    pub fn clear_clipboard(&mut self) {
        self.clipboard.clear();
    }

    /// Trim clipboard to max entries
    fn trim_clipboard(&mut self) {
        if self.clipboard.len() > self.max_clipboard_entries {
            self.clipboard.truncate(self.max_clipboard_entries);
        }
    }

    /// Start a drag and drop operation
    pub fn start_drag_drop(
        &mut self,
        source_window_id: String,
        item_type: String,
        item_id: String,
        payload: Option<String>,
    ) -> String {
        let mut operation = DragDropOperation::new(source_window_id, item_type, item_id);
        operation.payload = payload;
        let id = operation.id.clone();
        self.drag_drop_operations.insert(id.clone(), operation);
        id
    }

    /// Complete a drag and drop operation
    pub fn complete_drag_drop(
        &mut self,
        operation_id: &str,
        target_window_id: String,
    ) -> Result<(), String> {
        if let Some(operation) = self.drag_drop_operations.get_mut(operation_id) {
            operation.complete(target_window_id);
            Ok(())
        } else {
            Err(format!(
                "Drag and drop operation {} not found",
                operation_id
            ))
        }
    }

    /// Cancel a drag and drop operation
    pub fn cancel_drag_drop(&mut self, operation_id: &str) -> Result<(), String> {
        if let Some(operation) = self.drag_drop_operations.get_mut(operation_id) {
            operation.cancel();
            Ok(())
        } else {
            Err(format!(
                "Drag and drop operation {} not found",
                operation_id
            ))
        }
    }

    /// Get an active drag and drop operation
    pub fn get_drag_drop_operation(&self, operation_id: &str) -> Option<&DragDropOperation> {
        self.drag_drop_operations.get(operation_id)
    }

    /// Get all active drag and drop operations
    pub fn get_active_drag_drops(&self) -> Vec<&DragDropOperation> {
        self.drag_drop_operations
            .values()
            .filter(|op| op.status == DragDropStatus::InProgress)
            .collect()
    }

    /// Clean up completed drag and drop operations
    pub fn cleanup_drag_drops(&mut self) {
        self.drag_drop_operations
            .retain(|_, op| op.status == DragDropStatus::InProgress);
    }

    /// Add a window context reference
    pub fn add_context_reference(
        &mut self,
        window_id: String,
        context_type: String,
        context_key: String,
    ) -> Result<(), String> {
        let reference = WindowContextReference {
            window_id: window_id.clone(),
            context_type,
            context_key,
            created_at: Utc::now(),
        };

        self.context_references
            .entry(window_id)
            .or_default()
            .push(reference);

        Ok(())
    }

    /// Get context references for a window
    pub fn get_context_references(&self, window_id: &str) -> Vec<&WindowContextReference> {
        self.context_references
            .get(window_id)
            .map(|refs| refs.iter().collect())
            .unwrap_or_default()
    }

    /// Remove all context references for a window
    pub fn remove_context_references(&mut self, window_id: &str) {
        self.context_references.remove(window_id);
    }

    /// Create a shared agent context
    pub fn create_shared_agent_context(
        &mut self,
        owner_window_id: String,
        agent_type: String,
        context_data: String,
    ) -> String {
        let context = SharedAgentContext::new(owner_window_id, agent_type, context_data);
        let id = context.id.clone();
        self.agent_contexts.insert(id.clone(), context);
        id
    }

    /// Get a shared agent context
    pub fn get_agent_context(&self, context_id: &str) -> Option<&SharedAgentContext> {
        self.agent_contexts.get(context_id)
    }

    /// Update a shared agent context
    pub fn update_agent_context(
        &mut self,
        context_id: &str,
        context_data: String,
    ) -> Result<(), String> {
        if let Some(context) = self.agent_contexts.get_mut(context_id) {
            context.update(context_data);
            Ok(())
        } else {
            Err(format!("Agent context {} not found", context_id))
        }
    }

    /// Subscribe a window to an agent context
    pub fn subscribe_to_agent_context(
        &mut self,
        context_id: &str,
        window_id: String,
    ) -> Result<(), String> {
        if let Some(context) = self.agent_contexts.get_mut(context_id) {
            context.subscribe(window_id);
            Ok(())
        } else {
            Err(format!("Agent context {} not found", context_id))
        }
    }

    /// Unsubscribe a window from an agent context
    pub fn unsubscribe_from_agent_context(
        &mut self,
        context_id: &str,
        window_id: &str,
    ) -> Result<(), String> {
        if let Some(context) = self.agent_contexts.get_mut(context_id) {
            context.unsubscribe(window_id);
            Ok(())
        } else {
            Err(format!("Agent context {} not found", context_id))
        }
    }

    /// Get all agent contexts owned by a window
    pub fn get_agent_contexts_by_owner(&self, owner_window_id: &str) -> Vec<&SharedAgentContext> {
        self.agent_contexts
            .values()
            .filter(|ctx| ctx.owner_window_id == owner_window_id)
            .collect()
    }

    /// Get all agent contexts subscribed to by a window
    pub fn get_subscribed_agent_contexts(&self, window_id: &str) -> Vec<&SharedAgentContext> {
        self.agent_contexts
            .values()
            .filter(|ctx| ctx.subscribers.contains(&window_id.to_string()))
            .collect()
    }

    /// Remove all agent contexts owned by a window
    pub fn remove_agent_contexts_by_owner(&mut self, owner_window_id: &str) {
        self.agent_contexts
            .retain(|_, ctx| ctx.owner_window_id != owner_window_id);
    }

    /// Get total clipboard size in bytes
    pub fn total_clipboard_size(&self) -> usize {
        self.clipboard.iter().map(|entry| entry.size_bytes).sum()
    }
}

impl Default for CrossWindowContextManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clipboard_content_type_name() {
        assert_eq!(ClipboardContentType::Text.name(), "Text");
        assert_eq!(ClipboardContentType::Json.name(), "JSON");
        assert_eq!(ClipboardContentType::Task.name(), "Task");
        assert_eq!(ClipboardContentType::FilePaths.name(), "File Paths");
        assert_eq!(ClipboardContentType::AgentContext.name(), "Agent Context");
    }

    #[test]
    fn test_clipboard_entry_creation() {
        let entry = ClipboardEntry::new(
            "clip-1".to_string(),
            "window-1".to_string(),
            ClipboardContentType::Text,
            "Hello, world!".to_string(),
        );

        assert_eq!(entry.id, "clip-1");
        assert_eq!(entry.source_window_id, "window-1");
        assert_eq!(entry.content_type, ClipboardContentType::Text);
        assert_eq!(entry.content, "Hello, world!");
        assert_eq!(entry.size_bytes, 13);
    }

    #[test]
    fn test_clipboard_entry_with_metadata() {
        let entry = ClipboardEntry::new(
            "clip-1".to_string(),
            "window-1".to_string(),
            ClipboardContentType::Json,
            "{}".to_string(),
        )
        .with_metadata("{\"key\":\"value\"}".to_string());

        assert_eq!(entry.metadata, Some("{\"key\":\"value\"}".to_string()));
    }

    #[test]
    fn test_clipboard_entry_human_readable_size() {
        let small = ClipboardEntry::new(
            "clip-1".to_string(),
            "win-1".to_string(),
            ClipboardContentType::Text,
            "Small".to_string(),
        );
        assert_eq!(small.human_readable_size(), "5 B");

        let kb_content = "a".repeat(2048);
        let kb = ClipboardEntry::new(
            "clip-2".to_string(),
            "win-1".to_string(),
            ClipboardContentType::Text,
            kb_content,
        );
        assert_eq!(kb.human_readable_size(), "2.00 KB");

        let mb_content = "a".repeat(2 * 1024 * 1024);
        let mb = ClipboardEntry::new(
            "clip-3".to_string(),
            "win-1".to_string(),
            ClipboardContentType::Text,
            mb_content,
        );
        assert_eq!(mb.human_readable_size(), "2.00 MB");
    }

    #[test]
    fn test_drag_drop_operation_creation() {
        let op = DragDropOperation::new(
            "window-1".to_string(),
            "task".to_string(),
            "task-123".to_string(),
        );

        assert_eq!(op.source_window_id, "window-1");
        assert_eq!(op.item_type, "task");
        assert_eq!(op.item_id, "task-123");
        assert_eq!(op.status, DragDropStatus::InProgress);
        assert_eq!(op.target_window_id, None);
    }

    #[test]
    fn test_drag_drop_operation_complete() {
        let mut op = DragDropOperation::new(
            "window-1".to_string(),
            "task".to_string(),
            "task-123".to_string(),
        );
        op.complete("window-2".to_string());

        assert_eq!(op.status, DragDropStatus::Completed);
        assert_eq!(op.target_window_id, Some("window-2".to_string()));
        assert!(op.completed_at.is_some());
    }

    #[test]
    fn test_drag_drop_operation_cancel() {
        let mut op = DragDropOperation::new(
            "window-1".to_string(),
            "task".to_string(),
            "task-123".to_string(),
        );
        op.cancel();

        assert_eq!(op.status, DragDropStatus::Cancelled);
        assert!(op.completed_at.is_some());
    }

    #[test]
    fn test_shared_agent_context_creation() {
        let ctx = SharedAgentContext::new(
            "window-1".to_string(),
            "coder".to_string(),
            "{\"files\":[\"main.rs\"]}".to_string(),
        );

        assert_eq!(ctx.owner_window_id, "window-1");
        assert_eq!(ctx.agent_type, "coder");
        assert_eq!(ctx.context_data, "{\"files\":[\"main.rs\"]}");
        assert!(ctx.subscribers.is_empty());
    }

    #[test]
    fn test_shared_agent_context_subscribe() {
        let mut ctx = SharedAgentContext::new(
            "window-1".to_string(),
            "coder".to_string(),
            "{}".to_string(),
        );

        ctx.subscribe("window-2".to_string());
        assert_eq!(ctx.subscribers.len(), 1);
        assert_eq!(ctx.subscribers[0], "window-2");

        // Duplicate subscribe should not add twice
        ctx.subscribe("window-2".to_string());
        assert_eq!(ctx.subscribers.len(), 1);
    }

    #[test]
    fn test_shared_agent_context_unsubscribe() {
        let mut ctx = SharedAgentContext::new(
            "window-1".to_string(),
            "coder".to_string(),
            "{}".to_string(),
        );

        ctx.subscribe("window-2".to_string());
        ctx.subscribe("window-3".to_string());
        assert_eq!(ctx.subscribers.len(), 2);

        ctx.unsubscribe("window-2");
        assert_eq!(ctx.subscribers.len(), 1);
        assert_eq!(ctx.subscribers[0], "window-3");
    }

    #[test]
    fn test_manager_add_to_clipboard() {
        let mut manager = CrossWindowContextManager::new();

        let id = manager.add_to_clipboard(
            "window-1".to_string(),
            ClipboardContentType::Text,
            "Test content".to_string(),
            None,
        );

        assert_eq!(id, "clip-1");
        assert_eq!(manager.clipboard.len(), 1);
        assert_eq!(
            manager.get_latest_clipboard().unwrap().content,
            "Test content"
        );
    }

    #[test]
    fn test_manager_clipboard_max_entries() {
        let mut manager = CrossWindowContextManager::new();
        manager.set_max_clipboard_entries(3);

        for i in 1..=5 {
            manager.add_to_clipboard(
                "window-1".to_string(),
                ClipboardContentType::Text,
                format!("Content {}", i),
                None,
            );
        }

        assert_eq!(manager.clipboard.len(), 3);
        // Most recent should be first
        assert_eq!(manager.get_latest_clipboard().unwrap().content, "Content 5");
    }

    #[test]
    fn test_manager_get_clipboard_by_type() {
        let mut manager = CrossWindowContextManager::new();

        manager.add_to_clipboard(
            "window-1".to_string(),
            ClipboardContentType::Text,
            "Text 1".to_string(),
            None,
        );
        manager.add_to_clipboard(
            "window-1".to_string(),
            ClipboardContentType::Json,
            "{}".to_string(),
            None,
        );
        manager.add_to_clipboard(
            "window-1".to_string(),
            ClipboardContentType::Text,
            "Text 2".to_string(),
            None,
        );

        let text_entries = manager.get_clipboard_by_type(ClipboardContentType::Text);
        assert_eq!(text_entries.len(), 2);

        let json_entries = manager.get_clipboard_by_type(ClipboardContentType::Json);
        assert_eq!(json_entries.len(), 1);
    }

    #[test]
    fn test_manager_start_drag_drop() {
        let mut manager = CrossWindowContextManager::new();

        let id = manager.start_drag_drop(
            "window-1".to_string(),
            "task".to_string(),
            "task-123".to_string(),
            Some("payload".to_string()),
        );

        let operation = manager.get_drag_drop_operation(&id).unwrap();
        assert_eq!(operation.source_window_id, "window-1");
        assert_eq!(operation.item_type, "task");
        assert_eq!(operation.item_id, "task-123");
        assert_eq!(operation.payload, Some("payload".to_string()));
        assert_eq!(operation.status, DragDropStatus::InProgress);
    }

    #[test]
    fn test_manager_complete_drag_drop() {
        let mut manager = CrossWindowContextManager::new();

        let id = manager.start_drag_drop(
            "window-1".to_string(),
            "task".to_string(),
            "task-123".to_string(),
            None,
        );

        let result = manager.complete_drag_drop(&id, "window-2".to_string());
        assert!(result.is_ok());

        let operation = manager.get_drag_drop_operation(&id).unwrap();
        assert_eq!(operation.status, DragDropStatus::Completed);
        assert_eq!(operation.target_window_id, Some("window-2".to_string()));
    }

    #[test]
    fn test_manager_context_references() {
        let mut manager = CrossWindowContextManager::new();

        manager
            .add_context_reference(
                "window-1".to_string(),
                "workspace".to_string(),
                "ws-123".to_string(),
            )
            .unwrap();

        manager
            .add_context_reference(
                "window-1".to_string(),
                "file".to_string(),
                "main.rs".to_string(),
            )
            .unwrap();

        let refs = manager.get_context_references("window-1");
        assert_eq!(refs.len(), 2);
    }

    #[test]
    fn test_manager_shared_agent_context() {
        let mut manager = CrossWindowContextManager::new();

        let ctx_id = manager.create_shared_agent_context(
            "window-1".to_string(),
            "coder".to_string(),
            "{\"files\":[]}".to_string(),
        );

        let context = manager.get_agent_context(&ctx_id).unwrap();
        assert_eq!(context.owner_window_id, "window-1");
        assert_eq!(context.agent_type, "coder");
    }

    #[test]
    fn test_manager_subscribe_to_agent_context() {
        let mut manager = CrossWindowContextManager::new();

        let ctx_id = manager.create_shared_agent_context(
            "window-1".to_string(),
            "coder".to_string(),
            "{}".to_string(),
        );

        manager
            .subscribe_to_agent_context(&ctx_id, "window-2".to_string())
            .unwrap();

        let context = manager.get_agent_context(&ctx_id).unwrap();
        assert_eq!(context.subscribers.len(), 1);
        assert_eq!(context.subscribers[0], "window-2");
    }

    #[test]
    fn test_manager_update_agent_context() {
        let mut manager = CrossWindowContextManager::new();

        let ctx_id = manager.create_shared_agent_context(
            "window-1".to_string(),
            "coder".to_string(),
            "{}".to_string(),
        );

        let old_timestamp = manager.get_agent_context(&ctx_id).unwrap().updated_at;

        std::thread::sleep(std::time::Duration::from_millis(10));

        manager
            .update_agent_context(&ctx_id, "{\"updated\":true}".to_string())
            .unwrap();

        let context = manager.get_agent_context(&ctx_id).unwrap();
        assert_eq!(context.context_data, "{\"updated\":true}");
        assert!(context.updated_at > old_timestamp);
    }

    #[test]
    fn test_manager_get_agent_contexts_by_owner() {
        let mut manager = CrossWindowContextManager::new();

        manager.create_shared_agent_context(
            "window-1".to_string(),
            "coder".to_string(),
            "{}".to_string(),
        );
        manager.create_shared_agent_context(
            "window-1".to_string(),
            "reviewer".to_string(),
            "{}".to_string(),
        );
        manager.create_shared_agent_context(
            "window-2".to_string(),
            "tester".to_string(),
            "{}".to_string(),
        );

        let window1_contexts = manager.get_agent_contexts_by_owner("window-1");
        assert_eq!(window1_contexts.len(), 2);
    }

    #[test]
    fn test_manager_total_clipboard_size() {
        let mut manager = CrossWindowContextManager::new();

        manager.add_to_clipboard(
            "window-1".to_string(),
            ClipboardContentType::Text,
            "123".to_string(),
            None,
        );
        manager.add_to_clipboard(
            "window-1".to_string(),
            ClipboardContentType::Text,
            "45678".to_string(),
            None,
        );

        assert_eq!(manager.total_clipboard_size(), 8);
    }

    #[test]
    fn test_manager_cleanup_drag_drops() {
        let mut manager = CrossWindowContextManager::new();

        let id1 = manager.start_drag_drop(
            "window-1".to_string(),
            "task".to_string(),
            "task-1".to_string(),
            None,
        );

        // Add a small delay to ensure unique IDs
        std::thread::sleep(std::time::Duration::from_millis(2));

        let id2 = manager.start_drag_drop(
            "window-1".to_string(),
            "task".to_string(),
            "task-2".to_string(),
            None,
        );

        manager
            .complete_drag_drop(&id1, "window-2".to_string())
            .unwrap();

        assert_eq!(manager.drag_drop_operations.len(), 2);

        manager.cleanup_drag_drops();

        assert_eq!(manager.drag_drop_operations.len(), 1);
        assert!(manager.get_drag_drop_operation(&id2).is_some());
    }
}
