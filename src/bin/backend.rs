#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use my_todo::db::models::Task;
use my_todo::db::{establish_connection, list_tasks};

#[get("/tasks")]
fn tasks_get() -> String {
    list_tasks(&establish_connection())
        .into_iter()
        .map(|task| task.title)
        .collect::<Vec<String>>()
        .join("\n")
}

fn main() {
    rocket::ignite().mount("/", routes![tasks_get]).launch();
}
