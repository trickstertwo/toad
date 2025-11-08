/// Recent files tracking (MRU - Most Recently Used)
///
/// Maintains a list of recently accessed files with timestamps and persistence
///
/// # Examples
///
/// ```
/// use toad::recent_files::RecentFiles;
///
/// let mut recent = RecentFiles::new(10);
/// recent.add("/path/to/file.rs".to_string());
/// assert_eq!(recent.len(), 1);
/// ```

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use anyhow::{Context, Result};

/// A recent file entry with metadata
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecentFile {
    /// File path
    pub path: PathBuf,
    /// Last access timestamp (Unix timestamp)
    pub last_accessed: u64,
    /// Access count
    pub access_count: usize,
}

impl RecentFile {
    /// Create a new recent file entry
    pub fn new(path: PathBuf) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            path,
            last_accessed: now,
            access_count: 1,
        }
    }

    /// Update access time and increment count
    pub fn touch(&mut self) {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        self.last_accessed = now;
        self.access_count += 1;
    }

    /// Get file name
    pub fn file_name(&self) -> Option<&str> {
        self.path.file_name().and_then(|n| n.to_str())
    }

    /// Get parent directory
    pub fn parent(&self) -> Option<&Path> {
        self.path.parent()
    }

    /// Check if file still exists
    pub fn exists(&self) -> bool {
        self.path.exists()
    }
}

/// Recent files manager (MRU list)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentFiles {
    /// Maximum number of files to track
    max_size: usize,
    /// Recent files (sorted by last access time, newest first)
    files: Vec<RecentFile>,
}

impl RecentFiles {
    /// Create a new recent files manager
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::recent_files::RecentFiles;
    ///
    /// let recent = RecentFiles::new(20);
    /// assert_eq!(recent.max_size(), 20);
    /// ```
    pub fn new(max_size: usize) -> Self {
        Self {
            max_size,
            files: Vec::new(),
        }
    }

    /// Get the maximum size
    pub fn max_size(&self) -> usize {
        self.max_size
    }

    /// Add or update a file in the recent list
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::recent_files::RecentFiles;
    ///
    /// let mut recent = RecentFiles::new(5);
    /// recent.add("/path/to/file.rs".to_string());
    /// assert_eq!(recent.len(), 1);
    /// ```
    pub fn add(&mut self, path: String) {
        let path_buf = PathBuf::from(path);

        // Check if file already exists
        if let Some(pos) = self.files.iter().position(|f| f.path == path_buf) {
            // Update existing entry
            let mut file = self.files.remove(pos);
            file.touch();
            self.files.insert(0, file);
        } else {
            // Add new entry
            let file = RecentFile::new(path_buf);
            self.files.insert(0, file);

            // Enforce max size
            if self.files.len() > self.max_size {
                self.files.truncate(self.max_size);
            }
        }
    }

    /// Get recent file at index (0 = most recent)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::recent_files::RecentFiles;
    ///
    /// let mut recent = RecentFiles::new(5);
    /// recent.add("/path/to/file1.rs".to_string());
    /// recent.add("/path/to/file2.rs".to_string());
    ///
    /// let file = recent.get(0);
    /// assert!(file.is_some());
    /// ```
    pub fn get(&self, index: usize) -> Option<&RecentFile> {
        self.files.get(index)
    }

    /// Get all recent files
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::recent_files::RecentFiles;
    ///
    /// let mut recent = RecentFiles::new(5);
    /// recent.add("/path/to/file.rs".to_string());
    ///
    /// let files = recent.files();
    /// assert_eq!(files.len(), 1);
    /// ```
    pub fn files(&self) -> &[RecentFile] {
        &self.files
    }

