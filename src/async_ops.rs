/// Async operations framework for non-blocking I/O
///
/// Provides a simple async task system for handling non-blocking operations
///
/// # Examples
///
/// ```
/// use toad::async_ops::{AsyncOperation, AsyncOperationManager, OperationStatus};
///
/// let mut manager = AsyncOperationManager::new();
/// let op_id = manager.add_operation("Fetch data");
///
/// manager.start_operation(op_id);
/// assert_eq!(manager.get_operation(op_id).map(|o| o.status), Some(OperationStatus::Running));
/// ```

use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

/// Unique operation identifier
pub type OperationId = usize;

/// Operation status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OperationStatus {
    /// Operation is queued
    Queued,
    /// Operation is running
    Running,
    /// Operation completed successfully
    Completed,
    /// Operation failed
    Failed,
    /// Operation was cancelled
    Cancelled,
}

impl OperationStatus {
    /// Check if operation is finished
    pub fn is_finished(&self) -> bool {
        matches!(
            self,
            OperationStatus::Completed | OperationStatus::Failed | OperationStatus::Cancelled
        )
    }

    /// Check if operation is active
    pub fn is_active(&self) -> bool {
        matches!(self, OperationStatus::Queued | OperationStatus::Running)
    }

    /// Get status as string
    pub fn as_str(&self) -> &'static str {
        match self {
            OperationStatus::Queued => "Queued",
            OperationStatus::Running => "Running",
            OperationStatus::Completed => "Completed",
            OperationStatus::Failed => "Failed",
            OperationStatus::Cancelled => "Cancelled",
        }
    }
}

/// Async operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsyncOperation {
    /// Unique operation ID
    pub id: OperationId,
    /// Operation name
    pub name: String,
    /// Current status
    pub status: OperationStatus,
    /// Optional result data (JSON serializable)
    pub result: Option<String>,
    /// Optional error message
    pub error: Option<String>,
    /// When operation was created
    #[serde(skip, default = "Instant::now")]
    pub created_at: Instant,
    /// When operation started
    #[serde(skip, default)]
    pub started_at: Option<Instant>,
    /// When operation finished
    #[serde(skip, default)]
    pub finished_at: Option<Instant>,
}

impl AsyncOperation {
    /// Create a new async operation
    pub fn new(id: OperationId, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
            status: OperationStatus::Queued,
            result: None,
            error: None,
            created_at: Instant::now(),
            started_at: None,
            finished_at: None,
        }
    }

    /// Get elapsed time since creation
    pub fn elapsed(&self) -> Duration {
        self.created_at.elapsed()
    }

    /// Get running time
    pub fn running_time(&self) -> Option<Duration> {
        self.started_at.map(|start| start.elapsed())
    }

    /// Get completion time
    pub fn completion_time(&self) -> Option<Duration> {
        match (self.started_at, self.finished_at) {
            (Some(start), Some(end)) => Some(end.duration_since(start)),
            _ => None,
        }
    }

    /// Check if operation is finished
    pub fn is_finished(&self) -> bool {
        self.status.is_finished()
    }

    /// Check if operation is active
    pub fn is_active(&self) -> bool {
        self.status.is_active()
    }
}

/// Async operation manager
#[derive(Debug, Clone)]
pub struct AsyncOperationManager {
    /// All operations
    operations: Vec<AsyncOperation>,
    /// Next operation ID
    next_id: OperationId,
    /// Maximum finished operations to keep
    max_finished: usize,
}

