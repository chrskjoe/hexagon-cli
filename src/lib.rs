use rusqlite::{Connection, Result};
use std::path::{self};

pub fn connect() -> Result<Connection> {
    println!("Connecting to database...");
    let p = path::Path::new("hexagon.sqlite3");
    let db = Connection::open(p)?;
let path = db.path(); 
    println!("Connected to database: {:?}", path.unwrap());
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

/*
only one parent, multiple children
// TODO: add parent_id
// TODO: add  println
*/
pub struct Topic {
    pub id: Option<i32>,
    pub name: String, 
    pub parent_id: Option<i32>,
}


pub struct TopicTree {
    pub level: i32, // level in the current tree
    pub id: Option<i32>,
    pub name: String,
    pub children: Vec<TopicTree>,
    pub parents: Box<TopicTree>,
}

pub fn create_topic(name: String, parent_id: Option<i32>) -> Result<()> {
    let db = connect()?;
    match parent_id {
        Some(parent_id) => {
            // check if parent_id exists
            // let parent_topic = db.query_row( 
            //     "SELECT * FROM topic WHERE id = ?1",
            //     [parent_id],
            //     |row| { Ok(Topic { id: row.get(0)?, name: row.get(1)?, parent_id: row.get(2)? })} 
            // )?;            
            // println!("Parent topic: {:?}", parent_topic.name);

            let mut stmt = db.prepare("SELECT EXISTS(SELECT 1 FROM topic WHERE id = ?1 LIMIT 1)")?;
            let exists: bool = stmt.query_row([parent_id], |row| row.get(0))?;
            // return error if parent_id doesnot exist
            if !exists {
                return Err(rusqlite::Error::QueryReturnedNoRows);
            }
            // insert to database
            db.execute(
                "INSERT INTO topic (name, parent_id) VALUES (?1, ?2)",
                [name, parent_id.to_string()],
            )?;
        }
        None => {
            println!("Create root topic: {:?}", name);
            db.execute(
                "INSERT INTO topic (name) VALUES (?1)",
                [name],
            )?;
        }
        
    }

    return Ok(());
}

pub fn retrive_topics() -> Result<Vec<Topic>> {
    let db = connect()?;
    let mut stmt = db.prepare("SELECT * FROM topic")?;
    let topic_iter = stmt.query_map([], |row| {
        Ok(Topic {
            id: row.get(0)?,
            name: row.get(1)?,
            parent_id: row.get(2)?,
        })
    })?;
    let mut res = Vec::new();
    for topic in topic_iter {
        res.push(topic.unwrap());
    }
    return Ok(res);
}