use reqwest::header::CONTENT_TYPE;
use rstest::rstest;
use reqwest;
use serde_json;
use example::schemas::Todo;

#[rstest]
fn test_bin (){
    let client = reqwest::blocking::Client::new();
    let resp = client.get("http://localhost:8080/health").send().unwrap();

    assert_eq!(resp.status(), 200);

    let resp = client.get("http://localhost:8080/todo").send().unwrap();
    assert_eq!(resp.status(), 200);
    assert_eq!(resp.text().unwrap(), "[]");

    let todo: Todo = Todo{id:60, value:"test value".to_string(), checked:false};
    let resp = client.post("http://localhost:8080/todo").body(serde_json::to_string(&todo).unwrap()).header(CONTENT_TYPE, "application/json").send().unwrap();
    assert_eq!(resp.status(), 201);

    let resp = client.get("http://localhost:8080/todo/search").query(&[("value", "test")]).send().unwrap();
    assert_eq!(resp.status(), 200);
    let res: Vec<Todo> = serde_json::from_slice::<Vec<Todo>>(&resp.bytes().unwrap()).unwrap();

    assert_eq!(res.len(), 1);
    assert_eq!(res.get(0).unwrap(), &todo);

    let resp = client.get("http://localhost:8080/todo/60").send().unwrap();
    assert_eq!(resp.status(), 200);
    assert_eq!(&serde_json::from_slice::<Todo>(&resp.bytes().unwrap()).unwrap(), &todo);

    let resp = client.delete("http://localhost:8080/todo/60").send().unwrap();
    assert_eq!(resp.status(), 200);
}
