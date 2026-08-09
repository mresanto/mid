use clap::Subcommand;

use crate::core::query::QueryOutputFormat;

#[derive(Subcommand)]
pub enum ListCommands {
    /// List for tables in the database
    Tables {
        #[arg(short, long)]
        table_name: Option<String>,
        #[arg(short, long, value_enum, default_value = "table")]
        output_format: QueryOutputFormat,
    },
}
