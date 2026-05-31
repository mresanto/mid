use thiserror::Error;

use crate::core::{config::status, databases::adapters::DatabaseType};

pub struct RunQueryOnDatabaseCommandOptions {
    pub query: String,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to read config file: {0}")]
    CurrentConfigError(#[from] status::handler::Error),
}

pub fn execute_query_on_database(options: RunQueryOnDatabaseCommandOptions) -> Result<(), Error> {
    let config = status::get_current_config()?;

    match config.config.get_database_type() {
        Some(database_type) => match database_type {
            DatabaseType::Postgres => {
                panic!("Postgtres adapter not implemented yet");
            }
            DatabaseType::MySQL => {
                panic!("mysql adapter not implemented yet");
            }
            DatabaseType::SQLite => {
                panic!("sqlite adapter not implemented yet");
            }
        },
        None => {
            println!("Unsupported or unknown database type.");
        }
    };

    return Ok(());
}
