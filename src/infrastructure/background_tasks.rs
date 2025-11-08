/// Background task management with progress indicators
///
/// Tracks long-running operations with progress updates and status
///
/// # Examples
///
/// ```
/// use toad::background_tasks::{BackgroundTaskManager, TaskStatus};
///
/// let mut manager = BackgroundTaskManager::new();
/// let task_id = manager.add_task("Download file");
///
/// manager.update_progress(task_id, 50, Some("Downloading..."));
/// assert_eq!(manager.get_task(task_id).map(|t| t.progress), Some(50));
/// ```
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

/// Unique task identifier
pub type TaskId = usize;

/// Task status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    /// Task is queued but not started
    Pending,
    /// Task is currently running
    Running,
    /// Task completed successfully
    Completed,
    /// Task failed with error
    Failed,
    /// Task was cancelled
    Cancelled,
}

impl TaskStatus {
    /// Check if task is finished (completed, failed, or cancelled)
    pub fn is_finished(&self) -> bool {
        matches!(
            self,
            TaskStatus::Completed | TaskStatus::Failed | TaskStatus::Cancelled
        )
    }

    /// Check if task is active (pending or running)
    pub fn is_active(&self) -> bool {
        matches!(self, TaskStatus::Pending | TaskStatus::Running)
    }

    /// Get status as string
    pub fn as_str(&self) -> &'static str {
        match self {
            TaskStatus::Pending => "Pending",
            TaskStatus::Running => "Running",
            TaskStatus::Completed => "Completed",
            TaskStatus::Failed => "Failed",
            TaskStatus::Cancelled => "Cancelled",
        }
    }
}

/// Background task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackgroundTask {
    /// Unique task ID
    pub id: TaskId,
    /// Task name/description
    pub name: String,
    /// Current status
    pub status: TaskStatus,
    /// Progress percentage (0-100)
    pub progress: u8,
    /// Optional progress message
    pub message: Option<String>,
    /// When task was created
    #[serde(skip, default = "Instant::now")]
    pub created_at: Instant,
    /// When task started running
    #[serde(skip, default)]
    pub started_at: Option<Instant>,
    /// When task finished
    #[serde(skip, default)]
    pub finished_at: Option<Instant>,
}

impl BackgroundTask {
    /// Create a new background task
    pub fn new(id: TaskId, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
            status: TaskStatus::Pending,
            progress: 0,
            message: None,
            created_at: Instant::now(),
            started_at: None,
            finished_at: None,
        }
    }

    /// Get elapsed time since creation
    pub fn elapsed(&self) -> Duration {
        self.created_at.elapsed()
    }

    /// Get running time (time since started)
    pub fn running_time(&self) -> Option<Duration> {
        self.started_at.map(|start| start.elapsed())
    }

    /// Get completion time (time from start to finish)
    pub fn completion_time(&self) -> Option<Duration> {
        match (self.started_at, self.finished_at) {
            (Some(start), Some(end)) => Some(end.duration_since(start)),
            _ => None,
        }
    }

    /// Check if task is finished
    pub fn is_finished(&self) -> bool {
        self.status.is_finished()
    }

    /// Check if task is active
    pub fn is_active(&self) -> bool {
        self.status.is_active()
    }
}

/// Background task manager
#[derive(Debug, Clone)]
pub struct BackgroundTaskManager {
    /// All tasks
    tasks: Vec<BackgroundTask>,
    /// Next task ID to assign
    next_id: TaskId,
    /// Maximum number of finished tasks to keep
    max_finished: usize,
}

