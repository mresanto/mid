use std::collections::HashMap;

use crate::core::{
    config::types::MidConfigFile,
    databases::{
        adapters::{
            DatabaseType, mysql::query::generate_mysql_export,
            postgres::query::generate_postgres_export,
        },
        application::query::DbValue,
    },
};

pub fn render_output_as_sql(
    items: Vec<HashMap<String, DbValue>>,
    query: String,
    config: MidConfigFile,
) -> String {
    match config.get_database_type() {
        Some(database_type) => match database_type {
            DatabaseType::Postgres => {
                let table = postgres_table_from_query(&query).unwrap_or_else(|| {
                    panic!("could not find a PostgreSQL table name after FROM in query: {query}")
                });
                generate_postgres_export(&table, items)
            }
            DatabaseType::MySQL => {
                let table = mysql_table_from_query(&query).unwrap_or_else(|| {
                    panic!("could not find a MySQL table name after FROM in query: {query}")
                });
                generate_mysql_export(&table, items)
            }
            DatabaseType::SQLite => {
                let _table = sqlite_table_from_query(&query).unwrap_or_else(|| {
                    panic!("could not find a SQLite table name after FROM in query: {query}")
                });
                panic!("sqlite adapter not implemented yet")
            }
        },
        None => {
            panic!("database type not found");
        }
    }
}

fn table_token_after_from(query: &str) -> Option<&str> {
    query
        .split_whitespace()
        .skip_while(|word| !word.eq_ignore_ascii_case("FROM"))
        .nth(1)
        .map(|table| table.trim_end_matches([';', ',']))
}

fn mysql_table_from_query(query: &str) -> Option<String> {
    table_token_after_from(query).map(|table| {
        table
            .strip_prefix('`')
            .and_then(|table| table.strip_suffix('`'))
            .map_or_else(|| table.to_string(), |table| table.replace("``", "`"))
    })
}

fn postgres_table_from_query(query: &str) -> Option<String> {
    table_token_after_from(query).map(|table| {
        table
            .strip_prefix('"')
            .and_then(|table| table.strip_suffix('"'))
            .map_or_else(|| table.to_string(), |table| table.replace("\"\"", "\""))
    })
}

fn sqlite_table_from_query(query: &str) -> Option<String> {
    table_token_after_from(query).map(|table| {
        if let Some(table) = table
            .strip_prefix('[')
            .and_then(|table| table.strip_suffix(']'))
        {
            table.replace("]]", "]")
        } else if let Some(table) = table
            .strip_prefix('"')
            .and_then(|table| table.strip_suffix('"'))
        {
            table.replace("\"\"", "\"")
        } else {
            table.to_string()
        }
    })
}

#[cfg(test)]
mod tests {
    use super::{mysql_table_from_query, postgres_table_from_query, sqlite_table_from_query};

    #[test]
    fn extracts_database_specific_table_identifiers() {
        assert_eq!(
            mysql_table_from_query("SELECT * FROM `user``data`;"),
            Some("user`data".to_string())
        );
        assert_eq!(
            postgres_table_from_query("SELECT * FROM \"user\"\"data\";"),
            Some("user\"data".to_string())
        );
        assert_eq!(
            sqlite_table_from_query("SELECT * FROM [user_data];"),
            Some("user_data".to_string())
        );
    }
}
