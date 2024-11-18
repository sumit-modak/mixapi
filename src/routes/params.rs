use std::collections::HashMap;

use axum::{
    body::Body,
    extract::{MatchedPath, NestedPath, Path, Query},
    http::{Response, StatusCode},
    response::{Html, Json},
};
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

#[derive(Deserialize)]
pub struct ServerReq2(String, String);

#[derive(Serialize)]
pub struct ServerRes2(String, String, i64);

pub async fn query_params2(Query(q): Query<ServerReq2>) -> Json<ServerRes2> {
    Json(ServerRes2(q.0, q.1, 293092838478))
}

///////////////////////////////////////////////////////////////

pub async fn query_params3(Query(q): Query<HashMap<String, String>>) -> Response<Body> {
    println!("{q:?}");
    Response::builder()
        .status(StatusCode::OK)
        .header("X", "")
        .body(Body::from(""))
        .unwrap()
}

///////////////////////////////////////////////////////////////

pub async fn path_params(Path(path): Path<usize>) -> Html<String> {
    Html(format!("Square: <h4>{}</h4>", path * path))
}

///////////////////////////////////////////////////////////////

pub async fn path_params2(Path((p1, p2)): Path<(usize, usize)>) -> Html<String> {
    Html(format!("Product: <h4>{}</h4>", p1 * p2))
}

///////////////////////////////////////////////////////////////

pub async fn types_of_path(mp: MatchedPath) -> Html<String> {
    Html(format!("MatchedPath: {mp:?}"))
}

///////////////////////////////////////////////////////////////

// pub async fn types_of_path(np: NestedPath) -> Html<String> {
//     Html(format!("NestedPath: {np:?}"))
// }
