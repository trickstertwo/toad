/// Batch operations for applying actions to multiple items
///
/// Provides a framework for performing bulk operations with progress tracking
///
/// # Examples
///
/// ```
/// use toad::batch_ops::{BatchOperation, BatchResult};
///
/// let op = BatchOperation::new("Test Operation");
/// assert_eq!(op.name(), "Test Operation");
/// ```

use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Result of a single operation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpResult {
    /// Item identifier
    pub item: String,
    /// Success status
    pub success: bool,
    /// Error message (if failed)
    pub error: Option<String>,
}

impl OpResult {
    /// Create a successful result
    pub fn success<S: Into<String>>(item: S) -> Self {
        Self {
            item: item.into(),
            success: true,
            error: None,
        }
    }

    /// Create a failed result
    pub fn failure<S: Into<String>, E: Into<String>>(item: S, error: E) -> Self {
        Self {
            item: item.into(),
            success: false,
            error: Some(error.into()),
        }
    }
}

/// Batch operation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchResult {
    /// Operation name
    pub operation: String,
    /// Total items processed
    pub total: usize,
    /// Successful operations
    pub successful: usize,
    /// Failed operations
    pub failed: usize,
    /// Individual results
    pub results: Vec<OpResult>,
}

impl BatchResult {
    /// Create a new batch result
    pub fn new<S: Into<String>>(operation: S) -> Self {
        Self {
            operation: operation.into(),
            total: 0,
            successful: 0,
            failed: 0,
            results: Vec::new(),
        }
    }

    /// Add a result
    pub fn add_result(&mut self, result: OpResult) {
        self.total += 1;
        if result.success {
            self.successful += 1;
        } else {
            self.failed += 1;
        }
        self.results.push(result);
    }

    /// Get success rate (0.0 to 1.0)
    pub fn success_rate(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            self.successful as f64 / self.total as f64
        }
    }

    /// Check if all operations succeeded
    pub fn all_successful(&self) -> bool {
        self.failed == 0 && self.total > 0
    }

    /// Get failed items
    pub fn failed_items(&self) -> Vec<&OpResult> {
        self.results.iter().filter(|r| !r.success).collect()
    }

    /// Get summary string
    pub fn summary(&self) -> String {
        format!(
            "{}: {}/{} successful ({:.1}%)",
            self.operation,
            self.successful,
            self.total,
            self.success_rate() * 100.0
        )
    }
}

/// Batch operation handler
pub type BatchHandler<T> = Arc<dyn Fn(&T) -> Result<(), String> + Send + Sync>;

/// Batch operation
pub struct BatchOperation<T> {
    /// Operation name
    name: String,
    /// Items to process
    items: Vec<T>,
    /// Operation handler
    handler: Option<BatchHandler<T>>,
    /// Continue on error
    continue_on_error: bool,
}

impl<T> BatchOperation<T> {
    /// Create a new batch operation
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::batch_ops::BatchOperation;
    ///
    /// let op: BatchOperation<String> = BatchOperation::new("Delete Files");
    /// assert_eq!(op.name(), "Delete Files");
    /// ```
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            items: Vec::new(),
            handler: None,
            continue_on_error: true,
        }
    }

    /// Get operation name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set items
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::batch_ops::BatchOperation;
    ///
    /// let op = BatchOperation::new("Process")
    ///     .with_items(vec!["a", "b", "c"]);
    /// assert_eq!(op.item_count(), 3);
    /// ```
    pub fn with_items(mut self, items: Vec<T>) -> Self {
        self.items = items;
        self
    }

    /// Set handler
    pub fn with_handler(mut self, handler: BatchHandler<T>) -> Self {
        self.handler = Some(handler);
        self
    }

    /// Set whether to continue on error
    pub fn with_continue_on_error(mut self, continue_on_error: bool) -> Self {
        self.continue_on_error = continue_on_error;
        self
    }

    /// Get item count
    pub fn item_count(&self) -> usize {
        self.items.len()
    }

    /// Add item
    pub fn add_item(&mut self, item: T) {
        self.items.push(item);
    }
}

impl<T: std::fmt::Display> BatchOperation<T> {
    /// Execute the batch operation
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::batch_ops::{BatchOperation, BatchHandler};
    /// use std::sync::Arc;
    ///
    /// let handler: BatchHandler<String> = Arc::new(|item| {
    ///     if item == "fail" {
    ///         Err("Failed".to_string())
    ///     } else {
    ///         Ok(())
    ///     }
    /// });
    ///
    /// let op = BatchOperation::new("Test")
    ///     .with_items(vec!["a".to_string(), "fail".to_string(), "b".to_string()])
    ///     .with_handler(handler);
    ///
    /// let result = op.execute();
    /// assert_eq!(result.total, 3);
    /// assert_eq!(result.successful, 2);
    /// assert_eq!(result.failed, 1);
    /// ```
    pub fn execute(self) -> BatchResult {
        let mut result = BatchResult::new(self.name.clone());

        let handler = match self.handler {
            Some(h) => h,
            None => {
                // No handler provided, all items succeed
                for item in &self.items {
                    result.add_result(OpResult::success(item.to_string()));
                }
                return result;
            }
        };

        for item in &self.items {
            let op_result = match handler(item) {
                Ok(()) => OpResult::success(item.to_string()),
                Err(e) => {
                    let op_res = OpResult::failure(item.to_string(), e);
                    if !self.continue_on_error {
                        result.add_result(op_res);
                        break;
                    }
                    op_res
                }
            };

            result.add_result(op_result);
        }

        result
    }
}

