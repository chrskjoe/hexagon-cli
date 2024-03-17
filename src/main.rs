use std::path::PathBuf;
use clap::{Args, Parser, Subcommand};
use hexagon::{create_task, init, Task};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    Create(CreateArgs),// task or project or topic
    List(ListArgs),
}

#[derive(Debug, Args)]
struct CreateArgs {
    #[command(subcommand)]
    command: Option<CreateCommands>
}

#[derive(Debug, Subcommand)]
enum CreateCommands {
    Task {
        #[arg(short, long)]
        name: Option<String>
    },
    Project {
        name: Option<String>
    },
    Topic {
        name: Option<String>
    }
}

#[derive(Debug, Args)]
struct ListArgs {
    #[arg(short, long)]
    all: bool,
    #[command(subcommand)]
    command: Option<ListCommands>
}

#[derive(Debug, Subcommand)]
enum ListCommands {
    Task,
    Project,
    Topic   
}

fn main() {
    // parse command line arguments
    let cli = Cli::parse();

    // You can check the value provided by positional arguments, or option arguments
    if let Some(name) = cli.name.as_deref() {
        println!("Value for name: {name}");
    }

    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Init) => {
            println!("Initializing database");
            match init() {
                Ok(_) => println!("Database initialized"),
                Err(e) => println!("Error initializing database: {}", e)   
            }
        }
        Some(Commands::Create(CreateArgs { command })) => {
            match command {
                Some(CreateCommands::Task { name }) => {
                    println!("Creating task: {:?}", name);
                    let name = name.as_ref().unwrap();
                    let _ = create_task(&Task { name: name.to_string(), id: None });
                }
                Some(CreateCommands::Project { name }) => {
                    println!("Creating project: {:?}", name);
                }
                Some(CreateCommands::Topic { name }) => {
                    println!("Creating topic: {:?}", name);
                }
                None => println!("No subcommand provided")
            }
        }
        Some(Commands::List(ListArgs { all, command })) => {
            match command {
                Some(ListCommands::Task) => {
                    println!("Listing tasks");
                    let tasks = hexagon::retrive_task().unwrap();
                    for task in tasks {
                        println!("Task {:?} : {:?}",task.id, task.name);
                    }
                }
                Some(ListCommands::Project) => {
                    println!("Listing projects");
                }
                Some(ListCommands::Topic) => {
                    println!("Listing topics");
                }
                None => println!("No subcommand provided")
            }
        }
        &None => todo!()
    }

    // Continued program logic goes here...
    // sub commands can be handled here as well
}
