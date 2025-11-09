/// SWE-bench dataset management
///
/// This module handles downloading, caching, and managing SWE-bench datasets
/// for evaluation.
use super::Task;
use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

/// SWE-bench dataset sources
#[derive(Debug, Clone)]
pub enum DatasetSource {
    /// SWE-bench Verified (500 tasks, human-verified)
    Verified,
    /// SWE-bench Lite (300 tasks, high-quality subset)
    Lite,
    /// SWE-bench Full (2,294 tasks, complete dataset)
    Full,
    /// Local file
    Local(PathBuf),
}

impl DatasetSource {
    /// Get the HuggingFace dataset URL
    pub fn huggingface_url(&self) -> Option<String> {
        match self {
            DatasetSource::Verified => Some(
                "https://huggingface.co/datasets/princeton-nlp/SWE-bench_Verified/resolve/main/data/test.jsonl".to_string()
            ),
            DatasetSource::Lite => Some(
                "https://huggingface.co/datasets/princeton-nlp/SWE-bench_Lite/resolve/main/data/test.jsonl".to_string()
            ),
            DatasetSource::Full => Some(
                "https://huggingface.co/datasets/princeton-nlp/SWE-bench/resolve/main/data/test.jsonl".to_string()
            ),
            DatasetSource::Local(_) => None,
        }
    }

    /// Get the local cache filename
    pub fn cache_filename(&self) -> String {
        match self {
            DatasetSource::Verified => "swe_bench_verified.jsonl".to_string(),
            DatasetSource::Lite => "swe_bench_lite.jsonl".to_string(),
            DatasetSource::Full => "swe_bench_full.jsonl".to_string(),
            DatasetSource::Local(path) => path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
        }
    }
}

/// Manages SWE-bench datasets with caching
pub struct DatasetManager {
    cache_dir: PathBuf,
}

impl DatasetManager {
    /// Create a new dataset manager
    pub fn new(cache_dir: PathBuf) -> Self {
        Self { cache_dir }
    }

    /// Get the default cache directory (~/.toad/datasets)
    pub fn default_cache_dir() -> PathBuf {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        PathBuf::from(home).join(".toad").join("datasets")
    }

    /// Initialize the cache directory
    pub fn init(&self) -> Result<()> {
        fs::create_dir_all(&self.cache_dir).context("Failed to create cache directory")?;
        Ok(())
    }

    /// Get the cache path for a dataset
    pub fn cache_path(&self, source: &DatasetSource) -> PathBuf {
        match source {
            DatasetSource::Local(path) => path.clone(),
            _ => self.cache_dir.join(source.cache_filename()),
        }
    }

    /// Check if dataset is cached
    pub fn is_cached(&self, source: &DatasetSource) -> bool {
        self.cache_path(source).exists()
    }

    /// Download a dataset (stub - requires HTTP client)
    pub fn download(&self, source: &DatasetSource) -> Result<PathBuf> {
        match source {
            DatasetSource::Local(path) => {
                if !path.exists() {
                    anyhow::bail!("Local dataset file not found: {:?}", path);
                }
                Ok(path.clone())
            }
            _ => {
                let cache_path = self.cache_path(source);

                // TODO: Implement actual HTTP download
                // For now, return an error with instructions
                anyhow::bail!(
                    "Automatic dataset download not yet implemented.\n\
                     \n\
                     Please manually download the dataset:\n\
                     1. Download from: {}\n\
                     2. Save to: {:?}\n\
                     \n\
                     Or use --dataset flag to specify a local file.",
                    source.huggingface_url().unwrap_or_default(),
                    cache_path
                )
            }
        }
    }

    /// Get or download a dataset
    pub fn get_or_download(&self, source: &DatasetSource) -> Result<PathBuf> {
        self.init()?;

        if self.is_cached(source) {
            Ok(self.cache_path(source))
        } else {
            self.download(source)
        }
    }

