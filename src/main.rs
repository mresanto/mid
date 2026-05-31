use clap::Parser;

use crate::cli::{
    Cli,
    commands::{
        Commands, list::handle_list_command, remote::handle_remote_command,
        status::handle_status_command,
    },
};

mod cli;
mod core;

fn main() {
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
        Some(Commands::List { command }) => handle_list_command(command),
        Some(Commands::Status {}) => handle_status_command(),
        None => {}
    }
}