/// Batch operation manager
#[derive(Debug, Clone, Default)]
pub struct BatchManager {
    /// Operation history
    history: Vec<BatchResult>,
    /// Maximum history size
    max_history: usize,
}

impl BatchManager {
    /// Create a new batch manager
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::batch_ops::BatchManager;
    ///
    /// let manager = BatchManager::new();
    /// assert_eq!(manager.history_count(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
            max_history: 100,
        }
    }

    /// Set maximum history size
    pub fn with_max_history(mut self, max: usize) -> Self {
        self.max_history = max;
        self
    }

    /// Record a result
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::batch_ops::{BatchManager, BatchResult};
    ///
    /// let mut manager = BatchManager::new();
    /// let result = BatchResult::new("Test");
    /// manager.record(result);
    /// assert_eq!(manager.history_count(), 1);
    /// ```
    pub fn record(&mut self, result: BatchResult) {
        self.history.insert(0, result);

        if self.history.len() > self.max_history {
            self.history.truncate(self.max_history);
        }
    }

    /// Get history count
    pub fn history_count(&self) -> usize {
        self.history.len()
    }

    /// Get all history
    pub fn history(&self) -> &[BatchResult] {
        &self.history
    }

    /// Get recent history (last n items)
    pub fn recent(&self, n: usize) -> &[BatchResult] {
        let end = self.history.len().min(n);
        &self.history[0..end]
    }

    /// Clear history
    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    /// Get statistics
    pub fn stats(&self) -> BatchStats {
        let mut stats = BatchStats::default();

        for result in &self.history {
            stats.total_operations += 1;
            stats.total_items += result.total;
            stats.successful_items += result.successful;
            stats.failed_items += result.failed;
        }

        stats
    }
}

/// Batch operation statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BatchStats {
    /// Total number of batch operations
    pub total_operations: usize,
    /// Total items processed
    pub total_items: usize,
    /// Total successful items
    pub successful_items: usize,
    /// Total failed items
    pub failed_items: usize,
}

impl BatchStats {
    /// Get overall success rate
    pub fn success_rate(&self) -> f64 {
        if self.total_items == 0 {
            0.0
        } else {
            self.successful_items as f64 / self.total_items as f64
        }
    }

