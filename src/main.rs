use chrono::{Datelike, Local, NaiveDate};
use clap::{Parser, Subcommand};
use rusqlite::Connection;

///A command line todo app
#[derive(Debug, Parser)]
#[command(name = "todo")]
#[command(about = "A command line todo app", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Add todo list item
    #[command(arg_required_else_help = true)]
    Add {
        /// Task to add
        task: String,

        ///  Category of the task
        #[arg(short, long)]
        category: Option<String>,

        /// Due date: YYYY-MM-DD, MM-DD, DD (slashes allowed, leading zeros optional)
        #[arg(short, long)]
        due_date: Option<String>,
    },

    /// List all todo items
    List, // list by category or due_date or exclude done stuff?

    /// Edit todo list item
    #[command(arg_required_else_help = true)]
    Edit {
        /// Id of task to finish
        id: i32,

        /// Problem is finished
        #[arg(short, long, action = clap::ArgAction::SetTrue)]
        finished: bool,
        // edit thh category, info, un-done, due-date
    },
}

fn main() {
    let conn = Connection::open("./todo.db").unwrap();

    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS categories (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE CHECK(name != '')
        )"#,
        (),
    )
    .unwrap();

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
    )
    .unwrap();

    match Cli::parse().command {
        Commands::Add {
            task,
            category,
            due_date,
        } => match (category, due_date) {
            (None, None) => {
                conn.execute("INSERT INTO tasks (info) VALUES (?1)", [&task])
                    .unwrap();
            }
            (Some(c), None) => {
                conn.execute(
                        "INSERT INTO tasks (info, category) VALUES (?1, (SELECT id FROM categories WHERE name = ?2))",
                        [&task, &c]
                    )
                    .unwrap();
            }
            (None, Some(d)) => {
                let formatted_date = format_date(&d, &Local::now().date_naive()).unwrap();
                conn.execute(
                    "INSERT INTO tasks (info, due_date) VALUES (?1, ?2)",
                    [&task, &formatted_date],
                )
                .unwrap();
            }
            (Some(c), Some(d)) => {
                let formatted_date = format_date(&d, &Local::now().date_naive()).unwrap();
                conn.execute(
                        "INSERT INTO tasks (info, due_date, category) VALUES (?1, ?2, (SELECT id FROM categories WHERE name = ?3))",
                        [&task, &formatted_date, &c],
                    )
                    .unwrap();
            }
        },
        Commands::List => {
            struct Task {
                id: u32,
                info: String,
                done: bool,
                due_date: Option<String>,
                category: Option<String>,
            }

            let mut stmt = conn
                .prepare(
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
                        tasks.due_date IS NULL,
                        tasks.due_date
                    "#,
                )
                .unwrap();

            let task_iter = stmt
                .query_map((), |row| {
                    Ok(Task {
                        id: row.get(0)?,
                        info: row.get(1)?,
                        done: row.get(2)?,
                        due_date: row.get(3)?,
                        category: row.get(4)?,
                    })
                })
                .unwrap();

            println!("Tasks:");
            for task in task_iter {
                let task = task.unwrap();
                println!(
                    "{}. {}{}{}{}",
                    task.id,
                    task.info,
                    if task.done {
                        " | Finished"
                    } else {
                        " | Incomplete"
                    },
                    if let Some(d) = task.due_date {
                        format!(" | Due: {}", d)
                    } else {
                        String::new()
                    },
                    if let Some(c) = task.category {
                        format!(" | Category: {}", c)
                    } else {
                        String::new()
                    },
                )
            }
        }
        Commands::Edit { id, finished } => {
            if finished {
                let rows_changed = conn
                    .execute("UPDATE tasks SET done = true WHERE id = ?1", [&id])
                    .unwrap();
                if rows_changed == 0 {
                    println!("Did not edit anything")
                }
            } else {
                println!("Did not edit anything")
            }
        }
    }
}

// format input into YYYY-MM-DD
fn format_date(date: &str, today: &NaiveDate) -> Result<String, &'static str> {
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

