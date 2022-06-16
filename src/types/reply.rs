use serde::{Deserialize, Serialize};

use super::kb::KBId;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KBReply {
    pub id: KBReplyId,
    pub content: String,
    pub kb_id: KBId,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct KBReplyId(pub i32);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Reply {
    pub id: ReplyId,
    pub content: String,
    pub kb_id: KBId,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReplyId(pub i32);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewReply {
    pub content: String,
    pub kb_id: KBId,
}
