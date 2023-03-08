use super::schema::task;

#[derive(Insertable)]
#[table_name = "task"]
pub struct NewTask<'a> {
    pub title: &'a str,
    pub done: i32,
}

/// This has to be defined as a separate struct because we want to
/// be able to insert a task without an ID as it will be generated
/// for us, but we want to get the ID when we query a task.
#[derive(Queryable, Serialize)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub done: i32,
}
