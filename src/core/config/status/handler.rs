use std::{fs, io};

use thiserror::Error;

use crate::core::{config::types::MidConfigFile, globals};

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to serialize global config file: {0}")]
    TomlDeserialize(#[from] toml::de::Error),

    #[error("No active database found in the config file")]
    NoActiveRemote,

    #[error("Failed to read global config file: {0}")]
    Io(#[from] io::Error),
}

pub fn get_current_config() -> Result<MidConfigFile, Error> {
    let file_path = globals::get_global_config_file_path();

    let contents = fs::read_to_string(&file_path)?;

    let config = toml::from_str::<MidConfigFile>(&contents)?;

    let active_connection = config.get_active_database();

    match active_connection {
        Some(_) => Ok(config),
        None => Err(Error::NoActiveRemote),
    }
}

mod tests {
    use crate::core::config::new::handler;
    use crate::core::config::new::types::CreateNewConfigOptions;

    use super::*;

    #[test]
    fn test_add_remote_config_global() {
        let options = CreateNewConfigOptions {
            name: "test".to_string(),
            connection_string: "postgres://user:password@localhost:5432/db".to_string(),
            global: true,
        };

        handler::add_remote_config(options).expect("Failed to add remote config to global file");

        let config = get_current_config().expect("Failed to get current config");

        assert_eq!(config.active_remote, Some("test".to_string()));
    }
}
