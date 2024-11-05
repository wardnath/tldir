use anyhow::Result;
use candle::{Device, Tensor};
use tldir::{
    models::embeddings::{EmbeddingModel, PoolingStrategy},
    utils::test_utils::get_test_device,
};

#[test]
fn test_embedding_model_initialization() -> Result<()> {
    let device = get_test_device(true)?;
    let model = EmbeddingModel::new(
        "test_models/tokenizer.json".into(),
        "test_models/model.safetensors".into(),
        true,
    )?;
    
    assert!(model.is_ok());
    Ok(())
}

#[test]
fn test_embedding_pooling_strategies() -> Result<()> {
    let model = EmbeddingModel::new(
        "test_models/tokenizer.json".into(),
        "test_models/model.safetensors".into(),
        true,
    )?;

    let text = "Test sentence for embedding";
    
    // Test different pooling strategies
    let cls_embedding = model.encode_with_pooling(text, PoolingStrategy::Cls)?;
    let last_embedding = model.encode_with_pooling(text, PoolingStrategy::LastToken)?;
    let mean_embedding = model.encode_with_pooling(text, PoolingStrategy::Mean)?;
    
    assert_eq!(cls_embedding.len(), last_embedding.len());
    assert_eq!(cls_embedding.len(), mean_embedding.len());
    
    Ok(())
}
