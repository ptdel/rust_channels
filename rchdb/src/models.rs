use chrono::NaiveDateTime;
use super::schema::threads;
use serde::{Serialize,Deserialize};

#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
pub struct Thread {
    pub id: i32,
    pub created: NaiveDateTime,
    pub title: Option<String>,
    pub content: String,
    pub bumped: NaiveDateTime,
    pub parent: Option<i32>,
    pub locked: bool,
}

#[derive(Insertable)]
#[table_name="threads"]
pub struct NewThread {
    pub title: Option<String>,
    pub content: String,
}

impl NewThread {
    pub fn from_thread(thread: Thread) -> NewThread {
        NewThread {
            title: thread.title,
            content: thread.content,
        }
    }
}