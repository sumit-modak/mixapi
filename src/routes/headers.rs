use axum::http::{header, HeaderName};
use axum::{
    extract::{Host, Request},
    http::HeaderMap,
    response::Html,
    Json,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct SomeStruct {
    accept: String,
    user_agent: String,
    food: String,
}

pub async fn extract_headers(headers: HeaderMap) -> Json<SomeStruct> {
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

///////////////////////////////////////////////////////////////

pub async fn set_headers(mut req: Request) -> Html<String> {
    let h = req.headers_mut();
    println!("{req:#?}");
    Html("hello world".to_string())
}

///////////////////////////////////////////////////////////////

pub async fn extract_host(Host(host): Host) -> Html<String> {
    Html(host)
}

///////////////////////////////////////////////////////////////

pub async fn array_headers() -> [(HeaderName, &'static str); 2] {
    [
        (header::SERVER, "axum"),
        (header::CONTENT_TYPE, "text/plain"),
    ]
}
