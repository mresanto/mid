use thiserror::Error;

use crate::core::config::{manage, status::handler};

pub struct ChangeConnectionOptions {
    pub connection_name: String,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to read config file: {0}")]
    CurrentConnectionError(#[from] handler::Error),

    #[error("Fail to save the file")]
    SaveFileError(#[from] manage::Error),

    #[error("Connection not found in the config file")]
    ConnectionNotFoundError,
}

pub fn switch_connection(options: ChangeConnectionOptions) -> Result<(), Error> {
    let mut config = handler::get_current_config()?;

    let connection_exists = config.config.connection_exists(&options.connection_name);

    if connection_exists == false {
        return Err(Error::ConnectionNotFoundError);
    }

    config
        .config
        .set_active_database(options.connection_name.clone());

    manage::save_config(config.path, config.config)?;

    return Ok(());
}
