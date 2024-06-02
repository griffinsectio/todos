use rusqlite::Connection;
use crate::Todo;
use std::error::Error;
use colored::Colorize;

pub fn list_todo(conn: &Connection) -> Result<(), Box<dyn Error>> {
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
