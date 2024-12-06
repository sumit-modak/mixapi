use axum::{
    body::Body,
    http::{header, HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    Extension,
};

#[derive(Clone)]
#[allow(dead_code)]
struct Foo(&'static str);

pub async fn all_the_things() -> impl IntoResponse {
    let mut header_map = HeaderMap::new();
    header_map.insert(header::SERVER, "axum".parse().unwrap());
    header_map.insert(header::CONNECTION, "close".parse().unwrap());

    (
        // set status code
        StatusCode::NOT_FOUND,
        // headers with an array
        [("x-custom", "custom")],
        // some extensions
        Extension(Foo("foo")),
        Extension(Foo("bar")),
        // more headers, built dynamically
        header_map,
        // and finally the body
        "foo",
    )
}

pub async fn new_response() -> Response {
    Response::builder()
        .status(StatusCode::OK)
        .header("x-foo", "custom header")
        .body(Body::from("custom body"))
        .unwrap()
}

pub async fn always_error() -> Result<(), StatusCode> {
    Err(StatusCode::IM_A_TEAPOT)
}

pub async fn body_n_status() -> Response {
    (StatusCode::CREATED, "This is a very shit route").into_response()
}
