/// Streaming support for Anthropic Messages API
///
/// Implements Server-Sent Events (SSE) parsing for real-time responses
/// Reference: https://docs.claude.com/en/api/messages-streaming
use super::{StopReason, ToolUse, Usage};
use anyhow::{Context, Result};
use eventsource_stream::Eventsource;
use futures::Stream;
use pin_project::pin_project;
use serde::Deserialize;
use std::pin::Pin;
use std::task::{Context as TaskContext, Poll};

/// Stream event from Claude API
#[derive(Debug, Clone)]
pub enum StreamEvent {
    /// Message started with initial metadata
    MessageStart { message: MessageStart },

    /// Content block started (text, tool_use, thinking)
    ContentBlockStart {
        index: usize,
        content_block: ContentBlockStart,
    },

    /// Content block delta (incremental content)
    ContentBlockDelta { index: usize, delta: ContentDelta },

    /// Content block completed
    ContentBlockStop { index: usize },

    /// Message metadata update (token usage)
    MessageDelta {
        delta: MessageDeltaEvent,
        usage: Usage,
    },

    /// Message completed successfully
    MessageStop,

    /// Ping event (keepalive)
    Ping,

    /// Error occurred during streaming
    Error { error: ApiError },
}

/// Initial message metadata
#[derive(Debug, Clone, Deserialize)]
pub struct MessageStart {
    pub id: String,
    pub model: String,
    #[serde(rename = "type")]
    pub message_type: String,
    pub role: String,
    pub usage: StreamUsage,
}

/// Content block start information
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentBlockStart {
    Text { text: String },
    ToolUse { id: String, name: String },
    Thinking { thinking: String },
}

/// Content delta (incremental content)
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentDelta {
    TextDelta { text: String },
    InputJsonDelta { partial_json: String },
}

/// Message delta information
#[derive(Debug, Clone, Deserialize)]
pub struct MessageDeltaEvent {
    pub stop_reason: Option<StopReason>,
    pub stop_sequence: Option<String>,
}

/// Streaming usage information
#[derive(Debug, Clone, Deserialize)]
pub struct StreamUsage {
    pub input_tokens: u32,
    pub output_tokens: u32,
    #[serde(default)]
    pub cache_creation_input_tokens: Option<u32>,
    #[serde(default)]
    pub cache_read_input_tokens: Option<u32>,
}

impl From<StreamUsage> for Usage {
    fn from(usage: StreamUsage) -> Self {
        Usage {
            input_tokens: usage.input_tokens,
            output_tokens: usage.output_tokens,
            cache_creation_tokens: usage.cache_creation_input_tokens,
            cache_read_tokens: usage.cache_read_input_tokens,
        }
    }
}

/// API error during streaming
#[derive(Debug, Clone, Deserialize)]
pub struct ApiError {
    #[serde(rename = "type")]
    pub error_type: String,
    pub message: String,
}

/// Internal SSE event structure
#[derive(Debug, Deserialize)]
struct SseEvent {
    #[serde(rename = "type")]
    event_type: String,

    // MessageStart fields
    message: Option<MessageStartData>,

    // ContentBlock fields
    index: Option<usize>,
    content_block: Option<ContentBlockStart>,

    // Delta fields
    delta: Option<DeltaData>,

    // Usage fields
    usage: Option<StreamUsage>,

    // Error fields
    error: Option<ApiError>,
}

#[derive(Debug, Deserialize)]
struct MessageStartData {
    id: String,
    #[serde(rename = "type")]
    message_type: String,
    role: String,
    model: String,
    usage: StreamUsage,
    #[allow(dead_code)]
    content: Vec<serde_json::Value>, // Empty initially
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum DeltaData {
    Content(ContentDelta),
    Message(MessageDeltaEvent),
}

/// Wrapper for the SSE stream
#[pin_project]
pub struct MessageStream {
    #[pin]
    inner: Box<dyn Stream<Item = Result<StreamEvent>> + Send + Unpin>,
}

impl MessageStream {
    /// Create a new message stream from an HTTP response
    pub fn new(response: reqwest::Response) -> Self {
        let byte_stream = response.bytes_stream();
        let event_stream = byte_stream.eventsource();

        let mapped_stream = futures::stream::StreamExt::map(event_stream, |result| {
            match result {
                Ok(event) => {
                    // Parse the event data
                    let data = event.data;

                    // Parse as SSE event
                    let sse_event: SseEvent =
                        serde_json::from_str(&data).context("Failed to parse SSE event")?;

                    // Convert to StreamEvent
                    Self::parse_event(sse_event)
                }
                Err(e) => Err(anyhow::anyhow!("SSE stream error: {}", e)),
            }
        });

        Self {
            inner: Box::new(mapped_stream),
        }
    }

