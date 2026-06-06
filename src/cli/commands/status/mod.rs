use crate::core::{config::manage, globals};

pub fn handle_status_command() {
    let file_path = globals::get_global_config_file_path();
    let current_config = manage::read_config(file_path);

    match current_config {
        Ok(config) => {
            let active_remote = config.get_active_database();

            match active_remote {
                Some(remote) => println!("Active remote: {}", remote.name),
                None => println!("No active remote found in the config file"),
            }
        }
        Err(e) => eprintln!("Failed to get current config: {e}"),
    }
}
