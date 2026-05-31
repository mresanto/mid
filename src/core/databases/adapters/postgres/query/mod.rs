use std::collections::HashMap;

use sqlx::{AssertSqlSafe, Column, Row, TypeInfo, ValueRef, postgres::PgPoolOptions};
use thiserror::Error;

use crate::core::{config::types::DatabaseConfig, databases::application::query::DbValue};

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

    let rows = sqlx::query(safe_query).fetch_all(&pool).await?;

    pool.close().await;

    let mut parsed_rows = Vec::new();

    for row in &rows {
        let mut row_map = HashMap::new();

        for column in row.columns() {
            let column_name = column.name();

            let db_value = match row.try_get_raw(column_name) {
                Ok(value_ref) if !value_ref.is_null() => {
                    let type_name = column.type_info().name();
                    match type_name {
                        "VARCHAR" | "TEXT" | "BPCHAR" | "NAME" => row
                            .try_get::<String, _>(column_name)
                            .map(DbValue::Text)
                            .unwrap_or(DbValue::Null),
                        "INT2" | "INT4" | "INTEGER" => row
                            .try_get::<i32, _>(column_name)
                            .map(|n| DbValue::Integer(n as i64))
                            .unwrap_or(DbValue::Null),
                        "INT8" | "BIGINT" => row
                            .try_get::<i64, _>(column_name)
                            .map(DbValue::Integer)
                            .unwrap_or(DbValue::Null),
                        "BOOL" | "BOOLEAN" => row
                            .try_get::<bool, _>(column_name)
                            .map(DbValue::Boolean)
                            .unwrap_or(DbValue::Null),
                        "FLOAT4" | "REAL" => row
                            .try_get::<f32, _>(column_name)
                            .map(|n| DbValue::Float(n as f64))
                            .unwrap_or(DbValue::Null),
                        "FLOAT8" | "DOUBLE PRECISION" => row
                            .try_get::<f64, _>(column_name)
                            .map(DbValue::Float)
                            .unwrap_or(DbValue::Null),
                        // Safe fallback for complex types (Timestamps, UUIDs, JSON columns)
                        _ => row
                            .try_get::<String, _>(column_name)
                            .map(DbValue::Text)
                            .unwrap_or_else(|_| {
                                DbValue::Text(format!("<unsupported: {}>", type_name))
                            }),
                    }
                }
                _ => DbValue::Null,
            };

            row_map.insert(column_name.to_string(), db_value);
        }

        parsed_rows.push(row_map);
    }

    return Ok(());
}
