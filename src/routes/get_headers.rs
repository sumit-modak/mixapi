use axum::{http::HeaderMap, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct SomeStruct {
    accept: String,
    user_agent: String,
    food: String,
}

pub async fn get_headers(headers: HeaderMap) -> Json<SomeStruct> {
    Json(SomeStruct {
        accept: headers.get("accept").unwrap().to_str().unwrap().to_string(),
        user_agent: headers
            .get("user-agent")
            .unwrap()
            .to_str()
            .unwrap()
            .to_string(),
        food: "bread".to_string(),
    })
}
