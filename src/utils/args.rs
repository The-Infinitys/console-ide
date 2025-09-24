use std::path::PathBuf;

use clap::{Parser, Subcommand};

/// Console IDE
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to the file or directory to open
    pub path: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Option<Subcommands>,
}

#[derive(Subcommand, Debug)]
pub enum Subcommands {
    /// Run tests
    Test,
    /// Get information about an extension
    Info {
        #[command(subcommand)]
        command: InfoCommands,
    },
    /// Execute an extension command
    Exec {
        #[command(subcommand)]
        command: ExecCommands,
    },
}

#[derive(Subcommand, Debug)]
pub enum InfoCommands {
    /// Get relation information for an extension
    Relation,
}

#[derive(Subcommand, Debug)]
pub enum ExecCommands {
    /// Run the background process for an extension
    BackgroundProcess,
}
