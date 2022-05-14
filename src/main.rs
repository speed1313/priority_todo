use clap::Parser;
use rusqlite::{params, Connection, Result};
use std::fs;
use std::io;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long)]
    method: String,

    /// Number of times to greet
    #[clap(short, long, default_value_t = 1)]
    count: u8,
}

#[derive(Debug)]
struct Todolist{
    id: i32,
    todo: String,
    priority: u8,
}

fn main()->Result<()> {
    let conn = Connection::open_in_memory()?;
    conn.execute(
        "CREATE TABLE todolist (
            id  INTEGER PRIMARY KEY,
            todo TEXT NOT NULL,
            priority INTEGER
        )",
        [],
    )?;

    let me = Todolist{
        id: 0,
        todo: "Watch Hikakin TV".to_string(),
        priority: 5,
    };
    conn.execute(
        "INSERT INTO todolist (todo, priority) VALUES (?1, ?2)",
        params![me.todo, me.priority],
    )?;


    let args = Args::parse();
    match &*args.method {
        "add" => add(&conn)?,
        "complete" => complete(&conn)?,
        "show" => show(&conn)?,
        _ => panic!(),
    };
    let mut stmt = conn.prepare("SELECT id, todo, priority FROM todolist")?;
    let todolist_iter = stmt.query_map([], |row| {
        Ok(Todolist{
            id: row.get(0)?,
            todo: row.get(1)?,
            priority: row.get(2)?,
        })
    })?;
    let mut todolist = todolist_iter.collect::<Vec<Result<Todolist>>>();
    todolist.sort_by(|a,b| a.as_ref().unwrap().priority.partial_cmp(&b.as_ref().unwrap().priority).unwrap());

    for item in todolist{
        println!("TODO :{}, {}", item.as_ref().unwrap().todo, item.as_ref().unwrap().priority);
    }

    Ok(())
}

fn add(conn: &Connection) -> Result<()> {
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    stdin.read_line(&mut buffer);
    let todo = buffer.trim_end();
    let mut priority = String::new();
    stdin.read_line(&mut priority);
    let priority = priority.trim_end().parse::<u8>().unwrap();
    conn.execute(
        "INSERT INTO todolist (todo, priority) VALUES (?1, ?2)",
        params![todo, priority],
    )?;

    Ok(())
}

fn complete(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT id, todo, priority FROM todolist")?;
    let todolist_iter = stmt.query_map([], |row| {
        Ok(Todolist{
            id: row.get(0)?,
            todo: row.get(1)?,
            priority: row.get(2)?,
        })
    })?;
    let mut todolist = todolist_iter.collect::<Vec<Result<Todolist>>>();
    todolist.sort_by(|a,b| a.as_ref().unwrap().priority.partial_cmp(&b.as_ref().unwrap().priority).unwrap());

    for item in todolist{
        println!("TODO :{}, {}", item.as_ref().unwrap().todo, item.as_ref().unwrap().priority);
    }
    Ok(())
}

fn show(conn: &Connection) -> Result<()> {
    Ok(())
}
