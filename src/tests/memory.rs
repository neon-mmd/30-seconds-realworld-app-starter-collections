use rstest::*;

use std::collections::HashMap;

use crate::store_interface::ITodo;
use crate::stores::memory::InMemoryTodo;
use crate::schemas::{Todo, TodoUpdateRequest};


#[fixture]
fn hashmap() -> HashMap<u64, Todo>{
    let mut hm: HashMap<u64, Todo> = HashMap::new();
    hm.insert(1, Todo { id: 1, value: "some_value".to_owned(), checked: false });
    hm.insert(2, Todo { id: 2, value: "some_other".to_owned(), checked: true });
    hm
}

#[fixture]
fn repository(hashmap: HashMap<u64, Todo>) -> InMemoryTodo{
    InMemoryTodo::new(Some(hashmap))
}

#[rstest]
async fn test_read_all(repository: InMemoryTodo, hashmap: HashMap<u64, Todo>){
    let read_vals = repository.read_all().await;
    assert_eq!( read_vals.len(), 2);

    for val in read_vals.into_iter(){
        assert_eq!(hashmap.get(&val.id).unwrap(), &val);
    }
}

#[rstest]
async fn test_read_one(repository: InMemoryTodo, hashmap: HashMap<u64, Todo>){
    let read_val = repository.read_one(1).await;
    assert_eq!( &read_val.unwrap(), hashmap.get(&1).unwrap());
}

#[rstest]
async fn test_read_one_fail(repository: InMemoryTodo){
    let read_val = repository.read_one(42).await;
    assert_eq!( read_val.unwrap_err(), ());
}

#[rstest]
async fn create_one(repository: InMemoryTodo){
    let todo = Todo { id: 3, value: "new_value".to_owned(), checked: true };
    let result = repository.create_one(&todo).await;
    assert!(result.is_ok());
}

#[rstest]
async fn double_create_fail(repository: InMemoryTodo){
    let todo = Todo { id: 3, value: "new_value".to_owned(), checked: true };
    let _ = repository.create_one(&todo).await;
    let result = repository.create_one(&todo).await;
    assert_eq!(result.unwrap_err(), todo);
}

#[rstest]
async fn create_read_all(repository: InMemoryTodo){
    let todo = Todo { id: 3, value: "new_value".to_owned(), checked: true };
    let _ = repository.create_one(&todo).await;
    let read_vals = repository.read_all().await;
    assert_eq!( read_vals.len(), 3);
}

#[rstest]
async fn create_read_one(repository: InMemoryTodo){
    let todo = Todo { id: 3, value: "new_value".to_owned(), checked: true };
    let _ = repository.create_one(&todo).await;
    let read_one = repository.read_one(3).await.unwrap();
    assert_eq!( read_one, todo);
}

#[rstest]
async fn delete_read_one(repository: InMemoryTodo){
    let result_delete = repository.delete_one(1).await;
    let result_get = repository.read_one(1).await;
    assert!(result_delete.is_ok());
    assert!(result_get.is_err());
}