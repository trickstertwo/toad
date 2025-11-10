//! Project and workspace management system
//!
//! Provides multi-project support with templates, board cloning, archives,
//! and cross-board references for comprehensive project organization.
//!
//! # Examples
//!
//! ```
//! use toad::infrastructure::project_management::{ProjectManager, ProjectTemplate};
//!
//! let mut manager = ProjectManager::new();
//! manager.create_project("My App", "Main development project");
//! ```

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Project template type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProjectTemplate {
    /// Blank project
    Blank,
    /// Scrum/Agile sprint board
    Scrum,
    /// Bug tracking board
    BugTracking,
    /// Content calendar
    ContentCalendar,
    /// Personal tasks
    Personal,
    /// Team roadmap
    Roadmap,
    /// Custom template
    Custom,
}

impl ProjectTemplate {
    /// Get all template types
    pub fn all() -> &'static [ProjectTemplate] {
        &[
            ProjectTemplate::Blank,
            ProjectTemplate::Scrum,
            ProjectTemplate::BugTracking,
            ProjectTemplate::ContentCalendar,
            ProjectTemplate::Personal,
            ProjectTemplate::Roadmap,
            ProjectTemplate::Custom,
        ]
    }

    /// Get template name
    pub fn name(&self) -> &'static str {
        match self {
            ProjectTemplate::Blank => "Blank",
            ProjectTemplate::Scrum => "Scrum",
            ProjectTemplate::BugTracking => "Bug Tracking",
            ProjectTemplate::ContentCalendar => "Content Calendar",
            ProjectTemplate::Personal => "Personal",
            ProjectTemplate::Roadmap => "Roadmap",
            ProjectTemplate::Custom => "Custom",
        }
    }

    /// Get default columns for template
    pub fn default_columns(&self) -> Vec<String> {
        match self {
            ProjectTemplate::Blank => vec![],
            ProjectTemplate::Scrum => vec![
                "Backlog".to_string(),
                "To Do".to_string(),
                "In Progress".to_string(),
                "Review".to_string(),
                "Done".to_string(),
            ],
            ProjectTemplate::BugTracking => vec![
                "New".to_string(),
                "Confirmed".to_string(),
                "In Progress".to_string(),
                "Testing".to_string(),
                "Closed".to_string(),
            ],
            ProjectTemplate::ContentCalendar => vec![
                "Ideas".to_string(),
                "Draft".to_string(),
                "Review".to_string(),
                "Scheduled".to_string(),
                "Published".to_string(),
            ],
            ProjectTemplate::Personal => vec![
                "Inbox".to_string(),
                "Today".to_string(),
                "This Week".to_string(),
                "Someday".to_string(),
                "Done".to_string(),
            ],
            ProjectTemplate::Roadmap => vec![
                "Q1".to_string(),
                "Q2".to_string(),
                "Q3".to_string(),
                "Q4".to_string(),
                "Future".to_string(),
            ],
            ProjectTemplate::Custom => vec![],
        }
    }
}

/// Project status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProjectStatus {
    /// Project is active
    Active,
    /// Project is archived
    Archived,
    /// Project is on hold
    OnHold,
    /// Project is completed
    Completed,
}

impl ProjectStatus {
    /// Get display name
    pub fn name(&self) -> &'static str {
        match self {
            ProjectStatus::Active => "Active",
            ProjectStatus::Archived => "Archived",
            ProjectStatus::OnHold => "On Hold",
            ProjectStatus::Completed => "Completed",
        }
    }
}

/// Project definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    /// Unique project ID
    pub id: String,
    /// Project name
    pub name: String,
    /// Project description
    pub description: String,
    /// Project template used
    pub template: ProjectTemplate,
    /// Project status
    pub status: ProjectStatus,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
    /// Last modified timestamp
    pub modified_at: DateTime<Utc>,
    /// Column names
    pub columns: Vec<String>,
    /// Is starred/favorited
    pub starred: bool,
    /// Project tags
    pub tags: Vec<String>,
    /// Project owner
    pub owner: Option<String>,
    /// Project metadata
    pub metadata: HashMap<String, String>,
}

