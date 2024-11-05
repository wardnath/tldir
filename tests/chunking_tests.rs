use anyhow::Result;
use std::path::PathBuf;
use tldir::chunking::DirectoryChunker;

#[tokio::test]
async fn test_chunk_size_and_overlap() -> Result<()> {
    let chunker = DirectoryChunker::new(100, 20);
    let test_file = PathBuf::from("tests/fixtures/sample.txt");
    
    let chunks = chunker.chunk_file(&test_file).await?;
    
    // Check chunk properties
    for (i, chunk) in chunks.iter().enumerate() {
        if i > 0 {
            let prev_chunk = &chunks[i - 1];
            assert_eq!(
                chunk.content[..20],
                prev_chunk.content[prev_chunk.content.len() - 20..]
            );
        }
        assert!(chunk.content.len() <= 100);
    }
    
    Ok(())
}

#[tokio::test]
async fn test_utf8_validation() -> Result<()> {
    let chunker = DirectoryChunker::new(100, 20);
    let result = chunker
        .chunk_file(&PathBuf::from("tests/fixtures/binary.bin"))
        .await;
        
    assert!(result.is_err());
    Ok(())
}
