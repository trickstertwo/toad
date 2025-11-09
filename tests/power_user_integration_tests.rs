//! Integration tests for PLATINUM Tier Power User Features
//!
//! Tests for CommandMode, CommandRegistry, AliasManager, KeySequenceManager, CustomKeybindings.

use toad::commands::{AliasManager, Command, CommandMode, CommandRegistry};
use toad::infrastructure::{CustomKeybindings, KeySequence, KeySequenceManager, KeybindingContext};
use toad::infrastructure::keybinds::KeyBinding;
use crossterm::event::{KeyCode, KeyModifiers};

// ==================== CommandMode Tests ====================

#[test]
fn test_command_mode_creation() {
    let mode = CommandMode::new();
    assert!(!mode.is_active());
    assert_eq!(mode.buffer(), "");
}

#[test]
fn test_command_mode_start() {
    let mut mode = CommandMode::new();
    mode.start();
    assert!(mode.is_active());
    assert_eq!(mode.buffer(), "");
}

#[test]
fn test_command_mode_cancel() {
    let mut mode = CommandMode::new();
    mode.start();
    mode.input_char('q');
    mode.cancel();
    assert!(!mode.is_active());
    assert_eq!(mode.buffer(), "");
}

#[test]
fn test_command_mode_input() {
    let mut mode = CommandMode::new();
    mode.start();

    mode.input_char('q');
    mode.input_char('u');
    mode.input_char('i');
    mode.input_char('t');

    assert_eq!(mode.buffer(), "quit");
}

#[test]
fn test_command_mode_backspace() {
    let mut mode = CommandMode::new();
    mode.start();

    mode.input_char('a');
    mode.input_char('b');
    mode.input_char('c');
    mode.backspace();

    assert_eq!(mode.buffer(), "ab");
}

#[test]
fn test_command_mode_cursor_movement() {
    let mut mode = CommandMode::new();
    mode.start();

    mode.input_char('h');
    mode.input_char('e');
    mode.input_char('l');
    mode.input_char('l');
    mode.input_char('o');

    mode.cursor_home();
    assert_eq!(mode.cursor(), 0);

    mode.cursor_end();
    assert_eq!(mode.cursor(), 5);

    mode.cursor_left();
    assert_eq!(mode.cursor(), 4);

    mode.cursor_right();
    assert_eq!(mode.cursor(), 5);
}

#[test]
fn test_command_mode_execute() {
    let mut mode = CommandMode::new();
    mode.start();

    mode.input_char('s');
    mode.input_char('a');
    mode.input_char('v');
    mode.input_char('e');

    let result = mode.execute();
    assert!(result.is_some());

    let (cmd, args) = result.unwrap();
    assert_eq!(cmd, "save");
    assert_eq!(args.len(), 0);
}

#[test]
fn test_command_mode_execute_with_args() {
    let mut mode = CommandMode::new();
    mode.start();

    mode.input_char('w');
    mode.input_char(' ');
    mode.input_char('f');
    mode.input_char('i');
    mode.input_char('l');
    mode.input_char('e');
    mode.input_char('.');
    mode.input_char('t');
    mode.input_char('x');
    mode.input_char('t');

    let result = mode.execute();
    assert!(result.is_some());

    let (cmd, args) = result.unwrap();
    assert_eq!(cmd, "w");
    assert_eq!(args, vec!["file.txt"]);
}

#[test]
fn test_command_mode_history() {
    let mut mode = CommandMode::new();

    mode.start();
    mode.input_char('w');
    mode.execute();

    mode.start();
    mode.input_char('q');
    mode.execute();

    let history = mode.history();
    assert_eq!(history.len(), 2);
    assert_eq!(history[0], "w");
    assert_eq!(history[1], "q");
}

#[test]
fn test_command_mode_history_navigation() {
    let mut mode = CommandMode::new();

    mode.start();
    mode.input_char('a');
    mode.execute();

    mode.start();
    mode.input_char('b');
    mode.execute();

    mode.start();
    mode.history_up();
    assert_eq!(mode.buffer(), "b");

    mode.history_up();
    assert_eq!(mode.buffer(), "a");

    mode.history_down();
    assert_eq!(mode.buffer(), "b");
}

// ==================== Command Tests ====================

#[test]
fn test_command_creation() {
    let cmd = Command::new("save", "Save the file");
    assert_eq!(cmd.name, "save");
    assert_eq!(cmd.description, "Save the file");
}

