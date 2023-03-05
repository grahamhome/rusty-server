extern crate core;

use std::env;
use my_todo::db::{create_task, list_tasks, query_tasks_by_title, set_task_done_status, establish_connection};

/// A CLI tool for interacting with the to-do app database.


fn help() {
    println!("subcommands:");
    println!("\tnew <title>: create a new task");
    println!("\tshow: show list of tasks");
    println!("\tfinish <title> <done>: mark task with title as done (1) or not done (0)")
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        help();
        return;
    }
    let subcommand = &args[1];
    match subcommand.as_ref() {
        "new" => new_task(&args[2..]),
        "show" => show_tasks(&args[2..]),
        "finish" => mark_task_done(&args[2..]),
        _ => help(),
    }
}

fn new_task(args: &[String]) {
    if args.len() < 1 {
        println!("new: missing <title>");
        help();
        return;
    }
    let conn = establish_connection();
    create_task(&conn, &args[0]);
}

fn show_tasks(args: &[String]) {
    if args.len() > 0 {
        println!("show: unexpected argument");
        help();
        return;
    }
    let conn = establish_connection();
    println!("TASKS\n-----");
    for task in list_tasks(&conn) {
        let status = {
            if task.done == 0 {
                "not done"
            } else {
                "done"
            }
        };
        println!("Task '{}' is {}", task.title, status);
    }
}

fn mark_task_done(args: &[String]) {
    if args.len() != 2 {
        println!("finish: expects two arguments");
        help();
        return;
    }
    let done_status = {
        if &args[1] == "1" {
            true
        } else if &args[1] == "0" {
            false
        } else { panic!("finish: second argument must be 0 or 1")}
    };
    let conn = establish_connection();
    let task_query_result = query_tasks_by_title(&conn, &args[0]).pop();
    match task_query_result {
        Some(task) => set_task_done_status(&conn, task.id, done_status),
        None => println!("No task has title '{}'", &args[0]),
    };
}