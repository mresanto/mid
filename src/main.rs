use clap::{CommandFactory, Parser};

use crate::cli::Cli;
use crate::cli::commands::history::handle::handle_history_command;
use crate::cli::commands::list::handle::handle_list_command;
use crate::cli::commands::main_commands::Commands;
use crate::cli::commands::query::handle::handle_query_command;
use crate::cli::commands::remote::handle::handle_remote_command;
use crate::cli::commands::status::handle_status_command;

mod cli;
mod core;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Remote { command } => handle_remote_command(command),
        Commands::List {
            output_format,
            table_name,
        } => handle_list_command(output_format, table_name).await,
        Commands::Status {} => handle_status_command(),
        Commands::Query {
            query,
            output_format,
            command,
            id,
        } => handle_query_command(command, query, output_format, id).await,
        Commands::History { command } => handle_history_command(command).await,
        Commands::Generator { shell } => {
            eprintln!("Generating completion file for {shell}...");
            let mut cmd = Cli::command()
                .disable_help_flag(true)
                .disable_help_subcommand(true);
            clap_complete::generate(*shell, &mut cmd, "mid", &mut std::io::stdout());
            eprintln!("Done!");
        }
    }
}
