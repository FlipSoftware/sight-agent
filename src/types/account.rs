use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Account {
    pub id: Option<AccountId>,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct AccountId(pub i32);

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NewAccount {
    pub email: String,
    pub password: String,
    pub nbf: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Session {
    pub expiry: DateTime<Utc>,
    pub account_id: AccountId,
    pub nbf: DateTime<Utc>,
}
