/// Nerd Font icon support for file types and status indicators
///
/// Provides icon lookup for common file types, folders, and status indicators
/// using Nerd Font unicode characters
///
/// # Examples
///
/// ```
/// use toad::nerd_fonts::NerdFonts;
///
/// let icon = NerdFonts::file_icon("main.rs");
/// assert_eq!(icon, "");
/// ```
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Icon provider for Nerd Fonts
#[derive(Debug, Clone, Default)]
pub struct NerdFonts;

impl NerdFonts {
    /// Get icon for file based on extension or name
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::nerd_fonts::NerdFonts;
    ///
    /// assert_eq!(NerdFonts::file_icon("main.rs"), "");
    /// assert_eq!(NerdFonts::file_icon("app.js"), "");
    /// assert_eq!(NerdFonts::file_icon("style.css"), "");
    /// ```
    pub fn file_icon<P: AsRef<Path>>(path: P) -> &'static str {
        let path = path.as_ref();

        // Check filename first for special cases
        if let Some(name) = path.file_name() {
            let name_str = name.to_string_lossy().to_lowercase();

            // Special filenames
            match name_str.as_str() {
                "readme.md" | "readme" => return "",
                "license" | "license.md" | "license.txt" => return "",
                "dockerfile" => return "",
                "docker-compose.yml" | "docker-compose.yaml" => return "",
                ".gitignore" | ".gitattributes" | ".gitmodules" => return "",
                "makefile" => return "",
                "cargo.toml" | "cargo.lock" => return "",
                "package.json" | "package-lock.json" => return "",
                ".env" | ".env.example" | ".env.local" => return "",
                _ => {}
            }
        }

