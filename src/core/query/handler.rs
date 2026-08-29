use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use crate::{
    app::App,
    core::{
        config::{manage, types::MidConfigFile},
        databases::adapters::database_type::{DatabaseHandler, DbValue, Error as DatabaseError},
        editor::open_editor::open_editor_recover_text,
        globals, history,
        query::{
            Error, QueryOutputFormat, TableCommand, TableEvent,
            formats::json::render_output_as_json,
        },
    },
};
use sqlx::types::chrono::Utc;

pub async fn handle_query_command(
    query: String,
    options: QueryOutputFormat,
    table_command: Option<TableCommand>,
) -> Result<Option<TableEvent>, Error> {
    return Ok(execute(query, options, table_command).await?);
}

async fn execute(
    query: String,
    options: QueryOutputFormat,
    table_command: Option<TableCommand>,
) -> Result<Option<TableEvent>, Error> {
    match options {
        QueryOutputFormat::Table => {
            let command = table_command.unwrap_or_default();
            let mut current_query = query;
            let mut app = App::new(Vec::new(), command, current_query.clone());

            loop {
                let (items, duration, _) = execute_query_on_database(current_query.clone()).await?;
                app.update_app_results(items, current_query.clone(), duration);

                let event =
                    ratatui::run(|terminal| app.run(terminal)).map_err(color_eyre::Report::from)?;

                match event {
                    Some(TableEvent::EditQuery(query)) => {
                        let Some(edited_query) = open_editor_recover_text(&query)? else {
                            return Ok(None);
                        };
                        current_query = edited_query;
                    }
                    Some(TableEvent::UpdateValue(update_query)) => {
                        let Some(update_query) = open_editor_recover_text(&update_query)? else {
                            continue;
                        };
                        if !update_query.trim().is_empty() {
                            execute_query_on_database(update_query).await?;
                        }
                    }
                    Some(TableEvent::OpenSelectedRow(text)) => {
                        let Some(_) = open_editor_recover_text(&text)? else {
                            continue;
                        };
                    }
                    event => return Ok(event),
                }
            }
        }
        QueryOutputFormat::Json => {
            let (items, _, _) = execute_query_on_database(query.clone()).await?;

            render_output_as_json(items);
            Ok(None)
        }
        QueryOutputFormat::Sql => {
            let (items, _, config) = execute_query_on_database(query.clone()).await?;
            let database = config.get_database_type()?;

            let table_name = database.table_name(&query);
            println!("{}", database.export(&table_name, items));

            Ok(None)
        }
    }
}

async fn execute_query_on_database(
    query: String,
) -> Result<(Vec<HashMap<String, DbValue>>, Duration, MidConfigFile), DatabaseError> {
    let start = Instant::now();
    let file_path = globals::get_global_config_file_path();
    let config = manage::read_config(file_path)?;

    let active_database = config
        .get_active_database()
        .ok_or(DatabaseError::NoActiveRemoteConnection)?;
    let active_database_name = active_database.name.clone();

    let database = config.get_database_type()?;
    let res = database.execute(&query).await;

    let history_file_path = globals::get_global_history_file_path();
    if let Err(error) = history::add_request(
        history_file_path,
        query,
        active_database_name,
        Utc::now().to_rfc3339(),
        res.is_ok(),
    ) {
        eprintln!("Failed to save query history: {error}");
    }

    if res.is_err() {
        eprintln!("Failed to execute query: {res:?}");
        return Err(DatabaseError::FailedToExecuteQuery());
    }

    let response = res.unwrap();

    Ok((response, start.elapsed(), config))
}
