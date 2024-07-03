use anyhow::{Context, Result};
use clap::Parser;
use std::fs;
use std::net::IpAddr;

use crate::{
    cli::prompt_for_confirmation,
    http::{self, Response, ResponseType, Route, ServerInfo},
};

/// Generate a new server config file
#[derive(Parser, Debug)]
pub struct NewCommand {
    /// Path to output new config file to
    pub config_path: String,

    /// Specify the server's name
    #[arg(short, long, default_value = "Mockerize-cli server")]
    pub name: String,

    /// Specify the server's listen address
    #[arg(short, long, default_value = "127.0.0.1")]
    pub address: String,

    /// Specify the server's listen port
    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,

    /// Assume yes to prompt (ie. don't nag me)
    #[arg(short = 'y', long = "yes")]
    pub confirm: bool,
}

impl NewCommand {
    /// Handles `mockerize-cli new <FILENAME>` - generate a new config
    pub fn handle(&self) -> Result<()> {
        if fs::metadata(&self.config_path).is_ok() {
            // File exist? Prompt for confirmation to overwrite
            println!(
                "File `{}` already exists. Do you want to overwrite it? (yes/no)",
                &self.config_path
            );

            let user_confirmed = prompt_for_confirmation();
            if !user_confirmed {
                println!("Action aborted by user.");
                return Ok(());
            }
        }

        let response = Response::new("Example", 200, ResponseType::Text, "Hello, World");
        let resp_id = response.id; // Copy now to prevent move-related issues a few lines down
        let mut route = Route::new("/hello-world", http::Method::GET);
        route.add_response(response);
        route.set_active_response(resp_id);

        let mut serverinfo = ServerInfo::new()?;
        serverinfo.server.address = self.address.parse::<IpAddr>()?;
        serverinfo.server.port = self.port;
        serverinfo.server.name.clone_from(&self.name);
        serverinfo.server.description = "".into();
        serverinfo.router.add_route(route);
        serverinfo
            .write_to_file(&self.config_path)
            .with_context(|| {
                format!(
                    "Failed to write serialized serverinfo to file `{}`",
                    &self.config_path
                )
            })?;

        Ok(println!(
            "New config successfully saved to `{}`.",
            &self.config_path
        ))
    }
}
