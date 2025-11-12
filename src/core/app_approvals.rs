//! Approval system for dangerous AI operations
//!
//! Pauses execution before dangerous operations and requests user approval.
//!
//! # Safety Features
//!
//! - Risk level classification (Low/Medium/High)
//! - File diff previews for write operations
//! - Command validation for bash operations
//! - Never auto-approve file deletions or destructive commands
//! - Trust mode for experienced users (excludes HIGH risk)
//!
//! # Examples
//!
//! ```no_run
//! use toad::core::app_approvals::{ApprovalManager, ApprovalRequest, RiskLevel};
//! use std::path::PathBuf;
//!
//! let mut manager = ApprovalManager::new();
//! // let result = manager.request_approval(request).await;
//! ```

use std::path::PathBuf;

/// Risk level for an operation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RiskLevel {
    /// Low risk: read operations, tests, non-destructive commands
    Low,
    /// Medium risk: writes, non-destructive commands
    Medium,
    /// High risk: rm, git reset --hard, file deletions, etc.
    High,
}

impl RiskLevel {
    /// Get color for this risk level
    pub fn color(&self) -> ratatui::style::Color {
        match self {
            RiskLevel::Low => crate::ui::theme::ToadTheme::TOAD_GREEN,
            RiskLevel::Medium => crate::ui::theme::ToadTheme::YELLOW,
            RiskLevel::High => crate::ui::theme::ToadTheme::RED,
        }
    }

    /// Get label for this risk level
    pub fn label(&self) -> &'static str {
        match self {
            RiskLevel::Low => "LOW",
            RiskLevel::Medium => "MEDIUM",
            RiskLevel::High => "HIGH",
        }
    }
}

/// Type of operation requesting approval
#[derive(Debug, Clone)]
pub enum ApprovalRequest {
    /// Write or modify a file
    WriteFile {
        /// Path to the file
        path: PathBuf,
        /// Content to write
        content: String,
        /// Whether this is a new file
        is_new: bool,
        /// Risk level
        risk: RiskLevel,
        /// Previous content (for diff display)
        previous_content: Option<String>,
    },
    /// Execute a bash command
    BashCommand {
        /// Command to execute
        command: String,
        /// Working directory
        working_dir: PathBuf,
        /// Risk level
        risk: RiskLevel,
    },
    /// Create a git commit
    GitCommit {
        /// Commit message
        message: String,
        /// Files to commit
        files: Vec<PathBuf>,
    },
}

impl ApprovalRequest {
    /// Get the risk level for this request
    pub fn risk(&self) -> RiskLevel {
        match self {
            ApprovalRequest::WriteFile { risk, .. } => *risk,
            ApprovalRequest::BashCommand { risk, .. } => *risk,
            ApprovalRequest::GitCommit { .. } => RiskLevel::Medium, // Git commits are medium risk
        }
    }

    /// Get a summary of this request
    pub fn summary(&self) -> String {
        match self {
            ApprovalRequest::WriteFile { path, is_new, .. } => {
                if *is_new {
                    format!("Create new file: {}", path.display())
                } else {
                    format!("Modify file: {}", path.display())
                }
            }
            ApprovalRequest::BashCommand { command, .. } => {
                format!("Execute command: {}", command)
            }
            ApprovalRequest::GitCommit { message, files } => {
                format!(
                    "Commit {} file(s): {}",
                    files.len(),
                    message.lines().next().unwrap_or("")
                )
            }
        }
    }

    /// Check if this is a destructive operation
    pub fn is_destructive(&self) -> bool {
        match self {
            ApprovalRequest::WriteFile { .. } => false, // File writes are not destructive (can be undone)
            ApprovalRequest::BashCommand { command, .. } => {
                Self::is_destructive_command(command)
            }
            ApprovalRequest::GitCommit { .. } => false, // Commits can be undone
        }
    }

    /// Check if a bash command is destructive
    fn is_destructive_command(command: &str) -> bool {
        let lower = command.to_lowercase();

        // Check for dangerous commands
        lower.contains("rm -rf")
            || lower.contains("rm -fr")
            || lower.starts_with("rm ")
            || lower.contains("git reset --hard")
            || lower.contains("git clean -fdx")
            || lower.contains("mkfs")
            || lower.contains("dd if=")
            || lower.contains("> /dev/")
    }

