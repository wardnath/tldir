use anyhow::Result;
use std::path::Path;
use tokio::fs;

pub struct Chunk {
    pub content: String,
    pub file_path: String,
    pub start_pos: usize,
    pub end_pos: usize,
}

pub struct DirectoryChunker {
    chunk_size: usize,
    overlap: usize,
}

impl DirectoryChunker {
    pub fn new(chunk_size: usize, overlap: usize) -> Self {
        Self { 
            chunk_size,
            overlap,
        }
    }

    pub async fn chunk_directory(&self, dir_path: &Path) -> Result<Vec<Chunk>> {
        let mut chunks = Vec::new();
        let mut entries = fs::read_dir(dir_path).await?;

        while let Some(entry) = entries.next_entry().await? {
            if !self.should_process_file(&entry).await? {
                continue;
            }

            let file_chunks = self.chunk_file(&entry.path()).await?;
            chunks.extend(file_chunks);
        }

        Ok(chunks)
    }

    async fn chunk_file(&self, file_path: &Path) -> Result<Vec<Chunk>> {
        let content = fs::read_to_string(file_path).await?;
        let mut chunks = Vec::new();
        let mut start = 0;

        while start < content.len() {
            let end = (start + self.chunk_size).min(content.len());
            chunks.push(Chunk {
                content: content[start..end].to_string(),
                file_path: file_path.to_string_lossy().to_string(),
                start_pos: start,
                end_pos: end,
            });
            start = end - self.overlap;
        }

        Ok(chunks)
    }
}
