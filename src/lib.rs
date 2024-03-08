use rusqlite::{Connection, Result};
use std::path::{self};

pub fn connect() -> Result<Connection> {
    println!("Connecting to database...");
    let p = path::Path::new("hexagon.sqlite3");
    let db = Connection::open(p)?;
    return Ok(db);
}

pub fn init() -> Result<()> {
    let db = connect()?;
    db.execute(
        "CREATE TABLE IF NOT EXISTS task (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL
        )",
        [],
    )?;
    return Ok(());
}