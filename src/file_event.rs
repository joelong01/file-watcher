//! File Event module
//!
//! Defines the structure and formatting for file operation events captured
//! by the eBPF monitoring system.

use chrono::{DateTime, Utc};
use std::fmt;

/// Represents the type of file operation that occurred
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileAction {
    /// File was opened for reading or writing
    Opened,
    /// File was closed after being opened
    #[allow(dead_code)]
    Closed,
}

impl fmt::Display for FileAction {
    /// Format the file action for human-readable output
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileAction::Opened => write!(f, "opened"),
            FileAction::Closed => write!(f, "closed"),
        }
    }
}

/// Represents a file operation event captured from the system
///
/// Contains all relevant information about a file operation including
/// the file path, the program that performed the operation, the type
/// of operation, and when it occurred.
#[derive(Debug, Clone)]
pub struct FileEvent {
    /// Full path to the file that was accessed
    pub file_path: String,
    /// Name of the program/process that accessed the file
    pub program_name: String,
    /// Type of file operation (opened or closed)
    pub action: FileAction,
    /// Timestamp when the operation occurred
    pub timestamp: DateTime<Utc>,
    /// Process ID of the program that accessed the file
    pub pid: u32,
}

impl FileEvent {
    /// Create a new file event
    ///
    /// # Arguments
    /// * `file_path` - Path to the file that was accessed
    /// * `program_name` - Name of the program that accessed the file
    /// * `action` - Type of file operation
    /// * `pid` - Process ID of the accessing program
    ///
    /// # Returns
    /// * `FileEvent` - New file event with current timestamp
    pub fn new(
        file_path: String,
        program_name: String,
        action: FileAction,
        pid: u32,
    ) -> Self {
        Self {
            file_path,
            program_name,
            action,
            timestamp: Utc::now(),
            pid,
        }
    }

    /// Check if this event matches the specified file extensions filter
    ///
    /// # Arguments
    /// * `extensions` - Optional list of file extensions to match against
    ///
    /// # Returns
    /// * `bool` - True if the file matches the filter or no filter is set
    pub fn matches_extensions(&self, extensions: &Option<Vec<String>>) -> bool {
        match extensions {
            None => true, // No filter means all files match
            Some(exts) => {
                // Extract file extension from path
                if let Some(file_name) = self.file_path.split('/').next_back() {
                    if let Some(ext) = file_name.split('.').next_back() {
                        return exts
                            .iter()
                            .any(|e| e.eq_ignore_ascii_case(ext));
                    }
                }
                false // No extension found or doesn't match
            }
        }
    }
}

impl fmt::Display for FileEvent {
    /// Format the file event for output to stderr
    ///
    /// Output format: timestamp | program_name (pid) | action | file_path
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} | {} ({}) | {} | {}",
            self.timestamp.format("%Y-%m-%d %H:%M:%S UTC"),
            self.program_name,
            self.pid,
            self.action,
            self.file_path
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_action_display() {
        assert_eq!(format!("{}", FileAction::Opened), "opened");
        assert_eq!(format!("{}", FileAction::Closed), "closed");
    }

    #[test]
    fn test_file_event_matches_extensions_no_filter() {
        let event = FileEvent::new(
            "/path/to/file.rs".to_string(),
            "rustc".to_string(),
            FileAction::Opened,
            1234,
        );
        assert!(event.matches_extensions(&None));
    }

    #[test]
    fn test_file_event_matches_extensions_with_filter() {
        let event = FileEvent::new(
            "/path/to/file.rs".to_string(),
            "rustc".to_string(),
            FileAction::Opened,
            1234,
        );
        let extensions = Some(vec!["rs".to_string(), "md".to_string()]);
        assert!(event.matches_extensions(&extensions));

        let non_matching_extensions =
            Some(vec!["py".to_string(), "js".to_string()]);
        assert!(!event.matches_extensions(&non_matching_extensions));
    }

    #[test]
    fn test_file_event_format() {
        let event = FileEvent::new(
            "/path/to/file.rs".to_string(),
            "rustc".to_string(),
            FileAction::Opened,
            1234,
        );
        let formatted = format!("{}", event);
        assert!(formatted.contains("rustc (1234)"));
        assert!(formatted.contains("opened"));
        assert!(formatted.contains("/path/to/file.rs"));
    }
}
