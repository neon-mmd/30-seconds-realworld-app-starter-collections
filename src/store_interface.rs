use crate::schemas::{Todo, TodoUpdateRequest};
use crate::stores::memory::InMemoryTodo;
use async_trait::async_trait;

#[async_trait]
pub trait ITodo {
    async fn read_all(&self) -> Vec<Todo>;
    async fn read_one(&self, id: u64) -> Result<Todo, ()>;
    async fn create_one(&self, t: &Todo) -> Result<(), Todo>;
    async fn update_one(&self, id: u64, t: TodoUpdateRequest) -> Result<Todo, ()>;
    async fn delete_one(&self, id: u64) -> Result<(), ()>;
    async fn read_filter(&self, search_text: &str) -> Vec<Todo>;
}

// Swap this to another type of store as needed
pub struct TodoStore{
    repo: InMemoryTodo
}

impl TodoStore {
    pub fn new() -> Self {
        Self { repo: InMemoryTodo::new(None) }
    }
    pub fn get_repository(&self) -> &dyn ITodo {
        return &self.repo;
    }
}

