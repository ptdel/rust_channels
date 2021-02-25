use crate::partials::page;
use crate::DbConn;
use diesel::result::Error;
use maud::{html, Markup};
use rchdb::models::{NewReply, NewThread, Reply, Thread};
use rchdb::{delete, get, list, new, reply};
use rocket::response::status::{Created, NoContent};
use rocket_contrib::json::Json;

// Template Routes
#[get("/")]
pub fn list_threads(conn: DbConn) -> Markup {
    let threads = list(&conn).unwrap();
    page(
        "OverBoard",
        html! {
            table {
                @for thread in &threads {
                    tr {
                        td { (thread.id) }
                        td { (thread.title.as_ref().unwrap_or(&"".to_string())) }
                    }
                }
            }
        },
    )
}

#[get("/<id>")]
pub fn get_thread(id: i32, conn: DbConn) -> Markup {
    let (thread, replies) = get(id, &conn).unwrap();
    let title = thread.title.unwrap_or("".to_string());
    page(
        &title,
        html! {
            table {
                tr {
                    td { (thread.id) }
                    td { (thread.content) }
                }
                @for reply in &replies {
                    tr {
                        td { (reply.id) }
                        td { (reply.content) }
                    }
                }
            }
        },
    )
}

// API routes

#[get("/api")]
pub fn api_list_threads(conn: DbConn) -> Result<Json<Vec<Thread>>, Error> {
    list(&conn).map(Json).map_err(|err| err)
}

#[get("/api/<id>")]
pub fn api_get_thread(id: i32, conn: DbConn) -> Result<Json<(Thread, Vec<Reply>)>, Error> {
    get(id, &conn).map(Json).map_err(|err| err)
}

#[post("/api", format = "application/json", data = "<new_thread>")]
pub fn api_new_thread(
    new_thread: Json<NewThread>,
    conn: DbConn,
) -> Result<Created<Json<Thread>>, Error> {
    new(new_thread.into_inner(), &conn)
        .map(|thread| Created(format!("/thread/{id}", id = thread.id), Some(Json(thread))))
        .map_err(|err| err)
}

#[post("/api/<id>", format = "application/json", data = "<new_reply>")]
pub fn api_new_reply(
    id: i32,
    new_reply: Json<NewReply>,
    conn: DbConn,
) -> Result<Created<Json<Reply>>, Error> {
    let input = NewReply {
        thread_id: Some(id),
        ..new_reply.into_inner()
    };
    reply(input, &conn)
        .map(|reply| Created(format!("/reply/{id}", id = reply.id), Some(Json(reply))))
        .map_err(|err| err)
}

#[delete("/api/<id>")]
pub fn api_delete_thread(id: i32, conn: DbConn) -> Result<NoContent, Error> {
    delete(id, &conn).map(|_| NoContent).map_err(|err| err)
}
