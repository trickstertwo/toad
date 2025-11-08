/// File operations (copy, move, delete, rename)
///
/// Provides safe file system operations with error handling
///
/// # Examples
///
/// ```no_run
/// use toad::file_ops::{FileOps, FileOpResult};
///
/// let ops = FileOps::new();
/// let result = ops.copy("src.txt", "dest.txt");
/// assert!(result.is_ok());
/// ```
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// File operation result
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileOpResult {
    /// Operation type
    pub operation: String,
    /// Source path
    pub source: PathBuf,
    /// Destination path (if applicable)
    pub destination: Option<PathBuf>,
    /// Success status
    pub success: bool,
    /// Error message (if failed)
    pub error: Option<String>,
}

impl FileOpResult {
    /// Create a successful result
    pub fn success(operation: String, source: PathBuf, destination: Option<PathBuf>) -> Self {
        Self {
            operation,
            source,
            destination,
            success: true,
            error: None,
        }
    }

    /// Create a failed result
    pub fn failure(
        operation: String,
        source: PathBuf,
        destination: Option<PathBuf>,
        error: String,
    ) -> Self {
        Self {
            operation,
            source,
            destination,
            success: false,
            error: Some(error),
        }
    }
}

/// File operations manager
pub struct FileOps {
    /// Whether to allow overwriting existing files
    allow_overwrite: bool,
    /// Whether to create parent directories
    create_parents: bool,
}

impl FileOps {
    /// Create a new file operations manager
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::file_ops::FileOps;
    ///
    /// let ops = FileOps::new();
    /// ```
    pub fn new() -> Self {
        Self {
            allow_overwrite: false,
            create_parents: true,
        }
    }

    /// Set whether to allow overwriting existing files
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::file_ops::FileOps;
    ///
    /// let ops = FileOps::new().with_overwrite(true);
    /// ```
    pub fn with_overwrite(mut self, allow: bool) -> Self {
        self.allow_overwrite = allow;
        self
    }

    /// Set whether to create parent directories
    pub fn with_create_parents(mut self, create: bool) -> Self {
        self.create_parents = create;
        self
    }

    /// Copy a file
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use toad::file_ops::FileOps;
    ///
    /// let ops = FileOps::new();
    /// ops.copy("source.txt", "dest.txt").unwrap();
    /// ```
    pub fn copy<P: AsRef<Path>, Q: AsRef<Path>>(&self, src: P, dest: Q) -> Result<FileOpResult> {
        let src_path = src.as_ref();
        let dest_path = dest.as_ref();

        // Check source exists
        if !src_path.exists() {
            return Ok(FileOpResult::failure(
                "copy".to_string(),
                src_path.to_path_buf(),
                Some(dest_path.to_path_buf()),
                format!("Source file does not exist: {}", src_path.display()),
            ));
        }

        // Check destination
        if dest_path.exists() && !self.allow_overwrite {
            return Ok(FileOpResult::failure(
                "copy".to_string(),
                src_path.to_path_buf(),
                Some(dest_path.to_path_buf()),
                format!("Destination already exists: {}", dest_path.display()),
            ));
        }

        // Create parent directory if needed
        if self.create_parents {
            if let Some(parent) = dest_path.parent() {
                fs::create_dir_all(parent)
                    .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
            }
        }

        // Perform copy
        fs::copy(src_path, dest_path)
            .with_context(|| {
                format!(
                    "Failed to copy {} to {}",
                    src_path.display(),
                    dest_path.display()
                )
            })
            .map(|_| {
                FileOpResult::success(
                    "copy".to_string(),
                    src_path.to_path_buf(),
                    Some(dest_path.to_path_buf()),
                )
            })
            .or_else(|e| {
                Ok(FileOpResult::failure(
                    "copy".to_string(),
                    src_path.to_path_buf(),
                    Some(dest_path.to_path_buf()),
                    e.to_string(),
                ))
            })
    }

