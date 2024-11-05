use crate::models::{text_splitter::{ChunkConfig, TextChunker}, summarizer::Summarizer};
use anyhow::Result;
use std::path::PathBuf;
use tokio::fs;
use walkdir::WalkDir;

pub async fn scan_directory(
    dirname: PathBuf,
    config: &Config,
    cpu: bool,
    revision: &str,
) -> Result<()> {
    let chunker = TextChunker::new(ChunkConfig::default());
    let summarizer = Summarizer::new().await?;

    // Create .tldir directory
    let tldir_path = dirname.join(".tldir");
    fs::create_dir_all(&tldir_path).await?;

    let mut all_chunks = Vec::new();

    // Walk directory and process files
    for entry in WalkDir::new(&dirname)
        .follow_links(true)
        .into_iter()
        .filter_entry(|e| is_valid_entry(e, config.include_hidden))
    {
        let entry = entry?;
        if !entry.file_type().is_file() {
            continue;
        }

        // Process file
        let chunks = chunker.chunk_file(entry.path()).await?;
        all_chunks.extend(chunks);
    }

    // Generate summary
    let summary = summarizer
        .summarize_chunks(&all_chunks, config.summary_length)
        .await?;

    // Save summary
    fs::write(tldir_path.join("summary.txt"), summary).await?;

    Ok(())
}

fn is_valid_entry(entry: &walkdir::DirEntry, include_hidden: bool) -> bool {
    if !include_hidden && is_hidden(entry) {
        return false;
    }
    true
}

fn is_hidden(entry: &walkdir::DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}
