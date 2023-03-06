# Rusty Server
A Rust web server for a simple "to do" application.

## Usage

### CLI Commands

- `cargo run --bin todo new <title>`: Create a new Task record in the database.
- `cargo run --bin todo show`: List all Task records in the database.
- `cargo run --bin todo finish <title> <0 or 1>`: Mark a Task record with the given title as done (1) or not done (0)