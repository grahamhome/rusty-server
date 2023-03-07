use diesel::result::{DatabaseErrorKind, Error};
use diesel::{prelude::*, sqlite::SqliteConnection};

pub mod models;
pub mod schema;

/// Database functions used throughout the application.

/// Creates and returns a database connection.
pub fn establish_connection() -> SqliteConnection {
    let db = "./tododb.sqlite3";
    SqliteConnection::establish(db).unwrap_or_else(|_| panic!("Error connecting to {}", db))
}

/// Inserts a new task into the database.
pub fn create_task(connection: &SqliteConnection, title: &str) -> Result<(), String> {
    let task = models::NewTask { title, done: 0 };

    match diesel::insert_into(schema::task::table)
        .values(&task)
        .execute(connection)
    {
        Ok(_) => Ok(()),
        Err(err) => match err {
            Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _err) => {
                Err(format!("new: a task with title '{}' already exists", title))
            }
            _ => Err(String::from(
                "An error occurred while attempting to create the task",
            )),
        },
    }
}

/// Loads all tasks from the database.
pub fn list_tasks(connection: &SqliteConnection) -> Vec<models::Task> {
    schema::task::table
        .load::<models::Task>(connection)
        .expect("Error loading tasks")
}

/// Queries database eturns a Result containing either a task with the given title
/// or None if no such task is found.
pub fn query_tasks_by_title(
    connection: &SqliteConnection,
    title: &str,
) -> Result<models::Task, String> {
    match schema::task::table
        .filter(schema::task::title.eq(title))
        .first::<models::Task>(connection)
    {
        Ok(task) => Ok(task),
        Err(e) => match e {
            diesel::NotFound => Err(format!("No task has title '{title}'")),
            _ => Err(String::from("Error querying tasks by title")),
        },
    }
}

/// Updates the status of the task with the given ID to the given value.
pub fn set_task_done_status(connection: &SqliteConnection, task_id: i32, done_status: bool) {
    let done_status = match done_status {
        true => 1,
        false => 0,
    };
    diesel::update(schema::task::table.find(task_id))
        .set(schema::task::done.eq(done_status))
        .execute(connection)
        .expect(format!("Error updating task with id {task_id}").as_str());
}

/// Deletes the task with the given ID from the database.
pub fn delete_task(connection: &SqliteConnection, task_id: i32) {
    diesel::delete(schema::task::table.find(task_id))
        .execute(connection)
        .expect(format!("Error deleting task with id {task_id}").as_str());
}
