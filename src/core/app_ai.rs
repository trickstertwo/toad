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

        // Spawn async task to process query
        tokio::spawn(async move {
            match llm_client.send_message(conversation, None).await {
                Ok(response) => {
                    // Send AI response back to main thread
                    let assistant_message = Message::assistant(response.content);
                    let _ = event_tx.send(crate::core::event::Event::AIResponse(assistant_message));
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
