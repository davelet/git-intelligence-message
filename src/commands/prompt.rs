use gim_config::directory;
use indoc::indoc;
use std::{fs, io, path::PathBuf, process::Command};

use crate::config::constants::{DIFF_PROMPT_FILE, SUBJECT_PROMPT_FILE};
use crate::core::git;

fn file_dirs() -> io::Result<PathBuf> {
    directory::config_dir()
}

/// Checks for local .gim directory and returns prompt file paths if they exist
fn get_local_gim_prompt_files() -> Option<(PathBuf, PathBuf)> {
    if let Some(git_root) = git::get_git_root() {
        let gim_dir = git_root.join(".gim");
        if gim_dir.is_dir() {
            let diff_prompt_path = gim_dir.join(DIFF_PROMPT_FILE);
            let subject_prompt_path = gim_dir.join(SUBJECT_PROMPT_FILE);
            
            // Check if at least one of the prompt files exists
            if diff_prompt_path.exists() || subject_prompt_path.exists() {
                return Some((diff_prompt_path, subject_prompt_path));
            }
        }
    }
    None
}

fn trim_diff_prompt() -> String {
    indoc!(r#"
        You are an expert developer specialist in creating git commits.
        Provide a super concise one sentence summary for each changed file, describing the main change made.
        Each line must follow this format {FILE: CHANGES: (CHANGED_LINES_COUNT)}

        Please follow these rules strictly:
        - Output ONLY the lines of summaries, NO explanations, NO markdown, NO code blocks.
        - Each file change gets exactly one line.
        - Do not use general terms like "update" or "change", be specific.
        - Use present tense, active voice, and imperative mood (e.g., "Fix bug" instead of "Fixed bug").
        - Skip project lock files, like 'Cargo.lock' or 'package-lock.json', 'pnpm-lock.yaml', 'yarn.lock'
        - Skip binary files diff content
        - Ignore files under .code folder or .idea folder, unless there aren't other files changed.
        - Avoid phrases like "The main goal is to..." or "Based on...", just state the change directly.
        - The output should be ready to copy-paste as a commit message with no further modification.

        Examples:
        src/main.rs: Add login validation logic (87)
        README.md: Update installation instructions (12)
    "#)
    .to_string()
}

fn trim_subject_prompt() -> String {
    indoc!(r#"
        You are an expert developer specialist in creating git commits messages.
        Your only goal is to retrieve a single commit message.
        Based on the provided user changes, combine them in ONE SINGLE commit message retrieving the global idea, following strictly the next rules:
        - Assign the commit {type} according to the next conditions:
            feat: Only when adding a new feature.
            fix: When fixing a bug.
            docs: When updating documentation.
            style: When changing elements styles or design and/or making changes to the code style (formatting, missing semicolons, etc.) without changing the code logic.
            test: When adding or updating tests.
            chore: When making changes to the build process or auxiliary tools and libraries.
            revert: When undoing a previous commit.
            refactor: When restructuring code without changing its external behavior, or is any of the other refactor types.
        - Do not add any issues numeration, explain your output nor introduce your answer.
        - The number at the end of each file change is the count of changed lines; prioritize summarizing files with more line changes, except for newly added files which have medium priority
        - Output directly only one commit message in plain text with the next format: {type}: {commit_message}.
        - Be as concise as possible, keep the message under 50 characters or letters.
    "#)
    .to_string()
}

/// Returns the diff prompt string, reading from local .gim directory first if available,
/// then from config directory if available, or using the default if not.
/// If a custom prompt is provided, it will be used instead of any file-based prompts.
///
/// # Arguments
///
/// * `custom_prompt` - Optional custom prompt to use instead of file-based prompts
///
/// # Returns
///
/// * `String` containing the diff prompt.
pub fn get_diff_prompt(custom_prompt: Option<&str>) -> String {
    // If custom prompt is provided, use it directly
    if let Some(custom) = custom_prompt {
        return custom.to_string();
    }
    
    let trimmed = trim_diff_prompt();
    
    // First, check for local .gim directory
    if let Some((local_diff_path, _)) = get_local_gim_prompt_files() {
        if local_diff_path.exists() {
            match fs::read_to_string(&local_diff_path) {
                Ok(content) => return content,
                Err(e) => {
                    eprintln!("Failed to read diff prompt from local .gim directory: {}", e);
                    // Continue to fallback to config directory
                }
            }
        }
    }
    
    // Fallback to config directory
    let path = match file_dirs() {
        Ok(p) => p.join(DIFF_PROMPT_FILE),
        Err(_) => {
            eprintln!("Failed to get config dir for diff prompt");
            return trimmed;
        }
    };

    if !path.exists() {
        if let Err(e) = fs::write(&path, trimmed.clone()) {
            eprintln!("Failed to write diff prompt to file: {}", e);
        }
        return trimmed;
    }

    fs::read_to_string(path).unwrap_or_else(|e| {
        eprintln!("Failed to read diff prompt from file: {}", e);
        trimmed
    })
}

/// Returns the subject prompt string, reading from local .gim directory first if available,
/// then from config directory if available, or using the default if not.
/// If a custom prompt is provided, it will be used instead of any file-based prompts.
///
/// # Arguments
///
/// * `custom_prompt` - Optional custom prompt to use instead of file-based prompts
///
/// # Returns
///
/// * `String` containing the subject prompt.
pub fn get_subject_prompt(custom_prompt: Option<&str>) -> String {
    // If custom prompt is provided, use it directly
    if let Some(custom) = custom_prompt {
        return custom.to_string();
    }
    
    let trimmed = trim_subject_prompt();
    
    // First, check for local .gim directory
    if let Some((_, local_subject_path)) = get_local_gim_prompt_files() {
        if local_subject_path.exists() {
            match fs::read_to_string(&local_subject_path) {
                Ok(content) => return content,
                Err(e) => {
                    eprintln!("Failed to read subject prompt from local .gim directory: {}", e);
                    // Continue to fallback to config directory
                }
            }
        }
    }
    
    // Fallback to config directory
    let path = match file_dirs() {
        Ok(p) => p.join(SUBJECT_PROMPT_FILE),
        Err(_) => {
            eprintln!("Failed to get config dir for subject prompt");
            return trimmed;
        }
    };

    if !path.exists() {
        if let Err(e) = fs::write(&path, trimmed.clone()) {
            eprintln!("Failed to write subject prompt to file: {}", e);
        }
        return trimmed;
    }

    fs::read_to_string(path).unwrap_or_else(|e| {
        eprintln!("Failed to read subject prompt from file: {}", e);
        trimmed
    })
}

/// Deletes the prompt files.
///
/// # Returns
///
/// * `Ok(())` if successful, `Err` otherwise.
pub fn delete_prompt_files() -> io::Result<()> {
    let config_dir = directory::config_dir()?;
    let diff_prompt_path = config_dir.join(DIFF_PROMPT_FILE);
    let subject_prompt_path = config_dir.join(SUBJECT_PROMPT_FILE);
    if diff_prompt_path.exists() {
        std::fs::remove_file(&diff_prompt_path)?;
    }
    if subject_prompt_path.exists() {
        std::fs::remove_file(&subject_prompt_path)?;
    }
    Ok(())
}

/// Opens the config directory in the file manager.
///
/// # Returns
///
/// * `Ok(())` if successful, `Err` otherwise.
pub fn open_config_directory() -> io::Result<()> {
    let config_dir = directory::config_dir()?;
    // Open the directory with default file manager
    if cfg!(target_os = "macos") {
        Command::new("open")
            .arg(&config_dir)
            .status()?;
    } else if cfg!(target_os = "windows") {
        Command::new("explorer").arg(&config_dir).status()?;
    } else {
        // Linux and others
        Command::new("xdg-open").arg(&config_dir).status()?;
    }
    Ok(())
}

/// Handles the prompt command.
///
/// # Arguments
///
/// * `edit` - If true, opens editor for editing.
/// * `prompt` - Optional prompt type to edit.
/// * `editor` - Optional editor to use.
///
/// # Returns
///
/// * `Ok(())` if successful, `Err` otherwise.
pub fn handle_prompt_command(
    edit: bool,
    prompt: Option<&str>,
    editor: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let config_dir = directory::config_dir()?;
    let diff_prompt_path = config_dir.join(DIFF_PROMPT_FILE);
    let subject_prompt_path = config_dir.join(SUBJECT_PROMPT_FILE);

    let diff_prompt = get_diff_prompt(None);
    let subject_prompt = get_subject_prompt(None);

    if edit {
        if let Some(prompt_type) = prompt {
            let file_path = match prompt_type.to_lowercase().as_str() {
                "d" | "diff" | "diff_prompt" | DIFF_PROMPT_FILE => diff_prompt_path,
                "s" | "subject" | "subject_prompt" | SUBJECT_PROMPT_FILE => subject_prompt_path,
                _ => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!(
                            "Unknown prompt type '{}'. Use 'd' or 'diff' or 'diff_prompt' for diff prompt, and 's' or 'subject' or 'subject_prompt' for subject prompt",
                            prompt_type
                        )
                    ).into())
                }
            };

            if let Some(editor) = editor {
                // Use the specified editor
                if let Err(e) = Command::new(editor).arg(&file_path).status() {
                    eprintln!("Failed to open file with editor '{}': {}", editor, e);
                }
            } else {
                // Open the directory with default file manager
                if cfg!(target_os = "macos") {
                    Command::new("open")
                        .arg("-R") // Reveal in Finder
                        .arg(&file_path)
                        .status()?;
                } else if cfg!(target_os = "windows") {
                    Command::new("explorer")
                        .arg("/select,")
                        .arg(&file_path)
                        .status()?;
                } else {
                    // Linux and others
                    if let Err(_) = Command::new("xdg-open")
                        .arg(file_path.parent().unwrap_or_else(|| ".".as_ref()))
                        .status()
                    {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::NotFound,
                            "Failed to open file manager. Please specify an editor with --editor"
                        ).into());
                    }
                }
            }
        } else {
            open_config_directory()?;
            indoc::printdoc!(
                r#"
                Please edit the prompt files using your favorite editor in the popped window: {}
                1: {}
                2: {}
                "#,
                config_dir.display(),
                DIFF_PROMPT_FILE,
                SUBJECT_PROMPT_FILE
            );
        }
    } else {
        // Show the content of both prompt files
        indoc::printdoc!(
            r#"
            === Diff Prompt ===
            {}

            === Subject Prompt ===
            {}
            "#,
            &diff_prompt,
            &subject_prompt
        );
    }

    Ok(())
}
