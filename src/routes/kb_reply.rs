// use std::collections::HashMap;

// use crate::{
//     db::Database,
//     types::kb::KBId,
//     types::reply::{KBReply, KBReplyId},
// };

// pub async fn add_info_to_kb(
//     kb_db: Database,
//     params: HashMap<String, String>,
// ) -> Result<impl warp::Reply, warp::Rejection> {
//     let info = KBReply {
//         id: KBReplyId(rand::random::<i32>()),
//         content: params.get("content").unwrap().to_string(),
//         kb_id: KBId(params.get("kb_id").unwrap().parse::<i32>().unwrap()),
//     };

//     Ok(warp::reply::with_status(
//         "Information added successfully",
//         warp::http::StatusCode::OK,
//     ))
// }
