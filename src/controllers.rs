use crate::models::Todo;
use std::io::Write;
pub trait TodoRepository {
    fn find_all(&self) -> Vec<Todo>;
    fn save(&self) -> ();
}

pub struct TodoController<'a> {
    pub writer: &'a mut dyn Write,
    pub todo_repository: &'a dyn TodoRepository,
}

impl TodoController<'_> {
    pub fn show_all(&mut self) {
        let todos = self.todo_repository.find_all();
        if todos.is_empty() {
            self.writer.write(b"The list is empty\n").unwrap();
        } else {
            for todo in todos {
                self.writer.write(todo.name.as_bytes()).unwrap();
            }
        }
    }
}
