use super::schema::{threads,replies};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Queryable, Serialize, Deserialize)]
pub struct Thread {
    pub id: i32,
    pub created: NaiveDateTime,
    pub title: Option<String>,
    pub content: String,
    pub bumped: NaiveDateTime,
    pub locked: bool,
}

#[derive(Identifiable, Associations, Queryable, Serialize, Deserialize)]
#[belongs_to(parent="Thread")]
#[table_name = "replies"]
pub struct Reply {
    pub id: i32,
    pub created: NaiveDateTime,
    pub content: String,
    pub thread_id: i32,
}

#[derive(Insertable, Deserialize)]
#[table_name = "threads"]
pub struct NewThread {
    pub title: Option<String>,
    pub content: String,
}

#[derive(Insertable, Deserialize)]
#[table_name = "replies"]
pub struct NewReply {
    pub content: String,
    pub thread_id: Option<i32>,
}