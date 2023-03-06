use diesel::{prelude::*, sqlite::SqliteConnection};

pub mod models;
pub mod schema;

pub fn establish_connection() -> SqliteConnection {
    let db = "./tododb.sqlite3";
    SqliteConnection::establish(db)
        .unwrap_or_else(|_| panic!("Error connecting to {}", db))
}

pub fn create_task(connection: &SqliteConnection, title: &str) {
    let task = models::NewTask { title, done: 0 };

    diesel::insert_into(schema::task::table)
        .values(&task)
        .execute(connection)
        .expect("Error inserting new task");
}

pub fn list_tasks(connection: &SqliteConnection) -> Vec<models::Task> {
    schema::task::table
        .load::<models::Task>(connection)
        .expect("Error loading tasks")
}

pub fn query_tasks_by_title(connection: &SqliteConnection, title: &str) -> Vec<models::Task> {
    schema::task::table.filter(schema::task::title.eq(title)).load::<models::Task>(connection).expect("Error filtering tasks by title")
}

pub fn set_task_done_status(connection: &SqliteConnection, task_id: i32, done_status: bool) {
    let done_status = match done_status {
        true => 1,
        false => 0,
    };
    diesel::update(schema::task::table.find(task_id)).set(schema::task::done.eq(done_status)).execute(connection).expect(format!("Error updating task with id {}", task_id).as_str());
}

pub fn delete_task(connection: &SqliteConnection, task_id: i32) {
    diesel::delete(schema::task::table.find(task_id)).execute(connection).expect(format!("Error deleting task with id {}", task_id).as_str());
}