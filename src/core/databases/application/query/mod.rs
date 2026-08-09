pub mod execute_query;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to read config file: {0}")]
    CurrentConfigError(#[from] crate::core::config::manage::Error),

    #[error(
        "Cant find an active remote connection. Please set an active connection before running a query."
    )]
    NoActiveRemoteConnection,

    #[error("Failed to execute query")]
    FailedToExecuteQuery(),

    #[error("Database type not found")]
    DatabaseTypeNotFound,
}
