mod add_todo;
mod delete_todo;
mod done_todo;
mod list_todo;

use add_todo::add_todo;
use delete_todo::delete_todo;
use done_todo::done_todo;
use list_todo::list_todo;

use clap::{value_parser, Arg, Command};
use rusqlite::Connection;

#[derive(Debug, Clone)]
struct Todo {
    id: i64,
    task: String,
    done: String,
}

fn main() {
    let command = 
    Command::new("todo")
    .subcommand(
        Command::new("list")
        .about("List todos")
    )
    .subcommand(
        Command::new("add")
        .arg(
            Arg::new("tasks")
            .num_args(1..)
            .value_parser(value_parser!(String))
        )
        .about("Add new todos")
    )
    .subcommand(
        Command::new("del")
        .arg(
            Arg::new("ids")
            .value_parser(value_parser!(String))
            .num_args(1..)
        )
        .about("Delete todos")
    )
    .subcommand(
        Command::new("done")
        .arg(
            Arg::new("ids")
            .value_parser(value_parser!(String))
            .num_args(1..)
        )
        .about("Mark todos as done")
    )
    .arg_required_else_help(true)
    .about("Todo app written in Rust using rusqlite db")
    .get_matches();

    let conn = Connection::open("./todo.db").unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todos (
            id   INTEGER PRIMARY KEY,
            task TEXT NOT NULL,
            done TEXT NOT NULL
        )",
        (), // empty list of parameters.
    ).unwrap();

    // let _ = open_todo(&conn);
    // let _ = add_todo(&conn);
    // let _ = delete_todo(&conn);

    match command.subcommand() {
        Some(("list", _)) => {
            list_todo(&conn).unwrap();
        }
        Some(("add", sub_matches)) => {
            let tasks = sub_matches.get_many::<String>("tasks").unwrap_or_default().map(|v| v.to_string()).collect::<Vec<_>>();
            for task in tasks {
                add_todo(&conn, task).unwrap();
            }
        }
        Some(("del", sub_matches)) => {
            let ids = sub_matches.get_many::<String>("ids").unwrap_or_default().map(|v| v.trim().parse::<usize>().unwrap()).collect::<Vec<_>>();
            delete_todo(&conn, ids).unwrap();

        }
        Some(("done", sub_matches)) => {
            let ids = sub_matches.get_many::<String>("ids").unwrap_or_default().map(|v| v.trim().parse::<usize>().unwrap()).collect::<Vec<_>>();
            for id in ids {
                done_todo(&conn, id).unwrap();
            }
        }
        _ => unreachable!("This code shouldn't be reached under the right condition")
    }
}
