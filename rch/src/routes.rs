use rchdb::{list,get,delete};
use rchdb::models::{Thread};
use rocket_contrib::json::Json;
use rocket::response::status::NotFound;
use crate::DbConn;


#[get("/")]
pub fn list_threads(conn: DbConn) -> Result<Json<Vec<Thread>>, NotFound<String>> {
    list(&conn).map(|thread| Json(thread)).map_err(|e| NotFound(e.to_string()))
}

#[get("/<id>")]
pub fn get_thread(id: i32, conn: DbConn) -> Result<Json<Thread>, NotFound<String>> {
    get(id, &conn).map(|thread| Json(thread)).map_err(|e| NotFound(e.to_string()))
}

#[delete("/<id>")]
pub fn delete_thread(id: i32, conn: DbConn) -> Result<String, NotFound<String>> {
    match delete(id, &conn) {
        Ok(_) => format!("{{ \"deleted\": \"{}\" }}", id),
        Err(e) => NotFound(e.to_string()),
    }
}