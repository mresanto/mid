use std::collections::HashMap;

use crate::core::{
    config::types::DatabaseConfig,
    databases::adapters::database_type::{DatabaseHandler, DbValue, Error},
};

use super::methods::{
    execute_query::execute_postgres_query, export_sql::generate_postgres_export,
    list_table::list_tables_postgres, select_table::select_table_postgres,
    update_table::update_table_postgres,
};

pub struct PostgresHandler {
    config: DatabaseConfig,
}

impl PostgresHandler {
    pub fn new(config: DatabaseConfig) -> Self {
        Self { config }
    }
}

impl DatabaseHandler for PostgresHandler {
    async fn execute(&self, query: &str) -> Result<Vec<HashMap<String, DbValue>>, Error> {
        execute_postgres_query(&self.config, query.to_owned()).await
    }

    fn export(&self, table_name: &str, items: Vec<HashMap<String, DbValue>>) -> String {
        generate_postgres_export(table_name, items)
    }

    fn list_tables(&self) -> String {
        list_tables_postgres()
    }

    fn select(&self, table_name: &str) -> String {
        select_table_postgres(table_name)
    }

    fn update(
        &self,
        table_name: &str,
        id_column: &str,
        id: &DbValue,
        values: &[(&str, &DbValue)],
    ) -> String {
        update_table_postgres(table_name, id_column, id, values)
    }

    fn table_name(&self, table_name: &str) -> String {
        format!("\"{}\"", table_name.replace('"', "\"\""))
    }
}
