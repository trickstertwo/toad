# Toad Architecture

> World-class AI Coding Terminal - Architecture & Quality Gates

## Philosophy

Toad follows a **pragmatic layered architecture** optimized for TUI applications, combining:
- **Domain-Driven Design** for core business logic
- **Feature-based modules** for isolated functionality
- **Elm-style architecture** for UI state management (Model-Update-View)
- **Quality gates** at every level

## Folder Structure

```
toad/
├── src/
│   ├── core/              # Core application infrastructure
│   │   ├── app.rs         # Main application state & orchestration
│   │   ├── event.rs       # Event handling & loop
│   │   └── router.rs      # Screen routing logic (coming soon)
│   │
│   ├── domain/            # Business domain models
│   │   ├── workspace.rs   # Workspace/project management (coming soon)
│   │   ├── session.rs     # AI session state (coming soon)
│   │   └── context.rs     # Code context tracking (coming soon)
│   │
│   ├── features/          # Feature modules with quality gates
│   │   ├── chat/          # AI chat interface (coming soon)
│   │   │   ├── mod.rs
│   │   │   ├── state.rs   # Feature state
│   │   │   ├── ui.rs      # UI rendering
│   │   │   └── tests.rs   # Feature tests
│   │   │
│   │   ├── editor/        # Code editor (coming soon)
│   │   ├── search/        # Search & filtering (coming soon)
│   │   └── git/           # Git integration (coming soon)
│   │
│   ├── infrastructure/    # External systems & I/O
│   │   ├── tui/           # Terminal UI (ratatui/crossterm)
│   │   │   ├── mod.rs
│   │   │   ├── terminal.rs
│   │   │   └── event_loop.rs
│   │   │
│   │   ├── config/        # Configuration management
│   │   │   ├── mod.rs
│   │   │   └── loader.rs  # Config file loading
│   │   │
│   │   └── storage/       # Persistence layer
│   │       ├── history.rs # Command history
│   │       └── cache.rs   # Session cache (coming soon)
│   │
│   ├── ui/                # UI components & widgets
│   │   ├── widgets/       # Reusable widgets
│   │   │   ├── mod.rs
│   │   │   ├── dialog.rs
│   │   │   ├── input.rs
│   │   │   ├── toast.rs
│   │   │   ├── palette.rs
│   │   │   ├── filetree.rs
│   │   │   └── ...
│   │   │
│   │   ├── layouts/       # Layout compositions (coming soon)
│   │   │   ├── split.rs   # Split panes
│   │   │   └── tabs.rs    # Tab system
│   │   │
│   │   └── theme.rs       # Theme system
│   │
│   ├── utils/             # Cross-cutting utilities
│   │   ├── clipboard.rs   # Clipboard integration
│   │   ├── keybinds.rs    # Keyboard shortcuts
│   │   └── logger.rs      # Logging utilities (coming soon)
│   │
│   ├── lib.rs             # Public API
│   └── main.rs            # Entry point
│
├── tests/                 # Integration tests
│   ├── ui_tests.rs
│   └── e2e_tests.rs       # End-to-end tests (coming soon)
│
├── benches/               # Performance benchmarks (coming soon)
│   └── render_bench.rs
│
├── docs/                  # Documentation (coming soon)
│   ├── architecture.md
│   ├── contributing.md
│   └── quality_gates.md
│
├── Cargo.toml
├── ROADMAP.md
├── TESTING_CHECKLIST.md
└── README.md
```

## Architecture Layers

### 1. Core Layer (`src/core/`)
**Purpose**: Application orchestration, event loop, state management

**Responsibilities**:
- Application lifecycle management
- Event routing and handling
- Screen/view management
- Global state coordination

**Quality Gates**:
- ✅ Unit tests for state transitions
- ✅ Zero unwrap() calls
- ✅ Comprehensive error handling
- ✅ Full rustdoc coverage

### 2. Domain Layer (`src/domain/`)
**Purpose**: Pure business logic, independent of UI or I/O

**Responsibilities**:
- Workspace and project models
- AI session management
- Code context tracking
- Business rules and validations

**Quality Gates**:
- ✅ 100% unit test coverage
- ✅ Property-based testing (proptest)
- ✅ No external dependencies (pure Rust)
- ✅ Full documentation with examples

### 3. Features Layer (`src/features/`)
**Purpose**: Self-contained feature modules with their own state and UI

**Responsibilities**:
- Feature-specific state management
- Feature UI rendering
- Feature-specific business logic
- Integration with domain layer

**Quality Gates**:
- ✅ Integration tests per feature
- ✅ UI snapshot tests (coming soon)
- ✅ Performance benchmarks
- ✅ Feature flags for gradual rollout

**Example Feature Structure**:
```rust
// src/features/search/mod.rs
pub struct SearchFeature {
    state: SearchState,
    config: SearchConfig,
}

impl SearchFeature {
    pub fn new() -> Self { ... }
    pub fn update(&mut self, msg: SearchMsg) -> Result<()> { ... }
    pub fn view(&self, frame: &mut Frame, area: Rect) { ... }
}

#[cfg(test)]
mod tests {
    // Comprehensive feature tests
}
```

