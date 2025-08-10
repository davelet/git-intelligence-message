use cli::{command::GimCli, entry::run_cli, update::check_update_reminder, verbose};
use gim_config::config::get_config;
use std::env;

mod cli;
mod constants;

/// Main entry point for the GIM application.
/// 
/// This application is designed to work across multiple platforms:
/// - Windows (7 and later)
/// - macOS (10.12 and later)
/// - Linux (most distributions)
/// 
/// The application uses platform-specific code for certain operations like
/// opening file managers, but the core functionality is cross-platform.
#[tokio::main]
async fn main() {
    let cli = <GimCli as clap::Parser>::parse();

    // Set global verbose flag
    verbose::set_verbose(cli.verbose);

    // Only show update reminder for the main command, not for subcommands
    if env::args().nth(1).is_none_or(|arg| arg != "update")
        && let Err(e) = check_update_reminder()
    {
        eprintln!("Warning: {}", e)
    }

    // run the cli
    let config = get_config().expect("Failed to access config file");
    run_cli(&cli, config).await;
}
