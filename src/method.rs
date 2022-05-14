//! # todo crate
//! todo_crate is a collection of utility to operate todo table.

use rusqlite::{params, Connection, Result};
use std::io;

#[derive(Debug)]
pub struct Todolist {
    id: i32,
    todo: String,
    priority: u8,
}

/// add todo item to the Sqlite DB.
pub fn add(conn: &Connection) -> Result<()> {
    println!("input todo text:");
    let mut buffer = String::new();
    let stdin = io::stdin();
    match stdin.read_line(&mut buffer) {
        Err(_) => panic!(),
        _ => (),
    }
    let todo = buffer.trim_end();
    println!("input priority:");
    let mut priority = String::new();
    match stdin.read_line(&mut priority) {
        Err(_) => panic!(),
        _ => (),
    }
    let priority = priority.trim_end().parse::<u8>().unwrap();
    conn.execute(
        "INSERT INTO todolist (todo, priority) VALUES (?1, ?2)",
        params![todo, priority],
    )?;
    println!("--TODO LIST--");
    show(&conn)?;
    Ok(())
}
/// delete todo item form the SQlite DB.
pub fn complete(conn: &Connection) -> Result<()> {
    println!("--TODO LIST--");
    show(&conn)?;
    println!("input complete todo ID:");
    let mut buffer = String::new();
    let stdin = io::stdin();
    match stdin.read_line(&mut buffer) {
        Err(_) => panic!(),
        _ => (),
    }
    let id = match buffer.trim_end().parse::<u8>() {
        Err(_) => panic!("input integer"),
        Ok(id) => id,
    };
    conn.execute("DELETE FROM todolist WHERE id = ?1", params![id])?;

    println!("--TODO LIST--");
    show(&conn)?;
    Ok(())
}

/// Show all todo items of the DB table.
pub fn show(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT id, todo, priority FROM todolist")?;
    let todolist_iter = stmt.query_map([], |row| {
        Ok(Todolist {
            id: row.get(0)?,
            todo: row.get(1)?,
            priority: row.get(2)?,
        })
    })?;
    let mut todolist = todolist_iter.collect::<Vec<Result<Todolist>>>();
    todolist.sort_by(|a, b| {
        a.as_ref()
            .unwrap()
            .priority
            .partial_cmp(&b.as_ref().unwrap().priority)
            .unwrap()
    });
    println!("ID TODO PRIORITY");
    for item in todolist {
        println!(
            "{}, {}, {}",
            item.as_ref().unwrap().id,
            item.as_ref().unwrap().todo,
            item.as_ref().unwrap().priority
        );
    }
    println!();
    Ok(())
}

/// Drop todolist table from Sqlite DB.
pub fn reset(conn: &Connection) -> Result<()> {
    conn.execute("DROP TABLE todolist", params![])?;
    Ok(())
}
