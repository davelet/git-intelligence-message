use clap::{Parser, Subcommand};

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
    pub update: bool,

    /// Show verbose output
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,
}

#[derive(Subcommand)]
pub enum GimCommands {
    /// Check for updates and install the latest version
    Update {
        /// Force update even if the current version is the latest
        #[arg(short, long, default_value_t = false)]
        force: bool,
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
    },

    /// Setup the ai-api configuration
    Ai {
        /// the ai model name
        #[arg(short, long)]
        model: Option<String>,

        /// the ai api key
        #[arg(short = 'k', long)]
        apikey: Option<String>,

        /// the ai api url
        #[arg(short, long)]
        url: Option<String>,

        /// the answer language
        #[arg(short, long)]
        language: Option<String>,
    },
}
