use std::collections::HashMap;

use crate::core::databases::adapters::database_type::{
    DatabaseHandler, DatabaseType, DbValue, Error,
};

impl DatabaseHandler for DatabaseType {
    async fn execute(&self, query: &str) -> Result<Vec<HashMap<String, DbValue>>, Error> {
        match self {
            DatabaseType::Postgres(handler) => handler.execute(query).await,
            DatabaseType::MySQL(handler) => handler.execute(query).await,
            DatabaseType::SQLite() => todo!(),
        }
    }

    fn export(&self, table_name: &str, items: Vec<HashMap<String, DbValue>>) -> String {
        match self {
            DatabaseType::Postgres(handler) => handler.export(table_name, items),
            DatabaseType::MySQL(handler) => handler.export(table_name, items),
            DatabaseType::SQLite() => todo!(),
        }
    }

    fn list_tables(&self) -> String {
        match self {
            DatabaseType::Postgres(handler) => handler.list_tables(),
            DatabaseType::MySQL(handler) => handler.list_tables(),
            DatabaseType::SQLite() => todo!(),
        }
    }

    fn select(&self, table_name: &str) -> String {
        match self {
            DatabaseType::Postgres(handler) => handler.select(table_name),
            DatabaseType::MySQL(handler) => handler.select(table_name),
            DatabaseType::SQLite() => todo!(),
        }
    }

    fn update(
        &self,
        table_name: &str,
        id_column: &str,
        id: &DbValue,
        column: &str,
        value: &DbValue,
    ) -> String {
        match self {
            DatabaseType::Postgres(handler) => {
                handler.update(table_name, id_column, id, column, value)
            }
            DatabaseType::MySQL(handler) => {
                handler.update(table_name, id_column, id, column, value)
            }
            DatabaseType::SQLite() => todo!(),
        }
    }

    fn table_name(&self, table_name: &str) -> String {
        match self {
            DatabaseType::Postgres(handler) => handler.table_name(table_name),
            DatabaseType::MySQL(handler) => handler.table_name(table_name),
            DatabaseType::SQLite() => todo!(),
        }
    }
}