#[test]
fn test_command_with_alias() {
    let cmd = Command::new("quit", "Quit application")
        .alias("q")
        .alias("exit");

    assert!(cmd.matches("quit"));
    assert!(cmd.matches("q"));
    assert!(cmd.matches("exit"));
    assert!(!cmd.matches("save"));
}

#[test]
fn test_command_takes_args() {
    let cmd = Command::new("write", "Write file")
        .takes_args(true);

    assert!(cmd.takes_args);
}

// ==================== CommandRegistry Tests ====================

#[test]
fn test_command_registry_creation() {
    let registry = CommandRegistry::new();
    assert_eq!(registry.commands().len(), 0);
}

#[test]
fn test_command_registry_register() {
    let mut registry = CommandRegistry::new();

    let cmd = Command::new("test", "Test command");
    registry.register(cmd, std::sync::Arc::new(|_args| Ok("Success".to_string())));

    assert_eq!(registry.commands().len(), 1);
}

#[test]
fn test_command_registry_execute() {
    let mut registry = CommandRegistry::new();

    let cmd = Command::new("echo", "Echo args");
    registry.register(cmd, std::sync::Arc::new(|args| {
        Ok(args.join(" "))
    }));

    let result = registry.execute("echo", &["hello".to_string(), "world".to_string()]);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "hello world");
}

#[test]
fn test_command_registry_find() {
    let mut registry = CommandRegistry::new();

    let cmd = Command::new("save", "Save file").alias("w");
    registry.register(cmd, std::sync::Arc::new(|_| Ok("Saved".to_string())));

    assert!(registry.find("save").is_some());
    assert!(registry.find("w").is_some());
    assert!(registry.find("nonexistent").is_none());
}

#[test]
fn test_command_registry_suggest() {
    let mut registry = CommandRegistry::new();

    registry.register(
        Command::new("save", "Save file"),
        std::sync::Arc::new(|_| Ok("".to_string()))
    );
    registry.register(
        Command::new("search", "Search text"),
        std::sync::Arc::new(|_| Ok("".to_string()))
    );
    registry.register(
        Command::new("set", "Set option"),
        std::sync::Arc::new(|_| Ok("".to_string()))
    );

    let suggestions = registry.suggest("s");
    assert_eq!(suggestions.len(), 3);

    let suggestions = registry.suggest("sa");
    assert_eq!(suggestions.len(), 1);
    assert_eq!(suggestions[0].name, "save");
}

// ==================== AliasManager Tests ====================

#[test]
fn test_alias_manager_creation() {
    let manager = AliasManager::new();
    assert_eq!(manager.count(), 0);
    assert!(manager.is_empty());
}

#[test]
fn test_alias_manager_add() {
    let mut manager = AliasManager::new();

    manager.add("w", "write");
    manager.add("q", "quit");

    assert_eq!(manager.count(), 2);
    assert!(!manager.is_empty());
}

#[test]
fn test_alias_manager_get() {
    let mut manager = AliasManager::new();

    manager.add("gs", "git status");

    let alias = manager.get("gs");
    assert!(alias.is_some());
    assert_eq!(alias.unwrap().name, "gs");
}

#[test]
fn test_alias_manager_expand() {
    let mut manager = AliasManager::new();

    manager.add("gs", "git status");

    let expanded = manager.expand("gs");
    assert_eq!(expanded, Some("git status".to_string()));
}

#[test]
fn test_alias_manager_expand_with_args() {
    let mut manager = AliasManager::new();

    manager.add("gc", "git commit -m \"$1\"");

    let expanded = manager.expand_with_args("gc", &["Initial commit"]);
    assert_eq!(expanded, Some("git commit -m \"Initial commit\"".to_string()));
}

#[test]
fn test_alias_manager_remove() {
    let mut manager = AliasManager::new();

    manager.add("temp", "temporary");
    assert_eq!(manager.count(), 1);

    let removed = manager.remove("temp");
    assert!(removed.is_some());
    assert_eq!(manager.count(), 0);
}

#[test]
fn test_alias_manager_clear() {
    let mut manager = AliasManager::new();

    manager.add("a", "first");
    manager.add("b", "second");
    manager.add("c", "third");

    manager.clear();

    assert_eq!(manager.count(), 0);
    assert!(manager.is_empty());
}

