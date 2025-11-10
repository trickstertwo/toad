//! Integration tests for Git-related widgets
//!
//! Tests cross-widget interactions and workflows for GitStatusPanel, GitGraph,
//! GitDiffViewer, and related git functionality.

use toad::ui::widgets::{FileStatus, GitCommit, GitDiffViewer, GitFile, GitGraph, GitStatusPanel};

// ==================== Git Status Panel Tests ====================

#[test]
fn test_git_status_basic_workflow() {
    let mut panel = GitStatusPanel::new();

    // Set up repository state
    panel.set_branch("main");
    panel.set_ahead_behind(3, 1);

    // Add files with different statuses
    panel.add_file("src/lib.rs", FileStatus::Modified);
    panel.add_file("tests/test.rs", FileStatus::Staged);
    panel.add_file("README.md", FileStatus::Untracked);

    assert_eq!(panel.file_count(), 3);
}

#[test]
fn test_git_status_file_selection() {
    let mut panel = GitStatusPanel::new();

    panel.add_file("file1.rs", FileStatus::Modified);
    panel.add_file("file2.rs", FileStatus::Modified);
    panel.add_file("file3.rs", FileStatus::Untracked);

    // Initially no files selected
    assert_eq!(panel.selected_files().len(), 0);

    // Select first two files
    panel.toggle_selection(0);
    panel.toggle_selection(1);

    assert_eq!(panel.selected_files().len(), 2);

    // Deselect first file
    panel.toggle_selection(0);
    assert_eq!(panel.selected_files().len(), 1);
}

#[test]
fn test_git_status_clear_and_reset() {
    let mut panel = GitStatusPanel::new();

    panel.add_file("file1.rs", FileStatus::Modified);
    panel.add_file("file2.rs", FileStatus::Staged);
    assert_eq!(panel.file_count(), 2);

    panel.clear();
    assert_eq!(panel.file_count(), 0);

    // Can add files again after clearing
    panel.add_file("new_file.rs", FileStatus::Untracked);
    assert_eq!(panel.file_count(), 1);
}

#[test]
fn test_git_status_builder_pattern() {
    let panel = GitStatusPanel::new()
        .with_title("My Git Status")
        .with_summary(true)
        .with_compact(false);

    assert_eq!(panel.file_count(), 0);
}

#[test]
fn test_git_status_all_file_statuses() {
    let mut panel = GitStatusPanel::new();

    panel.add_file("modified.rs", FileStatus::Modified);
    panel.add_file("staged.rs", FileStatus::Staged);
    panel.add_file("untracked.rs", FileStatus::Untracked);
    panel.add_file("deleted.rs", FileStatus::Deleted);
    panel.add_file("renamed.rs", FileStatus::Renamed);
    panel.add_file("conflicted.rs", FileStatus::Conflicted);
    panel.add_file("modified_staged.rs", FileStatus::ModifiedStaged);

    assert_eq!(panel.file_count(), 7);
}

#[test]
fn test_git_status_set_files_batch() {
    let mut panel = GitStatusPanel::new();

    let files = vec![
        GitFile::new("file1.rs", FileStatus::Modified),
        GitFile::new("file2.rs", FileStatus::Staged),
        GitFile::new("file3.rs", FileStatus::Untracked),
    ];

    panel.set_files(files);
    assert_eq!(panel.file_count(), 3);

    // Setting files again replaces previous files
    let new_files = vec![GitFile::new("only_file.rs", FileStatus::Modified)];
    panel.set_files(new_files);
    assert_eq!(panel.file_count(), 1);
}

// ==================== Git Graph Tests ====================

#[test]
fn test_git_graph_basic_workflow() {
    let mut graph = GitGraph::new();

    graph.add_commit(GitCommit::new("abc123", "Initial commit").with_branch("main"));

    graph.add_commit(
        GitCommit::new("def456", "Add feature")
            .with_parent("abc123")
            .with_author("Alice")
            .with_branch("main"),
    );

    assert_eq!(graph.commit_count(), 2);
}

