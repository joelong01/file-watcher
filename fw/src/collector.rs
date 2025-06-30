//! File Collector module
//!
//! Orchestrates the file monitoring process by coordinating between the eBPF
//! monitor and event processing. Handles signal interruption (Ctrl+C) and
//! manages the event filtering and output.

use anyhow::{Context, Result};
use log::{info, warn};
use std::io::{self, Write};
use tokio::signal;

use crate::ebpf_monitor::EbpfMonitor;
use crate::file_event::FileEvent;

/// Run the file collection monitoring process
///
/// Starts the eBPF monitor, processes file events, and handles graceful
/// shutdown on Ctrl+C. Events are filtered by extensions if specified
/// and output to stderr.
///
/// # Arguments
/// * `extensions` - Optional list of file extensions to filter by
///
/// # Returns
/// * `Result<()>` - Success or error result
pub fn run_collect(extensions: Option<Vec<String>>) -> Result<()> {
    // Create a new async runtime for handling events
    let rt = tokio::runtime::Runtime::new()
        .context("Failed to create async runtime")?;

    rt.block_on(async {
        // Display filter information
        display_filter_info(&extensions);

        // Initialize the eBPF monitor
        let mut monitor =
            EbpfMonitor::new().context("Failed to initialize eBPF monitor")?;

        // Start monitoring in the background
        let mut event_receiver = monitor
            .start_monitoring()
            .await
            .context("Failed to start eBPF monitoring")?;

        // Set up Ctrl+C signal handling
        let ctrl_c = signal::ctrl_c();
        tokio::pin!(ctrl_c);

        info!("File monitoring started. Press Ctrl+C to stop.");

        loop {
            tokio::select! {
                // Handle incoming file events
                event_result = event_receiver.recv() => {
                    match event_result {
                        Some(event) => {
                            process_file_event(event, &extensions)?;
                        }
                        None => {
                            warn!("Event channel closed, stopping monitoring");
                            break;
                        }
                    }
                }
                // Handle Ctrl+C signal
                _ = &mut ctrl_c => {
                    info!("Received interrupt signal, stopping monitoring...");
                    break;
                }
            }
        }

        // Stop monitoring and cleanup
        monitor
            .stop_monitoring()
            .await
            .context("Failed to stop eBPF monitoring")?;

        info!("File monitoring stopped.");
        Ok(())
    })
}

/// Display information about active file extension filters
///
/// # Arguments
/// * `extensions` - Optional list of file extensions being filtered
fn display_filter_info(extensions: &Option<Vec<String>>) {
    match extensions {
        Some(exts) if !exts.is_empty() => {
            eprintln!("Monitoring files with extensions: {}", exts.join(", "));
        }
        _ => {
            eprintln!("Monitoring all file operations");
        }
    }
    eprintln!("Output format: timestamp | program (pid) | action | file_path");
    eprintln!("{}", "-".repeat(60));
}

/// Process a single file event and output it if it matches the filter
///
/// # Arguments
/// * `event` - The file event to process
/// * `extensions` - Optional list of file extensions to filter by
///
/// # Returns
/// * `Result<()>` - Success or error result
fn process_file_event(
    event: FileEvent,
    extensions: &Option<Vec<String>>,
) -> Result<()> {
    // Check if the event matches the extension filter
    if event.matches_extensions(extensions) {
        // Output to stderr as specified in requirements
        writeln!(io::stderr(), "{}", event)
            .context("Failed to write event to stderr")?;

        // Flush immediately for real-time output
        io::stderr().flush().context("Failed to flush stderr")?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_event::{FileAction, FileEvent};

    #[test]
    fn test_process_file_event_no_filter() {
        let event = FileEvent::new(
            "/path/to/file.rs".to_string(),
            "rustc".to_string(),
            FileAction::Opened,
            1234,
        );

        // Should not error when processing without filter
        assert!(process_file_event(event, &None).is_ok());
    }

    #[test]
    fn test_process_file_event_with_matching_filter() {
        let event = FileEvent::new(
            "/path/to/file.rs".to_string(),
            "rustc".to_string(),
            FileAction::Opened,
            1234,
        );
        let extensions = Some(vec!["rs".to_string()]);

        // Should not error when processing with matching filter
        assert!(process_file_event(event, &extensions).is_ok());
    }
}
