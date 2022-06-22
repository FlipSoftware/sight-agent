#![warn(clippy::all)]

use crate::{
    db::Database,
    types::{account::Session, reply::NewReply},
};

pub async fn add_reply(
    session: Session,
    kb_db: Database,
    new_reply: NewReply,
) -> Result<impl warp::Reply, warp::Rejection> {
    let account_id = session.account_id;
    match kb_db.add_reply(new_reply, &account_id).await {
        Ok(_) => Ok(warp::reply::with_status(
            "Reply added successfully",
            warp::http::StatusCode::OK,
        )),
        Err(e) => Err(warp::reject::custom(e)),
    }
}
