#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate serde;

use my_todo::db::models::Task;
use my_todo::db::{list_tasks};
use rocket_contrib::{json::Json, databases::diesel};

#[database("sqlite_tasks")]
struct TasksDbConn(diesel::SqliteConnection);

#[derive(Serialize)]
struct JsonApiResponse<T> {
    data: Vec<T>
}

#[derive(Serialize)]
struct TaskWrapper {
    id: i32,
    #[serde(rename = "type")]
    type_name: String,
    attributes: TaskAttributes,
}

impl TaskWrapper {
    fn new(task: Task) -> Self {
        Self {
            id: task.id,
            type_name: String::from("task"),
            attributes: TaskAttributes::new(task)
        }
    }
}

#[derive(Serialize)]
struct TaskAttributes {
    title: String,
    status: String,
}

impl TaskAttributes {
    fn new(task: Task) -> Self {
        Self {
            title: task.title,
            status: match task.done {
                0 => String::from("Not finished"),
                1 => String::from("Finished"),
                _ => panic!("Task status should always be 0 or 1")
            },
        }
    }
}

#[get("/tasks")]
fn tasks_get(conn: TasksDbConn) -> Json<JsonApiResponse<TaskWrapper>> {
    Json(JsonApiResponse{ data: list_tasks(&*conn).into_iter().map(|task| TaskWrapper::new(task)).collect::<Vec<TaskWrapper>>(), })
}

fn main() {
    rocket::ignite().attach(TasksDbConn::fairing()).mount("/", routes![tasks_get]).launch();
}
