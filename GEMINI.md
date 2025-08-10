# Git Intelligence Message (GIM) - Project Context for Qwen

## Project Overview

This project, Git Intelligence Message (GIM), is a command-line utility written in Rust. Its primary function is to automatically generate high-quality Git commit messages by analyzing the changes in the staging area of a Git repository. It leverages an AI model, accessed via an HTTP API, to interpret the code changes (diff) and produce a concise, structured commit message.

Key features include:
- Automatic generation of both a commit subject (type and summary) and a detailed commit body based on staged changes.
- Integration with various AI models (e.g., Qwen, GPT, Gemini) via configurable API endpoints.
- Customizable AI prompts for tailoring the commit message style.
- Subcommands for managing configuration (`ai`, `config`, `prompt`) and performing self-updates (`update`).

## Building and Running

### Prerequisites
- Rust toolchain (e.g., via rustup)
- Git
- An API key for an AI service (e.g., Qwen, OpenAI)

### Setup
1.  Clone the repository.
2.  Navigate to the project directory.
3.  Ensure Rust and Cargo are installed.

### Building
To build the project:
```bash
cargo build
```
To build an optimized release version:
```bash
cargo build --release
```

### Running
The main executable is `gim` (or `gim.exe` on Windows), located in `target/debug/` (or `target/release/`).

Before using GIM for the first time, configure the AI settings:
```bash
# Example for Qwen
gim ai -m qwen2.5-72b-instruct -k YOUR_API_KEY -u https://dashscope.aliyuncs.com/compatible-mode/v1 -l English
```

To generate a commit message for staged changes:
```bash
# Ensure changes are staged first, e.g., `git add .`
gim
# Or, to automatically add all changes and generate a commit:
gim -a
```

Other common commands:
```bash
# Show help
gim --help

# Show help for a specific subcommand
gim prompt --help

# Check for updates
gim update

# View or edit AI prompts
gim prompt
gim prompt -e -t diff

# View current AI configuration
gim ai
```

## Development Conventions

- **Language:** Rust is the primary language, targeting the 2024 edition.
- **CLI Framework:** Uses `clap` for parsing command-line arguments.
- **Configuration:** Uses TOML for configuration files, managed by the `gim-config` crate.
- **HTTP Client:** `reqwest` is used for making asynchronous HTTP requests to AI APIs.
- **Async Runtime:** `tokio` is used for asynchronous operations.
- **Logging:** Basic logging is available via `log` and `pretty_env_logger`.
- **Testing:** Unit tests are included within modules (e.g., `src/cli/http.rs`).

### Code Structure (src/)
- `main.rs`: Entry point, initializes CLI parsing and configuration.
- `cli/`: Contains core CLI logic.
    - `command.rs`: Defines the CLI structure and arguments using `clap`.
    - `entry.rs`: Main logic for handling commands and orchestrating the commit message generation process. This includes calling Git commands, interacting with the AI via `http.rs`, and executing the final `git commit`.
    - `http.rs`: Handles communication with the AI API.
    - `prompt.rs`: Manages the customizable AI prompts for generating the diff summary and the commit subject.
    - `ai_configer.rs`: Utilities for reading and updating the AI-related configuration.
    - `custom_param.rs`: Utilities for managing custom parameters like `lines_limit`.
    - `update/`: Logic related to self-updating the tool.
    - `verbose.rs`: Utility for controlling verbose output.

### Configuration
- AI settings (model, API key, URL, language) are stored in a TOML configuration file.
- Customizable prompt templates are stored in separate files within the user's configuration directory.
