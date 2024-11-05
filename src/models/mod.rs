use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct DirectoryChunk {
    pub id: i64,
    pub content: String,
    pub file_path: String,
    pub summary: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DirectoryEmbedding {
    pub chunk_id: i64,
    pub embedding: Vec<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub summary_length: usize,
    pub include_hidden: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            summary_length: 8192,
            include_hidden: false,
        }
    }
}
