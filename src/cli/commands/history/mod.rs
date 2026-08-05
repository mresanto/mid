use clap::Subcommand;

use crate::core::{self};

#[derive(Subcommand)]
pub enum HistoryCommands {
    #[command(name = "last", visible_alias = "-", alias = "Last")]
    Last,
    List,
}

pub async fn handle_history_command(command: &Option<HistoryCommands>) -> () {
    match command {
        Some(HistoryCommands::List {}) => {
            let file_path = core::globals::get_global_history_file_path();
            let res = core::history::read_history(file_path);

            match res {
                Ok(history) => {
                    for request in history.requests {
                        println!("id: {}", request.id);
                        println!("query: {}", request.query);
                        println!("created_at: {}", request.created_at);
                        println!("database: {}", request.database);
                        println!("");
                    }
                }
                Err(e) => eprintln!("No history found: {e}"),
            }
        }
        Some(HistoryCommands::Last {}) => {
            let file_path = core::globals::get_global_history_file_path();
            let res = core::history::read_history(file_path);

            match res {
                Ok(history) => match history.requests.last() {
                    Some(request) => {
                        println!("id: {}", request.id);
                        println!("query: {}", request.query);
                        println!("created_at: {}", request.created_at);
                        println!("database: {}", request.database);
                        println!("");
                    }
                    _ => println!("No history found"),
                },
                Err(e) => eprintln!("No history found: {e}"),
            }
        }
        _ => {}
    };
}
