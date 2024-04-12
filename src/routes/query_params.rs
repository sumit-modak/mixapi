use axum::extract::Query;
use axum::response::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ServerReq {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct ServerRes {
    username: String,
    password: String,
    cookie: usize,
}

pub async fn query_params(Query(q): Query<ServerReq>) -> Json<ServerRes> {
    Json(ServerRes {
        username: q.username,
        password: q.password,
        cookie: 293092838478,
    })
}
