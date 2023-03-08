#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate serde;

use my_todo::db::models::Task;
use my_todo::db::{establish_connection, list_tasks};
use rocket_contrib::{json::Json, databases::diesel};

#[database("sqlite_tasks")]
struct TasksDbConn(diesel::SqliteConnection);

#[derive(Serialize)]
struct JsonApiResponse {
    data: Vec<Task>
}

#[get("/tasks")]
fn tasks_get(conn: TasksDbConn) -> Json<JsonApiResponse> {
    Json(JsonApiResponse{ data: list_tasks(&*conn), })
}

fn main() {
    rocket::ignite().attach(TasksDbConn::fairing()).mount("/", routes![tasks_get]).launch();
}
