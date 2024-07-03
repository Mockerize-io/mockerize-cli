use std::path::Path;

use anyhow::{Ok, Result};
use clap::Parser;

use crate::http::ServerInfo;

/// Test if a server config is parsable and meets all requirements
#[derive(Parser, Debug)]
pub struct TestCommand {
    /// Path to output new config file to
    pub config_file: String,
}

impl TestCommand {
    /// Handles `mockerize-cli test <FILENAME>` - Tests config file to see if it contains errors
    pub fn handle(&self) -> Result<()> {
        let _serverinfo = ServerInfo::from_file(Path::new(&self.config_file))?;
        Ok(())
    }
}
