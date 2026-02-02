mod domain;
mod ports;
mod adapters;
mod application;

use adapters::cli::ConsoleOutput;
use adapters::storage::DirectoryStorage;
use adapters::sync::GitRefSync;
use anyhow::Result;
use application::{AddYak, DoneYak, EditContext, ListYaks, MoveYak, PruneYaks, RemoveYak, ShowContext, SyncYaks};
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
    Add {
        /// The yak name (space-separated words)
        name: Vec<String>,
    },
    /// List yaks
    #[command(alias = "ls")]
    List,
    /// Mark yak as done
    #[command(alias = "finish")]
    Done {
        /// The yak name (space-separated words)
        name: Vec<String>,
        #[arg(long)]
        undo: bool,
    },
    /// Remove a yak
    #[command(alias = "rm")]
    Remove {
        /// The yak name (space-separated words)
        name: Vec<String>,
    },
    /// Remove all done yaks
    Prune,
    /// Move/rename a yak
    #[command(alias = "mv")]
    Move { from: String, to: String },
    /// Edit or show yak context
    Context {
        /// The yak name (space-separated words)
        name: Vec<String>,
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
            let name_str = name.join(" ");
            let use_case = AddYak::new(&storage, &output);
            use_case.execute(&name_str)
        }
        Commands::List => {
            let use_case = ListYaks::new(&storage, &output);
            use_case.execute()
        }
        Commands::Done { name, undo } => {
            let name_str = name.join(" ");
            let use_case = DoneYak::new(&storage, &output);
            use_case.execute(&name_str, undo)
        }
        Commands::Remove { name } => {
            let name_str = name.join(" ");
            let use_case = RemoveYak::new(&storage, &output);
            use_case.execute(&name_str)
        }
        Commands::Prune => {
            let use_case = PruneYaks::new(&storage, &output);
            use_case.execute()
        }
        Commands::Move { from, to } => {
            let use_case = MoveYak::new(&storage, &output);
            use_case.execute(&from, &to)
        }
        Commands::Context { name, show } => {
            let name_str = name.join(" ");
            if show {
                let use_case = ShowContext::new(&storage, &output);
                use_case.execute(&name_str)
            } else {
                let use_case = EditContext::new(&storage, &output);
                use_case.execute(&name_str)
            }
        }
        Commands::Sync => {
            let sync = GitRefSync::new()?;
            let use_case = SyncYaks::new(&sync, &output);
            use_case.execute()
        }
    }
}
