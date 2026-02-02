use crate::commands::{ai, prompt};
use crate::core::ai::client;
use crate::utils::output;
use indoc::{eprintdoc, printdoc};

/// Generates commit message from diff content.
///
/// # Arguments
///
/// * `diff_content` - The diff content to generate message from.
/// * `url` - The AI API URL.
/// * `model_name` - The AI model name.
/// * `api_key` - The API key.
/// * `language` - The language for the commit message.
/// * `verbose` - Whether to print verbose output.
/// * `custom_title` - Optional custom title for the commit.
/// * `custom_diff_prompt` - Optional custom diff prompt.
/// * `custom_subject_prompt` - Optional custom subject prompt.
///
/// # Returns
///
/// * `Ok((subject, message))` containing the commit subject and message, or `Err` if generation fails.
pub async fn generate_commit_message(
    mut diff_content: String,
    url: String,
    model_name: String,
    api_key: String,
    language: String,
    verbose: bool,
    custom_title: Option<String>,
    custom_diff_prompt: Option<String>,
    custom_subject_prompt: Option<String>,
) -> Result<(String, String), Box<dyn std::error::Error>> {
    if language != "English" {
        diff_content.push_str(&format!(
            "\n The answer should be in {} language. If you cannot recognize this language, use English instead.",
            language
        ));
    }

    let system = prompt::get_diff_prompt(custom_diff_prompt.as_deref());
    let res = client::chat(
        url.clone(),
        model_name.clone(),
        api_key.clone(),
        Some(system),
        diff_content.clone(),
        verbose,
    )
    .await;

    let file_changes = match res {
        Ok(msg) => msg,
        Err(e) => {
            ai::ai_generating_error(&format!("Error: {}", e), true);
            return Err(e);
        }
    };

    let commit_subject = if let Some(title) = custom_title {
        title
    } else {
        let system = prompt::get_subject_prompt(custom_subject_prompt.as_deref());
        let res = client::chat(
            url,
            model_name,
            api_key,
            Some(system),
            format!("The changes are: \n{}", file_changes),
            verbose,
        )
        .await;

        match res {
            Ok(answer) => answer,
            Err(e) => format!("Error: {}", e),
        }
    };

    output::print_verbose(&format!("AI chat content: {}", diff_content));
    output::print_normal("");
    printdoc!(
        r#"
        >>>>>>>>>>>>>>>>>>>>>>>>>
        Commit subject: "{}"

        Commit message: "{}"
        <<<<<<<<<<<<<<<<<<<<<<<<<
        "#,
        commit_subject,
        file_changes
    );

    Ok((commit_subject, file_changes))
}

/// Commits the generated message to git.
///
/// # Arguments
///
/// * `subject` - The commit subject.
/// * `message` - The commit message body.
/// * `overwrite` - If true, amends the last commit.
pub fn execute_commit(subject: &str, message: &str, overwrite: bool) {
    if crate::core::git::git_commit(subject, message, overwrite) {
        output::print_normal(
            "✅ Successfully committed changes! If you were discontent with the commit message and want to polish or revise it, run 'gim -p' or 'git commit --amend'"
        );
    } else {
        eprintln!("Error: Failed to commit changes");
    }
}

/// Checks if diff content exceeds the line limit.
///
/// # Arguments
///
/// * `diff_content` - The diff content to check.
/// * `diff_limit` - The maximum allowed lines.
///
/// # Returns
///
/// * `Ok(())` if within limit, exits with error if exceeds.
pub fn check_diff_limit(diff_content: &str, diff_limit: usize) -> Result<(), Box<dyn std::error::Error>> {
    if diff_content.lines().count() > diff_limit {
        eprintdoc!(
            r"
            Your changed lines count ({}) exceeds the limit: {}.
            Please use 'git commit' to commit the changes or adjust the limit by 'gim config --change-limit <LIMIT>' and try again.
            ",
            diff_content.lines().count(),
            diff_limit
        );
        std::process::exit(1);
    }
    Ok(())
}
