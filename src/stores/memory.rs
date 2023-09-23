use std::sync::Mutex;
use std::collections::HashMap;

use crate::schemas::{Todo, TodoUpdateRequest};
use crate::store_interface::ITodo;
use async_trait::async_trait;

pub struct InMemoryTodo {
    todos: Mutex<HashMap<u64, Todo>>
}

impl InMemoryTodo {
    pub fn new() -> Self {
        Self{ todos: Mutex::new(HashMap::new())}
    }
}


#[async_trait]
impl ITodo for InMemoryTodo {

    async fn read_all(&self) -> Vec<Todo> {
        self.todos.lock().unwrap().values().cloned().collect()
    }

    async fn read_one(&self, id: u64) -> Result<Todo, ()> {
        let todos = self.todos.lock().unwrap();
        let one = todos.get(&id);
        if one.is_none(){
            return Err(());
        }
        Ok(one.unwrap().clone())
    }

    async fn create_one(&self, t: &Todo) -> Result<(), Todo> {
        let mut todos = self.todos.lock().unwrap();
        let existing_todo = todos.get(&t.id);
        if existing_todo.is_none(){
            todos.insert(t.id, t.clone());
            return Ok(());
        }
        Err(existing_todo.unwrap().clone())
    }

    async fn update_one(&self, id: u64, todo_update: TodoUpdateRequest) -> Result<Todo, ()> {
        let todos = self.todos.lock().unwrap();
        let existing_todo = todos.get(&id);
        if existing_todo.is_none(){
            return Err(())
        }
        let mut todo = existing_todo.unwrap().to_owned();
        todo.value = todo_update.value.unwrap_or(todo.value);
        todo.checked = todo_update.checked.unwrap_or(todo.checked);
        Ok(todo)
    }

    async fn delete_one(&self, id: u64) -> Result<(), ()> {
        let mut todos = self.todos.lock().unwrap();
        let existing = todos.remove(&id);
        if existing.is_none(){
            return Err(());
        }
        Ok(())
    }

    async fn read_filter(&self, search_text: &str) -> Vec<Todo>  {
        self.todos.lock().unwrap().values().cloned().filter(|todo| {
            todo.value
                .to_lowercase()
                .contains(&search_text.to_lowercase())
        }).collect()
    }
}
