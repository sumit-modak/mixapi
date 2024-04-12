use axum::extract::Path;
use axum::response::Html;

pub async fn path_params(Path(path): Path<usize>) -> Html<String> {
    Html(format!("Square: <h4>{}</h4>", path * path))
}
