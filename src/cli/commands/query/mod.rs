use crate::core::{self, query::QueryOutputFormat};

pub struct QueryCommandOptions {
    pub query: String,
    pub output_format: QueryOutputFormat,
}

pub async fn handle_query_command(options: QueryCommandOptions) -> () {
    let res = core::query::handle_query_command(options.query, options.output_format).await;

    match res {
        Ok(_) => {}
        Err(e) => eprintln!("Failed to execute query command: {e}"),
    }
}
