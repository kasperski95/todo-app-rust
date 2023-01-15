use clap::{Args, Parser, Subcommand};

pub fn get_parsed_args() -> Cli {
    Cli::parse()
}
#[derive(Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub action: Action,

    #[arg(short, long)]
    pub storage_path: std::path::PathBuf,
}

#[derive(Subcommand)]
pub enum Action {
    Add(AddArgs),
    Ls,
}

#[derive(Args)]
pub struct AddArgs {
    item: String,
}