#[test]
fn test_git_graph_branch_visualization() {
    let mut graph = GitGraph::new();

    // Main branch
    graph.add_commit(GitCommit::new("c1", "Initial").with_branch("main"));
    graph.add_commit(
        GitCommit::new("c2", "Second")
            .with_parent("c1")
            .with_branch("main"),
    );

    // Feature branch diverges
    graph.add_commit(
        GitCommit::new("f1", "Feature work")
            .with_parent("c2")
            .with_branch("feature"),
    );

    // Main continues
    graph.add_commit(
        GitCommit::new("c3", "Third")
            .with_parent("c2")
            .with_branch("main"),
    );

    assert_eq!(graph.commit_count(), 4);
}

#[test]
fn test_git_graph_merge_commits() {
    let mut graph = GitGraph::new();

    graph.add_commit(GitCommit::new("c1", "Initial").with_branch("main"));
    graph.add_commit(
        GitCommit::new("c2", "Second")
            .with_parent("c1")
            .with_branch("main"),
    );

    // Feature branch
    graph.add_commit(
        GitCommit::new("f1", "Feature")
            .with_parent("c2")
            .with_branch("feature"),
    );

    // Merge commit
    let merge = GitCommit::new("m1", "Merge feature into main")
        .with_parent("c2")
        .with_merge_parent("f1")
        .with_branch("main");

    assert!(merge.is_merge());
    graph.add_commit(merge);

    assert_eq!(graph.commit_count(), 4);
}

#[test]
fn test_git_graph_builder_pattern() {
    let graph = GitGraph::new()
        .with_title("Commit History")
        .with_authors(true)
        .with_branches(true)
        .with_compact(false)
        .with_max_commits(10);

    assert_eq!(graph.commit_count(), 0);
}

#[test]
fn test_git_graph_clear_commits() {
    let mut graph = GitGraph::new();

    graph.add_commit(GitCommit::new("c1", "Test"));
    graph.add_commit(GitCommit::new("c2", "Test 2"));
    assert_eq!(graph.commit_count(), 2);

    graph.clear();
    assert_eq!(graph.commit_count(), 0);
}

#[test]
fn test_git_graph_add_commits_batch() {
    let mut graph = GitGraph::new();

    let commits = vec![
        GitCommit::new("c1", "First"),
        GitCommit::new("c2", "Second"),
        GitCommit::new("c3", "Third"),
    ];

    graph.add_commits(commits);
    assert_eq!(graph.commit_count(), 3);
}

#[test]
fn test_git_graph_max_commits_limit() {
    let mut graph = GitGraph::new().with_max_commits(3);

    // Add 5 commits
    for i in 1..=5 {
        graph.add_commit(GitCommit::new(format!("c{}", i), format!("Commit {}", i)));
    }

    // All 5 are stored, but only 3 would be displayed (max_commits affects rendering only)
    assert_eq!(graph.commit_count(), 5);
}

// ==================== Git Commit Tests ====================

#[test]
fn test_git_commit_builder_pattern() {
    use ratatui::style::Color;

    let commit = GitCommit::new("abc123", "Fix critical bug")
        .with_author("Bob")
        .with_branch("hotfix")
        .with_parent("def456")
        .with_color(Color::Red);

    assert_eq!(commit.hash(), "abc123");
    assert_eq!(commit.message(), "Fix critical bug");
    assert_eq!(commit.author(), Some("Bob"));
    assert_eq!(commit.branch(), Some("hotfix"));
    assert_eq!(commit.parent(), Some("def456"));
    assert!(!commit.is_merge());
}

#[test]
fn test_git_commit_merge_detection() {
    let regular_commit = GitCommit::new("c1", "Regular commit");
    assert!(!regular_commit.is_merge());

    let merge_commit = GitCommit::new("m1", "Merge branch")
        .with_parent("main_parent")
        .with_merge_parent("feature_parent");
    assert!(merge_commit.is_merge());
}

#[test]
fn test_git_commit_multiple_merge_parents() {
    let octopus_merge = GitCommit::new("oct1", "Octopus merge")
        .with_parent("main")
        .with_merge_parent("feature1")
        .with_merge_parent("feature2")
        .with_merge_parent("feature3");

    assert!(octopus_merge.is_merge());
}

// ==================== Cross-Widget Integration Tests ====================

