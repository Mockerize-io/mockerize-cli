use std::process;

use anyhow::{Ok, Result};
use clap::{CommandFactory, Parser};
use dotenv::dotenv;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry};

use cli::{Args, Commands};

mod cli;
mod http;
mod startup;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    setup_logging()?;

    let args = Args::parse();
    match &args.command {
        Some(Commands::New(cmd)) => cmd.handle()?,
        Some(Commands::Run(cmd)) => cmd.handle().await?,
        Some(Commands::Test(cmd)) => match cmd.handle() {
            // Specifically, for the test command, we want to print OK|ERROR and exit with the appropriate code.
            // We handle that here as to separate the cmd logic from exit concerns - this also allows easier testing
            core::result::Result::Ok(_) => println!("OK"),
            Err(e) => {
                println!("ERROR");
                for cause in e.chain() {
                    eprintln!("{}", cause);
                }
                process::exit(1);
            }
        },
        None => {
            Args::command().print_help()?;
            println!();
        }
    }

    Ok(())
}

/// Configure logging, considering env vars with safe defaults
fn setup_logging() -> Result<()> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new("mockerize-cli".to_string(), std::io::stdout);
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);

    set_global_default(subscriber)?;

    Ok(())
}
