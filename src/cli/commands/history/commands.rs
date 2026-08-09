use clap::Subcommand;

#[derive(Subcommand)]
pub enum HistoryCommands {
    #[command(name = "last", visible_alias = "-", alias = "Last")]
    Last,
    List,
}
