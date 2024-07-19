use axum::body::Body;
use axum::response::Html;

pub async fn html_as_body(body: Body) -> Html<Body> {
    Html(body)
}
