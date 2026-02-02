use cli::{GimCli, GimCommands};
use commands::{ai as commands_ai, config as commands_config, commit, prompt, update};
use core::{ai::client, diff, git};
use gim_config::config::get_config;
use std::env;

mod cli;
mod commands;
mod config;
mod core;
mod utils;

#[tokio::main]
async fn main() {
    let cli = <GimCli as clap::Parser>::parse();

    // Set global flags
    utils::output::set_quiet(cli.quiet);
    utils::output::set_verbose(cli.verbose);

    let start_time = std::time::Instant::now();
    // Start update reminder check asynchronously in background
    // Skip if --dry flag is passed or if this is the update subcommand
    let update_check_handle = if !cli.dry && env::args().nth(1).map_or(true, |arg| arg != "update") {
        Some(tokio::spawn(update::check_update_reminder_async()))
    } else {
        None
    };

    // Run the cli
    let config = get_config().expect("Failed to access config file");
    if let Err(e) = run_cli(&cli, config).await {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
    utils::output::print_normal("");

    // Give the background update check task a chance to complete
    if let Some(handle) = update_check_handle {
        if !handle.is_finished() {
            utils::output::print_normal("Waiting for update check to complete... (Ctrl+C to exit)");
        }
        let _ = handle.await;
    }

    if utils::output::is_verbose() && !utils::output::is_quiet() {
        let time = start_time.elapsed();
        utils::output::print_verbose(&format!("Time elapsed: {:?}", time));
    }
}

async fn run_cli(cli: &GimCli, mut config: toml::Value) -> Result<(), Box<dyn std::error::Error>> {
    match &cli.command {
        Some(GimCommands::Update {
            force,
            max,
            interval,
        }) => {
            if max.is_some() || interval.is_some() {
                if *force {
                    eprintln!("Warning: won't update when --max or --interval provided");
                }
                'max: {
                    if let Some(max) = max {
                        if *max <= 0 {
                            eprintln!("Error: --max must be a positive integer");
                            break 'max;
                        }
                        if let Err(e) = update::set_max_try((*max).try_into().unwrap()) {
                            eprintln!("Failed to set max try: {}", e);
                            break 'max;
                        }
                    }
                }
                'interval: {
                    if let Some(interval) = interval {
                        if *interval <= 0 {
                            eprintln!("Error: --interval must be a positive integer");
                            break 'interval;
                        }
                        if let Err(e) =
                            update::set_try_interval((*interval).try_into().unwrap())
                        {
                            eprintln!("Failed to set try interval: {}", e);
                            break 'interval;
                        }
                    }
                }
            } else {
                if let Err(e) = update::check_and_install_update(*force).await {
                    eprintln!("Failed to update: {}", e);
                    std::process::exit(1);
                }
            }
            return Ok(());
        }
        Some(GimCommands::Prompt {
            edit,
            prompt,
            editor,
            reset,
        }) => {
            if *reset {
                if *edit || prompt.is_some() || editor.is_some() {
                    utils::output::print_warning("--edit, --prompt or --editor will be ignored when --reset provided");
                }
                // delete the 2 files
                if let Err(e) = prompt::delete_prompt_files() {
                    eprintln!("Error in reset prompt: {}", e);
                    std::process::exit(1);
                }
            } else if let Err(e) =
                prompt::handle_prompt_command(*edit, prompt.as_deref(), editor.as_deref())
            {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
            return Ok(());
        }
        Some(GimCommands::Ai {
            model,
            apikey,
            url,
            language,
        }) => {
            // Check if -k is used without a value (empty string means flag was provided but no value)
            if let Some(apikey_val) = apikey {
                if apikey_val.is_empty() {
                    // User ran: gim ai -k (without value)
                    // Show current API key (full, not masked)
                    let ai = commands_ai::get_validated_ai_config(false, false);
                    if let Some(ai) = ai {
                        utils::output::print_normal(&format!("Current API Key: {}", &ai.2));
                    } else {
                        eprintln!("Error: ai section is not configured");
                    }
                    return Ok(());
                }
            }

            if model.is_none() && apikey.is_none() && url.is_none() && language.is_none() {
                let ai = commands_ai::get_validated_ai_config(false, false);
                if let Some(ai) = ai {
                    let mut url = ai.0;
                    if url.is_empty() && !ai.1.is_empty() {
                        if let Some(str) = client::get_url_by_model(&ai.1) {
                            url = format!("(not configured. Will use default : {})", str);
                        } else {
                            eprintln!("Warning: you have not setup api url by 'gim ai -u <url>'");
                        }
                    }
                    let masked_key = commands_ai::mask_api_key(&ai.2);
                    indoc::printdoc!(
                        r#"
                        Model:      {}
                        API Key:    {}
                        URL:        {}
                        Language:   {}
                        You can use 'gim ai -m <model> -k <apikey> -u <url> -l <language>' respectively to update the configuration
                        "#,
                        &ai.1,
                        &masked_key,
                        &url,
                        &ai.3
                    );
                } else {
                    eprintln!("Error: ai section is not configured");
                }
                return Ok(());
            }
            commands_ai::update_ai_config(&mut config, model, apikey, url, language);
            return Ok(());
        }
        Some(GimCommands::Config {
            lines_limit,
            show_location,
        }) => {
            if *show_location {
                if let Err(e) = commands_config::get_config_and_print() {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
                if let Err(e) = prompt::open_config_directory() {
                    eprintln!("Error: {}", e);
                }
            }
            if let Some(lines_limit) = lines_limit {
                if let Err(e) = commands_config::set_lines_limit(*lines_limit) {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
            return Ok(());
        }
        None => {}
    }

    // Check if current directory is a git repository
    if !git::is_git_repo() {
        utils::output::print_warning("The current directory is not a git repository.");
        return Ok(());
    }

    // Get git status
    let changes: Vec<String> = git::get_git_status(cli.auto_add);

    // Build diff content
    let changes_ref: Vec<&str> = changes.iter().map(|s| s.as_str()).collect();
    let diff_content = diff::build_diff_content(cli.auto_add, &changes_ref, cli.overwrite);

    if diff_content.is_empty() {
        utils::output::print_normal("No changes to commit.");
        return Ok(());
    }

    // DRY RUN LOGIC
    if cli.dry {
        utils::output::print_normal(
            &format!("\n--- DRY RUN ---\nContent to be sent to AI:\n{}", diff_content)
        );
        return Ok(());
    }

    // Check diff limit
    let diff_limit = commands_config::get_lines_limit();
    commit::check_diff_limit(&diff_content, diff_limit)?;

    // Get AI config
    let config_result = commands_ai::get_validated_ai_config(cli.auto_add, changes.len() > 0);
    if config_result.is_none() {
        return Ok(());
    }
    let (url, model_name, api_key, language) = config_result.unwrap();

    // Generate commit message
    let (subject, message) = commit::generate_commit_message(
        diff_content,
        url,
        model_name,
        api_key,
        language,
        cli.verbose,
        cli.title.clone(),
        cli.diff_prompt.clone(),
        cli.subject_prompt.clone(),
    )
    .await?;

    // Execute commit
    commit::execute_commit(&subject, &message, cli.overwrite);

    Ok(())
}
