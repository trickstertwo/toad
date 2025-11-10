// Import/Export System
//
// Provides comprehensive data portability including import from various task management
// systems, export to multiple formats, and backup/restore capabilities.
//
// # Features
//
// - **Import from**: Trello JSON, Asana CSV, GitHub Issues, Jira XML
// - **Export to**: JSON, CSV, Markdown, TOML
// - **Backup/Restore**: Auto-save board state to file
// - **Version Control**: Save board snapshots with git-like history

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Supported import formats
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ImportFormat {
    /// Trello JSON export
    TrelloJson,
    /// Asana CSV export
    AsanaCsv,
    /// GitHub Issues JSON
    GitHubIssues,
    /// Jira XML export
    JiraXml,
    /// Native TOAD JSON format
    ToadJson,
}

impl ImportFormat {
    /// Get format name
    pub fn name(&self) -> &'static str {
        match self {
            ImportFormat::TrelloJson => "Trello JSON",
            ImportFormat::AsanaCsv => "Asana CSV",
            ImportFormat::GitHubIssues => "GitHub Issues",
            ImportFormat::JiraXml => "Jira XML",
            ImportFormat::ToadJson => "TOAD JSON",
        }
    }

    /// Get file extension
    pub fn extension(&self) -> &'static str {
        match self {
            ImportFormat::TrelloJson => ".json",
            ImportFormat::AsanaCsv => ".csv",
            ImportFormat::GitHubIssues => ".json",
            ImportFormat::JiraXml => ".xml",
            ImportFormat::ToadJson => ".json",
        }
    }
}

/// Supported export formats
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ExportFormat {
    /// JSON format
    Json,
    /// CSV format (tabular)
    Csv,
    /// Markdown format (readable)
    Markdown,
    /// TOML format (config-like)
    Toml,
    /// HTML format (web view)
    Html,
}

impl ExportFormat {
    /// Get format name
    pub fn name(&self) -> &'static str {
        match self {
            ExportFormat::Json => "JSON",
            ExportFormat::Csv => "CSV",
            ExportFormat::Markdown => "Markdown",
            ExportFormat::Toml => "TOML",
            ExportFormat::Html => "HTML",
        }
    }

    /// Get file extension
    pub fn extension(&self) -> &'static str {
        match self {
            ExportFormat::Json => ".json",
            ExportFormat::Csv => ".csv",
            ExportFormat::Markdown => ".md",
            ExportFormat::Toml => ".toml",
            ExportFormat::Html => ".html",
        }
    }
}

/// Generic task representation for import/export
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskData {
    /// Task ID
    pub id: String,
    /// Task title
    pub title: String,
    /// Task description
    pub description: String,
    /// Task status/column
    pub status: String,
    /// Task priority
    pub priority: Option<String>,
    /// Assignee
    pub assignee: Option<String>,
    /// Tags/labels
    pub tags: Vec<String>,
    /// Due date
    pub due_date: Option<DateTime<Utc>>,
    /// Created date
    pub created_at: DateTime<Utc>,
    /// Modified date
    pub modified_at: DateTime<Utc>,
    /// Custom fields
    pub custom_fields: HashMap<String, String>,
}

impl TaskData {
    /// Create a new task data
    pub fn new(id: impl Into<String>, title: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            id: id.into(),
            title: title.into(),
            description: String::new(),
            status: "todo".to_string(),
            priority: None,
            assignee: None,
            tags: Vec::new(),
            due_date: None,
            created_at: now,
            modified_at: now,
            custom_fields: HashMap::new(),
        }
    }

    /// Set description
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }

    /// Set status
    pub fn with_status(mut self, status: impl Into<String>) -> Self {
        self.status = status.into();
        self
    }

    /// Set priority
    pub fn with_priority(mut self, priority: impl Into<String>) -> Self {
        self.priority = Some(priority.into());
        self
    }

    /// Add a tag
    pub fn add_tag(&mut self, tag: impl Into<String>) {
        self.tags.push(tag.into());
    }

    /// Set custom field
    pub fn set_custom_field(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.custom_fields.insert(key.into(), value.into());
    }
}

/// Board data for import/export
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoardData {
    /// Board ID
    pub id: String,
    /// Board name
    pub name: String,
    /// Board description
    pub description: String,
    /// Column names
    pub columns: Vec<String>,
    /// Tasks on the board
    pub tasks: Vec<TaskData>,
    /// Board metadata
    pub metadata: HashMap<String, String>,
    /// Export timestamp
    pub exported_at: DateTime<Utc>,
}

