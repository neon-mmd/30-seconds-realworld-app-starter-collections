use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path, Query, ServiceConfig},
    HttpResponse, Responder,
};
use serde::Deserialize;


use crate::TodoStore;
use crate::schemas::{ErrorResponse, TodoUpdateRequest, Todo};


pub(super) fn configure(store: Data<TodoStore>) -> impl FnOnce(&mut ServiceConfig) {
    |config: &mut ServiceConfig| {
        config
            .app_data(store)
            .service(search_todos)
            .service(get_todos)
            .service(create_todo)
            .service(delete_todo)
            .service(get_todo_by_id)
            .service(update_todo);
    }
}


#[get("/health")]
async fn again() -> impl Responder {
    HttpResponse::Ok().body("")
}

/// Get list of todos.
///
/// List todos from todo store.
///
/// One could call the api endpoint with following curl.
/// ```text
/// curl localhost:8080/todo
/// ```
#[get("/todo")]
pub(super) async fn get_todos(todo_store: Data<TodoStore>) -> impl Responder {
    HttpResponse::Ok().json(todo_store.get_repository().read_all().await)
}

/// Create new Todo to storage.
///
/// Post a new `Todo` in request body as json to store it. Api will return
/// created `Todo` on success or `ErrorResponse::Conflict` if todo with same id already exists.
///
/// One could call the api with.
/// ```text
/// curl localhost:8080/todo -d '{"id": 1, "value": "Buy movie ticket", "checked": false}'
/// ```
#[post("/todo")]
pub(super) async fn create_todo(todo: Json<Todo>, todo_store: Data<TodoStore>) -> impl Responder {
    let result = todo_store.get_repository().create_one(&todo.into_inner()).await;
    return match result {
        Ok(todo_ret) => HttpResponse::Ok().json(todo_ret),
        Err(existing) =>  HttpResponse::Conflict().json(ErrorResponse::Conflict(format!("id = {}", existing.id)))
    }
}

/// Delete Todo by given path variable id.
///
/// This endpoint needs `api_key` authentication in order to call. Api key can be found from README.md.
///
/// Api will delete todo from storage by the provided id and return success 200.
/// If storage does not contain `Todo` with given id 404 not found will be returned.
#[delete("/todo/{id}")]
pub(super) async fn delete_todo(id: Path<u64>, todo_store: Data<TodoStore>) -> impl Responder {
    let result = todo_store.get_repository().delete_one(*id).await;
    return match result {
        Ok(()) => HttpResponse::Ok().finish(),
        Err(()) => HttpResponse::NotFound().json(ErrorResponse::NotFound(format!("id = {id}")))
    }
}

/// Get Todo by given todo id.
///
/// Return found `Todo` with status 200 or 404 not found if `Todo` is not found from shared in-memory storage.
#[get("/todo/{id}")]
pub(super) async fn get_todo_by_id(id: Path<u64>, todo_store: Data<TodoStore>) -> impl Responder {
    let result = todo_store.get_repository().read_one(*id).await;
    return match result {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(()) => HttpResponse::NotFound().json(ErrorResponse::NotFound(format!("id = {id}")))
    }

}

/// Update Todo with given id.
///
/// This endpoint supports optional authentication.
///
/// Tries to update `Todo` by given id as path variable. If todo is found by id values are
/// updated according `TodoUpdateRequest` and updated `Todo` is returned with status 200.
/// If todo is not found then 404 not found is returned.
#[put("/todo/{id}")]
pub(super) async fn update_todo(
    id: Path<u64>,
    todo: Json<TodoUpdateRequest>,
    todo_store: Data<TodoStore>,
) -> impl Responder {
    let result = todo_store.get_repository().update_one(*id, todo.into_inner()).await;
    return match result {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(()) => HttpResponse::NotFound().json(ErrorResponse::NotFound(format!("id = {id}")))
    }
}

/// Search todos Query
#[derive(Deserialize, Debug)]
pub(super) struct SearchTodos {
    /// Content that should be found from Todo's value field
    value: String,
}

/// Search Todos with by value
///
/// Perform search from `Todo`s present in in-memory storage by matching Todo's value to
/// value provided as query parameter. Returns 200 and matching `Todo` items.
#[get("/todo/search")]
pub(super) async fn search_todos(
    query: Query<SearchTodos>,
    todo_store: Data<TodoStore>,
) -> impl Responder {
    HttpResponse::Ok().json(todo_store.get_repository().read_filter(&query.value).await)
}