# Anthropic Integration Analysis

**Date:** 2025-11-08
**Status:** Current implementation is functional but missing critical features

---

## Executive Summary

The current TOAD Anthropic integration is a **custom implementation using reqwest** (not using any community SDKs). It provides basic functionality with good error handling but **lacks streaming support** and several important API parameters.

### Key Findings:

✅ **Working:**
- Basic message sending
- Tool use (function calling)
- Prompt caching support
- Cost calculation
- Error handling (retryable vs permanent)
- API key validation

❌ **Missing Critical Features:**
- **Message streaming** (Server-Sent Events)
- System prompts (using messages instead of top-level `system` parameter)
- Temperature, top_p, top_k controls
- Stop sequences
- Tool choice control
- Extended thinking
- Metadata tracking
- Service tier selection
- Beta features support

⚠️ **Issues:**
- Using potentially outdated model name
- API version is from 2023-06-01
- Could better parse Anthropic-specific error responses

---

## Current Implementation Review

### Architecture

**Location:** `src/ai/llm/`
- `mod.rs` - Types and trait definitions
- `anthropic.rs` - AnthropicClient implementation
- `errors.rs` - LLMError types
- `rate_limiter.rs` - Rate limiting

**HTTP Client:** `reqwest` v0.12
**Async Runtime:** Tokio
**SDK Used:** None (custom implementation)

### What's Implemented

#### ✅ Core Features
```rust
// Good practices observed:
- Async/await with tokio
- Proper error types with thiserror
- Usage tracking (input/output/cache tokens)
- Cost calculation ($3/MTok input, $15/MTok output)
- Tool use support with JSON schemas
- Prompt caching (cache_creation_tokens, cache_read_tokens)
- Rate limiting (separate module)
```

#### ✅ Error Handling
```rust
pub enum LLMError {
    ApiKey(String),
    ApiError { status: u16, message: String },
    RateLimit { retry_after: Option<u64> },
    Timeout { seconds: u64 },
    Network(reqwest::Error),
    ParseError(String),
    Configuration(String),
    TokenLimit { used: u32, max: u32 },
    ModelNotFound(String),
    Other(anyhow::Error),
}
```

**Strengths:**
- Distinguishes retryable vs permanent errors
- Provides retry delay suggestions
- Good test coverage (37 tests)

### What's Missing

#### ❌ Missing API Parameters