impl Project {
    /// Create a new project
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            id: id.into(),
            name: name.into(),
            description: String::new(),
            template: ProjectTemplate::Blank,
            status: ProjectStatus::Active,
            created_at: now,
            modified_at: now,
            columns: Vec::new(),
            starred: false,
            tags: Vec::new(),
            owner: None,
            metadata: HashMap::new(),
        }
    }

    /// Create from template
    pub fn from_template(
        id: impl Into<String>,
        name: impl Into<String>,
        template: ProjectTemplate,
    ) -> Self {
        let mut project = Self::new(id, name);
        project.template = template;
        project.columns = template.default_columns();
        project
    }

    /// Set description
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self.modified_at = Utc::now();
        self
    }

    /// Set status
    pub fn status(mut self, status: ProjectStatus) -> Self {
        self.status = status;
        self.modified_at = Utc::now();
        self
    }

    /// Star the project
    pub fn star(mut self) -> Self {
        self.starred = true;
        self.modified_at = Utc::now();
        self
    }

    /// Add tag
    pub fn add_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self.modified_at = Utc::now();
        self
    }

    /// Set owner
    pub fn owner(mut self, owner: impl Into<String>) -> Self {
        self.owner = Some(owner.into());
        self.modified_at = Utc::now();
        self
    }

    /// Add column
    pub fn add_column(&mut self, name: impl Into<String>) {
        self.columns.push(name.into());
        self.modified_at = Utc::now();
    }

    /// Clone project structure (without content)
    pub fn clone_structure(&self, new_id: impl Into<String>, new_name: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            id: new_id.into(),
            name: new_name.into(),
            description: self.description.clone(),
            template: self.template,
            status: ProjectStatus::Active,
            created_at: now,
            modified_at: now,
            columns: self.columns.clone(),
            starred: false,
            tags: self.tags.clone(),
            owner: self.owner.clone(),
            metadata: self.metadata.clone(),
        }
    }
}

/// Workspace containing multiple projects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    /// Unique workspace ID
    pub id: String,
    /// Workspace name
    pub name: String,
    /// Workspace description
    pub description: String,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
    /// Projects in this workspace
    pub project_ids: Vec<String>,
    /// Workspace settings
    pub settings: HashMap<String, String>,
}

impl Workspace {
    /// Create a new workspace
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: String::new(),
            created_at: Utc::now(),
            project_ids: Vec::new(),
            settings: HashMap::new(),
        }
    }

    /// Add project to workspace
    pub fn add_project(&mut self, project_id: impl Into<String>) {
        self.project_ids.push(project_id.into());
    }

    /// Remove project from workspace
    pub fn remove_project(&mut self, project_id: &str) -> bool {
        if let Some(pos) = self.project_ids.iter().position(|id| id == project_id) {
            self.project_ids.remove(pos);
            true
        } else {
            false
        }
    }
}

/// Project manager
///
/// Manages projects, workspaces, templates, and cross-project references.
#[derive(Debug)]
pub struct ProjectManager {
    /// All projects
    projects: HashMap<String, Project>,
    /// All workspaces
    workspaces: HashMap<String, Workspace>,
    /// Active workspace
    active_workspace: Option<String>,
    /// Starred projects
    starred_projects: Vec<String>,
    /// Recently accessed projects
    recent_projects: Vec<String>,
    /// Max recent projects to track
    max_recent: usize,
    /// Next project ID counter
    next_project_id: usize,
    /// Next workspace ID counter
    next_workspace_id: usize,
}

impl ProjectManager {
    /// Create a new project manager
    pub fn new() -> Self {
        Self {
            projects: HashMap::new(),
            workspaces: HashMap::new(),
            active_workspace: None,
            starred_projects: Vec::new(),
            recent_projects: Vec::new(),
            max_recent: 10,
            next_project_id: 1,
            next_workspace_id: 1,
        }
    }

    /// Create a new project
    pub fn create_project(
        &mut self,
        name: impl Into<String>,
        description: impl Into<String>,
    ) -> String {
        let id = format!("proj-{}", self.next_project_id);
        self.next_project_id += 1;
        let project = Project::new(id.clone(), name).description(description);
        self.projects.insert(id.clone(), project);
        self.add_to_recent(&id);
        id
    }

    /// Create project from template
    pub fn create_from_template(
        &mut self,
        name: impl Into<String>,
        template: ProjectTemplate,
    ) -> String {
        let id = format!("proj-{}", self.next_project_id);
        self.next_project_id += 1;
        let project = Project::from_template(id.clone(), name, template);
        self.projects.insert(id.clone(), project);
        self.add_to_recent(&id);
        id
    }