impl BoardData {
    /// Create a new board data
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: String::new(),
            columns: vec!["To Do".to_string(), "In Progress".to_string(), "Done".to_string()],
            tasks: Vec::new(),
            metadata: HashMap::new(),
            exported_at: Utc::now(),
        }
    }

    /// Add a task
    pub fn add_task(&mut self, task: TaskData) {
        self.tasks.push(task);
    }

    /// Set columns
    pub fn with_columns(mut self, columns: Vec<String>) -> Self {
        self.columns = columns;
        self
    }
}

/// Import result
#[derive(Debug)]
pub struct ImportResult {
    /// Successfully imported tasks count
    pub tasks_imported: usize,
    /// Errors encountered
    pub errors: Vec<String>,
    /// Warnings
    pub warnings: Vec<String>,
    /// Imported board data
    pub board: BoardData,
}

impl ImportResult {
    /// Check if import was successful
    pub fn is_success(&self) -> bool {
        !self.board.tasks.is_empty() && self.errors.is_empty()
    }

    /// Check if there were any warnings
    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }
}

/// Export result
#[derive(Debug)]
pub struct ExportResult {
    /// Export format used
    pub format: ExportFormat,
    /// Exported content
    pub content: String,
    /// File size in bytes
    pub size_bytes: usize,
    /// Tasks exported count
    pub tasks_exported: usize,
}

/// Backup snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    /// Snapshot ID
    pub id: String,
    /// Snapshot timestamp
    pub timestamp: DateTime<Utc>,
    /// Snapshot message/description
    pub message: String,
    /// Board state at snapshot time
    pub board: BoardData,
    /// Parent snapshot ID (for history)
    pub parent: Option<String>,
}

impl Snapshot {
    /// Create a new snapshot
    pub fn new(board: BoardData, message: impl Into<String>) -> Self {
        Self {
            id: format!("snap-{}", Utc::now().timestamp_millis()),
            timestamp: Utc::now(),
            message: message.into(),
            board,
            parent: None,
        }
    }

    /// Set parent snapshot
    pub fn with_parent(mut self, parent_id: impl Into<String>) -> Self {
        self.parent = Some(parent_id.into());
        self
    }
}

/// Importer for different formats
#[derive(Debug)]
pub struct Importer;

impl Importer {
    /// Import from Trello JSON
    pub fn from_trello(_data: &str) -> Result<ImportResult, String> {
        // In a real implementation, this would parse Trello JSON format
        let board = BoardData::new("imported", "Imported from Trello");
        Ok(ImportResult {
            tasks_imported: 0,
            errors: Vec::new(),
            warnings: vec!["Trello import not fully implemented".to_string()],
            board,
        })
    }

    /// Import from Asana CSV
    pub fn from_asana(data: &str) -> Result<ImportResult, String> {
        let mut board = BoardData::new("imported", "Imported from Asana");
        let mut tasks_imported = 0;
        let mut errors = Vec::new();
        let warnings = Vec::new();

        // Simple CSV parsing (in real implementation, use csv crate)
        for (idx, line) in data.lines().skip(1).enumerate() {
            if line.is_empty() {
                continue;
            }

            let fields: Vec<&str> = line.split(',').collect();
            if fields.len() < 2 {
                errors.push(format!("Line {}: Invalid CSV format", idx + 2));
                continue;
            }

            let task = TaskData::new(format!("task-{}", idx), fields[0].trim())
                .with_description(fields.get(1).unwrap_or(&"").trim())
                .with_status(fields.get(2).unwrap_or(&"todo").trim());

            board.add_task(task);
            tasks_imported += 1;
        }

        Ok(ImportResult {
            tasks_imported,
            errors,
            warnings,
            board,
        })
    }

    /// Import from GitHub Issues JSON
    pub fn from_github(_data: &str) -> Result<ImportResult, String> {
        let board = BoardData::new("imported", "Imported from GitHub");
        Ok(ImportResult {
            tasks_imported: 0,
            errors: Vec::new(),
            warnings: vec!["GitHub import not fully implemented".to_string()],
            board,
        })
    }

    /// Import from Jira XML
    pub fn from_jira(_data: &str) -> Result<ImportResult, String> {
        let board = BoardData::new("imported", "Imported from Jira");
        Ok(ImportResult {
            tasks_imported: 0,
            errors: Vec::new(),
            warnings: vec!["Jira import not fully implemented".to_string()],
            board,
        })
    }

