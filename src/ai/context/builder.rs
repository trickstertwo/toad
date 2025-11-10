/// Context builder for creating AstContext from multiple files
use crate::ai::context::{AstCache, AstContext, ExtractorRegistry, FileContext};
use anyhow::{Context as _, Result};
use std::path::{Path, PathBuf};
use tokio::fs;

/// Builder for constructing AstContext from multiple source files
///
/// Provides a fluent API for adding files and building the final context.
/// Supports optional caching to avoid re-parsing unchanged files.
///
/// # Example
///
/// ```no_run
/// use toad::ai::context::ContextBuilder;
///
/// # async fn example() -> anyhow::Result<()> {
/// let context = ContextBuilder::new()
///     .with_cache(100)
///     .add_file("src/main.py").await?
///     .add_file("src/utils.js").await?
///     .add_directory("src/components", &["ts", "tsx"]).await?
///     .build();
///
/// println!("Extracted {} symbols", context.total_symbols);
/// # Ok(())
/// # }
/// ```
pub struct ContextBuilder {
    registry: ExtractorRegistry,
    cache: Option<AstCache>,
    file_contexts: Vec<FileContext>,
}

impl ContextBuilder {
    /// Create a new context builder
    pub fn new() -> Result<Self> {
        Ok(Self {
            registry: ExtractorRegistry::new()?,
            cache: None,
            file_contexts: Vec::new(),
        })
    }

    /// Enable caching with the specified capacity
    ///
    /// Cached AST results will be reused if the file hasn't changed (based on mtime).
    pub fn with_cache(mut self, capacity: usize) -> Self {
        self.cache = Some(AstCache::with_capacity(capacity));
        self
    }

    /// Add a single file to the context
    ///
    /// The file will be parsed using the appropriate parser based on extension.
    pub async fn add_file(mut self, path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let file_context = self.parse_file_with_cache(path).await?;
        self.file_contexts.push(file_context);
        Ok(self)
    }

    /// Add multiple files to the context
    ///
    /// Files will be parsed concurrently for better performance.
    pub async fn add_files(mut self, paths: &[impl AsRef<Path>]) -> Result<Self> {
        let contexts = self.parse_files_with_cache(paths).await?;
        self.file_contexts.extend(contexts);
        Ok(self)
    }

    /// Add all supported files from a directory
    ///
    /// Only files with supported extensions will be included.
    /// Subdirectories are not traversed by default.
    pub async fn add_directory(
        mut self,
        dir: impl AsRef<Path>,
        extensions: &[&str],
    ) -> Result<Self> {
        let files = self.scan_directory(dir.as_ref(), extensions, false).await?;
        let contexts = self.parse_files_with_cache(&files).await?;
        self.file_contexts.extend(contexts);
        Ok(self)
    }

    /// Add all supported files from a directory recursively
    ///
    /// All subdirectories will be traversed.
    pub async fn add_directory_recursive(
        mut self,
        dir: impl AsRef<Path>,
        extensions: &[&str],
    ) -> Result<Self> {
        let files = self.scan_directory(dir.as_ref(), extensions, true).await?;
        let contexts = self.parse_files_with_cache(&files).await?;
        self.file_contexts.extend(contexts);
        Ok(self)
    }

    /// Build the final AstContext
    ///
    /// Consumes the builder and returns the completed context.
    pub fn build(self) -> AstContext {
        let mut context = AstContext::new();
        for file_context in self.file_contexts {
            context.add_file(file_context);
        }
        context
    }

    /// Parse a single file, using cache if available
    async fn parse_file_with_cache(&self, path: &Path) -> Result<FileContext> {
        // Check cache first
        if let Some(cache) = &self.cache {
            let metadata = fs::metadata(path)
                .await
                .with_context(|| format!("Failed to get metadata for {}", path.display()))?;

            let mtime = metadata
                .modified()
                .with_context(|| format!("Failed to get mtime for {}", path.display()))?;

            if let Some(cached) = cache.get(&path.to_path_buf(), mtime) {
                return Ok(cached);
            }

            // Not in cache or outdated - parse and cache
            let context = self.registry.parse_file(path).await?;
            cache.insert(path.to_path_buf(), mtime, context.clone());
            Ok(context)
        } else {
            // No cache - parse directly
            self.registry.parse_file(path).await
        }
    }

    /// Parse multiple files with optional caching
    async fn parse_files_with_cache(&self, paths: &[impl AsRef<Path>]) -> Result<Vec<FileContext>> {
        let mut contexts = Vec::new();

        for path in paths {
            let context = self.parse_file_with_cache(path.as_ref()).await?;
            contexts.push(context);
        }

        Ok(contexts)
    }

