use crate::model::Todo;
use std::io::Write;
pub trait TodoRepository {
    fn find_all(&self) -> Vec<Todo>;
    fn save(&self, todo: Todo) -> ();
    fn remove(&self, todo_name: String) -> ();
}

pub struct TodoController {
    pub writer: Box<dyn Write>,
    pub todo_repository: Box<dyn TodoRepository>,
}

impl TodoController {
    pub fn show_all(&mut self) {
        let todos = self.todo_repository.find_all();
        if todos.is_empty() {
            self.writer.write(b"The list is empty\n").unwrap();
        } else {
            for todo in todos {
                self.writer
                    .write(format!("{}\n", todo.name).as_bytes())
                    .unwrap();
            }
        }
    }

    pub fn add(&mut self, todo_name: String) {
        self.todo_repository.save(Todo { name: todo_name });
        self.writer.write(b"OK\n").unwrap();
    }

    pub fn remove(&mut self, todo_name: String) {
        self.todo_repository.remove(todo_name);
        self.writer.write(b"OK\n").unwrap();
    }
}
