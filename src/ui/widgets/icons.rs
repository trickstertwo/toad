//! Nerd Font icon support for file types and status indicators
//!
//! Provides icon mappings for various file types, folders, and status indicators
//! using Nerd Font glyphs for enhanced visual clarity.
//!
//! # Examples
//!
//! ```
//! use toad::widgets::{Icons, FileType};
//!
//! // Get icon for a file type
//! let icon = Icons::file_icon("main.rs");
//! assert_eq!(icon, "🦀");
//!
//! // Get icon for a folder
//! let folder = Icons::folder_icon(false);
//! assert_eq!(folder, "📁");
//! ```

use std::path::Path;

/// File type classifications
///
/// # Examples
///
/// ```
/// use toad::widgets::FileType;
///
/// let file_type = FileType::from_extension("rs");
/// assert_eq!(file_type, FileType::Rust);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    /// Rust source files (.rs)
    Rust,
    /// JavaScript files (.js, .jsx, .mjs)
    JavaScript,
    /// TypeScript files (.ts, .tsx)
    TypeScript,
    /// Python files (.py)
    Python,
    /// Go files (.go)
    Go,
    /// C files (.c, .h)
    C,
    /// C++ files (.cpp, .hpp, .cc, .cxx)
    Cpp,
    /// Java files (.java)
    Java,
    /// Ruby files (.rb)
    Ruby,
    /// PHP files (.php)
    Php,
    /// Shell scripts (.sh, .bash, .zsh)
    Shell,
    /// HTML files (.html, .htm)
    Html,
    /// CSS files (.css, .scss, .sass, .less)
    Css,
    /// JSON files (.json)
    Json,
    /// YAML files (.yaml, .yml)
    Yaml,
    /// TOML files (.toml)
    Toml,
    /// Markdown files (.md)
    Markdown,
    /// Text files (.txt)
    Text,
    /// Image files (.png, .jpg, .gif, .svg)
    Image,
    /// Archive files (.zip, .tar, .gz)
    Archive,
    /// Git files (.gitignore, .gitmodules)
    Git,
    /// Docker files (Dockerfile)
    Docker,
    /// Generic file
    File,
}

impl FileType {
    /// Detect file type from extension
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::FileType;
    ///
    /// assert_eq!(FileType::from_extension("rs"), FileType::Rust);
    /// assert_eq!(FileType::from_extension("js"), FileType::JavaScript);
    /// assert_eq!(FileType::from_extension("unknown"), FileType::File);
    /// ```
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            "rs" => FileType::Rust,
            "js" | "jsx" | "mjs" | "cjs" => FileType::JavaScript,
            "ts" | "tsx" => FileType::TypeScript,
            "py" | "pyw" | "pyc" => FileType::Python,
            "go" => FileType::Go,
            "c" | "h" => FileType::C,
            "cpp" | "hpp" | "cc" | "cxx" | "hxx" => FileType::Cpp,
            "java" => FileType::Java,
            "rb" | "erb" => FileType::Ruby,
            "php" => FileType::Php,
            "sh" | "bash" | "zsh" | "fish" => FileType::Shell,
            "html" | "htm" => FileType::Html,
            "css" | "scss" | "sass" | "less" => FileType::Css,
            "json" | "jsonc" => FileType::Json,
            "yaml" | "yml" => FileType::Yaml,
            "toml" => FileType::Toml,
            "md" | "markdown" => FileType::Markdown,
            "txt" => FileType::Text,
            "png" | "jpg" | "jpeg" | "gif" | "svg" | "ico" | "bmp" => FileType::Image,
            "zip" | "tar" | "gz" | "bz2" | "xz" | "7z" | "rar" => FileType::Archive,
            "gitignore" | "gitmodules" | "gitattributes" => FileType::Git,
            _ => FileType::File,
        }
    }

    /// Detect file type from filename
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::FileType;
    ///
    /// assert_eq!(FileType::from_filename("main.rs"), FileType::Rust);
    /// assert_eq!(FileType::from_filename("Dockerfile"), FileType::Docker);
    /// ```
    pub fn from_filename(filename: &str) -> Self {
        // Check for special filenames
        match filename.to_lowercase().as_str() {
            "dockerfile" | "dockerfile.dev" | "dockerfile.prod" => return FileType::Docker,
            ".gitignore" | ".gitmodules" | ".gitattributes" => return FileType::Git,
            _ => {}
        }

        // Extract extension
        if let Some(ext) = filename.split('.').next_back()
            && ext != filename
        {
            return Self::from_extension(ext);
        }

        FileType::File
    }

    /// Get Nerd Font icon for this file type
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::FileType;
    ///
    /// let rust_icon = FileType::Rust.icon();
    /// assert_eq!(rust_icon, "🦀");
    /// ```
    pub const fn icon(&self) -> &'static str {
        match self {
            FileType::Rust => "🦀",       // Rust logo
            FileType::JavaScript => "JS", // JS logo
            FileType::TypeScript => "TS", // TS logo
            FileType::Python => "🐍",     // Python logo
            FileType::Go => "Go",         // Go logo
            FileType::C => "C",           // C logo
            FileType::Cpp => "C++",       // C++ logo
            FileType::Java => "☕",       // Java logo
            FileType::Ruby => "💎",       // Ruby logo
            FileType::Php => "🐘",        // PHP logo
            FileType::Shell => "🐚",      // Shell icon
            FileType::Html => "🌐",       // HTML logo
            FileType::Css => "🎨",        // CSS logo
            FileType::Json => "{}",       // JSON icon
            FileType::Yaml => "📄",       // YAML icon
            FileType::Toml => "⚙",        // Config icon
            FileType::Markdown => "📝",   // Markdown icon
            FileType::Text => "📄",       // Text file
            FileType::Image => "🖼",       // Image icon
            FileType::Archive => "📦",    // Archive icon
            FileType::Git => "🔀",        // Git icon
            FileType::Docker => "🐳",     // Docker icon
            FileType::File => "📄",       // Generic file
        }
    }
}

