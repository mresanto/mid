use clap::Subcommand;
use thiserror::Error;

use crate::core::databases::application::tables;

#[derive(Subcommand)]
pub enum ListCommands {
    /// List for tables in the database
    Tables {
        #[arg()]
        table_name: String,
    },
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to list tables: {0}")]
    ListTablesError(#[from] tables::list::Error),
}

pub fn handle_list_command(command: &Option<ListCommands>) -> () {
    let res = execute(command);

    match res {
        Ok(_) => {}
        Err(e) => eprintln!("Failed to execute list command: {e}"),
    }
}

fn execute(command: &Option<ListCommands>) -> Result<(), Error> {
    match command {
        Some(ListCommands::Tables { table_name }) => {
            tables::list::list_database_tables()?;
            println!(
                "Listing tables in the database, filter by name: {}",
                table_name
            );
        }
        None => {}
    };

    return Ok(());
}