    /// Get number of tracked files
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::recent_files::RecentFiles;
    ///
    /// let mut recent = RecentFiles::new(5);
    /// assert_eq!(recent.len(), 0);
    ///
    /// recent.add("/path/to/file.rs".to_string());
    /// assert_eq!(recent.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.files.len()
    }

    /// Check if list is empty
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::recent_files::RecentFiles;
    ///
    /// let recent = RecentFiles::new(5);
    /// assert!(recent.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.files.is_empty()
    }

    /// Remove a file from the list
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::recent_files::RecentFiles;
    ///
    /// let mut recent = RecentFiles::new(5);
    /// recent.add("/path/to/file.rs".to_string());
    /// assert_eq!(recent.len(), 1);
    ///
    /// recent.remove("/path/to/file.rs");
    /// assert_eq!(recent.len(), 0);
    /// ```
    pub fn remove(&mut self, path: &str) -> bool {
        let path_buf = PathBuf::from(path);
        if let Some(pos) = self.files.iter().position(|f| f.path == path_buf) {
            self.files.remove(pos);
            true
        } else {
            false
        }
    }

    /// Clear all recent files
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::recent_files::RecentFiles;
    ///
    /// let mut recent = RecentFiles::new(5);
    /// recent.add("/path/to/file.rs".to_string());
    /// recent.clear();
    /// assert_eq!(recent.len(), 0);
    /// ```
    pub fn clear(&mut self) {
        self.files.clear();
    }

    /// Remove files that no longer exist on disk
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::recent_files::RecentFiles;
    ///
    /// let mut recent = RecentFiles::new(5);
    /// recent.add("/nonexistent/file.rs".to_string());
    /// recent.cleanup();
    /// assert_eq!(recent.len(), 0);
    /// ```
    pub fn cleanup(&mut self) {
        self.files.retain(|f| f.exists());
    }

    /// Search for files by name or path
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::recent_files::RecentFiles;
    ///
    /// let mut recent = RecentFiles::new(5);
    /// recent.add("/path/to/test.rs".to_string());
    /// recent.add("/path/to/main.rs".to_string());
    ///
    /// let matches = recent.search("test");
    /// assert_eq!(matches.len(), 1);
    /// ```
    pub fn search(&self, query: &str) -> Vec<&RecentFile> {
        self.files
            .iter()
            .filter(|f| {
                f.path.to_string_lossy().contains(query)
                    || f.file_name()
                        .map(|n| n.contains(query))
                        .unwrap_or(false)
            })
            .collect()
    }

    /// Get files sorted by access count (most accessed first)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::recent_files::RecentFiles;
    ///
    /// let mut recent = RecentFiles::new(5);
    /// recent.add("/path/to/file1.rs".to_string());
    /// recent.add("/path/to/file2.rs".to_string());
    /// recent.add("/path/to/file1.rs".to_string());
    ///
    /// let by_freq = recent.by_frequency();
    /// assert_eq!(by_freq[0].access_count, 2);
    /// ```
    pub fn by_frequency(&self) -> Vec<RecentFile> {
        let mut sorted = self.files.clone();
        sorted.sort_by(|a, b| b.access_count.cmp(&a.access_count));
        sorted
    }

    /// Save recent files to file
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use toad::recent_files::RecentFiles;
    /// use std::path::Path;
    ///
    /// let mut recent = RecentFiles::new(5);
    /// recent.add("/path/to/file.rs".to_string());
    /// recent.save_to_file(Path::new("recent.json")).unwrap();
    /// ```
    pub fn save_to_file(&self, path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// Load recent files from file
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use toad::recent_files::RecentFiles;
    /// use std::path::Path;
    ///
    /// let recent = RecentFiles::load_from_file(Path::new("recent.json")).unwrap();
    /// ```
    pub fn load_from_file(path: &Path) -> Result<Self> {
        let contents = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read recent files: {}", path.display()))?;

        let recent = serde_json::from_str(&contents)
            .with_context(|| format!("Failed to parse recent files: {}", path.display()))?;

        Ok(recent)
    }

    /// Get default recent files path
    pub fn default_path() -> PathBuf {
        let config_dir = if cfg!(target_os = "windows") {
            std::env::var("APPDATA")
                .map(PathBuf::from)
                .unwrap_or_else(|_| PathBuf::from("."))
        } else {
            std::env::var("XDG_CONFIG_HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|_| {
                    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
                    PathBuf::from(home).join(".config")
                })
        };

        config_dir.join("toad").join("recent_files.json")
    }

    /// Load from default path or create new
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::recent_files::RecentFiles;
    ///
    /// let recent = RecentFiles::load_or_new(20);
    /// assert_eq!(recent.max_size(), 20);
    /// ```
    pub fn load_or_new(max_size: usize) -> Self {
        let path = Self::default_path();

        if path.exists() {
            Self::load_from_file(&path).unwrap_or_else(|_| Self::new(max_size))
        } else {
            Self::new(max_size)
        }
    }
}

