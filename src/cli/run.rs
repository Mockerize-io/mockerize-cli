use anyhow::{Context, Result};
use clap::Parser;
use fs2::FileExt;
use std::fs::{remove_file, File};
use std::io::Write;
use std::net::TcpListener;
use std::path::Path;
use std::process::{self};

use crate::{http::ServerInfo, startup::run};

/// Run a mock server from a config file
#[derive(Parser, Debug)]
pub struct RunCommand {
    /// Server config file to load
    pub config_file: String,

    /// Specify the number of workers to use
    #[arg(short, long)]
    pub workers: Option<usize>,

    /// Path to write PID file to. Recommended if running multiple instances.
    #[arg(short, long, default_value = "mockerize-cli.pid")]
    pub pid_file: String,
}

impl RunCommand {
    /// Handles `mockerize-cli <FILENAME>` - run a mock server
    pub async fn handle(&self) -> Result<()> {
        let pid_handle = create_pid_file(&self.pid_file)?;
        let serverinfo = ServerInfo::from_file(&self.config_file)?;

        let addr = format!("{}:{}", serverinfo.server.address, serverinfo.server.port);
        println!("Listening on {}. Press CTRL+C to exit.", &addr);

        let listener = TcpListener::bind(addr.clone())
            .with_context(|| format!("Failed to bind to {}", &addr))?;

        run(serverinfo, listener, self.workers)
            .context("Failure encountered during server's run()")?
            .await?;

        // Cleanup PID file
        pid_handle
            .unlock()
            .with_context(|| format!("Could not unlock PID file `{}`", &self.pid_file))?;

        drop(pid_handle);
        delete_pid_file(&self.pid_file)?;

        Ok(())
    }
}

fn create_pid_file(path: &str) -> Result<File> {
    let pid = process::id();
    let mut file =
        File::create(path).with_context(|| format!("Could not create file at `{}`", &path))?;

    file.try_lock_exclusive()
        .with_context(|| format!("Could not gain exclusive lock on file `{}`", &path))?;

    writeln!(file, "{}", pid)
        .with_context(|| format!("Could not write contents to file `{}`", &path))?;

    Ok(file)
}

fn delete_pid_file(path: &str) -> Result<()> {
    if Path::new(path).exists() {
        remove_file(path).with_context(|| format!("Could not remove_file `{}`", &path))?;
    }
    Ok(())
}
