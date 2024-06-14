use std::net::TcpListener;

use mockerize_cli::http::{
    Header, Method, Response, ResponseType, Route, Router, Server, ServerInfo,
};

pub struct TestApp {
    pub address: String,
}

fn make_serverinfo() -> ServerInfo {
    let mut router = Router::new(None);
    let server = Server::new(router.id, "127.0.0.1", 0).unwrap(); // addr here is disregarded
    router.bind_server(&server);

    ServerInfo { server, router }
}

// Launch our application in the background
async fn spawn_app(serverinfo: ServerInfo) -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let server =
        mockerize_cli::startup::run(serverinfo, listener, None).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    TestApp { address: address }
}

#[tokio::test]
async fn startup_run_instantiates_a_working_http_server() {
    let serverinfo = make_serverinfo();
    let app = spawn_app(serverinfo).await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/not-exists", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_client_error());
    assert_eq!(response.status().as_u16(), 404);
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn server_is_able_to_serve_a_registered_route() {
    let mut serverinfo = make_serverinfo();

    // Set up  dummy routes and responses
    let response1 = Response::new("", 200, ResponseType::Text, "Route1");
    let resp1_id = response1.id;

    let mut route = Route::new("/route1", Method::GET);
    route.add_response(response1);
    route.set_active_response(resp1_id);
    serverinfo.router.add_route(route);

    let response2 = Response::new("", 200, ResponseType::Text, "Route2");
    let resp2_id = response2.id;

    let mut route = Route::new("/route2", Method::GET);
    route.add_response(response2);
    route.set_active_response(resp2_id);
    serverinfo.router.add_route(route);

    // Spin up actual server and test hitting our created route
    let app = spawn_app(serverinfo).await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/route1", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(response.status().as_u16(), 200);

    let body = response.text().await;
    assert_eq!("Route1", body.unwrap());

    // Retry against route2, to verify that routing is working correctly
    let response = client
        .get(&format!("{}/route2", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(response.status().as_u16(), 200);

    let body = response.text().await;
    assert_eq!("Route2", body.unwrap());
}

#[tokio::test]
async fn server_responses_include_cascaded_headers() {
    let mut serverinfo = make_serverinfo();
    serverinfo
        .server
        .add_header(Header::new("server-level-key", "server-level-value"));
    serverinfo
        .server
        .add_header(Header::new("override-by-route-key", "server-level-value")); // will be overriden later
    serverinfo.server.add_header(Header::new(
        "override-by-response-key",
        "server-level-value",
    )); // will be overriden later

    // Set up  dummy routes and responses
    let mut response = Response::new("", 200, ResponseType::Json, "{}");
    response.add_header(Header::new("response-level-key", "response-level-value"));
    response.add_header(Header::new(
        "override-by-response-key",
        "response-level-value",
    ));

    let mut route = Route::new("/test", Method::GET);
    route.add_response(response);
    route.add_header(Header::new("route-level-key", "route-level-value"));
    route.add_header(Header::new("override-by-route-key", "route-level-value"));
    route.add_header(Header::new("override-by-response-key", "route-level-value")); // will be overriden later
    serverinfo.router.add_route(route);

    // Spin up actual server and test hitting our created route
    let app = spawn_app(serverinfo).await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/test", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());

    let resp_headers = response.headers();
    println!("Headers: {:#?}", resp_headers);

    assert_eq!(
        resp_headers.get("server-level-key").unwrap(),
        "server-level-value"
    );
    assert_eq!(
        resp_headers.get("route-level-key").unwrap(),
        "route-level-value"
    );
    assert_eq!(
        resp_headers.get("override-by-route-key").unwrap(),
        "route-level-value"
    );
    assert_eq!(
        resp_headers.get("response-level-key").unwrap(),
        "response-level-value"
    );
}
