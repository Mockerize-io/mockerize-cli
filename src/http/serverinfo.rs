use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{Read, Write},
};

use super::{Router, Server};

const DEFAULT_SERVER_ADDR: &str = "127.0.0.1";
const DEFAULT_SERVER_PORT: u16 = 8080;

#[derive(Debug, Deserialize, Serialize)]
pub struct ServerInfo {
    pub server: Server,
    pub router: Router,
}

impl Default for ServerInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl ServerInfo {
    pub fn new() -> Self {
        let mut router = Router::new(None);
        let server = Server::new(router.id, DEFAULT_SERVER_ADDR, DEFAULT_SERVER_PORT);
        router.bind_server(&server);

        Self { server, router }
    }

    pub fn from_file(file_path: &str) -> Result<Self> {
        let mut file = File::open(file_path)
            .with_context(|| format!("Failed to open config file `{}`", file_path))?;

        let mut data = String::new();
        file.read_to_string(&mut data)
            .with_context(|| format!("Failed to read contents of file `{}`", file_path))?;

        let serverinfo = serde_json::from_str(&data).with_context(|| {
            format!(
                "Could not deserialize file contents `{}` into ServerInfo struct",
                file_path
            )
        })?;
        Ok(serverinfo)
    }

    pub fn write_to_file(&self, file_path: &str) -> Result<()> {
        let json = serde_json::to_string_pretty(self)
            .context("Could not serialize ServerInfo struct into JSON")?;

        let mut file = File::create(file_path)
            .with_context(|| format!("Failed to create file `{}` for output", file_path))?;

        file.write_all(json.as_bytes())
            .with_context(|| format!("Could not write contents to file `{}`", file_path))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::http::{Method, Response, ResponseType, Route};

    use super::*;
    use uuid::uuid;

    #[test]
    fn can_load_from_file() {
        let path = "tests/data/example.server.json";
        let serverinfo = ServerInfo::from_file(path);

        assert!(serverinfo.is_ok());
        let serverinfo = serverinfo.unwrap();
        assert_eq!(
            serverinfo.server.id,
            uuid!("a2fe836a-034e-4117-9e0c-4e05df6da784")
        );

        assert_eq!(
            serverinfo.router.id,
            uuid!("d5926f6e-155f-42cd-8f6b-580e1fc8ab1c")
        );
    }

    #[test]
    fn can_serialize_serverinfo() {
        let response = Response::new("response name", 200, ResponseType::Text, "body");
        let resp_id = response.id;
        let mut route = Route::new("/hello-world", Method::GET);
        route.add_response(response);
        route.set_active_response(resp_id);

        let mut serverinfo = ServerInfo::new();
        serverinfo.server.name = "server name".to_string();
        serverinfo.server.description = "description".to_string();
        serverinfo.router.add_route(route);

        // Serialize to JSON, then deserialize from that to make sure the structure is correct (implicit test)
        let json = serde_json::to_string(&serverinfo);
        assert!(json.is_ok());

        let json = json.ok().unwrap();
        let _serverinfo: ServerInfo =
            serde_json::from_str(&json).expect("Could not deserialize JSON");
    }
}
