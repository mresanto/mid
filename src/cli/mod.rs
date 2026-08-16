use clap::Parser;

use crate::cli::commands::main_commands::Commands;

pub mod commands;

#[derive(Parser)]
#[command(
    version,
    about,
    long_about = None,
    arg_required_else_help = true
)]
pub struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub debug: u8,

    #[command(subcommand)]
    pub command: Commands,
}
