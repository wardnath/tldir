use super::embeddings::{Collection, Embedding, EmbeddingMetadata};
use anyhow::Result;
use sqlx::sqlite::SqlitePool;
use uuid::Uuid;

pub struct EmbeddingStore {
    pool: SqlitePool,
}

impl EmbeddingStore {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create_collection(&self, name: &str, topic: &str) -> Result<String> {
        let id = Uuid::new_v4().to_string();
        
        sqlx::query!(
            "INSERT INTO collections (id, name, topic) VALUES (?, ?, ?)",
            id,
            name,
            topic
        )
        .execute(&self.pool)
        .await?;

        Ok(id)
    }

    pub async fn store_embedding(
        &self,
        segment_id: &str,
        vector: &[f32],
        metadata: Option<Vec<(&str, String)>>,
    ) -> Result<i64> {
        let embedding_id = Uuid::new_v4().to_string();
        let vector_bytes: Vec<u8> = bincode::serialize(vector)?;

        let id = sqlx::query!(
            "INSERT INTO embeddings (segment_id, embedding_id, vector) VALUES (?, ?, ?)",
            segment_id,
            embedding_id,
            vector_bytes,
        )
        .execute(&self.pool)
        .await?
        .last_insert_rowid();

        if let Some(metadata) = metadata {
            for (key, value) in metadata {
                sqlx::query!(
                    "INSERT INTO embedding_metadata (id, key, string_value) VALUES (?, ?, ?)",
                    id,
                    key,
                    value
                )
                .execute(&self.pool)
                .await?;
            }
        }

        Ok(id)
    }

    pub async fn find_similar_embeddings(
        &self,
        query_vector: &[f32],
        limit: usize,
    ) -> Result<Vec<(Embedding, f32)>> {
        // Implement cosine similarity search
        // This is a simplified version - in production you'd want to use 
        // more efficient similarity search algorithms
        let embeddings = sqlx::query_as!(
            Embedding,
            "SELECT * FROM embeddings"
        )
        .fetch_all(&self.pool)
        .await?;

        let mut results: Vec<(Embedding, f32)> = embeddings
            .into_iter()
            .map(|emb| {
                let vector: Vec<f32> = bincode::deserialize(&emb.vector).unwrap();
                let similarity = cosine_similarity(query_vector, &vector);
                (emb, similarity)
            })
            .collect();

        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        Ok(results.into_iter().take(limit).collect())
    }
}

fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    dot_product / (norm_a * norm_b)
}
