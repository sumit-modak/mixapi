use axum::Extension;

#[derive(Clone)]
pub struct HeaderMessage(String);

pub async fn handler(Extension(msg): Extension<HeaderMessage>) -> String {
    msg.0
}
