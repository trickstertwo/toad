//! Git Card Integration
//!
//! Links Kanban cards to git commits, branches, and pull requests.
//! Enables code review workflow integration with task management.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Type of git entity linked to a card
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GitEntityType {
    /// Git commit
    Commit,
    /// Git branch
    Branch,
    /// Pull request
    PullRequest,
    /// Git tag
    Tag,
}

impl GitEntityType {
    /// Get the display name for the entity type
    pub fn name(&self) -> &'static str {
        match self {
            GitEntityType::Commit => "Commit",
            GitEntityType::Branch => "Branch",
            GitEntityType::PullRequest => "Pull Request",
            GitEntityType::Tag => "Tag",
        }
    }
}

/// Link between a card and a git entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitCardLink {
    /// Unique identifier for the link
    pub id: String,
    /// Card ID
    pub card_id: String,
    /// Type of git entity
    pub entity_type: GitEntityType,
    /// Git entity identifier (commit hash, branch name, PR number, tag name)
    pub entity_id: String,
    /// Optional repository path or identifier
    pub repository: Option<String>,
    /// When this link was created
    pub created_at: DateTime<Utc>,
    /// User who created the link
    pub created_by: String,
    /// Whether this link is still active
    pub active: bool,
}

impl GitCardLink {
    /// Create a new git card link
    pub fn new(
        id: String,
        card_id: String,
        entity_type: GitEntityType,
        entity_id: String,
        created_by: String,
    ) -> Self {
        Self {
            id,
            card_id,
            entity_type,
            entity_id,
            repository: None,
            created_at: Utc::now(),
            created_by,
            active: true,
        }
    }

    /// Set the repository for this link
    pub fn with_repository(mut self, repository: String) -> Self {
        self.repository = Some(repository);
        self
    }

    /// Deactivate this link
    pub fn deactivate(&mut self) {
        self.active = false;
    }

    /// Reactivate this link
    pub fn activate(&mut self) {
        self.active = true;
    }
}

/// Git branch associated with a card
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardBranch {
    /// Branch name
    pub name: String,
    /// Card ID this branch is associated with
    pub card_id: String,
    /// Base branch this was created from
    pub base_branch: String,
    /// Branch creation timestamp
    pub created_at: DateTime<Utc>,
    /// Creator of the branch
    pub created_by: String,
    /// Whether the branch is currently active
    pub active: bool,
    /// Whether the branch has been merged
    pub merged: bool,
    /// When the branch was merged (if applicable)
    pub merged_at: Option<DateTime<Utc>>,
    /// Number of commits on this branch
    pub commit_count: usize,
}

impl CardBranch {
    /// Create a new card branch
    pub fn new(name: String, card_id: String, base_branch: String, created_by: String) -> Self {
        Self {
            name,
            card_id,
            base_branch,
            created_at: Utc::now(),
            created_by,
            active: true,
            merged: false,
            merged_at: None,
            commit_count: 0,
        }
    }

    /// Mark branch as merged
    pub fn mark_merged(&mut self) {
        self.merged = true;
        self.merged_at = Some(Utc::now());
        self.active = false;
    }

    /// Mark branch as deleted
    pub fn mark_deleted(&mut self) {
        self.active = false;
    }

    /// Increment commit count
    pub fn add_commit(&mut self) {
        self.commit_count += 1;
    }

    /// Get branch name suggestion for a card
    pub fn suggest_branch_name(card_id: &str, card_title: &str) -> String {
        let sanitized = card_title
            .to_lowercase()
            .chars()
            .map(|c| if c.is_alphanumeric() || c == ' ' { c } else { ' ' })
            .collect::<String>()
            .split_whitespace()
            .take(5)
            .collect::<Vec<_>>()
            .join("-");

        format!("feature/{}-{}", card_id, sanitized)
    }
}

