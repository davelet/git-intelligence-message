use clap::{Parser, Subcommand};

/// Command-line interface structure for the gim tool, using clap for argument parsing.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct GimCli {
    #[command(subcommand)]
    pub command: Option<GimCommands>,

    /// The commit message title
    #[arg(short, long)]
    pub title: Option<String>,

    /// Auto add the changes to the stage
    #[arg(short, long, default_value_t = false)]
    pub auto_add: bool,

    /// Ammend the last commit
    #[arg(short = 'p', long, default_value_t = false)]
    pub overwrite: bool,

    /// Show verbose output
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,

    /// Suppress all normal output (overrides verbose)
    #[arg(short, long, default_value_t = false)]
    pub quiet: bool,

    /// Dry run: only print the content to be sent to AI, do not actually send or commit
    #[arg(long, default_value_t = false)]
    pub dry: bool,
}

/// Enum representing all supported subcommands for the gim CLI.
#[derive(Subcommand)]
pub enum GimCommands {
    /// Check for updates and install the latest version
    Update {
        /// Force update even if the current version is the latest
        #[arg(short, long, default_value_t = false)]
        force: bool,

        /// Set the max_try param
        #[arg(long)]
        max: Option<usize>,

        /// Set the try_interval_days param
        #[arg(long)]
        interval: Option<usize>,
    },

    /// Manage ai model prompt files. Show content when no options specified
    Prompt {
        /// Optional: Edit the prompt files
        #[arg(short, long)]
        edit: bool,

        /// Optional: Specify which prompt to edit (d or diff or diff_prompt or subject_prompt)
        #[arg(short = 't', long)]
        prompt: Option<String>,

        /// Optional: Specify the editor to use (e.g., vim, code, nano)
        #[arg(short = 'o', long)]
        editor: Option<String>,

        /// Optional: Reset the prompt to default
        #[arg(long, default_value_t = false)]
        reset: bool,
    },

    /// Setup the ai-api configuration
    Ai {
        /// the ai model name
        #[arg(short, long)]
        model: Option<String>,

        /// the ai api key (use -k without value to view current key)
        #[arg(short = 'k', long, num_args = 0..=1, default_missing_value = "")]
        apikey: Option<String>,

        /// the ai api url
        #[arg(short, long)]
        url: Option<String>,

        /// the answer language
        #[arg(short, long)]
        language: Option<String>,
    },

    /// Setup the git configuration
    Config {
        /// Git commit changed lines limit
        #[arg(long)]
        lines_limit: Option<usize>,

        /// Print config file's location
        #[arg(long, default_value_t = false)]
        show_location: bool,
    },
}
