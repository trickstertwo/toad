//! Plugin System
//!
//! Extensibility framework for TOAD allowing custom plugins and scripts.
//! Supports plugin discovery, lifecycle management, and event hooks.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Plugin runtime type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PluginRuntime {
    /// Native Rust plugin (compiled)
    Native,
    /// WebAssembly plugin
    Wasm,
    /// Lua script
    Lua,
    /// Python script
    Python,
    /// JavaScript (via QuickJS or similar)
    JavaScript,
}

impl PluginRuntime {
    /// Get the display name for the runtime
    pub fn name(&self) -> &'static str {
        match self {
            PluginRuntime::Native => "Native",
            PluginRuntime::Wasm => "WebAssembly",
            PluginRuntime::Lua => "Lua",
            PluginRuntime::Python => "Python",
            PluginRuntime::JavaScript => "JavaScript",
        }
    }

    /// Get file extension for this runtime
    pub fn extension(&self) -> &'static str {
        match self {
            PluginRuntime::Native => "so",
            PluginRuntime::Wasm => "wasm",
            PluginRuntime::Lua => "lua",
            PluginRuntime::Python => "py",
            PluginRuntime::JavaScript => "js",
        }
    }
}

/// Plugin capability/permission
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PluginCapability {
    /// Read task data
    ReadTasks,
    /// Write/modify task data
    WriteTasks,
    /// Read file system
    ReadFiles,
    /// Write to file system
    WriteFiles,
    /// Network access
    Network,
    /// Execute shell commands
    Shell,
    /// Access clipboard
    Clipboard,
    /// Modify UI
    UI,
}

impl PluginCapability {
    /// Get the display name for the capability
    pub fn name(&self) -> &'static str {
        match self {
            PluginCapability::ReadTasks => "Read Tasks",
            PluginCapability::WriteTasks => "Write Tasks",
            PluginCapability::ReadFiles => "Read Files",
            PluginCapability::WriteFiles => "Write Files",
            PluginCapability::Network => "Network",
            PluginCapability::Shell => "Shell",
            PluginCapability::Clipboard => "Clipboard",
            PluginCapability::UI => "UI",
        }
    }

    /// Whether this capability is dangerous and requires user confirmation
    pub fn is_dangerous(&self) -> bool {
        matches!(
            self,
            PluginCapability::WriteFiles | PluginCapability::Shell | PluginCapability::Network
        )
    }
}

/// Plugin metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
    /// Plugin identifier
    pub id: String,
    /// Plugin name
    pub name: String,
    /// Plugin version
    pub version: String,
    /// Plugin author
    pub author: String,
    /// Plugin description
    pub description: String,
    /// Runtime type
    pub runtime: PluginRuntime,
    /// Required capabilities
    pub capabilities: Vec<PluginCapability>,
    /// Plugin dependencies (other plugin IDs)
    pub dependencies: Vec<String>,
    /// Homepage or repository URL
    pub homepage: Option<String>,
}

impl PluginMetadata {
    /// Create new plugin metadata
    pub fn new(id: String, name: String, version: String, runtime: PluginRuntime) -> Self {
        Self {
            id,
            name,
            version,
            author: String::new(),
            description: String::new(),
            runtime,
            capabilities: Vec::new(),
            dependencies: Vec::new(),
            homepage: None,
        }
    }

    /// Check if plugin has a specific capability
    pub fn has_capability(&self, capability: PluginCapability) -> bool {
        self.capabilities.contains(&capability)
    }

    /// Get dangerous capabilities required by this plugin
    pub fn dangerous_capabilities(&self) -> Vec<&PluginCapability> {
        self.capabilities
            .iter()
            .filter(|cap| cap.is_dangerous())
            .collect()
    }
}

/// Plugin lifecycle state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PluginState {
    /// Plugin is discovered but not loaded
    Discovered,
    /// Plugin is loaded but not initialized
    Loaded,
    /// Plugin is initialized and ready
    Ready,
    /// Plugin is currently running
    Running,
    /// Plugin is paused
    Paused,
    /// Plugin is disabled
    Disabled,
    /// Plugin failed to load or run
    Error,
}

