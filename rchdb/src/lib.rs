#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate serde;
extern crate serde_derive;

pub mod models;
pub mod schema;

use self::models::{NewThread, Thread};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use schema::threads;

pub fn list(conn: &PgConnection) -> QueryResult<Vec<Thread>> {
    threads::table.load::<Thread>(conn)
}

pub fn new(thread: NewThread, conn: &PgConnection) -> QueryResult<Thread> {
    diesel::insert_into(threads::table)
        .values(thread)
        .get_result(conn)
}

pub fn get(id: i32, conn: &PgConnection) -> QueryResult<Thread> {
    threads::table.find(id).get_result::<Thread>(conn)
}

pub fn delete(id: i32, conn: &PgConnection) -> QueryResult<usize> {
    diesel::delete(threads::table.find(id)).execute(conn)
}
