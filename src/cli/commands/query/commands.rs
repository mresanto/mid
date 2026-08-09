use crate::core::query::QueryOutputFormat;
use clap::Subcommand;

#[derive(Subcommand)]
pub enum QueryCommands {
    #[command(name = "last", visible_alias = "-", alias = "Last")]
    Last {
        #[arg(long, value_enum, default_value = "table")]
        output_format: QueryOutputFormat,
    },
    History {},
}
