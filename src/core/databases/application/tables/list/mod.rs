use thiserror::Error;

use crate::core::{
    config::manage,
    databases::adapters::{DatabaseType, postgres::query::list_tables_query},
    globals,
};

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to read config file: {0}")]
    CurrentConfigError(#[from] manage::Error),

    #[error("Failed to execute query")]
    FailedToExecuteQuery(),
}

pub fn list_database_tables() -> Result<String, Error> {
    let file_path = globals::get_global_config_file_path();
    let config = manage::read_config(file_path)?;

    match config.get_database_type() {
        Some(database_type) => match database_type {
            DatabaseType::Postgres => {
                return Ok(list_tables_query());
            }
            DatabaseType::MySQL => {
                panic!("mysql adapter not implemented yet");
            }
            DatabaseType::SQLite => {
                panic!("sqlite adapter not implemented yet");
            }
        },
        None => {
            return Err(Error::FailedToExecuteQuery());
        }
    };
}
