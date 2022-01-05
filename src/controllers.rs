// use crate::types;
// use crate::types::Todo;
// use rusqlite::{params, Connection};
//
// pub fn create_todo_table(conn: &Connection) -> rusqlite::Result<usize> {
//     conn.execute(
//         "CREATE TABLE IF NOT EXISTS todo (
//                   id                   INTEGER PRIMARY KEY,
//                   completed            BOOL DEFAULT FALSE,
//                   text                 TEXT NOT NULL,
//                   created_at           DATE DEFAULT (datetime('now','localtime')),
//                   updated_at           DATE DEFAULT (datetime('now','localtime'))
//                   );",
//         [],
//     )
// }
//
// pub fn add_todo(conn: &Connection, todo: &str) -> rusqlite::Result<usize> {
//     conn.execute("INSERT INTO todo (text) VALUES (?);", [todo])
// }
//
// pub fn remove_todo(conn: &Connection, id: u64) -> rusqlite::Result<usize> {
//     conn.execute("DELETE FROM todo WHERE id = ?;", [id])
// }
//
// pub fn edit_todo(conn: &Connection, id: u64, todo: &str) -> rusqlite::Result<usize> {
//     conn.execute("UPDATE todo SET text = ? WHERE id = ?", params![todo, id])
// }
//
// pub fn mark_todo(conn: &Connection, id: u64, done: bool) -> rusqlite::Result<usize> {
//     conn.execute(
//         "UPDATE todo SET completed = ? WHERE id = ?",
//         params![if done { 1 } else { 0 }, id,],
//     )
// }
//
// pub fn mark_todo_complete(conn: &Connection, id: u64) -> rusqlite::Result<usize> {
//     mark_todo(conn, id, true)
// }
//
// pub fn mark_todo_incomplete(conn: &Connection, id: u64) -> rusqlite::Result<usize> {
//     mark_todo(conn, id, false)
// }
//
// pub fn list(conn: &Connection) -> rusqlite::Result<Vec<Todo>> {
//     let mut stmt = conn.prepare("SELECT id, completed, text, created_at, updated_at FROM todo")?;
//     let x = stmt
//         .query_map([], |row| {
//             Ok(types::Todo {
//                 id: row.get(0)?,
//                 completed: row.get(1)?,
//                 text: row.get(2)?,
//                 created_at: row.get(3)?,
//                 updated_at: row.get(4)?,
//             })
//         })?
//         .collect();
//     x
// }
