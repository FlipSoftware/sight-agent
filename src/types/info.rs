use serde::{Deserialize, Serialize};

use super::knowledge_base::KBId;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Info {
    pub id: InfoID,
    pub content: String,
    pub kb_id: KBId,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct InfoID(pub String); // TODO: change to uuid
