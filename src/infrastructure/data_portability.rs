//! Data export and import for portability
//!
//! Provides serialization and deserialization of application data
//! to/from various formats (JSON, CSV, TOML).
//!
//! # Examples
//!
//! ```
//! use toad::infrastructure::{DataExporter, DataFormat};
//! use serde::{Serialize, Deserialize};
//!
//! #[derive(Serialize, Deserialize)]
//! struct MyData {
//!     name: String,
//!     value: i32,
//! }
//!
//! let data = MyData { name: "test".to_string(), value: 42 };
//! let exporter = DataExporter::new();
//! let json = exporter.export(&data, DataFormat::Json).unwrap();
//! ```

use anyhow::{Context, Result};
use serde::{Serialize, de::DeserializeOwned};
use std::path::Path;
use tokio::fs;

/// Data format for export/import
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataFormat {
    /// JSON format
    Json,
    /// Pretty-printed JSON
    JsonPretty,
    /// TOML format
    Toml,
    /// CSV format (for tabular data)
    Csv,
}

impl DataFormat {
    /// Get file extension for this format
    pub fn extension(&self) -> &'static str {
        match self {
            DataFormat::Json | DataFormat::JsonPretty => "json",
            DataFormat::Toml => "toml",
            DataFormat::Csv => "csv",
        }
    }

    /// Detect format from file extension
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "json" => Some(DataFormat::Json),
            "toml" => Some(DataFormat::Toml),
            "csv" => Some(DataFormat::Csv),
            _ => None,
        }
    }
}

/// Data exporter for serializing data to various formats
pub struct DataExporter {
    /// Default format
    default_format: DataFormat,
}

impl DataExporter {
    /// Create a new data exporter
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::infrastructure::DataExporter;
    ///
    /// let exporter = DataExporter::new();
    /// ```
    pub fn new() -> Self {
        Self {
            default_format: DataFormat::Json,
        }
    }

    /// Create exporter with a specific default format
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::infrastructure::{DataExporter, DataFormat};
    ///
    /// let exporter = DataExporter::with_format(DataFormat::Toml);
    /// ```
    pub fn with_format(format: DataFormat) -> Self {
        Self {
            default_format: format,
        }
    }

    /// Export data to a string
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::infrastructure::{DataExporter, DataFormat};
    /// use serde::Serialize;
    ///
    /// #[derive(Serialize)]
    /// struct Data { value: i32 }
    ///
    /// let exporter = DataExporter::new();
    /// let data = Data { value: 42 };
    /// let json = exporter.export(&data, DataFormat::Json).unwrap();
    /// assert!(json.contains("42"));
    /// ```
    pub fn export<T: Serialize>(&self, data: &T, format: DataFormat) -> Result<String> {
        match format {
            DataFormat::Json => serde_json::to_string(data).context("Failed to serialize to JSON"),
            DataFormat::JsonPretty => {
                serde_json::to_string_pretty(data).context("Failed to serialize to pretty JSON")
            }
            DataFormat::Toml => toml::to_string(data).context("Failed to serialize to TOML"),
            DataFormat::Csv => {
                anyhow::bail!("CSV export requires using export_csv_records")
            }
        }
    }

    /// Export data to a file
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use toad::infrastructure::{DataExporter, DataFormat};
    /// # use serde::Serialize;
    /// #
    /// # #[derive(Serialize)]
    /// # struct Data { value: i32 }
    /// #
    /// # async fn example() -> anyhow::Result<()> {
    /// let exporter = DataExporter::new();
    /// let data = Data { value: 42 };
    /// exporter.export_to_file(&data, "data.json", DataFormat::Json).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn export_to_file<T: Serialize>(
        &self,
        data: &T,
        path: impl AsRef<Path>,
        format: DataFormat,
    ) -> Result<()> {
        let content = self.export(data, format)?;
        fs::write(path.as_ref(), content)
            .await
            .context("Failed to write file")?;
        Ok(())
    }

    /// Get the default format
    pub fn default_format(&self) -> DataFormat {
        self.default_format
    }
}

