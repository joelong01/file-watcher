# Copilot / AI Development Rules

As we work, we will occasionally find patterns that should be consistently
followed in the project. If the AI thinks there might be a pattern to be
followed, it should ask "Should I add the following as a pattern?" And if so,
add it as a Copilot rule.

## Rule 1: Linter Execution After File Changes

If there is a linter available for the language/file type being worked on,
run the linter after making any changes to validate the modifications. This
includes:

- Markdown files: Run markdownlint or equivalent
- Rust files: Run `cargo clippy` and `cargo fmt`
- TypeScript/JavaScript files: Run ESLint and Prettier
- Python files: Run pylint, flake8, or equivalent
- Any other language-specific linters available in the project

## Rule 2: VS Code First Development Environment

When adding functionality or tooling to the project, prioritize VS Code-native
solutions:

- Use VS Code extensions instead of standalone tools when possible
- Configure linters, formatters, and build tools through VS Code settings
- Use `.devcontainer` for consistent development environments across machines
- Leverage VS Code's integrated terminal, debugger, and task runner
- Store project-specific settings in `.vscode/` directory
- Use Docker containers via devcontainers for reproducible development
  environments

## Rule 3: Copilot Rules Organization

- Store project-specific Copilot/AI development rules in
  `.vscode/copilot-rules.md`
- Keep `instructions.md` focused on project overview and session initialization
- Reference the Copilot rules file from `instructions.md` for discoverability
- Version control all Copilot rules as part of the project configuration

## Rule 4: Line Length Limit

All code and documentation should adhere to an 80-character line length limit:

- Rust code: Configure `rustfmt` with `max_width = 80`
- Markdown files: Line length limit disabled (MD013: false) to allow natural
  text flow based on editor window width for better readability
- Comments: Keep inline and block comments within 80 characters
- Configuration files: Format JSON, TOML, and other config files with 80-char
  limit
- When line breaks are needed, use appropriate continuation patterns for the
  language

## Rule 5: Small, Well-Commented Functions

All logic should be implemented in small, well-commented functions with crisp,
clean semantics:

- Functions should have a single, well-defined responsibility
- Function names should clearly describe what they do
- Each function should include comprehensive doc comments explaining purpose,
  parameters, return values, and any side effects
- Keep functions small (ideally under 20-30 lines) for easier AI interaction
- Complex operations should be broken down into smaller, composable functions
- Clear function semantics enable more effective AI collaboration and code
  understanding

## Rule 6: Communication and Code Readiness Standards

All interactions and code generation should follow professional standards that
ensure immediate usability:

- Use markdown formatting with backticks for file, function, and class names
- Generated code must be immediately runnable by the user
- Always include necessary imports, dependencies, and configuration files
- Read file contents before editing (unless making small, obvious changes)
- Don't over-apologize when unexpected results occur - explain circumstances
  and proceed
- Explain actions before taking them, but avoid mentioning specific tool names

## Rule 7: Debugging and Error Handling Best Practices

Follow systematic approaches to debugging and error resolution:

- Address root causes instead of symptoms when debugging
- Add descriptive logging and error messages to track code state
- Limit linter error fix attempts to 3 iterations per file
- If third attempt fails, stop and ask for guidance rather than continuing
- Use appropriate external packages without asking permission while following
  security best practices
- Highlight security concerns (like API keys) and suggest proper handling

## Rule 8: Tool Usage and Context Management

When using tools or making changes, follow transparent and context-aware
practices:

- Always provide clear explanations for why tools are being used and how they
  contribute to the goal
- When reading files, ensure complete context by checking if viewed content is
  sufficient before proceeding
- For file modifications, read sufficient context around the change area to
  avoid breaking dependencies
- Use targeted search strategies: semantic search for conceptual queries, regex
  for exact patterns, fuzzy search for partial file names
- Maintain awareness of workspace structure and file relationships
- When edits fail or produce unexpected results, use smarter retry approaches
  rather than repeating the same action
- For potentially destructive operations, default to requiring explicit
  approval

## Rule 9: Persistent Environment Configuration

All environment changes must be made in a way that persists across container
rebuilds and resets:

- Permission fixes should be scripted in the Dockerfile or postCreateCommand
- Software installations should be added to the Dockerfile, not done manually
- Configuration changes should be committed to devcontainer.json or related
  config files
- The postCreateCommand should handle authentication setup (e.g., gh auth login)
- Never make manual changes that won't survive a container rebuild
- Document any required manual steps in README.md if they cannot be automated
- Environment setup should be idempotent and repeatable across all team members

This ensures consistent development environments and prevents "works on my
machine" issues.
