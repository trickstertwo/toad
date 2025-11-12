//! Filesystem service for file and directory operations
//!
//! Provides a centralized service for all filesystem I/O operations,
//! keeping widgets and UI components free from direct filesystem access.
//!
//! # Design Principles
//!
//! 1. **Separation of Concerns**: UI widgets should not perform I/O
//! 2. **Testability**: Service can be mocked for testing
//! 3. **Centralization**: All filesystem logic in one place
//! 4. **Error Handling**: Consistent error handling across the application
//!
//! # Examples
//!
//! ```
//! use toad::services::FilesystemService;
//! use std::path::Path;
//!
//! let service = FilesystemService::new();
//!
//! // List directory entries
//! let entries = service.read_dir(Path::new(".")).unwrap();
//! for entry in entries {
//!     println!("{}", entry.file_name);
//! }
//! ```

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use tokio::fs as async_fs;

/// Directory entry information
///
/// Simplified representation of a filesystem entry,
/// extracted from `std::fs::DirEntry` for easier testing and usage.
#[derive(Debug, Clone)]
pub struct DirEntry {
    /// Full path to the entry
    pub path: PathBuf,
    /// File name (without path)
    pub file_name: String,
    /// Whether this entry is a directory
    pub is_dir: bool,
    /// Whether this entry is a file
    pub is_file: bool,
}

impl DirEntry {
    /// Create a new directory entry
    pub fn new(path: PathBuf, file_name: String, is_dir: bool, is_file: bool) -> Self {
        Self {
            path,
            file_name,
            is_dir,
            is_file,
        }
    }
}

/// Filesystem service
///
/// Provides filesystem operations with proper error handling and
/// separation from UI concerns.
///
/// # Examples
///
/// ```
/// use toad::services::FilesystemService;
///
/// let service = FilesystemService::new();
/// assert!(service.read_dir(".").is_ok());
/// ```
#[derive(Debug)]
pub struct FilesystemService {
    /// Configuration flags (for future extension)
    _config: (),
}

impl FilesystemService {
    /// Create a new filesystem service
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::services::FilesystemService;
    ///
    /// let service = FilesystemService::new();
    /// ```
    pub fn new() -> Self {
        Self { _config: () }
    }

