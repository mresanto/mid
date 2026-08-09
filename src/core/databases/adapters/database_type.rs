/// This module contains the database connection and related functionality.
pub enum DatabaseType {
    /// The Postgres database type.
    Postgres,

    /// The MySQL database type.
    MySQL,

    /// The SQLite database type.
    SQLite,
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
}
