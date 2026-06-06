use clap::Subcommand;

use crate::cli::commands::remote::RemoteCommands;

pub mod list;
pub mod query;
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

    List {
        // #[command(subcommand)]
        // command: Option<list::ListCommands>,
        #[arg(short, long)]
        output_format: Option<query::QueryOutputFormat>,
    },

    Query {
        #[arg()]
        query: String,
        #[arg(short, long)]
        output_format: Option<query::QueryOutputFormat>,
    },
}
