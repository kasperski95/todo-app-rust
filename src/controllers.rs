use crate::models::Todo;

pub trait TodoRepository {
    fn find_all(&self) -> Vec<Todo>;
    fn save(&self) -> ();
}

pub struct TodoController<'a> {
    pub todo_repository: &'a dyn TodoRepository,
}

impl TodoController<'_> {
    pub fn show_all(&self) -> Vec<Todo> {
        self.todo_repository.find_all()
    }
}
