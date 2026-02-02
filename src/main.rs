mod domain;
mod ports;
mod adapters;
mod application;

use adapters::cli::ConsoleOutput;
use adapters::storage::DirectoryStorage;
use anyhow::Result;
use application::{AddYak, DoneYak, ListYaks};
use clap::Parser;

/// DAG-based TODO list CLI for software teams
#[derive(Parser, Debug)]
#[command(name = "yx")]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Parser, Debug)]
enum Commands {
    /// Add a new yak
    Add { name: String },
    /// List yaks
    #[command(alias = "ls")]
    List,
    /// Mark yak as done
    #[command(alias = "finish")]
    Done {
        name: String,
        #[arg(long)]
        undo: bool,
    },
    /// Remove a yak
    #[command(alias = "rm")]
    Remove { name: String },
    /// Remove all done yaks
    Prune,
    /// Move/rename a yak
    #[command(alias = "mv")]
    Move { from: String, to: String },
    /// Edit or show yak context
    Context {
        name: String,
        #[arg(long)]
        show: bool,
    },
    /// Sync yaks with git refs
    Sync,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize adapters
    let storage = DirectoryStorage::new()?;
    let output = ConsoleOutput;

    match cli.command {
        Commands::Add { name } => {
            let use_case = AddYak::new(&storage, &output);
            use_case.execute(&name)
        }
        Commands::List => {
            let use_case = ListYaks::new(&storage, &output);
            use_case.execute()
        }
        Commands::Done { name, undo } => {
            let use_case = DoneYak::new(&storage, &output);
            use_case.execute(&name, undo)
        }
        Commands::Remove { name } => {
            println!("TODO: Remove yak '{}'", name);
            Ok(())
        }
        Commands::Prune => {
            println!("TODO: Prune done yaks");
            Ok(())
        }
        Commands::Move { from, to } => {
            println!("TODO: Move yak '{}' to '{}'", from, to);
            Ok(())
        }
        Commands::Context { name, show } => {
            if show {
                println!("TODO: Show context for '{}'", name);
            } else {
                println!("TODO: Edit context for '{}'", name);
            }
            Ok(())
        }
        Commands::Sync => {
            println!("TODO: Sync yaks");
            Ok(())
        }
    }
}
