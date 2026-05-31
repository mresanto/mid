/// This module contains the database connection and related functionality.
pub enum DatabaseType {
    /// The Postgres database type.
    Postgres,

    /// The MySQL database type.
    MySQL,

    /// The SQLite database type.
    SQLite,
}
