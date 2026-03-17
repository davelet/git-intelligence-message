use std::path::PathBuf;
use std::process::Command;

use crate::utils::output;

/// Gets the git repository root directory.
///
/// # Returns
///
/// * `Option<PathBuf>` containing the git root path if inside a git repository, `None` otherwise.
pub fn get_git_root() -> Option<PathBuf> {
    let output = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .ok()?;

    if output.status.success() {
        let path_str = String::from_utf8_lossy(&output.stdout);
        Some(PathBuf::from(path_str.trim()))
    } else {
        None
    }
}

/// Checks if the current directory is a git repository.
///
/// # Returns
///
/// * `true` if inside a git repository, `false` otherwise.
pub fn is_git_repo() -> bool {
    Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .output()
        .is_ok()
}

/// Gets git status as untracked files can be included or excluded.
///
/// # Arguments
///
/// * `auto_add` - If true, includes all untracked files; otherwise excludes them.
///
/// # Returns
///
/// * `Vec<String>` containing the status lines.
pub fn get_git_status(auto_add: bool) -> Vec<String> {
    let status_output = Command::new("git")
        .args([
            "status",
            "-s",
            &format!("--untracked-files={}", if auto_add { "all" } else { "no" }),
        ])
        .output()
        .expect("Failed to get git status");

    output::print_verbose(&format!(
        "Run 'git status -s --untracked-files={}'",
        if auto_add { "all" } else { "no" }
    ));

    let status_str = String::from_utf8_lossy(&status_output.stdout);
    status_str.lines().map(|s| s.to_string()).collect()
}

/// Adds all changes to git staging area.
pub fn git_add_all() {
    let add_output = Command::new("git")
        .args(["add", "."])
        .output()
        .expect("Failed to execute git add");

    if !add_output.status.success() {
        eprintln!("Error: Failed to add changes to git");
        panic!("git add failed");
    }

    output::print_verbose("Run 'git add .'");
}

/// Gets the name-status of staged changes.
///
/// # Returns
///
/// * `String` containing the name-status output.
pub fn get_staged_name_status() -> String {
    let diff_output = Command::new("git")
        .args(["diff", "--cached", "--name-status"])
        .output()
        .expect("Failed to get git diff --cached --name-status");

    output::print_verbose("Run 'git diff --cached --name-status'");

    String::from_utf8_lossy(&diff_output.stdout).to_string()
}

/// Gets the full diff for added/modified staged files.
///
/// # Returns
///
/// * `String` containing the diff output.
pub fn get_staged_diff() -> String {
    let full_diff_output = Command::new("git")
        .args(["diff", "--cached", "--diff-filter=AM"])
        .output()
        .expect("Failed to get git diff --cached --diff-filter=AM");

    output::print_verbose("Run 'git diff --cached --diff-filter=AM'");

    String::from_utf8_lossy(&full_diff_output.stdout).to_string()
}

/// Gets the numstat for staged changes.
///
/// # Returns
///
/// * `String` containing the numstat output (lines added/deleted per file).
pub fn get_staged_numstat() -> String {
    let numstat_output = Command::new("git")
        .args(["diff", "--cached", "--numstat"])
        .output()
        .expect("Failed to get git diff --cached --numstat");

    output::print_verbose("Run 'git diff --cached --numstat'");

    String::from_utf8_lossy(&numstat_output.stdout).to_string()
}

/// Gets the diff for specific files in staging area.
///
/// # Arguments
///
/// * `files` - List of file paths to get diff for.
///
/// # Returns
///
/// * `String` containing the diff output.
pub fn get_staged_diff_for_files(files: &[String]) -> String {
    let mut args = vec!["diff", "--cached", "--diff-filter=AM", "--"];
    let file_refs: Vec<&str> = files.iter().map(|s| s.as_str()).collect();
    args.extend(file_refs);

    let diff_output = Command::new("git")
        .args(&args)
        .output()
        .expect("Failed to get git diff for specific files");

    output::print_verbose(&format!(
        "Run 'git diff --cached --diff-filter=AM -- {:?}'",
        files
    ));

    String::from_utf8_lossy(&diff_output.stdout).to_string()
}

/// Gets the name-status of the last commit.
///
/// # Returns
///
/// * `String` containing the name-status output.
pub fn get_last_commit_name_status() -> String {
    let show_status_output = Command::new("git")
        .args(["show", "--pretty=format:", "--name-status", "HEAD"])
        .output()
        .expect("Failed to get git show --name-status");

    output::print_verbose("Run 'git show --pretty=format: --name-status HEAD'");

    String::from_utf8_lossy(&show_status_output.stdout).to_string()
}

/// Gets the full diff of added/modified files in the last commit.
///
/// # Returns
///
/// * `String` containing the diff output.
pub fn get_last_commit_diff() -> String {
    let show_diff_output = Command::new("git")
        .args(["show", "--pretty=format:", "--diff-filter=AM", "HEAD"])
        .output()
        .expect("Failed to get git show --diff-filter=AM");

    output::print_verbose("Run 'git show --pretty=format: --diff-filter=AM HEAD'");

    String::from_utf8_lossy(&show_diff_output.stdout).to_string()
}

/// Gets the numstat for the last commit.
///
/// # Returns
///
/// * `String` containing the numstat output.
pub fn get_last_commit_numstat() -> String {
    let show_numstat_output = Command::new("git")
        .args(["show", "--pretty=format:", "--numstat", "HEAD"])
        .output()
        .expect("Failed to get git show --numstat");

    output::print_verbose("Run 'git show --pretty=format: --numstat HEAD'");

    String::from_utf8_lossy(&show_numstat_output.stdout).to_string()
}

/// Gets the diff for specific files in the last commit.
///
/// # Arguments
///
/// * `files` - List of file paths to get diff for.
///
/// # Returns
///
/// * `String` containing the diff output.
pub fn get_last_commit_diff_for_files(files: &[String]) -> String {
    let mut args = vec!["show", "--pretty=format:", "--diff-filter=AM", "HEAD", "--"];
    let file_refs: Vec<&str> = files.iter().map(|s| s.as_str()).collect();
    args.extend(file_refs);

    let show_diff_output = Command::new("git")
        .args(&args)
        .output()
        .expect("Failed to get git show for specific files");

    output::print_verbose(&format!(
        "Run 'git show --pretty=format: --diff-filter=AM HEAD -- {:?}'",
        files
    ));

    String::from_utf8_lossy(&show_diff_output.stdout).to_string()
}

/// Commits changes with the given subject and message.
///
/// # Arguments
///
/// * `subject` - The commit subject/title.
/// * `message` - The commit message body.
/// * `overwrite` - If true, amends the last commit.
///
/// # Returns
///
/// * `true` if commit succeeded, `false` otherwise.
pub fn git_commit(subject: &str, message: &str, overwrite: bool) -> bool {
    let mut commit_args = vec!["commit"];
    if overwrite {
        commit_args.push("--amend");
    }
    commit_args.extend(["-m", subject, "-m", message]);

    output::print_verbose("Run 'git commit -m <subject> -m <message>'");

    let commit_output = Command::new("git")
        .args(&commit_args)
        .output()
        .expect("Failed to execute git commit");

    commit_output.status.success()
}