    /// Get project by ID
    pub fn get_project(&self, id: &str) -> Option<&Project> {
        self.projects.get(id)
    }

    /// Get mutable project by ID
    pub fn get_project_mut(&mut self, id: &str) -> Option<&mut Project> {
        self.projects.get_mut(id)
    }

    /// Delete project
    pub fn delete_project(&mut self, id: &str) -> bool {
        if self.projects.remove(id).is_some() {
            self.starred_projects.retain(|pid| pid != id);
            self.recent_projects.retain(|pid| pid != id);
            true
        } else {
            false
        }
    }

    /// Archive project
    pub fn archive_project(&mut self, id: &str) -> bool {
        if let Some(project) = self.projects.get_mut(id) {
            project.status = ProjectStatus::Archived;
            project.modified_at = Utc::now();
            true
        } else {
            false
        }
    }

    /// Clone project
    pub fn clone_project(&mut self, id: &str, new_name: impl Into<String>) -> Option<String> {
        let project = self.projects.get(id)?;
        let new_id = format!("proj-{}", self.next_project_id);
        self.next_project_id += 1;
        let cloned = project.clone_structure(new_id.clone(), new_name);
        self.projects.insert(new_id.clone(), cloned);
        self.add_to_recent(&new_id);
        Some(new_id)
    }

    /// Star/unstar project
    pub fn toggle_star(&mut self, id: &str) -> bool {
        if let Some(project) = self.projects.get_mut(id) {
            project.starred = !project.starred;
            project.modified_at = Utc::now();

            if project.starred {
                if !self.starred_projects.contains(&id.to_string()) {
                    self.starred_projects.push(id.to_string());
                }
            } else {
                self.starred_projects.retain(|pid| pid != id);
            }
            true
        } else {
            false
        }
    }

    /// Get all projects
    pub fn all_projects(&self) -> Vec<&Project> {
        self.projects.values().collect()
    }

    /// Get active projects
    pub fn active_projects(&self) -> Vec<&Project> {
        self.projects
            .values()
            .filter(|p| p.status == ProjectStatus::Active)
            .collect()
    }

    /// Get archived projects
    pub fn archived_projects(&self) -> Vec<&Project> {
        self.projects
            .values()
            .filter(|p| p.status == ProjectStatus::Archived)
            .collect()
    }

    /// Get starred projects
    pub fn starred_projects(&self) -> Vec<&Project> {
        self.starred_projects
            .iter()
            .filter_map(|id| self.projects.get(id))
            .collect()
    }

    /// Get recent projects
    pub fn recent_projects(&self) -> Vec<&Project> {
        self.recent_projects
            .iter()
            .filter_map(|id| self.projects.get(id))
            .collect()
    }

    /// Create a workspace
    pub fn create_workspace(&mut self, name: impl Into<String>) -> String {
        let id = format!("ws-{}", self.next_workspace_id);
        self.next_workspace_id += 1;
        let workspace = Workspace::new(id.clone(), name);
        self.workspaces.insert(id.clone(), workspace);
        id
    }

    /// Get workspace by ID
    pub fn get_workspace(&self, id: &str) -> Option<&Workspace> {
        self.workspaces.get(id)
    }

    /// Get mutable workspace by ID
    pub fn get_workspace_mut(&mut self, id: &str) -> Option<&mut Workspace> {
        self.workspaces.get_mut(id)
    }

    /// Add project to workspace
    pub fn add_to_workspace(&mut self, workspace_id: &str, project_id: &str) -> bool {
        if let Some(workspace) = self.workspaces.get_mut(workspace_id) {
            workspace.add_project(project_id);
            true
        } else {
            false
        }
    }

    /// Set active workspace
    pub fn set_active_workspace(&mut self, workspace_id: impl Into<String>) {
        self.active_workspace = Some(workspace_id.into());
    }

    /// Get active workspace
    pub fn active_workspace(&self) -> Option<&Workspace> {
        self.active_workspace
            .as_ref()
            .and_then(|id| self.workspaces.get(id))
    }

