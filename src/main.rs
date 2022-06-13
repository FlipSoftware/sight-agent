#![warn(clippy::all)]
use serde::{Deserialize, Serialize};
use std::{
    io::{Error, ErrorKind},
    str::FromStr,
};

use warp::{http::Method, reject::Reject, Filter, Rejection, Reply};

#[tokio::main]
async fn main() {
    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::GET, Method::POST, Method::PUT, Method::DELETE]);

    let kb_data = warp::get()
        .and(warp::path("kb"))
        .and(warp::path::end())
        .and_then(get_kb)
        .recover(handle_error);

    println!("Running server on port: 8080");
    warp::serve(kb_data.with(cors))
        .run(([127, 0, 0, 1], 8080))
        .await;
}

#[derive(Debug, Serialize, Deserialize)]
struct KnowledgeBase {
    id: KBId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct KBId(String);

impl KnowledgeBase {
    fn new(id: KBId, title: String, content: String, tags: Option<Vec<String>>) -> Self {
        Self {
            id,
            title,
            content,
            tags,
        }
    }
}

impl FromStr for KBId {
    type Err = Error;

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() {
            true => Err(Error::new(ErrorKind::InvalidInput, "ID must be provided")),
            false => Ok(KBId(id.to_string())),
        }
    }
}

#[derive(Debug)]
struct InvalidId;
impl Reject for InvalidId {}

async fn get_kb() -> Result<impl warp::Reply, warp::Rejection> {
    let kb = KnowledgeBase::new(
        KBId::from_str("1").expect("Invalid ID"),
        "Title".to_string(),
        "Content".to_string(),
        Some(vec!["tag1".to_string(), "tag2".to_string()]),
    );

    match kb.id.0.parse::<i32>() {
        Ok(_) => Ok(warp::reply::json(&kb)),
        Err(_) => Err(warp::reject::custom(InvalidId)),
    }
}

async fn handle_error(error: Rejection) -> Result<impl Reply, Rejection> {
    println!("{error:?}");
    if let Some(e) = error.find::<warp::filters::cors::CorsForbidden>() {
        Ok(warp::reply::with_status(
            e.to_string(),
            warp::http::StatusCode::FORBIDDEN,
        ))
    } else {
        match error.find() {
            Some(InvalidId) => Ok(warp::reply::with_status(
                "Invalid ID".to_string(),
                warp::http::StatusCode::UNPROCESSABLE_ENTITY,
            )),
            None => Ok(warp::reply::with_status(
                "Inacessible route".to_string(),
                warp::http::StatusCode::NOT_FOUND,
            )),
        }
    }
}