/// Git status indicators
///
/// # Examples
///
/// ```
/// use toad::widgets::GitStatus;
///
/// let modified = GitStatus::Modified;
/// assert_eq!(modified.icon(), "●");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GitStatus {
    /// File is untracked
    Untracked,
    /// File is modified
    Modified,
    /// File is added/staged
    Added,
    /// File is deleted
    Deleted,
    /// File is renamed
    Renamed,
    /// File has conflicts
    Conflict,
    /// File is ignored
    Ignored,
    /// File is clean (no changes)
    Clean,
}

impl GitStatus {
    /// Get icon for this git status
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::GitStatus;
    ///
    /// assert_eq!(GitStatus::Modified.icon(), "●");
    /// assert_eq!(GitStatus::Added.icon(), "+");
    /// ```
    pub const fn icon(&self) -> &'static str {
        match self {
            GitStatus::Untracked => "?",
            GitStatus::Modified => "●",
            GitStatus::Added => "+",
            GitStatus::Deleted => "-",
            GitStatus::Renamed => "→",
            GitStatus::Conflict => "!",
            GitStatus::Ignored => "◌",
            GitStatus::Clean => "✓",
        }
    }

    /// Get short text representation
    pub const fn short(&self) -> &'static str {
        match self {
            GitStatus::Untracked => "??",
            GitStatus::Modified => "M",
            GitStatus::Added => "A",
            GitStatus::Deleted => "D",
            GitStatus::Renamed => "R",
            GitStatus::Conflict => "U",
            GitStatus::Ignored => "!",
            GitStatus::Clean => " ",
        }
    }
}

/// Status indicators for various UI elements
///
/// # Examples
///
/// ```
/// use toad::widgets::StatusIcon;
///
/// let error = StatusIcon::Error;
/// assert_eq!(error.icon(), "✗");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusIcon {
    /// Success indicator
    Success,
    /// Error indicator
    Error,
    /// Warning indicator
    Warning,
    /// Info indicator
    Info,
    /// Loading/pending indicator
    Loading,
    /// Checkmark
    Check,
    /// Cross/X mark
    Cross,
    /// Question mark
    Question,
    /// Lightbulb (hint)
    Hint,
    /// Lock (secured)
    Locked,
    /// Unlock (unsecured)
    Unlocked,
}

impl StatusIcon {
    /// Get icon for this status
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::StatusIcon;
    ///
    /// assert_eq!(StatusIcon::Success.icon(), "✓");
    /// assert_eq!(StatusIcon::Error.icon(), "✗");
    /// ```
    pub const fn icon(&self) -> &'static str {
        match self {
            StatusIcon::Success => "✓",
            StatusIcon::Error => "✗",
            StatusIcon::Warning => "⚠",
            StatusIcon::Info => "ℹ",
            StatusIcon::Loading => "⟳",
            StatusIcon::Check => "✓",
            StatusIcon::Cross => "✗",
            StatusIcon::Question => "?",
            StatusIcon::Hint => "💡",
            StatusIcon::Locked => "🔒",
            StatusIcon::Unlocked => "🔓",
        }
    }
}

