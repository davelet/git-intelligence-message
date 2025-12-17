use cli::{
    command::GimCli,
    entry::run_cli,
    output::{self, print_normal, set_quiet, set_verbose, is_verbose, is_quiet},
    update::check_update_reminder_async,
};
use gim_config::config::get_config;
use std::env;
// use std::time::Duration;

mod cli;
mod constants;

#[tokio::main]
async fn main() {
    let cli = <GimCli as clap::Parser>::parse();

    // Set global flags
    set_quiet(cli.quiet);
    set_verbose(cli.verbose);

    let start_time = std::time::Instant::now();
    // Start update reminder check asynchronously in background
    // Skip if --dry flag is passed or if this is the update subcommand
    let update_check_handle = if !cli.dry && env::args().nth(1).map_or(true, |arg| arg != "update") {
        Some(tokio::spawn(check_update_reminder_async()))
    } else {
        None
    };

    // run the cli
    let config = get_config().expect("Failed to access config file");
    if let Err(e) = run_cli(&cli, config).await {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
    print_normal("");

    // Give the background update check task a chance to complete
    if let Some(handle) = update_check_handle {
        if !handle.is_finished() {
            output::print_normal("Waiting for update check to complete... (Ctrl+C to exit)");
        }
        let _ = handle.await;
    }

    if is_verbose() && !is_quiet() {
        let time = start_time.elapsed();
        output::print_verbose(&format!("Time elapsed: {:?}", time));
    }
}
