use axum::{body::Bytes, response::Html};

pub async fn body_as_bytes(s: Bytes) -> Html<Bytes> {
    Html(s)
}