#[test]
fn test_git_workflow_status_to_graph() {
    // Simulate workflow: check status → make changes → commit → see in graph
    let mut status = GitStatusPanel::new();
    let mut graph = GitGraph::new();

    // Step 1: Check status - working directory clean
    status.set_branch("main");
    assert_eq!(status.file_count(), 0);

    // Step 2: Make changes
    status.add_file("src/lib.rs", FileStatus::Modified);
    status.add_file("tests/test.rs", FileStatus::Modified);
    assert_eq!(status.file_count(), 2);

    // Step 3: Stage files
    status.clear();
    status.add_file("src/lib.rs", FileStatus::Staged);
    status.add_file("tests/test.rs", FileStatus::Staged);

    // Step 4: Commit (creates commit in graph)
    graph.add_commit(
        GitCommit::new("new_commit", "Add new feature")
            .with_branch("main")
            .with_author("Developer"),
    );
    assert_eq!(graph.commit_count(), 1);

    // Step 5: Clean working directory again
    status.clear();
    assert_eq!(status.file_count(), 0);
}

#[test]
fn test_git_workflow_feature_branch() {
    let mut status = GitStatusPanel::new();
    let mut graph = GitGraph::new();

    // Start on main branch
    status.set_branch("main");
    graph.add_commit(GitCommit::new("c1", "Initial commit").with_branch("main"));

    // Create feature branch
    status.set_branch("feature/new-widget");

    // Make changes on feature branch
    status.add_file("src/new_widget.rs", FileStatus::Untracked);
    status.add_file("tests/widget_test.rs", FileStatus::Untracked);

    // Commit on feature branch
    graph.add_commit(
        GitCommit::new("f1", "Add new widget")
            .with_branch("feature/new-widget")
            .with_parent("c1"),
    );

    // Switch back to main
    status.set_branch("main");
    status.clear();

    // Merge feature into main
    graph.add_commit(
        GitCommit::new("m1", "Merge new widget")
            .with_branch("main")
            .with_parent("c1")
            .with_merge_parent("f1"),
    );

    assert_eq!(graph.commit_count(), 3);
}

#[test]
fn test_git_workflow_status_diff_graph_integration() {
    // Complete workflow: status → diff → commit → graph
    let mut status = GitStatusPanel::new();
    let mut diff_viewer = GitDiffViewer::new();
    let mut graph = GitGraph::new();

    // Step 1: Check status
    status.set_branch("main");
    status.add_file("src/main.rs", FileStatus::Modified);

    // Step 2: View diff for the modified file
    let diff_content = r#"diff --git a/src/main.rs b/src/main.rs
index 1234567..abcdefg 100644
--- a/src/main.rs
+++ b/src/main.rs
@@ -1,3 +1,4 @@
 fn main() {
     println!("Hello, world!");
+    println!("New feature!");
 }
"#;
    diff_viewer.set_diff(diff_content);
    assert_eq!(diff_viewer.line_count(), 9);

    // Step 3: Stage and commit
    status.clear();
    status.add_file("src/main.rs", FileStatus::Staged);

    graph.add_commit(
        GitCommit::new("new123", "Add new feature")
            .with_branch("main")
            .with_author("Developer"),
    );

    // Step 4: Verify workflow completed
    assert_eq!(graph.commit_count(), 1);
    status.clear();
    assert_eq!(status.file_count(), 0);
}

// ==================== File Status Tests ====================

#[test]
fn test_file_status_display() {
    assert_eq!(FileStatus::Modified.char(), "M");
    assert_eq!(FileStatus::Staged.char(), "A");
    assert_eq!(FileStatus::Untracked.char(), "?");
    assert_eq!(FileStatus::Deleted.char(), "D");
    assert_eq!(FileStatus::Renamed.char(), "R");
    assert_eq!(FileStatus::Conflicted.char(), "C");
    assert_eq!(FileStatus::ModifiedStaged.char(), "M");
}

#[test]
fn test_file_status_colors() {
    use ratatui::style::Color;

    assert_eq!(FileStatus::Modified.color(), Color::Yellow);
    assert_eq!(FileStatus::Staged.color(), Color::Green);
    assert_eq!(FileStatus::Untracked.color(), Color::Red);
    assert_eq!(FileStatus::Deleted.color(), Color::Red);
    assert_eq!(FileStatus::Renamed.color(), Color::Cyan);
    assert_eq!(FileStatus::Conflicted.color(), Color::Magenta);
    assert_eq!(FileStatus::ModifiedStaged.color(), Color::Yellow);
}

