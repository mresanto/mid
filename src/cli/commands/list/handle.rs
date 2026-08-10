// use crate::cli::commands::list::commands::ListCommands;
use crate::core::query::{QueryOutputFormat, TableEvent};
use crate::core::{
    databases::application::tables::{list::list_database_tables, select::select_database_table},
    query::{TableCommand, handle::handle_query_command},
};

pub async fn handle_list_command(output_format: &QueryOutputFormat, table_name: &Option<String>) {
    if let Some(table_name) = table_name {
        handle_selected_table(table_name, output_format).await;
        return;
    }
    let query = match list_database_tables() {
        Ok(query) => query,
        Err(e) => {
            eprintln!("[List] Failed to build table list query: {e}");
            return;
        }
    };

    match handle_query_command(query, output_format.clone(), Some(TableCommand::ShowTables)).await {
        Ok(Some(TableEvent::SelectTable(table_name))) => {
            handle_selected_table(&table_name, output_format).await;
            return;
        }
        Ok(_) => {}
        Err(e) => eprintln!("[List] Failed to list tables: {e}"),
    }
}

async fn handle_selected_table(table_name: &str, output_format: &QueryOutputFormat) {
    let query = match select_database_table(&table_name) {
        Ok(query) => query,
        Err(e) => {
            eprintln!("[List] Failed to build query for selected table: {e}");
            return;
        }
    };

    if let Err(e) =
        handle_query_command(query, output_format.clone(), Some(TableCommand::ShowValue)).await
    {
        eprintln!("[List] Failed to query selected table: {e}");
    }
}
