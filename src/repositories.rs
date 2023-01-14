use crate::{controllers::TodoRepository, models::Todo};

pub struct JSONTodoRepository {}
impl TodoRepository for JSONTodoRepository {
    fn find_all(&self) -> Vec<Todo> {
        return Vec::from([Todo {
            name: "foo".to_string(),
        }]);
    }

    fn save(&self) -> () {}
}
