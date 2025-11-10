//! Task Dependencies module for critical path method
//!
//! This module provides comprehensive dependency management for tasks,
//! including dependency types, circular detection, and critical path calculation.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};

/// Dependency type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DependencyType {
    /// Task blocks another task (must complete before dependent can start)
    Blocks,
    /// Task is blocked by another task (cannot start until blocker completes)
    BlockedBy,
    /// Task relates to another task (informational link)
    RelatesTo,
    /// Task duplicates another task
    Duplicates,
}

impl DependencyType {
    /// Returns the name of the dependency type
    pub fn name(&self) -> &'static str {
        match self {
            DependencyType::Blocks => "Blocks",
            DependencyType::BlockedBy => "Blocked By",
            DependencyType::RelatesTo => "Relates To",
            DependencyType::Duplicates => "Duplicates",
        }
    }

    /// Returns the inverse dependency type
    pub fn inverse(&self) -> Option<DependencyType> {
        match self {
            DependencyType::Blocks => Some(DependencyType::BlockedBy),
            DependencyType::BlockedBy => Some(DependencyType::Blocks),
            DependencyType::RelatesTo => Some(DependencyType::RelatesTo),
            DependencyType::Duplicates => None, // Duplicates is not bidirectional
        }
    }

    /// Returns whether this dependency type affects scheduling
    pub fn affects_scheduling(&self) -> bool {
        matches!(self, DependencyType::Blocks | DependencyType::BlockedBy)
    }
}

/// Task dependency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    /// Dependency ID
    pub id: String,
    /// Source task ID
    pub from_task: String,
    /// Target task ID
    pub to_task: String,
    /// Dependency type
    pub dependency_type: DependencyType,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
    /// Created by user ID
    pub created_by: String,
}

impl Dependency {
    /// Creates a new dependency
    pub fn new(
        id: String,
        from_task: String,
        to_task: String,
        dependency_type: DependencyType,
        created_by: String,
    ) -> Self {
        Self {
            id,
            from_task,
            to_task,
            dependency_type,
            created_at: Utc::now(),
            created_by,
        }
    }
}

/// Critical path node
#[derive(Debug, Clone)]
pub struct CriticalPathNode {
    /// Task ID
    pub task_id: String,
    /// Earliest start time (in days from project start)
    pub earliest_start: f32,
    /// Latest start time (in days from project start)
    pub latest_start: f32,
    /// Duration (in days)
    pub duration: f32,
    /// Whether this node is on the critical path
    pub is_critical: bool,
    /// Slack/float (days of delay allowed without affecting project)
    pub slack: f32,
}

impl CriticalPathNode {
    /// Creates a new critical path node
    pub fn new(task_id: String, duration: f32) -> Self {
        Self {
            task_id,
            earliest_start: 0.0,
            latest_start: 0.0,
            duration,
            is_critical: false,
            slack: 0.0,
        }
    }

    /// Calculates the earliest finish time
    pub fn earliest_finish(&self) -> f32 {
        self.earliest_start + self.duration
    }

    /// Calculates the latest finish time
    pub fn latest_finish(&self) -> f32 {
        self.latest_start + self.duration
    }

    /// Updates slack (difference between latest and earliest start)
    pub fn update_slack(&mut self) {
        self.slack = self.latest_start - self.earliest_start;
        self.is_critical = self.slack.abs() < 0.01; // Floating point tolerance
    }
}

/// Dependency manager
#[derive(Debug)]
pub struct DependencyManager {
    dependencies: HashMap<String, Dependency>,
    task_dependencies: HashMap<String, Vec<String>>, // task_id -> dependency_ids
    next_dependency_id: usize,
}

impl DependencyManager {
    /// Creates a new dependency manager
    pub fn new() -> Self {
        Self {
            dependencies: HashMap::new(),
            task_dependencies: HashMap::new(),
            next_dependency_id: 1,
        }
    }