    fn parse_event(event: SseEvent) -> Result<StreamEvent> {
        match event.event_type.as_str() {
            "message_start" => {
                let msg_data = event.message.context("Missing message data")?;
                Ok(StreamEvent::MessageStart {
                    message: MessageStart {
                        id: msg_data.id,
                        model: msg_data.model,
                        message_type: msg_data.message_type,
                        role: msg_data.role,
                        usage: msg_data.usage,
                    },
                })
            }

            "content_block_start" => {
                let index = event.index.context("Missing index")?;
                let content_block = event.content_block.context("Missing content_block")?;
                Ok(StreamEvent::ContentBlockStart {
                    index,
                    content_block,
                })
            }

            "content_block_delta" => {
                let index = event.index.context("Missing index")?;
                let delta = match event.delta.context("Missing delta")? {
                    DeltaData::Content(d) => d,
                    _ => {
                        return Err(anyhow::anyhow!(
                            "Invalid delta type for content_block_delta"
                        ));
                    }
                };
                Ok(StreamEvent::ContentBlockDelta { index, delta })
            }

            "content_block_stop" => {
                let index = event.index.context("Missing index")?;
                Ok(StreamEvent::ContentBlockStop { index })
            }

            "message_delta" => {
                let delta = match event.delta.context("Missing delta")? {
                    DeltaData::Message(d) => d,
                    _ => return Err(anyhow::anyhow!("Invalid delta type for message_delta")),
                };
                let usage = event.usage.context("Missing usage")?.into();
                Ok(StreamEvent::MessageDelta { delta, usage })
            }

            "message_stop" => Ok(StreamEvent::MessageStop),

            "ping" => Ok(StreamEvent::Ping),

            "error" => {
                let error = event.error.context("Missing error data")?;
                Ok(StreamEvent::Error { error })
            }

            _ => {
                // Unknown event type - log but don't fail
                // This is per Anthropic's versioning policy
                tracing::warn!("Unknown SSE event type: {}", event.event_type);
                Ok(StreamEvent::Ping) // Treat as ping
            }
        }
    }
}

impl Stream for MessageStream {
    type Item = Result<StreamEvent>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut TaskContext<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.project();
        this.inner.as_mut().poll_next(cx)
    }
}

/// Helper to accumulate stream events into a complete response
pub struct StreamAccumulator {
    text_blocks: Vec<String>,
    tool_uses: Vec<ToolUse>,
    current_tool_json: Option<String>,
    current_tool_id: Option<String>,
    current_tool_name: Option<String>,
    stop_reason: Option<StopReason>,
    usage: Option<Usage>,
}

impl StreamAccumulator {
    pub fn new() -> Self {
        Self {
            text_blocks: Vec::new(),
            tool_uses: Vec::new(),
            current_tool_json: None,
            current_tool_id: None,
            current_tool_name: None,
            stop_reason: None,
            usage: None,
        }
    }

    /// Process a stream event and update accumulator state
    pub fn process_event(&mut self, event: StreamEvent) -> Result<()> {
        match event {
            StreamEvent::MessageStart { message } => {
                self.usage = Some(message.usage.into());
            }

            StreamEvent::ContentBlockStart { content_block, .. } => {
                match content_block {
                    ContentBlockStart::Text { text } => {
                        self.text_blocks.push(text);
                    }
                    ContentBlockStart::ToolUse { id, name } => {
                        self.current_tool_id = Some(id);
                        self.current_tool_name = Some(name);
                        self.current_tool_json = Some(String::new());
                    }
                    ContentBlockStart::Thinking { .. } => {
                        // Skip thinking content for now
                    }
                }
            }

            StreamEvent::ContentBlockDelta { delta, .. } => match delta {
                ContentDelta::TextDelta { text } => {
                    if let Some(last) = self.text_blocks.last_mut() {
                        last.push_str(&text);
                    } else {
                        self.text_blocks.push(text);
                    }
                }
                ContentDelta::InputJsonDelta { partial_json } => {
                    if let Some(json) = &mut self.current_tool_json {
                        json.push_str(&partial_json);
                    }
                }
            },

            StreamEvent::ContentBlockStop { .. } => {
                // If we were accumulating tool JSON, parse it now
                if let (Some(id), Some(name), Some(json_str)) = (
                    self.current_tool_id.take(),
                    self.current_tool_name.take(),
                    self.current_tool_json.take(),
                ) {
                    let input: serde_json::Value = serde_json::from_str(&json_str)
                        .context("Failed to parse tool input JSON")?;

                    self.tool_uses.push(ToolUse { id, name, input });
                }
            }

            StreamEvent::MessageDelta { delta, usage } => {
                if let Some(reason) = delta.stop_reason {
                    self.stop_reason = Some(reason);
                }
                self.usage = Some(usage);
            }

            StreamEvent::MessageStop => {
                // Message complete
            }

            StreamEvent::Ping => {
                // Keepalive, no action needed
            }

            StreamEvent::Error { error } => {
                return Err(anyhow::anyhow!(
                    "Stream error ({}): {}",
                    error.error_type,
                    error.message
                ));
            }
        }

        Ok(())
    }

