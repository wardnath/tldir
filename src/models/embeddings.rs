use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use anyhow::Result;
use candle::{DType, Device, Tensor};
use candle_nn::VarBuilder;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Embedding {
    pub id: i64,
    pub segment_id: String,
    pub embedding_id: String,
    pub vector: Vec<f32>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct EmbeddingMetadata {
    pub id: i64,
    pub key: String,
    pub string_value: Option<String>,
    pub int_value: Option<i64>,
    pub float_value: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Collection {
    pub id: String,
    pub name: String,
    pub topic: String,
}

pub enum PoolingStrategy {
    Cls,
    LastToken,
    Mean,
}

pub struct EmbeddingModel {
    model: candle_transformers::models::stella_en_v5::EmbeddingModel,
    device: Device,
    tokenizer: tokenizers::Tokenizer,
}

impl EmbeddingModel {
    pub fn new(model_path: PathBuf, tokenizer_path: PathBuf, cpu: bool) -> Result<Self> {
        let device = if cpu {
            Device::Cpu
        } else {
            Device::new_cuda(0)?
        };

        // Load tokenizer
        let tokenizer = tokenizers::Tokenizer::from_file(tokenizer_path)?;

        // Load model weights
        let weights = std::fs::read(model_path)?;
        let vb = unsafe { 
            VarBuilder::from_mmaped_safetensors(&[weights], DType::F32, &device)?
        };

        // Initialize model
        let model = candle_transformers::models::stella_en_v5::EmbeddingModel::new(
            &candle_transformers::models::stella_en_v5::Config::default(),
            vb.pp("base"),
            vb.pp("embed"),
        )?;

        Ok(Self {
            model,
            device,
            tokenizer,
        })
    }

    pub fn encode_with_pooling(
        &self,
        text: &str,
        pooling: PoolingStrategy,
    ) -> Result<Vec<f32>> {
        let tokens = self.tokenizer.encode(text, true)?;
        let token_ids = Tensor::new(tokens.get_ids(), &self.device)?.unsqueeze(0)?;
        
        let embeddings = self.model.forward(&token_ids)?;
        let (_n_sentence, n_tokens, _hidden_size) = embeddings.dims3()?;
        
        let pooled = match pooling {
            PoolingStrategy::Cls => embeddings.narrow(1, 0, 1)?,
            PoolingStrategy::LastToken => embeddings.narrow(1, n_tokens - 1, 1)?,
            PoolingStrategy::Mean => (embeddings.sum(1)? / (n_tokens as f64))?,
        };
        
        let normalized = pooled.broadcast_div(&pooled.sqr()?.sum_keepdim(1)?.sqrt()?)?;
        Ok(normalized.squeeze(0)?.to_vec1()?)
    }
}
