use thiserror::Error;

use crate::core::databases::application::query;

pub struct QueryCommandOptions {
    pub query: String,
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
    query::execute_query_on_database(query::RunQueryOnDatabaseCommandOptions {
        query: options.query,
    })
    .await?;

    return Ok(());
}
