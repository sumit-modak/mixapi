use crate::routes::SharedData;
use axum::Extension;

// shared data is set on layer
pub async fn middleware_msg(Extension(shared_data): Extension<SharedData>) -> String {
    shared_data.message
}

///////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct HeaderMessage(pub String);

// the header is passed in request
pub async fn read_mw_custom_hdr(Extension(msg): Extension<HeaderMessage>) -> String {
    msg.0
}

///////////////////////////////////////////////////////////////

pub async fn set_mw_custom_hdr() {}
