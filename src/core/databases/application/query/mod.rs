use std::collections::HashMap;

use thiserror::Error;

use crate::core::{
    config::status,
    databases::adapters::{DatabaseType, postgres::query::execute_postgres_query},
};

#[derive(Debug, Clone)]
pub enum DbValue {
    Null,
    Text(String),
    TextArray(Vec<String>),
    Numeric(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
}

pub struct RunQueryOnDatabaseCommandOptions {
    pub query: String,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to read config file: {0}")]
    CurrentConfigError(#[from] status::handler::Error),

    #[error(
        "Cant find an active remote connection. Please set an active connection before running a query."
    )]
    NoActiveRemoteConnection,

    #[error("Failed to execute query")]
    FailedToExecuteQuery(),
}

pub async fn execute_query_on_database(
    options: RunQueryOnDatabaseCommandOptions,
) -> Result<Vec<HashMap<String, DbValue>>, Error> {
    let config = status::get_current_config()?;

    let active_database = config.config.get_active_database();

    if active_database.is_none() {
        return Err(Error::NoActiveRemoteConnection);
    }

    let active_database = active_database.unwrap();

    let response = match config.config.get_database_type() {
        Some(database_type) => match database_type {
            DatabaseType::Postgres => {
                let res = execute_postgres_query(active_database, options.query).await;

                if res.is_err() {
                    eprintln!("Failed to execute query on PostgreSQL: {res:?}");
                    return Err(Error::FailedToExecuteQuery());
                }

                res.unwrap()
            }
            DatabaseType::MySQL => {
                panic!("mysql adapter not implemented yet");
            }
            DatabaseType::SQLite => {
                panic!("sqlite adapter not implemented yet");
            }
        },
        None => {
            return Err(Error::NoActiveRemoteConnection);
        }
    };

    return Ok(response);
}
