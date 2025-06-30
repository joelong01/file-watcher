//! Command Line Interface (CLI) module
//!
//! Defines the command line interface structure and parsing logic using clap.
//! Supports the `collect` command with optional file extension filtering.

use clap::{Parser, Subcommand};

/// File Watcher (fw) - Monitor file operations using eBPF
#[derive(Parser)]
#[command(
    name = "fw",
    about = "A command-line file monitoring tool using eBPF",
    long_about = "Monitor file open/close operations across all running \
                  processes on the system with minimal overhead using eBPF \
                  technology."
)]
#[command(version, author)]
pub struct Cli {
    /// The command to execute
    #[command(subcommand)]
    pub command: Commands,
}

/// Available commands for the file watcher tool
#[derive(Subcommand)]
pub enum Commands {
    /// Collect file operation events until interrupted (Ctrl+C)
    ///
    /// Monitors file open/close operations and outputs events to stderr.
    /// Each event includes the file path, program name, action type, and
    /// timestamp.
    Collect {
        /// Comma-separated list of file extensions to monitor
        ///
        /// If specified, only files with these extensions will be monitored.
        /// Extensions should be provided without the leading dot (e.g., "rs,md,toml").
        /// If not specified, all file operations will be monitored.
        #[arg(
            short = 'e',
            long = "extensions",
            value_delimiter = ',',
            help = "File extensions to monitor (e.g., rs,md,toml)"
        )]
        extensions: Option<Vec<String>>,
    },
}
