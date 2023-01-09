use anyhow::Result;
use clap::{Args, Parser, Subcommand};

#[derive(Args)]
struct AddArgs {
    item: String,
    #[arg(short, long)]
    storage_path: std::path::PathBuf,
}

#[derive(Args)]
struct LsArgs {
    #[arg(short, long)]
    storage_path: std::path::PathBuf,
}

#[derive(Subcommand)]
enum Action {
    Add(AddArgs),
    Ls(LsArgs),
}
#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    action: Action,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    match args.action {
        Action::Add(_) => {
            println!("ADD")
        }
        Action::Ls(_) => {
            println!("LS")
        }
    }

    Ok(())
}