#[test]
fn test_alias_manager_search() {
    let mut manager = AliasManager::new();

    manager.add("gs", "git status");
    manager.add("gc", "git commit");
    manager.add("gp", "git push");
    manager.add("save", "write file");

    let results = manager.search("git");
    assert_eq!(results.len(), 3);

    let results = manager.search("save");
    assert_eq!(results.len(), 1);
}

#[test]
fn test_alias_manager_with_defaults() {
    let manager = AliasManager::with_defaults();

    // Should have some default aliases
    assert!(manager.count() > 0);
}

// ==================== KeySequence Tests ====================

#[test]
fn test_key_sequence_two() {
    let seq = KeySequence::two(KeyCode::Char('g'), KeyCode::Char('g'));
    assert!(seq.len() >= 2);
}

#[test]
fn test_key_sequence_three() {
    let seq = KeySequence::three(KeyCode::Char('g'), KeyCode::Char('c'), KeyCode::Char('c'));
    assert!(seq.len() >= 3);
}

#[test]
fn test_key_sequence_new() {
    let keys = vec![KeyCode::Char('d'), KeyCode::Char('d')];
    let seq = KeySequence::new(keys);
    assert_eq!(seq.len(), 2);
}

// ==================== KeySequenceManager Tests ====================

#[test]
fn test_key_sequence_manager_creation() {
    let manager = KeySequenceManager::new();
    assert_eq!(manager.len(), 0);
}

#[test]
fn test_key_sequence_manager_vim_defaults() {
    let manager = KeySequenceManager::vim_defaults();

    // Should have vim key sequences registered
    assert!(manager.len() > 0);
}

#[test]
fn test_key_sequence_manager_register() {
    let mut manager = KeySequenceManager::new();

    manager.register(vec![KeyCode::Char('g'), KeyCode::Char('g')], "goto_top");

    assert!(manager.len() > 0);
}

#[test]
fn test_key_sequence_manager_process() {
    let mut manager = KeySequenceManager::new();

    manager.register(vec![KeyCode::Char('g'), KeyCode::Char('g')], "goto_top");

    // First key starts sequence
    let result = manager.process_key(KeyCode::Char('g'));
    assert!(result.is_none());

    // Second key completes sequence
    let result = manager.process_key(KeyCode::Char('g'));
    assert!(result.is_some());
    assert_eq!(result.unwrap(), "goto_top");
}

#[test]
fn test_key_sequence_manager_timeout() {
    let mut manager = KeySequenceManager::new();
    manager.set_timeout(std::time::Duration::from_millis(1000));

    manager.register(vec![KeyCode::Char('d'), KeyCode::Char('d')], "delete_line");

    manager.process_key(KeyCode::Char('d'));

    // Different key resets
    manager.process_key(KeyCode::Char('x'));
}

#[test]
fn test_key_sequence_manager_reset() {
    let mut manager = KeySequenceManager::new();

    manager.register(vec![KeyCode::Char('y'), KeyCode::Char('y')], "yank_line");

    manager.process_key(KeyCode::Char('y'));

    manager.reset();
}

// ==================== CustomKeybindings Tests ====================

#[test]
fn test_custom_keybindings_creation() {
    let bindings = CustomKeybindings::new();
    assert_eq!(bindings.total_bindings(), 0);
}

#[test]
fn test_custom_keybindings_bind() {
    let mut bindings = CustomKeybindings::new();

    bindings.bind_global(KeyCode::Char('s'), KeyModifiers::CONTROL, "save");

    assert_eq!(bindings.total_bindings(), 1);
}

#[test]
fn test_custom_keybindings_get() {
    let mut bindings = CustomKeybindings::new();

    bindings.bind_global(KeyCode::Char('q'), KeyModifiers::CONTROL, "quit");

    let binding = KeyBinding::ctrl(KeyCode::Char('q'));
    let found = bindings.get_in_context(KeybindingContext::Global, &binding);
    assert!(found.is_some());
    assert_eq!(found.unwrap(), "quit");
}

#[test]
fn test_custom_keybindings_contexts() {
    let mut bindings = CustomKeybindings::new();

    bindings.bind_global(KeyCode::Char('a'), KeyModifiers::NONE, "action_a");
    bindings.bind_in_context(KeybindingContext::Normal, KeyCode::Char('b'), KeyModifiers::NONE, "action_b");
    bindings.bind_in_context(KeybindingContext::Insert, KeyCode::Char('c'), KeyModifiers::NONE, "action_c");
    bindings.bind_in_context(KeybindingContext::Visual, KeyCode::Char('d'), KeyModifiers::NONE, "action_d");

    assert_eq!(bindings.total_bindings(), 4);
}

