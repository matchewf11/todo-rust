// use chrono::NaiveDate;
// use rusqlite::{Connection, Result};
//
// pub struct TodoConn {
//     client: Connection,
// }
//
// impl TodoConn {
//     pub fn build(db_path: &str) -> Result<Self> {
//         let conn = Connection::open(db_path)?;
//         conn.execute(
//             r#"
//             CREATE TABLE IF NOT EXISTS categories (
//                 id INTEGER PRIMARY KEY AUTOINCREMENT,
//                 name TEXT NOT NULL UNIQUE CHECK(name != '')
//             )
//             "#,
//             (),
//         )?;
//         conn.execute(
//             r#"
//             CREATE TABLE IF NOT EXISTS tasks (
//                 id INTEGER PRIMARY KEY,
//                 info TEXT NOT NULL UNIQUE CHECK(info != ''),
//                 done BOOLEAN NOT NULL DEFAULT false CHECK(done in (0, 1)),
//                 due_date TEXT CHECK(
//                     due_date IS NULL OR
//                     (due_date GLOB '[0-9][0-9][0-9][0-9]-[0-1][0-9]-[0-3][0-9]' AND
//                         date(due_date) IS NOT NULL)
//             ),
//             category INTEGER,
//             FOREIGN KEY(category) REFERENCES categories(id)
//             )"#,
//             (),
//         )?;
//         Ok(TodoConn { client: conn })
//     }
//
//     pub fn add_task(
//         &self,
//         task: &str,
//         category: Option<&str>,
//         due_date: Option<&str>,
//     ) -> Result<()> {
//         Ok(())
//         // insert category if not there
//         //
//         // match (category, due_date) {
//         //     (None, None) => {
//         //         conn.execute("INSERT INTO tasks (info) VALUES (?1)", [&task])
//         //     }
//         //     (Some(c), None) => {
//         //         conn.execute("INSERT INTO tasks (info, category) VALUES (?1, (SELECT id FROM categories WHERE name = ?2))", [task, c])
//         //     } (None, Some(d)) => {
//         //         let formatted_date = format_date(d, &Local::now().date_naive()).unwrap();
//         //         conn.execute(
//         //             "INSERT INTO tasks (info, due_date) VALUES (?1, ?2)",
//         //             [task, &formatted_date],
//         //         )
//         //     }
//         //     (Some(c), Some(d)) => {
//         //         let formatted_date = format_date(d, &Local::now().date_naive()).unwrap();
//         //         conn.execute(
//         //            "INSERT INTO tasks (info, due_date, category) VALUES (?1, ?2, (SELECT id FROM categories WHERE name = ?3))",
//         //            [task, &formatted_date, c],
//         //        )
//         //     }
//         // }.map(|_| ())
//     }
//
//     pub fn get_task(&self, sort_by_cat: bool, include_done: bool) -> Result<Vec<String>> {
//         // should be tasks instead of string
//         Ok(vec![])
//         // let mut sql = String::from(
//         //     r#"
//         //     SELECT tasks.id,
//         //         tasks.info,
//         //         tasks.done,
//         //         tasks.due_date,
//         //         categories.name
//         //     FROM tasks
//         //     LEFT JOIN
//         //         categories
//         //     ON
//         //         tasks.category = categories.id
//         //     "#,
//         // );
//         // if !include_done {
//         //     sql.push_str("WHERE tasks.done = false\n");
//         // }
//         // sql.push_str("ORDER BY\n");
//         // if include_done {
//         //     sql.push_str("tasks.done, \n");
//         // }
//         // if sort_by_cat {
//         //     sql.push_str("categories.id, \n");
//         // }
//         // sql.push_str("tasks.due_date IS NULL, tasks.due_date");
//         // conn.prepare(&sql)?
//         //     .query_map((), |row| {
//         //         Ok(Task {
//         //             id: row.get(0)?,
//         //             info: row.get(1)?,
//         //             done: row.get(2)?,
//         //             due_date: row.get(3)?,
//         //             category: row.get(4)?,
//         //         })
//         //     })?
//         //     .collect()
//         // }
//         //
//     }
//
//     // make sure all the function signatures look good
//
//     pub fn edit_task(
//         &self,
//         id: i32,
//         finish: Option<bool>,
//         due_date: Option<&str>,
//         category: Option<&str>,
//         info: Option<&str>,
//         remove: bool,
//     ) -> Result<(), Box<dyn std::error::Error>> {
//         Ok(())
//         // ) -> Result<(), Box<dyn std::error::Error>> {
//         //     Ok(())
//         //
//         //     //     // insert the category if it is not there
//         //     //
//         //     //     if let Some(f) = finish {
//         //     //         conn.execute(
//         //     //             "UPDATE tasks SET done = ?1 WHERE id = ?2",
//         //     //             rusqlite::params![&f, &id],
//         //     //         )?;
//         //     //     }
//         //     //
//         //     //     if let Some(i) = info {
//         //     //         conn.execute(
//         //     //             "UPDATE tasks SET info = ?1 WHERE id = ?2",
//         //     //             rusqlite::params![i, &id],
//         //     //         )?;
//         //     //     }
//         //     //
//         //     //     if let Some(d) = due_date {
//         //     //         let fmt_date = format_date(d, &Local::now().date_naive())?;
//         //     //         conn.execute(
//         //     //             "UPDATE tasks SET due_date = ?1 WHERE id = ?2",
//         //     //             rusqlite::params![&fmt_date, &id],
//         //     //         )?;
//         //     //     }
//         //     //     if let Some(c) = category {
//         //     //         conn.execute(
//         //     //             r#"
//         //     //         UPDATE tasks
//         //     //         SET category = (SELECT id FROM categories WHERE name = ?1)
//         //     //         WHERE id = ?2
//         //     //         "#,
//         //     //             rusqlite::params![c, &id],
//         //     //         )?;
//         //     //     }
//         //     //
//         //     //     if remove {
//         //     //         conn.execute("DELETE FROM tasks WHERE id = ?1", [&id])?;
//         //     //     }
//         //     //
//         //     //     Ok(())
//         // }
//         //
//     }
// }
//
// // test good case
// // test bad case
// // test today
// // tests - and /
// // test tmr
// // tests for next month
// // tests for next year
// // test absolute
// // tests for 1->4 years like each digit
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     fn test_date() -> NaiveDate {
//         NaiveDate::from_ymd_opt(2025, 9, 20).unwrap()
//     }
//
//     fn assert_fmt(input: &str, expected: &str) {
//         assert_eq!(Ok(expected.to_string()), format_date(input, &test_date()));
//     }
//
//     #[test]
//     fn test_format_date_identity() {
//         assert_fmt("2025-09-20", "2025-09-20")
//     }
//
//     #[test]
//     fn test_format_date_trim() {
//         assert_fmt("\t  2025-09-20 \n ", "2025-09-20")
//     }
//
//     #[test]
//     fn test_format_date_delimeter() {
//         assert_fmt("2025/09/20", "2025-09-20")
//     }
//
//     // #[test]
//     // fn test_format_date_one_part() {
//     //     assert!(format_date("20", &test_date()).is_ok())
//     // }
//     //
//     // #[test]
//     // fn test_format_date_two_part() {
//     //     assert!(format_date("9-20", &test_date()).is_ok())
//     // }
//     //
//     // #[test]
//     // fn test_format_date_four_part() {
//     //     assert_eq!(
//     //         Err("bad number of parts"),
//     //         format_date("20-20-20-20", &test_date()),
//     //     )
//     // }
// }
//
// pub fn format_date(date: &str, today: &NaiveDate) -> Result<String, &'static str> {
//     let cleaned = date.trim().replace("/", "-");
//     let parts: Vec<_> = cleaned.trim().split('-').collect();
//
//     // let parse_part = |part: &str, msg| part.parse::<u32>().map_err(|_| msg);
//     // let (y, m, d) = match parts.len() {
//     //     3 => (
//     //         Some(parse_part(parts[0], "Y-M-D invalid year")?),
//     //         Some(parse_part(parts[1], "Y-M-D invalid month")?),
//     //         Some(parse_part(parts[2], "Y-M-D invalid day")?),
//     //     ),
//     //     2 => (
//     //         None,
//     //         Some(parse_part(parts[0], "M-D invalid month")?),
//     //         Some(parse_part(parts[1], "M-D invalid day")?),
//     //     ),
//     //     1 => (None, None, Some(parse_part(parts[0], "D invalid day")?)),
//     //     _ => return Err("bad number of parts"),
//     // };
//     //
//     Ok(cleaned)
//
//     // fn make_date(y: i32, m: u32, d: u32) -> Result<String, &'static str> {
//     //     if let Some(date) = NaiveDate::from_ymd_opt(y, m, d) {
//     //         Ok(date.format("%Y-%m-%d").to_string())
//     //     } else {
//     //         Err("unable to parse date")
//     //     }
//     // }
//     //
//     // match (y, m, d) {
//     //     // wont work past 2100 :(, will fix then
//     //     (Some(year), Some(month), Some(day)) => make_date(
//     //         (if year < 100 { year + 2000 } else { year }) as i32,
//     //         month,
//     //         day,
//     //     ),
//     //     (None, Some(month), Some(day)) => {
//     //         if month > today.month() || (month == today.month() && day >= today.day()) {
//     //             make_date(today.year(), month, day)
//     //         } else {
//     //             make_date(today.year() + 1, month, day)
//     //         }
//     //     }
//     //     (None, None, Some(day)) => {
//     //         if day >= today.day() {
//     //             make_date(today.year(), today.month(), day)
//     //         } else {
//     //             let today_month = today.month();
//     //             let (new_year, new_month) = if today_month == 12 {
//     //                 (today.year() + 1, 1)
//     //             } else {
//     //                 (today.year(), today_month + 1)
//     //             };
//     //             make_date(new_year, new_month, day)
//     //         }
//     //     }
//     //     _ => Err("cannot parse the date"),
//     // }
// }
//
// // //look into chrono for this
// // //read env for time zones change time to my time
// // // include env arguments for customazation or should i have
// // // a lua/toml file
// //
// //
// // test all these functions
// // cargo doc with tests also make a custom connection that is a Connection
// // under the hood, only can make it with tables
// // make all the task methods on them
// // pub struct Task {
// //     id: u32,
// //     info: String,
// //     done: bool,
// //     due_date: Option<String>,
// //     category: Option<String>,
// // }
// //
// // impl Display for Task {
// //     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
// //         write!(
// //             f,
// //             "{}. {} | {} | {:?} | {:?}",
// //             self.id, self.info, self.done, self.due_date, self.category,
// //         )
// //     }
// // }
// //
// // test good case and bad case when entering into the db
// // make custom err type
// //rust cli to a supabase db
// // makee sujre to add category
