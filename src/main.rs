use chrono::{Datelike, Local, NaiveDate};
use clap::{Parser, Subcommand};
use rusqlite::Result;
use std::fmt;
use std::process;

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
        finish: bool,
        // edit thh category, info, un-done, due-date
    },
}

fn main() {
    let conn = todo::init_db("./todo.db").unwrap_or_else(|err| {
        println!("Could not initalize db connection: {err}");
        process::exit(1)
    });

    match Cli::parse().command {
        Commands::Add {
            task,
            category,
            due_date,
        } => add_task(&conn, &task, &category, &due_date).unwrap_or_else(|err| {
            println!("Could not add task: {err}");
            process::exit(1)
        }),
        Commands::List => {
            get_tasks(&conn)
                .unwrap_or_else(|err| {
                    println!("Could not get tasks: {err}");
                    process::exit(1)
                })
                .iter()
                .for_each(|t| println!("{t}"));
        }
        Commands::Edit { id, finish } => todo::edit_task(&conn, id, finish).unwrap_or_else(|err| {
            println!("Could not edit task: {err}");
            process::exit(1)
        }),
    }
}

fn add_task(
    conn: &rusqlite::Connection,
    task: &str,
    category: &Option<String>,
    due_date: &Option<String>,
) -> rusqlite::Result<()> {
    match (category, due_date) {
        (None, None) => {
            conn.execute("INSERT INTO tasks (info) VALUES (?1)", [&task])?;
        }
        (Some(c), None) => {
            conn.execute("INSERT INTO tasks (info, category) VALUES (?1, (SELECT id FROM categories WHERE name = ?2))", [task, c])?;
        }
        (None, Some(d)) => {
            let formatted_date = format_date(d, &Local::now().date_naive()).unwrap();
            conn.execute(
                "INSERT INTO tasks (info, due_date) VALUES (?1, ?2)",
                [task, &formatted_date],
            )?;
        }
        (Some(c), Some(d)) => {
            let formatted_date = format_date(d, &Local::now().date_naive()).unwrap();
            conn.execute(
               "INSERT INTO tasks (info, due_date, category) VALUES (?1, ?2, (SELECT id FROM categories WHERE name = ?3))",
               [task, &formatted_date, c],
           )?;
        }
    }

    Ok(())
}

struct Task {
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

fn get_tasks(conn: &rusqlite::Connection) -> rusqlite::Result<Vec<Task>> {
    conn.prepare(
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
    )?
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

// clean this up later

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

// include env arguments for customazation or should i have
// a lua/toml file
