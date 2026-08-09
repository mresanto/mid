use clap::Parser;

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

    // You can check the value provided by positional arguments, or option arguments
    if let Some(name) = cli.name.as_deref() {
        println!("Value for name: {name}");
    }

    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Test { list }) => {
            if *list {
                println!("Printing testing lists...");
            } else {
                println!("Not printing testing lists...");
            }
        }

        Some(Commands::Remote { command }) => handle_remote_command(command),
        Some(Commands::List { command }) => handle_list_command(command).await,
        Some(Commands::Status {}) => handle_status_command(),
        Some(Commands::Query {
            query,
            output_format,
            command,
            id,
        }) => handle_query_command(command, query, output_format, id).await,
        Some(Commands::History { command }) => handle_history_command(command).await,
        _ => {}
    }
}
