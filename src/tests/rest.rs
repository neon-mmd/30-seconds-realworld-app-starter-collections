

#[cfg(test)]
mod tests {
    use actix_web::{test::{self, TestRequest}, App, web::Data,  dev::ServiceResponse};
    use coi::{container, Container};
    use crate::{schemas::Todo, stores::memory::{TodoRepository, TestTodoProvider}};
    use crate::rest::configure;

    async fn call_with_request(req: TestRequest, todo_store: Container) -> ServiceResponse {
        let app = test::init_service(App::new().app_data(todo_store).configure(configure())).await;
        test::call_service(&app, req.to_request()).await
    }

    async fn call_and_read_one(req: TestRequest, todo_store: Container) -> Vec<Todo> {
        let app = test::init_service(App::new().app_data(todo_store).configure(configure())).await;
        test::call_and_read_body_json::<_, _, Vec<Todo>>(&app, req.to_request()).await
    }

    #[actix_web::test]
    async fn test_todo_get() {
        let repo = container!{
            repository => TestTodoProvider; singleton
        };
        let req = test::TestRequest::get().uri("/todo");
        let resp = call_with_request(req, repo.clone()).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_todo_post() {
        let repo = container!{
            repository => TestTodoProvider; singleton
        };
        let req = test::TestRequest::post().uri("/todo").set_json(Data::new(Todo{checked: false, value: "some_value".to_owned(), id: 42}));
        let resp = call_with_request(req, repo.clone()).await;
        assert!(resp.status().is_success());
        assert_eq!(repo.resolve::<dyn TodoRepository>("repository").unwrap().read_all().await.len(), 2)
    }

    #[actix_web::test]
    async fn test_todo_search() {
        let repo = container!{
            repository => TestTodoProvider; singleton
        };
        let expected_todo = Todo{id:1, value:"some value".to_owned(), checked:true};
        let todo_expected_list : Vec<Todo> = [expected_todo].to_vec();

        let req = test::TestRequest::get().uri("/todo/search?value=value");
        let resp = call_and_read_one(req, repo.clone()).await;

        assert_eq!(resp, todo_expected_list);
    }
}