// Diff {
//     #[arg(value_name = "COMMIT")]
//     base: Option<OsString>,
//     #[arg(value_name = "COMMIT")]
//     head: Option<OsString>,
//     #[arg(last = true)]
//     path: Option<OsString>,
//     #[arg(
//         long,
//         require_equals = true,
//         value_name = "WHEN",
//         num_args = 0..=1,
//         default_value_t = ColorWhen::Auto,
//         default_missing_value = "always",
//         value_enum
//     )]
//     color: ColorWhen,
// },
// /// pushes things
// #[command(arg_required_else_help = true)]
// Push {
//     /// The remote to target
//     remote: String,
// },
// /// adds things
// #[command(arg_required_else_help = true)]
// Add {
//     /// Stuff to add
//     #[arg(required = true)]
//     path: Vec<PathBuf>,
// },
// Stash(StashArgs),
// #[command(external_subcommand)]
// External(Vec<OsString>),
// #[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
// enum ColorWhen {
//     Always,
//     Auto,
//     Never,
// }
// impl std::fmt::Display for ColorWhen {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         self.to_possible_value()
//             .expect("no values are skipped")
//             .get_name()
//             .fmt(f)
//     }
// }
// #[derive(Debug, Args)]
// #[command(args_conflicts_with_subcommands = true)]
// #[command(flatten_help = true)]
// struct StashArgs {
//     #[command(subcommand)]
//     command: Option<StashCommands>,
//
//     #[command(flatten)]
//     push: StashPushArgs,
// }
//
// #[derive(Debug, Subcommand)]
// enum StashCommands {
//     Push(StashPushArgs),
//     Pop { stash: Option<String> },
//     Apply { stash: Option<String> },
// }
//
// #[derive(Debug, Args)]
// struct StashPushArgs {
//     #[arg(short, long)]
//     message: Option<String>,
// }
// match args.command {
//     Commands::Diff {
//         mut base,
//         mut head,
//         mut path,
//         color,
//     } => {
//         if path.is_none() {
//             path = head;
//             head = None;
//             if path.is_none() {
//                 path = base;
//                 base = None;
//             }
//         }
//         let base = base
//             .as_deref()
//             .map(|s| s.to_str().unwrap())
//             .unwrap_or("stage");
//         let head = head
//             .as_deref()
//             .map(|s| s.to_str().unwrap())
//             .unwrap_or("worktree");
//         let path = path.as_deref().unwrap_or_else(|| OsStr::new(""));
//         println!(
//             "Diffing {}..{} {} (color={})",
//             base,
//             head,
//             path.to_string_lossy(),
//             color
//         );
//     }
//     Commands::Push { remote } => {
//         println!("Pushing to {remote}");
//     }
//     Commands::Add { path } => {
//         println!("Adding {path:?}");
//     }
//     Commands::Stash(stash) => {
//         let stash_cmd = stash.command.unwrap_or(StashCommands::Push(stash.push));
//         match stash_cmd {
//             StashCommands::Push(push) => {
//                 println!("Pushing {push:?}");
//             }
//             StashCommands::Pop { stash } => {
//                 println!("Popping {stash:?}");
//             }
//             StashCommands::Apply { stash } => {
//                 println!("Applying {stash:?}");
//             }
//         }
//     }
//     Commands::External(args) => {
//         println!("Calling out to {:?} with {:?}", &args[0], &args[1..]);
//     }
// }
//look into chrono for this
//read env for time zones change time to my time

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn make_today(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn test_full_dates() {
        let today = make_today(2025, 9, 11);

        // Already future full date
        assert_eq!(format_date("2025-10-15", &today).unwrap(), "2025-10-15");

        // Past full date should still parse, but your logic might move to next future
        assert_eq!(format_date("2025-09-10", &today).unwrap(), "2025-09-10");

        // 2-digit year
        assert_eq!(format_date("25-12-01", &today).unwrap(), "2025-12-01");

        // 1-digit year (assuming your code handles this as 200Y)
        assert_eq!(format_date("5-12-01", &today).unwrap(), "2005-12-01");
    }

    #[test]
    fn test_month_day() {
        let today = make_today(2025, 9, 11);

        // Later this month
        assert_eq!(format_date("09-15", &today).unwrap(), "2025-09-15");

        // Earlier in month -> next year
        assert_eq!(format_date("09-01", &today).unwrap(), "2026-09-01");

        // December -> same year
        assert_eq!(format_date("12-25", &today).unwrap(), "2025-12-25");
    }

    #[test]
    fn test_day_only() {
        let today = make_today(2025, 9, 11);

        // Later day this month
        assert_eq!(format_date("15", &today).unwrap(), "2025-09-15");

        // Earlier day -> next month
        assert_eq!(format_date("01", &today).unwrap(), "2025-10-01");

        // Same day -> today
        assert_eq!(format_date("11", &today).unwrap(), "2025-09-11");

        // End-of-year rollover
        let dec_today = make_today(2025, 12, 31);
        assert_eq!(format_date("01", &dec_today).unwrap(), "2026-01-01");
    }

    #[test]
    fn test_invalid_dates() {
        let today = make_today(2025, 9, 11);

        // Invalid month
        assert!(format_date("2025-13-01", &today).is_err());

        // Invalid day
        assert!(format_date("2025-02-30", &today).is_err());

        // Non-numeric
        assert!(format_date("abcd", &today).is_err());

        // Empty string
        assert!(format_date("", &today).is_err());
    }

    #[test]
    fn test_slash_separator() {
        let today = make_today(2025, 9, 11);

        // Full date with slashes
        assert_eq!(format_date("2025/10/12", &today).unwrap(), "2025-10-12");

        // Month/day with slashes
        assert_eq!(format_date("10/15", &today).unwrap(), "2025-10-15");
    }
}
