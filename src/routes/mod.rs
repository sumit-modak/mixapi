mod body_as_bytes;
mod body_as_string;
mod get_html;
mod headers;
mod html_as_body;
mod json_as_body;
mod middleware;
mod params;

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
        .route("/get_html", get(get_html::handler))
        // uri
        .route("/path_params/:user", get(params::path_params))
        .route("/path_params/:p1/:p2", get(params::path_params2))
        .route("/query_params", get(params::query_params))
        // headers
        .route("/extract_headers", get(headers::extract_headers))
        .route("/set_headers", get(headers::set_headers))
        // sending different body types
        .route("/body_as_string", post(body_as_string::body_as_string))
        .route("/body_as_bytes", post(body_as_bytes::body_as_bytes))
        .route("/html_as_body", post(html_as_body::html_as_body))
        .route("/json_as_body", post(json_as_body::json_as_body))
        // middleware routes
        .route("/middleware_msg", get(middleware::middleware_msg))
        .route("/read_mw_custom_hdr", get(middleware::read_mw_custom_hdr))
        // middleware
        .layer(Extension(SharedData {
            message: "this message is shared with all routes".to_string(),
        }))
        .layer(cors)
}
