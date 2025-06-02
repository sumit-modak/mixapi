mod bytes;
mod headers;
mod html;
mod into_resp;
mod json;
mod middleware;
mod params;
mod string;
mod websock;

///////////////////////////////////////////////////////////////

use axum::{
    Extension, Router,
    http::Method,
    routing::{get, post},
};
use tower_http::cors::{Any, CorsLayer};

///////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct SharedData {
    message: String,
}

pub fn all_routes() -> Router {
    let _cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    Router::new()
        // sending different body types
        .route("/string", get(string::body_as_string))
        .route("/bytes", get(bytes::body_as_bytes))
        .route("/send/html", get(html::send_html))
        .route("/get_html", get(html::handler))
        .route("/send/json", get(json::send_json))
        .route("/extract/json", post(json::extract_json))
        // uri
        .route("/path_params/:user", get(params::path_params))
        .route("/path_params/:p1/:p2", get(params::path_params2))
        .route("/query_params", get(params::query_params))
        .route("/query_params2", get(params::query_params2))
        .route("/query_params3", get(params::query_params3))
        .route("/types_of_path/:hello", get(params::types_of_path))
        // headers
        .route("/extract_headers", get(headers::extract_headers))
        .route("/set_headers", get(headers::set_headers))
        .route("/array_headers", get(headers::array_headers))
        .route("/append_headers", get(headers::append_headers))
        // into response
        .route("/response/tuple", get(into_resp::all_the_things))
        .route("/response/builder", get(into_resp::new_response))
        .route("/response/error", get(into_resp::always_error))
        .route("/response/shit", get(into_resp::body_n_status))
        // middleware routes
        .route("/middleware_msg", get(middleware::middleware_msg))
        .route("/read_mw_custom_hdr", get(middleware::read_mw_custom_hdr))
        // websockets
        .route("/websock", get(websock::handler))
        .route("/websock2", get(websock::ws_handler))
        // middleware
        .layer(Extension(SharedData {
            message: "this message is shared with all routes".to_string(),
        }))
    // .layer(cors)
}

pub async fn main(_args: crate::args::WebArgs, _cfg: &mut crate::AppConfig) {
    let app = all_routes();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("[-] listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
