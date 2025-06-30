//! Integration tests for the file watcher CLI
//!
//! These tests verify the end-to-end functionality of the file watcher
//! by creating test files and monitoring them with different extensions.

use anyhow::Result;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;
use tempfile::TempDir;

/// Test configuration for file monitoring scenarios
#[derive(Debug)]
struct TestConfig {
    /// Directory where test files will be created
    test_dir: TempDir,
    /// Extensions to monitor
    extensions: Vec<String>,
    /// Expected number of events
    expected_events: usize,
}

impl TestConfig {
    /// Create a new test configuration
    ///
    /// # Arguments
    /// * `extensions` - File extensions to monitor
    /// * `expected_events` - Number of events expected during test
    ///
    /// # Returns
    /// * `Result<TestConfig>` - New test configuration
    fn new(extensions: Vec<String>, expected_events: usize) -> Result<Self> {
        let test_dir = tempfile::tempdir()?;
        Ok(Self {
            test_dir,
            extensions,
            expected_events,
        })
    }

    /// Get the path to the test directory
    fn test_dir_path(&self) -> &std::path::Path {
        self.test_dir.path()
    }

    /// Create a test file with the given name
    ///
    /// # Arguments
    /// * `filename` - Name of the file to create
    /// * `content` - Content to write to the file
    ///
    /// # Returns
    /// * `Result<PathBuf>` - Path to the created file
    fn create_test_file(&self, filename: &str, content: &str) -> Result<PathBuf> {
        let file_path = self.test_dir_path().join(filename);
        let mut file = fs::File::create(&file_path)?;
        file.write_all(content.as_bytes())?;
        file.sync_all()?;
        Ok(file_path)
    }
}

/// Test the CLI with specific file extensions
///
/// This test verifies that:
/// 1. Files with matching extensions are monitored and reported
/// 2. Files with non-matching extensions are ignored
/// 3. Output format is correct
#[tokio::test]
async fn test_cli_file_extension_filtering() -> Result<()> {
    // Test configuration: monitor "fw-123" and "f2-234" extensions
    let config = TestConfig::new(
        vec!["fw-123".to_string(), "f2-234".to_string()],
        2, // Expect 2 events for the matching files
    )?;

    // Start the file watcher process with extension filtering
    // Run from workspace directory, not temp directory
    let mut child = Command::new("cargo")
        .args([
            "run",
            "--",
            "collect",
            "--extensions",
            "fw-123,f2-234",
        ])
        .current_dir("/workspace")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    // Give the process time to start up
    thread::sleep(Duration::from_millis(500));

    // Create test files
    println!("Creating test files...");

    // Create files that should be monitored
    let file1_path = config.create_test_file(
        "test_file_1.fw-123",
        "This is a test file with fw-123 extension",
    )?;

    let file2_path = config.create_test_file(
        "test_file_2.f2-234",
        "This is a test file with f2-234 extension",
    )?;

    // Create a file that should NOT be monitored
    let file3_path = config.create_test_file(
        "test_file_3.fw-456",
        "This file should be ignored",
    )?;

    println!("Created test files:");
    println!("  - {} (should be monitored)", file1_path.display());
    println!("  - {} (should be monitored)", file2_path.display());
    println!("  - {} (should be ignored)", file3_path.display());

    // Wait for file operations to be processed
    thread::sleep(Duration::from_millis(1000));

    // Stop the monitoring process
    child.kill()?;
    let output = child.wait_with_output()?;

    // Parse stderr output (where events are written)
    let stderr_output = String::from_utf8_lossy(&output.stderr);
    println!("Stderr output:\n{}", stderr_output);

    // Verify the output
    verify_output(&stderr_output, &config)?;

    Ok(())
}

/// Verify that the output contains expected events and formatting
///
/// # Arguments
/// * `output` - The stderr output from the file watcher
/// * `config` - Test configuration with expectations
///
/// # Returns
/// * `Result<()>` - Success if verification passes
fn verify_output(output: &str, _config: &TestConfig) -> Result<()> {
    let lines: Vec<&str> = output
        .lines()
        .filter(|line| line.contains(" | ") && line.contains("opened"))
        .collect();

    println!("Found {} event lines in output", lines.len());

    // Should find events for fw-123 and f2-234 files
    let fw_123_events = lines
        .iter()
        .filter(|line| line.contains(".fw-123"))
        .count();

    let f2_234_events = lines
        .iter()
        .filter(|line| line.contains(".f2-234"))
        .count();

    let fw_456_events = lines
        .iter()
        .filter(|line| line.contains(".fw-456"))
        .count();

    println!("Event breakdown:");
    println!("  - .fw-123 files: {} events", fw_123_events);
    println!("  - .f2-234 files: {} events", f2_234_events);
    println!("  - .fw-456 files: {} events (should be 0)", fw_456_events);

    // Verify expectations
    assert!(
        fw_123_events > 0,
        "Expected events for .fw-123 files, but found none"
    );
    assert!(
        f2_234_events > 0,
        "Expected events for .f2-234 files, but found none"
    );
    assert_eq!(
        fw_456_events, 0,
        "Found unexpected events for .fw-456 files (should be filtered out)"
    );

    // Verify output format
    for line in &lines {
        verify_event_format(line)?;
    }

    println!("✅ All output verification checks passed!");
    Ok(())
}

