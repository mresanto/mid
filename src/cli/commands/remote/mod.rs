use clap::Subcommand;

use crate::core::{
    config::{manage, types::DatabaseConfig},
    globals,
};

#[derive(Subcommand)]
pub enum RemoteCommands {
    List {},
    Add {
        #[arg()]
        connection_string: String,

        #[arg(short, long)]
        name: Option<String>,
        // #[arg(short, long)]
        // global: bool,
    },
    Remove {
        #[arg()]
        name: String,
    },
    Switch {
        #[arg()]
        name: String,
    },
}

pub fn handle_remote_command(command: &Option<RemoteCommands>) {
    let file_path = globals::get_global_config_file_path();
    match command {
        Some(RemoteCommands::List {}) => {
            let res = manage::read_databases(file_path);

            match res {
                Ok(databases) => {
                    println!("Databases: ");
                    for database in databases {
                        println!(" {}", database.name);
                    }
                }
                Err(e) => eprintln!("Failed to list remote configs: {e}"),
            }
        }
        Some(RemoteCommands::Add {
            connection_string,
            name,
        }) => {
            let random_name = format!("remote-{}", rand::random::<u32>());
            let real_name = name.clone().unwrap_or_else(|| random_name);

            let res = manage::add_database(
                file_path,
                DatabaseConfig {
                    name: real_name.clone(),
                    connection_string: connection_string.clone(),
                },
            );

            match res {
                Ok(_) => println!("Remote config added successfully. Database: {}", real_name),
                Err(e) => eprintln!("Failed to add remote config: {e}"),
            }
        }
        Some(RemoteCommands::Switch { name }) => {
            let res = manage::change_active_database(file_path, name.clone());

            match res {
                Ok(_) => println!("Switched active connection to {}", name),
                Err(e) => eprintln!("Failed to switch active connection: {e}"),
            }

            return;
        }
        Some(RemoteCommands::Remove { name }) => {
            let res = manage::remove_database(file_path, name.clone());

            match res {
                Ok(_) => println!("Remote config removed successfully. Database: {}", name),
                Err(e) => eprintln!("Failed to remove remote config: {e}"),
            }

            return;
        }
        None => {}
    }
}
