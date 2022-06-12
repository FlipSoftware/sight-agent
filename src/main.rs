use std::{
    io::{Error, ErrorKind},
    str::FromStr,
};
use warp::Filter;

#[tokio::main]
async fn main() {
    let q = KnowledgeBase::new(
        KBId::from_str("1").expect("ID must be provided"),
        "First".to_string(),
        "Content".to_string(),
        Some(vec!["FAQ".to_string()]),
    );
    println!("{q:?}");

    // Init Server
    let routes = warp::path("api")
        .and(warp::path::end())
        .map(|| warp::reply::json(&"Hello, Rust!".to_string()));

    // Start Server
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

#[derive(Debug)]
struct KnowledgeBase {
    id: KBId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

#[derive(Debug)]
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
