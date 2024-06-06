use anyhow::Result;
use clap::{CommandFactory, Parser};
use dotenv::dotenv;
use env_logger::Builder;
use log::LevelFilter;

use cli::{Args, Commands};

mod cli;
mod http;
mod startup;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    setup_logging();

    let args = Args::parse();
    match &args.command {
        Some(Commands::New(cmd)) => cmd.handle()?,
        Some(Commands::Run(cmd)) => cmd.handle().await?,
        None => {
            Args::command().print_help()?;
            println!();
        }
    }

    Ok(())
}

/// Configure logging, considering env vars with safe defaults
fn setup_logging() {
    // Default to Info level if a RUST_LOG env var not set
    if std::env::var("RUST_LOG").is_err() {
        Builder::from_default_env()
            .filter(None, LevelFilter::Info)
            .init();
    } else {
        env_logger::init();
    }
}
