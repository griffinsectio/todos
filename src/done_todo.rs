use crate::list_todo::list_todo;
use rusqlite::Connection;
use std::error::Error;

pub fn done_todo(conn: &Connection, id: usize) -> Result<(), Box<dyn Error>> {
    let done_command_str = format!("UPDATE todos SET done='true' WHERE id={}", id);
    conn.execute(&done_command_str, ())?;

    list_todo(conn)?;
    Ok(())
}