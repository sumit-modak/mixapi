use axum::{
    Json,
    extract::Request,
    http::{HeaderMap, HeaderName, StatusCode, header},
    response::{AppendHeaders, Html, IntoResponse},
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

pub async fn set_headers(mut req: Request) -> (StatusCode, HeaderMap, Html<String>) {
    let mut h = req.headers_mut().to_owned();
    println!("{h:?}");
    h.append("foo", "bar".parse().unwrap());

    (StatusCode::OK, h, Html("hello world".to_string()))
}

///////////////////////////////////////////////////////////////

pub async fn array_headers() -> [(HeaderName, &'static str); 2] {
    [
        (header::SERVER, "axum"),
        (header::CONTENT_TYPE, "text/plain"),
    ]
}

///////////////////////////////////////////////////////////////

pub async fn append_headers() -> impl IntoResponse {
    AppendHeaders([
        (header::SET_COOKIE, "foo=bar"),
        (header::SET_COOKIE, "baz=qux"),
    ])
}
