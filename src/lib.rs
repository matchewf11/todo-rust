use chrono::{Datelike, Local, NaiveDate};
use rusqlite::{Connection, Result};
use std::fmt;

pub struct Task {
    id: u32,
    info: String,
    done: bool,
    due_date: Option<String>,
    category: Option<String>,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}. {} | {} | {:?} | {:?}",
            self.id, self.info, self.done, self.due_date, self.category,
        )
    }
}

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

pub fn edit_task(
    conn: &Connection,
    id: i32,
    finish: Option<bool>,
    due_date: Option<&str>,
    category: Option<&str>,
    info: Option<&str>,
    remove: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(f) = finish {
        conn.execute(
            "UPDATE tasks SET done = ?1 WHERE id = ?2",
            rusqlite::params![&f, &id],
        )?;
    }

    if let Some(i) = info {
        conn.execute(
            "UPDATE tasks SET info = ?1 WHERE id = ?2",
            rusqlite::params![i, &id],
        )?;
    }

    if let Some(d) = due_date {
        let fmt_date = format_date(d, &Local::now().date_naive())?;
        conn.execute(
            "UPDATE tasks SET due_date = ?1 WHERE id = ?2",
            rusqlite::params![&fmt_date, &id],
        )?;
    }
    if let Some(c) = category {
        conn.execute(
            r#"
        UPDATE tasks
        SET category = (SELECT id FROM categories WHERE name = ?1)
        WHERE id = ?2
        "#,
            rusqlite::params![c, &id],
        )?;
    }

    if remove {
        conn.execute("DELETE FROM tasks WHERE id = ?1", [&id])?;
    }

    Ok(())
}

pub fn get_tasks(conn: &Connection, sort_by_cat: bool, include_done: bool) -> Result<Vec<Task>> {
    conn.prepare(match (sort_by_cat, include_done) {
        (false, false) => {
            r#"
        SELECT
            tasks.id,
            tasks.info,
            tasks.done,
            tasks.due_date,
            categories.name
        FROM tasks
        LEFT JOIN
            categories
        ON
            tasks.category = categories.id
        WHERE
            tasks.done = false
        ORDER BY
            tasks.due_date IS NULL,
            tasks.due_date
        "#
        }
        (true, false) => {
            r#"
        SELECT
            tasks.id,
            tasks.info,
            tasks.done,
            tasks.due_date,
            categories.name
        FROM tasks
        LEFT JOIN
            categories
        ON
            tasks.category = categories.id
        WHERE
            tasks.done = false
        ORDER BY
            categories.id,
            tasks.due_date IS NULL,
            tasks.due_date
        "#
        }
        (false, true) => {
            r#"
        SELECT
            tasks.id,
            tasks.info,
            tasks.done,
            tasks.due_date,
            categories.name
        FROM tasks
        LEFT JOIN
            categories
        ON
            tasks.category = categories.id
        ORDER BY
            tasks.done,
            tasks.due_date IS NULL,
            tasks.due_date
        "#
        }
        (true, true) => {
            r#"
        SELECT
            tasks.id,
            tasks.info,
            tasks.done,
            tasks.due_date,
            categories.name
        FROM tasks
        LEFT JOIN
            categories
        ON
            tasks.category = categories.id
        ORDER BY
            tasks.done,
            categories.id,
            tasks.due_date IS NULL,
            tasks.due_date
        "#
        }
    })?
    .query_map((), |row| {
        Ok(Task {
            id: row.get(0)?,
            info: row.get(1)?,
            done: row.get(2)?,
            due_date: row.get(3)?,
            category: row.get(4)?,
        })
    })?
    .collect()
}

pub fn add_task(
    conn: &rusqlite::Connection,
    task: &str,
    category: Option<&str>,
    due_date: Option<&str>,
) -> rusqlite::Result<()> {
    match (category, due_date) {
        (None, None) => {
            conn.execute("INSERT INTO tasks (info) VALUES (?1)", [&task])
        }
        (Some(c), None) => {
            conn.execute("INSERT INTO tasks (info, category) VALUES (?1, (SELECT id FROM categories WHERE name = ?2))", [task, c])
        }
        (None, Some(d)) => {
            let formatted_date = format_date(d, &Local::now().date_naive()).unwrap();
            conn.execute(
                "INSERT INTO tasks (info, due_date) VALUES (?1, ?2)",
                [task, &formatted_date],
            )
        }
        (Some(c), Some(d)) => {
            let formatted_date = format_date(d, &Local::now().date_naive()).unwrap();
            conn.execute(
               "INSERT INTO tasks (info, due_date, category) VALUES (?1, ?2, (SELECT id FROM categories WHERE name = ?3))",
               [task, &formatted_date, c],
           )
        }
    }.map(|_| ())
}

pub fn format_date(date: &str, today: &NaiveDate) -> Result<String, &'static str> {
    let cleaned = date.trim().replace("/", "-");
    let parts: Vec<_> = cleaned.trim().split('-').collect();

    fn parse_part(part: &str, msg: &'static str) -> Result<u32, &'static str> {
        part.parse().map_err(|_| msg)
    }

    let (y, m, d) = match parts.len() {
        3 => (
            Some(parse_part(parts[0], "Y-M-D invalid year")?),
            Some(parse_part(parts[1], "Y-M-D invalid month")?),
            Some(parse_part(parts[2], "Y-M-D invalid day")?),
        ),
        2 => (
            None,
            Some(parse_part(parts[0], "M-D invalid month")?),
            Some(parse_part(parts[1], "M-D invalid day")?),
        ),
        1 => (None, None, Some(parse_part(parts[0], "D invalid day")?)),
        _ => return Err("could not parse"),
    };

    fn make_date(y: i32, m: u32, d: u32) -> Result<String, &'static str> {
        if let Some(date) = NaiveDate::from_ymd_opt(y, m, d) {
            Ok(date.format("%Y-%m-%d").to_string())
        } else {
            Err("unable to parse date")
        }
    }

    match (y, m, d) {
        // wont work past 2100 :(, will fix then
        (Some(year), Some(month), Some(day)) => make_date(
            (if year < 100 { year + 2000 } else { year }) as i32,
            month,
            day,
        ),
        (None, Some(month), Some(day)) => {
            if month > today.month() || (month == today.month() && day >= today.day()) {
                make_date(today.year(), month, day)
            } else {
                make_date(today.year() + 1, month, day)
            }
        }
        (None, None, Some(day)) => {
            if day >= today.day() {
                make_date(today.year(), today.month(), day)
            } else {
                let today_month = today.month();
                let (new_year, new_month) = if today_month == 12 {
                    (today.year() + 1, 1)
                } else {
                    (today.year(), today_month + 1)
                };
                make_date(new_year, new_month, day)
            }
        }
        _ => Err("cannot parse the date"),
    }
}

//look into chrono for this
//read env for time zones change time to my time
// include env arguments for customazation or should i have
// a lua/toml file
