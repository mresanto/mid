use crate::core::{
    config::{
        new::types::CreateNewConfigOptions,
        types::{DatabaseConfig, MidConfigFile},
    },
    globals,
};
use std::{fs, io};
use thiserror::Error;
use toml;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to parse global config file: {0}")]
    TomlSerialize(#[from] toml::ser::Error),

    #[error("Failed to serialize global config file: {0}")]
    TomlDeserialize(#[from] toml::de::Error),

    #[error("Failed to read global config file: {0}")]
    Io(#[from] io::Error),
}

pub fn add_remote_config_to_global_file(options: CreateNewConfigOptions) -> Result<(), Error> {
    let file_path = globals::get_global_config_file_path();

    add_remote_to_file(file_path, options)
}

pub fn add_remote_config_to_local_file(options: CreateNewConfigOptions) -> Result<(), Error> {
    let file_path = get_cwd_config();

    add_remote_to_file(file_path, options)
}

pub fn get_cwd_config() -> String {
    let cwd = std::env::current_dir()
        .expect("Failed to get current working directory")
        .to_str()
        .unwrap()
        .to_string();

    let file_path = format!("{}/{}", cwd, globals::CONFIG_FILE_NAME);

    file_path
}

pub fn add_remote_to_file(file_path: String, options: CreateNewConfigOptions) -> Result<(), Error> {
    let contents = fs::read_to_string(&file_path);

    let mut content = match contents {
        Ok(c) => {
            let config = toml::from_str::<MidConfigFile>(&c)?;

            Ok(config)
        }
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(MidConfigFile::default()),
        Err(e) => Err(Error::Io(e)),
    }?;

    // TODO: Add Validation to assert that name is unique, and connection string is valid.

    content.databases.push(DatabaseConfig {
        connection_string: options.connection_string,
        name: options.name.clone(),
    });

    content.active_remote = Some(options.name);

    let config_string = toml::to_string_pretty(&content)?;

    fs::write(file_path, config_string)?;

    return Ok(());
}

mod tests {
    use super::*;

    #[test]
    fn test_add_remote_config_global() {
        let options = CreateNewConfigOptions {
            name: "test".to_string(),
            connection_string: "postgres://user:password@localhost:5432/db".to_string(),
            global: true,
        };

        add_remote_config_to_global_file(options)
            .expect("Failed to add remote config to global file");
    }

    use super::*;

    #[test]
    fn test_add_remote_config_local() {
        let options = CreateNewConfigOptions {
            name: "test".to_string(),
            connection_string: "postgres://user:password@localhost:5432/db".to_string(),
            global: true,
        };

        add_remote_config_to_local_file(options)
            .expect("Failed to add remote config to global file");
    }
}
