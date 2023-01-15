use serde::{Deserialize, Serialize};

use std::{fs::read_to_string, path::PathBuf};
extern crate serde_json;

use crate::{controllers::TodoRepository, models::Todo};

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
        let file_content = read_to_string(&self.file_path).unwrap();
        let content: JSONContent = serde_json::from_str(file_content.as_str()).unwrap();
        return content.todos;
    }

    fn save(&self) -> () {}
}
