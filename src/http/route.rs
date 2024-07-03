use serde::de::Deserializer;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use super::Header;
use super::Method;
use super::Response;

#[derive(Debug, Serialize)]
pub struct Route {
    pub id: Uuid,
    pub path: String,
    pub method: Method,
    pub headers: Vec<Header>,
    pub responses: Vec<Response>,

    #[serde(rename = "activeResponse")] // rename handled manually in deserialize() below
    active_response: Option<Uuid>,
    #[serde(skip_serializing)]
    active_response_index: Option<usize>, // Store the index of the active response so we don't need to look it up every time
}

impl Route {
    #[allow(unused)]
    pub fn new(path: &str, method: Method) -> Self {
        Route {
            id: Uuid::new_v4(),
            path: String::from(path),
            active_response: None,
            method,
            headers: vec![],
            responses: vec![],
            active_response_index: None,
        }
    }

    /**
    Get the active response for this route if there is one.

    If a response has not been specifically assigned,
    and there are responses registered to this route,
    then this will return the first found response.
    **/
    pub fn get_active_response(&self) -> Option<&Response> {
        if self.active_response_index.is_none() && !self.responses.is_empty() {
            return self.responses.first();
        }

        self.active_response_index
            .and_then(|idx| self.responses.get(idx))
    }

    /**
    Updates the internal references to the wanted active response, such that
    subsequent calls to `get_active_response()` would return that response.
    This affects private vars: `active_response`, `active_response_index`
    **/
    pub fn set_active_response(&mut self, id: Uuid) {
        self.active_response_index = self.responses.iter().position(|r| {
            if r.id == id {
                self.active_response = Some(r.id);
                true
            } else {
                false
            }
        });
    }

    /**
    Adds an additional response to the valid responses list. Does not
    update the active response.
    **/
    #[allow(unused)]
    pub fn add_response(&mut self, response: Response) {
        self.responses.push(response);
    }

    #[allow(unused)]
    pub fn add_header(&mut self, header: Header) {
        self.headers.push(header);
    }
}

impl<'de> Deserialize<'de> for Route {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        struct RouteHelper {
            id: Uuid,
            path: String,
            #[serde(rename = "activeResponse")]
            active_response: Option<Uuid>,
            method: Method,
            headers: Vec<Header>,
            responses: Vec<Response>,
        }

        let route_helper = RouteHelper::deserialize(deserializer)?;

        let mut route = Route {
            id: route_helper.id,
            path: route_helper.path,
            active_response: route_helper.active_response,
            method: route_helper.method,
            headers: route_helper.headers,
            responses: route_helper.responses,
            active_response_index: None,
        };

        // Call set_active_response only if active_response is Some
        if let Some(active_response_id) = route.active_response {
            route.set_active_response(active_response_id);
        }

        Ok(route)
    }
}

#[cfg(test)]
mod tests {
    use crate::http::ResponseType;

    use super::*;
    use uuid::uuid;

    #[test]
    fn can_deserialize_route() {
        let json = r###"
        {
            "id": "bba078b1-742f-43c1-aded-5ad665decaa0",
                "path": "/index",
                "responses": [
                    {
                        "id": "df5d9688-9af9-4377-ba09-72056ea2570c",
                        "name": "Not Found",
                        "status": 404,
                        "response": "<html><body><h1>404 - Page Not Found</h1>Could not locate the requested resource.</body></html>",
                        "responseType": "text",
                        "active": false,
                        "headers": []
                    },
                    {
                        "id": "df5d9688-9af9-4377-ba09-72056ea2570c",
                        "name": "Index",
                        "status": 200,
                        "response": "<html><body><h1>Hello, World!</h1>Example only.</body></html>",
                        "responseType": "text",
                        "active": true,
                        "headers": []
                    },
                    {
                        "id": "13163edf-2650-4533-baad-64e986621781",
                        "name": "Index - as JSON",
                        "status": 200,
                        "response": "{\"status\": 200, \"title\": \"Hello, World\", \"content\": \"Example only.\"}",
                        "responseType": "json",
                        "active": false,
                        "headers": []
                    }
                ],
                "activeResponse": "df5d9688-9af9-4377-ba09-72056ea2570c",
                "method": "GET",
                "headers": []
        }
        "###;

        let route: Route = serde_json::from_str(json).expect("Unable to parse JSON.");
        assert_eq!(route.id, uuid!("bba078b1-742f-43c1-aded-5ad665decaa0"));
        assert_eq!(route.path, "/index");
        assert_eq!(route.responses.len(), 3);
        assert_eq!(route.method, Method::GET);

        // After deserialization, the correct active response index should have been set
        // so `get_active_response()` should return the actual Response referenced as `active_response`
        assert_eq!(
            route.active_response.unwrap(),
            uuid!("df5d9688-9af9-4377-ba09-72056ea2570c")
        );
        assert_eq!(
            route.get_active_response().unwrap().id,
            uuid!("df5d9688-9af9-4377-ba09-72056ea2570c")
        );
    }

    #[test]
    fn can_default_active_response() {
        let response1 = Response::new("", 200, ResponseType::Text, "hello");
        let resp1_id = response1.id;
        let response2 = Response::new("", 204, ResponseType::Text, "");
        let resp2_id = response2.id;
        let mut route = Route::new("/text", Method::GET);
        route.add_response(response1);
        route.add_response(response2);

        let active_response = route.get_active_response();
        assert!(active_response.is_some());
        assert_eq!(active_response.unwrap().id, resp1_id);

        route.set_active_response(resp2_id);
        let new_active_response = route.get_active_response();
        assert_eq!(new_active_response.unwrap().id, resp2_id);
    }

    #[test]
    fn set_active_response_updates_all_internal_references() {
        let mut route = Route::new("/", Method::GET);

        assert_eq!(route.active_response, None);
        assert!(route.get_active_response().is_none());

        let resp = Response::new("", 200, ResponseType::Text, "");
        let resp_id = resp.id;
        route.add_response(resp);
        route.set_active_response(resp_id);

        assert_eq!(route.active_response, Some(resp_id));
        let returned_resp = route.get_active_response();
        assert!(returned_resp.is_some());
        let returned_resp = returned_resp.unwrap();
        assert_eq!(returned_resp.id, resp_id);
    }
}
