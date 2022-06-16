use colored::Colorize;
use std::collections::HashMap;
use tracing::{event, instrument, Level};

use crate::{
    db::Database,
    types::{kb::KnowledgeBase, pagination::Pagination},
    types::{kb::NewKB, pagination::get_page_offset},
};

#[instrument]
pub async fn get_kb(
    params: HashMap<String, String>,
    kb_db: Database,
) -> Result<impl warp::Reply, warp::Rejection> {
    event!(target: "rust-kb-center", Level::INFO, "fetching database...");
    let mut page = Pagination::default();

    if !params.is_empty() {
        event!(Level::INFO, page = true);
        page = get_page_offset(params)?;
    }

    match kb_db.get_kb(page.limit, page.offset).await {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn add_kb(kb_db: Database, new_kb: NewKB) -> Result<impl warp::Reply, warp::Rejection> {
    match kb_db.add_kb(new_kb).await {
        Ok(_) => Ok(warp::reply::with_status(
            "Your question has been added to the base",
            warp::http::StatusCode::OK,
        )),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn update_kb(
    id: i32,
    kb_db: Database,
    update_kb: KnowledgeBase,
) -> Result<impl warp::Reply, warp::Rejection> {
    match kb_db.update_kb(update_kb, id).await {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn delete_kb(id: i32, kb_db: Database) -> Result<impl warp::Reply, warp::Rejection> {
    match kb_db.delete_kb(id).await {
        Ok(_) => Ok(warp::reply::with_status(
            format!("KB{} deleted", id),
            warp::http::StatusCode::OK,
        )),
        Err(e) => Err(warp::reject::custom(e)),
    }
}
