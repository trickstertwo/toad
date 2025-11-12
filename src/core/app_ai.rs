//! AI chat processing methods
//!
//! Handles AI query processing, LLM communication, and conversation management.

use crate::ai::llm::Message;
use crate::core::app::App;

impl App {
    /// Process an AI query asynchronously
    ///
    /// Sends the user's query to the LLM and updates the conversation with the response.
    ///
    /// # Parameters
    ///
    /// - `query`: The user's query string
    ///
    /// # Errors
    ///
    /// Shows error toast if:
    /// - No LLM client is available (missing API key)
    /// - LLM request fails
    /// - Network error occurs
    pub(crate) fn process_ai_query(&mut self, query: String) {
        // Check if LLM client is available
        if !self.has_llm_client() {
            self.toast_error(
                "No LLM client available. Set ANTHROPIC_API_KEY environment variable.",
            );
            self.status_message =
                "Error: ANTHROPIC_API_KEY not set. Cannot process AI queries.".to_string();
            return;
        }

        // Add user message to conversation
        let user_message = Message::user(&query);
        self.add_message(user_message.clone());

        // Set AI processing state
        self.set_ai_processing(true);
        self.status_message = "AI is thinking...".to_string();

        // Get event sender for async response
        let event_tx = match &self.event_tx {
            Some(tx) => tx.clone(),
            None => {
                self.toast_error("Event system not initialized");
                self.set_ai_processing(false);
                return;
            }
        };

        // Get LLM client
        let llm_client = match self.llm_client() {
            Some(client) => client.clone(),
            None => {
                self.toast_error("LLM client not available");
                self.set_ai_processing(false);
                return;
            }
        };

        // Build conversation history for context
        let conversation = self.conversation.clone();

        // Spawn async task to process query with streaming
        tokio::spawn(async move {
            use crate::ai::llm::streaming::{ContentDelta, StreamEvent};
            use futures::StreamExt;

            match llm_client.send_message_stream(conversation, None).await {
                Ok(mut stream) => {
                    // Send stream start event
                    let _ = event_tx.send(crate::core::event::Event::AIStreamStart);

                    // Process streaming events
                    while let Some(result) = stream.next().await {
                        match result {
                            Ok(event) => match event {
                                StreamEvent::ContentBlockDelta { delta, .. } => {
                                    // Extract text from delta and send to UI
                                    if let ContentDelta::TextDelta { text } = delta {
                                        let _ = event_tx
                                            .send(crate::core::event::Event::AIStreamDelta(text));
                                    }
                                }
                                StreamEvent::MessageDelta { usage, .. } => {
                                    // Send token usage update
                                    let _ = event_tx.send(crate::core::event::Event::AITokenUsage {
                                        input_tokens: usage.input_tokens,
                                        output_tokens: usage.output_tokens,
                                    });
                                }
                                StreamEvent::Error { error } => {
                                    // Error during streaming
                                    let _ = event_tx.send(crate::core::event::Event::AIError(
                                        format!("{}: {}", error.error_type, error.message),
                                    ));
                                    return;
                                }
                                _ => {
                                    // Ignore other event types (MessageStart, Ping, etc.)
                                }
                            },
                            Err(e) => {
                                // Stream error
                                let _ = event_tx.send(crate::core::event::Event::AIError(
                                    e.to_string(),
                                ));
                                return;
                            }
                        }
                    }

                    // Send stream complete event
                    let _ = event_tx.send(crate::core::event::Event::AIStreamComplete);
                }
                Err(e) => {
                    // Send error back to main thread
                    let _ = event_tx.send(crate::core::event::Event::AIError(e.to_string()));
                }
            }
        });
    }

    /// Handle AI response event
    ///
    /// Called when an AI response is received from the async task.
    pub(crate) fn handle_ai_response(&mut self, message: Message) {
        self.add_message(message);
        self.set_ai_processing(false);
        self.status_message = "AI response received".to_string();
    }

    /// Handle AI error event
    ///
    /// Called when an AI request fails.
    pub(crate) fn handle_ai_error(&mut self, error: String) {
        self.toast_error(format!("AI error: {}", error));
        self.status_message = format!("AI error: {}", error);
        self.set_ai_processing(false);

        // Add error message to conversation
        let error_msg = Message::assistant(format!("Error: {}", error));
        self.add_message(error_msg);
    }

    /// Handle AI stream start event
    ///
    /// Called when AI starts streaming a response.
    pub(crate) fn handle_ai_stream_start(&mut self) {
        self.conversation_view.start_streaming();
        self.status_message = "AI is responding...".to_string();
    }

    /// Handle AI stream delta event
    ///
    /// Called when AI sends a chunk of streaming content.
    pub(crate) fn handle_ai_stream_delta(&mut self, content: String) {
        self.conversation_view.append_streaming_content(&content);
    }

    /// Handle AI stream complete event
    ///
    /// Called when AI finishes streaming a response.
    pub(crate) fn handle_ai_stream_complete(&mut self) {
        // Only complete if still streaming (handles cancellation gracefully)
        if self.conversation_view.is_streaming() {
            self.conversation_view.complete_streaming();
            self.set_ai_processing(false);
            self.status_message = "AI response complete".to_string();

            // Auto-save session after AI response
            if let Err(e) = self.save_session() {
                // Log error but don't interrupt user experience
                tracing::warn!("Failed to save session after AI response: {}", e);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai::llm::{MockResponseBuilder, SequencedMockClient};
    use std::sync::Arc;

    #[test]
    fn test_process_ai_query_no_client() {
        let mut app = App::new();
        app.llm_client = None; // Ensure no client

        app.process_ai_query("test query".to_string());

        // Should show error
        assert!(app.status_message.contains("ANTHROPIC_API_KEY"));
    }

    #[test]
    fn test_process_ai_query_with_mock_client() {
        let mut app = App::new();

        // Create mock client
        let mock = MockResponseBuilder::new()
            .with_text("Mock response")
            .build();

        app.llm_client = Some(Arc::new(mock));

        // This will fail because event_tx is None, but it tests the initial logic
        app.process_ai_query("test query".to_string());

        // Should have added user message to conversation
        assert_eq!(app.conversation().len(), 1);
        assert_eq!(app.conversation()[0].content, "test query");
    }

    #[test]
    fn test_handle_ai_response() {
        let mut app = App::new();
        app.set_ai_processing(true);

        let response = Message::assistant("Test response");
        app.handle_ai_response(response);

        assert!(!app.is_ai_processing());
        assert_eq!(app.conversation().len(), 1);
        assert_eq!(app.conversation()[0].content, "Test response");
    }

    #[test]
    fn test_handle_ai_error() {
        let mut app = App::new();
        app.set_ai_processing(true);

        app.handle_ai_error("Test error".to_string());

        assert!(!app.is_ai_processing());
        assert!(app.status_message.contains("error"));
        assert_eq!(app.conversation().len(), 1);
        assert!(app.conversation()[0].content.contains("Error"));
    }

    #[test]
    fn test_conversation_accumulation() {
        let mut app = App::new();

        // Simulate conversation
        app.add_message(Message::user("First question"));
        app.add_message(Message::assistant("First answer"));
        app.add_message(Message::user("Second question"));
        app.add_message(Message::assistant("Second answer"));

        assert_eq!(app.conversation().len(), 4);
    }

    #[test]
    fn test_clear_conversation() {
        let mut app = App::new();

        app.add_message(Message::user("Test"));
        app.add_message(Message::assistant("Response"));
        assert_eq!(app.conversation().len(), 2);

        app.clear_conversation();
        assert_eq!(app.conversation().len(), 0);
    }
}
