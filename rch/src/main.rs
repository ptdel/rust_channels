#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

mod routes;

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
                routes::new_thread,
                routes::delete_thread,
            ],
        )
        .launch();
}
