extern crate core;

use std::env;
use diesel::result::{DatabaseErrorKind, Error};
use my_todo::db::*;

/// A CLI tool for interacting with the to-do app database.


fn help() {
    println!("subcommands:");
    println!("\thelp: show these instructions");
    println!("\tnew <title>: create a new task");
    println!("\tshow: show list of tasks");
    println!("\tfinish <title> <done>: mark task with title as done (1) or not done (0)");
    println!("\tdelete <title>: Delete task with title");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        help();
        return;
    }
    let subcommand = &args[1];
    match subcommand.as_ref() {
        "help" => help(),
        "new" => new_task(&args[2..]),
        "show" => show_tasks(&args[2..]),
        "finish" => mark_task_done(&args[2..]),
        "delete" => delete(&args[2..]),
        _ => help(),
    }
}

fn new_task(args: &[String]) {
    if args.len() != 1 {
        println!("new: expects one argument, got {}", args.len());
        help();
        return;
    }
    let conn = establish_connection();
    match create_task(&conn, &args[0]) {
        Ok(_) => (),
        Err(err) => match err {
            Error::DatabaseError(DatabaseErrorKind::UniqueViolation, err) => {
                println!("new: a task with title '{}' already exists", &args[0])
            },
            _ => println!("An error occurred while attempting to create the task")
        }
    };
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
        println!("finish: expects two arguments, got {}", args.len());
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

fn delete(args: &[String]) {
    if args.len() < 1 {
        println!("delete: expects one argument, got {}", args.len());
        help();
        return;
    };
    let conn = establish_connection();
    let task_query_result = query_tasks_by_title(&conn, &args[0]).pop();
    match task_query_result {
        Some(task) => delete_task(&conn, task.id),
        None => println!("No task has title '{}'", &args[0]),
    };
}