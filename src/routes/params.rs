use axum::extract::{Path, Query};
use axum::response::{Html, Json};
use serde::{Deserialize, Serialize};

///////////////////////////////////////////////////////////////

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

///////////////////////////////////////////////////////////////

pub async fn path_params(Path(path): Path<usize>) -> Html<String> {
    Html(format!("Square: <h4>{}</h4>", path * path))
}

///////////////////////////////////////////////////////////////

pub async fn path_params2(Path((p1, p2)): Path<(usize, usize)>) -> Html<String> {
    Html(format!("Product: <h4>{}</h4>", p1 * p2))
}