impl AsyncOperationManager {
    /// Create a new async operation manager
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::async_ops::AsyncOperationManager;
    ///
    /// let manager = AsyncOperationManager::new();
    /// assert_eq!(manager.operation_count(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            operations: Vec::new(),
            next_id: 0,
            max_finished: 100,
        }
    }

    /// Add a new operation
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::async_ops::AsyncOperationManager;
    ///
    /// let mut manager = AsyncOperationManager::new();
    /// let op_id = manager.add_operation("Download file");
    /// assert_eq!(manager.operation_count(), 1);
    /// ```
    pub fn add_operation(&mut self, name: impl Into<String>) -> OperationId {
        let id = self.next_id;
        self.next_id += 1;

        let operation = AsyncOperation::new(id, name);
        self.operations.push(operation);

        self.cleanup_finished();

        id
    }

    /// Get operation by ID
    pub fn get_operation(&self, id: OperationId) -> Option<&AsyncOperation> {
        self.operations.iter().find(|op| op.id == id)
    }

    /// Get mutable operation by ID
    pub fn get_operation_mut(&mut self, id: OperationId) -> Option<&mut AsyncOperation> {
        self.operations.iter_mut().find(|op| op.id == id)
    }

    /// Start an operation
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::async_ops::{AsyncOperationManager, OperationStatus};
    ///
    /// let mut manager = AsyncOperationManager::new();
    /// let op_id = manager.add_operation("Fetch");
    ///
    /// manager.start_operation(op_id);
    /// assert_eq!(manager.get_operation(op_id).map(|o| o.status), Some(OperationStatus::Running));
    /// ```
    pub fn start_operation(&mut self, id: OperationId) -> bool {
        if let Some(op) = self.get_operation_mut(id) {
            op.status = OperationStatus::Running;
            op.started_at = Some(Instant::now());
            true
        } else {
            false
        }
    }

    /// Complete an operation with result
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::async_ops::{AsyncOperationManager, OperationStatus};
    ///
    /// let mut manager = AsyncOperationManager::new();
    /// let op_id = manager.add_operation("Query");
    ///
    /// manager.complete_operation(op_id, Some("Success"));
    /// assert_eq!(manager.get_operation(op_id).map(|o| o.status), Some(OperationStatus::Completed));
    /// ```
    pub fn complete_operation(
        &mut self,
        id: OperationId,
        result: Option<impl Into<String>>,
    ) -> bool {
        if let Some(op) = self.get_operation_mut(id) {
            op.status = OperationStatus::Completed;
            op.result = result.map(|r| r.into());
            op.finished_at = Some(Instant::now());
            true
        } else {
            false
        }
    }

    /// Fail an operation with error
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::async_ops::{AsyncOperationManager, OperationStatus};
    ///
    /// let mut manager = AsyncOperationManager::new();
    /// let op_id = manager.add_operation("Request");
    ///
    /// manager.fail_operation(op_id, "Connection timeout");
    /// assert_eq!(manager.get_operation(op_id).map(|o| o.status), Some(OperationStatus::Failed));
    /// ```
    pub fn fail_operation(&mut self, id: OperationId, error: impl Into<String>) -> bool {
        if let Some(op) = self.get_operation_mut(id) {
            op.status = OperationStatus::Failed;
            op.error = Some(error.into());
            op.finished_at = Some(Instant::now());
            true
        } else {
            false
        }
    }

    /// Cancel an operation
    pub fn cancel_operation(&mut self, id: OperationId) -> bool {
        if let Some(op) = self.get_operation_mut(id) {
            op.status = OperationStatus::Cancelled;
            op.finished_at = Some(Instant::now());
            true
        } else {
            false
        }
    }

    /// Get all operations
    pub fn operations(&self) -> &[AsyncOperation] {
        &self.operations
    }

    /// Get active operations
    pub fn active_operations(&self) -> Vec<&AsyncOperation> {
        self.operations.iter().filter(|op| op.is_active()).collect()
    }

    /// Get finished operations
    pub fn finished_operations(&self) -> Vec<&AsyncOperation> {
        self.operations
            .iter()
            .filter(|op| op.is_finished())
            .collect()
    }

    /// Get operation count
    pub fn operation_count(&self) -> usize {
        self.operations.len()
    }

    /// Get active operation count
    pub fn active_count(&self) -> usize {
        self.operations.iter().filter(|op| op.is_active()).count()
    }

    /// Remove finished operations
    pub fn cleanup_finished(&mut self) {
        let finished_count = self.operations.iter().filter(|op| op.is_finished()).count();

        if finished_count > self.max_finished {
            let to_remove = finished_count - self.max_finished;
            let mut removed = 0;

            self.operations.retain(|op| {
                if removed < to_remove && op.is_finished() {
                    removed += 1;
                    false
                } else {
                    true
                }
            });
        }
    }

    /// Remove operation by ID
    pub fn remove_operation(&mut self, id: OperationId) -> Option<AsyncOperation> {
        if let Some(idx) = self.operations.iter().position(|op| op.id == id) {
            Some(self.operations.remove(idx))
        } else {
            None
        }
    }

    /// Clear all operations
    pub fn clear(&mut self) {
        self.operations.clear();
    }
}

impl Default for AsyncOperationManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operation_status() {
        assert!(OperationStatus::Completed.is_finished());
        assert!(OperationStatus::Failed.is_finished());
        assert!(OperationStatus::Cancelled.is_finished());
        assert!(!OperationStatus::Queued.is_finished());
        assert!(!OperationStatus::Running.is_finished());

