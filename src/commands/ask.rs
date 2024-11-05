use anyhow::Result;
use std::path::PathBuf;
use sqlx::SqlitePool;

pub async fn ask_question(
    dirname: PathBuf,
    question: Option<String>,
) -> Result<()> {
    let tldir_path = dirname.join(".tldir");
    let db_path = tldir_path.join("embeddings.db");
    
    // Connect to database
    let pool = SqlitePool::connect(&format!("sqlite:{}", db_path.display())).await?;

    match question {
        Some(q) => answer_question(&pool, &q).await?,
        None => interactive_mode(&pool).await?,
    }

    Ok(())
}

async fn answer_question(pool: &SqlitePool, question: &str) -> Result<()> {
    // Generate embedding for question
    let question_embedding = generate_embedding(question)?;

    // Find relevant chunks using Chroma/SQLite
    let chunks = find_relevant_chunks(pool, &question_embedding).await?;

    // Generate answer using Mistral
    let answer = generate_answer(&chunks, question)?;
    println!("{}", answer);

    Ok(())
}

async fn interactive_mode(pool: &SqlitePool) -> Result<()> {
    println!("Enter your questions (type 'exit' to quit):");
    
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        
        let input = input.trim();
        if input == "exit" {
            break;
        }
        
        answer_question(pool, input).await?;
    }

    Ok(())
}
