use crate::list_todo::list_todo;
use rusqlite::Connection;
use crate::Todo;
use std::error::Error;

pub fn add_todo(conn: &Connection, task: String) -> Result<(), Box<dyn Error>> {
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