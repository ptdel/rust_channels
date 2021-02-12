#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate rocket;

mod routes;

#[database("rchdb")]
pub struct DbConn(diesel::PgConnection);

fn main() {
    rocket::ignite()
        .attach(DbConn::fairing())
        .mount("/", routes![
            routes::list_threads,
            routes::get_thread,
        ])
        .launch();
}