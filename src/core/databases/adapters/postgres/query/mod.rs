use sqlx::{AssertSqlSafe, postgres::PgPoolOptions};
use thiserror::Error;

use crate::core::config::types::DatabaseConfig;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to execute query: {0}")]
    SqlError(#[from] sqlx::Error),
}

/// Use this method to run an arbitrary query on the active database connection.
pub async fn execute_postgres_query(config: &DatabaseConfig, query: String) -> Result<(), Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.connection_string)
        .await?;

    // We cant assert the query is safe, but this will only affect the user database, so theres
    // no point to try to validate the query, since the user is the one writing it, and if they
    // write a malicious query, its their own fault, so we will just execute it as is.
    let safe_query = AssertSqlSafe(query);

    let row_stream = sqlx::query(safe_query).fetch_all(&pool).await?;

    for (i, row) in row_stream.iter().enumerate() {
        println!("{:?}", row);
    }

    pool.close().await;

    return Ok(());
}
