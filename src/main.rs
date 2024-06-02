use clap::{value_parser, Arg, Command};
use colored::Colorize;
use rusqlite::{Connection, Result};
use std::error::Error;

#[derive(Debug, Clone)]
struct Todo {
    id: i64,
    task: String,
    done: String,
}

fn add_todo(conn: &Connection, task: String) -> Result<(), Box<dyn Error>> {
    let mut stmt = conn.prepare("SELECT id, task, done FROM todos")?;
    let todo_vec = stmt.query_map([], |row| {
        Ok(Todo {
            id: row.get(0)?,
            task: row.get(1)?,
            done: row.get(2)?,
        })
    })?;
    
    let next_id = todo_vec.count() + 1;

    let new_todo = Todo {
        id: next_id as i64,
        task: task,
        done: "false".to_string(),
    };

    conn.execute (
        "INSERT INTO todos (task, done) VALUES (?1, ?2)",
        (&new_todo.task, &new_todo.done),
    )?;
    
    list_todo(conn)?;
    Ok(())
}

fn delete_todo(conn: &Connection, ids: Vec<usize>) -> Result<(), Box<dyn Error>> {
    let mut stmt = conn.prepare("SELECT * FROM todos")?;

    let mut rows = stmt.query([])?;
    let mut todos_rows = Vec::new();
    while let Some(row) = rows.next()? {
        todos_rows.push(Todo {
            id: row.get(0)?,
            task: row.get(1)?,
            done: row.get(2)?,
        })
    }

    conn.execute("DELETE FROM todos", ())?;

    for row in todos_rows {
        if !ids.contains(&(row.id as usize)) {
            conn.execute("INSERT INTO todos (task, done) VALUES (?1, ?2)", 
            (&row.task, &row.done)
            )?;
        } else {
            continue;
        }
    };

    list_todo(conn)?;
    Ok(())
}

fn done_todo(conn: &Connection, id: usize) -> Result<(), Box<dyn Error>> {
    let done_command_str = format!("UPDATE todos SET done='true' WHERE id={}", id);
    conn.execute(&done_command_str, ())?;

    list_todo(conn)?;
    Ok(())
}

fn list_todo(conn: &Connection) -> Result<(), Box<dyn Error>> {
    let mut stmt = conn.prepare("SELECT id, task, done FROM todos")?;
    let todo_iter = stmt.query_map([], |row| {
        Ok(Todo {
            id: row.get(0)?,
            task: row.get(1)?,
            done: row.get(2)?,

        })
    })?;

    let no_todos_message  = 
    r#"     _________________________________________ 
    / You've got no todos, maybe it's time to \
    \ relax ;)                                /
     ----------------------------------------- 
            \   ^__^
             \  (oo)\_______
                (__)\       )\/\
                    ||----w |
                    ||     ||"#;

    let todo_iter_vec = todo_iter.into_iter().map(|v| v).collect::<Vec<_>>();

    for todo in &todo_iter_vec {
        let id = todo.as_ref().unwrap().id;
        let task = todo.as_ref().unwrap().task.trim_end().to_string();
        let done = todo.as_ref().unwrap().done.to_string();

        if done == "true" {
            println!("{}. {}", id, task.blue().bold().strikethrough());
        } else {
            println!("{}. {}", id, task);
        }
    }

    if todo_iter_vec.len() == 0 as usize {
        println!("{}", no_todos_message.green().bold());
    }
    Ok(())
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
