use axum::body::Body;
use axum::response::Html;

pub async fn custom_body(body: Body) -> Html<Body> {
    Html(body)
}