    /// Creates a new dependency
    pub fn create_dependency(
        &mut self,
        from_task: String,
        to_task: String,
        dependency_type: DependencyType,
        created_by: String,
    ) -> Result<String, String> {
        // Check for circular dependencies (only for blocking dependencies)
        if dependency_type.affects_scheduling()
            && self.would_create_cycle(&from_task, &to_task, dependency_type)
        {
            return Err("Creating this dependency would create a circular dependency chain".to_string());
        }

        let id = format!("dep-{}", self.next_dependency_id);
        self.next_dependency_id += 1;

        let dependency = Dependency::new(id.clone(), from_task.clone(), to_task.clone(), dependency_type, created_by);

        // Add to dependencies
        self.dependencies.insert(id.clone(), dependency);

        // Add to task_dependencies
        self.task_dependencies
            .entry(from_task.clone())
            .or_default()
            .push(id.clone());
        self.task_dependencies
            .entry(to_task.clone())
            .or_default()
            .push(id.clone());

        Ok(id)
    }

    /// Checks if creating a dependency would create a cycle
    fn would_create_cycle(
        &self,
        from_task: &str,
        to_task: &str,
        dependency_type: DependencyType,
    ) -> bool {
        // Only check for Blocks dependencies (to_task cannot reach from_task)
        if dependency_type == DependencyType::Blocks {
            // If adding "from blocks to", check if there's a path from to -> from
            self.has_path(to_task, from_task)
        } else if dependency_type == DependencyType::BlockedBy {
            // If adding "from blocked by to", check if there's a path from from -> to
            self.has_path(from_task, to_task)
        } else {
            false
        }
    }

    /// Checks if there's a dependency path from start to end
    fn has_path(&self, start: &str, end: &str) -> bool {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(start.to_string());

        while let Some(current) = queue.pop_front() {
            if current == end {
                return true;
            }

            if visited.contains(&current) {
                continue;
            }
            visited.insert(current.clone());

            // Get all tasks that current blocks
            let blocked_tasks = self.get_blocked_tasks(&current);
            for task in blocked_tasks {
                queue.push_back(task);
            }
        }

        false
    }

    /// Gets all tasks blocked by a given task
    fn get_blocked_tasks(&self, task_id: &str) -> Vec<String> {
        let mut blocked = Vec::new();

        if let Some(dep_ids) = self.task_dependencies.get(task_id) {
            for dep_id in dep_ids {
                if let Some(dep) = self.dependencies.get(dep_id)
                    && dep.from_task == task_id && dep.dependency_type == DependencyType::Blocks {
                        blocked.push(dep.to_task.clone());
                    }
            }
        }

        blocked
    }

    /// Gets a dependency by ID
    pub fn get_dependency(&self, dep_id: &str) -> Option<&Dependency> {
        self.dependencies.get(dep_id)
    }

    /// Deletes a dependency
    pub fn delete_dependency(&mut self, dep_id: &str) -> Option<Dependency> {
        let dep = self.dependencies.remove(dep_id)?;

        // Remove from task_dependencies
        if let Some(deps) = self.task_dependencies.get_mut(&dep.from_task) {
            deps.retain(|id| id != dep_id);
        }
        if let Some(deps) = self.task_dependencies.get_mut(&dep.to_task) {
            deps.retain(|id| id != dep_id);
        }

        Some(dep)
    }

