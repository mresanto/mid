use crate::cli::commands::list::commands::ListCommands;
use crate::core::{
    databases::application::tables::{list::list_database_tables, select::select_database_table},
    query::{TableCommand, TableEvent, handle::handle_query_command},
};

pub async fn handle_list_command(command: &Option<ListCommands>) {
    match command {
        Some(ListCommands::Tables {
            table_name: Some(table_name),
            output_format,
        }) => {
            let query = match select_database_table(table_name) {
                Ok(query) => query,
                Err(e) => {
                    eprintln!("[List] Failed to build query for selected table: {e}");
                    return;
                }
            };

            if let Err(e) =
                handle_query_command(query, output_format.clone(), Some(TableCommand::ShowValue))
                    .await
            {
                eprintln!("[List] Failed to query selected table: {e}");
            }
        }
        Some(ListCommands::Tables {
            table_name: None,
            output_format,
        }) => {
            let query = match list_database_tables() {
                Ok(query) => query,
                Err(e) => {
                    eprintln!("[List] Failed to build table list query: {e}");
                    return;
                }
            };

            match handle_query_command(query, output_format.clone(), Some(TableCommand::ShowTables))
                .await
            {
                Ok(Some(TableEvent::SelectTable(table_name))) => {
                    let query = match select_database_table(&table_name) {
                        Ok(query) => query,
                        Err(e) => {
                            eprintln!("[List] Failed to build query for selected table: {e}");
                            return;
                        }
                    };

                    if let Err(e) = handle_query_command(
                        query,
                        output_format.clone(),
                        Some(TableCommand::ShowValue),
                    )
                    .await
                    {
                        eprintln!("[List] Failed to query selected table: {e}");
                    }
                }
                Ok(_) => {}
                Err(e) => eprintln!("[List] Failed to list tables: {e}"),
            }
        }
        None => {}
    }
}