    /// Move/rename a file
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use toad::file_ops::FileOps;
    ///
    /// let ops = FileOps::new();
    /// ops.move_file("old.txt", "new.txt").unwrap();
    /// ```
    pub fn move_file<P: AsRef<Path>, Q: AsRef<Path>>(
        &self,
        src: P,
        dest: Q,
    ) -> Result<FileOpResult> {
        let src_path = src.as_ref();
        let dest_path = dest.as_ref();

        // Check source exists
        if !src_path.exists() {
            return Ok(FileOpResult::failure(
                "move".to_string(),
                src_path.to_path_buf(),
                Some(dest_path.to_path_buf()),
                format!("Source file does not exist: {}", src_path.display()),
            ));
        }

        // Check destination
        if dest_path.exists() && !self.allow_overwrite {
            return Ok(FileOpResult::failure(
                "move".to_string(),
                src_path.to_path_buf(),
                Some(dest_path.to_path_buf()),
                format!("Destination already exists: {}", dest_path.display()),
            ));
        }

        // Create parent directory if needed
        if self.create_parents {
            if let Some(parent) = dest_path.parent() {
                fs::create_dir_all(parent)
                    .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
            }
        }

        // Perform move
        fs::rename(src_path, dest_path)
            .with_context(|| {
                format!(
                    "Failed to move {} to {}",
                    src_path.display(),
                    dest_path.display()
                )
            })
            .map(|_| {
                FileOpResult::success(
                    "move".to_string(),
                    src_path.to_path_buf(),
                    Some(dest_path.to_path_buf()),
                )
            })
            .or_else(|e| {
                Ok(FileOpResult::failure(
                    "move".to_string(),
                    src_path.to_path_buf(),
                    Some(dest_path.to_path_buf()),
                    e.to_string(),
                ))
            })
    }

    /// Rename a file (alias for move_file)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use toad::file_ops::FileOps;
    ///
    /// let ops = FileOps::new();
    /// ops.rename("old.txt", "new.txt").unwrap();
    /// ```
    pub fn rename<P: AsRef<Path>, Q: AsRef<Path>>(&self, src: P, dest: Q) -> Result<FileOpResult> {
        self.move_file(src, dest)
    }

    /// Delete a file
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use toad::file_ops::FileOps;
    ///
    /// let ops = FileOps::new();
    /// ops.delete("file.txt").unwrap();
    /// ```
    pub fn delete<P: AsRef<Path>>(&self, path: P) -> Result<FileOpResult> {
        let path = path.as_ref();

        // Check if exists
        if !path.exists() {
            return Ok(FileOpResult::failure(
                "delete".to_string(),
                path.to_path_buf(),
                None,
                format!("File does not exist: {}", path.display()),
            ));
        }

        // Delete file or directory
        if path.is_dir() {
            fs::remove_dir_all(path)
                .with_context(|| format!("Failed to delete directory: {}", path.display()))
                .map(|_| FileOpResult::success("delete".to_string(), path.to_path_buf(), None))
                .or_else(|e| {
                    Ok(FileOpResult::failure(
                        "delete".to_string(),
                        path.to_path_buf(),
                        None,
                        e.to_string(),
                    ))
                })
        } else {
            fs::remove_file(path)
                .with_context(|| format!("Failed to delete file: {}", path.display()))
                .map(|_| FileOpResult::success("delete".to_string(), path.to_path_buf(), None))
                .or_else(|e| {
                    Ok(FileOpResult::failure(
                        "delete".to_string(),
                        path.to_path_buf(),
                        None,
                        e.to_string(),
                    ))
                })
        }
    }

    /// Create a directory
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use toad::file_ops::FileOps;
    ///
    /// let ops = FileOps::new();
    /// ops.create_dir("new_dir").unwrap();
    /// ```
    pub fn create_dir<P: AsRef<Path>>(&self, path: P) -> Result<FileOpResult> {
        let path = path.as_ref();

        // Check if already exists
        if path.exists() {
            return Ok(FileOpResult::failure(
                "create_dir".to_string(),
                path.to_path_buf(),
                None,
                format!("Directory already exists: {}", path.display()),
            ));
        }

        // Create directory (with parents if configured)
        let result = if self.create_parents {
            fs::create_dir_all(path)
        } else {
            fs::create_dir(path)
        };

        result
            .with_context(|| format!("Failed to create directory: {}", path.display()))
            .map(|_| FileOpResult::success("create_dir".to_string(), path.to_path_buf(), None))
            .or_else(|e| {
                Ok(FileOpResult::failure(
                    "create_dir".to_string(),
                    path.to_path_buf(),
                    None,
                    e.to_string(),
                ))
            })
    }

