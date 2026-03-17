use crate::core::git;
use crate::utils::output;
use std::collections::HashSet;

/// File type classification for prioritization.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FileType {
    Code,   // Programming language files
    Config, // Configuration files (xml, toml, yaml, json, etc.)
    Doc,    // Documentation files (md, txt, rst, etc.)
    Other,  // Other files
}

/// Represents a changed file with metadata.
#[derive(Debug, Clone)]
struct FileChange {
    path: String,
    lines_changed: usize,
    file_type: FileType,
}

/// Classifies a file based on its extension.
fn classify_file_type(path: &str) -> FileType {
    let extension = path.split('.').last().unwrap_or("").to_lowercase();

    match extension.as_str() {
        // Code files
        "rs" | "go" | "py" | "js" | "ts" | "jsx" | "tsx" | "java" | "c" | "cpp" | "cc" | "h"
        | "hpp" | "cs" | "php" | "rb" | "swift" | "kt" | "scala" | "m" | "mm" | "dart" | "ex"
        | "exs" | "erl" | "hrl" | "clj" | "cljs" | "hs" | "ml" | "mli" | "fs" | "fsx" | "r"
        | "jl" | "lua" | "pl" | "pm" | "sh" | "bash" | "zsh" | "fish" | "vim" | "el" => {
            FileType::Code
        }

        // Config files
        "xml" | "toml" | "yaml" | "yml" | "json" | "ini" | "cfg" | "conf" | "config"
        | "properties" | "env" | "lock" => FileType::Config,

        // Documentation files
        "md" | "txt" | "rst" | "adoc" | "org" | "tex" => FileType::Doc,

        _ => FileType::Other,
    }
}

/// Parses git numstat output into FileChange objects.
fn parse_diff_stats(numstat: &str, name_status: &str) -> Vec<FileChange> {
    let mut changes = Vec::new();
    let mut status_map = std::collections::HashMap::new();

    // Parse name-status to get file status (A/M/D)
    for line in name_status.lines() {
        if let Some((status, filename)) = line.split_once('\t') {
            status_map.insert(filename.to_string(), status.chars().next().unwrap_or('M'));
        }
    }

    // Parse numstat
    for line in numstat.lines() {
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() >= 3 {
            let added = parts[0].parse::<usize>().unwrap_or(0);
            let deleted = parts[1].parse::<usize>().unwrap_or(0);
            let path = parts[2].to_string();
            let lines_changed = added + deleted;
            let status = *status_map.get(&path).unwrap_or(&'M');

            // Skip deleted files (they don't contribute to the diff content)
            if status != 'D' && lines_changed > 0 {
                changes.push(FileChange {
                    path: path.clone(),
                    lines_changed,
                    file_type: classify_file_type(&path),
                });
            }
        }
    }

    changes
}

/// Selects files to include based on limits and priorities.
fn select_files(changes: Vec<FileChange>, max_files: usize) -> Vec<String> {
    if changes.is_empty() {
        return Vec::new();
    }

    // Calculate total lines and code file lines
    let total_lines: usize = changes.iter().map(|c| c.lines_changed).sum();
    let code_lines: usize = changes
        .iter()
        .filter(|c| c.file_type == FileType::Code)
        .map(|c| c.lines_changed)
        .sum();

    // If code changes are more than 50%, only keep code files
    let mut filtered_changes: Vec<FileChange> = if code_lines * 2 > total_lines {
        output::print_verbose(&format!(
            "Code changes ({} lines) exceed 50% of total ({} lines), filtering to code files only",
            code_lines, total_lines
        ));
        changes.into_iter()
            .filter(|c| c.file_type == FileType::Code)
            .collect()
    } else {
        output::print_verbose(&format!(
            "Code changes ({} lines) do not exceed 50% of total ({} lines), keeping all files",
            code_lines, total_lines
        ));
        changes
    };

    // Sort by lines_changed descending to identify most significant changes
    filtered_changes.sort_by(|a, b| b.lines_changed.cmp(&a.lines_changed));

    // Take top N files by lines changed if max_files is set
    if max_files > 0 {
        filtered_changes.into_iter()
            .take(max_files)
            .map(|c| c.path)
            .collect()
    } else {
        filtered_changes.into_iter()
            .map(|c| c.path)
            .collect()
    }
}

