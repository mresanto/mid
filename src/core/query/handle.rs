use std::{
    env,
    fs::{self, OpenOptions},
    io::{self, Write},
    path::PathBuf,
    process::Command,
};

use crate::core::{
    databases::application::query::execute_query::{
        RunQueryOnDatabaseCommandOptions, execute_query_on_database,
    },
    query::{
        Error, QueryOutputFormat, TableCommand, TableEvent,
        formats::{app::App, json::render_output_as_json, sql::render_output_as_sql},
    },
};

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
                let (items, duration, _) =
                    execute_query_on_database(RunQueryOnDatabaseCommandOptions {
                        query: current_query.clone(),
                    })
                    .await?;
                app.update_app_results(items, current_query.clone(), duration);

                let event =
                    ratatui::run(|terminal| app.run(terminal)).map_err(color_eyre::Report::from)?;

                match event {
                    Some(TableEvent::EditQuery(query)) => {
                        let Some(edited_query) = edit_query(&query)? else {
                            return Ok(None);
                        };
                        current_query = edited_query;
                    }
                    Some(TableEvent::UpdateValue(update_query)) => {
                        let Some(update_query) = edit_query(&update_query)? else {
                            continue;
                        };
                        if !update_query.trim().is_empty() {
                            execute_query_on_database(RunQueryOnDatabaseCommandOptions {
                                query: update_query,
                            })
                            .await?;
                        }
                    }
                    event => return Ok(event),
                }
            }
        }
        QueryOutputFormat::Json => {
            let (items, _, _) =
                execute_query_on_database(RunQueryOnDatabaseCommandOptions { query }).await?;

            render_output_as_json(items);
            Ok(None)
        }
        QueryOutputFormat::Sql => {
            let (items, _, config) = execute_query_on_database(RunQueryOnDatabaseCommandOptions {
                query: query.clone(),
            })
            .await?;

            println!("{}", render_output_as_sql(items, query, config));

            Ok(None)
        }
    }
}

fn edit_query(query: &str) -> color_eyre::Result<Option<String>> {
    let editor = env::var("EDITOR")
        .ok()
        .filter(|editor| !editor.trim().is_empty())
        .ok_or(Error::EditorNotConfigured())?;

    let (path, mut file) = create_query_temp_file()?;
    file.write_all(query.as_bytes())?;
    file.flush()?;
    drop(file);

    let result = (|| -> color_eyre::Result<Option<String>> {
        let mut editor_parts = editor.split_whitespace();
        let program = editor_parts.next().ok_or(Error::EditorNotConfigured())?;
        let status = Command::new(program)
            .args(editor_parts)
            .arg(&path)
            .status()
            .map_err(|_| Error::OpenEditor())?;
        if !status.success() {
            return Err(Error::OpenEditor().into());
        }

        Ok(Some(fs::read_to_string(&path)?))
    })();

    let _ = fs::remove_file(&path);
    Ok(result?)
}

fn create_query_temp_file() -> std::result::Result<(PathBuf, fs::File), Error> {
    for attempt in 0..100 {
        let path = env::temp_dir().join(format!("mid-query-{}-{attempt}.sql", std::process::id()));
        match OpenOptions::new().write(true).create_new(true).open(&path) {
            Ok(file) => return Ok((path, file)),
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
            Err(error) => return Err(Error::CreateTempFile(error)),
        }
    }

    Err(Error::CreateTempFile(io::Error::new(
        io::ErrorKind::AlreadyExists,
        "could not create a unique query temporary file after 100 attempts",
    )))
}
