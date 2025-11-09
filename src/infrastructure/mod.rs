//! Infrastructure domain
//!
//! Contains infrastructure and utility modules including async operations,
//! error handling, input handling, and file operations.

pub mod async_ops;
pub mod background_tasks;
pub mod batch_ops;
pub mod clipboard;
pub mod custom_keybindings;
pub mod data_portability;
pub mod diff;
pub mod errors;
pub mod file_ops;
pub mod history;
pub mod key_sequences;
pub mod keybinds;
pub mod mouse;
pub mod validation;

pub use async_ops::{AsyncOperation, AsyncOperationManager, OperationId, OperationStatus};
pub use background_tasks::{BackgroundTask, BackgroundTaskManager, TaskId, TaskStatus};
pub use batch_ops::{
    BatchHandler, BatchManager, BatchOperation, BatchResult, BatchStats, OpResult,
};
pub use clipboard::Clipboard;
pub use custom_keybindings::{ContextualBinding, CustomKeybindings, KeybindingContext};
pub use data_portability::{DataExporter, DataFormat, DataImporter};
pub use diff::{ChunkHeader, DiffHunk, DiffLine, DiffLineType, DiffParser, DiffStats, FileDiff};
pub use errors::{ErrorEntry, ErrorHandler, ErrorSeverity};
pub use file_ops::{FileOpResult, FileOps};
pub use history::History;
pub use key_sequences::{KeySequence, KeySequenceManager};
pub use keybinds::{KeyBinding, KeyBindings};
pub use mouse::{ClickAction, MouseAction, MouseState, ScrollDirection};
pub use validation::{
    CompositeValidator, InputValidator, LengthValidator, NotEmptyValidator, RegexValidator,
    ValidationResult, Validator,
};
