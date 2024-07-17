use axum::extract::{Json, Request};
use serde::Serialize;

#[derive(Serialize)]
pub struct MyJson {
    method: String,
    uri: String,
    // version:
    // headers:
    // body: axum::body::Body,
}

// #[axum::debug_handler]
pub async fn body_as_json(req: Request) -> Json<MyJson> {
    println!("{req:#?}");
    Json(MyJson {
        method: req.method().to_string(),
        uri: req.uri().to_string(),
        // body: req.body().to_string(),
    })
}