impl Default for RecentFiles {
    fn default() -> Self {
        Self::new(20)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recent_file_creation() {
        let file = RecentFile::new(PathBuf::from("/path/to/file.rs"));
        assert_eq!(file.path, PathBuf::from("/path/to/file.rs"));
        assert_eq!(file.access_count, 1);
        assert!(file.last_accessed > 0);
    }

    #[test]
    fn test_recent_file_touch() {
        let mut file = RecentFile::new(PathBuf::from("/path/to/file.rs"));
        let old_time = file.last_accessed;

        std::thread::sleep(std::time::Duration::from_millis(10));
        file.touch();

        assert!(file.last_accessed >= old_time);
        assert_eq!(file.access_count, 2);
    }

    #[test]
    fn test_recent_file_name() {
        let file = RecentFile::new(PathBuf::from("/path/to/file.rs"));
        assert_eq!(file.file_name(), Some("file.rs"));
    }

    #[test]
    fn test_recent_file_parent() {
        let file = RecentFile::new(PathBuf::from("/path/to/file.rs"));
        assert_eq!(file.parent(), Some(Path::new("/path/to")));
    }

    #[test]
    fn test_recent_files_creation() {
        let recent = RecentFiles::new(10);
        assert_eq!(recent.max_size(), 10);
        assert_eq!(recent.len(), 0);
        assert!(recent.is_empty());
    }

    #[test]
    fn test_recent_files_add() {
        let mut recent = RecentFiles::new(5);
        recent.add("/path/to/file1.rs".to_string());
        recent.add("/path/to/file2.rs".to_string());

        assert_eq!(recent.len(), 2);
        assert_eq!(recent.get(0).unwrap().path, PathBuf::from("/path/to/file2.rs"));
        assert_eq!(recent.get(1).unwrap().path, PathBuf::from("/path/to/file1.rs"));
    }

    #[test]
    fn test_recent_files_update() {
        let mut recent = RecentFiles::new(5);
        recent.add("/path/to/file1.rs".to_string());
        recent.add("/path/to/file2.rs".to_string());
        recent.add("/path/to/file1.rs".to_string());

        assert_eq!(recent.len(), 2);
        assert_eq!(recent.get(0).unwrap().path, PathBuf::from("/path/to/file1.rs"));
        assert_eq!(recent.get(0).unwrap().access_count, 2);
    }

    #[test]
    fn test_recent_files_max_size() {
        let mut recent = RecentFiles::new(3);
        recent.add("/path/to/file1.rs".to_string());
        recent.add("/path/to/file2.rs".to_string());
        recent.add("/path/to/file3.rs".to_string());
        recent.add("/path/to/file4.rs".to_string());

        assert_eq!(recent.len(), 3);
        assert_eq!(recent.get(0).unwrap().path, PathBuf::from("/path/to/file4.rs"));
    }

    #[test]
    fn test_recent_files_remove() {
        let mut recent = RecentFiles::new(5);
        recent.add("/path/to/file1.rs".to_string());
        recent.add("/path/to/file2.rs".to_string());

        assert!(recent.remove("/path/to/file1.rs"));
        assert_eq!(recent.len(), 1);
        assert!(!recent.remove("/path/to/nonexistent.rs"));
    }

    #[test]
    fn test_recent_files_clear() {
        let mut recent = RecentFiles::new(5);
        recent.add("/path/to/file1.rs".to_string());
        recent.add("/path/to/file2.rs".to_string());

        recent.clear();
        assert_eq!(recent.len(), 0);
        assert!(recent.is_empty());
    }

    #[test]
    fn test_recent_files_search() {
        let mut recent = RecentFiles::new(5);
        recent.add("/path/to/test.rs".to_string());
        recent.add("/path/to/main.rs".to_string());
        recent.add("/other/test_utils.rs".to_string());

        let matches = recent.search("test");
        assert_eq!(matches.len(), 2);

        let matches = recent.search("main");
        assert_eq!(matches.len(), 1);
    }

    #[test]
    fn test_recent_files_by_frequency() {
        let mut recent = RecentFiles::new(5);
        recent.add("/path/to/file1.rs".to_string());
        recent.add("/path/to/file2.rs".to_string());
        recent.add("/path/to/file1.rs".to_string());
        recent.add("/path/to/file1.rs".to_string());
        recent.add("/path/to/file3.rs".to_string());

        let by_freq = recent.by_frequency();
        assert_eq!(by_freq[0].path, PathBuf::from("/path/to/file1.rs"));
        assert_eq!(by_freq[0].access_count, 3);
    }

    #[test]
    fn test_recent_files_cleanup() {
        let mut recent = RecentFiles::new(5);
        recent.add("/nonexistent/file.rs".to_string());

        recent.cleanup();
        assert_eq!(recent.len(), 0);
    }

    #[test]
    fn test_recent_files_default() {
        let recent = RecentFiles::default();
        assert_eq!(recent.max_size(), 20);
    }

    #[test]
    fn test_default_path() {
        let path = RecentFiles::default_path();
        let path_str = path.to_string_lossy();

        assert!(path_str.contains("toad"));
        assert!(path_str.contains("recent_files.json"));
    }
}
