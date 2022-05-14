mod method;
use clap::Parser;
use rusqlite::{Connection, Result};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Method name: add or complete or show or reset
    #[clap(short, long)]
    method: String,
}

fn main() -> Result<()> {
    let path = "./my_db.db3";
    let conn = Connection::open(&path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todolist (
            id  INTEGER PRIMARY KEY,
            todo TEXT NOT NULL,
            priority INTEGER
        )",
        [],
    )?;

    let args = Args::parse();
    match &*args.method {
        "add" => method::add(&conn)?,
        "complete" => method::complete(&conn)?,
        "show" => method::show(&conn)?,
        "reset" => method::reset(&conn)?,
        _ => panic!(),
    };

    Ok(())
}
