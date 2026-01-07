use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod brain;
mod catalyst;
mod commands;
mod config;
mod context;
mod embeddings;
mod git_ops;
mod heuristics;
mod history;
mod llm;
mod memory;
mod planning;
mod scaffolding;
mod session;
mod state;
mod templating;
mod watcher;

#[derive(Parser)]
#[command(name = "nexus")]
#[command(about = "CLI tool for Obsidian-based project management", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new project from template
    Init {
        /// Name of the project to create
        project_name: String,
        /// Mode: "sprint" (default) or "adhoc"
        #[arg(long, default_value = "sprint")]
        mode: String,
    },
    /// Check if planning documents are complete and ready
    Gate {
        /// Path to the project directory
        project_path: PathBuf,
    },
    /// Generate CLAUDE.md from completed planning documents
    Unlock {
        /// Path to the project directory
        project_path: PathBuf,
    },
    /// Create a new sprint branch with scoped context
    Sprint {
        /// Path to the project directory
        project_path: PathBuf,
        /// Sprint number to activate
        sprint_number: u32,
    },
    /// Start an interactive shell (REPL)
    Shell,
    /// Start working on an ad-hoc task
    TaskStart {
        /// Path to the project directory
        project_path: PathBuf,
    },
    /// Mark an ad-hoc task as completed
    TaskDone {
        /// Path to the project directory
        project_path: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { project_name, mode } => {
            if let Err(e) = commands::init::execute(&project_name, &mode) {
                eprintln!("{e}");
                std::process::exit(1);
            }
        }
        Commands::Gate { project_path } => {
            if let Err(e) = commands::gate::execute(&project_path) {
                eprintln!("{e}");
                std::process::exit(1);
            }
        }
        Commands::Unlock { project_path } => {
            if let Err(e) = commands::unlock::execute(&project_path) {
                eprintln!("{e}");
                std::process::exit(1);
            }
        }
        Commands::Sprint {
            project_path,
            sprint_number,
        } => {
            if let Err(e) = commands::sprint::execute(&project_path, sprint_number) {
                eprintln!("{e}");
                std::process::exit(1);
            }
        }
        Commands::Shell => {
            if let Err(e) = commands::shell::execute() {
                eprintln!("{e}");
                std::process::exit(1);
            }
        }
        Commands::TaskStart { project_path } => {
            if let Err(e) = commands::task::execute_start(&project_path) {
                eprintln!("{e}");
                std::process::exit(1);
            }
        }
        Commands::TaskDone { project_path } => {
            if let Err(e) = commands::task::execute_done(&project_path) {
                eprintln!("{e}");
                std::process::exit(1);
            }
        }
    }
}