/// Commit linked to a card
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardCommit {
    /// Commit hash
    pub hash: String,
    /// Card ID this commit is linked to
    pub card_id: String,
    /// Commit message
    pub message: String,
    /// Commit author
    pub author: String,
    /// Commit timestamp
    pub committed_at: DateTime<Utc>,
    /// Branch this commit is on
    pub branch: String,
    /// Files changed in this commit
    pub files_changed: Vec<String>,
    /// Number of additions
    pub additions: usize,
    /// Number of deletions
    pub deletions: usize,
}

impl CardCommit {
    /// Create a new card commit
    pub fn new(
        hash: String,
        card_id: String,
        message: String,
        author: String,
        committed_at: DateTime<Utc>,
        branch: String,
    ) -> Self {
        Self {
            hash,
            card_id,
            message,
            author,
            committed_at,
            branch,
            files_changed: Vec::new(),
            additions: 0,
            deletions: 0,
        }
    }

    /// Add file changes to this commit
    pub fn with_changes(mut self, files: Vec<String>, additions: usize, deletions: usize) -> Self {
        self.files_changed = files;
        self.additions = additions;
        self.deletions = deletions;
        self
    }

    /// Extract card ID from commit message
    /// Looks for patterns like #CARD-123, CARD-123, [CARD-123]
    pub fn extract_card_id_from_message(message: &str) -> Option<String> {
        // Try #CARD-XXX pattern
        if let Some(idx) = message.find("#CARD-") {
            if let Some(end) = message[idx + 6..].find(|c: char| !c.is_alphanumeric()) {
                return Some(format!("CARD-{}", &message[idx + 6..idx + 6 + end]));
            } else if idx + 6 < message.len() {
                return Some(format!("CARD-{}", &message[idx + 6..]));
            }
        }

        // Try [CARD-XXX] pattern
        if let Some(idx) = message.find("[CARD-") {
            if let Some(end_idx) = message[idx..].find(']') {
                return Some(message[idx + 1..idx + end_idx].to_string());
            }
        }

        None
    }
}

/// Code review status for a card
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReviewStatus {
    /// No review requested yet
    None,
    /// Review has been requested
    Pending,
    /// Changes have been requested
    ChangesRequested,
    /// Review approved
    Approved,
    /// Merged
    Merged,
}

impl ReviewStatus {
    /// Get the display name for the review status
    pub fn name(&self) -> &'static str {
        match self {
            ReviewStatus::None => "No Review",
            ReviewStatus::Pending => "Pending",
            ReviewStatus::ChangesRequested => "Changes Requested",
            ReviewStatus::Approved => "Approved",
            ReviewStatus::Merged => "Merged",
        }
    }

    /// Get color for the status
    pub fn color(&self) -> &'static str {
        match self {
            ReviewStatus::None => "gray",
            ReviewStatus::Pending => "yellow",
            ReviewStatus::ChangesRequested => "orange",
            ReviewStatus::Approved => "green",
            ReviewStatus::Merged => "purple",
        }
    }
}

/// Code review workflow state for a card
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardReviewWorkflow {
    /// Card ID
    pub card_id: String,
    /// Current review status
    pub status: ReviewStatus,
    /// Pull request number (if applicable)
    pub pr_number: Option<String>,
    /// Reviewers assigned
    pub reviewers: Vec<String>,
    /// Review comments count
    pub comments_count: usize,
    /// When the review was requested
    pub requested_at: Option<DateTime<Utc>>,
    /// When the review was completed
    pub completed_at: Option<DateTime<Utc>>,
}

impl CardReviewWorkflow {
    /// Create a new review workflow
    pub fn new(card_id: String) -> Self {
        Self {
            card_id,
            status: ReviewStatus::None,
            pr_number: None,
            reviewers: Vec::new(),
            comments_count: 0,
            requested_at: None,
            completed_at: None,
        }
    }

    /// Request review
    pub fn request_review(&mut self, pr_number: String, reviewers: Vec<String>) {
        self.status = ReviewStatus::Pending;
        self.pr_number = Some(pr_number);
        self.reviewers = reviewers;
        self.requested_at = Some(Utc::now());
    }

    /// Mark as changes requested
    pub fn request_changes(&mut self) {
        self.status = ReviewStatus::ChangesRequested;
    }

