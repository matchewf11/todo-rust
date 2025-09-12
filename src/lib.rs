use rusqlite::{Connection, Error, Result};

pub fn init_db(db_path: &str) -> Result<Connection> {
    let conn = Connection::open(db_path)?;

    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS categories (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE CHECK(name != '')
        )"#,
        (),
    )?;

    conn.execute(
        r#"CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY,
            info TEXT NOT NULL UNIQUE CHECK(info != ''),
            done BOOLEAN NOT NULL DEFAULT false CHECK(done in (0, 1)),
            due_date TEXT CHECK(
                due_date IS NULL OR
                (due_date GLOB '[0-9][0-9][0-9][0-9]-[0-1][0-9]-[0-3][0-9]' AND
                    date(due_date) IS NOT NULL)
            ),
            category INTEGER,
            FOREIGN KEY(category) REFERENCES categories(id)
        )"#,
        (),
    )?;

    Ok(conn)
}

pub fn edit_task(conn: &Connection, id: i32, finish: bool) -> Result<()> {
    // fix this method up!!!!

    if !finish {
        return Ok(());
    }
    let rows_updated = conn.execute("UPDATE tasks SET done = true WHERE id = ?1", [&id])?;

    if rows_updated == 0 {
        return Err(Error::QueryReturnedNoRows);
    }
    Ok(())
}
