extern crate serde;
extern crate serde_json;

mod lib;

use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use lib::{JSONTodoRepository, TodoController};
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Deserialize, Serialize)]
struct Storage {
    items: Vec<String>,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    match args.action {
        Action::Add(_) => {
            println!("ADD");
            let storage = Storage {
                items: vec!["foo".to_string(), "bar".to_string(), "baz".to_string()],
            };
            println!("{}", serde_json::to_string(&storage).unwrap());
        }
        Action::Ls(_) => {
            println!("LS");
            let json_todo_repository = JSONTodoRepository {};
            let todo_controller = TodoController {
                todo_repository: &json_todo_repository,
            };
            let todos = todo_controller.show_all();
            for todo in todos {
                println!("{}", todo.name)
            }
        }
    }

    Ok(())
}
