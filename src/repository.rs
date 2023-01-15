use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
extern crate serde_json;

use crate::{controller::TodoRepository, model::Todo};

#[derive(Debug, Deserialize, Serialize)]
struct JSONContent {
    todos: Vec<Todo>,
}

pub struct JSONTodoRepository {
    pub file_path: PathBuf,
}
impl TodoRepository for JSONTodoRepository {
    fn find_all(&self) -> Vec<Todo> {
        if !self.file_path.exists() {
            return Vec::new();
        }
        let file_content = fs::read_to_string(&self.file_path).unwrap();
        let content: JSONContent = serde_json::from_str(file_content.as_str()).unwrap();
        return content.todos;
    }

    fn save(&self, todo: Todo) -> () {
        let mut todos = self.find_all();
        todos.push(todo);
        let file_content = serde_json::to_string(&JSONContent { todos: todos }).unwrap();
        fs::write(&self.file_path, file_content).unwrap();
    }

    fn remove(&self, todo_name: String) -> () {
        let mut todos = self.find_all();
        todos = todos
            .into_iter()
            .filter(|todo| !todo.name.starts_with(&todo_name))
            .collect();
        let file_content = serde_json::to_string(&JSONContent { todos }).unwrap();
        fs::write(&self.file_path, file_content).unwrap();
    }
}