    /// Get the accumulated text content
    pub fn text(&self) -> String {
        self.text_blocks.join("\n")
    }

    /// Get accumulated tool uses
    pub fn tool_uses(&self) -> &[ToolUse] {
        &self.tool_uses
    }

    /// Get stop reason
    pub fn stop_reason(&self) -> Option<StopReason> {
        self.stop_reason.clone()
    }

    /// Get usage information
    pub fn usage(&self) -> Option<Usage> {
        self.usage.clone()
    }
}

impl Default for StreamAccumulator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stream_accumulator() {
        let mut acc = StreamAccumulator::new();

        // Simulate message start
        let usage = StreamUsage {
            input_tokens: 100,
            output_tokens: 0,
            cache_creation_input_tokens: None,
            cache_read_input_tokens: None,
        };
        acc.process_event(StreamEvent::MessageStart {
            message: MessageStart {
                id: "msg_123".to_string(),
                model: "claude-sonnet-4-5-20250929".to_string(),
                message_type: "message".to_string(),
                role: "assistant".to_string(),
                usage,
            },
        })
        .unwrap();

        // Simulate text content
        acc.process_event(StreamEvent::ContentBlockStart {
            index: 0,
            content_block: ContentBlockStart::Text {
                text: "Hello".to_string(),
            },
        })
        .unwrap();

        acc.process_event(StreamEvent::ContentBlockDelta {
            index: 0,
            delta: ContentDelta::TextDelta {
                text: " world".to_string(),
            },
        })
        .unwrap();

        acc.process_event(StreamEvent::ContentBlockStop { index: 0 })
            .unwrap();

        assert_eq!(acc.text(), "Hello world");
        assert_eq!(acc.tool_uses().len(), 0);
    }

    #[test]
    fn test_stream_accumulator_tool_use() {
        let mut acc = StreamAccumulator::new();

        // Simulate tool use
        acc.process_event(StreamEvent::ContentBlockStart {
            index: 0,
            content_block: ContentBlockStart::ToolUse {
                id: "tool_123".to_string(),
                name: "calculator".to_string(),
            },
        })
        .unwrap();

        // Send complete JSON in parts
        acc.process_event(StreamEvent::ContentBlockDelta {
            index: 0,
            delta: ContentDelta::InputJsonDelta {
                partial_json: "{\"a\":".to_string(),
            },
        })
        .unwrap();

        acc.process_event(StreamEvent::ContentBlockDelta {
            index: 0,
            delta: ContentDelta::InputJsonDelta {
                partial_json: "1,\"b\":2}".to_string(),
            },
        })
        .unwrap();

        let result = acc.process_event(StreamEvent::ContentBlockStop { index: 0 });
        if let Err(e) = &result {
            eprintln!("Error processing ContentBlockStop: {}", e);
        }
        result.unwrap();

        assert_eq!(acc.tool_uses().len(), 1);
        assert_eq!(acc.tool_uses()[0].id, "tool_123");
        assert_eq!(acc.tool_uses()[0].name, "calculator");
        assert_eq!(acc.tool_uses()[0].input["a"], 1);
        assert_eq!(acc.tool_uses()[0].input["b"], 2);
    }

    #[test]
    fn test_content_delta_variants() {
        let text_delta = ContentDelta::TextDelta {
            text: "test".to_string(),
        };
        assert!(matches!(text_delta, ContentDelta::TextDelta { .. }));

        let json_delta = ContentDelta::InputJsonDelta {
            partial_json: "{}".to_string(),
        };
        assert!(matches!(json_delta, ContentDelta::InputJsonDelta { .. }));
    }

    #[test]
    fn test_usage_conversion() {
        let stream_usage = StreamUsage {
            input_tokens: 100,
            output_tokens: 50,
            cache_creation_input_tokens: Some(20),
            cache_read_input_tokens: Some(10),
        };

        let usage: Usage = stream_usage.into();
        assert_eq!(usage.input_tokens, 100);
        assert_eq!(usage.output_tokens, 50);
        assert_eq!(usage.cache_creation_tokens, Some(20));
        assert_eq!(usage.cache_read_tokens, Some(10));
    }
}
