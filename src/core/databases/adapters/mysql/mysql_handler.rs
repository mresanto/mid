use std::collections::HashMap;

use crate::core::{
    config::types::DatabaseConfig,
    databases::adapters::database_type::{DatabaseHandler, DbValue, Error},
};

use super::methods::{
    execute_query::execute_mysql_query, export_sql::generate_mysql_export,
    list_table::list_tables_mysql, select_table::select_table_mysql,
    update_table::update_table_mysql,
};

pub struct MySqlHandler {
    config: DatabaseConfig,
}

impl MySqlHandler {
    pub fn new(config: DatabaseConfig) -> Self {
        Self { config }
    }
}

impl DatabaseHandler for MySqlHandler {
    async fn execute(&self, query: &str) -> Result<Vec<HashMap<String, DbValue>>, Error> {
        execute_mysql_query(&self.config, query.to_owned()).await
    }

    fn export(&self, table_name: &str, items: Vec<HashMap<String, DbValue>>) -> String {
        generate_mysql_export(table_name, items)
    }

    fn list_tables(&self) -> String {
        list_tables_mysql()
    }

    fn select(&self, table_name: &str) -> String {
        select_table_mysql(table_name)
    }

    fn update(
        &self,
        table_name: &str,
        id_column: &str,
        id: &DbValue,
        values: &[(&str, &DbValue)],
    ) -> String {
        update_table_mysql(table_name, id_column, id, values)
    }

    fn table_name(&self, table_name: &str) -> String {
        format!("`{}`", table_name.replace('`', "``"))
    }
}
