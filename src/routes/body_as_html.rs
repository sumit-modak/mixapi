use axum::body::Body;
use axum::response::Html;

pub async fn body_as_html(body: Body) -> Html<Body> {
    Html(body)
}
