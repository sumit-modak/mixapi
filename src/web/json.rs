use axum::extract::{Json, Request};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MyJson {
    method: String,
    uri: String,
    // version:
    // headers:
    // body: axum::body::Body,
}

// #[axum::debug_handler]
pub async fn send_json(req: Request) -> Json<MyJson> {
    println!("{req:#?}");
    Json(MyJson {
        method: req.method().to_string(),
        uri: req.uri().to_string(),
        // body: req.body().to_string(),
    })
}

pub async fn extract_json(Json(mut j): Json<MyJson>) -> Json<MyJson> {
    j.method = "CUSTOM".to_string();
    j.uri = "/extract/json".to_string();
    j.into()
}
