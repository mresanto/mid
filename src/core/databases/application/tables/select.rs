use crate::core::databases::adapters::mysql::query::select_table::select_table_mysql;
use crate::core::databases::adapters::postgres::query::select_table::select_table_postgres;
use crate::core::{
    config::manage,
    databases::{adapters::database_type::DatabaseType, application::tables::Error},
    globals,
};

pub fn select_database_table(table_name: &str) -> Result<String, Error> {
    let file_path = globals::get_global_config_file_path();
    let config = manage::read_config(file_path)?;

    match config.get_database_type() {
        Some(DatabaseType::Postgres) => Ok(select_table_postgres(table_name)),
        Some(DatabaseType::MySQL) => Ok(select_table_mysql(table_name)),
        Some(DatabaseType::SQLite) | None => Err(Error::UnsupportedDatabase),
    }
}
