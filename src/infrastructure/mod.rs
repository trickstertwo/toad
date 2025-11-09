//! Infrastructure domain
//!
//! Contains infrastructure and utility modules including async operations,
//! error handling, input handling, and file operations.

pub mod achievements;
pub mod advanced_mouse;
pub mod async_ops;
pub mod background_tasks;
pub mod batch_ops;
pub mod builtin_automation;
pub mod calendar_integration;
pub mod card_comments;
pub mod clipboard;
pub mod communication_integrations;
pub mod custom_keybindings;
pub mod custom_reports;
pub mod dashboard_metrics;
pub mod data_portability;
pub mod diff;
pub mod errors;
pub mod fallback_mode;
pub mod file_attachments;
pub mod file_ops;
pub mod filtering_search;
pub mod history;
pub mod import_export;
pub mod key_sequences;
pub mod keyboard_recorder;
pub mod keyboard_shortcuts;
pub mod keybinds;
pub mod mouse;
pub mod multiple_views;
pub mod project_management;
pub mod rich_task_cards;
pub mod task_dependencies;
pub mod team_collaboration;
pub mod terminal_capabilities;
pub mod text_truncation;
pub mod time_tracking;
pub mod validation;
pub mod visual_kanban_board;

pub use achievements::{
    Achievement, AchievementSystem, AchievementTier, AchievementType, LeaderboardEntry, Streak,
    UnlockedAchievement, UserStats,
};
pub use advanced_mouse::{AdvancedMouseHandler, MouseButton, MouseGesture};
pub use async_ops::{AsyncOperation, AsyncOperationManager, OperationId, OperationStatus};
pub use background_tasks::{BackgroundTask, BackgroundTaskManager, TaskId, TaskStatus};
pub use batch_ops::{
    BatchHandler, BatchManager, BatchOperation, BatchResult, BatchStats, OpResult,
};
pub use builtin_automation::{
    AutomationAction, AutomationManager, AutomationRule, BulkActionResult, BulkActionType,
    RecurrencePattern, RecurringTask, TaskTemplate, TriggerCondition,
};
pub use calendar_integration::{
    CalendarEvent, CalendarExporter, EventPriority, EventStatus, Recurrence,
};
pub use card_comments::{
    ActivityLogEntry, Comment, CommentManager, EditHistory, Reaction, ReactionEntry,
};
pub use clipboard::Clipboard;
pub use communication_integrations::{
    DiscordMessage, EmailConfig, EmailMessage, EventType, IntegrationManager,
    IntegrationPlatform, NotificationEvent, SlackMessage, TeamsMessage, WebhookConfig,
};
pub use custom_keybindings::{ContextualBinding, CustomKeybindings, KeybindingContext};
pub use custom_reports::{
    Report, ReportBuilder, ReportFilter, ReportFormat, ReportFrequency, ReportManager,
    ReportRow, ReportTemplate, ReportType, ScheduledReport,
};
pub use dashboard_metrics::{
    BlockedTask, BurndownData, BurnupData, ChartType, CumulativeFlowData, CycleTimeMetric,
    DashboardMetrics, DataPoint, LeadTimeMetric, TeamMemberMetrics, TimePeriod, TimeInStageMetric,
    VelocityMetric, WipMetric,
};
pub use data_portability::{DataExporter, DataFormat, DataImporter};
pub use file_attachments::{Attachment, AttachmentManager, AttachmentType, AttachmentVersion};
pub use diff::{ChunkHeader, DiffHunk, DiffLine, DiffLineType, DiffParser, DiffStats, FileDiff};
pub use errors::{ErrorEntry, ErrorHandler, ErrorSeverity};
pub use fallback_mode::{BoxChars, FallbackMode};
pub use file_ops::{FileOpResult, FileOps};
pub use filtering_search::{
    Filter, FilterCondition, FilterField, FilterManager, FilterOperator, LogicalOperator,
    QuickFilter, SearchParser,
};
pub use history::History;
pub use import_export::{
    BackupManager, BoardData, ExportFormat, ExportResult, Exporter, ImportFormat, ImportResult,
    Importer, Snapshot, TaskData,
};
pub use key_sequences::{KeySequence, KeySequenceManager};
pub use keyboard_recorder::{KeyboardRecorder, RecorderState, RecordedKey};
pub use keyboard_shortcuts::{Shortcut, ShortcutAction, ShortcutCategory, ShortcutRegistry};
pub use keybinds::{KeyBinding, KeyBindings};
pub use mouse::{ClickAction, MouseAction, MouseState, ScrollDirection};
pub use multiple_views::{
    CalendarMode, GroupBy, MindMapOrientation, SortBy, SortOrder, TimelineZoom, ViewConfig,
    ViewManager, ViewSettings, ViewType,
};
pub use project_management::{
    Project, ProjectManager, ProjectStatus, ProjectTemplate, Workspace,
};
pub use rich_task_cards::{
    Assignee, CustomField, CustomFieldType, EffortEstimate, Priority, RichTaskCard,
    RichTaskCardManager, Subtask, Tag,
};
pub use task_dependencies::{
    CriticalPathNode, Dependency, DependencyManager, DependencyType,
};
pub use team_collaboration::{
    Activity, ActivityType, BoardMember, CollaborationManager, Notification, NotificationType,
    Permission, Watcher,
};
pub use terminal_capabilities::{ColorSupport, FeatureLevel, TerminalCapabilities};
pub use text_truncation::{SmartTruncate, TruncationStrategy};
pub use time_tracking::{
    ActiveTimer, Billability, TimeEntry, TimeEntryType, TimeStats, TimeTracker,
};
pub use validation::{
    CompositeValidator, InputValidator, LengthValidator, NotEmptyValidator, RegexValidator,
    ValidationResult, Validator,
};
pub use visual_kanban_board::{
    BoardManager, CardPosition, KanbanBoard, KanbanColumn, Swimlane, SwimlaneGrouping,
};
