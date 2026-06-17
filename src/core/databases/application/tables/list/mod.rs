use thiserror::Error;

use crate::core::{
    config::manage,
    databases::adapters::{
        DatabaseType, mysql::query::list_tables_mysql, postgres::query::list_tables_postgres,
    },
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
                return Ok(list_tables_postgres());
            }
            DatabaseType::MySQL => {
                return Ok(list_tables_mysql());
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
