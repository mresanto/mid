mod json;
mod table;

use sqlx::Result;
use thiserror::Error;

use crate::core::{
    databases::application::query,
    query::{json::render_output_as_json, table::render_output_as_table},
};

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum QueryOutputFormat {
    Table,
    Json,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Fail to run the query")]
    ExecuteQuery(#[from] query::Error),
}

pub async fn handle_query_command(query: String, options: QueryOutputFormat) -> Result<(), Error> {
    execute(query.clone(), options).await?;

    return Ok(());
}

async fn execute(query: String, options: QueryOutputFormat) -> Result<(), Error> {
    let items =
        query::execute_query_on_database(query::RunQueryOnDatabaseCommandOptions { query: query })
            .await?;

    match options {
        QueryOutputFormat::Table => {
            render_output_as_table(items).unwrap();
        }
        QueryOutputFormat::Json => {
            render_output_as_json(items);
        }
    }

    return Ok(());
}