        // Check extension
        if let Some(ext) = path.extension() {
            let ext_str = ext.to_string_lossy().to_lowercase();

            match ext_str.as_str() {
                // Rust
                "rs" => "",
                "toml" => "",

                // JavaScript/TypeScript
                "js" | "mjs" | "cjs" => "",
                "ts" => "",
                "jsx" => "",
                "tsx" => "",
                "json" => "",

                // Python
                "py" | "pyw" | "pyc" | "pyo" | "pyd" => "",

                // Web
                "html" | "htm" => "",
                "css" | "scss" | "sass" | "less" => "",

                // C/C++
                "c" => "",
                "h" => "",
                "cpp" | "cc" | "cxx" => "",
                "hpp" | "hh" | "hxx" => "",

                // Java/JVM
                "java" => "",
                "class" | "jar" => "",
                "kt" | "kts" => "",
                "scala" => "",

                // Go
                "go" => "",

                // Shell
                "sh" | "bash" | "zsh" | "fish" => "",

                // Ruby
                "rb" | "erb" => "",

                // PHP
                "php" => "",

                // Markup
                "xml" => "",
                "yaml" | "yml" => "",
                "md" | "markdown" => "",

                // Data
                "csv" => "",
                "sql" | "db" | "sqlite" => "",

                // Images
                "png" | "jpg" | "jpeg" | "gif" | "bmp" | "svg" | "ico" => "",

                // Documents
                "pdf" => "",
                "doc" | "docx" => "",
                "xls" | "xlsx" => "",
                "ppt" | "pptx" => "",
                "txt" => "",

                // Archives
                "zip" | "tar" | "gz" | "bz2" | "xz" | "7z" | "rar" => "",

                // Config
                "ini" | "cfg" | "conf" | "config" => "",

                // Lock files
                "lock" => "",

                // Logs
                "log" => "",

                // Git
                "git" => "",

                // Docker
                "dockerfile" => "",

                // Default
                _ => "",
            }
        } else {
            // No extension
            ""
        }
    }

    /// Get icon for directory/folder
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::nerd_fonts::NerdFonts;
    ///
    /// assert_eq!(NerdFonts::folder_icon("src", false), "");
    /// assert_eq!(NerdFonts::folder_icon("src", true), "");
    /// ```
    pub fn folder_icon(name: &str, open: bool) -> &'static str {
        let name_lower = name.to_lowercase();

        // Special folder names
        

        (match name_lower.as_str() {
            ".git" => "",
            "node_modules" => "",
            "target" => "",
            "dist" | "build" => "",
            "src" | "source" => "",
            "test" | "tests" | "__tests__" => "",
            "docs" | "doc" | "documentation" => "",
            "images" | "img" | "assets" => "",
            ".config" | "config" => "",
            ".github" => "",
            ".vscode" => "",
            "bin" => "",
            "lib" => "",
            _ => ""
        }) as _
    }

    /// Get git status icon
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::nerd_fonts::{NerdFonts, GitStatus};
    ///
    /// assert_eq!(NerdFonts::git_status_icon(GitStatus::Modified), "");
    /// assert_eq!(NerdFonts::git_status_icon(GitStatus::Added), "");
    /// ```
    pub fn git_status_icon(status: GitStatus) -> &'static str {
        match status {
            GitStatus::Unmodified => "",
            GitStatus::Modified => "",
            GitStatus::Added => "",
            GitStatus::Deleted => "",
            GitStatus::Renamed => "",
            GitStatus::Copied => "",
            GitStatus::Untracked => "",
            GitStatus::Ignored => "",
            GitStatus::Conflicted => "",
        }
    }

    /// Get UI/status icon
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::nerd_fonts::{NerdFonts, UiIcon};
    ///
    /// assert_eq!(NerdFonts::ui_icon(UiIcon::Error), "");
    /// assert_eq!(NerdFonts::ui_icon(UiIcon::Warning), "");
    /// assert_eq!(NerdFonts::ui_icon(UiIcon::Info), "");
    /// ```
    pub fn ui_icon(icon: UiIcon) -> &'static str {
        match icon {
            UiIcon::Error => "",
            UiIcon::Warning => "",
            UiIcon::Info => "",
            UiIcon::Success => "",
            UiIcon::Question => "",
            UiIcon::Search => "",
            UiIcon::Edit => "",
            UiIcon::Save => "",
            UiIcon::Delete => "",
            UiIcon::Copy => "",
            UiIcon::Paste => "",
            UiIcon::Cut => "",
            UiIcon::Undo => "",
            UiIcon::Redo => "",
            UiIcon::Settings => "",
            UiIcon::Close => "",
            UiIcon::CheckboxChecked => "",
            UiIcon::CheckboxUnchecked => "",
            UiIcon::RadioChecked => "",
            UiIcon::RadioUnchecked => "",
            UiIcon::ArrowUp => "",
            UiIcon::ArrowDown => "",
            UiIcon::ArrowLeft => "",
            UiIcon::ArrowRight => "",
            UiIcon::ChevronUp => "",
            UiIcon::ChevronDown => "",
            UiIcon::ChevronLeft => "",
            UiIcon::ChevronRight => "",
            UiIcon::Loading => "",
            UiIcon::Home => "",
            UiIcon::Folder => "",
            UiIcon::File => "",
            UiIcon::Calendar => "",
            UiIcon::Clock => "",
            UiIcon::Lock => "",
            UiIcon::Unlock => "",
            UiIcon::Star => "",
            UiIcon::StarEmpty => "",
            UiIcon::Heart => "",
            UiIcon::HeartEmpty => "",
            UiIcon::Bookmark => "",
            UiIcon::BookmarkEmpty => "",
            UiIcon::Tag => "",
            UiIcon::Plus => "",
            UiIcon::Minus => "",
            UiIcon::Multiply => "",
            UiIcon::Divide => "",
        }
    }

    /// Get icon for programming language
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::nerd_fonts::NerdFonts;
    ///
    /// assert_eq!(NerdFonts::language_icon("rust"), "");
    /// assert_eq!(NerdFonts::language_icon("javascript"), "");
    /// ```
    pub fn language_icon(language: &str) -> &'static str {
        match language.to_lowercase().as_str() {
            "rust" => "",
            "javascript" | "js" => "",
            "typescript" | "ts" => "",
            "python" | "py" => "",
            "java" => "",
            "c" => "",
            "cpp" | "c++" => "",
            "csharp" | "c#" => "",
            "go" | "golang" => "",
            "ruby" | "rb" => "",
            "php" => "",
            "swift" => "",
            "kotlin" | "kt" => "",
            "scala" => "",
            "haskell" | "hs" => "",
            "lua" => "",
            "perl" => "",
            "r" => "ﳒ",
            "elixir" => "",
            "erlang" => "",
            "clojure" => "",
            "vim" | "viml" => "",
            "html" => "",
            "css" => "",
            "shell" | "bash" | "sh" => "",
            "sql" => "",
            "markdown" | "md" => "",
            "json" => "",
            "yaml" | "yml" => "",
            "xml" => "",
            "docker" => "",
            "git" => "",
            _ => "",
        }
    }
}