    /// Copy a directory recursively
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use toad::file_ops::FileOps;
    ///
    /// let ops = FileOps::new();
    /// ops.copy_dir("src_dir", "dest_dir").unwrap();
    /// ```
    pub fn copy_dir<P: AsRef<Path>, Q: AsRef<Path>>(
        &self,
        src: P,
        dest: Q,
    ) -> Result<FileOpResult> {
        let src_path = src.as_ref();
        let dest_path = dest.as_ref();

        // Check source exists and is directory
        if !src_path.exists() || !src_path.is_dir() {
            return Ok(FileOpResult::failure(
                "copy_dir".to_string(),
                src_path.to_path_buf(),
                Some(dest_path.to_path_buf()),
                format!("Source is not a directory: {}", src_path.display()),
            ));
        }

        // Create destination directory
        fs::create_dir_all(dest_path)
            .with_context(|| format!("Failed to create directory: {}", dest_path.display()))?;

        // Copy contents recursively
        for entry in fs::read_dir(src_path)
            .with_context(|| format!("Failed to read directory: {}", src_path.display()))?
        {
            let entry = entry?;
            let ty = entry.file_type()?;
            let src_file = entry.path();
            let dest_file = dest_path.join(entry.file_name());

            if ty.is_dir() {
                self.copy_dir(&src_file, &dest_file)?;
            } else {
                self.copy(&src_file, &dest_file)?;
            }
        }

        Ok(FileOpResult::success(
            "copy_dir".to_string(),
            src_path.to_path_buf(),
            Some(dest_path.to_path_buf()),
        ))
    }
}

impl Default for FileOps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_file_op_result_success() {
        let result = FileOpResult::success(
            "copy".to_string(),
            PathBuf::from("src"),
            Some(PathBuf::from("dest")),
        );