    /// Load a sample from a dataset source
    pub fn load_sample(&self, source: DatasetSource, count: usize) -> Result<Vec<Task>> {
        use super::task_loader::TaskLoader;

        let path = self.get_or_download(&source)?;
        let loader = TaskLoader::new(path);
        loader.load_sample(count)
    }

    /// Load stratified sample
    pub fn load_stratified(
        &self,
        source: DatasetSource,
        simple: usize,
        medium: usize,
        hard: usize,
    ) -> Result<Vec<Task>> {
        use super::task_loader::TaskLoader;

        let path = self.get_or_download(&source)?;
        let loader = TaskLoader::new(path);
        loader.load_stratified(simple, medium, hard)
    }

    /// Get dataset info
    pub fn dataset_info(&self, source: &DatasetSource) -> DatasetInfo {
        DatasetInfo {
            source: source.clone(),
            cached: self.is_cached(source),
            cache_path: self.cache_path(source),
            size: self
                .cache_path(source)
                .metadata()
                .ok()
                .map(|m| m.len())
                .unwrap_or(0),
        }
    }
}

impl Default for DatasetManager {
    fn default() -> Self {
        Self::new(Self::default_cache_dir())
    }
}

/// Information about a dataset
#[derive(Debug, Clone)]
pub struct DatasetInfo {
    pub source: DatasetSource,
    pub cached: bool,
    pub cache_path: PathBuf,
    pub size: u64,
}

impl DatasetInfo {
    pub fn print(&self) {
        println!("Dataset: {:?}", self.source);
        println!("Cached: {}", if self.cached { "Yes" } else { "No" });
        println!("Path: {:?}", self.cache_path);
        if self.cached {
            println!("Size: {} bytes", self.size);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_dataset_source_urls() {
        assert!(DatasetSource::Verified.huggingface_url().is_some());
        assert!(DatasetSource::Lite.huggingface_url().is_some());
        assert!(DatasetSource::Full.huggingface_url().is_some());
        assert!(DatasetSource::Local(PathBuf::from("/tmp/test.json"))
            .huggingface_url()
            .is_none());
    }

    #[test]
    fn test_cache_filenames() {
        assert_eq!(
            DatasetSource::Verified.cache_filename(),
            "swe_bench_verified.jsonl"
        );
        assert_eq!(DatasetSource::Lite.cache_filename(), "swe_bench_lite.jsonl");
        assert_eq!(DatasetSource::Full.cache_filename(), "swe_bench_full.jsonl");
    }

    #[test]
    fn test_dataset_manager_init() {
        let temp_dir = TempDir::new().unwrap();
        let manager = DatasetManager::new(temp_dir.path().to_path_buf());

        manager.init().unwrap();
        assert!(temp_dir.path().exists());
    }

    #[test]
    fn test_is_cached() {
        let temp_dir = TempDir::new().unwrap();
        let manager = DatasetManager::new(temp_dir.path().to_path_buf());
        manager.init().unwrap();

        let source = DatasetSource::Lite;
        assert!(!manager.is_cached(&source));

        // Create a fake cache file
        let cache_path = manager.cache_path(&source);
        std::fs::write(&cache_path, "test").unwrap();

        assert!(manager.is_cached(&source));
    }

    #[test]
    fn test_local_source() {
        use std::io::Write;

        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test.jsonl");

        let mut file = std::fs::File::create(&test_file).unwrap();
        write!(file, "{{\"instance_id\": \"test-1\"}}").unwrap();

        let source = DatasetSource::Local(test_file.clone());
        let manager = DatasetManager::new(temp_dir.path().join("cache"));

        assert_eq!(manager.cache_path(&source), test_file);
        assert!(manager.is_cached(&source));
    }

    #[test]
    fn test_dataset_info() {
        let temp_dir = TempDir::new().unwrap();
        let manager = DatasetManager::new(temp_dir.path().to_path_buf());
        manager.init().unwrap();

        let source = DatasetSource::Lite;
        let info = manager.dataset_info(&source);

        assert!(!info.cached);
        assert_eq!(info.size, 0);
    }
}
