use super::schema::threads;
use chrono::NaiveDateTime;
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

#[derive(Deserialize, Insertable)]
#[table_name="threads"]
pub struct NewThread {
    pub title: Option<String>,
    pub content: String,
    pub parent: Option<i32>
}