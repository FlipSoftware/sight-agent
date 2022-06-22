#![warn(clippy::all)]

use std::collections::HashMap;
use tracing::{event, instrument, Level};

use crate::{
    db::Database,
    types::{
        account::Session,
        kb::{KBId, KnowledgeBase},
        pagination::Pagination,
    },
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
        Ok(kb) => Ok(warp::reply::json(&kb)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn get_kb_by_id(id: i32, kb_db: Database) -> Result<impl warp::Reply, warp::Rejection> {
    event!(target: "rust-kb-center", Level::INFO, "pick selected KD id from database...");
    match kb_db.get_kb_by_id(id).await {
        Ok(kb) => Ok(warp::reply::json(&kb)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn add_kb(
    session: Session,
    kb_db: Database,
    new_kb: NewKB,
) -> Result<impl warp::Reply, warp::Rejection> {
    let account_id = session.account_id;

    match kb_db.add_kb(new_kb, &account_id).await {
        Ok(kb) => Ok(warp::reply::json(&kb)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn update_kb(
    id: i32,
    session: Session,
    kb_db: Database,
    update_kb: KnowledgeBase,
) -> Result<impl warp::Reply, warp::Rejection> {
    let account_id = session.account_id;
    if kb_db.is_owner_of_kb(&account_id, id).await? {
        let updated_kb = KnowledgeBase {
            id: KBId(id),
            title: update_kb.title,
            content: update_kb.content,
            tags: update_kb.tags,
        };
        match kb_db.update_kb(updated_kb, id, &account_id).await {
            Ok(kb) => Ok(warp::reply::json(&kb)),
            Err(e) => Err(warp::reject::custom(e)),
        }
    } else {
        Err(warp::reject::custom(handle_errors::Error::Unauthorized))
    }
}

pub async fn delete_kb(
    id: i32,
    session: Session,
    kb_db: Database,
) -> Result<impl warp::Reply, warp::Rejection> {
    let account_id = session.account_id;
    match kb_db.delete_kb(id, &account_id).await {
        Ok(_) => Ok(warp::reply::with_status(
            format!("KB{} deleted", id),
            warp::http::StatusCode::OK,
        )),
        Err(e) => Err(warp::reject::custom(e)),
    }
}
