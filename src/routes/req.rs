use axum::extract::{Json, Request};
use serde::Serialize;

#[derive(Serialize)]
pub struct ServRes {
    method: String,
    uri: String,
    // version:
    // headers:
    // body: axum::body::Body,
}

// #[axum::debug_handler]
pub async fn request(req: Request) -> Json<ServRes> {
    println!("{req:#?}");
    Json(ServRes {
        method: req.method().to_string(),
        uri: req.uri().to_string(),
        // body: req.body().to_string(),
    })
}
