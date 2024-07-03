use actix_web::dev::Server;
use actix_web::http::StatusCode;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use std::collections::HashMap;
use std::net::TcpListener;
use std::sync::Arc;
use tracing::{debug, info, info_span, Instrument};
use uuid::Uuid;

use crate::http::{Header, Route, ServerInfo};

/// Starts a new Actix server to listen and begin accepting HTTP traffic on the
/// given `TcpListener`. Responses to those HTTP requests will be dicated by
/// the given `ServerInfo`. By default, Actix will start one worker per CPU
/// core, however, this may be overridden by specifying a count for `workers`.
pub fn run(
    serverinfo: ServerInfo,
    listener: TcpListener,
    workers: Option<usize>,
) -> Result<Server, std::io::Error> {
    for route in &serverinfo.router.routes {
        info!("Registering route {} {}", &route.method, &route.path);
    }

    // We wrap serverinfo in an Arc, otherwise we end up with issues cloning its components down the line
    // as each Actix worker will need access
    let serverinfo = Arc::new(serverinfo);

    let server = HttpServer::new(move || {
        let mut app = App::new().wrap(Logger::default());

        for route in &serverinfo.router.routes {
            let path = transform_route_path(&route.path);
            let server_headers = Arc::new(serverinfo.server.headers.clone());
            let route_handler = make_route_handler(server_headers, route);

            if let Some(handler) = route_handler {
                app = app.route(&path, handler);
            }
        }

        app
    });

    let server = if let Some(workers) = workers {
        server.workers(workers)
    } else {
        server
    };

    let server = server.listen(listener)?.run();
    Ok(server)
}

/*
Merge server and route-level headers into a single vector.
Any similar keys between the two will let the route-level override.
*/
pub fn merge_headers(
    server_headers: &[Header],
    route_headers: &[Header],
    response_headers: &[Header],
) -> Vec<Header> {
    let mut map: HashMap<String, Header> = HashMap::new();

    for header in server_headers {
        map.insert(header.key.clone(), header.clone());
    }

    // Layer route-level headers over top, overwriting any similar key
    for header in route_headers {
        map.insert(header.key.clone(), header.clone());
    }

    // Layer response-level headers over top, overwriting any similar key
    for header in response_headers {
        map.insert(header.key.clone(), header.clone());
    }

    // Collect the values of the hashmap into a vector
    map.into_values().collect()
}

// Convert our Route into an Actix-web route handler, which can be bound to an Actix Web App
fn make_route_handler(server_headers: Arc<Vec<Header>>, route: &Route) -> Option<actix_web::Route> {
    let response = route.get_active_response()?;
    let body = Arc::new(response.get_response_body());
    let status = response.status;
    let resp_id = response.id;
    let method = route.method.clone();
    let path = route.path.clone();

    let route_headers = Arc::new(route.headers.clone());
    let response_headers = Arc::new(response.headers.clone());

    let handler = move |_req: HttpRequest| {
        // I'm sure this double-clone isn't the most efficient thing in the world, but...
        // I don't know a better alternative to prevent borrowed data escaping function.
        // An Arc on Route won't work, because it would still need to outlive a static lifetime.
        // TODO: Figure this out.
        let body = body.clone();
        let server_headers = Arc::clone(&server_headers);
        let route_headers = Arc::clone(&route_headers);
        let response_headers = Arc::clone(&response_headers);
        let headers = merge_headers(&server_headers, &route_headers, &response_headers.clone());

        let request_id = Uuid::new_v4();
        let request_span = info_span!(
            "Client requested mock endpoint",
            %request_id,
            method = %method,
            path = %path,
            response_id = %resp_id
        );

        let _request_span_guard = request_span.enter();
        let handler_span = info_span!("Handling response");

        async move {
            let mut resp = HttpResponse::build(StatusCode::from_u16(status).unwrap());
            let body_len = body.len();
            debug!("Responding with status code {status}, body {body_len} bytes");

            for header in headers {
                resp.append_header((header.key.clone(), header.value.clone()));
            }
            resp.body(body.to_string())
        }
        .instrument(handler_span)
    };

    let method: actix_web::http::Method = route.method.clone().into();
    let route_handler = web::route().method(method).to(handler);

    Some(route_handler)
}

/*
Takes a route path, transforms substitution bindings from Mockerize format to Actix-Web, returns the result.
Any non-alphanumeric characters within a substitution will be converted to underscore (_).
Multiple non-alphanumerics in a row will be condensed down to a single underscore.
If the substutition begins with a non-alphanumeric, the first instance will be ignored.

For example, `/api/v1/users/:user-id` would be transformed to `/api/v1/users/{user_id}`
*/
fn transform_route_path(input: &str) -> String {
    input
        .split('/')
        .map(|s| {
            if s.starts_with(':') {
                let mut result = "{".to_string();
                let mut prev_char_alphanumeric = true;

                // Iterate over each character in the string, collapsing non-alphanumerics into a single _
                for (i, c) in s.chars().skip(1).enumerate() {
                    if c.is_ascii_alphanumeric() {
                        if !prev_char_alphanumeric {
                            result.push('_');
                        }
                        result.push(c.to_ascii_lowercase());
                        prev_char_alphanumeric = true;
                    } else {
                        // Ignore first character being non-alpha
                        prev_char_alphanumeric = i == 0;
                    }
                }
                result.push('}');
                result
            } else {
                s.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("/")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http::{Method, Response, ResponseType};

    #[test]
    fn test_transform_route_path_handles_substitutions_correctly() {
        let result = transform_route_path("/apple/banana/:substitute/date");
        assert_eq!(result, "/apple/banana/{substitute}/date");

        let result = transform_route_path(":substitute-with-non__alpha.numerics");
        assert_eq!(result, "{substitute_with_non_alpha_numerics}");

        let result = transform_route_path("/doesn't/modify+the/normal/segments#at all");
        assert_eq!(result, "/doesn't/modify+the/normal/segments#at all");

        let result = transform_route_path(":-begin-with-non-alpha");
        assert_eq!(result, "{begin_with_non_alpha}");
    }

    #[test]
    fn test_make_route_handler() {
        let response = Response::new("name", 200, ResponseType::Text, "body");
        let id = response.id.clone();

        let mut route = Route::new("/index", Method::GET);
        route.add_response(response);
        route.set_active_response(id);

        let server_headers = Arc::new(vec![]);
        let handler = make_route_handler(server_headers, &route);
        assert!(handler.is_some());
    }
}
