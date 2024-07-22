mod bytes;
mod headers;
mod html;
mod into_resp;
mod json;
mod middleware;
mod params;
mod string;

///////////////////////////////////////////////////////////////

use axum::{
    http::Method,
    routing::{get, post},
    Extension, Router,
};
use tower_http::cors::{Any, CorsLayer};

///////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct SharedData {
    message: String,
}

pub fn all_routes() -> Router {
    let cors = CorsLayer::new()
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
        .route("/types_of_path/:hello", get(params::types_of_path))
        // headers
        .route("/extract_headers", get(headers::extract_headers))
        .route("/set_headers", get(headers::set_headers))
        .route("/extract_host", get(headers::extract_host))
        .route("/array_headers", get(headers::array_headers))
        // into response
        .route("/response/tuple", get(into_resp::all_the_things))
        .route("/response/builder", get(into_resp::new_response))
        // middleware routes
        .route("/middleware_msg", get(middleware::middleware_msg))
        .route("/read_mw_custom_hdr", get(middleware::read_mw_custom_hdr))
        // middleware
        .layer(Extension(SharedData {
            message: "this message is shared with all routes".to_string(),
        }))
        .layer(cors)
}
