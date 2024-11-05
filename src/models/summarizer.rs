use anyhow::Result;
use mistralrs::{Mistral, ModelBuilder, TextMessageRole, TextMessages};

pub struct Summarizer {
    model: Mistral,
}

impl Summarizer {
    pub async fn new() -> Result<Self> {
        let model = ModelBuilder::new("mistralai/Mistral-7B-Instruct-v0.2")
            .with_logging()
            .build()
            .await?;

        Ok(Self { model })
    }

    pub async fn summarize(&self, text: &str, max_length: usize) -> Result<String> {
        let prompt = format!(
            "Please summarize the following text concisely in {} tokens or less:\n\n{}",
            max_length, text
        );

        let messages = TextMessages::new().add_message(TextMessageRole::User, &prompt);
        let response = self.model.send_chat_request(messages).await?;

        Ok(response
            .choices
            .first()
            .and_then(|c| c.message.content.as_ref())
            .map(|s| s.to_string())
            .unwrap_or_default())
    }

    pub async fn summarize_chunks(&self, chunks: &[String], max_length: usize) -> Result<String> {
        let mut summaries = Vec::new();
        
        // First level: Summarize individual chunks
        for chunk in chunks {
            let summary = self.summarize(chunk, max_length / chunks.len()).await?;
            summaries.push(summary);
        }

        // Second level: Combine summaries
        if summaries.len() > 1 {
            let combined = summaries.join("\n\n");
            self.summarize(&combined, max_length).await
        } else {
            Ok(summaries.into_iter().next().unwrap_or_default())
        }
    }
}