/// Builds the diff content for staging area changes.
///
/// # Arguments
///
/// * `selected_files` - Optional list of files to include. If None, includes all files.
///
/// # Returns
///
/// * `String` containing the formatted diff content, or empty string if no changes.
pub fn build_staging_diff(selected_files: Option<&[String]>) -> String {
    let mut diff_content = String::new();

    let name_status = git::get_staged_name_status();
    let full_diff = if let Some(files) = selected_files {
        if files.is_empty() {
            String::new()
        } else {
            git::get_staged_diff_for_files(files)
        }
    } else {
        git::get_staged_diff()
    };

    if !name_status.is_empty() {
        diff_content.push_str("When I use `git diff`, I got the following output: \n");

        // Add file status information (including deleted files)
        // If selected_files is provided, only show those files
        let selected_set: Option<HashSet<&str>> =
            selected_files.map(|files| files.iter().map(|s| s.as_str()).collect());

        for line in name_status.lines() {
            if let Some((status, filename)) = line.split_once('\t') {
                // Skip if not in selected files
                if let Some(ref set) = selected_set {
                    if !set.contains(filename) && status != "D" {
                        continue;
                    }
                }

                if status == "D" {
                    diff_content.push_str(&format!("Deleted: {}\n", filename));
                } else {
                    diff_content.push_str(line);
                    diff_content.push('\n');
                }
            } else {
                diff_content.push_str(line);
                diff_content.push('\n');
            }
        }
        diff_content.push_str("\n");

        // Add full diff content only for added/modified files
        if !full_diff.is_empty() {
            diff_content.push_str(
                "\nDetailed changes for added/modified files (excluding deleted files):\n",
            );
            diff_content.push_str(&full_diff);
            diff_content.push_str("\n");
        }
    }

    diff_content
}

/// Builds the diff content for the last commit.
///
/// # Arguments
///
/// * `selected_files` - Optional list of files to include. If None, includes all files.
///
/// # Returns
///
/// * `String` containing the formatted diff content.
pub fn build_last_commit_diff(selected_files: Option<&[String]>) -> String {
    let mut diff_content = String::new();
    diff_content.push_str(
        "As I want to amend commit message, I use `git show` and got the following output: \n",
    );

    let name_status = git::get_last_commit_name_status();
    let full_diff = if let Some(files) = selected_files {
        if files.is_empty() {
            String::new()
        } else {
            git::get_last_commit_diff_for_files(files)
        }
    } else {
        git::get_last_commit_diff()
    };

    // Parse name-status, only output filename for deleted files
    let selected_set: Option<HashSet<&str>> =
        selected_files.map(|files| files.iter().map(|s| s.as_str()).collect());

    for line in name_status.lines() {
        if let Some((status, filename)) = line.split_once('\t') {
            // Skip if not in selected files
            if let Some(ref set) = selected_set {
                if !set.contains(filename) && status != "D" {
                    continue;
                }
            }

            if status == "D" {
                diff_content.push_str(&format!("Deleted: {}\n", filename));
            } else {
                diff_content.push_str(line);
                diff_content.push('\n');
            }
        } else {
            diff_content.push_str(line);
            diff_content.push('\n');
        }
    }
    diff_content.push_str("\n");

    // Append detailed diff only for added/modified files
    if !full_diff.is_empty() {
        diff_content.push_str("\nDetailed changes for added/modified files in last commit (excluding deleted files):\n");
        diff_content.push_str(&full_diff);
        diff_content.push_str("\n");
    }

    output::print_normal("As '-p' option is enabled, I will amend the last commit message");

    diff_content
}

/// Builds complete diff content for commit generation.
///
/// # Arguments
///
/// * `auto_add` - If true, automatically adds changes before building diff.
/// * `changes` - List of changed files.
/// * `overwrite` - If true, includes last commit diff.
/// * `max_files` - Maximum number of files to include in diff (0 means no limit).
///
/// # Returns
///
/// * `String` containing the complete diff content, or empty if no changes.
pub fn build_diff_content(
    auto_add: bool,
    changes: &[&str],
    overwrite: bool,
    max_files: usize,
) -> String {
    let mut diff_content = String::new();

    if !changes.is_empty() {
        output::print_normal(&format!("Found {} changes:", changes.len()));
        for entry in changes.iter() {
            output::print_normal(&format!(
                "{:?} {}",
                entry,
                if !auto_add && (entry.starts_with(' ') || entry.starts_with('?')) {
                    " - <<Ignored>>"
                } else {
                    ""
                }
            ));
        }

        // Auto add changes if enabled
        if auto_add {
            git::git_add_all();
        }

        // Select files based on max_files limit and 50% rule
        let selected_files = {
            let numstat = git::get_staged_numstat();
            let name_status = git::get_staged_name_status();
            let file_changes = parse_diff_stats(&numstat, &name_status);

            let selected = select_files(file_changes, max_files);
            if !selected.is_empty() && selected.len() < changes.len() {
                output::print_normal(&format!(
                    "Limiting diff to {} most significant files (out of {} total changes)",
                    selected.len(),
                    changes.len()
                ));
                Some(selected)
            } else if !selected.is_empty() {
                Some(selected)
            } else {
                None
            }
        };

        diff_content.push_str(&build_staging_diff(selected_files.as_deref()));
    }

    if overwrite {
        // Apply same file selection logic to last commit
        let selected_files = {
            let numstat = git::get_last_commit_numstat();
            let name_status = git::get_last_commit_name_status();
            let file_changes = parse_diff_stats(&numstat, &name_status);

            let selected = select_files(file_changes, max_files);
            if !selected.is_empty() {
                output::print_verbose(&format!(
                    "Limiting last commit diff to {} files",
                    selected.len()
                ));
                Some(selected)
            } else {
                None
            }
        };

        diff_content.push_str(&build_last_commit_diff(selected_files.as_deref()));
    }

    diff_content
}