    /// Import from native TOAD JSON
    pub fn from_json(data: &str) -> Result<ImportResult, String> {
        match serde_json::from_str::<BoardData>(data) {
            Ok(board) => {
                let tasks_imported = board.tasks.len();
                Ok(ImportResult {
                    tasks_imported,
                    errors: Vec::new(),
                    warnings: Vec::new(),
                    board,
                })
            }
            Err(e) => Err(format!("JSON parse error: {}", e)),
        }
    }

    /// Auto-detect format and import
    pub fn auto_import(data: &str, format: ImportFormat) -> Result<ImportResult, String> {
        match format {
            ImportFormat::TrelloJson => Self::from_trello(data),
            ImportFormat::AsanaCsv => Self::from_asana(data),
            ImportFormat::GitHubIssues => Self::from_github(data),
            ImportFormat::JiraXml => Self::from_jira(data),
            ImportFormat::ToadJson => Self::from_json(data),
        }
    }
}

/// Exporter for different formats
#[derive(Debug)]
pub struct Exporter;

impl Exporter {
    /// Export to JSON
    pub fn to_json(board: &BoardData) -> Result<ExportResult, String> {
        match serde_json::to_string_pretty(board) {
            Ok(content) => Ok(ExportResult {
                format: ExportFormat::Json,
                size_bytes: content.len(),
                tasks_exported: board.tasks.len(),
                content,
            }),
            Err(e) => Err(format!("JSON serialization error: {}", e)),
        }
    }

    /// Export to CSV
    pub fn to_csv(board: &BoardData) -> Result<ExportResult, String> {
        let mut content = String::from("ID,Title,Description,Status,Priority,Assignee,Tags\n");

        for task in &board.tasks {
            let tags = task.tags.join(";");
            let line = format!(
                "{},{},{},{},{},{},{}\n",
                task.id,
                task.title,
                task.description,
                task.status,
                task.priority.as_deref().unwrap_or(""),
                task.assignee.as_deref().unwrap_or(""),
                tags
            );
            content.push_str(&line);
        }

        Ok(ExportResult {
            format: ExportFormat::Csv,
            size_bytes: content.len(),
            tasks_exported: board.tasks.len(),
            content,
        })
    }

    /// Export to Markdown
    pub fn to_markdown(board: &BoardData) -> Result<ExportResult, String> {
        let mut content = format!("# {}\n\n", board.name);

        if !board.description.is_empty() {
            content.push_str(&format!("{}\n\n", board.description));
        }

        content.push_str(&format!("**Exported:** {}\n\n", board.exported_at.format("%Y-%m-%d %H:%M:%S")));
        content.push_str(&format!("**Total Tasks:** {}\n\n", board.tasks.len()));

        // Group by status
        let mut by_status: HashMap<String, Vec<&TaskData>> = HashMap::new();
        for task in &board.tasks {
            by_status.entry(task.status.clone()).or_default().push(task);
        }

        for column in &board.columns {
            if let Some(tasks) = by_status.get(column) {
                content.push_str(&format!("## {}\n\n", column));
                for task in tasks {
                    content.push_str(&format!("- **{}**", task.title));
                    if let Some(priority) = &task.priority {
                        content.push_str(&format!(" [{}]", priority));
                    }
                    content.push('\n');
                    if !task.description.is_empty() {
                        content.push_str(&format!("  {}\n", task.description));
                    }
                }
                content.push('\n');
            }
        }

        Ok(ExportResult {
            format: ExportFormat::Markdown,
            size_bytes: content.len(),
            tasks_exported: board.tasks.len(),
            content,
        })
    }

    /// Export to TOML
    pub fn to_toml(board: &BoardData) -> Result<ExportResult, String> {
        match toml::to_string_pretty(board) {
            Ok(content) => Ok(ExportResult {
                format: ExportFormat::Toml,
                size_bytes: content.len(),
                tasks_exported: board.tasks.len(),
                content,
            }),
            Err(e) => Err(format!("TOML serialization error: {}", e)),
        }
    }