    /// Mark as approved
    pub fn approve(&mut self) {
        self.status = ReviewStatus::Approved;
        self.completed_at = Some(Utc::now());
    }

    /// Mark as merged
    pub fn mark_merged(&mut self) {
        self.status = ReviewStatus::Merged;
        if self.completed_at.is_none() {
            self.completed_at = Some(Utc::now());
        }
    }

    /// Add a review comment
    pub fn add_comment(&mut self) {
        self.comments_count += 1;
    }
}

/// Manager for git card integration
#[derive(Debug)]
pub struct GitCardIntegrationManager {
    /// All git-card links
    links: HashMap<String, GitCardLink>,
    /// Card ID to links mapping
    card_links: HashMap<String, Vec<String>>,
    /// Branches associated with cards
    card_branches: HashMap<String, Vec<CardBranch>>,
    /// Commits associated with cards
    card_commits: HashMap<String, Vec<CardCommit>>,
    /// Review workflows for cards
    review_workflows: HashMap<String, CardReviewWorkflow>,
    /// Next link ID
    next_link_id: usize,
}

impl GitCardIntegrationManager {
    /// Create a new git card integration manager
    pub fn new() -> Self {
        Self {
            links: HashMap::new(),
            card_links: HashMap::new(),
            card_branches: HashMap::new(),
            card_commits: HashMap::new(),
            review_workflows: HashMap::new(),
            next_link_id: 1,
        }
    }

    /// Create a link between a card and a git entity
    pub fn create_link(
        &mut self,
        card_id: String,
        entity_type: GitEntityType,
        entity_id: String,
        created_by: String,
        repository: Option<String>,
    ) -> String {
        let id = format!("link-{}", self.next_link_id);
        self.next_link_id += 1;

        let mut link = GitCardLink::new(id.clone(), card_id.clone(), entity_type, entity_id, created_by);
        if let Some(repo) = repository {
            link = link.with_repository(repo);
        }

        self.card_links
            .entry(card_id)
            .or_insert_with(Vec::new)
            .push(id.clone());

        self.links.insert(id.clone(), link);
        id
    }

    /// Get a link by ID
    pub fn get_link(&self, link_id: &str) -> Option<&GitCardLink> {
        self.links.get(link_id)
    }

