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
    },
    /// List all todo items
    List,

    /// Remove todo list item
    #[command(arg_required_else_help = true)]
    Remove {
        /// Id of task to remove
        id: i32,
    },
}

struct Task {
    id: u32,
    info: String,
}

fn main() {
    let conn = Connection::open_in_memory().expect("Could not open connection");
    conn.execute(
        // NOT NULL
        // UNIQUE
        // CHECK
        // DEFAULT
        "CREATE TABLE tasks (
            id INTEGER PRIMARY KEY,
            info TEXT NOT NULL UNIQUE CHECK(info != ''),
            deleted BOOLEAN NOT NULL DEFAULT false CHECK(deleted in (0, 1))
        )",
        (),
    )
    .expect("Was not able to create table");

    // for testing
    [
        "Buy groceries",
        "Clean the house",
        "Finish Rust project",
        "Read a book",
        "Go for a run",
    ]
    .iter()
    .for_each(|task| {
        conn.execute("INSERT INTO tasks (info) VALUES (?1)", [task])
            .expect("Could not insert task");
    });

    match Cli::parse().command {
        Commands::Add { task } => {
            conn.execute("INSERT INTO tasks (info) VALUES (?1)", [&task])
                .expect("Was not able to insert task");
            println!("Adding task: {task}")
        }
        Commands::List => {
            let mut stmt = conn
                .prepare("SELECT id, info FROM tasks WHERE deleted = false")
                .unwrap();
            let task_iter = stmt
                .query_map([], |row| {
                    Ok(Task {
                        id: row.get(0).unwrap(),
                        info: row.get(1).unwrap(),
                    })
                })
                .unwrap();

            println!("Tasks:");
            for task in task_iter {
                let task = task.unwrap();
                println!("{}: {}", task.id, task.info)
            }
        }
        Commands::Remove { id } => {
            let mut stmt = conn
                .prepare("UPDATE tasks SET deleted = true WHERE id = ?1 RETURNING info")
                .unwrap();
            let task_info: String = stmt.query_row([id], |row| Ok(row.get(0).unwrap())).unwrap();
            println!("Deleted task: \"{task_info}\"")
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

// Continued program logic goes here...

// let person_iter = stmt.query_map([], |row| {
//     Ok(Person {
//         id: row.get(0)?,
//         name: row.get(1)?,
//         data: row.get(2)?,
//     })
// })?;
// for person in person_iter {
//     println!("Found person {:?}", person?);
// }