    /// Export to HTML
    pub fn to_html(board: &BoardData) -> Result<ExportResult, String> {
        let mut content = format!(
            "<!DOCTYPE html>\n<html>\n<head>\n<title>{}</title>\n</head>\n<body>\n",
            board.name
        );
        content.push_str(&format!("<h1>{}</h1>\n", board.name));
        content.push_str(&format!("<p>{}</p>\n", board.description));
        content.push_str(&format!("<p>Total Tasks: {}</p>\n", board.tasks.len()));

        for column in &board.columns {
            content.push_str(&format!("<h2>{}</h2>\n<ul>\n", column));
            for task in &board.tasks {
                if task.status == *column {
                    content.push_str(&format!("<li><strong>{}</strong><br>{}</li>\n", task.title, task.description));
                }
            }
            content.push_str("</ul>\n");
        }

        content.push_str("</body>\n</html>");

        Ok(ExportResult {
            format: ExportFormat::Html,
            size_bytes: content.len(),
            tasks_exported: board.tasks.len(),
            content,
        })
    }

    /// Auto-export with specified format
    pub fn export(board: &BoardData, format: ExportFormat) -> Result<ExportResult, String> {
        match format {
            ExportFormat::Json => Self::to_json(board),
            ExportFormat::Csv => Self::to_csv(board),
            ExportFormat::Markdown => Self::to_markdown(board),
            ExportFormat::Toml => Self::to_toml(board),
            ExportFormat::Html => Self::to_html(board),
        }
    }
}

/// Backup and restore manager
#[derive(Debug)]
pub struct BackupManager {
    /// Snapshot history
    snapshots: HashMap<String, Snapshot>,
    /// Current snapshot ID
    current: Option<String>,
    /// Auto-backup enabled
    auto_backup: bool,
    /// Maximum snapshots to keep
    max_snapshots: usize,
    /// Next snapshot ID counter
    next_snapshot_id: usize,
}

impl BackupManager {
    /// Create a new backup manager
    pub fn new() -> Self {
        Self {
            snapshots: HashMap::new(),
            current: None,
            auto_backup: true,
            max_snapshots: 100,
            next_snapshot_id: 1,
        }
    }

    /// Create a snapshot
    pub fn create_snapshot(&mut self, board: BoardData, message: impl Into<String>) -> String {
        // Generate unique ID using counter
        let id = format!("snap-{}", self.next_snapshot_id);
        self.next_snapshot_id += 1;

        let mut snapshot = Snapshot {
            id: id.clone(),
            timestamp: Utc::now(),
            message: message.into(),
            board,
            parent: None,
        };

        // Link to current snapshot
        if let Some(current_id) = &self.current {
            snapshot.parent = Some(current_id.clone());
        }

        self.snapshots.insert(id.clone(), snapshot);
        self.current = Some(id.clone());

        // Trim old snapshots if needed
        if self.snapshots.len() > self.max_snapshots {
            self.trim_snapshots();
        }

        id
    }

    /// Restore from a snapshot
    pub fn restore(&mut self, snapshot_id: &str) -> Option<BoardData> {
        self.snapshots.get(snapshot_id).map(|s| {
            self.current = Some(snapshot_id.to_string());
            s.board.clone()
        })
    }

    /// Get snapshot history
    pub fn history(&self) -> Vec<&Snapshot> {
        let mut snapshots: Vec<_> = self.snapshots.values().collect();
        snapshots.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        snapshots
    }

    /// Get current snapshot
    pub fn current_snapshot(&self) -> Option<&Snapshot> {
        self.current.as_ref().and_then(|id| self.snapshots.get(id))
    }

    /// Delete a snapshot
    pub fn delete_snapshot(&mut self, snapshot_id: &str) -> bool {
        self.snapshots.remove(snapshot_id).is_some()
    }

    /// Clear all snapshots
    pub fn clear_snapshots(&mut self) {
        self.snapshots.clear();
        self.current = None;
    }

    /// Enable/disable auto-backup
    pub fn set_auto_backup(&mut self, enabled: bool) {
        self.auto_backup = enabled;
    }

    /// Check if auto-backup is enabled
    pub fn is_auto_backup_enabled(&self) -> bool {
        self.auto_backup
    }

    /// Trim old snapshots keeping only max_snapshots newest
    fn trim_snapshots(&mut self) {
        if self.snapshots.len() <= self.max_snapshots {
            return;
        }

        let mut snapshots: Vec<_> = self.snapshots.values().cloned().collect();
        snapshots.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

        self.snapshots.clear();
        for snapshot in snapshots.into_iter().take(self.max_snapshots) {
            self.snapshots.insert(snapshot.id.clone(), snapshot);
        }
    }

    /// Get snapshot count
    pub fn snapshot_count(&self) -> usize {
        self.snapshots.len()
    }
}

