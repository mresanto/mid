use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use crate::core::{
    config::{manage, types::MidConfigFile},
    databases::adapters::{
        DatabaseType, mysql::query::execute_mysql_query, postgres::query::execute_postgres_query,
    },
    globals::{self, get_global_history_file_path},
    history::{HistoryRequest, add_request, last_history_or_default},
};
use sqlx::types::chrono::Utc;
use thiserror::Error;

#[derive(Debug, Clone)]
pub enum DbValue {
    Null,
    Text(String),
    TextArray(Vec<String>),
    Json(serde_json::Value),
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
    CurrentConfigError(#[from] manage::Error),

    #[error(
        "Cant find an active remote connection. Please set an active connection before running a query."
    )]
    NoActiveRemoteConnection,

    #[error("Failed to execute query")]
    FailedToExecuteQuery(),

    #[error("Database type not found")]
    DatabaseTypeNotFound,
}

pub async fn execute_query_on_database(
    options: RunQueryOnDatabaseCommandOptions,
) -> Result<(Vec<HashMap<String, DbValue>>, Duration, MidConfigFile), Error> {
    let start = Instant::now();
    let file_path = globals::get_global_config_file_path();
    let config = manage::read_config(file_path)?;

    let active_database = config.get_active_database();

    if active_database.is_none() {
        return Err(Error::NoActiveRemoteConnection);
    }

    let active_database = active_database.unwrap();

    let response = match config.get_database_type() {
        Some(database_type) => match database_type {
            DatabaseType::Postgres => {
                let res = execute_postgres_query(active_database, options.query.clone()).await;

                if res.is_err() {
                    eprintln!("Failed to execute query on PostgreSQL: {res:?}");
                    return Err(Error::FailedToExecuteQuery());
                }

                res.unwrap()
            }
            DatabaseType::MySQL => {
                let res = execute_mysql_query(active_database, options.query.clone()).await;

                if res.is_err() {
                    eprintln!("Failed to execute query on MySQL: {res:?}");
                    return Err(Error::FailedToExecuteQuery());
                }

                res.unwrap()
            }
            DatabaseType::SQLite => {
                panic!("sqlite adapter not implemented yet");
            }
        },
        _ => {
            return Err(Error::DatabaseTypeNotFound);
        }
    };

    let file_path = get_global_history_file_path();
    let last = last_history_or_default(file_path.clone());
    let history_response = add_request(
        file_path,
        HistoryRequest {
            id: last.unwrap_or_default().id + 1,
            query: options.query,
            database: active_database.name.clone(),
            created_at: Utc::now().to_string(),
        },
    );

    if history_response.is_err() {
        eprintln!("Failed to save history to database",);
    }

    return Ok((response, start.elapsed(), config));
}
