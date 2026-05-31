use clap::Subcommand;

use crate::core::config::new::{handler::add_remote_config, types::CreateNewConfigOptions};

#[derive(Subcommand)]
pub enum RemoteCommands {
    Add {
        #[arg()]
        connection_string: String,

        #[arg(short, long)]
        name: Option<String>,
        // #[arg(short, long)]
        // global: bool,
    },
}

pub fn handle_remote_command(command: &Option<RemoteCommands>) {
    match command {
        Some(RemoteCommands::Add {
            connection_string,
            // TODO: Feature to allow local or global config file, and add to the correct one based
            // on that.
            // global,
            name,
        }) => {
            let random_name = format!("remote-{}", rand::random::<u32>());

            let arg = CreateNewConfigOptions {
                name: name.clone().unwrap_or_else(|| random_name),
                connection_string: connection_string.clone(),
                global: true,
            };
            let res = add_remote_config(arg);

            match res {
                Ok(_) => println!("Remote config added successfully"),
                Err(e) => eprintln!("Failed to add remote config: {e}"),
            }
        }
        None => {}
    }
}
