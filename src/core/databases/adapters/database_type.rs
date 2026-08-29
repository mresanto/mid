use std::collections::HashMap;

use sqlx::types::chrono;
use thiserror::Error;

use crate::core::databases::adapters::{
    mysql::mysql_handler::MySqlHandler, postgres::postgres_handler::PostgresHandler,
};

/// This module contains the database connection and related functionality.
pub enum DatabaseType {
    /// The Postgres database type.
    Postgres(PostgresHandler),

    /// The MySQL database type.
    MySQL(MySqlHandler),

    /// The SQLite database type.
    SQLite(),
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to execute query: {0}")]
    SqlError(#[from] sqlx::Error),

    #[error("Failed to read config file: {0}")]
    CurrentConfigError(#[from] crate::core::config::manage::Error),

    #[error(
        "Cant find an active remote connection. Please set an active connection before running a query"
    )]
    NoActiveRemoteConnection,

    #[error("Failed to execute query")]
    FailedToExecuteQuery(),

    #[error("Database type not found")]
    DatabaseTypeNotFound,
}

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
    DateTime(chrono::DateTime<chrono::Utc>),
}

pub trait DatabaseHandler {
    async fn execute(&self, query: &str) -> Result<Vec<HashMap<String, DbValue>>, Error>;
    fn export(&self, table_name: &str, items: Vec<HashMap<String, DbValue>>) -> String;
    fn list_tables(&self) -> String;
    fn select(&self, table_name: &str) -> String;
    fn update(
        &self,
        table_name: &str,
        id_column: &str,
        id: &DbValue,
        values: &[(&str, &DbValue)],
    ) -> String;
    fn table_name(&self, table_name: &str) -> String;
}