impl PluginState {
    /// Get the display name for the state
    pub fn name(&self) -> &'static str {
        match self {
            PluginState::Discovered => "Discovered",
            PluginState::Loaded => "Loaded",
            PluginState::Ready => "Ready",
            PluginState::Running => "Running",
            PluginState::Paused => "Paused",
            PluginState::Disabled => "Disabled",
            PluginState::Error => "Error",
        }
    }
}

/// Plugin event hook type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PluginHook {
    /// Called when TOAD starts
    OnStartup,
    /// Called when TOAD shuts down
    OnShutdown,
    /// Called when a task is created
    OnTaskCreated,
    /// Called when a task is updated
    OnTaskUpdated,
    /// Called when a task is deleted
    OnTaskDeleted,
    /// Called when a task is completed
    OnTaskCompleted,
    /// Called before rendering
    OnPreRender,
    /// Called after rendering
    OnPostRender,
    /// Called on keyboard input
    OnKeyPress,
    /// Called on mouse input
    OnMouseEvent,
}

impl PluginHook {
    /// Get the display name for the hook
    pub fn name(&self) -> &'static str {
        match self {
            PluginHook::OnStartup => "On Startup",
            PluginHook::OnShutdown => "On Shutdown",
            PluginHook::OnTaskCreated => "On Task Created",
            PluginHook::OnTaskUpdated => "On Task Updated",
            PluginHook::OnTaskDeleted => "On Task Deleted",
            PluginHook::OnTaskCompleted => "On Task Completed",
            PluginHook::OnPreRender => "On Pre-Render",
            PluginHook::OnPostRender => "On Post-Render",
            PluginHook::OnKeyPress => "On Key Press",
            PluginHook::OnMouseEvent => "On Mouse Event",
        }
    }
}

/// Registered plugin
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plugin {
    /// Plugin metadata
    pub metadata: PluginMetadata,
    /// Current state
    pub state: PluginState,
    /// File path to plugin
    pub path: String,
    /// When the plugin was loaded
    pub loaded_at: Option<DateTime<Utc>>,
    /// When the plugin was last executed
    pub last_executed_at: Option<DateTime<Utc>>,
    /// Number of times the plugin has been executed
    pub execution_count: usize,
    /// Error message if state is Error
    pub error_message: Option<String>,
    /// Hooks this plugin subscribes to
    pub hooks: Vec<PluginHook>,
}

impl Plugin {
    /// Create a new plugin
    pub fn new(metadata: PluginMetadata, path: String) -> Self {
        Self {
            metadata,
            state: PluginState::Discovered,
            path,
            loaded_at: None,
            last_executed_at: None,
            execution_count: 0,
            error_message: None,
            hooks: Vec::new(),
        }
    }

    /// Load the plugin
    pub fn load(&mut self) -> Result<(), String> {
        if self.state != PluginState::Discovered {
            return Err(format!("Plugin is in state {:?}, expected Discovered", self.state));
        }

        self.state = PluginState::Loaded;
        self.loaded_at = Some(Utc::now());
        Ok(())
    }

    /// Initialize the plugin
    pub fn initialize(&mut self) -> Result<(), String> {
        if self.state != PluginState::Loaded {
            return Err(format!("Plugin is in state {:?}, expected Loaded", self.state));
        }

        self.state = PluginState::Ready;
        Ok(())
    }

    /// Enable the plugin
    pub fn enable(&mut self) -> Result<(), String> {
        if self.state == PluginState::Error {
            return Err("Cannot enable plugin in error state".to_string());
        }

        self.state = PluginState::Running;
        Ok(())
    }

    /// Disable the plugin
    pub fn disable(&mut self) {
        self.state = PluginState::Disabled;
    }

