use cli::{command::GimCli, entry::run_cli, update::check_update_reminder, verbose};
use gim_config::config::get_config;
use std::env;

use crate::cli::verbose::print_verbose;

mod cli;
mod constants;

#[tokio::main]
async fn main() {
    let cli = <GimCli as clap::Parser>::parse();

    // Set global verbose flag
    verbose::set_verbose(cli.verbose);

    // Only show update reminder for the main command, not for subcommands
    if env::args().nth(1).map_or(true, |arg| arg != "update") {
        if let Err(e) = check_update_reminder() {
            print_verbose(&format!("Warning: {}", e));
        }
    }

    // run the cli
    let config = get_config().expect("Failed to access config file");
    run_cli(&cli, config).await;
}
