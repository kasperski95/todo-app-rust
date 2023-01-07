use clap::Parser;

#[derive(Parser)]
struct Cli {
    name: String,
    storage_path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.storage_path).unwrap();
    println!("Hello, {}!. File content: {}", args.name, content);
}
