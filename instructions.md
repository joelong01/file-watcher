# File Watcher (fw)

This is a program called file watcher. We started building it together from scratch. The program will be written in **Rust**, use **eBPF**, and follow the best practices for building Rust programs, including:

1. All code will be reviewed with a PR. Copilot should do a code pilot review prior to creating a PR
2. All functions will contain canonical best practice Rust comments
3. Code will have code comments to indicate logic to make it easier for a human to read

## Application Semantics

This app is a command line interface named `fw` ("file watcher") that while running hooks into the file open activity of every app running on the machine. My dev machine is typically a Mac, but we also might run this on Linux. In the future we may extend it to work on Windows.

## Commands

### Collect Command

The initial implementation takes the following commands:

**Collect**: Runs until Ctrl+C is hit and collects information about what files were opened and closed. It writes to stderr (not stdout – we'll use that for IPC) a row with:

- The name of the file that was acted on
- The name of the program that acted on it
- The action (opened/closed)
- The time

**Parameters**:

- `--extensions`: A CSV separated list of file extensions that will be watched

### Help Command

`fw` also has a help command that reports on usage and semantics.

## Inner Loop

When I ask you to "run the inner loop", you will:

1. Run `cargo build`, capture the output, and propose fixes for any compiler issues
2. Run `cargo check` and ensure we are linter clean. Fix any linter errors
3. Before the merge give me a table with the lines deleted, added, and modified counts give me a summary of the changes made. Do this FOR ALL suggested changes. I am particularly concerned about you editing code outside the scope of a request. Avoid this at all costs.

## Copilot / AI Rules

Project-specific Copilot and AI development rules are maintained in [`.vscode/copilot-rules.md`](.vscode/copilot-rules.md).
These rules help ensure consistent development patterns and quality standards across the project.
