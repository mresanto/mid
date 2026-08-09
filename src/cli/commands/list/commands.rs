use clap::Subcommand;

#[derive(Subcommand)]
pub enum ListCommands {
    /// List for tables in the database
    Tables {
        #[arg()]
        table_name: String,
    },
}
