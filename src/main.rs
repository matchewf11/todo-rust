use chrono::Local;
use clap::{Parser, Subcommand};
use todo::Conn;

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
    List {
        /// Sort By Category
        #[arg(short, long, action = clap::ArgAction::SetTrue)]
        category: bool,

        /// Include Finshed Tasks
        #[arg(short, long, action = clap::ArgAction::SetTrue)]
        include_done: bool,
    },

    /// Edit todo list item
    #[command(arg_required_else_help = true)]
    Edit {
        /// Id of task to finish
        id: i32,

        /// Set if problems is done with true or false
        #[arg(short, long)]
        finish: Option<bool>,

        /// Set problem due date
        #[arg(short, long)]
        due_date: Option<String>,

        /// Set problem category
        #[arg(short, long)]
        category: Option<String>,

        /// Set problem info
        #[arg(short, long)]
        info: Option<String>,

        /// Remove problem (use with caution, must write 'delete')
        #[arg(short, long, action = clap::ArgAction::SetTrue)]
        remove: bool,
    },
}

fn main() {
    let today = Local::now().date_naive();
    println!("Welcome to todo: {today}");

    let conn = Conn::build("./todo.db").unwrap_or_else(|err| {
        eprintln!("Could not acess db: {err}");
        std::process::exit(1)
    });

    match Cli::parse().command {
        Commands::Add {
            task,
            category,
            due_date,
        } => println!("Add command"),
        Commands::List {
            category,
            include_done,
        } => println!("List command"),
        Commands::Edit {
            id,
            finish,
            due_date,
            category,
            info,
            remove,
        } => println!("Edit command"),
    }

    println!("Operation was a Success")
}
