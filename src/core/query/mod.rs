mod json;
mod table;

use sqlx::Result;
use thiserror::Error;

use crate::core::{
    databases::application::query,
    query::{json::render_output_as_json, table::render_output_as_table},
};

#[derive(Debug, Clone)]
pub enum QueryOutputFormat {
    Table,
    Json,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Fail to run the query")]
    ExecuteQuery(#[from] query::Error),
}

impl clap::ValueEnum for QueryOutputFormat {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Table, Self::Json]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            Self::Table => Some(
                clap::builder::PossibleValue::new("table")
                    .help("Output the query results in a table format."),
            ),
            Self::Json => Some(
                clap::builder::PossibleValue::new("json")
                    .help("Output the query results in JSON format."),
            ),
        }
    }
}

pub async fn handle_query_command(query: String, options: QueryOutputFormat) -> Result<(), Error> {
    let res = execute(query, options).await;

    return res;
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