    /// Pause the plugin
    pub fn pause(&mut self) -> Result<(), String> {
        if self.state != PluginState::Running {
            return Err(format!("Plugin is in state {:?}, expected Running", self.state));
        }

        self.state = PluginState::Paused;
        Ok(())
    }

    /// Resume the plugin
    pub fn resume(&mut self) -> Result<(), String> {
        if self.state != PluginState::Paused {
            return Err(format!("Plugin is in state {:?}, expected Paused", self.state));
        }

        self.state = PluginState::Running;
        Ok(())
    }

    /// Mark plugin as errored
    pub fn set_error(&mut self, error: String) {
        self.state = PluginState::Error;
        self.error_message = Some(error);
    }

    /// Record an execution
    pub fn record_execution(&mut self) {
        self.execution_count += 1;
        self.last_executed_at = Some(Utc::now());
    }

    /// Subscribe to a hook
    pub fn subscribe_to_hook(&mut self, hook: PluginHook) {
        if !self.hooks.contains(&hook) {
            self.hooks.push(hook);
        }
    }

    /// Unsubscribe from a hook
    pub fn unsubscribe_from_hook(&mut self, hook: PluginHook) {
        self.hooks.retain(|h| *h != hook);
    }

    /// Check if plugin is subscribed to a hook
    pub fn is_subscribed_to(&self, hook: PluginHook) -> bool {
        self.hooks.contains(&hook)
    }
}

/// Plugin manager
#[derive(Debug)]
pub struct PluginManager {
    /// All registered plugins
    plugins: HashMap<String, Plugin>,
    /// Enabled plugins by hook
    hooks: HashMap<PluginHook, Vec<String>>,
    /// Plugin load order (for dependencies)
    load_order: Vec<String>,
}

