# Testing Checklist for Toad TUI

## ✅ Automated Tests (All Passing)

- [x] App initialization test
- [x] ESC quit from welcome screen
- [x] Ctrl+C quit from main screen
- [x] Input field character insertion
- [x] Backspace functionality
- [x] Cargo clippy (zero warnings)
- [x] Release build

## 🧪 Manual Testing Checklist

### Startup Sequence
- [ ] Welcome screen displays correctly
  - [ ] TOAD logo renders properly (not "TOAT")
  - [ ] Tips panel shows on right side
  - [ ] Toad green color theme is applied
  - [ ] Press any key advances to trust dialog

### Trust Dialog
- [ ] Dialog displays centered
  - [ ] Current directory path shows in info box
  - [ ] Three options display with numbers
  - [ ] Arrow keys (↑↓) change selection
  - [ ] Number keys (1-3) select directly
  - [ ] Selected option has green highlight and ❯ prefix
  - [ ] Enter confirms selection
  - [ ] ESC quits application
  - [ ] Option 1: Advances to main screen
  - [ ] Option 2: Advances to main screen (TODO: save to config)
  - [ ] Option 3: Quits application

### Main Interface
- [ ] System info header displays
  - [ ] "Sonnet 4.5 · Rust TUI" shows
  - [ ] "Active Plugins: 0 installed" shows
  - [ ] Current project path displays
- [ ] Main content area shows welcome message
- [ ] Horizontal separator line (────) displays
- [ ] Input prompt shows at bottom
- [ ] Keyboard shortcuts bar displays
  - [ ] "Ctrl+C quit | ? help | / commands | Ctrl+P palette | Tab autocomplete"

### Input Field
- [ ] Placeholder text shows when empty
  - [ ] "> Ask me anything or type a command..." in gray italic
- [ ] Typing characters works
  - [ ] Each character appears at cursor position
  - [ ] Cursor (green background) visible
- [ ] Cursor movement
  - [ ] Left arrow moves cursor left
  - [ ] Right arrow moves cursor right
  - [ ] Home key jumps to start
  - [ ] End key jumps to end
  - [ ] Ctrl+A jumps to start (Emacs-style)
  - [ ] Ctrl+E jumps to end (Emacs-style)
- [ ] Editing
  - [ ] Backspace deletes character before cursor
  - [ ] Ctrl+U clears entire input
- [ ] Enter submits command
  - [ ] Status message shows "Submitted: <input>"
  - [ ] Input field clears after submit
- [ ] Special characters work
  - [ ] Spaces
  - [ ] Punctuation
  - [ ] UTF-8 characters (emoji, etc.)

### Graceful Shutdown
- [ ] Ctrl+C from main screen quits cleanly
- [ ] Ctrl+D from main screen quits cleanly
- [ ] ESC from welcome quits cleanly
- [ ] ESC from trust dialog quits cleanly
- [ ] Terminal state restored after quit
  - [ ] Raw mode disabled
  - [ ] Alternate screen exited
  - [ ] Mouse capture disabled
  - [ ] Cursor visible
  - [ ] No corruption or artifacts

### Error Handling
- [ ] Panic hook restores terminal
  - [ ] Test by triggering panic (manually)
  - [ ] Terminal should still be usable after panic
- [ ] Drop trait cleanup
  - [ ] Terminal restored even if not gracefully quit

### Visual Polish
- [ ] Toad green accent color used consistently
  - [ ] Borders
  - [ ] Logo
  - [ ] Input prompt
  - [ ] Keyboard shortcuts
- [ ] Grayscale palette
  - [ ] White for titles
  - [ ] Light gray for content
  - [ ] Dark gray for separators
  - [ ] Black background
- [ ] Box-drawing characters render correctly
  - [ ] ╭╮╰╯│ for dialogs
  - [ ] ─ for separators
- [ ] No visual glitches or flickering
- [ ] Proper spacing and alignment

### Logging
- [ ] toad.log file created
- [ ] Log entries written
  - [ ] "Starting Toad TUI"
  - [ ] "TUI initialized, entering main loop"
  - [ ] "Exiting main loop"
  - [ ] "Application shutdown successfully"

### Resize Handling
- [ ] Terminal resize detected
- [ ] UI reflows properly
- [ ] No crashes on small terminal
- [ ] No crashes on large terminal

### Performance
- [ ] No input lag
- [ ] Smooth cursor movement
- [ ] Fast rendering (no flicker)
- [ ] Low CPU usage when idle

## 🐛 Known Issues / TODOs

- [ ] Trust folder "remember" option doesn't persist (TODO in code)
- [ ] "?" help command not implemented
- [ ] "/" commands not implemented
- [ ] "Ctrl+P" palette not implemented
- [ ] "Tab" autocomplete not implemented
- [ ] No actual AI integration yet
- [ ] No plugin system yet

## 🎯 Rust Best Practices Verified

- [x] Zero clippy warnings with `-D warnings`
- [x] No `unwrap()` outside of tests
- [x] Proper error handling with `Result` types
- [x] Drop trait for resource cleanup
- [x] Panic hook for safety
- [x] No unused code
- [x] Arrays instead of vec! where possible
- [x] inspect() instead of map() where appropriate
- [x] No useless format!() calls
- [x] Proper visibility (pub/private)
- [x] Documentation comments
- [x] Module organization

## 📊 Code Quality Metrics

- **Lines of Code**: ~1500
- **Test Coverage**: 5 tests, all passing
- **Clippy Warnings**: 0
- **Build Warnings**: 0
- **Dependencies**: 11 (6 unused but planned)
- **Release Binary Size**: ~4.5 MB (stripped)
- **Compile Time (release)**: ~30s

## ✅ Ready for Next Features

The foundation is solid! Ready to implement:
1. Command processing (/help, /commands)
2. Command palette (Ctrl+P)
3. Help screen (?)
4. Chat history panel
5. AI integration
6. Plugin system
