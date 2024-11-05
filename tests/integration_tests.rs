use anyhow::Result;
use tldir::cli::{Cli, Commands};
use std::path::PathBuf;
use tempfile::tempdir;

#[tokio::test]
async fn test_scan_and_query_workflow() -> Result<()> {
    let temp_dir = tempdir()?;
    let test_dir = PathBuf::from("tests/fixtures/sample_project");
    
    // Test scan command
    let scan_result = tldir::commands::scan::scan_directory(
        test_dir.clone(),
        &tldir::Config::default(),
        true,
        "main",
    ).await;
    assert!(scan_result.is_ok());
    
    // Test ask command
    let response = tldir::commands::ask::ask_question(
        test_dir,
        Some("What files are in the project?".to_string()),
        true,
        "main",
    ).await?;
    
    assert!(!response.is_empty());
    Ok(())
}