    /// Read directory entries
    ///
    /// Returns a list of directory entries sorted alphabetically,
    /// with directories listed before files.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the directory to read
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<DirEntry>)` - List of directory entries
    /// * `Err(io::Error)` - If the directory cannot be read
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use toad::services::FilesystemService;
    /// use std::path::Path;
    ///
    /// let service = FilesystemService::new();
    /// let entries = service.read_dir(Path::new(".")).unwrap();
    ///
    /// for entry in entries {
    ///     if entry.is_dir {
    ///         println!("DIR:  {}", entry.file_name);
    ///     } else {
    ///         println!("FILE: {}", entry.file_name);
    ///     }
    /// }
    /// ```
    pub fn read_dir(&self, path: &Path) -> io::Result<Vec<DirEntry>> {
        let mut entries = Vec::new();

        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            let file_name = entry.file_name().to_string_lossy().to_string();
            let is_dir = path.is_dir();
            let is_file = path.is_file();

            entries.push(DirEntry::new(path, file_name, is_dir, is_file));
        }

        // Sort: directories first, then alphabetically
        entries.sort_by(|a, b| match (a.is_dir, b.is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.file_name.cmp(&b.file_name),
        });

        Ok(entries)
    }

    /// Check if a path exists
    ///
    /// # Arguments
    ///
    /// * `path` - Path to check
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::services::FilesystemService;
    /// use std::path::Path;
    ///
    /// let service = FilesystemService::new();
    /// assert!(service.exists(Path::new(".")));
    /// ```
    pub fn exists(&self, path: &Path) -> bool {
        path.exists()
    }

    /// Check if a path is a directory
    ///
    /// # Arguments
    ///
    /// * `path` - Path to check
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::services::FilesystemService;
    /// use std::path::Path;
    ///
    /// let service = FilesystemService::new();
    /// assert!(service.is_dir(Path::new(".")));
    /// ```
    pub fn is_dir(&self, path: &Path) -> bool {
        path.is_dir()
    }

    /// Check if a path is a file
    ///
    /// # Arguments
    ///
    /// * `path` - Path to check
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::services::FilesystemService;
    /// use std::path::Path;
    ///
    /// let service = FilesystemService::new();
    /// assert!(!service.is_file(Path::new(".")));
    /// ```
    pub fn is_file(&self, path: &Path) -> bool {
        path.is_file()
    }

    // ===== Async File Operations =====

    /// Get file metadata asynchronously
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the file
    ///
    /// # Returns
    ///
    /// * `Ok(fs::Metadata)` - File metadata
    /// * `Err(io::Error)` - If metadata cannot be read
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use toad::services::FilesystemService;
    /// # use std::path::Path;
    /// #
    /// # async fn example() -> std::io::Result<()> {
    /// let service = FilesystemService::new();
    /// let metadata = service.read_file_metadata(Path::new("Cargo.toml")).await?;
    /// println!("File size: {} bytes", metadata.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn read_file_metadata(&self, path: &Path) -> io::Result<fs::Metadata> {
        async_fs::metadata(path).await
    }

    /// Read a file to string asynchronously
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the file
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - File contents as UTF-8 string
    /// * `Err(io::Error)` - If file cannot be read or is not valid UTF-8
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use toad::services::FilesystemService;
    /// # use std::path::Path;
    /// #
    /// # async fn example() -> std::io::Result<()> {
    /// let service = FilesystemService::new();
    /// let content = service.read_file_to_string(Path::new("Cargo.toml")).await?;
    /// println!("File content: {}", content);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn read_file_to_string(&self, path: &Path) -> io::Result<String> {
        async_fs::read_to_string(path).await
    }

    /// Read a file as bytes asynchronously
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the file
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<u8>)` - File contents as bytes
    /// * `Err(io::Error)` - If file cannot be read
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use toad::services::FilesystemService;
    /// # use std::path::Path;
    /// #
    /// # async fn example() -> std::io::Result<()> {
    /// let service = FilesystemService::new();
    /// let bytes = service.read_file(Path::new("image.png")).await?;
    /// println!("File size: {} bytes", bytes.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn read_file(&self, path: &Path) -> io::Result<Vec<u8>> {
        async_fs::read(path).await
    }
}

impl Default for FilesystemService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_filesystem_service_creation() {
        let service = FilesystemService::new();
        assert!(service.exists(Path::new(".")));
    }

    #[test]
    fn test_read_dir_current() {
        let service = FilesystemService::new();
        let entries = service.read_dir(Path::new("."));
        assert!(entries.is_ok());

        let entries = entries.unwrap();
        assert!(!entries.is_empty(), "Current directory should have entries");
    }

    #[test]
    fn test_dir_entry_properties() {
        let service = FilesystemService::new();
        let entries = service.read_dir(Path::new(".")).unwrap();

        for entry in &entries {
            // Either file or dir (but not both for normal entries)
            assert!(entry.is_file || entry.is_dir);
            assert!(!entry.file_name.is_empty());
            assert!(entry.path.exists());
        }
    }

    #[test]
    fn test_sorted_directories_first() {
        let service = FilesystemService::new();
        let entries = service.read_dir(Path::new(".")).unwrap();

        let mut saw_file = false;
        for entry in &entries {
            if entry.is_dir {
                assert!(
                    !saw_file,
                    "Directories should come before files in sorted output"
                );
            } else {
                saw_file = true;
            }
        }
    }

    #[test]
    fn test_exists() {
        let service = FilesystemService::new();
        assert!(service.exists(Path::new(".")));
        assert!(service.exists(Path::new("src")));
        assert!(!service.exists(Path::new("nonexistent_dir_12345")));
    }

    #[test]
    fn test_is_dir() {
        let service = FilesystemService::new();
        assert!(service.is_dir(Path::new(".")));
        assert!(service.is_dir(Path::new("src")));
        assert!(!service.is_dir(Path::new("Cargo.toml")));
    }

    #[test]
    fn test_is_file() {
        let service = FilesystemService::new();
        assert!(!service.is_file(Path::new(".")));
        assert!(service.is_file(Path::new("Cargo.toml")));
    }

    #[test]
    fn test_read_dir_nonexistent() {
        let service = FilesystemService::new();
        let result = service.read_dir(Path::new("nonexistent_dir_12345"));
        assert!(result.is_err());
    }

    #[test]
    fn test_default() {
        let service = FilesystemService::default();
        assert!(service.exists(Path::new(".")));
    }
}