impl PluginManager {
    /// Create a new plugin manager
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
            hooks: HashMap::new(),
            load_order: Vec::new(),
        }
    }

    /// Register a plugin
    pub fn register_plugin(&mut self, plugin: Plugin) -> Result<(), String> {
        let plugin_id = plugin.metadata.id.clone();

        if self.plugins.contains_key(&plugin_id) {
            return Err(format!("Plugin {} is already registered", plugin_id));
        }

        self.plugins.insert(plugin_id.clone(), plugin);
        Ok(())
    }

    /// Get a plugin by ID
    pub fn get_plugin(&self, plugin_id: &str) -> Option<&Plugin> {
        self.plugins.get(plugin_id)
    }

    /// Get a mutable plugin by ID
    pub fn get_plugin_mut(&mut self, plugin_id: &str) -> Option<&mut Plugin> {
        self.plugins.get_mut(plugin_id)
    }

    /// Get all plugins
    pub fn get_all_plugins(&self) -> Vec<&Plugin> {
        self.plugins.values().collect()
    }

    /// Get plugins by state
    pub fn get_plugins_by_state(&self, state: PluginState) -> Vec<&Plugin> {
        self.plugins
            .values()
            .filter(|p| p.state == state)
            .collect()
    }

    /// Get plugins by runtime
    pub fn get_plugins_by_runtime(&self, runtime: PluginRuntime) -> Vec<&Plugin> {
        self.plugins
            .values()
            .filter(|p| p.metadata.runtime == runtime)
            .collect()
    }

    /// Load a plugin
    pub fn load_plugin(&mut self, plugin_id: &str) -> Result<(), String> {
        if let Some(plugin) = self.plugins.get_mut(plugin_id) {
            plugin.load()?;
            self.load_order.push(plugin_id.to_string());
            Ok(())
        } else {
            Err(format!("Plugin {} not found", plugin_id))
        }
    }

    /// Initialize a plugin
    pub fn initialize_plugin(&mut self, plugin_id: &str) -> Result<(), String> {
        if let Some(plugin) = self.plugins.get_mut(plugin_id) {
            plugin.initialize()
        } else {
            Err(format!("Plugin {} not found", plugin_id))
        }
    }

    /// Enable a plugin
    pub fn enable_plugin(&mut self, plugin_id: &str) -> Result<(), String> {
        if let Some(plugin) = self.plugins.get_mut(plugin_id) {
            plugin.enable()?;

            // Register plugin hooks
            for hook in &plugin.hooks.clone() {
                self.hooks
                    .entry(*hook)
                    .or_insert_with(Vec::new)
                    .push(plugin_id.to_string());
            }

            Ok(())
        } else {
            Err(format!("Plugin {} not found", plugin_id))
        }
    }

    /// Disable a plugin
    pub fn disable_plugin(&mut self, plugin_id: &str) -> Result<(), String> {
        if let Some(plugin) = self.plugins.get_mut(plugin_id) {
            plugin.disable();

            // Unregister plugin hooks
            for hook_plugins in self.hooks.values_mut() {
                hook_plugins.retain(|id| id != plugin_id);
            }

            Ok(())
        } else {
            Err(format!("Plugin {} not found", plugin_id))
        }
    }

    /// Get plugins subscribed to a hook
    pub fn get_plugins_for_hook(&self, hook: PluginHook) -> Vec<&Plugin> {
        self.hooks
            .get(&hook)
            .map(|plugin_ids| {
                plugin_ids
                    .iter()
                    .filter_map(|id| self.plugins.get(id))
                    .filter(|p| p.state == PluginState::Running)
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Execute all plugins for a hook
    pub fn execute_hook(&mut self, hook: PluginHook) -> Result<usize, String> {
        let plugin_ids: Vec<String> = self
            .hooks
            .get(&hook)
            .map(|ids| ids.clone())
            .unwrap_or_default();

        let mut executed_count = 0;

        for plugin_id in plugin_ids {
            if let Some(plugin) = self.plugins.get_mut(&plugin_id) {
                if plugin.state == PluginState::Running {
                    plugin.record_execution();
                    executed_count += 1;
                }
            }
        }

        Ok(executed_count)
    }

    /// Check plugin dependencies are satisfied
    pub fn check_dependencies(&self, plugin_id: &str) -> Result<(), String> {
        if let Some(plugin) = self.plugins.get(plugin_id) {
            for dep_id in &plugin.metadata.dependencies {
                if let Some(dep_plugin) = self.plugins.get(dep_id) {
                    if dep_plugin.state != PluginState::Running && dep_plugin.state != PluginState::Ready {
                        return Err(format!(
                            "Dependency {} is not ready (state: {:?})",
                            dep_id, dep_plugin.state
                        ));
                    }
                } else {
                    return Err(format!("Dependency {} not found", dep_id));
                }
            }
            Ok(())
        } else {
            Err(format!("Plugin {} not found", plugin_id))
        }
    }

    /// Get enabled plugin count
    pub fn enabled_count(&self) -> usize {
        self.plugins
            .values()
            .filter(|p| p.state == PluginState::Running)
            .count()
    }

    /// Get total plugin count
    pub fn total_count(&self) -> usize {
        self.plugins.len()
    }

    /// Reload a plugin
    pub fn reload_plugin(&mut self, plugin_id: &str) -> Result<(), String> {
        // Disable first
        self.disable_plugin(plugin_id)?;

        // Reset state to discovered
        if let Some(plugin) = self.plugins.get_mut(plugin_id) {
            plugin.state = PluginState::Discovered;
            plugin.loaded_at = None;
        }

        // Load and enable again
        self.load_plugin(plugin_id)?;
        self.initialize_plugin(plugin_id)?;
        self.enable_plugin(plugin_id)?;

        Ok(())
    }

    /// Unregister a plugin
    pub fn unregister_plugin(&mut self, plugin_id: &str) -> Result<(), String> {
        // Disable first
        let _ = self.disable_plugin(plugin_id);

        // Remove from load order
        self.load_order.retain(|id| id != plugin_id);

        // Remove from plugins
        if self.plugins.remove(plugin_id).is_some() {
            Ok(())
        } else {
            Err(format!("Plugin {} not found", plugin_id))
        }
    }
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_metadata() -> PluginMetadata {
        PluginMetadata::new(
            "test-plugin".to_string(),
            "Test Plugin".to_string(),
            "1.0.0".to_string(),
            PluginRuntime::Native,
        )
    }

    #[test]
    fn test_plugin_runtime_name() {
        assert_eq!(PluginRuntime::Native.name(), "Native");
        assert_eq!(PluginRuntime::Wasm.name(), "WebAssembly");
        assert_eq!(PluginRuntime::Lua.name(), "Lua");
    }

    #[test]
    fn test_plugin_runtime_extension() {
        assert_eq!(PluginRuntime::Native.extension(), "so");
        assert_eq!(PluginRuntime::Wasm.extension(), "wasm");
        assert_eq!(PluginRuntime::Lua.extension(), "lua");
    }

    #[test]
    fn test_plugin_capability_name() {
        assert_eq!(PluginCapability::ReadTasks.name(), "Read Tasks");
        assert_eq!(PluginCapability::WriteTasks.name(), "Write Tasks");
    }

    #[test]
    fn test_plugin_capability_is_dangerous() {
        assert!(!PluginCapability::ReadTasks.is_dangerous());
        assert!(PluginCapability::WriteFiles.is_dangerous());
        assert!(PluginCapability::Shell.is_dangerous());
        assert!(PluginCapability::Network.is_dangerous());
    }

    #[test]
    fn test_plugin_metadata_creation() {
        let metadata = create_test_metadata();
        assert_eq!(metadata.id, "test-plugin");
        assert_eq!(metadata.name, "Test Plugin");
        assert_eq!(metadata.version, "1.0.0");
        assert_eq!(metadata.runtime, PluginRuntime::Native);
    }

    #[test]
    fn test_plugin_metadata_has_capability() {
        let mut metadata = create_test_metadata();
        metadata.capabilities.push(PluginCapability::ReadTasks);

        assert!(metadata.has_capability(PluginCapability::ReadTasks));
        assert!(!metadata.has_capability(PluginCapability::WriteTasks));
    }

    #[test]
    fn test_plugin_metadata_dangerous_capabilities() {
        let mut metadata = create_test_metadata();
        metadata.capabilities.push(PluginCapability::ReadTasks);
        metadata.capabilities.push(PluginCapability::WriteFiles);
        metadata.capabilities.push(PluginCapability::Shell);

        let dangerous = metadata.dangerous_capabilities();
        assert_eq!(dangerous.len(), 2);
    }

    #[test]
    fn test_plugin_state_name() {
        assert_eq!(PluginState::Discovered.name(), "Discovered");
        assert_eq!(PluginState::Running.name(), "Running");
        assert_eq!(PluginState::Error.name(), "Error");
    }

    #[test]
    fn test_plugin_creation() {
        let metadata = create_test_metadata();
        let plugin = Plugin::new(metadata, "/path/to/plugin.so".to_string());

        assert_eq!(plugin.state, PluginState::Discovered);
        assert_eq!(plugin.path, "/path/to/plugin.so");
        assert_eq!(plugin.execution_count, 0);
    }

    #[test]
    fn test_plugin_lifecycle() {
        let metadata = create_test_metadata();
        let mut plugin = Plugin::new(metadata, "/path/to/plugin.so".to_string());

        // Load
        assert!(plugin.load().is_ok());
        assert_eq!(plugin.state, PluginState::Loaded);
        assert!(plugin.loaded_at.is_some());

        // Initialize
        assert!(plugin.initialize().is_ok());
        assert_eq!(plugin.state, PluginState::Ready);

        // Enable
        assert!(plugin.enable().is_ok());
        assert_eq!(plugin.state, PluginState::Running);

        // Pause
        assert!(plugin.pause().is_ok());
        assert_eq!(plugin.state, PluginState::Paused);

        // Resume
        assert!(plugin.resume().is_ok());
        assert_eq!(plugin.state, PluginState::Running);

        // Disable
        plugin.disable();
        assert_eq!(plugin.state, PluginState::Disabled);
    }

    #[test]
    fn test_plugin_error_state() {
        let metadata = create_test_metadata();
        let mut plugin = Plugin::new(metadata, "/path/to/plugin.so".to_string());

        plugin.set_error("Something went wrong".to_string());
        assert_eq!(plugin.state, PluginState::Error);
        assert_eq!(plugin.error_message, Some("Something went wrong".to_string()));

        // Cannot enable from error state
        assert!(plugin.enable().is_err());
    }

    #[test]
    fn test_plugin_execution_tracking() {
        let metadata = create_test_metadata();
        let mut plugin = Plugin::new(metadata, "/path/to/plugin.so".to_string());

        assert_eq!(plugin.execution_count, 0);
        assert!(plugin.last_executed_at.is_none());

        plugin.record_execution();
        assert_eq!(plugin.execution_count, 1);
        assert!(plugin.last_executed_at.is_some());

        plugin.record_execution();
        assert_eq!(plugin.execution_count, 2);
    }

    #[test]
    fn test_plugin_hooks() {
        let metadata = create_test_metadata();
        let mut plugin = Plugin::new(metadata, "/path/to/plugin.so".to_string());

        plugin.subscribe_to_hook(PluginHook::OnTaskCreated);
        assert!(plugin.is_subscribed_to(PluginHook::OnTaskCreated));
        assert!(!plugin.is_subscribed_to(PluginHook::OnTaskUpdated));

        plugin.unsubscribe_from_hook(PluginHook::OnTaskCreated);
        assert!(!plugin.is_subscribed_to(PluginHook::OnTaskCreated));
    }

    #[test]
    fn test_manager_register_plugin() {
        let mut manager = PluginManager::new();
        let metadata = create_test_metadata();
        let plugin = Plugin::new(metadata, "/path/to/plugin.so".to_string());

        assert!(manager.register_plugin(plugin).is_ok());
        assert_eq!(manager.total_count(), 1);
    }

    #[test]
    fn test_manager_duplicate_registration() {
        let mut manager = PluginManager::new();
        let metadata = create_test_metadata();
        let plugin1 = Plugin::new(metadata.clone(), "/path/to/plugin.so".to_string());
        let plugin2 = Plugin::new(metadata, "/path/to/plugin.so".to_string());

        assert!(manager.register_plugin(plugin1).is_ok());
        assert!(manager.register_plugin(plugin2).is_err());
    }

    #[test]
    fn test_manager_load_plugin() {
        let mut manager = PluginManager::new();
        let metadata = create_test_metadata();
        let plugin = Plugin::new(metadata, "/path/to/plugin.so".to_string());

        manager.register_plugin(plugin).unwrap();
        assert!(manager.load_plugin("test-plugin").is_ok());

        let loaded_plugin = manager.get_plugin("test-plugin").unwrap();
        assert_eq!(loaded_plugin.state, PluginState::Loaded);
    }

    #[test]
    fn test_manager_enable_disable_plugin() {
        let mut manager = PluginManager::new();
        let metadata = create_test_metadata();
        let plugin = Plugin::new(metadata, "/path/to/plugin.so".to_string());

        manager.register_plugin(plugin).unwrap();
        manager.load_plugin("test-plugin").unwrap();
        manager.initialize_plugin("test-plugin").unwrap();
        manager.enable_plugin("test-plugin").unwrap();

        assert_eq!(manager.enabled_count(), 1);

        manager.disable_plugin("test-plugin").unwrap();
        assert_eq!(manager.enabled_count(), 0);
    }

    #[test]
    fn test_manager_get_plugins_by_state() {
        let mut manager = PluginManager::new();
        let metadata1 = create_test_metadata();
        let mut metadata2 = create_test_metadata();
        metadata2.id = "test-plugin-2".to_string();

        let plugin1 = Plugin::new(metadata1, "/path/to/plugin1.so".to_string());
        let plugin2 = Plugin::new(metadata2, "/path/to/plugin2.so".to_string());

        manager.register_plugin(plugin1).unwrap();
        manager.register_plugin(plugin2).unwrap();

        let discovered = manager.get_plugins_by_state(PluginState::Discovered);
        assert_eq!(discovered.len(), 2);
    }

    #[test]
    fn test_manager_hooks() {
        let mut manager = PluginManager::new();
        let metadata = create_test_metadata();
        let mut plugin = Plugin::new(metadata, "/path/to/plugin.so".to_string());

        plugin.subscribe_to_hook(PluginHook::OnTaskCreated);
        manager.register_plugin(plugin).unwrap();
        manager.load_plugin("test-plugin").unwrap();
        manager.initialize_plugin("test-plugin").unwrap();
        manager.enable_plugin("test-plugin").unwrap();

        let hook_plugins = manager.get_plugins_for_hook(PluginHook::OnTaskCreated);
        assert_eq!(hook_plugins.len(), 1);
    }

    #[test]
    fn test_manager_execute_hook() {
        let mut manager = PluginManager::new();
        let metadata = create_test_metadata();
        let mut plugin = Plugin::new(metadata, "/path/to/plugin.so".to_string());

        plugin.subscribe_to_hook(PluginHook::OnTaskCreated);
        manager.register_plugin(plugin).unwrap();
        manager.load_plugin("test-plugin").unwrap();
        manager.initialize_plugin("test-plugin").unwrap();
        manager.enable_plugin("test-plugin").unwrap();

        let executed = manager.execute_hook(PluginHook::OnTaskCreated).unwrap();
        assert_eq!(executed, 1);

        let plugin = manager.get_plugin("test-plugin").unwrap();
        assert_eq!(plugin.execution_count, 1);
    }

    #[test]
    fn test_manager_check_dependencies() {
        let mut manager = PluginManager::new();

        // Create dependent plugin
        let mut metadata1 = create_test_metadata();
        metadata1.id = "plugin-1".to_string();
        let plugin1 = Plugin::new(metadata1, "/path/to/plugin1.so".to_string());

        // Create plugin with dependency
        let mut metadata2 = create_test_metadata();
        metadata2.id = "plugin-2".to_string();
        metadata2.dependencies.push("plugin-1".to_string());
        let plugin2 = Plugin::new(metadata2, "/path/to/plugin2.so".to_string());

        manager.register_plugin(plugin1).unwrap();
        manager.register_plugin(plugin2).unwrap();

        // Dependency not ready
        assert!(manager.check_dependencies("plugin-2").is_err());

        // Load dependency
        manager.load_plugin("plugin-1").unwrap();
        manager.initialize_plugin("plugin-1").unwrap();

        // Now dependency is ready
        assert!(manager.check_dependencies("plugin-2").is_ok());
    }

    #[test]
    fn test_manager_reload_plugin() {
        let mut manager = PluginManager::new();
        let metadata = create_test_metadata();
        let plugin = Plugin::new(metadata, "/path/to/plugin.so".to_string());

        manager.register_plugin(plugin).unwrap();
        manager.load_plugin("test-plugin").unwrap();
        manager.initialize_plugin("test-plugin").unwrap();
        manager.enable_plugin("test-plugin").unwrap();

        assert!(manager.reload_plugin("test-plugin").is_ok());

        let reloaded = manager.get_plugin("test-plugin").unwrap();
        assert_eq!(reloaded.state, PluginState::Running);
    }

    #[test]
    fn test_manager_unregister_plugin() {
        let mut manager = PluginManager::new();
        let metadata = create_test_metadata();
        let plugin = Plugin::new(metadata, "/path/to/plugin.so".to_string());

        manager.register_plugin(plugin).unwrap();
        assert_eq!(manager.total_count(), 1);

        manager.unregister_plugin("test-plugin").unwrap();
        assert_eq!(manager.total_count(), 0);
    }
}
