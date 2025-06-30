//! File Watcher (fw) - A command-line file monitoring tool using eBPF
//!
//! This tool monitors file open/close operations across all running processes
//! on the system using eBPF technology for minimal overhead and maximum
//! visibility.

use anyhow::{Context, Result};
use clap::Parser;
use log::{error, info};
use std::process;

mod cli;
mod collector;
mod ebpf_monitor;
mod file_event;

use cli::{Cli, Commands};

/// Main entry point for the file watcher application
///
/// Initializes logging, parses command line arguments, and dispatches to
/// the appropriate command handler.
fn main() {
    // Initialize logging
    env_logger::init();

    // Parse command line arguments
    let cli = Cli::parse();

    // Execute the requested command and handle any errors
    if let Err(e) = run_command(cli) {
        error!("Error: {}", e);
        process::exit(1);
    }
}

/// Execute the requested command based on CLI arguments
///
/// # Arguments
/// * `cli` - Parsed command line interface structure
///
/// # Returns
/// * `Result<()>` - Success or error result
fn run_command(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Collect { extensions } => {
            info!("Starting file collection with extensions: {:?}", extensions);
            collector::run_collect(extensions)
                .context("Failed to run file collection")?;
        }
    }
    Ok(())
}
