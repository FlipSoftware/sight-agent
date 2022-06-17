use sqlx::{postgres::PgPoolOptions, PgPool, Row};

use crate::types::{
    kb::{KBId, KnowledgeBase, NewKB},
    reply::{NewReply, Reply, ReplyId},
};

#[derive(Debug, Clone)]
pub struct Database {
    pub connection: PgPool,
}

impl Database {
    pub async fn new(db_url: &str) -> Self {
        let db_pool = match PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await
        {
            Ok(pool) => pool,
            _ => panic!("Connection to the database failed!"),
        };

        Database {
            connection: db_pool,
        }
    }

    pub async fn get_kb(
        &self,
        limit: Option<u32>,
        offset: u32,
    ) -> Result<Vec<KnowledgeBase>, handle_errors::Error> {
        match sqlx::query("SELECT * from kb LIMIT $1 OFFSET $2")
            .bind(limit)
            .bind(offset)
            .map(|row| KnowledgeBase {
                id: KBId(row.get("id")),
                title: row.get("title"),
                content: row.get("content"),
                tags: row.get("tags"),
            })
            .fetch_all(&self.connection)
            .await
        {
            Ok(kb) => Ok(kb),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{}", e);
                Err(handle_errors::Error::DBQueryError)
            }
        }
    }

    pub async fn add_kb(self, new_kb: NewKB) -> Result<KnowledgeBase, handle_errors::Error> {
        match sqlx::query(
            "
INSERT INTO kb (title, content, tags) VALUES ($1, $2, $3) RETURNING id, title, content, tags
",
        )
        .bind(new_kb.title)
        .bind(new_kb.content)
        .bind(new_kb.tags)
        .map(|row| KnowledgeBase {
            id: KBId(row.get("id")),
            title: row.get("title"),
            content: row.get("content"),
            tags: row.get("tags"),
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(kb) => Ok(kb),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{}", e);
                Err(handle_errors::Error::DBQueryError)
            }
        }
    }

    pub async fn update_kb(
        self,
        kb: KnowledgeBase,
        kb_id: i32,
    ) -> Result<KnowledgeBase, handle_errors::Error> {
        match sqlx::query(
            "
UPDATE kb SET title = $1, content = $2, tags = $3 WHERE id = $4 RETURNING id, title, content, tags
            ",
        )
        .bind(kb.title)
        .bind(kb.content)
        .bind(kb.tags)
        .bind(kb_id)
        .map(|row| KnowledgeBase {
            id: KBId(row.get("id")),
            title: row.get("title"),
            content: row.get("content"),
            tags: row.get("tags"),
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(kb) => Ok(kb),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{}", e);
                Err(handle_errors::Error::DBQueryError)
            }
        }
    }

    pub async fn delete_kb(self, kb_id: i32) -> Result<bool, handle_errors::Error> {
        match sqlx::query(
            "
DELETE FROM kb WHERE id = $1
            ",
        )
        .bind(kb_id)
        .execute(&self.connection)
        .await
        {
            Ok(_) => Ok(true),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{}", e);
                Err(handle_errors::Error::DBQueryError)
            }
        }
    }

    pub async fn get_kb_by_id(&self, kb_id: i32) -> Result<KnowledgeBase, handle_errors::Error> {
        match sqlx::query("SELECT * from kb WHERE id = $1")
            .bind(kb_id)
            .map(|row| KnowledgeBase {
                id: KBId(row.get("id")),
                title: row.get("title"),
                content: row.get("content"),
                tags: row.get("tags"),
            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(kb) => Ok(kb),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{}", e);
                Err(handle_errors::Error::DBQueryError)
            }
        }
    }

    pub async fn add_reply(&self, new_reply: NewReply) -> Result<Reply, handle_errors::Error> {
        match sqlx::query(
            "
INSERT INTO reply (content, kb_id) VALUES ($1, $2)
            ",
        )
        .bind(new_reply.content)
        .bind(new_reply.kb_id.0)
        .map(|row| Reply {
            id: ReplyId(row.get("id")),
            content: row.get("content"),
            kb_id: KBId(row.get("question_id")),
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(reply) => Ok(reply),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(handle_errors::Error::DBQueryError)
            }
        }
    }
}
