use clap::Subcommand;

use crate::cli::commands::history::commands::HistoryCommands;
// use crate::cli::commands::list::commands::ListCommands;
use crate::cli::commands::query::commands::QueryCommands;
use crate::cli::commands::remote::commands::RemoteCommands;
use crate::core::query::QueryOutputFormat;

#[derive(Subcommand)]
pub enum Commands {
    #[command(arg_required_else_help = true)]
    Remote {
        #[command(subcommand)]
        command: Option<RemoteCommands>,
    },

    Status {},

    List {
        // #[command(subcommand)]
        // command: Option<ListCommands>,
        #[arg(short, long)]
        table_name: Option<String>,

        #[arg(long, value_enum, default_value = "table")]
        output_format: QueryOutputFormat,
    },

    #[command(args_conflicts_with_subcommands = true, arg_required_else_help = true)]
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

    #[command(arg_required_else_help = true)]
    History {
        #[command(subcommand)]
        command: Option<HistoryCommands>,
    },
}
