use sqlx::{postgres::PgPoolOptions, PgPool, Row};

use crate::types::{
    account::{Account, AccountId},
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
        limit: Option<i32>,
        offset: i32,
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
                Err(handle_errors::Error::DBQueryError(e))
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
                Err(handle_errors::Error::DBQueryError(e))
            }
        }
    }

    pub async fn add_kb(
        self,
        new_kb: NewKB,
        account_id: &AccountId,
    ) -> Result<KnowledgeBase, handle_errors::Error> {
        match sqlx::query(
            "
INSERT INTO kb (title, content, tags, account_id) VALUES ($1, $2, $3, $4) RETURNING id, title, content, tags, account_id
",
        )
        .bind(new_kb.title)
        .bind(new_kb.content)
        .bind(new_kb.tags)
        .bind(account_id.0)
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
                Err(handle_errors::Error::DBQueryError(e))
            }
        }
    }

    pub async fn update_kb(
        self,
        kb: KnowledgeBase,
        kb_id: i32,
        account_id: &AccountId,
    ) -> Result<KnowledgeBase, handle_errors::Error> {
        match sqlx::query(
            "
UPDATE kb SET title = $1, content = $2, tags = $3
WHERE id = $4 AND account_id = $5
RETURNING id, title, content, tags
            ",
        )
        .bind(kb.title)
        .bind(kb.content)
        .bind(kb.tags)
        .bind(kb_id)
        .bind(account_id.0)
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
                Err(handle_errors::Error::DBQueryError(e))
            }
        }
    }

    pub async fn delete_kb(
        self,
        kb_id: i32,
        account_id: &AccountId,
    ) -> Result<bool, handle_errors::Error> {
        match sqlx::query(
            "
DELETE FROM kb WHERE id = $1 AND account_id = $2
            ",
        )
        .bind(kb_id)
        .bind(account_id.0)
        .execute(&self.connection)
        .await
        {
            Ok(_) => Ok(true),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{}", e);
                Err(handle_errors::Error::DBQueryError(e))
            }
        }
    }

    pub async fn add_reply(
        &self,
        new_reply: NewReply,
        account_id: &AccountId,
    ) -> Result<Reply, handle_errors::Error> {
        match sqlx::query(
            "
INSERT INTO reply (content, kb_id, account_id) VALUES ($1, $2, $3)
            ",
        )
        .bind(new_reply.content)
        .bind(new_reply.kb_id.0)
        .bind(account_id.0)
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
                Err(handle_errors::Error::DBQueryError(e))
            }
        }
    }

    pub async fn is_owner_of_kb(
        &self,
        account_id: &AccountId,
        kb_id: i32,
    ) -> Result<bool, handle_errors::Error> {
        match sqlx::query(
            "
SELECT * FROM kb WHERE id = $1 AND account_id = $2
            ",
        )
        .bind(kb_id)
        .bind(account_id.0)
        .fetch_optional(&self.connection)
        .await
        {
            Ok(id) => Ok(id.is_some()),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(handle_errors::Error::DBQueryError(e))
            }
        }
    }

    pub async fn is_owner_of_reply(
        &self,
        account_id: &AccountId,
        reply_id: i32,
    ) -> Result<bool, handle_errors::Error> {
        match sqlx::query(
            "
SELECT * FROM reply WHERE id = $1 AND account_id = $2
            ",
        )
        .bind(reply_id)
        .bind(account_id.0)
        .fetch_optional(&self.connection)
        .await
        {
            Ok(id) => Ok(id.is_some()),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{}", e);
                Err(handle_errors::Error::DBQueryError(e))
            }
        }
    }

    pub async fn get_account(self, email: String) -> Result<Account, handle_errors::Error> {
        match sqlx::query("SELECT * from accounts WHERE email = $1")
            .bind(email)
            .map(|row| Account {
                id: Some(AccountId(row.get("id"))),
                email: row.get("email"),
                password: row.get("password"),
            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(acc) => Ok(acc),
            Err(error) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", error);
                Err(handle_errors::Error::DBQueryError(error))
            }
        }
    }

    pub async fn add_account(self, account: Account) -> Result<bool, handle_errors::Error> {
        match sqlx::query(
            "
INSERT INTO accounts (email, password) VALUES ($1, $2)
            ",
        )
        .bind(account.email)
        .bind(account.password)
        .execute(&self.connection)
        .await
        {
            Ok(_) => Ok(true),
            Err(e) => {
                // Error type captured is slqx::error::Error
                tracing::event!(
                    tracing::Level::ERROR,
                    code = e
                        .as_database_error()
                        .unwrap()
                        .code()
                        .unwrap()
                        .parse::<i32>()
                        .unwrap(),
                    db_message = e.as_database_error().unwrap().message(),
                    constraint = e.as_database_error().unwrap().constraint().unwrap()
                );
                Err(handle_errors::Error::DBQueryError(e))
            }
        }
    }
}