/// Verify that an event line has the correct format
///
/// Expected format: "timestamp | program (pid) | action | file_path"
///
/// # Arguments
/// * `line` - Event line to verify
///
/// # Returns
/// * `Result<()>` - Success if format is correct
fn verify_event_format(line: &str) -> Result<()> {
    let parts: Vec<&str> = line.split(" | ").collect();

    if parts.len() != 4 {
        return Err(anyhow::anyhow!(
            "Invalid event format: expected 4 parts separated by ' | ', got {} parts in line: {}",
            parts.len(),
            line
        ));
    }

    // Verify timestamp format (should contain date and time)
    let timestamp = parts[0];
    if !timestamp.contains("UTC") {
        return Err(anyhow::anyhow!(
            "Invalid timestamp format: expected UTC timestamp, got: {}",
            timestamp
        ));
    }

    // Verify program format (should contain program name and PID)
    let program = parts[1];
    if !program.contains("(") || !program.contains(")") {
        return Err(anyhow::anyhow!(
            "Invalid program format: expected 'program (pid)', got: {}",
            program
        ));
    }

    // Verify action
    let action = parts[2];
    if action != "opened" && action != "closed" {
        return Err(anyhow::anyhow!(
            "Invalid action: expected 'opened' or 'closed', got: {}",
            action
        ));
    }

    // Verify file path
    let file_path = parts[3];
    if file_path.is_empty() {
        return Err(anyhow::anyhow!("File path cannot be empty"));
    }

    Ok(())
}

/// Test that the help command works correctly
#[tokio::test]
async fn test_cli_help_command() -> Result<()> {
    let output = Command::new("cargo")
        .args(["run", "--", "--help"])
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Verify help content
    assert!(stdout.contains("file monitoring tool"));
    assert!(stdout.contains("collect"));
    assert!(stdout.contains("eBPF"));

    println!("✅ Help command test passed!");
    Ok(())
}

/// Test that collect command help works correctly
#[tokio::test]
async fn test_cli_collect_help() -> Result<()> {
    let output = Command::new("cargo")
        .args(["run", "--", "collect", "--help"])
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Verify collect help content
    assert!(stdout.contains("Collect file operation events"));
    assert!(stdout.contains("--extensions"));
    assert!(stdout.contains("Comma-separated list"));

    println!("✅ Collect help command test passed!");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test the TestConfig functionality
    #[test]
    fn test_config_creation() -> Result<()> {
        let config = TestConfig::new(vec!["rs".to_string(), "md".to_string()], 2)?;
        assert_eq!(config.extensions.len(), 2);
        assert_eq!(config.expected_events, 2);
        assert!(config.test_dir_path().exists());
        Ok(())
    }

    /// Test file creation functionality
    #[test]
    fn test_file_creation() -> Result<()> {
        let config = TestConfig::new(vec!["test".to_string()], 1)?;
        let file_path = config.create_test_file("test.txt", "Hello, world!")?;

        assert!(file_path.exists());
        let content = fs::read_to_string(&file_path)?;
        assert_eq!(content, "Hello, world!");

        Ok(())
    }

    /// Test event format verification
    #[test]
    fn test_event_format_verification() -> Result<()> {
        // Valid format
        let valid_line = "2023-06-27 12:34:56 UTC | test_program (1234) | opened | /path/to/file.txt";
        assert!(verify_event_format(valid_line).is_ok());

        // Invalid format - wrong number of parts
        let invalid_line = "timestamp | program | action";
        assert!(verify_event_format(invalid_line).is_err());

        // Invalid timestamp
        let invalid_timestamp = "invalid-timestamp | test_program (1234) | opened | /path/to/file.txt";
        assert!(verify_event_format(invalid_timestamp).is_err());

        // Invalid program format
        let invalid_program = "2023-06-27 12:34:56 UTC | test_program | opened | /path/to/file.txt";
        assert!(verify_event_format(invalid_program).is_err());

        Ok(())
    }
}
