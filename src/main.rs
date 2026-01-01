use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod commands;
mod config;
mod heuristics;
mod planning;

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
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { project_name } => {
            if let Err(e) = commands::init::execute(&project_name) {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
        Commands::Gate { project_path } => {
            if let Err(e) = commands::gate::execute(&project_path) {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
        Commands::Unlock { project_path } => {
            println!("Unlock command called for: {}", project_path.display());
            // TODO: Implement unlock logic
        }
        Commands::Sprint {
            project_path,
            sprint_number,
        } => {
            println!(
                "Sprint command called for: {} (Sprint {})",
                project_path.display(),
                sprint_number
            );
            // TODO: Implement sprint logic
        }
    }
}