    /// Classify risk level for a file write operation
    pub fn classify_write_risk(path: &PathBuf, is_deletion: bool) -> RiskLevel {
        if is_deletion {
            return RiskLevel::High; // File deletions are high risk
        }

        let path_str = path.to_string_lossy().to_lowercase();

        // High risk: system files, config files
        if path_str.contains("/etc/")
            || path_str.contains("passwd")
            || path_str.contains("shadow")
            || path_str.contains(".bashrc")
            || path_str.contains(".zshrc")
        {
            return RiskLevel::High;
        }

        // Medium risk: source files, docs
        if path_str.ends_with(".rs")
            || path_str.ends_with(".py")
            || path_str.ends_with(".js")
            || path_str.ends_with(".ts")
            || path_str.ends_with(".go")
            || path_str.ends_with(".md")
        {
            return RiskLevel::Medium;
        }

        // Low risk: test files, temp files
        if path_str.contains("test")
            || path_str.contains("/tmp/")
            || path_str.ends_with(".txt")
            || path_str.ends_with(".log")
        {
            return RiskLevel::Low;
        }

        // Default to medium
        RiskLevel::Medium
    }

    /// Classify risk level for a bash command
    pub fn classify_bash_risk(command: &str) -> RiskLevel {
        if Self::is_destructive_command(command) {
            return RiskLevel::High;
        }

        let lower = command.to_lowercase();

        // High risk: package managers, sudo, chmod
        if lower.starts_with("sudo ")
            || lower.contains("apt ")
            || lower.contains("yum ")
            || lower.contains("chmod ")
            || lower.contains("chown ")
        {
            return RiskLevel::High;
        }

        // Low risk: read operations
        if lower.starts_with("cat ")
            || lower.starts_with("ls ")
            || lower.starts_with("echo ")
            || lower.starts_with("grep ")
            || lower.starts_with("find ")
        {
            return RiskLevel::Low;
        }

        // Default to medium
        RiskLevel::Medium
    }
}

/// Result of an approval request
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApprovalResult {
    /// User approved the operation
    Approved,
    /// User rejected the operation
    Rejected,
    /// User wants to edit before applying
    Edit,
    /// User wants to view full details
    ViewDetails,
}

/// Approval manager state
#[derive(Debug)]
pub struct ApprovalManager {
    /// Whether trust mode is enabled (auto-approve non-HIGH risk)
    trust_mode: bool,
    /// Currently pending approval request
    pending: Option<ApprovalRequest>,
}

impl ApprovalManager {
    /// Create a new approval manager
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::core::app_approvals::ApprovalManager;
    ///
    /// let manager = ApprovalManager::new();
    /// assert!(!manager.is_trust_mode());
    /// ```
    pub fn new() -> Self {
        Self {
            trust_mode: false,
            pending: None,
        }
    }

    /// Check if trust mode is enabled
    pub fn is_trust_mode(&self) -> bool {
        self.trust_mode
    }

    /// Enable or disable trust mode
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::core::app_approvals::ApprovalManager;
    ///
    /// let mut manager = ApprovalManager::new();
    /// manager.set_trust_mode(true);
    /// assert!(manager.is_trust_mode());
    /// ```
    pub fn set_trust_mode(&mut self, enabled: bool) {
        self.trust_mode = enabled;
    }

    /// Toggle trust mode
    pub fn toggle_trust_mode(&mut self) {
        self.trust_mode = !self.trust_mode;
    }

    /// Get the currently pending request
    pub fn pending_request(&self) -> Option<&ApprovalRequest> {
        self.pending.as_ref()
    }

    /// Set the pending request
    pub fn set_pending(&mut self, request: ApprovalRequest) {
        self.pending = Some(request);
    }

    /// Clear the pending request
    pub fn clear_pending(&mut self) {
        self.pending = None;
    }

    /// Check if approval should be auto-granted
    ///
    /// # Auto-Approval Rules
    ///
    /// - If trust mode is OFF: never auto-approve
    /// - If trust mode is ON: auto-approve LOW and MEDIUM risk
    /// - HIGH risk operations always require approval
    /// - Destructive operations always require approval
    pub fn should_auto_approve(&self, request: &ApprovalRequest) -> bool {
        if !self.trust_mode {
            return false;
        }

        if request.is_destructive() {
            return false; // Never auto-approve destructive operations
        }

        match request.risk() {
            RiskLevel::Low | RiskLevel::Medium => true,
            RiskLevel::High => false, // High risk always requires approval
        }
    }
}