    /// Gets all dependencies for a task
    pub fn dependencies_for_task(&self, task_id: &str) -> Vec<&Dependency> {
        self.task_dependencies
            .get(task_id)
            .map(|dep_ids| {
                dep_ids
                    .iter()
                    .filter_map(|id| self.dependencies.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Gets blocking dependencies (tasks that block this task)
    pub fn get_blockers(&self, task_id: &str) -> Vec<&Dependency> {
        self.dependencies_for_task(task_id)
            .into_iter()
            .filter(|dep| {
                dep.to_task == task_id && dep.dependency_type == DependencyType::Blocks
                    || dep.from_task == task_id && dep.dependency_type == DependencyType::BlockedBy
            })
            .collect()
    }

    /// Gets tasks blocked by this task
    pub fn get_blocked(&self, task_id: &str) -> Vec<&Dependency> {
        self.dependencies_for_task(task_id)
            .into_iter()
            .filter(|dep| {
                dep.from_task == task_id && dep.dependency_type == DependencyType::Blocks
                    || dep.to_task == task_id && dep.dependency_type == DependencyType::BlockedBy
            })
            .collect()
    }

    /// Calculates critical path for a set of tasks
    pub fn calculate_critical_path(
        &self,
        task_durations: HashMap<String, f32>,
    ) -> Vec<CriticalPathNode> {
        let mut nodes: HashMap<String, CriticalPathNode> = HashMap::new();

        // Initialize nodes
        for (task_id, duration) in &task_durations {
            nodes.insert(task_id.clone(), CriticalPathNode::new(task_id.clone(), *duration));
        }

        // Forward pass: Calculate earliest start times
        let sorted_tasks = self.topological_sort(&task_durations);
        for task_id in &sorted_tasks {
            // Calculate max_finish before getting mutable reference
            let blockers = self.get_blockers(task_id);
            let max_finish = blockers
                .iter()
                .filter_map(|dep| {
                    let blocker_id = if dep.from_task.as_str() == task_id {
                        &dep.to_task
                    } else {
                        &dep.from_task
                    };
                    nodes.get(blocker_id).map(|n| n.earliest_finish())
                })
                .fold(0.0, f32::max);

            // Now get mutable reference and update
            if let Some(node) = nodes.get_mut(task_id) {
                node.earliest_start = max_finish;
            }
        }

        // Find project completion time
        let project_duration = nodes
            .values()
            .map(|n| n.earliest_finish())
            .fold(0.0, f32::max);

        // Backward pass: Calculate latest start times
        for task_id in sorted_tasks.iter().rev() {
            // Calculate latest_start before getting mutable reference
            let blocked = self.get_blocked(task_id);
            let duration = nodes.get(task_id).map(|n| n.duration).unwrap_or(0.0);

            let latest_start = if blocked.is_empty() {
                // No successors, can finish at project end
                project_duration - duration
            } else {
                let min_start = blocked
                    .iter()
                    .filter_map(|dep| {
                        let blocked_id = if dep.from_task.as_str() == task_id {
                            &dep.to_task
                        } else {
                            &dep.from_task
                        };
                        nodes.get(blocked_id).map(|n| n.latest_start)
                    })
                    .fold(f32::INFINITY, f32::min);

                min_start - duration
            };

            // Now get mutable reference and update
            if let Some(node) = nodes.get_mut(task_id) {
                node.latest_start = latest_start;
                node.update_slack();
            }
        }

        nodes.into_values().collect()
    }

    /// Performs topological sort of tasks based on dependencies
    fn topological_sort(&self, task_durations: &HashMap<String, f32>) -> Vec<String> {
        let mut in_degree: HashMap<String, usize> = HashMap::new();
        let mut adj_list: HashMap<String, Vec<String>> = HashMap::new();

        // Initialize
        for task_id in task_durations.keys() {
            in_degree.insert(task_id.clone(), 0);
            adj_list.insert(task_id.clone(), Vec::new());
        }

        // Build adjacency list and in-degrees
        for dep in self.dependencies.values() {
            if dep.dependency_type == DependencyType::Blocks {
                *in_degree.entry(dep.to_task.clone()).or_insert(0) += 1;
                adj_list
                    .entry(dep.from_task.clone())
                    .or_default()
                    .push(dep.to_task.clone());
            }
        }

        // Kahn's algorithm
        let mut queue: VecDeque<String> = in_degree
            .iter()
            .filter(|(_, degree)| **degree == 0)
            .map(|(task, _)| task.clone())
            .collect();

        let mut result = Vec::new();

        while let Some(task) = queue.pop_front() {
            result.push(task.clone());

            if let Some(neighbors) = adj_list.get(&task) {
                for neighbor in neighbors {
                    if let Some(degree) = in_degree.get_mut(neighbor) {
                        *degree -= 1;
                        if *degree == 0 {
                            queue.push_back(neighbor.clone());
                        }
                    }
                }
            }
        }

        result
    }

    /// Gets the critical path (tasks with zero slack)
    pub fn get_critical_path(&self, task_durations: HashMap<String, f32>) -> Vec<String> {
        self.calculate_critical_path(task_durations)
            .into_iter()
            .filter(|node| node.is_critical)
            .map(|node| node.task_id)
            .collect()
    }

    /// Detects all circular dependencies
    pub fn detect_cycles(&self) -> Vec<Vec<String>> {
        let mut cycles = Vec::new();
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();

        let all_tasks: HashSet<String> = self
            .dependencies
            .values()
            .flat_map(|dep| vec![dep.from_task.clone(), dep.to_task.clone()])
            .collect();

        for task in all_tasks {
            if !visited.contains(&task) {
                let mut path = Vec::new();
                self.dfs_cycle_detection(&task, &mut visited, &mut rec_stack, &mut path, &mut cycles);
            }
        }

        cycles
    }

    fn dfs_cycle_detection(
        &self,
        task: &str,
        visited: &mut HashSet<String>,
        rec_stack: &mut HashSet<String>,
        path: &mut Vec<String>,
        cycles: &mut Vec<Vec<String>>,
    ) {
        visited.insert(task.to_string());
        rec_stack.insert(task.to_string());
        path.push(task.to_string());

        let blocked = self.get_blocked_tasks(task);
        for next_task in blocked {
            if !visited.contains(&next_task) {
                self.dfs_cycle_detection(&next_task, visited, rec_stack, path, cycles);
            } else if rec_stack.contains(&next_task) {
                // Found a cycle
                if let Some(start_idx) = path.iter().position(|t| t == &next_task) {
                    cycles.push(path[start_idx..].to_vec());
                }
            }
        }

        path.pop();
        rec_stack.remove(task);
    }

    /// Gets total dependency count
    pub fn total_dependencies(&self) -> usize {
        self.dependencies.len()
    }
}

impl Default for DependencyManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dependency_type_name() {
        assert_eq!(DependencyType::Blocks.name(), "Blocks");
        assert_eq!(DependencyType::RelatesTo.name(), "Relates To");
    }

    #[test]
    fn test_dependency_type_inverse() {
        assert_eq!(
            DependencyType::Blocks.inverse(),
            Some(DependencyType::BlockedBy)
        );
        assert_eq!(
            DependencyType::BlockedBy.inverse(),
            Some(DependencyType::Blocks)
        );
        assert_eq!(
            DependencyType::RelatesTo.inverse(),
            Some(DependencyType::RelatesTo)
        );
        assert_eq!(DependencyType::Duplicates.inverse(), None);
    }

    #[test]
    fn test_dependency_type_affects_scheduling() {
        assert!(DependencyType::Blocks.affects_scheduling());
        assert!(DependencyType::BlockedBy.affects_scheduling());
        assert!(!DependencyType::RelatesTo.affects_scheduling());
        assert!(!DependencyType::Duplicates.affects_scheduling());
    }

    #[test]
    fn test_create_dependency() {
        let mut manager = DependencyManager::new();
        let dep_id = manager
            .create_dependency(
                "task-1".to_string(),
                "task-2".to_string(),
                DependencyType::Blocks,
                "user-1".to_string(),
            )
            .unwrap();

        assert_eq!(dep_id, "dep-1");
        let dep = manager.get_dependency(&dep_id).unwrap();
        assert_eq!(dep.from_task, "task-1");
        assert_eq!(dep.to_task, "task-2");
        assert_eq!(dep.dependency_type, DependencyType::Blocks);
    }

    #[test]
    fn test_circular_dependency_detection() {
        let mut manager = DependencyManager::new();

        // Create: task-1 blocks task-2
        manager
            .create_dependency(
                "task-1".to_string(),
                "task-2".to_string(),
                DependencyType::Blocks,
                "user-1".to_string(),
            )
            .unwrap();

        // Try to create: task-2 blocks task-1 (would create cycle)
        let result = manager.create_dependency(
            "task-2".to_string(),
            "task-1".to_string(),
            DependencyType::Blocks,
            "user-1".to_string(),
        );

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("circular dependency"));
    }

    #[test]
    fn test_delete_dependency() {
        let mut manager = DependencyManager::new();
        let dep_id = manager
            .create_dependency(
                "task-1".to_string(),
                "task-2".to_string(),
                DependencyType::Blocks,
                "user-1".to_string(),
            )
            .unwrap();

        assert_eq!(manager.total_dependencies(), 1);

        let deleted = manager.delete_dependency(&dep_id);
        assert!(deleted.is_some());
        assert_eq!(manager.total_dependencies(), 0);
    }

    #[test]
    fn test_dependencies_for_task() {
        let mut manager = DependencyManager::new();
        manager
            .create_dependency(
                "task-1".to_string(),
                "task-2".to_string(),
                DependencyType::Blocks,
                "user-1".to_string(),
            )
            .unwrap();
        manager
            .create_dependency(
                "task-1".to_string(),
                "task-3".to_string(),
                DependencyType::Blocks,
                "user-1".to_string(),
            )
            .unwrap();

        let deps = manager.dependencies_for_task("task-1");
        assert_eq!(deps.len(), 2);
    }

    #[test]
    fn test_get_blockers() {
        let mut manager = DependencyManager::new();
        manager
            .create_dependency(
                "task-1".to_string(),
                "task-2".to_string(),
                DependencyType::Blocks,
                "user-1".to_string(),
            )
            .unwrap();

        let blockers = manager.get_blockers("task-2");
        assert_eq!(blockers.len(), 1);
        assert_eq!(blockers[0].from_task, "task-1");
    }

    #[test]
    fn test_get_blocked() {
        let mut manager = DependencyManager::new();
        manager
            .create_dependency(
                "task-1".to_string(),
                "task-2".to_string(),
                DependencyType::Blocks,
                "user-1".to_string(),
            )
            .unwrap();

        let blocked = manager.get_blocked("task-1");
        assert_eq!(blocked.len(), 1);
        assert_eq!(blocked[0].to_task, "task-2");
    }

    #[test]
    fn test_critical_path_node() {
        let mut node = CriticalPathNode::new("task-1".to_string(), 5.0);
        node.earliest_start = 0.0;
        node.latest_start = 0.0;
        node.update_slack();

        assert_eq!(node.earliest_finish(), 5.0);
        assert_eq!(node.slack, 0.0);
        assert!(node.is_critical);
    }

    #[test]
    fn test_critical_path_calculation() {
        let mut manager = DependencyManager::new();

        // task-1 (3 days) -> task-2 (2 days) -> task-3 (4 days)
        manager
            .create_dependency(
                "task-1".to_string(),
                "task-2".to_string(),
                DependencyType::Blocks,
                "user-1".to_string(),
            )
            .unwrap();
        manager
            .create_dependency(
                "task-2".to_string(),
                "task-3".to_string(),
                DependencyType::Blocks,
                "user-1".to_string(),
            )
            .unwrap();

        let mut durations = HashMap::new();
        durations.insert("task-1".to_string(), 3.0);
        durations.insert("task-2".to_string(), 2.0);
        durations.insert("task-3".to_string(), 4.0);

        let critical_path = manager.get_critical_path(durations.clone());

        // All tasks should be on critical path (linear dependency)
        assert_eq!(critical_path.len(), 3);
    }

    #[test]
    fn test_topological_sort() {
        let mut manager = DependencyManager::new();

        manager
            .create_dependency(
                "task-1".to_string(),
                "task-2".to_string(),
                DependencyType::Blocks,
                "user-1".to_string(),
            )
            .unwrap();
        manager
            .create_dependency(
                "task-2".to_string(),
                "task-3".to_string(),
                DependencyType::Blocks,
                "user-1".to_string(),
            )
            .unwrap();

        let mut durations = HashMap::new();
        durations.insert("task-1".to_string(), 1.0);
        durations.insert("task-2".to_string(), 1.0);
        durations.insert("task-3".to_string(), 1.0);

        let sorted = manager.topological_sort(&durations);

        // task-1 should come before task-2, task-2 before task-3
        let pos1 = sorted.iter().position(|t| t == "task-1").unwrap();
        let pos2 = sorted.iter().position(|t| t == "task-2").unwrap();
        let pos3 = sorted.iter().position(|t| t == "task-3").unwrap();

        assert!(pos1 < pos2);
        assert!(pos2 < pos3);
    }

    #[test]
    fn test_detect_cycles_no_cycle() {
        let mut manager = DependencyManager::new();

        manager
            .create_dependency(
                "task-1".to_string(),
                "task-2".to_string(),
                DependencyType::Blocks,
                "user-1".to_string(),
            )
            .unwrap();

        let cycles = manager.detect_cycles();
        assert_eq!(cycles.len(), 0);
    }

    #[test]
    fn test_relates_to_no_cycle_check() {
        let mut manager = DependencyManager::new();

        // RelatesTo should not trigger cycle detection
        manager
            .create_dependency(
                "task-1".to_string(),
                "task-2".to_string(),
                DependencyType::RelatesTo,
                "user-1".to_string(),
            )
            .unwrap();
        let result = manager.create_dependency(
            "task-2".to_string(),
            "task-1".to_string(),
            DependencyType::RelatesTo,
            "user-1".to_string(),
        );

        assert!(result.is_ok()); // Should allow bidirectional RelatesTo
    }

    #[test]
    fn test_complex_dependency_graph() {
        let mut manager = DependencyManager::new();

        // Create a diamond dependency:
        // task-1 -> task-2 -> task-4
        // task-1 -> task-3 -> task-4

        manager
            .create_dependency(
                "task-1".to_string(),
                "task-2".to_string(),
                DependencyType::Blocks,
                "user-1".to_string(),
            )
            .unwrap();
        manager
            .create_dependency(
                "task-1".to_string(),
                "task-3".to_string(),
                DependencyType::Blocks,
                "user-1".to_string(),
            )
            .unwrap();
        manager
            .create_dependency(
                "task-2".to_string(),
                "task-4".to_string(),
                DependencyType::Blocks,
                "user-1".to_string(),
            )
            .unwrap();
        manager
            .create_dependency(
                "task-3".to_string(),
                "task-4".to_string(),
                DependencyType::Blocks,
                "user-1".to_string(),
            )
            .unwrap();

        let mut durations = HashMap::new();
        durations.insert("task-1".to_string(), 2.0);
        durations.insert("task-2".to_string(), 3.0);
        durations.insert("task-3".to_string(), 1.0);
        durations.insert("task-4".to_string(), 2.0);

        let nodes = manager.calculate_critical_path(durations);

        // task-4 should have earliest_start = 5.0 (max of task-2 and task-3 paths)
        let task4_node = nodes.iter().find(|n| n.task_id == "task-4").unwrap();
        assert_eq!(task4_node.earliest_start, 5.0); // 2 (task-1) + 3 (task-2)
    }

    #[test]
    fn test_three_way_cycle_prevention() {
        let mut manager = DependencyManager::new();

        // Create: A -> B -> C
        manager
            .create_dependency(
                "A".to_string(),
                "B".to_string(),
                DependencyType::Blocks,
                "user-1".to_string(),
            )
            .unwrap();
        manager
            .create_dependency(
                "B".to_string(),
                "C".to_string(),
                DependencyType::Blocks,
                "user-1".to_string(),
            )
            .unwrap();

        // Try to create: C -> A (would create cycle A -> B -> C -> A)
        let result = manager.create_dependency(
            "C".to_string(),
            "A".to_string(),
            DependencyType::Blocks,
            "user-1".to_string(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_slack_calculation() {
        let mut manager = DependencyManager::new();

        // Create two parallel paths with different durations
        // task-1 (5 days) -> task-3 (1 day)
        // task-2 (2 days) -> task-3 (1 day)

        manager
            .create_dependency(
                "task-1".to_string(),
                "task-3".to_string(),
                DependencyType::Blocks,
                "user-1".to_string(),
            )
            .unwrap();
        manager
            .create_dependency(
                "task-2".to_string(),
                "task-3".to_string(),
                DependencyType::Blocks,
                "user-1".to_string(),
            )
            .unwrap();

        let mut durations = HashMap::new();
        durations.insert("task-1".to_string(), 5.0);
        durations.insert("task-2".to_string(), 2.0);
        durations.insert("task-3".to_string(), 1.0);

        let nodes = manager.calculate_critical_path(durations);

        // task-2 should have slack (shorter path)
        let task2_node = nodes.iter().find(|n| n.task_id == "task-2").unwrap();
        assert!(task2_node.slack > 0.0);

        // task-1 and task-3 should be on critical path
        let task1_node = nodes.iter().find(|n| n.task_id == "task-1").unwrap();
        assert!(task1_node.is_critical);
    }
}
