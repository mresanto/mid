use crate::core::databases::adapters::mysql::query::list_table::list_tables_mysql;
use crate::core::databases::adapters::postgres::query::list_table::list_tables_postgres;

use crate::core::{
    config::manage,
    databases::{adapters::database_type::DatabaseType, application::tables::Error},
    globals,
};

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
