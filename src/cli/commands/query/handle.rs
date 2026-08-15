use crate::cli::commands::query::commands::QueryCommands;
use crate::core::{self, query::QueryOutputFormat};

pub async fn handle_query_command(
    command: &Option<QueryCommands>,
    query: &Option<String>,
    output_format: &QueryOutputFormat,
    id: &Option<u16>,
) -> () {
    match command {
        Some(QueryCommands::Last {
            output_format,
            skip,
        }) => {
            let file_path_history = core::globals::get_global_history_file_path();
            let last_request = core::history::read_history(file_path_history);

            match last_request {
                Ok(history) => match history.requests.iter().rev().nth(*skip) {
                    Some(last) => {
                        let res = core::query::handle::handle_query_command(
                            last.query.clone(),
                            output_format.clone(),
                            None,
                        )
                        .await;

                        match res {
                            Ok(_) => {}
                            Err(e) => eprintln!("Failed to execute query command: {e}"),
                        }
                    }
                    _ if history.requests.is_empty() => println!("No history found"),
                    _ => println!(
                        "Cannot skip {skip} queries: history contains only {} entries",
                        history.requests.len()
                    ),
                },
                Err(e) => println!("No history found {}", e),
            }
        }
        _ => {
            if let Some(id) = id {
                let file_path = core::globals::get_global_history_file_path();
                let history_by_id = core::history::get_history_id(file_path, id);

                let request = history_by_id
                    .unwrap()
                    .expect("history request not found {id}");

                let res = core::query::handle::handle_query_command(
                    request.query.to_string(),
                    output_format.clone(),
                    None,
                )
                .await;

                match res {
                    Ok(_) => {}
                    Err(e) => eprintln!("Failed to execute query command by id {id}: {e}"),
                }
                return;
            }

            let Some(query) = query else {
                eprintln!("Failed to execute query command: query is required");
                return;
            };

            let res = core::query::handle::handle_query_command(
                query.to_string(),
                output_format.clone(),
                None,
            )
            .await;

            match res {
                Ok(_) => {}
                Err(e) => eprintln!("Failed to execute query command: {e}"),
            }
        }
    };
}