impl BackgroundTaskManager {
    /// Create a new task manager
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::background_tasks::BackgroundTaskManager;
    ///
    /// let manager = BackgroundTaskManager::new();
    /// assert_eq!(manager.task_count(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            next_id: 0,
            max_finished: 50, // Keep last 50 finished tasks
        }
    }

    /// Add a new task
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::background_tasks::BackgroundTaskManager;
    ///
    /// let mut manager = BackgroundTaskManager::new();
    /// let task_id = manager.add_task("Process data");
    /// assert_eq!(manager.task_count(), 1);
    /// ```
    pub fn add_task(&mut self, name: impl Into<String>) -> TaskId {
        let id = self.next_id;
        self.next_id += 1;

        let task = BackgroundTask::new(id, name);
        self.tasks.push(task);

        // Clean up old finished tasks if needed
        self.cleanup_finished();

        id
    }

    /// Get task by ID
    pub fn get_task(&self, id: TaskId) -> Option<&BackgroundTask> {
        self.tasks.iter().find(|t| t.id == id)
    }

    /// Get mutable task by ID
    pub fn get_task_mut(&mut self, id: TaskId) -> Option<&mut BackgroundTask> {
        self.tasks.iter_mut().find(|t| t.id == id)
    }

    /// Update task progress
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::background_tasks::BackgroundTaskManager;
    ///
    /// let mut manager = BackgroundTaskManager::new();
    /// let task_id = manager.add_task("Upload");
    ///
    /// manager.update_progress(task_id, 50, Some("Uploading..."));
    /// assert_eq!(manager.get_task(task_id).map(|t| t.progress), Some(50));
    /// ```
    pub fn update_progress(
        &mut self,
        id: TaskId,
        progress: u8,
        message: Option<impl Into<String>>,
    ) -> bool {
        if let Some(task) = self.get_task_mut(id) {
            task.progress = progress.min(100);
            task.message = message.map(|m| m.into());
            true
        } else {
            false
        }
    }

    /// Start a task (change status to Running)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::background_tasks::{BackgroundTaskManager, TaskStatus};
    ///
    /// let mut manager = BackgroundTaskManager::new();
    /// let task_id = manager.add_task("Build");
    ///
    /// manager.start_task(task_id);
    /// assert_eq!(manager.get_task(task_id).map(|t| t.status), Some(TaskStatus::Running));
    /// ```
    pub fn start_task(&mut self, id: TaskId) -> bool {
        if let Some(task) = self.get_task_mut(id) {
            task.status = TaskStatus::Running;
            task.started_at = Some(Instant::now());
            true
        } else {
            false
        }
    }

    /// Complete a task successfully
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::background_tasks::{BackgroundTaskManager, TaskStatus};
    ///
    /// let mut manager = BackgroundTaskManager::new();
    /// let task_id = manager.add_task("Test");
    ///
    /// manager.complete_task(task_id);
    /// assert_eq!(manager.get_task(task_id).map(|t| t.status), Some(TaskStatus::Completed));
    /// ```
    pub fn complete_task(&mut self, id: TaskId) -> bool {
        if let Some(task) = self.get_task_mut(id) {
            task.status = TaskStatus::Completed;
            task.progress = 100;
            task.finished_at = Some(Instant::now());
            true
        } else {
            false
        }
    }

    /// Fail a task
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::background_tasks::{BackgroundTaskManager, TaskStatus};
    ///
    /// let mut manager = BackgroundTaskManager::new();
    /// let task_id = manager.add_task("Deploy");
    ///
    /// manager.fail_task(task_id, "Connection timeout");
    /// assert_eq!(manager.get_task(task_id).map(|t| t.status), Some(TaskStatus::Failed));
    /// ```
    pub fn fail_task(&mut self, id: TaskId, message: impl Into<String>) -> bool {
        if let Some(task) = self.get_task_mut(id) {
            task.status = TaskStatus::Failed;
            task.message = Some(message.into());
            task.finished_at = Some(Instant::now());
            true
        } else {
            false
        }
    }

    /// Cancel a task
    pub fn cancel_task(&mut self, id: TaskId) -> bool {
        if let Some(task) = self.get_task_mut(id) {
            task.status = TaskStatus::Cancelled;
            task.finished_at = Some(Instant::now());
            true
        } else {
            false
        }
    }

    /// Get all tasks
    pub fn tasks(&self) -> &[BackgroundTask] {
        &self.tasks
    }

    /// Get all active tasks (pending or running)
    pub fn active_tasks(&self) -> Vec<&BackgroundTask> {
        self.tasks.iter().filter(|t| t.is_active()).collect()
    }

    /// Get all finished tasks
    pub fn finished_tasks(&self) -> Vec<&BackgroundTask> {
        self.tasks.iter().filter(|t| t.is_finished()).collect()
    }

    /// Get task count
    pub fn task_count(&self) -> usize {
        self.tasks.len()
    }

    /// Get active task count
    pub fn active_count(&self) -> usize {
        self.tasks.iter().filter(|t| t.is_active()).count()
    }

    /// Remove finished tasks
    pub fn cleanup_finished(&mut self) {
        let finished_count = self.tasks.iter().filter(|t| t.is_finished()).count();

        if finished_count > self.max_finished {
            // Keep only max_finished most recent finished tasks
            let to_remove = finished_count - self.max_finished;
            let mut removed = 0;

            self.tasks.retain(|task| {
                if removed < to_remove && task.is_finished() {
                    removed += 1;
                    false
                } else {
                    true
                }
            });
        }
    }

    /// Remove a specific task
    pub fn remove_task(&mut self, id: TaskId) -> Option<BackgroundTask> {
        if let Some(idx) = self.tasks.iter().position(|t| t.id == id) {
            Some(self.tasks.remove(idx))
        } else {
            None
        }
    }

    /// Clear all tasks
    pub fn clear(&mut self) {
        self.tasks.clear();
    }
}

impl Default for BackgroundTaskManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_status() {
        assert!(TaskStatus::Completed.is_finished());
        assert!(TaskStatus::Failed.is_finished());
        assert!(TaskStatus::Cancelled.is_finished());
        assert!(!TaskStatus::Pending.is_finished());
        assert!(!TaskStatus::Running.is_finished());

