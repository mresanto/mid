use clap::CommandFactory;
use clap::Parser;

use crate::cli::commands::history_handler;
use crate::cli::commands::list_handler;
use crate::cli::commands::query_handler;
use crate::cli::commands::remote_handler;
use crate::cli::commands::status_handler;
use crate::cli::config::Cli;
use crate::cli::config::HistoryCommands;
use crate::cli::config::MainCommands;
use crate::cli::config::QueryCommands;
use crate::cli::config::RemoteCommands;

mod cli;
mod core;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        MainCommands::Remote { command } => match command {
            Some(RemoteCommands::List {}) => remote_handler::list(),
            Some(RemoteCommands::Add {
                name,
                connection_string,
            }) => remote_handler::add(name, connection_string),
            Some(RemoteCommands::Remove { name }) => remote_handler::remove(name),
            Some(RemoteCommands::Switch { name }) => remote_handler::switch(name),
            None => {}
        },
        MainCommands::List {
            output_format,
            table_name,
        } => list_handler::list(output_format, table_name).await,
        MainCommands::Status {} => status_handler::status(),
        MainCommands::Query {
            query,
            output_format,
            command,
            id,
        } => match command {
            Some(QueryCommands::Last {
                output_format,
                skip,
            }) => query_handler::last(output_format, *skip).await,
            None => match id {
                Some(id) => query_handler::by_id(output_format, id).await,
                None => query_handler::query(query, output_format).await,
            },
        },
        MainCommands::History { command } => match command {
            Some(HistoryCommands::List {}) => history_handler::list(),
            Some(HistoryCommands::Last {}) => history_handler::last(),
            None => {}
        },
        MainCommands::Generator { shell } => {
            eprintln!("Generating completion file for {shell}...");
            let mut cmd = Cli::command()
                .disable_help_flag(true)
                .disable_help_subcommand(true);
            clap_complete::generate(*shell, &mut cmd, "mid", &mut std::io::stdout());
            eprintln!("Done!");
        }
    }
}
