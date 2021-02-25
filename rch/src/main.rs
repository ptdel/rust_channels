#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

mod routes;
mod partials;

#[database("rchdb")]
pub struct DbConn(diesel::PgConnection);

fn main() {
    rocket::ignite()
        .attach(DbConn::fairing())
        .mount(
            "/",
            routes![
                routes::list_threads,
                routes::get_thread,
                routes::api_list_threads,
                routes::api_get_thread,
                routes::api_new_thread,
                routes::api_new_reply,
                routes::api_delete_thread,
            ],
        )
        .launch();
}