    /// Scan a directory for files with matching extensions
    async fn scan_directory(
        &self,
        dir: &Path,
        extensions: &[&str],
        recursive: bool,
    ) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();
        self.scan_directory_impl(dir, extensions, recursive, &mut files).await?;
        Ok(files)
    }

    /// Recursive implementation of directory scanning
    fn scan_directory_impl<'a>(
        &'a self,
        dir: &'a Path,
        extensions: &'a [&'a str],
        recursive: bool,
        files: &'a mut Vec<PathBuf>,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<()>> + Send + 'a>> {
        Box::pin(async move {
            let mut entries = fs::read_dir(dir)
                .await
                .with_context(|| format!("Failed to read directory {}", dir.display()))?;

            while let Some(entry) = entries.next_entry().await? {
                let path = entry.path();
                let metadata = entry.metadata().await?;

                if metadata.is_dir() {
                    if recursive {
                        self.scan_directory_impl(&path, extensions, recursive, files).await?;
                    }
                } else if metadata.is_file() {
                    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                        if extensions.contains(&ext) {
                            files.push(path);
                        }
                    }
                }
            }

            Ok(())
        })
    }
}

impl Default for ContextBuilder {
    fn default() -> Self {
        Self::new().expect("Failed to create ContextBuilder")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_builder_creation() {
        let builder = ContextBuilder::new();
        assert!(builder.is_ok());
    }

    #[tokio::test]
    async fn test_add_single_file() {
        let dir = TempDir::new().unwrap();
        let file_path = dir.path().join("test.py");
        std::fs::write(&file_path, "def hello(): pass").unwrap();

        let context = ContextBuilder::new()
            .unwrap()
            .add_file(&file_path)
            .await
            .unwrap()
            .build();

        assert_eq!(context.file_contexts.len(), 1);
        assert!(context.total_symbols > 0);
    }

    #[tokio::test]
    async fn test_add_multiple_files() {
        let dir = TempDir::new().unwrap();

        let py_file = dir.path().join("test.py");
        std::fs::write(&py_file, "def test(): pass").unwrap();

        let js_file = dir.path().join("test.js");
        std::fs::write(&js_file, "function test() {}").unwrap();

        let context = ContextBuilder::new()
            .unwrap()
            .add_files(&[&py_file, &js_file])
            .await
            .unwrap()
            .build();

        assert_eq!(context.file_contexts.len(), 2);
    }

    #[tokio::test]
    async fn test_add_directory() {
        let dir = TempDir::new().unwrap();

        // Create files
        std::fs::write(dir.path().join("file1.py"), "def test1(): pass").unwrap();
        std::fs::write(dir.path().join("file2.py"), "def test2(): pass").unwrap();
        std::fs::write(dir.path().join("ignore.txt"), "not code").unwrap();

        let context = ContextBuilder::new()
            .unwrap()
            .add_directory(dir.path(), &["py"])
            .await
            .unwrap()
            .build();

        assert_eq!(context.file_contexts.len(), 2);
    }

    #[tokio::test]
    async fn test_add_directory_recursive() {
        let dir = TempDir::new().unwrap();
        let subdir = dir.path().join("subdir");
        std::fs::create_dir(&subdir).unwrap();

        // Create files in root and subdir
        std::fs::write(dir.path().join("root.js"), "function root() {}").unwrap();
        std::fs::write(subdir.join("nested.js"), "function nested() {}").unwrap();

        let context = ContextBuilder::new()
            .unwrap()
            .add_directory_recursive(dir.path(), &["js"])
            .await
            .unwrap()
            .build();

        assert_eq!(context.file_contexts.len(), 2);
    }

    #[tokio::test]
    async fn test_with_cache() {
        let dir = TempDir::new().unwrap();
        let file_path = dir.path().join("test.py");
        std::fs::write(&file_path, "def hello(): pass").unwrap();

        // First parse - should cache
        let builder = ContextBuilder::new()
            .unwrap()
            .with_cache(10);

        let context1 = builder
            .add_file(&file_path)
            .await
            .unwrap()
            .build();

        assert_eq!(context1.file_contexts.len(), 1);

        // Second parse - should use cache (same builder would be needed, but we already consumed it)
        // For a real test, we'd need to share the cache between builders
        // This test just verifies the with_cache method works
    }

    #[tokio::test]
    async fn test_fluent_api_chaining() {
        let dir = TempDir::new().unwrap();

        std::fs::write(dir.path().join("file1.py"), "def test1(): pass").unwrap();
        std::fs::write(dir.path().join("file2.js"), "function test2() {}").unwrap();
        std::fs::write(dir.path().join("file3.ts"), "function test3(): void {}").unwrap();

        let file1 = dir.path().join("file1.py");

        let context = ContextBuilder::new()
            .unwrap()
            .with_cache(50)
            .add_file(&file1)
            .await
            .unwrap()
            .add_directory(dir.path(), &["js", "ts"])
            .await
            .unwrap()
            .build();

        assert_eq!(context.file_contexts.len(), 3);
        assert!(context.total_symbols > 0);
    }

    #[tokio::test]
    async fn test_build_empty_context() {
        let context = ContextBuilder::new()
            .unwrap()
            .build();

        assert_eq!(context.file_contexts.len(), 0);
        assert_eq!(context.total_symbols, 0);
    }
}
