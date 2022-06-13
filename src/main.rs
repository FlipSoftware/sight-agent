#![warn(clippy::all)]
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};

use warp::{http::Method, reject::Reject, Filter, Rejection, Reply};

#[tokio::main]
async fn main() {
    let db = Database::new();
    let db_access = warp::any().map(move || db.clone());

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::GET, Method::POST, Method::PUT, Method::DELETE]);

    let kb_data = warp::get()
        .and(warp::path("kb"))
        .and(warp::path::end())
        .and(warp::query())
        .and(db_access.clone())
        .and_then(get_kb);

    let add_kb = warp::post()
        .and(warp::path("kb"))
        .and(warp::path::end())
        .and(db_access.clone())
        .and(warp::body::json())
        .and_then(add_kb);

    let update_kb = warp::put()
        .and(warp::path("kb"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(db_access.clone())
        .and(warp::body::json())
        .and_then(update_kb);

    let delete_kb = warp::put()
        .and(warp::path("kb"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(db_access.clone())
        .and_then(delete_kb);

    let add_info_to_kb = warp::post()
        .and(warp::path("info"))
        .and(warp::path::end())
        .and(db_access.clone())
        .and(warp::body::form())
        .and_then(add_info_to_kb);

    // TODO: add :id route

    let router = kb_data
        .or(add_kb)
        .or(update_kb)
        .or(delete_kb)
        .or(add_info_to_kb)
        .with(cors)
        .recover(handle_error);
    println!("Running server on port: 8080");
    warp::serve(router).run(([127, 0, 0, 1], 8080)).await;
}

// SECTION: TYPES

#[derive(Debug, Serialize, Deserialize, Clone)]
struct KnowledgeBase {
    id: KBId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
struct KBId(String);

#[derive(Clone)]
struct Database {
    kb: Arc<RwLock<HashMap<KBId, KnowledgeBase>>>,
    kb_answer: Arc<RwLock<HashMap<InfoID, Info>>>,
}

impl Database {
    fn new() -> Self {
        Database {
            kb: Arc::new(RwLock::new(Self::init())),
            kb_answer: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn init() -> HashMap<KBId, KnowledgeBase> {
        let file = include_str!("../db.json");
        serde_json::from_str(file).expect("Error reading the Database.")
    }
}

#[derive(Debug)]
enum Error {
    ParseError(std::num::ParseIntError),
    ParamsAbsent,
    KbAbsent,
}
impl Reject for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::ParseError(ref err) => write!(f, "Invalid parameter: {}", err),
            Error::ParamsAbsent => write!(f, "Missing required parameter"),
            Error::KbAbsent => write!(f, "KB not found in the database"),
        }
    }
}

#[derive(Debug)]
struct Pagination {
    start: usize,
    end: usize,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
struct InfoID(String);

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Info {
    id: InfoID,
    content: String,
    kb_id: KBId, // TODO: change to uuid
}

// SECTION: FUNCTIONS

async fn get_kb(
    params: HashMap<String, String>,
    kb_db: Database,
) -> Result<impl warp::Reply, warp::Rejection> {
    println!("{params:?}");
    if !params.is_empty() {
        let page = get_pagination(params)?;
        let kb_response: Vec<KnowledgeBase> = kb_db.kb.read().values().cloned().collect();
        let kb_response = &kb_response[page.start..page.end];
        Ok(warp::reply::json(&kb_response))
    } else {
        let kb_response: Vec<KnowledgeBase> = kb_db.kb.read().values().cloned().collect();
        Ok(warp::reply::json(&kb_response))
    }
}

// TODO: check for possible missing errors
async fn handle_error(error: Rejection) -> Result<impl Reply, Rejection> {
    println!("{error:?}");
    if let Some(e) = error.find::<Error>() {
        Ok(warp::reply::with_status(
            e.to_string(),
            warp::http::StatusCode::RANGE_NOT_SATISFIABLE,
        ))
    } else if let Some(e) = error.find::<warp::filters::cors::CorsForbidden>() {
        Ok(warp::reply::with_status(
            e.to_string(),
            warp::http::StatusCode::FORBIDDEN,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Inacessible route".to_string(),
            warp::http::StatusCode::NOT_FOUND,
        ))
    }
}

fn get_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
    if params.contains_key("start") && params.contains_key("end") {
        return Ok(Pagination {
            start: params
                .get("start")
                .unwrap()
                .parse::<usize>()
                .map_err(Error::ParseError)?,
            end: params
                .get("start")
                .unwrap()
                .parse::<usize>()
                .map_err(Error::ParseError)?,
        });
    }
    Err(Error::ParamsAbsent)
}

async fn add_kb(kb_db: Database, kb: KnowledgeBase) -> Result<impl warp::Reply, warp::Rejection> {
    kb_db.kb.write().insert(kb.id.clone(), kb);

    Ok(warp::reply::with_status(
        "Your question has been added",
        warp::http::StatusCode::OK,
    ))
}

async fn update_kb(
    id: String,
    kb_db: Database,
    kb: KnowledgeBase,
) -> Result<impl warp::Reply, warp::Rejection> {
    match kb_db.kb.write().get_mut(&KBId(id)) {
        Some(k) => *k = kb,
        _ => return Err(warp::reject::custom(Error::KbAbsent)),
    }

    Ok(warp::reply::with_status(
        "Question added to the knowledge base succesfully",
        warp::http::StatusCode::OK,
    ))
}

async fn delete_kb(id: String, kb_db: Database) -> Result<impl warp::Reply, warp::Rejection> {
    match kb_db.kb.write().remove(&KBId(id)) {
        Some(_key) => Ok(warp::reply::with_status(
            "Question deleted from the database",
            warp::http::StatusCode::OK,
        )),
        _ => Err(warp::reject::custom(Error::KbAbsent)),
    }
}

async fn add_info_to_kb(
    kb_db: Database,
    params: HashMap<String, String>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let info = Info {
        id: InfoID("1".to_string()),
        content: params.get("content").unwrap().to_string(),
        kb_id: KBId(params.get("kbId").unwrap().to_string()),
    };

    kb_db.kb_answer.write().insert(info.id.clone(), info);

    Ok(warp::reply::with_status(
        "Information added successfully",
        warp::http::StatusCode::OK,
    ))
}
