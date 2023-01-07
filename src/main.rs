use anyhow::{Context, Result};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    name: String,
    storage_path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.storage_path)
        .with_context(|| format!("could not read file `...`"))?;

    println!("Hello, {}!. File content: {}", args.name, content);
    Ok(())
}
