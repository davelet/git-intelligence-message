use cli::{command::GimCli, entry::run_cli, update::check_update_reminder_async, verbose};
use gim_config::config::get_config;
use std::env;
// use std::time::Duration;

mod cli;
mod constants;

#[tokio::main]
async fn main() {
    let cli = <GimCli as clap::Parser>::parse();

    // Set global verbose flag
    verbose::set_verbose(cli.verbose);

    let start_time = std::time::Instant::now();
    // Start update reminder check asynchronously in background
    // Only show update reminder for the main command, not for subcommands
    let update_check_handle = if env::args().nth(1).map_or(true, |arg| arg != "update") {
        Some(tokio::spawn(check_update_reminder_async()))
    } else {
        None
    };

    // run the cli
    let config = get_config().expect("Failed to access config file");
    run_cli(&cli, config).await;
    println!();

    // Give the background update check task a chance to complete
    // If it's still running, wait up to 2 seconds for it to finish
    if let Some(handle) = update_check_handle {
        // let _ = tokio::time::timeout(Duration::from_secs(6), handle).await;
        let _ = handle.await;
    }

    if cli.verbose {
        let time = start_time.elapsed();
        cli::verbose::print_verbose(&format!("Time elapsed: {:?}", time));
    }
}
