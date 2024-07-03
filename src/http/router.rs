use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{route::Route, Server};

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
pub struct Router {
    pub id: Uuid,
    #[serde(rename = "serverId")]
    pub server_id: Option<Uuid>,
    pub routes: Vec<Route>,
}

impl Router {
    #[allow(dead_code)]
    pub fn new(server_id: Option<Uuid>) -> Self {
        Router {
            id: Uuid::new_v4(),
            server_id,
            routes: vec![],
        }
    }

    #[allow(dead_code)]
    /// Records the paired server ID with this router, nothing more.
    pub fn bind_server(&mut self, server: &Server) -> &mut Self {
        self.server_id = Some(server.id);
        self
    }

    #[allow(dead_code)]
    /// Adds a new `mockerize_cli::http::Route` to the `Router` instance
    pub fn add_route(&mut self, route: Route) -> &mut Self {
        self.routes.push(route);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::uuid;

    #[test]
    fn can_deserialize_router() {
        let json = r###"
        {
            "id": "38fcf7b9-c409-4941-9520-a913ae8e205f",
            "serverId": "9c2e8c77-7c90-4d8c-8154-0ea25d3723c4",
            "routes": [
                {
                    "id": "a27d777e-0321-44e3-a233-f4271cf7e05c",
                    "path": "/hello-world",
                    "responses": [
                        {
                            "id": "8cd81b4a-046a-47e9-87ec-75a01b850a64",
                            "name": "Show Hello World",
                            "status": 200,
                            "response": "Hello World",
                            "responseType": "text",
                            "active": true,
                            "headers": []
                        }
                    ],
                    "activeResponse": "8cd81b4a-046a-47e9-87ec-75a01b850a64",
                    "method": "GET",
                    "headers": []
                },
                {
                    "id": "9f0ed395-3acc-46bb-9b8d-df8fd62f593f",
                    "path": "/healthz",
                    "responses": [
                        {
                            "id": "3af4f7c7-79b7-4c3c-8495-a899e6ed3a3f",
                            "name": "Health check OK",
                            "status": 204,
                            "response": "{\"status\": \"OK\"}",
                            "responseType": "json",
                            "active": true,
                            "headers": []
                        }
                    ],
                    "activeResponse": "3af4f7c7-79b7-4c3c-8495-a899e6ed3a3f",
                    "method": "GET",
                    "headers": []
                }
            ]
        }
        "###;

        let router: Router = serde_json::from_str(json).expect("Unable to parse JSON.");
        assert_eq!(router.id, uuid!("38fcf7b9-c409-4941-9520-a913ae8e205f"));
        assert_eq!(
            router.server_id,
            Some(uuid!("9c2e8c77-7c90-4d8c-8154-0ea25d3723c4"))
        );
        assert_eq!(router.routes.len(), 2);
    }
}