        assert!(TaskStatus::Pending.is_active());
        assert!(TaskStatus::Running.is_active());
        assert!(!TaskStatus::Completed.is_active());
    }

    #[test]
    fn test_task_creation() {
        let task = BackgroundTask::new(0, "Test task");
        assert_eq!(task.id, 0);
        assert_eq!(task.name, "Test task");
        assert_eq!(task.status, TaskStatus::Pending);
        assert_eq!(task.progress, 0);
        assert!(task.message.is_none());
    }

    #[test]
    fn test_manager_creation() {
        let manager = BackgroundTaskManager::new();
        assert_eq!(manager.task_count(), 0);
        assert_eq!(manager.active_count(), 0);
    }

    #[test]
    fn test_add_task() {
        let mut manager = BackgroundTaskManager::new();
        let task_id = manager.add_task("Task 1");

        assert_eq!(manager.task_count(), 1);
        assert_eq!(
            manager.get_task(task_id).map(|t| t.name.as_str()),
            Some("Task 1")
        );
    }

    #[test]
    fn test_update_progress() {
        let mut manager = BackgroundTaskManager::new();
        let task_id = manager.add_task("Download");

        assert!(manager.update_progress(task_id, 50, Some("Downloading...")));
        let task = manager.get_task(task_id).unwrap();
        assert_eq!(task.progress, 50);
        assert_eq!(task.message.as_deref(), Some("Downloading..."));

        // Progress clamped at 100
        assert!(manager.update_progress(task_id, 150, Option::<&str>::None));
        assert_eq!(manager.get_task(task_id).unwrap().progress, 100);
    }

    #[test]
    fn test_start_task() {
        let mut manager = BackgroundTaskManager::new();
        let task_id = manager.add_task("Build");

        assert!(manager.start_task(task_id));
        let task = manager.get_task(task_id).unwrap();
        assert_eq!(task.status, TaskStatus::Running);
        assert!(task.started_at.is_some());
    }

    #[test]
    fn test_complete_task() {
        let mut manager = BackgroundTaskManager::new();
        let task_id = manager.add_task("Test");

        assert!(manager.complete_task(task_id));
        let task = manager.get_task(task_id).unwrap();
        assert_eq!(task.status, TaskStatus::Completed);
        assert_eq!(task.progress, 100);
        assert!(task.finished_at.is_some());
    }

    #[test]
    fn test_fail_task() {
        let mut manager = BackgroundTaskManager::new();
        let task_id = manager.add_task("Deploy");

        assert!(manager.fail_task(task_id, "Connection timeout"));
        let task = manager.get_task(task_id).unwrap();
        assert_eq!(task.status, TaskStatus::Failed);
        assert_eq!(task.message.as_deref(), Some("Connection timeout"));
        assert!(task.finished_at.is_some());
    }

    #[test]
    fn test_cancel_task() {
        let mut manager = BackgroundTaskManager::new();
        let task_id = manager.add_task("Long operation");

        assert!(manager.cancel_task(task_id));
        let task = manager.get_task(task_id).unwrap();
        assert_eq!(task.status, TaskStatus::Cancelled);
        assert!(task.finished_at.is_some());
    }

    #[test]
    fn test_active_and_finished_tasks() {
        let mut manager = BackgroundTaskManager::new();
        let task1 = manager.add_task("Task 1");
        let task2 = manager.add_task("Task 2");
        let _task3 = manager.add_task("Task 3");

        manager.start_task(task1);
        manager.complete_task(task2);

        assert_eq!(manager.active_count(), 2); // task1 running, task3 pending
        assert_eq!(manager.active_tasks().len(), 2);
        assert_eq!(manager.finished_tasks().len(), 1); // task2 completed
    }

    #[test]
    fn test_remove_task() {
        let mut manager = BackgroundTaskManager::new();
        let task_id = manager.add_task("Task");

        assert_eq!(manager.task_count(), 1);

        let removed = manager.remove_task(task_id);
        assert!(removed.is_some());
        assert_eq!(manager.task_count(), 0);

        assert!(manager.remove_task(999).is_none());
    }

    #[test]
    fn test_clear_tasks() {
        let mut manager = BackgroundTaskManager::new();
        manager.add_task("Task 1");
        manager.add_task("Task 2");

        assert_eq!(manager.task_count(), 2);

        manager.clear();
        assert_eq!(manager.task_count(), 0);
    }

    #[test]
    fn test_task_lifecycle() {
        let mut manager = BackgroundTaskManager::new();
        let task_id = manager.add_task("Full lifecycle");

        // Start
        manager.start_task(task_id);
        assert_eq!(
            manager.get_task(task_id).unwrap().status,
            TaskStatus::Running
        );

        // Progress updates
        manager.update_progress(task_id, 30, Some("Step 1"));
        manager.update_progress(task_id, 70, Some("Step 2"));

        // Complete
        manager.complete_task(task_id);
        let task = manager.get_task(task_id).unwrap();
        assert_eq!(task.status, TaskStatus::Completed);
        assert_eq!(task.progress, 100);
        assert!(task.is_finished());
    }
}
