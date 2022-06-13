use handle_errors::Error;
use std::collections::HashMap;

use crate::{
    db::Database,
    types::knowledge_base::{KBId, KnowledgeBase},
    types::pagination::get_pagination,
};

pub async fn get_kb(
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

pub async fn add_kb(
    kb_db: Database,
    kb: KnowledgeBase,
) -> Result<impl warp::Reply, warp::Rejection> {
    kb_db.kb.write().insert(kb.id.clone(), kb);

    Ok(warp::reply::with_status(
        "Your question has been added",
        warp::http::StatusCode::OK,
    ))
}

pub async fn update_kb(
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

pub async fn delete_kb(id: String, kb_db: Database) -> Result<impl warp::Reply, warp::Rejection> {
    match kb_db.kb.write().remove(&KBId(id)) {
        Some(_key) => Ok(warp::reply::with_status(
            "Question deleted from the database",
            warp::http::StatusCode::OK,
        )),
        _ => Err(warp::reject::custom(Error::KbAbsent)),
    }
}
