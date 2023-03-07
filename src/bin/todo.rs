extern crate core;

use my_todo::db::*;
use std::env;

/// A CLI tool for interacting with the to-do app database.

fn help() {
    println!("subcommands:");
    println!("\thelp: show these instructions");
    println!("\tnew <title>: create a new task");
    println!("\tshow: show list of tasks");
    println!("\tfinish <title> <done>: mark task with title as done (1) or not done (0)");
    println!("\tdelete <title>: Delete task with title");
}

/// CLI entry point: calls subcommand functions based on user input.
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
        "finish" => finish(&args[2..]),
        "delete" => delete(&args[2..]),
        _ => help(),
    }
}

/// Calls database function to create new task and handles errors.
fn new_task(args: &[String]) {
    if args.len() != 1 {
        println!("new: expects one argument, got {}", args.len());
        help();
        return;
    }
    let conn = establish_connection();
    match create_task(&conn, &args[0]) {
        Ok(_) => (),
        Err(msg) => println!("{}", msg),
    }
}

/// Calls database function to list all tasks.
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

/// Calls database function to update task status. Handles any errors.
fn finish(args: &[String]) {
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
        } else {
            panic!("finish: second argument must be 0 or 1")
        }
    };
    let conn = establish_connection();
    match query_tasks_by_title(&conn, &args[0]) {
        Ok(task) => set_task_done_status(&conn, task.id, done_status),
        Err(msg) => println!("{}", msg),
    };
}

/// Calls database function to delete a task, handling any errors.
fn delete(args: &[String]) {
    if args.len() < 1 {
        println!("delete: expects one argument, got {}", args.len());
        help();
        return;
    };
    let conn = establish_connection();
    match query_tasks_by_title(&conn, &args[0]) {
        Ok(task) => delete_task(&conn, task.id),
        Err(msg) => println!("{}", msg),
    };
}
