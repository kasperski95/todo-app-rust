pub struct Todo {
    pub name: String,
}

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

pub struct JSONTodoRepository {}
impl TodoRepository for JSONTodoRepository {
    fn find_all(&self) -> Vec<Todo> {
        return Vec::from([Todo {
            name: "foo".to_string(),
        }]);
    }

    fn save(&self) -> () {}
}
