use crate::DbConn;
use crate::partials::page;
use diesel::result::Error;
use rchdb::models::{NewThread, Thread};
use rchdb::{delete, get, list, new};
use rocket::response::status::{Created, NoContent};
use rocket_contrib::json::Json;
use maud::{html, Markup};

// Template Routes
#[get("/")]
pub fn tmpl_list_threads(conn: DbConn) -> Markup {
    let threads = list(&conn).unwrap();
    page("OverBoard", html! {
        table {
            @for thread in &threads {
                tr {
                    td {
                        @match &thread.title {
                            Some(title) => (title),
                            None => "",
                        }
                    }
                    td { (thread.id) }
                    td { (thread.content) }
                }
            }
        }
    })
}


// API routes

#[get("/api")]
pub fn api_list_threads(conn: DbConn) -> Result<Json<Vec<Thread>>, Error> {
    list(&conn).map(Json).map_err(|err| err)
}

#[get("/api/<id>")]
pub fn api_get_thread(id: i32, conn: DbConn) -> Result<Json<Thread>, Error> {
    get(id, &conn).map(Json).map_err(|err| err)
}

#[post("/api", format = "application/json", data = "<thread>")]
pub fn api_new_thread(thread: Json<NewThread>, conn: DbConn) -> Result<Created<Json<Thread>>, Error> {
    new(thread.into_inner(), &conn)
        .map(|person| Created(format!("/people/{id}", id = person.id), Some(Json(person))))
        .map_err(|err| err)
}

#[delete("/api/<id>")]
pub fn api_delete_thread(id: i32, conn: DbConn) -> Result<NoContent, Error> {
    delete(id, &conn).map(|_| NoContent).map_err(|err| err)
}
