mod custom_body;
mod get_headers;
mod get_html;
mod path_params;
mod query_params;

/* this is the main file of the routes */
use axum::{
    routing::{get, post},
    Router,
};

pub fn all_routes() -> Router {
    Router::new()
        .route("/get_html", get(get_html::handler))
        .route("/custom_body", post(custom_body::custom_body))
        .route("/path_params/:user", get(path_params::path_params))
        .route("/query_params", get(query_params::query_params))
        .route("/get_headers", get(get_headers::get_headers))
}
