# Rust Development Guidelines

Auto-Activates: When working with Rust code, implementing features, or asking about best practices

---

## Code Style

**Rust Conventions**:
- Follow Rust API Guidelines (https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` for consistent formatting
- Run `cargo clippy -- -D warnings` to catch common mistakes
- Module organization: group related functionality, use `mod.rs` for public API

**Naming Conventions**:
- Types: `PascalCase` (structs, enums, traits)
- Functions/variables: `snake_case`
- Constants: `SCREAMING_SNAKE_CASE`
- Lifetimes: `'a`, `'b` (short, descriptive)
- Type parameters: `T`, `E`, `K`, `V` (single letter for generic, descriptive for domain)

**Module Organization**:
```rust
// lib.rs or main.rs - Public API
pub mod evaluation;
pub mod agent;
pub mod tools;

mod internal_helpers; // Private module

// evaluation/mod.rs - Module public API
pub use self::task::Task;
pub use self::dataset::Dataset;

mod task;
mod dataset;
```

## Error Handling

**Pattern** (Rust):
```rust
use anyhow::{Context, Result};

// Application code - use anyhow::Result
pub fn load_config(path: &Path) -> Result<Config> {
    let contents = fs::read_to_string(path)
        .context("failed to read config file")?;

    let config: Config = serde_json::from_str(&contents)
        .context("failed to parse config JSON")?;

    Ok(config)
}

// Library code - use thiserror for custom errors
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AgentError {
    #[error("LLM API error: {0}")]
    LLMError(String),

    #[error("max steps exceeded: {0}")]
    MaxStepsExceeded(usize),

    #[error("tool execution failed: {tool}")]
    ToolError { tool: String, source: anyhow::Error },
}
```

**FORBIDDEN in src/**:
- `unwrap()` - Use `?` or `expect()` with justification
- `expect()` without context - Only in tests or with clear panic message
- Ignoring `Result` values

**Error Context Best Practices**:
```rust
// GOOD - Add context at each layer
file_content
    .parse::<i32>()
    .context("failed to parse user ID")?
    .validate()
    .context("invalid user ID range")?;

// BAD - No context
file_content.parse::<i32>()?;
```

## Testing

**Test Structure**:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_from_json_valid() {
        let json = r#"{"instance_id": "test-1", "repo": "owner/repo"}"#;
        let task = Task::from_json(json).unwrap();

        assert_eq!(task.instance_id, "test-1");
        assert_eq!(task.repo, "owner/repo");
    }

    #[test]
    fn test_task_from_json_invalid() {
        let json = r#"{"invalid": "schema"}"#;
        let result = Task::from_json(json);

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_agent_execute() {
        let client = MockLLMClient::new();
        let agent = Agent::new(client);

        let result = agent.execute_task(&Task::example()).await;
        assert!(result.is_ok());
    }
}
```

**Coverage Targets**:
| Layer | Target | What to Test |
|-------|--------|--------------|
| Models | 95%+ | Business logic, validation, edge cases |
| Services | 80%+ | Orchestration, error paths, state transitions |
| Tools | 80%+ | External interactions (mocked), error handling |
| Infrastructure | 60%+ | Integration points |
| UI | 40%+ | Interaction logic |

## Common Patterns

### Pattern 1: Result Propagation
```rust
// Use ? operator for clean error propagation
fn process_data() -> Result<ProcessedData> {
    let raw = load_raw_data()?;
    let validated = validate_data(raw)?;
    let transformed = transform_data(validated)?;
    Ok(transformed)
}
```

### Pattern 2: Builder Pattern
```rust
pub struct Config {
    host: String,
    port: u16,
    timeout: Duration,
}

impl Config {
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }
}

#[derive(Default)]
pub struct ConfigBuilder {
    host: Option<String>,
    port: Option<u16>,
    timeout: Option<Duration>,
}

impl ConfigBuilder {
    pub fn host(mut self, host: impl Into<String>) -> Self {
        self.host = Some(host.into());
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    pub fn build(self) -> Result<Config> {
        Ok(Config {
            host: self.host.ok_or_else(|| anyhow!("host required"))?,
            port: self.port.unwrap_or(8080),
            timeout: self.timeout.unwrap_or(Duration::from_secs(30)),
        })
    }
}

// Usage
let config = Config::builder()
    .host("localhost")
    .port(3000)
    .build()?;
```

### Pattern 3: Async with Tokio
```rust
// Spawn tasks with Send bounds
async fn concurrent_tasks() -> Result<()> {
    let task1 = tokio::spawn(async move {
        // Async work
    });

    let task2 = tokio::spawn(async move {
        // Async work
    });

    let (r1, r2) = tokio::try_join!(task1, task2)?;
    Ok(())
}

// Blocking operations in async context
async fn process_file(path: &Path) -> Result<String> {
    let path = path.to_owned();
    tokio::task::spawn_blocking(move || {
        std::fs::read_to_string(&path)
    })
    .await?
    .context("failed to read file")
}
```

### Pattern 4: Trait Bounds and Generics
```rust
// Clear, documented trait bounds
pub fn process_items<I, T>(items: I) -> Result<Vec<ProcessedItem>>
where
    I: IntoIterator<Item = T>,
    T: Processable + Send,
{
    items
        .into_iter()
        .map(|item| item.process())
        .collect()
}

// Associated types for cleaner APIs
pub trait Loader {
    type Item;
    type Error;

    fn load(&self, id: &str) -> Result<Self::Item, Self::Error>;
}
```

### Pattern 5: Error Context Wrapping
```rust
// Add context at appropriate layers
pub fn evaluate_task(task: &Task) -> Result<EvalResult> {
    let agent = create_agent()
        .context("failed to initialize agent")?;

    let result = agent.execute(task)
        .with_context(|| format!("failed to execute task {}", task.id))?;

    result.validate()
        .context("validation failed after execution")?;

    Ok(result)
}
```

## Quality Gates

Before marking work complete:
- [ ] All tests pass (`cargo test`)
- [ ] Code formatted (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy -- -D warnings`)
- [ ] No unwrap() in src/ (except with justification)
- [ ] Rustdoc complete for public items
- [ ] Error handling uses ? or proper context
- [ ] Async functions have Send bounds documented
- [ ] Coverage meets layer target

## Async Rust Guidelines

**Send + Sync Bounds**:
```rust
// Document Send requirement
/// Executes task asynchronously.
///
/// # Requirements
/// Requires tokio runtime. Future is Send + 'static.
pub async fn execute<F>(future: F) -> Result<()>
where
    F: Future<Output = Result<()>> + Send + 'static,
{
    tokio::spawn(future).await?
}
```

**Blocking Operations**:
```rust
// WRONG - Blocking in async context
async fn load_file() -> Result<String> {
    Ok(std::fs::read_to_string("file.txt")?) // Blocks executor
}

// RIGHT - Use spawn_blocking
async fn load_file() -> Result<String> {
    tokio::task::spawn_blocking(|| {
        std::fs::read_to_string("file.txt")
    })
    .await?
    .context("failed to read file")
}
```

**Select and Cancellation**:
```rust
// Cancel-safe patterns
tokio::select! {
    result = async_op1() => {
        // Handle result
    }
    _ = cancellation_token.cancelled() => {
        // Clean cancellation
        return Ok(());
    }
}
```

## Performance Tips

**Avoid Unnecessary Clones**:
```rust
// BAD
fn process_items(items: Vec<String>) {
    for item in items {
        let copied = item.clone(); // Unnecessary
        handle(&copied);
    }
}

// GOOD
fn process_items(items: &[String]) {
    for item in items {
        handle(item); // Borrow
    }
}
```

**Use Appropriate Collections**:
- `Vec<T>` - Sequential access, frequent iteration
- `HashMap<K, V>` - Fast lookups, unordered
- `BTreeMap<K, V>` - Ordered keys, range queries
- `HashSet<T>` - Unique items, membership tests

**String Handling**:
```rust
// BAD - Multiple allocations
let mut result = String::new();
for item in items {
    result = result + &item.to_string(); // Allocates each time
}

// GOOD - Pre-allocate or use iterator
let result: String = items
    .iter()
    .map(|i| i.to_string())
    .collect();
```

## Documentation (Rustdoc)

**Module-Level Docs**:
```rust
//! Agent execution engine for SWE-bench tasks.
//!
//! # Architecture
//! Implements a 25-step execution loop with tool use.
//!
//! # Examples
//! ```
//! use toad::agent::Agent;
//!
//! let agent = Agent::new(llm_client, tools);
//! let result = agent.execute_task(&task).await?;
//! ```
```

**Item-Level Docs**:
```rust
/// Executes a task with the agent loop.
///
/// # Arguments
/// * `task` - The SWE-bench task to solve
///
/// # Returns
/// `AgentResult` containing metrics and solution
///
/// # Errors
/// Returns error if:
/// - LLM API fails after retries
/// - Tool execution fails critically
/// - Max steps (25) exceeded
///
/// # Panics
/// Does not panic. All errors returned as `Result`.
///
/// # Examples
/// ```
/// let agent = Agent::new(client);
/// let result = agent.execute_task(&task).await?;
/// assert!(result.solved);
/// ```
pub async fn execute_task(&self, task: &Task) -> Result<AgentResult> {
    // Implementation
}
```

## Safety and Unsafe

**When Unsafe is Needed**:
```rust
/// Converts bytes to string without UTF-8 validation.
///
/// # Safety
/// Caller must ensure `bytes` contains valid UTF-8.
/// Violating this will cause undefined behavior.
pub unsafe fn from_utf8_unchecked(bytes: &[u8]) -> &str {
    // SAFETY: Caller guarantees valid UTF-8 per function contract
    std::str::from_utf8_unchecked(bytes)
}
```

**Prefer Safe Alternatives**:
- Use `std::str::from_utf8()` instead of unchecked version
- Use `Vec::get()` instead of unchecked indexing
- Use `Arc` instead of raw pointers for shared ownership

## References

- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Effective Rust](https://www.lurklurk.org/effective-rust/)
- [Rust Async Book](https://rust-lang.github.io/async-book/)
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Tokio Documentation](https://tokio.rs/)
- [Anyhow Documentation](https://docs.rs/anyhow/)
- [Thiserror Documentation](https://docs.rs/thiserror/)
