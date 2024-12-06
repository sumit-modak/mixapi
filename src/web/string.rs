use axum::response::Html;

pub async fn body_as_string(s: String) -> Html<String> {
    Html(format!("You sent: <h1>{s}</h1>"))
}
