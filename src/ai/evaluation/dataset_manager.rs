/// SWE-bench dataset management
///
/// This module handles downloading, caching, and managing SWE-bench datasets
/// for evaluation.
use super::Task;
use anyhow::{Context, Result};
use parquet::file::reader::{FileReader, SerializedFileReader};
use std::fs;
use std::path::PathBuf;

/// SWE-bench dataset sources
#[derive(Debug, Clone, PartialEq, Eq)]
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
    /// Get the HuggingFace dataset URL (Parquet format)
    pub fn huggingface_url(&self) -> Option<String> {
        match self {
            DatasetSource::Verified => Some(
                "https://huggingface.co/datasets/princeton-nlp/SWE-bench_Verified/resolve/main/data/test-00000-of-00001.parquet".to_string()
            ),
            DatasetSource::Lite => Some(
                "https://huggingface.co/datasets/princeton-nlp/SWE-bench_Lite/resolve/main/data/test-00000-of-00001.parquet".to_string()
            ),
            DatasetSource::Full => Some(
                "https://huggingface.co/datasets/princeton-nlp/SWE-bench/resolve/main/data/test-00000-of-00001.parquet".to_string()
            ),
            DatasetSource::Local(_) => None,
        }
    }

    /// Get the local cache filename
    pub fn cache_filename(&self) -> String {
        match self {
            DatasetSource::Verified => "swe_bench_verified.parquet".to_string(),
            DatasetSource::Lite => "swe_bench_lite.parquet".to_string(),
            DatasetSource::Full => "swe_bench_full.parquet".to_string(),
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

    /// Download a dataset from HuggingFace
    pub async fn download(&self, source: &DatasetSource) -> Result<PathBuf> {
        match source {
            DatasetSource::Local(path) => {
                if !path.exists() {
                    anyhow::bail!("Local dataset file not found: {:?}", path);
                }
                Ok(path.clone())
            }
            _ => {
                let url = source
                    .huggingface_url()
                    .context("No download URL available for this source")?;
                let cache_path = self.cache_path(source);

                tracing::info!(
                    "Downloading {} dataset from HuggingFace...",
                    match source {
                        DatasetSource::Verified => "SWE-bench Verified",
                        DatasetSource::Lite => "SWE-bench Lite",
                        DatasetSource::Full => "SWE-bench Full",
                        _ => "dataset",
                    }
                );
                tracing::info!("URL: {}", url);
                tracing::info!("Saving to: {:?}", cache_path);

                // Download the file
                let response = reqwest::get(&url)
                    .await
                    .context("Failed to download dataset")?;

                if !response.status().is_success() {
                    let status = response.status();
                    if status.as_u16() == 404 {
                        anyhow::bail!(
                            "Dataset file not found at HuggingFace (HTTP 404).\n\
                             \n\
                             The SWE-bench datasets have migrated to Parquet format.\n\
                             \n\
                             📥 Manual Download Instructions:\n\
                             \n\
                             Option 1: Use Python with datasets library\n\
                             -----------------------------------------\n\
                             pip install datasets\n\
                             python -c \"from datasets import load_dataset; \\\n\
                                        ds = load_dataset('princeton-nlp/SWE-bench_Verified', split='test'); \\\n\
                                        ds.to_json('swe_bench_verified.jsonl')\"\n\
                             cargo run -- eval --dataset swe_bench_verified.jsonl --count 10\n\
                             \n\
                             Option 2: Use huggingface-cli\n\
                             -----------------------------\n\
                             pip install huggingface-hub\n\
                             huggingface-cli download princeton-nlp/SWE-bench_Verified --repo-type dataset --local-dir ./data\n\
                             \n\
                             Dataset variants:\n\
                             - princeton-nlp/SWE-bench_Verified (500 tasks)\n\
                             - princeton-nlp/SWE-bench_Lite (300 tasks)\n\
                             - princeton-nlp/SWE-bench (2,294 tasks)\n\
                             \n\
                             See SWEBENCH_USAGE.md for more details."
                        );
                    }
                    anyhow::bail!("Failed to download dataset: HTTP {}", status);
                }

                let bytes = response
                    .bytes()
                    .await
                    .context("Failed to read response body")?;

                // Save Parquet file to cache
                fs::write(&cache_path, &bytes).context("Failed to write dataset to cache")?;

                tracing::info!("Dataset downloaded successfully ({} bytes)", bytes.len());

                // Convert Parquet to JSONL for easier reading
                let jsonl_path = cache_path.with_extension("jsonl");
                if !jsonl_path.exists() {
                    tracing::info!("Converting Parquet to JSONL...");
                    convert_parquet_to_jsonl(&cache_path, &jsonl_path)?;
                    let size = jsonl_path.metadata()?.len();
                    tracing::info!("Converted to JSONL ({} bytes)", size);
                }

                Ok(jsonl_path)
            }
        }
    }

    /// Get or download a dataset
    pub async fn get_or_download(&self, source: &DatasetSource) -> Result<PathBuf> {
        self.init()?;

        let cache_path = self.cache_path(source);
        let jsonl_path = cache_path.with_extension("jsonl");

        // Check if we already have the converted JSONL file
        if jsonl_path.exists() {
            tracing::info!("Using cached dataset: {:?}", jsonl_path);
            return Ok(jsonl_path);
        }

        // Check if we have the parquet file but not the jsonl
        if cache_path.exists() {
            tracing::info!("Converting cached Parquet to JSONL...");
            convert_parquet_to_jsonl(&cache_path, &jsonl_path)?;
            tracing::info!("Converted to JSONL");
            return Ok(jsonl_path);
        }

        // Need to download
        self.download(source).await
    }

    /// Load a sample from a dataset source
    pub async fn load_sample(&self, source: DatasetSource, count: usize) -> Result<Vec<Task>> {
        use super::task_loader::TaskLoader;

        let path = self.get_or_download(&source).await?;
        let loader = TaskLoader::new(path);
        loader.load_sample(count)
    }

    /// Load stratified sample
    pub async fn load_stratified(
        &self,
        source: DatasetSource,
        simple: usize,
        medium: usize,
        hard: usize,
    ) -> Result<Vec<Task>> {
        use super::task_loader::TaskLoader;

        let path = self.get_or_download(&source).await?;
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

/// Convert Parquet file to JSONL
fn convert_parquet_to_jsonl(parquet_path: &PathBuf, jsonl_path: &PathBuf) -> Result<()> {
    use std::fs::File;
    use std::io::{BufWriter, Write};

    // Open parquet file
    let file = File::open(parquet_path).context("Failed to open Parquet file")?;

    let reader = SerializedFileReader::new(file).context("Failed to create Parquet reader")?;

    // Create output file
    let output = File::create(jsonl_path).context("Failed to create JSONL file")?;
    let mut writer = BufWriter::new(output);

    // Read all row groups and convert to JSON
    let metadata = reader.metadata();
    let num_rows = metadata.file_metadata().num_rows();

    tracing::info!("Converting {} rows from Parquet to JSONL", num_rows);

    // Use arrow to read parquet
    use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
    let arrow_reader = ParquetRecordBatchReaderBuilder::try_new(File::open(parquet_path)?)
        .context("Failed to create Arrow reader")?
        .build()
        .context("Failed to build Arrow reader")?;

    let mut row_count = 0;
    for batch_result in arrow_reader {
        let batch = batch_result.context("Failed to read record batch")?;
        let schema = batch.schema();

        // Convert each row to JSON
        for row_idx in 0..batch.num_rows() {
            let mut json_obj = serde_json::Map::new();

            for col_idx in 0..batch.num_columns() {
                let column = batch.column(col_idx);
                let field = schema.field(col_idx);
                let field_name = field.name();

                // Convert arrow value to JSON value
                let json_value = arrow_to_json_value(column, row_idx)?;
                json_obj.insert(field_name.clone(), json_value);
            }

            // Write as JSONL (one JSON object per line)
            let json_str = serde_json::to_string(&json_obj).context("Failed to serialize JSON")?;
            writeln!(writer, "{}", json_str).context("Failed to write JSONL line")?;

            row_count += 1;
            if row_count % 100 == 0 {
                tracing::debug!("Converted {} rows", row_count);
            }
        }
    }

    writer.flush().context("Failed to flush writer")?;
    tracing::info!("Successfully converted {} rows", row_count);

    Ok(())
}

/// Convert Arrow array value to JSON value
fn arrow_to_json_value(
    column: &arrow::array::ArrayRef,
    row_idx: usize,
) -> Result<serde_json::Value> {
    use arrow::array::*;
    use arrow::datatypes::DataType;

    if column.is_null(row_idx) {
        return Ok(serde_json::Value::Null);
    }

    let value = match column.data_type() {
        DataType::Utf8 => {
            let array = column
                .as_any()
                .downcast_ref::<StringArray>()
                .context("Failed to downcast to StringArray")?;
            serde_json::Value::String(array.value(row_idx).to_string())
        }
        DataType::Int64 => {
            let array = column
                .as_any()
                .downcast_ref::<Int64Array>()
                .context("Failed to downcast to Int64Array")?;
            serde_json::Value::Number(array.value(row_idx).into())
        }
        DataType::Int32 => {
            let array = column
                .as_any()
                .downcast_ref::<Int32Array>()
                .context("Failed to downcast to Int32Array")?;
            serde_json::Value::Number(array.value(row_idx).into())
        }
        DataType::Boolean => {
            let array = column
                .as_any()
                .downcast_ref::<BooleanArray>()
                .context("Failed to downcast to BooleanArray")?;
            serde_json::Value::Bool(array.value(row_idx))
        }
        DataType::Float64 => {
            let array = column
                .as_any()
                .downcast_ref::<Float64Array>()
                .context("Failed to downcast to Float64Array")?;
            serde_json::Number::from_f64(array.value(row_idx))
                .map(serde_json::Value::Number)
                .unwrap_or(serde_json::Value::Null)
        }
        _ => {
            // For other types, try to convert to string
            serde_json::Value::String(format!("{:?}", column))
        }
    };

    Ok(value)
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
        assert!(
            DatasetSource::Local(PathBuf::from("/tmp/test.json"))
                .huggingface_url()
                .is_none()
        );
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
