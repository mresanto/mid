use thiserror::Error;

use crate::core::config::manage;

pub mod list;
pub mod select;
pub mod update;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to read config file: {0}")]
    CurrentConfigError(#[from] manage::Error),

    #[error("Failed to execute query")]
    FailedToExecuteQuery(),

    #[error("Failed to select table: unsupported database type")]
    UnsupportedDatabase,
}