        assert!(result.success);
        assert!(result.error.is_none());
        assert_eq!(result.operation, "copy");
    }

    #[test]
    fn test_file_op_result_failure() {
        let result = FileOpResult::failure(
            "copy".to_string(),
            PathBuf::from("src"),
            Some(PathBuf::from("dest")),
            "error message".to_string(),
        );

        assert!(!result.success);
        assert!(result.error.is_some());
        assert_eq!(result.error.unwrap(), "error message");
    }

    #[test]
    fn test_file_ops_creation() {
        let ops = FileOps::new();
        assert!(!ops.allow_overwrite);
        assert!(ops.create_parents);
    }

    #[test]
    fn test_file_ops_with_overwrite() {
        let ops = FileOps::new().with_overwrite(true);
        assert!(ops.allow_overwrite);
    }

    #[test]
    fn test_file_ops_copy() {
        let dir = tempdir().unwrap();
        let src_path = dir.path().join("source.txt");
        let dest_path = dir.path().join("dest.txt");

        // Create source file
        let mut file = File::create(&src_path).unwrap();
        file.write_all(b"test content").unwrap();

        let ops = FileOps::new();
        let result = ops.copy(&src_path, &dest_path).unwrap();

        assert!(result.success);
        assert!(dest_path.exists());

        let content = fs::read_to_string(&dest_path).unwrap();
        assert_eq!(content, "test content");
    }

    #[test]
    fn test_file_ops_copy_nonexistent() {
        let dir = tempdir().unwrap();
        let src_path = dir.path().join("nonexistent.txt");
        let dest_path = dir.path().join("dest.txt");

        let ops = FileOps::new();
        let result = ops.copy(&src_path, &dest_path).unwrap();

        assert!(!result.success);
        assert!(result.error.is_some());
    }

    #[test]
    fn test_file_ops_copy_no_overwrite() {
        let dir = tempdir().unwrap();
        let src_path = dir.path().join("source.txt");
        let dest_path = dir.path().join("dest.txt");

        // Create both files
        File::create(&src_path).unwrap();
        File::create(&dest_path).unwrap();

        let ops = FileOps::new();
        let result = ops.copy(&src_path, &dest_path).unwrap();

        assert!(!result.success);
    }

    #[test]
    fn test_file_ops_copy_with_overwrite() {
        let dir = tempdir().unwrap();
        let src_path = dir.path().join("source.txt");
        let dest_path = dir.path().join("dest.txt");

        // Create both files
        let mut file = File::create(&src_path).unwrap();
        file.write_all(b"new content").unwrap();
        File::create(&dest_path).unwrap();

        let ops = FileOps::new().with_overwrite(true);
        let result = ops.copy(&src_path, &dest_path).unwrap();

        assert!(result.success);

        let content = fs::read_to_string(&dest_path).unwrap();
        assert_eq!(content, "new content");
    }

    #[test]
    fn test_file_ops_move() {
        let dir = tempdir().unwrap();
        let src_path = dir.path().join("source.txt");
        let dest_path = dir.path().join("dest.txt");

        // Create source file
        let mut file = File::create(&src_path).unwrap();
        file.write_all(b"test content").unwrap();

        let ops = FileOps::new();
        let result = ops.move_file(&src_path, &dest_path).unwrap();

        assert!(result.success);
        assert!(!src_path.exists());
        assert!(dest_path.exists());
    }

    #[test]
    fn test_file_ops_rename() {
        let dir = tempdir().unwrap();
        let old_path = dir.path().join("old.txt");
        let new_path = dir.path().join("new.txt");

        File::create(&old_path).unwrap();

        let ops = FileOps::new();
        let result = ops.rename(&old_path, &new_path).unwrap();

        assert!(result.success);
        assert!(!old_path.exists());
        assert!(new_path.exists());
    }

    #[test]
    fn test_file_ops_delete_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("file.txt");

        File::create(&file_path).unwrap();

        let ops = FileOps::new();
        let result = ops.delete(&file_path).unwrap();

        assert!(result.success);
        assert!(!file_path.exists());
    }

    #[test]
    fn test_file_ops_delete_dir() {
        let dir = tempdir().unwrap();
        let dir_path = dir.path().join("subdir");

        fs::create_dir(&dir_path).unwrap();

        let ops = FileOps::new();
        let result = ops.delete(&dir_path).unwrap();

        assert!(result.success);
        assert!(!dir_path.exists());
    }

    #[test]
    fn test_file_ops_delete_nonexistent() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("nonexistent.txt");

        let ops = FileOps::new();
        let result = ops.delete(&file_path).unwrap();

        assert!(!result.success);
    }

    #[test]
    fn test_file_ops_create_dir() {
        let dir = tempdir().unwrap();
        let new_dir = dir.path().join("new_directory");

        let ops = FileOps::new();
        let result = ops.create_dir(&new_dir).unwrap();

        assert!(result.success);
        assert!(new_dir.exists());
        assert!(new_dir.is_dir());
    }

    #[test]
    fn test_file_ops_create_dir_exists() {
        let dir = tempdir().unwrap();

        let ops = FileOps::new();
        let result = ops.create_dir(dir.path()).unwrap();

        assert!(!result.success);
    }

    #[test]
    fn test_file_ops_copy_dir() {
        let dir = tempdir().unwrap();
        let src_dir = dir.path().join("src");
        let dest_dir = dir.path().join("dest");

        // Create source directory with files
        fs::create_dir(&src_dir).unwrap();
        File::create(src_dir.join("file1.txt")).unwrap();
        File::create(src_dir.join("file2.txt")).unwrap();

        let ops = FileOps::new();
        let result = ops.copy_dir(&src_dir, &dest_dir).unwrap();

        assert!(result.success);
        assert!(dest_dir.exists());
        assert!(dest_dir.join("file1.txt").exists());
        assert!(dest_dir.join("file2.txt").exists());
    }

    #[test]
    fn test_file_ops_default() {
        let ops = FileOps::default();
        assert!(!ops.allow_overwrite);
    }
}
