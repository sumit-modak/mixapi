use axum::body::Body;
use axum::response::Html;

pub async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

pub async fn send_html(body: Body) -> Html<Body> {
    Html(body)
}
