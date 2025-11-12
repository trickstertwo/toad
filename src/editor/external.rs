//! External editor integration
//!
//! Allows opening $EDITOR to compose longer prompts.

use std::env;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use uuid::Uuid;

/// Errors that can occur when using external editor
#[derive(Debug, thiserror::Error)]
pub enum EditorError {
    /// Failed to determine editor command
    #[error("No editor found. Set $EDITOR or $VISUAL environment variable.")]
    NoEditorFound,

    /// Failed to create temp file
    #[error("Failed to create temp file: {0}")]
    TempFileCreation(#[from] std::io::Error),

    /// Editor exited with non-zero status
    #[error("Editor exited with status: {0}")]
    EditorFailed(i32),

    /// Editor was cancelled or returned empty content
    #[error("Editor was cancelled or returned empty content")]
    EmptyContent,

    /// Failed to read file after editing
    #[error("Failed to read edited file: {0}")]
    ReadError(String),
}

/// Get the editor command from environment variables
///
/// Checks in order:
/// 1. $EDITOR
/// 2. $VISUAL
/// 3. Default to "vim"
///
/// # Examples
///
/// ```
/// use toad::editor::external::get_editor_command;
///
/// let editor = get_editor_command();
/// assert!(!editor.is_empty());
/// ```
pub fn get_editor_command() -> String {
    env::var("EDITOR")
        .or_else(|_| env::var("VISUAL"))
        .unwrap_or_else(|_| "vim".to_string())
}

/// Create a temporary file for editing
///
/// Creates a file at /tmp/toad-prompt-{uuid}.md with the given content.
///
/// # Parameters
///
/// - `initial_content`: The initial content to populate the file with
///
/// # Returns
///
/// The path to the created temporary file
///
/// # Errors
///
/// Returns `EditorError::TempFileCreation` if file creation fails
///
/// # Examples
///
/// ```no_run
/// use toad::editor::external::create_temp_file;
///
/// let path = create_temp_file("Hello world").unwrap();
/// assert!(path.exists());
/// ```
pub fn create_temp_file(initial_content: &str) -> Result<PathBuf, EditorError> {
    let uuid = Uuid::new_v4();
    let filename = format!("toad-prompt-{}.md", uuid);
    let temp_dir = env::temp_dir();
    let path = temp_dir.join(filename);

    let mut file = fs::File::create(&path)?;
    file.write_all(initial_content.as_bytes())?;
    file.sync_all()?;

    Ok(path)
}

/// Launch external editor and wait for completion
///
/// Opens the specified file in the user's preferred editor and waits for it to close.
///
/// # Parameters
///
/// - `editor_cmd`: The editor command to use (e.g., "vim", "nano")
/// - `file_path`: The path to the file to edit
///
/// # Returns
///
/// Ok(()) if editor exited successfully
///
/// # Errors
///
/// - `EditorError::EditorFailed` if editor exits with non-zero status
/// - `EditorError::TempFileCreation` if editor process cannot be spawned
///
/// # Examples
///
/// ```no_run
/// use toad::editor::external::launch_editor;
/// use std::path::PathBuf;
///
/// let path = PathBuf::from("/tmp/test.md");
/// launch_editor("vim", &path).unwrap();
/// ```
pub fn launch_editor(editor_cmd: &str, file_path: &PathBuf) -> Result<(), EditorError> {
    let status = Command::new(editor_cmd)
        .arg(file_path)
        .status()
        .map_err(|e| EditorError::TempFileCreation(e))?;

    if !status.success() {
        return Err(EditorError::EditorFailed(
            status.code().unwrap_or(-1)
        ));
    }

    Ok(())
}

/// Read the edited content from the file
///
/// # Parameters
///
/// - `file_path`: The path to the file to read
///
/// # Returns
///
/// The content of the file as a String
///
/// # Errors
///
/// - `EditorError::ReadError` if file cannot be read
/// - `EditorError::EmptyContent` if file is empty
///
/// # Examples
///
/// ```no_run
/// use toad::editor::external::read_edited_content;
/// use std::path::PathBuf;
///
/// let path = PathBuf::from("/tmp/test.md");
/// let content = read_edited_content(&path).unwrap();
/// assert!(!content.is_empty());
/// ```
pub fn read_edited_content(file_path: &PathBuf) -> Result<String, EditorError> {
    let content = fs::read_to_string(file_path)
        .map_err(|e| EditorError::ReadError(e.to_string()))?;

    // Trim whitespace but preserve markdown formatting
    let trimmed = content.trim();

    if trimmed.is_empty() {
        return Err(EditorError::EmptyContent);
    }

    Ok(trimmed.to_string())
}

/// Clean up temporary file
///
/// # Parameters
///
/// - `file_path`: The path to the file to delete
///
/// # Examples
///
/// ```no_run
/// use toad::editor::external::cleanup_temp_file;
/// use std::path::PathBuf;
///
/// let path = PathBuf::from("/tmp/test.md");
/// cleanup_temp_file(&path);
/// ```
pub fn cleanup_temp_file(file_path: &PathBuf) {
    // Best effort cleanup - don't propagate errors
    let _ = fs::remove_file(file_path);
}

/// Edit text with external editor
///
/// Opens the user's preferred editor ($EDITOR, $VISUAL, or vim) with the given
/// initial content. Returns the edited content after the editor closes.
///
/// # Parameters
///
/// - `initial_content`: The initial content to show in the editor
///
/// # Returns
///
/// The edited content as a String
///
/// # Errors
///
/// - `EditorError::NoEditorFound` if no editor is configured
/// - `EditorError::TempFileCreation` if temp file creation fails
/// - `EditorError::EditorFailed` if editor exits with error
/// - `EditorError::EmptyContent` if editor returns empty content
/// - `EditorError::ReadError` if edited content cannot be read
///
/// # Examples
///
/// ```no_run
/// use toad::editor::external::edit_with_external_editor;
///
/// let initial = "Hello world";
/// let edited = edit_with_external_editor(initial).unwrap();
/// assert!(!edited.is_empty());
/// ```
pub fn edit_with_external_editor(initial_content: &str) -> Result<String, EditorError> {
    // Get editor command
    let editor_cmd = get_editor_command();

    // Create temp file with initial content
    let temp_path = create_temp_file(initial_content)?;

    // Launch editor and wait for completion
    let result = launch_editor(&editor_cmd, &temp_path);

    // Read edited content if editor succeeded
    let edited_content = if result.is_ok() {
        read_edited_content(&temp_path)
    } else {
        result?;
        unreachable!()
    };

    // Clean up temp file
    cleanup_temp_file(&temp_path);

    edited_content
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_get_editor_command_default() {
        // Should return something (at least "vim")
        let editor = get_editor_command();
        assert!(!editor.is_empty());
    }

    #[test]
    fn test_get_editor_command_respects_editor_env() {
        std::env::set_var("EDITOR", "nano");
        let editor = get_editor_command();
        assert_eq!(editor, "nano");
        std::env::remove_var("EDITOR");
    }

    #[test]
    fn test_get_editor_command_respects_visual_env() {
        std::env::remove_var("EDITOR");
        std::env::set_var("VISUAL", "emacs");
        let editor = get_editor_command();
        assert_eq!(editor, "emacs");
        std::env::remove_var("VISUAL");
    }

    #[test]
    fn test_create_temp_file() {
        let content = "Test content";
        let path = create_temp_file(content).unwrap();

        assert!(path.exists());
        let read_content = fs::read_to_string(&path).unwrap();
        assert_eq!(read_content, content);

        // Cleanup
        fs::remove_file(&path).unwrap();
    }

    #[test]
    fn test_create_temp_file_path_format() {
        let path = create_temp_file("test").unwrap();
        let filename = path.file_name().unwrap().to_str().unwrap();

        assert!(filename.starts_with("toad-prompt-"));
        assert!(filename.ends_with(".md"));

        // Cleanup
        fs::remove_file(&path).unwrap();
    }

    #[test]
    fn test_read_edited_content() {
        let content = "Test content\n\nWith multiple lines\n";
        let path = create_temp_file(content).unwrap();

        let read = read_edited_content(&path).unwrap();
        assert_eq!(read, content.trim());

        // Cleanup
        fs::remove_file(&path).unwrap();
    }

    #[test]
    fn test_read_edited_content_trims_whitespace() {
        let content = "\n\n  Test content  \n\n";
        let path = create_temp_file(content).unwrap();

        let read = read_edited_content(&path).unwrap();
        assert_eq!(read, "Test content");

        // Cleanup
        fs::remove_file(&path).unwrap();
    }

    #[test]
    fn test_read_edited_content_empty_file() {
        let path = create_temp_file("").unwrap();

        let result = read_edited_content(&path);
        assert!(result.is_err());
        assert!(matches!(result, Err(EditorError::EmptyContent)));

        // Cleanup
        fs::remove_file(&path).unwrap();
    }

    #[test]
    fn test_read_edited_content_whitespace_only() {
        let path = create_temp_file("   \n\n  \n  ").unwrap();

        let result = read_edited_content(&path);
        assert!(result.is_err());
        assert!(matches!(result, Err(EditorError::EmptyContent)));

        // Cleanup
        fs::remove_file(&path).unwrap();
    }

    #[test]
    fn test_cleanup_temp_file() {
        let path = create_temp_file("test").unwrap();
        assert!(path.exists());

        cleanup_temp_file(&path);
        assert!(!path.exists());
    }

    #[test]
    fn test_cleanup_temp_file_nonexistent() {
        let path = PathBuf::from("/tmp/nonexistent-file-12345.md");
        // Should not panic
        cleanup_temp_file(&path);
    }

    #[test]
    fn test_read_edited_content_preserves_markdown() {
        let content = "# Heading\n\n**Bold** and *italic*\n\n- List item\n";
        let path = create_temp_file(content).unwrap();

        let read = read_edited_content(&path).unwrap();
        assert!(read.contains("# Heading"));
        assert!(read.contains("**Bold**"));
        assert!(read.contains("- List item"));

        // Cleanup
        fs::remove_file(&path).unwrap();
    }
}
