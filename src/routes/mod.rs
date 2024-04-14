mod body_as_bytes;
mod body_as_string;
mod custom_body;
mod get_headers;
mod get_html;
mod middleware_msg;
mod path_params;
mod query_params;
mod req;

/* this is the main file of the routes */
use axum::{
    http::Method,
    routing::{get, post},
    Extension, Router,
};
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
pub struct SharedData {
    message: String,
}

pub fn all_routes() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);
    let shared_data = SharedData {
        message: "hello world!".to_string(),
    };

    Router::new()
        .route("/get_html", get(get_html::handler))
        .route("/custom_body", post(custom_body::custom_body))
        .route("/path_params/:user", get(path_params::path_params))
        .route("/query_params", get(query_params::query_params))
        .route("/get_headers", get(get_headers::get_headers))
        .route("/body_as_string", post(body_as_string::body_as_string))
        .route("/body_as_bytes", post(body_as_bytes::body_as_bytes))
        .route("/request", get(req::request))
        .route("/middleware_msg", get(middleware_msg::middleware_msg))
        .layer(cors)
        .layer(Extension(shared_data))
}
