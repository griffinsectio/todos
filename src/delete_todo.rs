use crate::list_todo::list_todo;
use rusqlite::Connection;
use crate::Todo;
use std::error::Error;

pub fn delete_todo(conn: &Connection, ids: Vec<usize>) -> Result<(), Box<dyn Error>> {
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