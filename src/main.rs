use clap::{CommandFactory, Parser};

use crate::cli::commands::history::handle::handle_history_command;
use crate::cli::commands::list::handle::handle_list_command;
use crate::cli::commands::query::handle::handle_query_command;
use crate::cli::commands::remote::remote_handler;
use crate::cli::commands::status::handle_status_command;
use crate::commands::{Cli, MainCommands, RemoteCommands};

mod cli;
mod commands;
mod core;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let file_path = core::globals::get_global_history_file_path();

    match &cli.command {
        MainCommands::Remote { command } => match command {
            Some(RemoteCommands::List {}) => remote_handler::list(&file_path),
            Some(RemoteCommands::Add {
                name,
                connection_string,
            }) => remote_handler::add(&file_path, name, connection_string),
            Some(RemoteCommands::Remove { name }) => remote_handler::remove(&file_path, name),
            Some(RemoteCommands::Switch { name }) => remote_handler::switch(&file_path, name),
            None => {}
        },
        MainCommands::List {
            output_format,
            table_name,
        } => handle_list_command(output_format, table_name).await,
        MainCommands::Status {} => handle_status_command(),
        MainCommands::Query {
            query,
            output_format,
            command,
            id,
        } => handle_query_command(command, query, output_format, id).await,
        MainCommands::History { command } => handle_history_command(command).await,
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
