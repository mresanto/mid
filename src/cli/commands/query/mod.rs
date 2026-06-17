use crate::core::{self, query::QueryOutputFormat};
use clap::Subcommand;

#[derive(Subcommand)]
pub enum QueryCommands {
    Last {},
    History {},
}

pub async fn handle_query_command(
    command: &Option<QueryCommands>,
    query: &Option<String>,
    output_format: &Option<QueryOutputFormat>,
) -> () {
    match command {
        Some(QueryCommands::Last {}) => {}
        _ => {
            let Some(query) = query else {
                eprintln!("Failed to execute query command: query is required");
                return;
            };

            let res = core::query::handle_query_command(
                query.to_string(),
                output_format
                    .as_ref()
                    .unwrap_or(&QueryOutputFormat::Table)
                    .clone(),
            )
            .await;

            match res {
                Ok(_) => {}
                Err(e) => eprintln!("Failed to execute query command: {e}"),
            }
        }
    };
}
