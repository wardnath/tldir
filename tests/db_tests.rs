use anyhow::Result;
use tldir::models::store::EmbeddingStore;
use tempfile::tempdir;

#[tokio::test]
async fn test_embedding_storage_and_retrieval() -> Result<()> {
    let temp_dir = tempdir()?;
    let db_pool = tldir::models::db::init_db(temp_dir.path()).await?;
    let store = EmbeddingStore::new(db_pool);
    
    // Test collection creation
    let collection_id = store.create_collection("test", "code").await?;
    
    // Test embedding storage
    let vector = vec![0.1, 0.2, 0.3];
    let metadata = vec![("file", "test.rs".to_string())];
    let id = store.store_embedding("test-segment", &vector, Some(metadata)).await?;
    
    // Test similarity search
    let similar = store.find_similar_embeddings(&vector, 1).await?;
    assert_eq!(similar.len(), 1);
    assert_eq!(similar[0].0.id, id);
    
    Ok(())
}
