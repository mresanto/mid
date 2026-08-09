pub mod execute_query;
pub mod export_sql;
pub mod list_table;
pub mod select_table;
pub mod update_table;

use thiserror::Error;
#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to execute query: {0}")]
    SqlError(#[from] sqlx::Error),
}
