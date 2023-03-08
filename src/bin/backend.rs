#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate serde;

use my_todo::db::models::Task;
use my_todo::db::{establish_connection, list_tasks};
use rocket_contrib::json::Json;

#[derive(Serialize)]
struct JsonApiResponse {
    data: Vec<Task>
}

#[get("/tasks")]
fn tasks_get() -> Json<JsonApiResponse> {
    Json(JsonApiResponse{ data: list_tasks(&establish_connection()), })
}

fn main() {
    rocket::ignite().mount("/", routes![tasks_get]).launch();
}
