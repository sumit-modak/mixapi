mod body_as_bytes;
mod body_as_html;
mod body_as_json;
mod body_as_string;
mod extract_headers;
mod get_html;
mod middleware_msg;
mod path_params;
mod query_params;
mod read_mw_custom_hdr;

/////////////////////////////////////////
use axum::{
    http::Method,
    routing::{get, post},
    Extension, Router,
};
use tower_http::cors::{Any, CorsLayer};

/* this is the main file of the routes */
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
        .route("/path_params/:user", get(path_params::path_params))
        .route("/query_params", get(query_params::query_params))
        // headers
        .route("/extract_headers", get(extract_headers::extract_headers))
        // sending different body types
        .route("/body_as_string", post(body_as_string::body_as_string))
        .route("/body_as_bytes", post(body_as_bytes::body_as_bytes))
        .route("/body_as_html", post(body_as_html::body_as_html))
        .route("/body_as_json", post(body_as_json::body_as_json))
        // middleware routes
        .route("/middleware_msg", get(middleware_msg::middleware_msg))
        .route("/read_mw_custom_hdr", get(read_mw_custom_hdr::handler))
        // middleware
        .layer(Extension(SharedData {
            message: "this message is shared with all routes".to_string(),
        }))
        .layer(cors)
}
