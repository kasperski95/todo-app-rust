extern crate serde;
extern crate serde_json;

mod cli;
mod controllers;
mod models;
mod repositories;

use anyhow::Result;
use cli::{get_parsed_args, Action};
use controllers::TodoController;
use repositories::JSONTodoRepository;
use std::{io::stdout, path::PathBuf};

fn main() -> Result<()> {
    let args = get_parsed_args();
    let mut todo_controller = create_todo_controller(args.storage_path);
    run_action(args.action, &mut todo_controller);
    Ok(())
}

fn create_todo_controller(storage_path: PathBuf) -> TodoController {
    let json_todo_repository = Box::new(JSONTodoRepository {
        file_path: storage_path,
    });
    TodoController {
        todo_repository: json_todo_repository,
        writer: Box::new(stdout()),
    }
}

fn run_action(action: Action, todo_controller: &mut TodoController) {
    match action {
        Action::Add(_) => {
            println!("TODO: ADD");
        }
        Action::Ls => {
            todo_controller.show_all();
        }
    }
}
