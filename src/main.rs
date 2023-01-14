extern crate serde;
extern crate serde_json;

mod controllers;
mod models;
mod repositories;

use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use controllers::TodoController;
use repositories::JSONTodoRepository;

#[derive(Args)]
struct AddArgs {
    item: String,
    #[arg(short, long)]
    storage_path: std::path::PathBuf,
}

#[derive(Subcommand)]
enum Action {
    Add(AddArgs),
    Ls,
}
#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    action: Action,

    #[arg(short, long)]
    storage_path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    match args.action {
        Action::Add(_) => {
            println!("TODO: ADD");
        }
        Action::Ls => {
            let json_todo_repository = JSONTodoRepository {
                file_path: &args.storage_path,
            };
            let todo_controller = TodoController {
                todo_repository: &json_todo_repository,
            };
            let todos = todo_controller.show_all();
            if todos.is_empty() {
                println!("The list is empty")
            } else {
                for todo in todos {
                    println!("{}", todo.name)
                }
            }
        }
    }

    Ok(())
}