impl Default for DataExporter {
    fn default() -> Self {
        Self::new()
    }
}

/// Data importer for deserializing data from various formats
pub struct DataImporter {
    /// Default format
    default_format: DataFormat,
}

impl DataImporter {
    /// Create a new data importer
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::infrastructure::DataImporter;
    ///
    /// let importer = DataImporter::new();
    /// ```
    pub fn new() -> Self {
        Self {
            default_format: DataFormat::Json,
        }
    }

    /// Create importer with a specific default format
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::infrastructure::{DataImporter, DataFormat};
    ///
    /// let importer = DataImporter::with_format(DataFormat::Toml);
    /// ```
    pub fn with_format(format: DataFormat) -> Self {
        Self {
            default_format: format,
        }
    }

    /// Import data from a string
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::infrastructure::{DataImporter, DataFormat};
    /// use serde::Deserialize;
    ///
    /// #[derive(Deserialize, Debug, PartialEq)]
    /// struct Data { value: i32 }
    ///
    /// let importer = DataImporter::new();
    /// let json = r#"{"value": 42}"#;
    /// let data: Data = importer.import(json, DataFormat::Json).unwrap();
    /// assert_eq!(data.value, 42);
    /// ```
    pub fn import<T: DeserializeOwned>(&self, content: &str, format: DataFormat) -> Result<T> {
        match format {
            DataFormat::Json | DataFormat::JsonPretty => {
                serde_json::from_str(content).context("Failed to deserialize from JSON")
            }
            DataFormat::Toml => toml::from_str(content).context("Failed to deserialize from TOML"),
            DataFormat::Csv => {
                anyhow::bail!("CSV import requires using import_csv_records")
            }
        }
    }

    /// Import data from a file
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use toad::infrastructure::{DataImporter, DataFormat};
    /// # use serde::Deserialize;
    /// #
    /// # #[derive(Deserialize)]
    /// # struct Data { value: i32 }
    /// #
    /// # async fn example() -> anyhow::Result<()> {
    /// let importer = DataImporter::new();
    /// let data: Data = importer.import_from_file("data.json", DataFormat::Json).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn import_from_file<T: DeserializeOwned>(
        &self,
        path: impl AsRef<Path>,
        format: DataFormat,
    ) -> Result<T> {
        let content = fs::read_to_string(path.as_ref())
            .await
            .context("Failed to read file")?;
        self.import(&content, format)
    }

    /// Import data from a file, auto-detecting format from extension
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use toad::infrastructure::DataImporter;
    /// # use serde::Deserialize;
    /// #
    /// # #[derive(Deserialize)]
    /// # struct Data { value: i32 }
    /// #
    /// # async fn example() -> anyhow::Result<()> {
    /// let importer = DataImporter::new();
    /// let data: Data = importer.import_from_file_auto("data.json").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn import_from_file_auto<T: DeserializeOwned>(
        &self,
        path: impl AsRef<Path>,
    ) -> Result<T> {
        let path = path.as_ref();

        // Detect format from extension
        let format = path
            .extension()
            .and_then(|e| e.to_str())
            .and_then(DataFormat::from_extension)
            .unwrap_or(self.default_format);

        self.import_from_file(path, format).await
    }

    /// Get the default format
    pub fn default_format(&self) -> DataFormat {
        self.default_format
    }
}