From [Claude API docs](https://docs.claude.com/en/api/messages):

| Parameter | Status | Priority | Notes |
|-----------|--------|----------|-------|
| `stream` | ❌ Missing | **CRITICAL** | Enables real-time streaming |
| `system` | ❌ Missing | **HIGH** | Should be top-level, not in messages |
| `temperature` | ❌ Missing | HIGH | Control randomness (0-1) |
| `top_p` | ❌ Missing | MEDIUM | Nucleus sampling |
| `top_k` | ❌ Missing | MEDIUM | Top-K sampling |
| `stop_sequences` | ❌ Missing | MEDIUM | Custom stop conditions |
| `tool_choice` | ❌ Missing | HIGH | Control tool use (auto/any/tool/none) |
| `thinking` | ❌ Missing | LOW | Extended thinking config |
| `metadata` | ❌ Missing | LOW | Request context tracking |
| `service_tier` | ❌ Missing | LOW | Priority routing |
| `anthropic-beta` header | ❌ Missing | MEDIUM | Beta features |

#### ❌ Streaming Support

**Current state:** Not implemented
**Impact:** Cannot provide real-time responses in TUI

**From docs:**
- Set `"stream": true` in request
- Receive Server-Sent Events (SSE)
- Event types: `message_start`, `content_block_delta`, `message_delta`, `message_stop`, `ping`, `error`
- Requires SSE parsing library (e.g., `eventsource-stream` or `reqwest-eventsource`)

**Benefits of streaming:**
- Real-time user feedback
- Better UX for long responses
- Can display partial results
- Can cancel in-flight requests

---

## Comparison: Custom vs SDK

### Available Rust SDKs

Research found **no official Anthropic Rust SDK**. Community options:

| Crate | Version | Stars | Streaming | Maintained | Notes |
|-------|---------|-------|-----------|------------|-------|
| `anthropic-sdk-rust` | Latest | N/A | ✅ Yes | ✅ Active | Full feature parity with TS SDK |
| `anthropic_rust` | Latest | N/A | ✅ Yes | ✅ Active | Modern, type-safe, async-first |
| `async-anthropic` | Latest | N/A | ✅ Yes | ✅ Active | Robust error handling |
| `anthropic-api` | Latest | N/A | ✅ Yes | ⚠️ Unknown | Streaming + tools |
| `anthropic-rs` (AbdelStark) | 0.x | 66 | ⚠️ Partial | ⚠️ Low activity | 38 commits, inspired by async-openai |

### Recommendation: Continue Custom Implementation

**Why:**
1. **No official SDK** - All are community-maintained
2. **Current code is high quality** - Good error handling, tests, documentation
3. **Flexibility** - Can implement exactly what TOAD needs
4. **No extra dependencies** - Already using reqwest
5. **Learning** - Team understands the code

**But add:**
- Streaming support (critical)
- Missing API parameters
- Better error parsing
- Update to latest API version

---

## Implementation Gaps Analysis

### 1. Model Name

**Current:**
```rust
const DEFAULT_MODEL: &str = "claude-sonnet-4-20250514";
```

**Latest from docs:**
- `claude-sonnet-4-5-20250929` (Sonnet 4.5 - latest)
- `claude-opus-4-20250514` (Opus 4)
- `claude-haiku-4-20250514` (Haiku 4)

**Action:** Update to Sonnet 4.5

### 2. API Version

**Current:**
```rust
.header("anthropic-version", "2023-06-01")
```

**Latest:** Check docs for current version

**Action:** Research and update to latest stable API version

### 3. System Prompt Handling

**Current:** System prompt likely sent as message
**Correct:** Should be top-level parameter

```rust
// Current (incorrect)
let body = json!({
    "model": self.model,
    "max_tokens": self.max_tokens,
    "messages": messages, // includes system as first message?
});

// Correct
let body = json!({
    "model": self.model,
    "max_tokens": self.max_tokens,
    "system": system_prompt, // Top-level
    "messages": messages, // User/assistant only
});
```

### 4. Error Response Parsing

**Current:** Generic error string
**Better:** Parse Anthropic error structure

```rust
// Anthropic error response format:
{
    "type": "error",
    "error": {
        "type": "invalid_request_error",
        "message": "..."
    }
}
```

**Error types from docs:**
- `invalid_request_error` - 400
- `authentication_error` - 401
- `permission_error` - 403
- `not_found_error` - 404
- `rate_limit_error` - 429
- `api_error` - 500
- `overloaded_error` - 529

---

## Recommendations

### Phase 1: Critical Fixes (Immediate)

1. **Update model name** to `claude-sonnet-4-5-20250929`
2. **Add system parameter** to request builder
3. **Add temperature, top_p, top_k** parameters
4. **Add tool_choice** parameter
5. **Add stop_sequences** support
6. **Improve error parsing** with Anthropic error types

**Estimated effort:** 2-4 hours
**Risk:** Low
**Tests needed:** Update existing tests, add 10-15 new tests

### Phase 2: Streaming Support (High Priority)

1. **Add streaming method** to AnthropicClient
2. **Implement SSE parsing** using `eventsource-stream` or similar
3. **Add stream event types:**
   - `MessageStart`
   - `ContentBlockStart`
   - `ContentBlockDelta`
   - `ContentBlockStop`
   - `MessageDelta`
   - `MessageStop`
   - `Ping`
   - `Error`
4. **Update LLMClient trait** with streaming method
5. **Add TUI integration** for real-time display

**Estimated effort:** 8-12 hours
**Risk:** Medium
**Dependencies:** `eventsource-stream` or `reqwest-eventsource`
**Tests needed:** 15-20 new tests

### Phase 3: Nice-to-Have (Low Priority)

1. Add `metadata` parameter for tracking
2. Add `service_tier` for priority routing
3. Add `thinking` parameter for extended thinking
4. Add `anthropic-beta` header support
5. Update API version to latest

**Estimated effort:** 2-4 hours
**Risk:** Low

---

## Testing Without API Key

**Current behavior:** ✅ Correct

```bash
$ cargo run -- eval --count 1 --milestone 1
Error: Failed to get API key. Set ANTHROPIC_API_KEY environment variable

Caused by:
    0: ANTHROPIC_API_KEY environment variable not set
    1: environment variable not found
```

**Validation:**
- ✅ Clear error message
- ✅ Fails fast before making API calls
- ✅ Provides actionable guidance
- ✅ Proper error propagation

---

## Proposed Implementation Plan

### Step 1: Create Feature Branch
```bash
git checkout -b feature/anthropic-improvements
```

### Step 2: Update Dependencies (if needed)
```toml
[dependencies]
# For streaming support
eventsource-stream = "0.2"  # Or reqwest-eventsource
```

### Step 3: Enhance AnthropicClient

**File:** `src/ai/llm/anthropic.rs`

```rust
pub struct AnthropicClient {
    api_key: String,
    model: String,
    max_tokens: u32,
    system: Option<String>,
    temperature: Option<f32>,
    top_p: Option<f32>,
    top_k: Option<u32>,
    stop_sequences: Option<Vec<String>>,
    http_client: reqwest::Client,
}

impl AnthropicClient {
    // Add builder methods
    pub fn with_system(mut self, system: impl Into<String>) -> Self { ... }
    pub fn with_temperature(mut self, temperature: f32) -> Self { ... }
    pub fn with_top_p(mut self, top_p: f32) -> Self { ... }
    pub fn with_stop_sequences(mut self, sequences: Vec<String>) -> Self { ... }
}
```

### Step 4: Add Streaming

**New file:** `src/ai/llm/streaming.rs`

```rust
pub enum StreamEvent {
    MessageStart { message: MessageStart },
    ContentBlockStart { index: usize, content_block: ContentBlock },
    ContentBlockDelta { index: usize, delta: Delta },
    ContentBlockStop { index: usize },
    MessageDelta { delta: MessageDelta },
    MessageStop,
    Ping,
    Error { error: ApiError },
}

pub struct MessageStream {
    // SSE stream handling
}

impl Stream for MessageStream {
    type Item = Result<StreamEvent>;
    // ...
}
```

### Step 5: Update LLMClient Trait

```rust
#[async_trait::async_trait]
pub trait LLMClient: Send + Sync {
    async fn send_message(...) -> Result<LLMResponse>;

    // New method
    async fn send_message_stream(
        &self,
        messages: Vec<Message>,
        tools: Option<Vec<serde_json::Value>>,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<StreamEvent>> + Send>>>;
}
```

### Step 6: Comprehensive Testing

```rust
#[cfg(test)]
mod tests {
    // Test Phase 1
    #[test] fn test_system_parameter() { ... }
    #[test] fn test_temperature_parameter() { ... }
    #[test] fn test_tool_choice_parameter() { ... }
    #[test] fn test_stop_sequences() { ... }
    #[test] fn test_updated_model_name() { ... }
    #[test] fn test_anthropic_error_parsing() { ... }

    // Test Phase 2
    #[tokio::test] async fn test_streaming_basic() { ... }
    #[tokio::test] async fn test_streaming_events() { ... }
    #[tokio::test] async fn test_streaming_error_recovery() { ... }
    #[tokio::test] async fn test_streaming_cancellation() { ... }
}
```

---

## Success Criteria

### Phase 1 (Critical Fixes)
- [ ] Model updated to `claude-sonnet-4-5-20250929`
- [ ] System prompt is top-level parameter
- [ ] Temperature, top_p, top_k supported
- [ ] Tool choice control implemented
- [ ] Stop sequences working
- [ ] Anthropic error types parsed correctly
- [ ] All existing tests pass
- [ ] 10+ new tests added
- [ ] Cost calculations still accurate

### Phase 2 (Streaming)
- [ ] Streaming method implemented
- [ ] SSE events parsed correctly
- [ ] All event types handled
- [ ] Error recovery works
- [ ] Stream cancellation works
- [ ] TUI can display streaming responses
- [ ] 15+ streaming tests added
- [ ] Memory usage acceptable during streaming

### Phase 3 (Nice-to-Have)
- [ ] Metadata parameter supported
- [ ] Service tier supported
- [ ] Extended thinking supported
- [ ] Beta features accessible
- [ ] API version updated

---

## Risk Assessment

| Risk | Severity | Mitigation |
|------|----------|------------|
| Breaking existing functionality | HIGH | Comprehensive test suite, gradual rollout |
| Streaming implementation complexity | MEDIUM | Use proven library (eventsource-stream), extensive testing |
| API version incompatibility | LOW | Check docs, test with real API |
| Performance degradation | LOW | Benchmark before/after, profile if needed |
| Increased dependencies | LOW | Evaluate bundle size, consider alternatives |

---

## Conclusion

**Current state:** Functional but limited
**Recommendation:** Enhance custom implementation rather than switch to SDK
**Priority:** Phase 1 (immediate) → Phase 2 (2-3 weeks) → Phase 3 (optional)

**Key advantages of improvements:**
1. **Streaming** enables real-time TUI experience
2. **System prompts** improve accuracy and consistency
3. **Temperature/sampling** provides control over creativity
4. **Tool choice** enables better agent behavior
5. **Better errors** improve debugging

**Timeline:**
- Phase 1: 1 day
- Phase 2: 2-3 days
- Phase 3: 1 day
- **Total:** ~1 week for complete implementation

---

## References

- [Claude Messages API](https://docs.claude.com/en/api/messages)
- [Claude Streaming API](https://docs.claude.com/en/api/messages-streaming)
- [Claude Models](https://docs.claude.com/en/docs/models-overview)
- [Anthropic Error Types](https://docs.claude.com/en/api/errors)
- Current implementation: `src/ai/llm/anthropic.rs`
