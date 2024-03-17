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
    // create task table
    // TODO: add more columns
    // prior 
    // subtask
    // task-level
    db.execute(
        "CREATE TABLE IF NOT EXISTS task (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL
        )",
        // add status column
        [],
    )?;

    // create commit table
    // db.execute(
    //     "CREATE TABLE IF NOT EXISTS commit (
    //         id INTEGER PRIMARY KEY,
    //         task_id INTEGER NOT NULL
    //     )",
    //     []
    // )?;
    // // create topic table
    // db.execute(
    //     "CREATE TABLE IF NOT EXISTS topic (
    //         id INTEGER PRIMARY KEY,
    //         name TEXT NOT NULL,
    //         parent_id INTEGER
    //     )",
    //     []
    // )?;
    // create project table
    return Ok(());
}

pub struct Task {
    pub id: Option<i32>,
    pub name: String,
}

pub fn create_task(task: &Task) -> Result<()> {
    let db = connect()?;
    db.execute(
        "INSERT INTO task (name) VALUES (?1)",
        [&task.name],
    )?;
    return Ok(());
}

pub fn retrive_task() -> Result<Vec<Task>> {
    let db = connect()?;
    let mut stmt = db.prepare("SELECT id, name FROM task")?;
    let task_iter = stmt.query_map([], |row| {
        Ok(Task {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    })?;
    let mut res = Vec::new();
    for task in task_iter {
        res.push(task.unwrap());
    }
    return Ok(res);
}