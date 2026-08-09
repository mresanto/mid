use clap::Subcommand;

use crate::cli::commands::history::commands::HistoryCommands;
use crate::cli::commands::list::commands::ListCommands;
use crate::cli::commands::query::commands::QueryCommands;
use crate::cli::commands::remote::commands::RemoteCommands;
use crate::core::query::QueryOutputFormat;

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
        #[command(subcommand)]
        command: Option<ListCommands>,
    },

    #[command(args_conflicts_with_subcommands = true)]
    Query {
        #[arg()]
        query: Option<String>,
        #[arg(long, value_enum, default_value = "table")]
        output_format: QueryOutputFormat,
        #[command(subcommand)]
        command: Option<QueryCommands>,
        #[arg(long)]
        id: Option<u16>,
    },

    History {
        #[command(subcommand)]
        command: Option<HistoryCommands>,
    },
}
