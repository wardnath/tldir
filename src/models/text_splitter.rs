use anyhow::Result;
use std::path::Path;
use text_splitter::{TextSplitter, TokenSplitter};
use tokio::fs;

pub struct ChunkConfig {
    pub chunk_size: usize,
    pub overlap: usize,
    pub min_chunk_size: usize,
}

impl Default for ChunkConfig {
    fn default() -> Self {
        Self {
            chunk_size: 1024,
            overlap: 100,
            min_chunk_size: 100,
        }
    }
}

pub struct TextChunker {
    config: ChunkConfig,
    splitter: TokenSplitter,
}

impl TextChunker {
    pub fn new(config: ChunkConfig) -> Self {
        Self {
            splitter: TokenSplitter::default().with_trim_chunks(true),
            config,
        }
    }

    pub async fn chunk_file(&self, file_path: &Path) -> Result<Vec<String>> {
        let content = fs::read_to_string(file_path).await?;
        self.chunk_text(&content)
    }

    pub fn chunk_text(&self, text: &str) -> Result<Vec<String>> {
        Ok(self
            .splitter
            .chunks(
                text,
                self.config.chunk_size,
                self.config.overlap,
                self.config.min_chunk_size,
            )
            .collect())
    }
}