/// Icon utilities and helpers
///
/// # Examples
///
/// ```
/// use toad::widgets::Icons;
///
/// let icon = Icons::file_icon("main.rs");
/// assert_eq!(icon, "🦀");
/// ```
pub struct Icons;

impl Icons {
    /// Get icon for a file path
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Icons;
    ///
    /// assert_eq!(Icons::file_icon("src/main.rs"), "🦀");
    /// assert_eq!(Icons::file_icon("package.json"), "{}");
    /// ```
    pub fn file_icon(path: &str) -> &'static str {
        let filename = Path::new(path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(path);

        FileType::from_filename(filename).icon()
    }

    /// Get icon for a folder
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Icons;
    ///
    /// assert_eq!(Icons::folder_icon(false), "📁");
    /// assert_eq!(Icons::folder_icon(true), "📂");
    /// ```
    pub const fn folder_icon(open: bool) -> &'static str {
        if open { "📂" } else { "📁" }
    }

    /// Get icon for a specific folder name
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Icons;
    ///
    /// assert_eq!(Icons::special_folder_icon("src"), "📁");
    /// assert_eq!(Icons::special_folder_icon("docs"), "📁");
    /// ```
    pub fn special_folder_icon(name: &str) -> &'static str {
        match name.to_lowercase().as_str() {
            "src" | "source" => "📁",
            "test" | "tests" | "__tests__" => "📁",
            "docs" | "doc" | "documentation" => "📁",
            ".git" => "📁",
            "node_modules" => "📁",
            "target" => "📁",
            "build" | "dist" => "📁",
            "config" | "conf" => "📁",
            _ => "📁",
        }
    }

    /// Get icon with filename for display
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Icons;
    ///
    /// let display = Icons::with_icon("main.rs");
    /// assert_eq!(display, "🦀 main.rs");
    /// ```
    pub fn with_icon(filename: &str) -> String {
        format!("{} {}", Self::file_icon(filename), filename)
    }

    /// Get folder with icon for display
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Icons;
    ///
    /// let display = Icons::folder_with_icon("src", false);
    /// assert_eq!(display, "📁 src");
    /// ```
    pub fn folder_with_icon(name: &str, _open: bool) -> String {
        let icon = Self::special_folder_icon(name);
        format!("{} {}", icon, name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_type_from_extension() {
        assert_eq!(FileType::from_extension("rs"), FileType::Rust);
        assert_eq!(FileType::from_extension("js"), FileType::JavaScript);
        assert_eq!(FileType::from_extension("ts"), FileType::TypeScript);
        assert_eq!(FileType::from_extension("py"), FileType::Python);
        assert_eq!(FileType::from_extension("go"), FileType::Go);
    }

    #[test]
    fn test_file_type_from_filename() {
        assert_eq!(FileType::from_filename("main.rs"), FileType::Rust);
        assert_eq!(FileType::from_filename("app.js"), FileType::JavaScript);
        assert_eq!(FileType::from_filename("Dockerfile"), FileType::Docker);
        assert_eq!(FileType::from_filename(".gitignore"), FileType::Git);
    }

    #[test]
    fn test_file_type_icon() {
        assert_eq!(FileType::Rust.icon(), "🦀");
        assert_eq!(FileType::JavaScript.icon(), "JS");
        assert_eq!(FileType::Python.icon(), "🐍");
    }

    #[test]
    fn test_git_status_icon() {
        assert_eq!(GitStatus::Modified.icon(), "●");
        assert_eq!(GitStatus::Added.icon(), "+");
        assert_eq!(GitStatus::Deleted.icon(), "-");
    }

    #[test]
    fn test_git_status_short() {
        assert_eq!(GitStatus::Modified.short(), "M");
        assert_eq!(GitStatus::Added.short(), "A");
        assert_eq!(GitStatus::Untracked.short(), "??");
    }

    #[test]
    fn test_status_icon() {
        assert_eq!(StatusIcon::Success.icon(), "✓");
        assert_eq!(StatusIcon::Error.icon(), "✗");
        assert_eq!(StatusIcon::Warning.icon(), "⚠");
    }

    #[test]
    fn test_icons_file_icon() {
        assert_eq!(Icons::file_icon("main.rs"), "🦀");
        assert_eq!(Icons::file_icon("package.json"), "{}");
        assert_eq!(Icons::file_icon("index.html"), "🌐");
    }

    #[test]
    fn test_icons_folder_icon() {
        assert_eq!(Icons::folder_icon(false), "📁");
        assert_eq!(Icons::folder_icon(true), "📂");
    }

    #[test]
    fn test_icons_special_folder() {
        assert_eq!(Icons::special_folder_icon("src"), "📁");
        assert_eq!(Icons::special_folder_icon("test"), "📁");
        assert_eq!(Icons::special_folder_icon(".git"), "📁");
    }

    #[test]
    fn test_icons_with_icon() {
        let result = Icons::with_icon("main.rs");
        assert_eq!(result, "🦀 main.rs");
    }

    #[test]
    fn test_icons_folder_with_icon() {
        let result = Icons::folder_with_icon("src", false);
        assert_eq!(result, "📁 src");
    }

    #[test]
    fn test_multiple_extensions() {
        assert_eq!(FileType::from_extension("jsx"), FileType::JavaScript);
        assert_eq!(FileType::from_extension("tsx"), FileType::TypeScript);
        assert_eq!(FileType::from_extension("cpp"), FileType::Cpp);
    }

    #[test]
    fn test_case_insensitive() {
        assert_eq!(FileType::from_extension("RS"), FileType::Rust);
        assert_eq!(FileType::from_extension("Py"), FileType::Python);
    }

    #[test]
    fn test_unknown_extension() {
        assert_eq!(FileType::from_extension("xyz"), FileType::File);
        assert_eq!(FileType::from_extension(""), FileType::File);
    }

    #[test]
    fn test_archive_types() {
        assert_eq!(FileType::from_extension("zip"), FileType::Archive);
        assert_eq!(FileType::from_extension("tar"), FileType::Archive);
        assert_eq!(FileType::from_extension("gz"), FileType::Archive);
    }

    #[test]
    fn test_image_types() {
        assert_eq!(FileType::from_extension("png"), FileType::Image);
        assert_eq!(FileType::from_extension("jpg"), FileType::Image);
        assert_eq!(FileType::from_extension("svg"), FileType::Image);
    }

    #[test]
    fn test_config_files() {
        assert_eq!(FileType::from_extension("json"), FileType::Json);
        assert_eq!(FileType::from_extension("yaml"), FileType::Yaml);
        assert_eq!(FileType::from_extension("toml"), FileType::Toml);
    }

    #[test]
    fn test_special_filenames() {
        assert_eq!(FileType::from_filename("Dockerfile"), FileType::Docker);
        assert_eq!(FileType::from_filename("Dockerfile.dev"), FileType::Docker);
    }

    #[test]
    fn test_all_git_statuses() {
        let statuses = [
            GitStatus::Untracked,
            GitStatus::Modified,
            GitStatus::Added,
            GitStatus::Deleted,
            GitStatus::Renamed,
            GitStatus::Conflict,
            GitStatus::Ignored,
            GitStatus::Clean,
        ];

        for status in &statuses {
            // All statuses should have icons and short representations
            let icon = status.icon();
            let short = status.short();
            assert!(
                !icon.is_empty() || !short.is_empty(),
                "Status {:?} should have icon or short",
                status
            );
        }
    }

    #[test]
    fn test_all_status_icons() {
        let icons = [
            StatusIcon::Success,
            StatusIcon::Error,
            StatusIcon::Warning,
            StatusIcon::Info,
            StatusIcon::Loading,
            StatusIcon::Check,
            StatusIcon::Cross,
            StatusIcon::Question,
            StatusIcon::Hint,
            StatusIcon::Locked,
            StatusIcon::Unlocked,
        ];

        for icon in &icons {
            // All status icons should have representations
            assert!(
                !icon.icon().is_empty(),
                "Icon {:?} should have representation",
                icon
            );
        }
    }

    #[test]
    fn test_path_with_directory() {
        assert_eq!(Icons::file_icon("src/main.rs"), "🦀");
        assert_eq!(Icons::file_icon("lib/utils.js"), "JS");
    }

    #[test]
    fn test_folder_names() {
        assert_eq!(Icons::special_folder_icon("node_modules"), "📁");
        assert_eq!(Icons::special_folder_icon("target"), "📁");
        assert_eq!(Icons::special_folder_icon("docs"), "📁");
    }
}
