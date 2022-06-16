use crate::{db::Database, types::reply::NewReply};

pub async fn add_reply(
    kb_db: Database,
    new_reply: NewReply,
) -> Result<impl warp::Reply, warp::Rejection> {
    match kb_db.add_reply(new_reply).await {
        Ok(_) => Ok(warp::reply::with_status(
            "Reply added successfully",
            warp::http::StatusCode::OK,
        )),
        Err(e) => Err(warp::reject::custom(e)),
    }
}
