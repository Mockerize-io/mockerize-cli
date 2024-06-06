use clap::{Parser, Subcommand};
use std::io;

use super::{NewCommand, RunCommand};

#[derive(Parser, Debug)]
#[command(name = "mockerize-cli")]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    New(NewCommand),
    Run(RunCommand),
}

/// Require that a user confirm an action. They *must* enter yes/y or no/n
pub fn prompt_for_confirmation() -> bool {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let trimmed_input = input.trim().to_lowercase();

        match trimmed_input.as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => {
                println!("Please enter 'yes' or 'no'.");
            }
        }
    }
}
