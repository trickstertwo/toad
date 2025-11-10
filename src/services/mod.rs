//! Service layer for TOAD
//!
//! This module contains services that provide business logic and I/O operations,
//! keeping widgets and UI components pure and focused on presentation.
//!
//! # Architecture
//!
//! The service layer follows the **Separation of Concerns** principle:
//! - **Widgets** (UI layer): Pure presentation, no I/O, no business logic
//! - **Services** (Service layer): I/O operations, business logic, state management
//! - **Models** (Data layer): Pure data structures
//!
//! # Services
//!
//! - [`FilesystemService`]: File and directory operations
//!
//! # Examples
//!
//! ```
//! use toad::services::FilesystemService;
//!
//! let fs_service = FilesystemService::new();
//! let entries = fs_service.read_dir(".").expect("Failed to read directory");
//! ```

mod filesystem;

pub use filesystem::FilesystemService;