        assert!(OperationStatus::Queued.is_active());
        assert!(OperationStatus::Running.is_active());
        assert!(!OperationStatus::Completed.is_active());
    }

    #[test]
    fn test_operation_creation() {
        let op = AsyncOperation::new(0, "Test operation");
        assert_eq!(op.id, 0);
        assert_eq!(op.name, "Test operation");
        assert_eq!(op.status, OperationStatus::Queued);
        assert!(op.result.is_none());
        assert!(op.error.is_none());
    }

    #[test]
    fn test_manager_creation() {
        let manager = AsyncOperationManager::new();
        assert_eq!(manager.operation_count(), 0);
        assert_eq!(manager.active_count(), 0);
    }

    #[test]
    fn test_add_operation() {
        let mut manager = AsyncOperationManager::new();
        let op_id = manager.add_operation("Operation 1");

        assert_eq!(manager.operation_count(), 1);
        assert_eq!(
            manager.get_operation(op_id).map(|o| o.name.as_str()),
            Some("Operation 1")
        );
    }

    #[test]
    fn test_start_operation() {
        let mut manager = AsyncOperationManager::new();
        let op_id = manager.add_operation("Start test");

        assert!(manager.start_operation(op_id));
        let op = manager.get_operation(op_id).unwrap();
        assert_eq!(op.status, OperationStatus::Running);
        assert!(op.started_at.is_some());
    }

    #[test]
    fn test_complete_operation() {
        let mut manager = AsyncOperationManager::new();
        let op_id = manager.add_operation("Complete test");

        assert!(manager.complete_operation(op_id, Some("Success")));
        let op = manager.get_operation(op_id).unwrap();
        assert_eq!(op.status, OperationStatus::Completed);
        assert_eq!(op.result.as_deref(), Some("Success"));
        assert!(op.finished_at.is_some());
    }

    #[test]
    fn test_fail_operation() {
        let mut manager = AsyncOperationManager::new();
        let op_id = manager.add_operation("Fail test");

        assert!(manager.fail_operation(op_id, "Network error"));
        let op = manager.get_operation(op_id).unwrap();
        assert_eq!(op.status, OperationStatus::Failed);
        assert_eq!(op.error.as_deref(), Some("Network error"));
        assert!(op.finished_at.is_some());
    }

    #[test]
    fn test_cancel_operation() {
        let mut manager = AsyncOperationManager::new();
        let op_id = manager.add_operation("Cancel test");

        assert!(manager.cancel_operation(op_id));
        let op = manager.get_operation(op_id).unwrap();
        assert_eq!(op.status, OperationStatus::Cancelled);
        assert!(op.finished_at.is_some());
    }

    #[test]
    fn test_active_and_finished_operations() {
        let mut manager = AsyncOperationManager::new();
        let op1 = manager.add_operation("Op 1");
        let op2 = manager.add_operation("Op 2");
        let _op3 = manager.add_operation("Op 3");

        manager.start_operation(op1);
        manager.complete_operation(op2, Option::<&str>::None);

        assert_eq!(manager.active_count(), 2); // op1 running, op3 queued
        assert_eq!(manager.active_operations().len(), 2);
        assert_eq!(manager.finished_operations().len(), 1); // op2 completed
    }

    #[test]
    fn test_remove_operation() {
        let mut manager = AsyncOperationManager::new();
        let op_id = manager.add_operation("Remove test");

        assert_eq!(manager.operation_count(), 1);

        let removed = manager.remove_operation(op_id);
        assert!(removed.is_some());
        assert_eq!(manager.operation_count(), 0);
    }

    #[test]
    fn test_clear_operations() {
        let mut manager = AsyncOperationManager::new();
        manager.add_operation("Op 1");
        manager.add_operation("Op 2");

        assert_eq!(manager.operation_count(), 2);

        manager.clear();
        assert_eq!(manager.operation_count(), 0);
    }

    #[test]
    fn test_operation_lifecycle() {
        let mut manager = AsyncOperationManager::new();
        let op_id = manager.add_operation("Lifecycle test");

        // Start
        manager.start_operation(op_id);
        assert_eq!(
            manager.get_operation(op_id).unwrap().status,
            OperationStatus::Running
        );

        // Complete
        manager.complete_operation(op_id, Some("Done"));
        let op = manager.get_operation(op_id).unwrap();
        assert_eq!(op.status, OperationStatus::Completed);
        assert!(op.is_finished());
    }
}