impl Default for ApprovalManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_approval_manager_new() {
        let manager = ApprovalManager::new();
        assert!(!manager.is_trust_mode());
        assert!(manager.pending_request().is_none());
    }

    #[test]
    fn test_trust_mode() {
        let mut manager = ApprovalManager::new();
        manager.set_trust_mode(true);
        assert!(manager.is_trust_mode());

        manager.toggle_trust_mode();
        assert!(!manager.is_trust_mode());
    }

    #[test]
    fn test_classify_write_risk() {
        assert_eq!(
            ApprovalRequest::classify_write_risk(&PathBuf::from("/tmp/test.txt"), false),
            RiskLevel::Low
        );

        assert_eq!(
            ApprovalRequest::classify_write_risk(&PathBuf::from("src/main.rs"), false),
            RiskLevel::Medium
        );

        assert_eq!(
            ApprovalRequest::classify_write_risk(&PathBuf::from("/etc/passwd"), false),
            RiskLevel::High
        );

        assert_eq!(
            ApprovalRequest::classify_write_risk(&PathBuf::from("any_file.txt"), true),
            RiskLevel::High
        );
    }

    #[test]
    fn test_classify_bash_risk() {
        assert_eq!(
            ApprovalRequest::classify_bash_risk("cat file.txt"),
            RiskLevel::Low
        );

        assert_eq!(
            ApprovalRequest::classify_bash_risk("npm install"),
            RiskLevel::Medium
        );

        assert_eq!(
            ApprovalRequest::classify_bash_risk("sudo apt install foo"),
            RiskLevel::High
        );

        assert_eq!(
            ApprovalRequest::classify_bash_risk("rm -rf /"),
            RiskLevel::High
        );
    }

    #[test]
    fn test_is_destructive_command() {
        assert!(ApprovalRequest::is_destructive_command("rm -rf /tmp"));
        assert!(ApprovalRequest::is_destructive_command("git reset --hard"));
        assert!(!ApprovalRequest::is_destructive_command("cat file.txt"));
        assert!(!ApprovalRequest::is_destructive_command("npm test"));
    }

    #[test]
    fn test_approval_request_risk() {
        let request = ApprovalRequest::BashCommand {
            command: "cat file.txt".to_string(),
            working_dir: PathBuf::from("/tmp"),
            risk: RiskLevel::Low,
        };
        assert_eq!(request.risk(), RiskLevel::Low);
    }

    #[test]
    fn test_approval_request_summary() {
        let request = ApprovalRequest::WriteFile {
            path: PathBuf::from("test.txt"),
            content: "Hello".to_string(),
            is_new: true,
            risk: RiskLevel::Low,
            previous_content: None,
        };

        let summary = request.summary();
        assert!(summary.contains("Create new file"));
        assert!(summary.contains("test.txt"));
    }

    #[test]
    fn test_should_auto_approve() {
        let mut manager = ApprovalManager::new();

        let low_risk = ApprovalRequest::BashCommand {
            command: "cat file.txt".to_string(),
            working_dir: PathBuf::from("/tmp"),
            risk: RiskLevel::Low,
        };

        // Trust mode off: never auto-approve
        assert!(!manager.should_auto_approve(&low_risk));

        // Trust mode on: auto-approve low/medium risk
        manager.set_trust_mode(true);
        assert!(manager.should_auto_approve(&low_risk));

        // High risk: never auto-approve even in trust mode
        let high_risk = ApprovalRequest::BashCommand {
            command: "sudo rm -rf /".to_string(),
            working_dir: PathBuf::from("/"),
            risk: RiskLevel::High,
        };
        assert!(!manager.should_auto_approve(&high_risk));
    }

    #[test]
    fn test_pending_request_management() {
        let mut manager = ApprovalManager::new();
        assert!(manager.pending_request().is_none());

        let request = ApprovalRequest::BashCommand {
            command: "test".to_string(),
            working_dir: PathBuf::from("/tmp"),
            risk: RiskLevel::Low,
        };

        manager.set_pending(request);
        assert!(manager.pending_request().is_some());

        manager.clear_pending();
        assert!(manager.pending_request().is_none());
    }
}
