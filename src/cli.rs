use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "tldir")]
#[command(about = "Directory summarization and querying tool")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Scan and summarize the directory
    Scan {
        /// Directory to scan
        dirname: PathBuf,

        /// Set the summary length in tokens
        #[arg(short, long, default_value = "8192")]
        summary_length: usize,

        /// Include hidden files and directories
        #[arg(short, long)]
        include_hidden: bool,

        /// Run on CPU rather than GPU
        #[arg(long)]
        cpu: bool,

        /// Enable tracing (generates a trace-timestamp.json file)
        #[arg(long)]
        tracing: bool,

        /// Model revision to use
        #[arg(long, default_value = "main")]
        revision: String,
    },

    /// Ask questions about the directory's content
    Ask {
        /// Directory to query
        dirname: PathBuf,

        /// Optional direct question (if not provided, enters interactive mode)
        #[arg(long)]
        question: Option<String>,

        /// Run on CPU rather than GPU
        #[arg(long)]
        cpu: bool,

        /// Model revision to use
        #[arg(long, default_value = "main")]
        revision: String,
    },
}
