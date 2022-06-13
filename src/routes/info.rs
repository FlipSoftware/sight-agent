use std::collections::HashMap;

use crate::{
    db::Database,
    types::info::{Info, InfoID},
    types::knowledge_base::KBId,
};

pub async fn add_info_to_kb(
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
