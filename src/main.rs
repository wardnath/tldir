mod cli;
mod commands;
mod models;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan {
            dirname,
            summary_length,
            include_hidden,
            cpu,
            tracing,
            revision,
        } => {
            if tracing {
                setup_tracing()?;
            }

            let config = models::Config {
                summary_length,
                include_hidden,
            };

            commands::scan::scan_directory(dirname, &config, cpu, &revision).await?;
        }
        Commands::Ask {
            dirname,
            question,
            cpu,
            revision,
        } => {
            commands::ask::ask_question(dirname, question, cpu, &revision).await?;
        }
    }

    Ok(())
}

fn setup_tracing() -> Result<()> {
    use tracing_chrome::ChromeLayerBuilder;
    use tracing_subscriber::prelude::*;

    let (chrome_layer, _guard) = ChromeLayerBuilder::new().build();
    tracing_subscriber::registry()
        .with(chrome_layer)
        .init();
    Ok(())
}
