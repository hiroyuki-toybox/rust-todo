use std::collections::HashMap;

use anyhow::Result;

trait Todo {
    fn new() -> Self;
    fn add(&mut self, title: String) -> Result<(), TodoError>;
    fn list(&mut self) -> Result<Vec<TodoDetail>, TodoError>;
    fn delete(&mut self, id: i32) -> Result<()>;
    fn done(&mut self, id: i32) -> Result<&TodoDetail, TodoError>;
    fn get(&mut self, id: i32) -> Result<&TodoDetail, TodoError>;
}

#[derive(Debug, Clone)]
struct TodoDetail {
    title: String,
    done: bool,
}

struct TodoRepository {
    map: HashMap<i32, TodoDetail>,
}

#[derive(Debug)]
enum TodoError {
    NotFound,
}

impl Todo for TodoRepository {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    fn add(&mut self, title: String) -> Result<(), TodoError> {
        let id = self.map.len() as i32;
        self.map.insert(id, TodoDetail { title, done: false });

        Ok(())
    }

    fn list(&mut self) -> Result<Vec<TodoDetail>, TodoError> {
        let todos: Vec<TodoDetail> = self.map.values().cloned().collect();

        Ok(todos)
    }

    fn delete(&mut self, id: i32) -> Result<()> {
        self.map.remove(&id);

        Ok(())
    }
    fn done(&mut self, id: i32) -> Result<&TodoDetail, TodoError> {
        let todo = self.map.get_mut(&id).unwrap();
        todo.title = todo.title.clone();
        todo.done = true;

        Ok(todo)
    }

    fn get(&mut self, id: i32) -> Result<&TodoDetail, TodoError> {
        let todo = self.map.get(&id).ok_or(TodoError::NotFound)?;

        Ok(todo)
    }
}

fn main() -> Result<(), TodoError> {
    let mut todo_repo = TodoRepository::new();

    todo_repo.add("やああ".to_string())?;

    let todo = todo_repo.get(0).unwrap();

    println!("{:?}", todo);

    let list = todo_repo.list().unwrap();

    println!("{:?}", list);

    todo_repo.done(0);

    let list = todo_repo.list().unwrap();

    println!("{:?}", list);

    todo_repo.delete(0);

    let list = todo_repo.list().unwrap();

    println!("{:?}", list);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_todo_repo() {
        let todo_repo = TodoRepository::new();

        assert_eq!(todo_repo.map.len(), 0);
    }

    #[test]
    fn add_todo() {
        let mut todo_repo = TodoRepository::new();

        todo_repo.add("test".to_string()).unwrap();

        assert_eq!(todo_repo.map.len(), 1);
        assert_eq!(todo_repo.map.get(&0).unwrap().title, "test".to_string());
    }

    #[test]
    fn get_todo() {
        let mut todo_repo = TodoRepository::new();

        let test_title = "test".to_string();

        todo_repo.add(test_title.clone()).unwrap();

        let todo = todo_repo.get(0).unwrap();

        assert_eq!(todo.title, test_title)
    }

    #[test]
    fn list_todo() {
        let mut todo_repo = TodoRepository::new();

        todo_repo.add("test".to_string());
        todo_repo.add("test".to_string());

        let list = todo_repo.list().unwrap();
        assert_eq!(list.len(), 2);

        assert_eq!(list[0].title, "test".to_string());
        assert_eq!(list[1].title, "test".to_string());
    }
}
