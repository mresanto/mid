use thiserror::Error;

use crate::core::databases::application::query::{self, DbValue};

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
            // Get the column names from the first row, or use an empty vector if there are no rows
        }
        QueryOutputFormat::Json => {
            // Convert your Vec<HashMap<String, DbValue>> to serde_json::Value
            let json_elements: Vec<serde_json::Value> = items
                .iter()
                .map(|row| {
                    let mut map = serde_json::Map::new();
                    for (k, v) in row {
                        let json_val = match v {
                            DbValue::Null => serde_json::Value::Null,
                            DbValue::Text(s) => serde_json::Value::String(s.clone()),
                            DbValue::Integer(n) => serde_json::Value::Number((*n).into()),
                            DbValue::Float(f) => serde_json::Number::from_f64(*f)
                                .map(serde_json::Value::Number)
                                .unwrap_or(serde_json::Value::Null),
                            DbValue::Boolean(b) => serde_json::Value::Bool(*b),
                        };
                        map.insert(k.clone(), json_val);
                    }
                    serde_json::Value::Object(map)
                })
                .collect();

            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::Value::Array(json_elements)).unwrap()
            );
        }
    }

    return Ok(());
}
