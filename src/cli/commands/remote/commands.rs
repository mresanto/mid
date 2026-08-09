use clap::Subcommand;

#[derive(Subcommand)]
pub enum RemoteCommands {
    List {},
    Add {
        #[arg()]
        connection_string: String,

        #[arg(short, long)]
        name: Option<String>,
        // #[arg(short, long)]
        // global: bool,
    },
    Remove {
        #[arg()]
        name: String,
    },
    Switch {
        #[arg()]
        name: String,
    },
}