    /// Get projects in workspace
    pub fn workspace_projects(&self, workspace_id: &str) -> Vec<&Project> {
        self.workspaces
            .get(workspace_id)
            .map(|ws| {
                ws.project_ids
                    .iter()
                    .filter_map(|id| self.projects.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Add project to recent list
    fn add_to_recent(&mut self, id: &str) {
        self.recent_projects.retain(|pid| pid != id);
        self.recent_projects.insert(0, id.to_string());
        self.recent_projects.truncate(self.max_recent);
    }

    /// Search projects by name or description
    pub fn search(&self, query: &str) -> Vec<&Project> {
        let query_lower = query.to_lowercase();
        self.projects
            .values()
            .filter(|p| {
                p.name.to_lowercase().contains(&query_lower)
                    || p.description.to_lowercase().contains(&query_lower)
                    || p.tags
                        .iter()
                        .any(|t| t.to_lowercase().contains(&query_lower))
            })
            .collect()
    }

    /// Get project count
    pub fn project_count(&self) -> usize {
        self.projects.len()
    }

    /// Get workspace count
    pub fn workspace_count(&self) -> usize {
        self.workspaces.len()
    }
}

impl Default for ProjectManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_template_all() {
        let templates = ProjectTemplate::all();
        assert_eq!(templates.len(), 7);
    }

    #[test]
    fn test_project_template_name() {
        assert_eq!(ProjectTemplate::Scrum.name(), "Scrum");
        assert_eq!(ProjectTemplate::BugTracking.name(), "Bug Tracking");
    }

    #[test]
    fn test_project_template_columns() {
        let columns = ProjectTemplate::Scrum.default_columns();
        assert_eq!(columns.len(), 5);
        assert_eq!(columns[0], "Backlog");
        assert_eq!(columns[4], "Done");
    }

    #[test]
    fn test_project_status_name() {
        assert_eq!(ProjectStatus::Active.name(), "Active");
        assert_eq!(ProjectStatus::Archived.name(), "Archived");
    }

    #[test]
    fn test_project_creation() {
        let project = Project::new("proj-1", "My Project");
        assert_eq!(project.id, "proj-1");
        assert_eq!(project.name, "My Project");
        assert_eq!(project.status, ProjectStatus::Active);
        assert!(!project.starred);
    }

    #[test]
    fn test_project_from_template() {
        let project = Project::from_template("proj-1", "Scrum Board", ProjectTemplate::Scrum);
        assert_eq!(project.template, ProjectTemplate::Scrum);
        assert_eq!(project.columns.len(), 5);
    }

    #[test]
    fn test_project_builder() {
        let project = Project::new("proj-1", "Test")
            .description("Test project")
            .status(ProjectStatus::Active)
            .star()
            .add_tag("important")
            .owner("user-1");

        assert_eq!(project.description, "Test project");
        assert_eq!(project.status, ProjectStatus::Active);
        assert!(project.starred);
        assert_eq!(project.tags, vec!["important"]);
        assert_eq!(project.owner, Some("user-1".to_string()));
    }

    #[test]
    fn test_project_add_column() {
        let mut project = Project::new("proj-1", "Test");
        project.add_column("Column 1");
        project.add_column("Column 2");
        assert_eq!(project.columns.len(), 2);
    }

    #[test]
    fn test_project_clone_structure() {
        let project = Project::from_template("proj-1", "Original", ProjectTemplate::Scrum)
            .description("Original description")
            .add_tag("tag1");

        let cloned = project.clone_structure("proj-2", "Cloned");

        assert_eq!(cloned.id, "proj-2");
        assert_eq!(cloned.name, "Cloned");
        assert_eq!(cloned.description, "Original description");
        assert_eq!(cloned.columns.len(), 5);
        assert_eq!(cloned.tags, vec!["tag1"]);
        assert!(!cloned.starred);
    }

    #[test]
    fn test_workspace_creation() {
        let workspace = Workspace::new("ws-1", "My Workspace");
        assert_eq!(workspace.id, "ws-1");
        assert_eq!(workspace.name, "My Workspace");
        assert_eq!(workspace.project_ids.len(), 0);
    }

    #[test]
    fn test_workspace_add_remove_project() {
        let mut workspace = Workspace::new("ws-1", "Test");
        workspace.add_project("proj-1");
        workspace.add_project("proj-2");
        assert_eq!(workspace.project_ids.len(), 2);

        assert!(workspace.remove_project("proj-1"));
        assert_eq!(workspace.project_ids.len(), 1);
        assert!(!workspace.remove_project("proj-3"));
    }

    #[test]
    fn test_manager_creation() {
        let manager = ProjectManager::new();
        assert_eq!(manager.project_count(), 0);
        assert_eq!(manager.workspace_count(), 0);
    }

    #[test]
    fn test_manager_create_project() {
        let mut manager = ProjectManager::new();
        let id = manager.create_project("Test Project", "Description");

        assert_eq!(manager.project_count(), 1);
        let project = manager.get_project(&id).unwrap();
        assert_eq!(project.name, "Test Project");
    }

    #[test]
    fn test_manager_create_from_template() {
        let mut manager = ProjectManager::new();
        let id = manager.create_from_template("Scrum Board", ProjectTemplate::Scrum);

        let project = manager.get_project(&id).unwrap();
        assert_eq!(project.template, ProjectTemplate::Scrum);
        assert_eq!(project.columns.len(), 5);
    }

    #[test]
    fn test_manager_delete_project() {
        let mut manager = ProjectManager::new();
        let id = manager.create_project("Test", "Desc");

        assert!(manager.delete_project(&id));
        assert_eq!(manager.project_count(), 0);
        assert!(!manager.delete_project(&id));
    }

    #[test]
    fn test_manager_archive_project() {
        let mut manager = ProjectManager::new();
        let id = manager.create_project("Test", "Desc");

        assert!(manager.archive_project(&id));
        let project = manager.get_project(&id).unwrap();
        assert_eq!(project.status, ProjectStatus::Archived);
    }

    #[test]
    fn test_manager_clone_project() {
        let mut manager = ProjectManager::new();
        let id = manager.create_from_template("Original", ProjectTemplate::Scrum);

        let cloned_id = manager.clone_project(&id, "Cloned").unwrap();
        assert_eq!(manager.project_count(), 2);

        let cloned = manager.get_project(&cloned_id).unwrap();
        assert_eq!(cloned.name, "Cloned");
        assert_eq!(cloned.columns.len(), 5);
    }

    #[test]
    fn test_manager_toggle_star() {
        let mut manager = ProjectManager::new();
        let id = manager.create_project("Test", "Desc");

        assert!(manager.toggle_star(&id));
        assert_eq!(manager.starred_projects().len(), 1);

        assert!(manager.toggle_star(&id));
        assert_eq!(manager.starred_projects().len(), 0);
    }

    #[test]
    fn test_manager_active_archived_projects() {
        let mut manager = ProjectManager::new();
        let id1 = manager.create_project("Active", "Desc");
        let id2 = manager.create_project("Archived", "Desc");
        manager.archive_project(&id2);

        assert_eq!(manager.active_projects().len(), 1);
        assert_eq!(manager.archived_projects().len(), 1);
    }

    #[test]
    fn test_manager_recent_projects() {
        let mut manager = ProjectManager::new();
        let id1 = manager.create_project("First", "Desc");
        let id2 = manager.create_project("Second", "Desc");

        let recent = manager.recent_projects();
        assert_eq!(recent.len(), 2);
        assert_eq!(recent[0].id, id2); // Most recent first
    }

    #[test]
    fn test_manager_workspaces() {
        let mut manager = ProjectManager::new();
        let ws_id = manager.create_workspace("My Workspace");

        assert_eq!(manager.workspace_count(), 1);
        let workspace = manager.get_workspace(&ws_id).unwrap();
        assert_eq!(workspace.name, "My Workspace");
    }

    #[test]
    fn test_manager_add_to_workspace() {
        let mut manager = ProjectManager::new();
        let proj_id = manager.create_project("Test", "Desc");
        let ws_id = manager.create_workspace("Workspace");

        assert!(manager.add_to_workspace(&ws_id, &proj_id));
        let projects = manager.workspace_projects(&ws_id);
        assert_eq!(projects.len(), 1);
    }

    #[test]
    fn test_manager_active_workspace() {
        let mut manager = ProjectManager::new();
        let ws_id = manager.create_workspace("Active");

        manager.set_active_workspace(ws_id.clone());
        let active = manager.active_workspace().unwrap();
        assert_eq!(active.id, ws_id);
    }

    #[test]
    fn test_manager_search() {
        let mut manager = ProjectManager::new();
        manager.create_project("Frontend App", "React application");
        manager.create_project("Backend API", "Node.js API");
        manager.create_project("Mobile App", "React Native");

        let results = manager.search("react");
        assert_eq!(results.len(), 2);

        let results = manager.search("api");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_default_manager() {
        let manager = ProjectManager::default();
        assert_eq!(manager.project_count(), 0);
    }
}
