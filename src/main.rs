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

// time
// chrono

// combine add to add category

// i want to accept 11 (assume it is <today if it is 11>, the next 11) 2-11 or 02-11 (assume it is the next feb 11) 2025-2-11 or 25-2-11 or (abosultue day) all of these strings in rust and i want to parse it into sqlite date format also accept / instead of -

fn main() {
    let conn = Connection::open("./todo.db").unwrap();

    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS categories (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE CHECK(name != '')
        )",
        (),
    )
    .unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY,
            info TEXT NOT NULL UNIQUE CHECK(info != ''),
            done BOOLEAN NOT NULL DEFAULT false CHECK(done in (0, 1)),
            category INTEGER,
            FOREIGN KEY(category) REFERENCES categories(id)
        )",
        (),
    )
    .unwrap();

    match Cli::parse().command {
        Commands::Add { task, category } => {
            let category = category.unwrap_or("No Category".to_string());
            conn.execute(
                "INSERT OR IGNORE INTO categories (name) VALUES (?1)",
                [&category],
            )
            .unwrap();
            conn.execute(
                "INSERT INTO tasks (info, category) VALUES (?1, (SELECT id FROM categories WHERE name = ?2))",
                [&task, &category],
            )
            .unwrap();
        }
        Commands::List => {
            struct Task {
                id: u32,
                info: String,
                category: String,
            }
            let mut stmt = conn
                .prepare(
                    "
                    SELECT
                        tasks.id,
                        tasks.info,
                        categories.name
                    FROM tasks
                    LEFT JOIN categories ON tasks.category = categories.id
                    WHERE done == false",
                )
                .unwrap();
            let task_iter = stmt
                .query_map((), |row| {
                    Ok(Task {
                        id: row.get(0).unwrap(),
                        info: row.get(1).unwrap(),
                        category: row.get(2).unwrap(),
                    })
                })
                .unwrap();
            println!("Tasks:");
            for task in task_iter {
                let task = task.unwrap();
                println!("{}: {}: {}", task.id, task.category, task.info)
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
