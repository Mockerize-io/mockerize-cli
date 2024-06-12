use std::process;

use anyhow::{Ok, Result};
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