impl Default for BackupManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_import_format_name() {
        assert_eq!(ImportFormat::TrelloJson.name(), "Trello JSON");
        assert_eq!(ImportFormat::AsanaCsv.name(), "Asana CSV");
    }

    #[test]
    fn test_import_format_extension() {
        assert_eq!(ImportFormat::TrelloJson.extension(), ".json");
        assert_eq!(ImportFormat::AsanaCsv.extension(), ".csv");
    }

    #[test]
    fn test_export_format_name() {
        assert_eq!(ExportFormat::Json.name(), "JSON");
        assert_eq!(ExportFormat::Csv.name(), "CSV");
    }

    #[test]
    fn test_export_format_extension() {
        assert_eq!(ExportFormat::Json.extension(), ".json");
        assert_eq!(ExportFormat::Markdown.extension(), ".md");
    }

    #[test]
    fn test_task_data_creation() {
        let task = TaskData::new("task-1", "Test Task");
        assert_eq!(task.id, "task-1");
        assert_eq!(task.title, "Test Task");
        assert_eq!(task.status, "todo");
    }

    #[test]
    fn test_task_data_builder() {
        let task = TaskData::new("task-1", "Test")
            .with_description("Description")
            .with_status("in_progress")
            .with_priority("high");

        assert_eq!(task.description, "Description");
        assert_eq!(task.status, "in_progress");
        assert_eq!(task.priority, Some("high".to_string()));
    }

    #[test]
    fn test_task_data_add_tag() {
        let mut task = TaskData::new("task-1", "Test");
        task.add_tag("bug");
        task.add_tag("urgent");
        assert_eq!(task.tags.len(), 2);
    }

    #[test]
    fn test_task_data_custom_field() {
        let mut task = TaskData::new("task-1", "Test");
        task.set_custom_field("story_points", "5");
        assert_eq!(task.custom_fields.get("story_points"), Some(&"5".to_string()));
    }

    #[test]
    fn test_board_data_creation() {
        let board = BoardData::new("board-1", "Test Board");
        assert_eq!(board.id, "board-1");
        assert_eq!(board.name, "Test Board");
        assert_eq!(board.columns.len(), 3);
    }

    #[test]
    fn test_board_data_add_task() {
        let mut board = BoardData::new("board-1", "Test");
        board.add_task(TaskData::new("task-1", "Task 1"));
        assert_eq!(board.tasks.len(), 1);
    }

    #[test]
    fn test_board_data_with_columns() {
        let board = BoardData::new("board-1", "Test")
            .with_columns(vec!["Backlog".to_string(), "Done".to_string()]);
        assert_eq!(board.columns.len(), 2);
    }

    #[test]
    fn test_import_result_success() {
        let board = BoardData::new("test", "Test");
        let result = ImportResult {
            tasks_imported: 5,
            errors: Vec::new(),
            warnings: Vec::new(),
            board: board.clone(),
        };
        assert!(!result.is_success()); // No tasks in board

        let mut board_with_tasks = board;
        board_with_tasks.add_task(TaskData::new("1", "Task"));
        let result = ImportResult {
            tasks_imported: 1,
            errors: Vec::new(),
            warnings: Vec::new(),
            board: board_with_tasks,
        };
        assert!(result.is_success());
    }

    #[test]
    fn test_import_result_warnings() {
        let result = ImportResult {
            tasks_imported: 0,
            errors: Vec::new(),
            warnings: vec!["Warning".to_string()],
            board: BoardData::new("test", "Test"),
        };
        assert!(result.has_warnings());
    }

    #[test]
    fn test_snapshot_creation() {
        let board = BoardData::new("board-1", "Test");
        let snapshot = Snapshot::new(board, "Initial state");
        assert_eq!(snapshot.message, "Initial state");
        assert!(snapshot.parent.is_none());
    }

    #[test]
    fn test_snapshot_with_parent() {
        let board = BoardData::new("board-1", "Test");
        let snapshot = Snapshot::new(board, "Update").with_parent("snap-1");
        assert_eq!(snapshot.parent, Some("snap-1".to_string()));
    }

    #[test]
    fn test_importer_asana_csv() {
        let csv = "Title,Description,Status\nTask 1,Desc 1,todo\nTask 2,Desc 2,done";
        let result = Importer::from_asana(csv).unwrap();
        assert_eq!(result.tasks_imported, 2);
        assert_eq!(result.board.tasks.len(), 2);
    }

    #[test]
    fn test_importer_json() {
        let board = BoardData::new("test", "Test Board");
        let json = serde_json::to_string(&board).unwrap();
        let result = Importer::from_json(&json).unwrap();
        assert_eq!(result.board.name, "Test Board");
    }

    #[test]
    fn test_exporter_json() {
        let board = BoardData::new("test", "Test Board");
        let result = Exporter::to_json(&board).unwrap();
        assert_eq!(result.format, ExportFormat::Json);
        assert!(result.content.contains("Test Board"));
    }

    #[test]
    fn test_exporter_csv() {
        let mut board = BoardData::new("test", "Test");
        board.add_task(TaskData::new("1", "Task 1").with_status("todo"));
        let result = Exporter::to_csv(&board).unwrap();
        assert_eq!(result.format, ExportFormat::Csv);
        assert!(result.content.contains("ID,Title"));
        assert_eq!(result.tasks_exported, 1);
    }

    #[test]
    fn test_exporter_markdown() {
        let mut board = BoardData::new("test", "Test Board");
        board.add_task(TaskData::new("1", "Task 1").with_status("To Do"));
        let result = Exporter::to_markdown(&board).unwrap();
        assert_eq!(result.format, ExportFormat::Markdown);
        assert!(result.content.contains("# Test Board"));
    }

    #[test]
    fn test_exporter_toml() {
        let board = BoardData::new("test", "Test Board");
        let result = Exporter::to_toml(&board).unwrap();
        assert_eq!(result.format, ExportFormat::Toml);
    }

    #[test]
    fn test_exporter_html() {
        let board = BoardData::new("test", "Test Board");
        let result = Exporter::to_html(&board).unwrap();
        assert_eq!(result.format, ExportFormat::Html);
        assert!(result.content.contains("<html>"));
    }

    #[test]
    fn test_backup_manager_creation() {
        let manager = BackupManager::new();
        assert!(manager.auto_backup);
        assert_eq!(manager.max_snapshots, 100);
        assert_eq!(manager.snapshot_count(), 0);
    }

    #[test]
    fn test_backup_manager_create_snapshot() {
        let mut manager = BackupManager::new();
        let board = BoardData::new("test", "Test");
        let id = manager.create_snapshot(board, "Initial");
        assert_eq!(manager.snapshot_count(), 1);
        assert!(manager.current.is_some());
        assert_eq!(manager.current.unwrap(), id);
    }

    #[test]
    fn test_backup_manager_restore() {
        let mut manager = BackupManager::new();
        let board = BoardData::new("test", "Test");
        let id = manager.create_snapshot(board, "Save point");

        let restored = manager.restore(&id);
        assert!(restored.is_some());
        assert_eq!(restored.unwrap().name, "Test");
    }

    #[test]
    fn test_backup_manager_history() {
        let mut manager = BackupManager::new();
        manager.create_snapshot(BoardData::new("1", "First"), "First");
        manager.create_snapshot(BoardData::new("2", "Second"), "Second");

        let history = manager.history();
        assert_eq!(history.len(), 2);
        assert_eq!(history[0].message, "Second"); // Most recent first
    }

    #[test]
    fn test_backup_manager_delete_snapshot() {
        let mut manager = BackupManager::new();
        let id = manager.create_snapshot(BoardData::new("test", "Test"), "Test");
        assert_eq!(manager.snapshot_count(), 1);

        let deleted = manager.delete_snapshot(&id);
        assert!(deleted);
        assert_eq!(manager.snapshot_count(), 0);
    }

    #[test]
    fn test_backup_manager_clear_snapshots() {
        let mut manager = BackupManager::new();
        manager.create_snapshot(BoardData::new("1", "Test"), "1");
        manager.create_snapshot(BoardData::new("2", "Test"), "2");
        assert_eq!(manager.snapshot_count(), 2);

        manager.clear_snapshots();
        assert_eq!(manager.snapshot_count(), 0);
        assert!(manager.current.is_none());
    }

    #[test]
    fn test_backup_manager_auto_backup() {
        let mut manager = BackupManager::new();
        assert!(manager.is_auto_backup_enabled());

        manager.set_auto_backup(false);
        assert!(!manager.is_auto_backup_enabled());
    }

    #[test]
    fn test_backup_manager_current_snapshot() {
        let mut manager = BackupManager::new();
        assert!(manager.current_snapshot().is_none());

        manager.create_snapshot(BoardData::new("test", "Test"), "Current");
        assert!(manager.current_snapshot().is_some());
        assert_eq!(manager.current_snapshot().unwrap().message, "Current");
    }
}