### 4. Infrastructure Layer (`src/infrastructure/`)
**Purpose**: External integrations, I/O, persistence

**Responsibilities**:
- Terminal UI (ratatui/crossterm)
- Configuration file management
- History persistence
- AI API clients (coming soon)
- File system operations

**Quality Gates**:
- ✅ Integration tests with mocks
- ✅ Error handling for all I/O
- ✅ Graceful degradation
- ✅ Cross-platform testing

### 5. UI Layer (`src/ui/`)
**Purpose**: Reusable UI components, widgets, themes

**Responsibilities**:
- Widget implementations
- Layout compositions
- Theme management
- Styling utilities

**Quality Gates**:
- ✅ Visual regression tests (coming soon)
- ✅ Accessibility checks
- ✅ Performance profiling
- ✅ Component documentation

### 6. Utils Layer (`src/utils/`)
**Purpose**: Cross-cutting concerns and utilities

**Responsibilities**:
- Clipboard integration
- Keyboard shortcuts
- Logging
- Common utilities

**Quality Gates**:
- ✅ Unit tests
- ✅ Platform-specific testing
- ✅ Documentation
- ✅ No side effects

## Quality Gates Framework

Every feature MUST pass these quality gates before merging:

### 1. Code Quality
```bash
# Zero clippy warnings (strict mode)
cargo clippy --all-targets --all-features -- -D warnings

# Format check
cargo fmt --check

# No unsafe code (unless explicitly documented)
cargo geiger
```

### 2. Testing
```bash
# Unit tests (minimum 80% coverage)
cargo test

# Integration tests
cargo test --test '*'

# Doc tests
cargo test --doc

# Coverage report
cargo tarpaulin --out Html
```

### 3. Documentation
```toml
# All public items documented
#![warn(missing_docs)]

# Examples in documentation
#![warn(rustdoc::missing_doc_code_examples)]
```

### 4. Performance
```bash
# Benchmarks (no regression)
cargo bench

# Binary size check (< 10MB stripped)
cargo build --release
strip target/release/toad
ls -lh target/release/toad

# Startup time (< 100ms)
hyperfine './target/release/toad --version'
```

### 5. Dependencies
```bash
# Check for outdated deps
cargo outdated

# Security audit
cargo audit

# Unused dependencies
cargo udeps
```

### 6. Cross-platform
```yaml
# CI matrix testing
platforms:
  - Linux (x86_64, aarch64)
  - macOS (Intel, Apple Silicon)
  - Windows (x86_64)
```

## Design Principles

### 1. Elm Architecture for UI
```rust
// Model: Application state
struct Model {
    screen: Screen,
    input: String,
    history: Vec<String>,
}

// Message: Events/actions
enum Msg {
    KeyPress(KeyEvent),
    Submit,
    Quit,
}

// Update: State transitions
fn update(model: &mut Model, msg: Msg) -> Result<()> {
    match msg {
        Msg::Submit => {
            model.history.push(model.input.clone());
            model.input.clear();
        }
        // ...
    }
}

// View: Rendering
fn view(model: &Model, frame: &mut Frame) {
    // Render UI
}
```

### 2. Error Handling Strategy
```rust
// Use thiserror for custom errors
#[derive(Debug, Error)]
pub enum ToadError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Configuration error: {0}")]
    Config(String),
}

// Use anyhow for application errors
pub type Result<T> = anyhow::Result<T>;

// Never panic in production code
// Use Result and proper error propagation
```

### 3. Dependency Injection
```rust
// Use traits for testability
pub trait AiClient {
    fn send_message(&self, msg: &str) -> Result<String>;
}

// Concrete implementation
pub struct ClaudeClient { ... }

impl AiClient for ClaudeClient {
    fn send_message(&self, msg: &str) -> Result<String> { ... }
}

// Mock for testing
#[cfg(test)]
pub struct MockAiClient { ... }
```

### 4. Performance-First
- **Lazy rendering**: Only render visible elements
- **Virtual scrolling**: Handle 1M+ items
- **Async I/O**: Non-blocking operations with tokio
- **Incremental compilation**: Fast development cycles
- **Binary size**: Optimized release builds (LTO, codegen-units=1)

### 5. Documentation-Driven Development
- Write documentation first
- Include examples in docs
- Keep docs up-to-date with code
- Document "why" not just "what"

## Migration Strategy

### Phase 1: Foundation (Current)
✅ Basic structure in place
✅ Core widgets implemented
✅ Quality gates established

### Phase 2: Reorganization (This PR)
- Create new folder structure
- Move existing code to new locations
- Update imports and tests
- Maintain backward compatibility

### Phase 3: Feature Implementation
- Implement features with quality gates
- One feature at a time
- Full test coverage per feature
- Documentation per feature

### Phase 4: Polish & Optimization
- Performance optimization
- Visual polish
- Advanced features
- Plugin system

## Contributing

All contributions must:
1. Follow this architecture
2. Pass all quality gates
3. Include tests and documentation
4. Be reviewed by maintainers

## References

- [Elm Architecture](https://guide.elm-lang.org/architecture/)
- [Domain-Driven Design](https://www.domainlanguage.com/ddd/)
- [Ratatui Best Practices](https://ratatui.rs/best-practices/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