    /// Get all links for a card
    pub fn get_card_links(&self, card_id: &str) -> Vec<&GitCardLink> {
        self.card_links
            .get(card_id)
            .map(|link_ids| {
                link_ids
                    .iter()
                    .filter_map(|id| self.links.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get active links for a card
    pub fn get_active_card_links(&self, card_id: &str) -> Vec<&GitCardLink> {
        self.get_card_links(card_id)
            .into_iter()
            .filter(|link| link.active)
            .collect()
    }

    /// Get links by entity type for a card
    pub fn get_card_links_by_type(&self, card_id: &str, entity_type: GitEntityType) -> Vec<&GitCardLink> {
        self.get_card_links(card_id)
            .into_iter()
            .filter(|link| link.entity_type == entity_type)
            .collect()
    }

    /// Deactivate a link
    pub fn deactivate_link(&mut self, link_id: &str) -> Result<(), String> {
        if let Some(link) = self.links.get_mut(link_id) {
            link.deactivate();
            Ok(())
        } else {
            Err(format!("Link {} not found", link_id))
        }
    }

    /// Create a branch for a card
    pub fn create_branch(&mut self, card_id: String, base_branch: String, card_title: &str, created_by: String) -> CardBranch {
        let branch_name = CardBranch::suggest_branch_name(&card_id, card_title);
        let branch = CardBranch::new(branch_name, card_id.clone(), base_branch, created_by);

        self.card_branches
            .entry(card_id)
            .or_insert_with(Vec::new)
            .push(branch.clone());

        branch
    }

    /// Get branches for a card
    pub fn get_card_branches(&self, card_id: &str) -> Vec<&CardBranch> {
        self.card_branches
            .get(card_id)
            .map(|branches| branches.iter().collect())
            .unwrap_or_default()
    }

    /// Get active branches for a card
    pub fn get_active_card_branches(&self, card_id: &str) -> Vec<&CardBranch> {
        self.get_card_branches(card_id)
            .into_iter()
            .filter(|branch| branch.active)
            .collect()
    }

    /// Mark a branch as merged
    pub fn mark_branch_merged(&mut self, card_id: &str, branch_name: &str) -> Result<(), String> {
        if let Some(branches) = self.card_branches.get_mut(card_id) {
            if let Some(branch) = branches.iter_mut().find(|b| b.name == branch_name) {
                branch.mark_merged();
                Ok(())
            } else {
                Err(format!("Branch {} not found for card {}", branch_name, card_id))
            }
        } else {
            Err(format!("No branches found for card {}", card_id))
        }
    }

    /// Add a commit to a card
    pub fn add_commit(&mut self, commit: CardCommit) {
        let card_id = commit.card_id.clone();
        self.card_commits
            .entry(card_id)
            .or_insert_with(Vec::new)
            .push(commit);
    }

    /// Get commits for a card
    pub fn get_card_commits(&self, card_id: &str) -> Vec<&CardCommit> {
        self.card_commits
            .get(card_id)
            .map(|commits| commits.iter().collect())
            .unwrap_or_default()
    }

    /// Get commits for a card on a specific branch
    pub fn get_card_commits_on_branch(&self, card_id: &str, branch: &str) -> Vec<&CardCommit> {
        self.get_card_commits(card_id)
            .into_iter()
            .filter(|commit| commit.branch == branch)
            .collect()
    }

    /// Get total changes (additions + deletions) for a card
    pub fn get_card_total_changes(&self, card_id: &str) -> (usize, usize) {
        let commits = self.get_card_commits(card_id);
        let additions = commits.iter().map(|c| c.additions).sum();
        let deletions = commits.iter().map(|c| c.deletions).sum();
        (additions, deletions)
    }

    /// Create or get review workflow for a card
    pub fn get_or_create_review_workflow(&mut self, card_id: String) -> &mut CardReviewWorkflow {
        self.review_workflows
            .entry(card_id.clone())
            .or_insert_with(|| CardReviewWorkflow::new(card_id))
    }

    /// Get review workflow for a card
    pub fn get_review_workflow(&self, card_id: &str) -> Option<&CardReviewWorkflow> {
        self.review_workflows.get(card_id)
    }

    /// Request review for a card
    pub fn request_review(&mut self, card_id: String, pr_number: String, reviewers: Vec<String>) {
        let workflow = self.get_or_create_review_workflow(card_id);
        workflow.request_review(pr_number, reviewers);
    }

    /// Approve review for a card
    pub fn approve_review(&mut self, card_id: &str) -> Result<(), String> {
        if let Some(workflow) = self.review_workflows.get_mut(card_id) {
            workflow.approve();
            Ok(())
        } else {
            Err(format!("No review workflow found for card {}", card_id))
        }
    }

    /// Mark review as merged
    pub fn mark_review_merged(&mut self, card_id: &str) -> Result<(), String> {
        if let Some(workflow) = self.review_workflows.get_mut(card_id) {
            workflow.mark_merged();
            Ok(())
        } else {
            Err(format!("No review workflow found for card {}", card_id))
        }
    }

    /// Get all cards with active reviews
    pub fn get_cards_in_review(&self) -> Vec<&CardReviewWorkflow> {
        self.review_workflows
            .values()
            .filter(|w| matches!(w.status, ReviewStatus::Pending | ReviewStatus::ChangesRequested))
            .collect()
    }

    /// Get all cards with approved reviews
    pub fn get_cards_approved(&self) -> Vec<&CardReviewWorkflow> {
        self.review_workflows
            .values()
            .filter(|w| w.status == ReviewStatus::Approved)
            .collect()
    }
}

impl Default for GitCardIntegrationManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_git_entity_type_name() {
        assert_eq!(GitEntityType::Commit.name(), "Commit");
        assert_eq!(GitEntityType::Branch.name(), "Branch");
        assert_eq!(GitEntityType::PullRequest.name(), "Pull Request");
        assert_eq!(GitEntityType::Tag.name(), "Tag");
    }

    #[test]
    fn test_git_card_link_creation() {
        let link = GitCardLink::new(
            "link-1".to_string(),
            "card-123".to_string(),
            GitEntityType::Commit,
            "abc123".to_string(),
            "user".to_string(),
        );

        assert_eq!(link.id, "link-1");
        assert_eq!(link.card_id, "card-123");
        assert_eq!(link.entity_type, GitEntityType::Commit);
        assert_eq!(link.entity_id, "abc123");
        assert_eq!(link.created_by, "user");
        assert!(link.active);
    }

    #[test]
    fn test_git_card_link_with_repository() {
        let link = GitCardLink::new(
            "link-1".to_string(),
            "card-123".to_string(),
            GitEntityType::Branch,
            "main".to_string(),
            "user".to_string(),
        )
        .with_repository("repo1".to_string());

        assert_eq!(link.repository, Some("repo1".to_string()));
    }

    #[test]
    fn test_git_card_link_deactivate() {
        let mut link = GitCardLink::new(
            "link-1".to_string(),
            "card-123".to_string(),
            GitEntityType::Commit,
            "abc123".to_string(),
            "user".to_string(),
        );

        assert!(link.active);
        link.deactivate();
        assert!(!link.active);
    }

    #[test]
    fn test_card_branch_creation() {
        let branch = CardBranch::new(
            "feature/card-123-login".to_string(),
            "card-123".to_string(),
            "main".to_string(),
            "user".to_string(),
        );

        assert_eq!(branch.name, "feature/card-123-login");
        assert_eq!(branch.card_id, "card-123");
        assert_eq!(branch.base_branch, "main");
        assert!(branch.active);
        assert!(!branch.merged);
        assert_eq!(branch.commit_count, 0);
    }

    #[test]
    fn test_card_branch_suggest_name() {
        let name = CardBranch::suggest_branch_name("CARD-123", "Add user login feature");
        assert_eq!(name, "feature/CARD-123-add-user-login-feature");

        let name2 = CardBranch::suggest_branch_name("CARD-456", "Fix: bug in payment (critical)");
        assert_eq!(name2, "feature/CARD-456-fix-bug-in-payment-critical");
    }

    #[test]
    fn test_card_branch_mark_merged() {
        let mut branch = CardBranch::new(
            "feature/test".to_string(),
            "card-1".to_string(),
            "main".to_string(),
            "user".to_string(),
        );

        branch.mark_merged();
        assert!(branch.merged);
        assert!(!branch.active);
        assert!(branch.merged_at.is_some());
    }

    #[test]
    fn test_card_commit_creation() {
        let commit = CardCommit::new(
            "abc123".to_string(),
            "card-123".to_string(),
            "Add login feature".to_string(),
            "user".to_string(),
            Utc::now(),
            "main".to_string(),
        );

        assert_eq!(commit.hash, "abc123");
        assert_eq!(commit.card_id, "card-123");
        assert_eq!(commit.message, "Add login feature");
        assert_eq!(commit.branch, "main");
    }

    #[test]
    fn test_card_commit_with_changes() {
        let commit = CardCommit::new(
            "abc123".to_string(),
            "card-123".to_string(),
            "Add login".to_string(),
            "user".to_string(),
            Utc::now(),
            "main".to_string(),
        )
        .with_changes(vec!["auth.rs".to_string(), "main.rs".to_string()], 50, 10);

        assert_eq!(commit.files_changed.len(), 2);
        assert_eq!(commit.additions, 50);
        assert_eq!(commit.deletions, 10);
    }

    #[test]
    fn test_card_commit_extract_card_id_from_message() {
        assert_eq!(
            CardCommit::extract_card_id_from_message("Fix login #CARD-123"),
            Some("CARD-123".to_string())
        );

        assert_eq!(
            CardCommit::extract_card_id_from_message("[CARD-456] Add feature"),
            Some("CARD-456".to_string())
        );

        assert_eq!(CardCommit::extract_card_id_from_message("Regular commit message"), None);
    }

    #[test]
    fn test_review_status_name() {
        assert_eq!(ReviewStatus::None.name(), "No Review");
        assert_eq!(ReviewStatus::Pending.name(), "Pending");
        assert_eq!(ReviewStatus::ChangesRequested.name(), "Changes Requested");
        assert_eq!(ReviewStatus::Approved.name(), "Approved");
        assert_eq!(ReviewStatus::Merged.name(), "Merged");
    }

    #[test]
    fn test_review_status_color() {
        assert_eq!(ReviewStatus::None.color(), "gray");
        assert_eq!(ReviewStatus::Pending.color(), "yellow");
        assert_eq!(ReviewStatus::ChangesRequested.color(), "orange");
        assert_eq!(ReviewStatus::Approved.color(), "green");
        assert_eq!(ReviewStatus::Merged.color(), "purple");
    }

    #[test]
    fn test_card_review_workflow_creation() {
        let workflow = CardReviewWorkflow::new("card-123".to_string());

        assert_eq!(workflow.card_id, "card-123");
        assert_eq!(workflow.status, ReviewStatus::None);
        assert_eq!(workflow.pr_number, None);
        assert_eq!(workflow.reviewers.len(), 0);
    }

    #[test]
    fn test_card_review_workflow_request_review() {
        let mut workflow = CardReviewWorkflow::new("card-123".to_string());

        workflow.request_review("PR-1".to_string(), vec!["reviewer1".to_string(), "reviewer2".to_string()]);

        assert_eq!(workflow.status, ReviewStatus::Pending);
        assert_eq!(workflow.pr_number, Some("PR-1".to_string()));
        assert_eq!(workflow.reviewers.len(), 2);
        assert!(workflow.requested_at.is_some());
    }

    #[test]
    fn test_card_review_workflow_approve() {
        let mut workflow = CardReviewWorkflow::new("card-123".to_string());
        workflow.request_review("PR-1".to_string(), vec!["reviewer1".to_string()]);

        workflow.approve();

        assert_eq!(workflow.status, ReviewStatus::Approved);
        assert!(workflow.completed_at.is_some());
    }

    #[test]
    fn test_manager_create_link() {
        let mut manager = GitCardIntegrationManager::new();

        let link_id = manager.create_link(
            "card-123".to_string(),
            GitEntityType::Commit,
            "abc123".to_string(),
            "user".to_string(),
            None,
        );

        assert_eq!(link_id, "link-1");
        assert!(manager.get_link(&link_id).is_some());
    }

    #[test]
    fn test_manager_get_card_links() {
        let mut manager = GitCardIntegrationManager::new();

        manager.create_link(
            "card-123".to_string(),
            GitEntityType::Commit,
            "abc123".to_string(),
            "user".to_string(),
            None,
        );
        manager.create_link(
            "card-123".to_string(),
            GitEntityType::Branch,
            "main".to_string(),
            "user".to_string(),
            None,
        );

        let links = manager.get_card_links("card-123");
        assert_eq!(links.len(), 2);
    }

    #[test]
    fn test_manager_get_card_links_by_type() {
        let mut manager = GitCardIntegrationManager::new();

        manager.create_link(
            "card-123".to_string(),
            GitEntityType::Commit,
            "abc123".to_string(),
            "user".to_string(),
            None,
        );
        manager.create_link(
            "card-123".to_string(),
            GitEntityType::Branch,
            "main".to_string(),
            "user".to_string(),
            None,
        );
        manager.create_link(
            "card-123".to_string(),
            GitEntityType::Commit,
            "def456".to_string(),
            "user".to_string(),
            None,
        );

        let commit_links = manager.get_card_links_by_type("card-123", GitEntityType::Commit);
        assert_eq!(commit_links.len(), 2);

        let branch_links = manager.get_card_links_by_type("card-123", GitEntityType::Branch);
        assert_eq!(branch_links.len(), 1);
    }

    #[test]
    fn test_manager_create_branch() {
        let mut manager = GitCardIntegrationManager::new();

        let branch = manager.create_branch(
            "card-123".to_string(),
            "main".to_string(),
            "Add login feature",
            "user".to_string(),
        );

        assert_eq!(branch.name, "feature/card-123-add-login-feature");

        let branches = manager.get_card_branches("card-123");
        assert_eq!(branches.len(), 1);
    }

    #[test]
    fn test_manager_mark_branch_merged() {
        let mut manager = GitCardIntegrationManager::new();

        let branch = manager.create_branch(
            "card-123".to_string(),
            "main".to_string(),
            "Add feature",
            "user".to_string(),
        );

        let result = manager.mark_branch_merged("card-123", &branch.name);
        assert!(result.is_ok());

        let branches = manager.get_card_branches("card-123");
        assert!(branches[0].merged);
    }

    #[test]
    fn test_manager_add_commit() {
        let mut manager = GitCardIntegrationManager::new();

        let commit = CardCommit::new(
            "abc123".to_string(),
            "card-123".to_string(),
            "Fix bug".to_string(),
            "user".to_string(),
            Utc::now(),
            "main".to_string(),
        );

        manager.add_commit(commit);

        let commits = manager.get_card_commits("card-123");
        assert_eq!(commits.len(), 1);
    }

    #[test]
    fn test_manager_get_card_total_changes() {
        let mut manager = GitCardIntegrationManager::new();

        let commit1 = CardCommit::new(
            "abc123".to_string(),
            "card-123".to_string(),
            "Commit 1".to_string(),
            "user".to_string(),
            Utc::now(),
            "main".to_string(),
        )
        .with_changes(vec!["file1.rs".to_string()], 50, 10);

        let commit2 = CardCommit::new(
            "def456".to_string(),
            "card-123".to_string(),
            "Commit 2".to_string(),
            "user".to_string(),
            Utc::now(),
            "main".to_string(),
        )
        .with_changes(vec!["file2.rs".to_string()], 30, 5);

        manager.add_commit(commit1);
        manager.add_commit(commit2);

        let (additions, deletions) = manager.get_card_total_changes("card-123");
        assert_eq!(additions, 80);
        assert_eq!(deletions, 15);
    }

    #[test]
    fn test_manager_request_review() {
        let mut manager = GitCardIntegrationManager::new();

        manager.request_review(
            "card-123".to_string(),
            "PR-1".to_string(),
            vec!["reviewer1".to_string()],
        );

        let workflow = manager.get_review_workflow("card-123").unwrap();
        assert_eq!(workflow.status, ReviewStatus::Pending);
        assert_eq!(workflow.pr_number, Some("PR-1".to_string()));
    }

    #[test]
    fn test_manager_approve_review() {
        let mut manager = GitCardIntegrationManager::new();

        manager.request_review("card-123".to_string(), "PR-1".to_string(), vec!["reviewer1".to_string()]);

        let result = manager.approve_review("card-123");
        assert!(result.is_ok());

        let workflow = manager.get_review_workflow("card-123").unwrap();
        assert_eq!(workflow.status, ReviewStatus::Approved);
    }

    #[test]
    fn test_manager_get_cards_in_review() {
        let mut manager = GitCardIntegrationManager::new();

        manager.request_review("card-1".to_string(), "PR-1".to_string(), vec![]);
        manager.request_review("card-2".to_string(), "PR-2".to_string(), vec![]);
        manager.request_review("card-3".to_string(), "PR-3".to_string(), vec![]);

        manager.approve_review("card-2").unwrap();

        let in_review = manager.get_cards_in_review();
        assert_eq!(in_review.len(), 2);
    }

    #[test]
    fn test_manager_get_cards_approved() {
        let mut manager = GitCardIntegrationManager::new();

        manager.request_review("card-1".to_string(), "PR-1".to_string(), vec![]);
        manager.request_review("card-2".to_string(), "PR-2".to_string(), vec![]);

        manager.approve_review("card-1").unwrap();

        let approved = manager.get_cards_approved();
        assert_eq!(approved.len(), 1);
        assert_eq!(approved[0].card_id, "card-1");
    }
}
