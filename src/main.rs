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
    List,

    /// Finish todo list item
    #[command(arg_required_else_help = true)]
    Done {
        /// Id of task to finish
        id: i32,
    },

    /// Add a category
    #[command(arg_required_else_help = true)]
    AddCat {
        /// Category to add
        category: String,
    },

    /// List categories
    ListCat,

    /// Remove a category
    #[command(arg_required_else_help = true)]
    RemoveCat {
        /// Category to add
        category: String,
    },
}

// combine add to add category

fn main() {
    let conn = Connection::open_in_memory().expect("Could not open connection");
    conn.execute(
        "
        CREATE TABLE categories (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE CHECK(name != '')
        )",
        (),
    )
    .expect("Was not able to create tables");
    conn.execute(
        "CREATE TABLE tasks (
            id INTEGER PRIMARY KEY,
            info TEXT NOT NULL UNIQUE CHECK(info != ''),
            done BOOLEAN NOT NULL DEFAULT false CHECK(done in (0, 1)),
            category INTEGER,
            FOREIGN KEY(category) REFERENCES categories(id)
        )",
        (),
    )
    .expect("Was not able to create tables");

    match Cli::parse().command {
        Commands::Add { task, category } => {}
        Commands::List => {}
        Commands::Done { id } => {}
        Commands::AddCat { category } => {}
        Commands::ListCat => {}
        Commands::RemoveCat { category } => {}
    }

    // match Cli::parse().command {
    //     Commands::Add { task } => {
    //         // TODO: insert the category too?
    //         conn.execute("INSERT INTO tasks (info) VALUES (?1)", [&task])
    //             .expect("Was not able to insert task");
    //         println!("Adding task: {task}")
    //     }
    //     Commands::List => {
    //         struct Task {
    //             id: u32,
    //             info: String,
    //         }
    //         let mut stmt = conn
    //             // TODO: PRINT THE cat too
    //             .prepare("SELECT id, info FROM tasks WHERE done = false")
    //             .unwrap();
    //         let task_iter = stmt
    //             .query_map([], |row| {
    //                 Ok(Task {
    //                     id: row.get(0).unwrap(),
    //                     info: row.get(1).unwrap(),
    //                 })
    //             })
    //             .unwrap();
    //
    //         println!("Tasks:");
    //         for task in task_iter {
    //             let task = task.unwrap();
    //             println!("{}: {}", task.id, task.info)
    //         }
    //     }
    //     Commands::Done { id } => {
    //         let mut stmt = conn
    //             .prepare("UPDATE tasks SET done = true WHERE id = ?1 RETURNING info")
    //             .unwrap();
    //         let task_info: String = stmt.query_row([id], |row| Ok(row.get(0).unwrap())).unwrap();
    //         println!("Finished task: \"{task_info}\"")
    //     }
    //     Commands::AddCat { category } => {
    //         conn.execute("INSERT INTO categories (name) VALUES (?1)", [&category])
    //             .unwrap();
    //         println!("Inserting cat: {category}")
    //     }
    //     Commands::ListCat => {
    //         // fix this query??
    //         let mut stmt = conn.prepare("SELECT name FROM tasks").unwrap();
    //         let cat_iter = stmt.query_map([], |row| Ok(row.get(0).unwrap())).unwrap();
    //         println!("Categories:");
    //         for cat in cat_iter {
    //             let cat: String = cat.unwrap();
    //             println!("- {}", cat)
    //         }
    //     }
    //     Commands::RemoveCat { category: _ } => {
    //         // todo: remove category
    //         // set all problems with this cat to null
    //     }
    // }
    //
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
