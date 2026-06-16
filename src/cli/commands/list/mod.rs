use clap::Subcommand;

use crate::core::{
    databases::application::tables,
    query::{QueryOutputFormat, handle_query_command},
};

#[derive(Subcommand)]
pub enum ListCommands {
    /// List for tables in the database
    Tables {
        #[arg()]
        table_name: String,
    },
}

pub async fn handle_list_command(
    //command: &Option<RemoteCommands>,
    output_format: QueryOutputFormat,
) -> () {
    let res = tables::list::list_database_tables();

    let query = res.unwrap_or_default();
    let res = handle_query_command(query, output_format).await;

    match res {
        Ok(_) => {}
        Err(e) => eprintln!("Failed to execute query command: {e}"),
    }
}
