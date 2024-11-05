use anyhow::Result;
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use std::path::Path;

pub async fn init_db(dir_path: &Path) -> Result<SqlitePool> {
    let db_path = dir_path.join(".tldir").join("embeddings.db");
    let db_url = format!("sqlite:{}", db_path.display());
    
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    // Create tables based on Chroma's schema
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS collections (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            topic TEXT NOT NULL,
            UNIQUE (name)
        );

        CREATE TABLE IF NOT EXISTS collection_metadata (
            collection_id TEXT REFERENCES collections(id) ON DELETE CASCADE,
            key TEXT NOT NULL,
            str_value TEXT,
            int_value INTEGER,
            float_value REAL,
            PRIMARY KEY (collection_id, key)
        );

        CREATE TABLE IF NOT EXISTS embeddings (
            id INTEGER PRIMARY KEY,
            segment_id TEXT NOT NULL,
            embedding_id TEXT NOT NULL,
            vector BLOB NOT NULL,
            created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
            UNIQUE (segment_id, embedding_id)
        );

        CREATE TABLE IF NOT EXISTS embedding_metadata (
            id INTEGER REFERENCES embeddings(id),
            key TEXT NOT NULL,
            string_value TEXT,
            int_value INTEGER,
            float_value REAL,
            PRIMARY KEY (id, key)
        );

        CREATE VIRTUAL TABLE IF NOT EXISTS embedding_fulltext 
        USING fts5(id, string_value);
        "#
    )
    .execute(&pool)
    .await?;

    Ok(pool)
}
