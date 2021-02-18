use rocket::response::status::{Created,NoContent};
use diesel::result::Error;
use rchdb::{list,get,new,delete};
use rchdb::models::{Thread,NewThread};
use rocket_contrib::json::{Json};
use crate::DbConn;


#[get("/")]
pub fn list_threads(conn: DbConn) -> Result<Json<Vec<Thread>>, Error> {
    list(&conn)
        .map(|thread| Json(thread))
        .map_err(|err| err)
}

#[get("/<id>")]
pub fn get_thread(id: i32, conn: DbConn) -> Result<Json<Thread>, Error> {
    get(id, &conn)
        .map(|thread| Json(thread))
        .map_err(|err| err)
}

#[post("/", format = "application/json", data = "<thread>")]
pub fn new_thread(thread: Json<NewThread>, conn: DbConn) -> Result<Created<Json<Thread>>, Error> {
    new(thread.into_inner(), &conn)
        .map(|person|
            Created(
                format!("/people/{id}", id = person.id),
                Some(Json(person))))
        .map_err(|err| err)
}

#[delete("/<id>")]
pub fn delete_thread(id: i32, conn: DbConn) -> Result<NoContent, Error> {
    delete(id, &conn)
        .map(|_| NoContent)
        .map_err(|err| err)
}