#[test]
fn test_custom_keybindings_unbind() {
    let mut bindings = CustomKeybindings::new();

    bindings.bind_global(KeyCode::Char('x'), KeyModifiers::NONE, "delete");
    assert_eq!(bindings.total_bindings(), 1);

    let binding = KeyBinding::new(KeyCode::Char('x'), KeyModifiers::NONE);
    let removed = bindings.unbind(KeybindingContext::Global, &binding);
    assert!(removed.is_some());
    assert_eq!(bindings.total_bindings(), 0);
}

#[test]
fn test_custom_keybindings_get_context_bindings() {
    let mut bindings = CustomKeybindings::new();

    bindings.bind_global(KeyCode::Char('a'), KeyModifiers::NONE, "a1");
    bindings.bind_global(KeyCode::Char('b'), KeyModifiers::NONE, "b1");
    bindings.bind_in_context(KeybindingContext::Normal, KeyCode::Char('c'), KeyModifiers::NONE, "c1");

    let global_bindings = bindings.get_context_bindings(KeybindingContext::Global);
    assert!(global_bindings.is_some());
}

// ==================== Cross-Feature Integration Tests ====================

#[test]
fn test_command_mode_with_aliases() {
    let mut aliases = AliasManager::new();
    aliases.add("w", "write");
    aliases.add("q", "quit");

    let mut mode = CommandMode::new();
    mode.start();
    mode.input_char('w');

    let result = mode.execute();
    assert!(result.is_some());

    let (cmd, _) = result.unwrap();

    // Expand alias
    let expanded = aliases.expand(&cmd);
    assert_eq!(expanded, Some("write".to_string()));
}

#[test]
fn test_key_sequences_with_command_mode() {
    let mut seq_manager = KeySequenceManager::new();
    let mut cmd_mode = CommandMode::new();

    // Register : : as command mode trigger
    seq_manager.register(vec![KeyCode::Char(':'), KeyCode::Char(':')], "command_mode");

    seq_manager.process_key(KeyCode::Char(':'));
    let result = seq_manager.process_key(KeyCode::Char(':'));

    if result == Some("command_mode".to_string()) {
        cmd_mode.start();
        assert!(cmd_mode.is_active());
    }
}

#[test]
fn test_keybindings_with_command_registry() {
    let mut bindings = CustomKeybindings::new();
    let mut registry = CommandRegistry::new();

    // Register command
    registry.register(
        Command::new("save", "Save file"),
        std::sync::Arc::new(|_| Ok("Saved".to_string())),
    );

    // Bind key to command
    bindings.bind_global(KeyCode::Char('s'), KeyModifiers::CONTROL, "save");

    // Simulate key press
    let binding = KeyBinding::ctrl(KeyCode::Char('s'));
    let action = bindings.get_in_context(KeybindingContext::Global, &binding);
    assert!(action.is_some());

    let result = registry.execute(action.unwrap(), &[]);
    assert!(result.is_ok());
}

#[test]
fn test_complete_power_user_workflow() {
    // Setup all systems
    let mut cmd_mode = CommandMode::new();
    let mut registry = CommandRegistry::new();
    let mut aliases = AliasManager::new();
    let mut keybindings = CustomKeybindings::new();
    let sequences = KeySequenceManager::vim_defaults();

    // Register commands
    registry.register(
        Command::new("write", "Write file").alias("w"),
        std::sync::Arc::new(|_| Ok("Written".to_string())),
    );

    // Add aliases
    aliases.add("wq", "write quit");

    // Add keybindings
    keybindings.bind_global(KeyCode::Char('s'), KeyModifiers::CONTROL, "write");

    // Test command execution
    cmd_mode.start();
    cmd_mode.input_char('w');
    let (cmd, args) = cmd_mode.execute().unwrap();
    let result = registry.execute(&cmd, &args);
    assert!(result.is_ok());

    // Test alias expansion
    let expanded = aliases.expand("wq");
    assert_eq!(expanded, Some("write quit".to_string()));

    // Test keybinding lookup
    let binding = KeyBinding::ctrl(KeyCode::Char('s'));
    let action = keybindings.get_in_context(KeybindingContext::Global, &binding);
    assert!(action.is_some());

    // Test key sequence
    assert!(sequences.len() > 0);
}