    /// Get summary string
    pub fn summary(&self) -> String {
        format!(
            "{} operations, {} items ({} successful, {} failed, {:.1}% success rate)",
            self.total_operations,
            self.total_items,
            self.successful_items,
            self.failed_items,
            self.success_rate() * 100.0
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_op_result_success() {
        let result = OpResult::success("test_item");
        assert!(result.success);
        assert!(result.error.is_none());
        assert_eq!(result.item, "test_item");
    }

    #[test]
    fn test_op_result_failure() {
        let result = OpResult::failure("test_item", "error message");
        assert!(!result.success);
        assert_eq!(result.error, Some("error message".to_string()));
    }

    #[test]
    fn test_batch_result_creation() {
        let result = BatchResult::new("Test Operation");
        assert_eq!(result.operation, "Test Operation");
        assert_eq!(result.total, 0);
        assert_eq!(result.successful, 0);
        assert_eq!(result.failed, 0);
    }

    #[test]
    fn test_batch_result_add_result() {
        let mut result = BatchResult::new("Test");

        result.add_result(OpResult::success("item1"));
        assert_eq!(result.total, 1);
        assert_eq!(result.successful, 1);

        result.add_result(OpResult::failure("item2", "error"));
        assert_eq!(result.total, 2);
        assert_eq!(result.successful, 1);
        assert_eq!(result.failed, 1);
    }

    #[test]
    fn test_batch_result_success_rate() {
        let mut result = BatchResult::new("Test");

        result.add_result(OpResult::success("item1"));
        result.add_result(OpResult::success("item2"));
        result.add_result(OpResult::failure("item3", "error"));

        assert_eq!(result.success_rate(), 2.0 / 3.0);
    }

    #[test]
    fn test_batch_result_all_successful() {
        let mut result = BatchResult::new("Test");

        result.add_result(OpResult::success("item1"));
        result.add_result(OpResult::success("item2"));
        assert!(result.all_successful());

        result.add_result(OpResult::failure("item3", "error"));
        assert!(!result.all_successful());
    }

    #[test]
    fn test_batch_result_failed_items() {
        let mut result = BatchResult::new("Test");

        result.add_result(OpResult::success("item1"));
        result.add_result(OpResult::failure("item2", "error1"));
        result.add_result(OpResult::failure("item3", "error2"));

        let failed = result.failed_items();
        assert_eq!(failed.len(), 2);
    }

    #[test]
    fn test_batch_result_summary() {
        let mut result = BatchResult::new("Test Operation");
        result.add_result(OpResult::success("item1"));
        result.add_result(OpResult::success("item2"));

        let summary = result.summary();
        assert!(summary.contains("Test Operation"));
        assert!(summary.contains("2/2"));
    }

    #[test]
    fn test_batch_operation_creation() {
        let op: BatchOperation<String> = BatchOperation::new("Test");
        assert_eq!(op.name(), "Test");
        assert_eq!(op.item_count(), 0);
    }

    #[test]
    fn test_batch_operation_with_items() {
        let op = BatchOperation::new("Test")
            .with_items(vec!["a", "b", "c"]);
        assert_eq!(op.item_count(), 3);
    }

    #[test]
    fn test_batch_operation_add_item() {
        let mut op: BatchOperation<String> = BatchOperation::new("Test");
        op.add_item("item1".to_string());
        op.add_item("item2".to_string());
        assert_eq!(op.item_count(), 2);
    }

    #[test]
    fn test_batch_operation_execute_no_handler() {
        let op = BatchOperation::new("Test")
            .with_items(vec!["a".to_string(), "b".to_string()]);

        let result = op.execute();
        assert_eq!(result.total, 2);
        assert_eq!(result.successful, 2);
    }

    #[test]
    fn test_batch_operation_execute_with_handler() {
        let handler: BatchHandler<String> = Arc::new(|item| {
            if item == "fail" {
                Err("Failed".to_string())
            } else {
                Ok(())
            }
        });

        let op = BatchOperation::new("Test")
            .with_items(vec!["a".to_string(), "fail".to_string(), "b".to_string()])
            .with_handler(handler);

        let result = op.execute();
        assert_eq!(result.total, 3);
        assert_eq!(result.successful, 2);
        assert_eq!(result.failed, 1);
    }

    #[test]
    fn test_batch_operation_stop_on_error() {
        let handler: BatchHandler<String> = Arc::new(|item| {
            if item == "fail" {
                Err("Failed".to_string())
            } else {
                Ok(())
            }
        });

        let op = BatchOperation::new("Test")
            .with_items(vec!["a".to_string(), "fail".to_string(), "b".to_string()])
            .with_handler(handler)
            .with_continue_on_error(false);

        let result = op.execute();
        assert_eq!(result.total, 2); // Stops after fail
    }

    #[test]
    fn test_batch_manager_creation() {
        let manager = BatchManager::new();
        assert_eq!(manager.history_count(), 0);
    }

    #[test]
    fn test_batch_manager_record() {
        let mut manager = BatchManager::new();
        let result = BatchResult::new("Test");

        manager.record(result);
        assert_eq!(manager.history_count(), 1);
    }

    #[test]
    fn test_batch_manager_max_history() {
        let mut manager = BatchManager::new().with_max_history(2);

        manager.record(BatchResult::new("Op1"));
        manager.record(BatchResult::new("Op2"));
        manager.record(BatchResult::new("Op3"));

        assert_eq!(manager.history_count(), 2);
    }

    #[test]
    fn test_batch_manager_recent() {
        let mut manager = BatchManager::new();

        manager.record(BatchResult::new("Op1"));
        manager.record(BatchResult::new("Op2"));
        manager.record(BatchResult::new("Op3"));

        let recent = manager.recent(2);
        assert_eq!(recent.len(), 2);
        assert_eq!(recent[0].operation, "Op3"); // Most recent first
    }

    #[test]
    fn test_batch_manager_clear() {
        let mut manager = BatchManager::new();
        manager.record(BatchResult::new("Test"));
        manager.clear_history();
        assert_eq!(manager.history_count(), 0);
    }

    #[test]
    fn test_batch_manager_stats() {
        let mut manager = BatchManager::new();

        let mut result1 = BatchResult::new("Op1");
        result1.add_result(OpResult::success("item1"));
        result1.add_result(OpResult::failure("item2", "error"));

        let mut result2 = BatchResult::new("Op2");
        result2.add_result(OpResult::success("item3"));

        manager.record(result1);
        manager.record(result2);

        let stats = manager.stats();
        assert_eq!(stats.total_operations, 2);
        assert_eq!(stats.total_items, 3);
        assert_eq!(stats.successful_items, 2);
        assert_eq!(stats.failed_items, 1);
    }

    #[test]
    fn test_batch_stats_success_rate() {
        let stats = BatchStats {
            total_operations: 2,
            total_items: 10,
            successful_items: 8,
            failed_items: 2,
        };

        assert_eq!(stats.success_rate(), 0.8);
    }

    #[test]
    fn test_batch_stats_summary() {
        let stats = BatchStats {
            total_operations: 2,
            total_items: 10,
            successful_items: 8,
            failed_items: 2,
        };

        let summary = stats.summary();
        assert!(summary.contains("2 operations"));
        assert!(summary.contains("10 items"));
        assert!(summary.contains("8 successful"));
        assert!(summary.contains("2 failed"));
    }

    #[test]
    fn test_batch_manager_default() {
        let manager = BatchManager::default();
        assert_eq!(manager.history_count(), 0);
    }
}