/// Git file status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GitStatus {
    /// File is unmodified
    Unmodified,
    /// File is modified
    Modified,
    /// File is newly added
    Added,
    /// File is deleted
    Deleted,
    /// File is renamed
    Renamed,
    /// File is copied
    Copied,
    /// File is untracked
    Untracked,
    /// File is ignored
    Ignored,
    /// File has conflicts
    Conflicted,
}

/// UI/status icons
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiIcon {
    // Status
    Error,
    Warning,
    Info,
    Success,
    Question,

    // Actions
    Search,
    Edit,
    Save,
    Delete,
    Copy,
    Paste,
    Cut,
    Undo,
    Redo,
    Settings,
    Close,

    // Selection
    CheckboxChecked,
    CheckboxUnchecked,
    RadioChecked,
    RadioUnchecked,

    // Navigation
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    ChevronUp,
    ChevronDown,
    ChevronLeft,
    ChevronRight,

    // Other
    Loading,
    Home,
    Folder,
    File,
    Calendar,
    Clock,
    Lock,
    Unlock,
    Star,
    StarEmpty,
    Heart,
    HeartEmpty,
    Bookmark,
    BookmarkEmpty,
    Tag,
    Plus,
    Minus,
    Multiply,
    Divide,
}

/// Helper to check if terminal supports Nerd Fonts
///
/// # Examples
///
/// ```
/// use toad::nerd_fonts::supports_nerd_fonts;
///
/// if supports_nerd_fonts() {
///     println!("Nerd Fonts are supported!");
/// }
/// ```
pub fn supports_nerd_fonts() -> bool {
    // Check environment variables for common terminals that support Nerd Fonts
    std::env::var("TERM_PROGRAM")
        .map(|term| {
            matches!(
                term.as_str(),
                "iTerm.app" | "WezTerm" | "Alacritty" | "kitty" | "Ghostty"
            )
        })
        .unwrap_or(false)
        || std::env::var("TERMINAL_EMULATOR")
            .map(|term| term.contains("JetBrains"))
            .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_icon_rust() {
        assert_eq!(NerdFonts::file_icon("main.rs"), "");
        assert_eq!(NerdFonts::file_icon("lib.rs"), "");
    }

    #[test]
    fn test_file_icon_javascript() {
        assert_eq!(NerdFonts::file_icon("app.js"), "");
        assert_eq!(NerdFonts::file_icon("index.mjs"), "");
    }

    #[test]
    fn test_file_icon_typescript() {
        assert_eq!(NerdFonts::file_icon("app.ts"), "");
        assert_eq!(NerdFonts::file_icon("component.tsx"), "");
    }

    #[test]
    fn test_file_icon_python() {
        assert_eq!(NerdFonts::file_icon("script.py"), "");
    }

    #[test]
    fn test_file_icon_web() {
        assert_eq!(NerdFonts::file_icon("index.html"), "");
        assert_eq!(NerdFonts::file_icon("style.css"), "");
    }

    #[test]
    fn test_file_icon_special_names() {
        assert_eq!(NerdFonts::file_icon("README.md"), "");
        assert_eq!(NerdFonts::file_icon("Dockerfile"), "");
        assert_eq!(NerdFonts::file_icon(".gitignore"), "");
    }

    #[test]
    fn test_file_icon_unknown() {
        assert_eq!(NerdFonts::file_icon("unknown.xyz"), "");
    }

    #[test]
    fn test_folder_icon_closed() {
        assert_eq!(NerdFonts::folder_icon("src", false), "");
        assert_eq!(NerdFonts::folder_icon("regular", false), "");
    }

    #[test]
    fn test_folder_icon_open() {
        assert_eq!(NerdFonts::folder_icon("src", true), "");
        assert_eq!(NerdFonts::folder_icon("regular", true), "");
    }

    #[test]
    fn test_folder_icon_special() {
        assert_eq!(NerdFonts::folder_icon("node_modules", false), "");
        assert_eq!(NerdFonts::folder_icon(".git", false), "");
        assert_eq!(NerdFonts::folder_icon("target", false), "");
    }

    #[test]
    fn test_git_status_icons() {
        assert_eq!(NerdFonts::git_status_icon(GitStatus::Modified), "");
        assert_eq!(NerdFonts::git_status_icon(GitStatus::Added), "");
        assert_eq!(NerdFonts::git_status_icon(GitStatus::Deleted), "");
        assert_eq!(NerdFonts::git_status_icon(GitStatus::Untracked), "");
    }

    #[test]
    fn test_ui_icons() {
        assert_eq!(NerdFonts::ui_icon(UiIcon::Error), "");
        assert_eq!(NerdFonts::ui_icon(UiIcon::Warning), "");
        assert_eq!(NerdFonts::ui_icon(UiIcon::Info), "");
        assert_eq!(NerdFonts::ui_icon(UiIcon::Success), "");
    }

    #[test]
    fn test_ui_icon_actions() {
        assert_eq!(NerdFonts::ui_icon(UiIcon::Edit), "");
        assert_eq!(NerdFonts::ui_icon(UiIcon::Save), "");
        assert_eq!(NerdFonts::ui_icon(UiIcon::Delete), "");
    }

    #[test]
    fn test_ui_icon_navigation() {
        assert_eq!(NerdFonts::ui_icon(UiIcon::ArrowUp), "");
        assert_eq!(NerdFonts::ui_icon(UiIcon::ArrowDown), "");
        assert_eq!(NerdFonts::ui_icon(UiIcon::ChevronRight), "");
    }

    #[test]
    fn test_language_icons() {
        assert_eq!(NerdFonts::language_icon("rust"), "");
        assert_eq!(NerdFonts::language_icon("javascript"), "");
        assert_eq!(NerdFonts::language_icon("python"), "");
        assert_eq!(NerdFonts::language_icon("go"), "");
    }

    #[test]
    fn test_language_icon_case_insensitive() {
        assert_eq!(NerdFonts::language_icon("RUST"), "");
        assert_eq!(NerdFonts::language_icon("JavaScript"), "");
    }

    #[test]
    fn test_language_icon_unknown() {
        assert_eq!(NerdFonts::language_icon("unknown"), "");
    }

    #[test]
    fn test_git_status_all() {
        // Test all git status variants
        NerdFonts::git_status_icon(GitStatus::Unmodified);
        NerdFonts::git_status_icon(GitStatus::Modified);
        NerdFonts::git_status_icon(GitStatus::Added);
        NerdFonts::git_status_icon(GitStatus::Deleted);
        NerdFonts::git_status_icon(GitStatus::Renamed);
        NerdFonts::git_status_icon(GitStatus::Copied);
        NerdFonts::git_status_icon(GitStatus::Untracked);
        NerdFonts::git_status_icon(GitStatus::Ignored);
        NerdFonts::git_status_icon(GitStatus::Conflicted);
    }

    #[test]
    fn test_ui_icon_all_variants() {
        // Test all UI icon variants to ensure exhaustive match
        NerdFonts::ui_icon(UiIcon::CheckboxChecked);
        NerdFonts::ui_icon(UiIcon::CheckboxUnchecked);
        NerdFonts::ui_icon(UiIcon::RadioChecked);
        NerdFonts::ui_icon(UiIcon::RadioUnchecked);
        NerdFonts::ui_icon(UiIcon::Loading);
        NerdFonts::ui_icon(UiIcon::Home);
        NerdFonts::ui_icon(UiIcon::Folder);
        NerdFonts::ui_icon(UiIcon::File);
        NerdFonts::ui_icon(UiIcon::Calendar);
        NerdFonts::ui_icon(UiIcon::Clock);
        NerdFonts::ui_icon(UiIcon::Lock);
        NerdFonts::ui_icon(UiIcon::Unlock);
        NerdFonts::ui_icon(UiIcon::Star);
        NerdFonts::ui_icon(UiIcon::StarEmpty);
        NerdFonts::ui_icon(UiIcon::Heart);
        NerdFonts::ui_icon(UiIcon::HeartEmpty);
        NerdFonts::ui_icon(UiIcon::Bookmark);
        NerdFonts::ui_icon(UiIcon::BookmarkEmpty);
        NerdFonts::ui_icon(UiIcon::Tag);
        NerdFonts::ui_icon(UiIcon::Plus);
        NerdFonts::ui_icon(UiIcon::Minus);
        NerdFonts::ui_icon(UiIcon::Multiply);
        NerdFonts::ui_icon(UiIcon::Divide);
        NerdFonts::ui_icon(UiIcon::Copy);
        NerdFonts::ui_icon(UiIcon::Paste);
        NerdFonts::ui_icon(UiIcon::Cut);
        NerdFonts::ui_icon(UiIcon::Undo);
        NerdFonts::ui_icon(UiIcon::Redo);
        NerdFonts::ui_icon(UiIcon::Settings);
        NerdFonts::ui_icon(UiIcon::Close);
        NerdFonts::ui_icon(UiIcon::ArrowLeft);
        NerdFonts::ui_icon(UiIcon::ArrowRight);
        NerdFonts::ui_icon(UiIcon::ChevronUp);
        NerdFonts::ui_icon(UiIcon::ChevronDown);
        NerdFonts::ui_icon(UiIcon::ChevronLeft);
        NerdFonts::ui_icon(UiIcon::Question);
        NerdFonts::ui_icon(UiIcon::Search);
    }

    #[test]
    fn test_file_icon_archives() {
        assert_eq!(NerdFonts::file_icon("archive.zip"), "");
        assert_eq!(NerdFonts::file_icon("file.tar"), "");
        assert_eq!(NerdFonts::file_icon("file.gz"), "");
    }

    #[test]
    fn test_file_icon_images() {
        assert_eq!(NerdFonts::file_icon("photo.png"), "");
        assert_eq!(NerdFonts::file_icon("image.jpg"), "");
        assert_eq!(NerdFonts::file_icon("icon.svg"), "");
    }

    #[test]
    fn test_file_icon_documents() {
        assert_eq!(NerdFonts::file_icon("doc.pdf"), "");
        assert_eq!(NerdFonts::file_icon("note.txt"), "");
        assert_eq!(NerdFonts::file_icon("sheet.xlsx"), "");
    }

    #[test]
    fn test_file_icon_config() {
        assert_eq!(NerdFonts::file_icon("config.ini"), "");
        assert_eq!(NerdFonts::file_icon("settings.yaml"), "");
        assert_eq!(NerdFonts::file_icon("data.json"), "");
    }
}