// ==================== Performance & Edge Cases ====================

#[test]
fn test_git_status_large_changeset() {
    let mut panel = GitStatusPanel::new();

    // Simulate large changeset (100 modified files)
    for i in 0..100 {
        panel.add_file(format!("src/file_{}.rs", i), FileStatus::Modified);
    }

    assert_eq!(panel.file_count(), 100);

    // Select every other file
    for i in (0..100).step_by(2) {
        panel.toggle_selection(i);
    }

    assert_eq!(panel.selected_files().len(), 50);
}

#[test]
fn test_git_graph_deep_history() {
    let mut graph = GitGraph::new();

    // Create a linear history with 100 commits
    let mut prev = None;
    for i in 0..100 {
        let mut commit = GitCommit::new(format!("c{}", i), format!("Commit {}", i))
            .with_branch("main")
            .with_author("Developer");

        if let Some(parent) = prev {
            commit = commit.with_parent(parent);
        }

        graph.add_commit(commit);
        prev = Some(format!("c{}", i));
    }

    assert_eq!(graph.commit_count(), 100);
}

#[test]
fn test_git_status_panel_edge_cases() {
    let mut panel = GitStatusPanel::new();

    // Toggle selection on empty panel (should not panic)
    panel.toggle_selection(0);
    panel.toggle_selection(999);

    // Toggle selection on out-of-bounds index (should not panic)
    panel.add_file("file.rs", FileStatus::Modified);
    panel.toggle_selection(10); // Only 1 file exists

    assert_eq!(panel.file_count(), 1);
}

#[test]
fn test_git_graph_empty_state() {
    let graph = GitGraph::new();

    assert_eq!(graph.commit_count(), 0);
}

// ==================== Real-World Scenario Tests ====================

#[test]
fn test_scenario_merge_conflict_resolution() {
    let mut status = GitStatusPanel::new();
    let mut graph = GitGraph::new();

    // Setup: two branches diverged
    graph.add_commit(GitCommit::new("c1", "Base").with_branch("main"));
    graph.add_commit(
        GitCommit::new("c2", "Main work")
            .with_parent("c1")
            .with_branch("main"),
    );
    graph.add_commit(
        GitCommit::new("f1", "Feature work")
            .with_parent("c1")
            .with_branch("feature"),
    );

    // Attempt to merge creates conflict
    status.set_branch("main");
    status.add_file("src/conflicted.rs", FileStatus::Conflicted);

    // Resolve conflict
    status.clear();
    status.add_file("src/conflicted.rs", FileStatus::Staged);

    // Complete merge
    graph.add_commit(
        GitCommit::new("m1", "Merge feature")
            .with_parent("c2")
            .with_merge_parent("f1")
            .with_branch("main"),
    );

    assert_eq!(graph.commit_count(), 4);
    assert_eq!(status.file_count(), 1);
}

#[test]
fn test_scenario_hotfix_workflow() {
    let mut status = GitStatusPanel::new();
    let mut graph = GitGraph::new();

    // Production has a bug
    status.set_branch("main");
    graph.add_commit(GitCommit::new("v1.0", "Release 1.0").with_branch("main"));

    // Create hotfix branch
    status.set_branch("hotfix/critical-bug");
    status.add_file("src/bug.rs", FileStatus::Modified);

    // Fix and commit
    graph.add_commit(
        GitCommit::new("fix1", "Fix critical bug")
            .with_parent("v1.0")
            .with_branch("hotfix/critical-bug"),
    );
    status.clear();

    // Merge to main
    status.set_branch("main");
    graph.add_commit(
        GitCommit::new("v1.0.1", "Release 1.0.1")
            .with_parent("v1.0")
            .with_merge_parent("fix1")
            .with_branch("main"),
    );

    assert_eq!(graph.commit_count(), 3);
}

#[test]
fn test_scenario_ahead_behind_tracking() {
    let mut panel = GitStatusPanel::new();

    // Scenario: local branch is ahead and behind remote
    panel.set_branch("feature/wip");
    panel.set_ahead_behind(5, 2); // 5 commits ahead, 2 behind

    // This indicates need to pull and merge before pushing
    panel.add_file("src/work.rs", FileStatus::Modified);

    assert_eq!(panel.file_count(), 1);
}
