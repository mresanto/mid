use clap::Subcommand;

use crate::{
    cli::commands::query::{QueryCommandOptions, QueryOutputFormat, handle_query_command},
    core::databases::application::tables,
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
    let options = QueryCommandOptions {
        query,
        output_format: output_format,
    };

    handle_query_command(options).await
}
