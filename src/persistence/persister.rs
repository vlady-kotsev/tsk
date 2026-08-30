use std::{
    sync::{Arc, mpsc::Receiver},
    thread::{self, JoinHandle},
};

use color_eyre::Result;
use rusqlite::Connection;
use uuid::Uuid;

use crate::models::{BoardState, TaskState};

const DB_PATH: &str = "src/persistence/db/db.sqlite3";

const CREATE_SCHEMA: &str = include_str!("sql/schema.sql");
const CREATE_BOARD_SQL: &str = include_str!("sql/create_board.sql");
const DELETE_BOARD_SQL: &str = include_str!("sql/delete_board.sql");
const CREATE_TASK_SQL: &str = include_str!("sql/create_task.sql");
const DELETE_TASK_SQL: &str = include_str!("sql/delete_task.sql");
const GET_ALL_BOARDS_SQL: &str = include_str!("sql/get_all_boards.sql");
const GET_TASKS_BY_BOARD_SQL: &str = include_str!("sql/get_tasks_by_board.sql");
/*



#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

fn main() -> Result<()> {


    conn.execute(
        "CREATE TABLE person (
            id    INTEGER PRIMARY KEY,
            name  TEXT NOT NULL,
            data  BLOB
        )",
        (), // empty list of parameters.
    )?;
    let me = Person {
        id: 0,
        name: "Steven".to_string(),
        data: None,
    };
    conn.execute(
        "INSERT INTO person (name, data) VALUES (?1, ?2)",
        (&me.name, &me.data),
    )?;

    let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
    let person_iter = stmt.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            data: row.get(2)?,
        })
    })?;

    for person in person_iter {
        println!("Found person {:?}", person.unwrap());
    }
    Ok(())
}
*/

pub enum PersistMessage {
    CreateTask(Uuid, Arc<TaskState>),
    DeleteTask(Uuid),
    CreateBoard(Arc<BoardState>),
    DeleteBoard(Uuid),
    QuitPersister,
}

struct Persister {
    db: Connection,

    persist_chan: Receiver<PersistMessage>,
}

impl Persister {
    fn new(persist_chan: Receiver<PersistMessage>) -> Result<Self> {
        let db = Connection::open(DB_PATH)?;
        let persister = Self { db, persist_chan };
        persister.init_db()?;
        Ok(persister)
    }

    fn init_db(&self) -> Result<()> {
        self.db.execute_batch(CREATE_SCHEMA)?;
        Ok(())
    }

    fn persist_board(&mut self, board: Arc<BoardState>) -> Result<()> {
        self.db
            .execute(CREATE_BOARD_SQL, (board.id().to_string(), board.title()))?;
        Ok(())
    }

    fn persist_task(&mut self, board_id: Uuid, task: Arc<TaskState>) -> Result<()> {
        self.db.execute(
            CREATE_TASK_SQL,
            (task.id().to_string(), task.content(), board_id.to_string()),
        )?;
        Ok(())
    }

    fn erase_board(&mut self, board_id: Uuid) -> Result<()> {
        self.db.execute(DELETE_BOARD_SQL, (board_id.to_string(),))?;
        Ok(())
    }

    fn erase_task(&mut self, task_id: Uuid) -> Result<()> {
        self.db.execute(DELETE_TASK_SQL, (task_id.to_string(),))?;
        Ok(())
    }
}

pub fn load_boards() -> Result<Vec<BoardState>> {
    let db = Connection::open(DB_PATH)?;
    db.execute_batch(CREATE_SCHEMA)?;

    let mut boards = Vec::new();
    {
        let mut board_stmt = db.prepare(GET_ALL_BOARDS_SQL)?;
        let mut board_rows = board_stmt.query(())?;
        while let Some(row) = board_rows.next()? {
            let id: String = row.get(0)?;
            let title: String = row.get(1)?;
            boards.push((Uuid::parse_str(&id)?, title));
        }
    }

    let mut task_stmt = db.prepare(GET_TASKS_BY_BOARD_SQL)?;
    let mut result = Vec::with_capacity(boards.len());
    for (board_id, title) in boards {
        let mut tasks = Vec::new();
        let mut task_rows = task_stmt.query((board_id.to_string(),))?;
        while let Some(row) = task_rows.next()? {
            let id: String = row.get(0)?;
            let content: String = row.get(1)?;
            tasks.push(TaskState::from_parts(Uuid::parse_str(&id)?, content));
        }
        result.push(BoardState::from_parts(board_id, title, tasks));
    }

    Ok(result)
}

pub fn run(persist_chan: Receiver<PersistMessage>) -> Result<JoinHandle<Result<()>>> {
    let mut persister = Persister::new(persist_chan)?;
    let handle = thread::spawn(move || -> Result<()> {
        loop {
            let msg = persister.persist_chan.recv()?;

            match msg {
                PersistMessage::CreateTask(board_id, task_state) => {
                    persister.persist_task(board_id, task_state)?
                }
                PersistMessage::DeleteTask(task_id) => persister.erase_task(task_id)?,
                PersistMessage::CreateBoard(board_state) => persister.persist_board(board_state)?,
                PersistMessage::DeleteBoard(board_id) => persister.erase_board(board_id)?,
                PersistMessage::QuitPersister => break,
            }
        }
        Ok(())
    });
    Ok(handle)
}

#[cfg(test)]
mod scratch_verify_tests {
    use super::*;

    #[test]
    fn scratch_load_boards_creates_schema() {
        let boards = load_boards().expect("load_boards failed");
        assert!(boards.is_empty());
    }
}