impl Default for DataImporter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use tempfile::TempDir;

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
    struct TestData {
        name: String,
        value: i32,
        items: Vec<String>,
    }

    fn sample_data() -> TestData {
        TestData {
            name: "test".to_string(),
            value: 42,
            items: vec!["a".to_string(), "b".to_string(), "c".to_string()],
        }
    }

    #[test]
    fn test_data_format_extension() {
        assert_eq!(DataFormat::Json.extension(), "json");
        assert_eq!(DataFormat::Toml.extension(), "toml");
        assert_eq!(DataFormat::Csv.extension(), "csv");
    }

    #[test]
    fn test_data_format_from_extension() {
        assert_eq!(DataFormat::from_extension("json"), Some(DataFormat::Json));
        assert_eq!(DataFormat::from_extension("JSON"), Some(DataFormat::Json));
        assert_eq!(DataFormat::from_extension("toml"), Some(DataFormat::Toml));
        assert_eq!(DataFormat::from_extension("unknown"), None);
    }

    #[test]
    fn test_export_json() {
        let exporter = DataExporter::new();
        let data = sample_data();

        let json = exporter.export(&data, DataFormat::Json).unwrap();
        assert!(json.contains("test"));
        assert!(json.contains("42"));
    }

    #[test]
    fn test_export_json_pretty() {
        let exporter = DataExporter::new();
        let data = sample_data();

        let json = exporter.export(&data, DataFormat::JsonPretty).unwrap();
        assert!(json.contains("test"));
        assert!(json.contains("\n")); // Pretty-printed
    }

    #[test]
    fn test_export_toml() {
        let exporter = DataExporter::new();
        let data = sample_data();

        let toml_str = exporter.export(&data, DataFormat::Toml).unwrap();
        assert!(toml_str.contains("test"));
        assert!(toml_str.contains("42"));
    }

    #[test]
    fn test_import_json() {
        let importer = DataImporter::new();
        let json = r#"{"name":"test","value":42,"items":["a","b","c"]}"#;

        let data: TestData = importer.import(json, DataFormat::Json).unwrap();
        assert_eq!(data, sample_data());
    }

    #[test]
    fn test_import_toml() {
        let importer = DataImporter::new();
        let toml_str = r#"
            name = "test"
            value = 42
            items = ["a", "b", "c"]
        "#;

        let data: TestData = importer.import(toml_str, DataFormat::Toml).unwrap();
        assert_eq!(data, sample_data());
    }

    #[tokio::test]
    async fn test_export_import_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.json");

        let exporter = DataExporter::new();
        let data = sample_data();

        // Export
        exporter
            .export_to_file(&data, &file_path, DataFormat::Json)
            .await
            .unwrap();

        // Import
        let importer = DataImporter::new();
        let loaded: TestData = importer
            .import_from_file(&file_path, DataFormat::Json)
            .await
            .unwrap();

        assert_eq!(loaded, data);
    }

    #[tokio::test]
    async fn test_import_auto_detect() {
        let temp_dir = TempDir::new().unwrap();

        // Test JSON
        let json_path = temp_dir.path().join("test.json");
        let exporter = DataExporter::new();
        let data = sample_data();
        exporter
            .export_to_file(&data, &json_path, DataFormat::Json)
            .await
            .unwrap();

        let importer = DataImporter::new();
        let loaded: TestData = importer.import_from_file_auto(&json_path).await.unwrap();
        assert_eq!(loaded, data);

        // Test TOML
        let toml_path = temp_dir.path().join("test.toml");
        exporter
            .export_to_file(&data, &toml_path, DataFormat::Toml)
            .await
            .unwrap();

        let loaded: TestData = importer.import_from_file_auto(&toml_path).await.unwrap();
        assert_eq!(loaded, data);
    }

    #[test]
    fn test_round_trip() {
        let exporter = DataExporter::new();
        let importer = DataImporter::new();
        let original = sample_data();

        // JSON round trip
        let json = exporter.export(&original, DataFormat::Json).unwrap();
        let loaded: TestData = importer.import(&json, DataFormat::Json).unwrap();
        assert_eq!(loaded, original);

        // TOML round trip
        let toml_str = exporter.export(&original, DataFormat::Toml).unwrap();
        let loaded: TestData = importer.import(&toml_str, DataFormat::Toml).unwrap();
        assert_eq!(loaded, original);
    }
}
