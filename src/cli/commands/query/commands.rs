use crate::core::query::QueryOutputFormat;
use clap::Subcommand;

#[derive(Subcommand)]
pub enum QueryCommands {
    #[command(name = "last", visible_alias = "-", alias = "Last")]
    Last {
        #[arg(short, long)]
        output_format: Option<QueryOutputFormat>,
    },
    History {},
}
