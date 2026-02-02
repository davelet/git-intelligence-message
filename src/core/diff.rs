use crate::core::git;
use crate::utils::output;

/// Builds the diff content for staging area changes.
///
/// # Returns
///
/// * `String` containing the formatted diff content, or empty string if no changes.
pub fn build_staging_diff() -> String {
    let mut diff_content = String::new();

    let name_status = git::get_staged_name_status();
    let full_diff = git::get_staged_diff();

    if !name_status.is_empty() {
        diff_content.push_str("When I use `git diff`, I got the following output: \n");

        // Add file status information (including deleted files)
        for line in name_status.lines() {
            if let Some((status, filename)) = line.split_once('\t') {
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
/// # Returns
///
/// * `String` containing the formatted diff content.
pub fn build_last_commit_diff() -> String {
    let mut diff_content = String::new();
    diff_content.push_str(
        "As I want to amend commit message, I use `git show` and got the following output: \n",
    );

    let name_status = git::get_last_commit_name_status();
    let full_diff = git::get_last_commit_diff();

    // Parse name-status, only output filename for deleted files
    for line in name_status.lines() {
        if let Some((status, filename)) = line.split_once('\t') {
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
///
/// # Returns
///
/// * `String` containing the complete diff content, or empty if no changes.
pub fn build_diff_content(auto_add: bool, changes: &[&str], overwrite: bool) -> String {
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

        diff_content.push_str(&build_staging_diff());
    }

    if overwrite {
        diff_content.push_str(&build_last_commit_diff());
    }

    diff_content
}
