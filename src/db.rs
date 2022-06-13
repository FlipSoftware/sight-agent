use std::{collections::HashMap, sync::Arc};

use parking_lot::RwLock;

use crate::types::{
    info::{Info, InfoID},
    knowledge_base::{KBId, KnowledgeBase},
};

#[derive(Clone)]
pub struct Database {
    pub kb: Arc<RwLock<HashMap<KBId, KnowledgeBase>>>,
    pub kb_answer: Arc<RwLock<HashMap<InfoID, Info>>>,
}

impl Database {
    pub fn new() -> Self {
        Database {
            kb: Arc::new(RwLock::new(Self::init())),
            kb_answer: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn init() -> HashMap<KBId, KnowledgeBase> {
        let file = include_str!("../db.json");
        serde_json::from_str(file).expect("Error reading the Database.")
    }
}
