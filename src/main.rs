use chrono::NaiveDate;
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

        /// Due date of the task: YYYY/MM/DD or YY/MM/DD
        #[arg(short, long)]
        due_date: Option<String>,
    },

    /// List all todo items
    List, // make it list categoies too?

    /// Edit todo list item
    #[command(arg_required_else_help = true)]
    Edit {
        /// Id of task to finish
        id: i32,

        /// Problem is finished
        #[arg(short, long, action = clap::ArgAction::SetTrue)]
        finished: bool,
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
            due_date TEXT,
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
                let formatted_date = format_date(&d).unwrap();
                conn.execute(
                    "INSERT INTO tasks (info, due_date) VALUES (?1, ?2)",
                    [&task, &formatted_date],
                )
                .unwrap();
            }
            (Some(c), Some(d)) => {
                let formatted_date = format_date(&d).unwrap();
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
                        tasks.category = categories.id"#,
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
                    "{} : {} : {} : {:?} : {:?}",
                    task.id, task.info, task.done, task.due_date, task.category
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

fn format_date(date: &str) -> Option<String> {
    let cleaned = date.trim().replace("/", "-");
    let parts: Vec<_> = cleaned.trim().split('-').collect();

    match parts.len() {
        3 => {
            // YYYY-MM-DD
            // YYYY-M-D
            // YYYY-M-DD
            // YYYY-MM-D
            if let Ok(date) = NaiveDate::parse_from_str(&cleaned, "%Y-%m-%d") {
                return Some(date.format("%Y-%m-%d").to_string());
            }

            if let Ok(year) = parts[0].parse::<i32>() {
                if year > 0 && year < 100 {
                    let full_year = 2000 + year;

                    let month = match parts[1].parse::<u32>() {
                        Ok(m) => m,
                        Err(_) => return None,
                    };

                    let day = match parts[2].parse::<u32>() {
                        Ok(d) => d,
                        Err(_) => return None,
                    };

                    // YY-MM-DD
                    // YY-M-DD
                    // YY-MM-D
                    // YY-M-D
                    if let Some(date) = NaiveDate::from_ymd_opt(full_year, month, day) {
                        return Some(date.format("%Y-%m-%d").to_string());
                    }
                } else {
                    return None;
                }
            }
            None
        }
        _ => None, // handle 2 and 1
    }

    // 2025-09-10 21:13:14.960340156 -07:00
    // ["2025", "09", "10"]
    // let now = Local::now();
    // println!("{}", now);
    // let parts: Vec<_> = cleaned.split('-').collect();
    // println!("{:?}", parts);
    // i want to make all of these into YYYY-MM-DD (if things are ommited assume that they are the
    // next instanse of that time(assume it is today if possible))
    // let date = match parts.len() {
    //     // D or DD
    //     1 => "",
    //     // MM-DD
    //     // M-D
    //     // M-DD
    //     // MM-D
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
