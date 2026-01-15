use clap::{Parser, Subcommand};
use std::path::PathBuf;

use doc_vault::organize_directory;

/// Academic file organization CLI
#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Organize files inside a directory
    Organize {
        /// Path to the directory (e.g. ~/Downloads)
        path: PathBuf,

        /// Simulate actions without modifying files
        #[arg(long)]
        dry_run: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Organize { path, dry_run } => {
            if !path.exists() {
                eprintln!("Error: The provided path does not exist.");
                std::process::exit(1);
            }

            if let Err(e) = organize_directory(&path, dry_run) {
                eprintln!("Failed to organize directory: {}", e);
                std::process::exit(1);
            }

            if dry_run {
                println!("Dry run completed. No files were modified.");
            } else {
                println!("Directory successfully organized.");
            }
        }
    }
}

