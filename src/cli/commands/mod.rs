use clap::Subcommand;

use crate::cli::commands::remote::RemoteCommands;

pub mod remote;
pub mod status;

#[derive(Subcommand)]
pub enum Commands {
    /// does testing things
    Test {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },

    Remote {
        #[command(subcommand)]
        command: Option<RemoteCommands>,
    },

    Status {},
}
