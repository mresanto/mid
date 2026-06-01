use thiserror::Error;

use crate::{
    cli::commands::query::{json::render_output_as_json, table::render_outout_as_table},
    core::databases::application::query::{self},
};

mod json;
mod table;

#[derive(Debug, Clone)]
pub enum QueryOutputFormat {
    Table,
    Json,
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

pub struct QueryCommandOptions {
    pub query: String,
    pub output_format: QueryOutputFormat,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Fail to run the query")]
    ExecuteQuery(#[from] query::Error),
}

pub async fn handle_query_command(options: QueryCommandOptions) -> () {
    let res = execute(options).await;

    match res {
        Ok(_) => {}
        Err(e) => eprintln!("Failed to execute query command: {e}"),
    }
}

async fn execute(options: QueryCommandOptions) -> Result<(), Error> {
    let items = query::execute_query_on_database(query::RunQueryOnDatabaseCommandOptions {
        query: options.query,
    })
    .await?;

    match options.output_format {
        QueryOutputFormat::Table => {
            render_outout_as_table(items);
        }
        QueryOutputFormat::Json => {
            render_output_as_json(items);
        }
    }

    return Ok(());
}
