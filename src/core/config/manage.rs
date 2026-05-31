use std::{fs, io};

use thiserror::Error;

use super::types;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to parse global config file: {0}")]
    TomlSerialize(#[from] toml::ser::Error),

    #[error("Failed to read global config file: {0}")]
    Io(#[from] io::Error),
}

pub fn save_config(file_path: String, content: types::MidConfigFile) -> Result<(), Error> {
    let config_string = toml::to_string_pretty(&content)?;
    fs::write(file_path, config_string)?;

    return Ok(());
}
