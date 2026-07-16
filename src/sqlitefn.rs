use rusqlite::*;

use crate::todoitem;
use todoitem::*;
//so basicly we will make
// create db fn[*]
pub fn connect_db(path_: &str) -> Result<Connection> {
    let conn = Connection::open(path_)?;

    conn.execute(
        r#"CREATE TABLE IF NOT EXISTS "Task" (
	"id" INTEGER NOT NULL UNIQUE,
	"Name" TEXT,
	"descreption" TEXT,
	"status" TEXT,
	PRIMARY KEY("id")
	);"#,
        (),
    )?;
    Ok(conn)
}
// add task fn  []
pub fn add_task(item: &Item, conn: Connection) -> Result<Connection> {
    conn.execute(
        "INSERT INTO Task (Name,descreption,status) VALUES (?1,?2,?3)",
        (
            item.get_item_info(2),
            item.get_item_info(3),
            item.get_item_info(4),
        ),
    )?;

    Ok(conn)
}
// delete task fn[]
// update status []
// get task []
// get all taskes []
// delete all tasks []
// search for task []
