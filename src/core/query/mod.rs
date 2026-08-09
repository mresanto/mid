pub mod formats;
pub mod handle;

#[derive(Debug, Clone)]
pub enum TableEvent {
    EditQuery(String),
    UpdateValue(String),
    SelectTable(String),
}
use std::io;

use thiserror::Error;

use crate::core::databases::application::query;

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum QueryOutputFormat {
    Table,
    Json,
    Sql,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, clap::ValueEnum)]
pub enum TableCommand {
    #[default]
    ShowValue,
    ShowTables,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Fail to run the query :{0}")]
    ExecuteQuery(#[from] query::Error),

    #[error("Fail to render query results")]
    RenderTable(#[from] color_eyre::Report),

    #[error("Fail to open Editor")]
    OpenEditor(),

    #[error("Fail to create temporary query file")]
    CreateTempFile(#[from] io::Error),
}
