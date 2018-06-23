use rocket_contrib::Json;
use rocket::response::status::Accepted;

use models::ToDo;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

/// TODOリストを返す
#[get("/todos")]
fn todos() -> Accepted<Json<Vec<ToDo>>> {
    Accepted(Some(Json(vec![ToDo {
        id: 1,
        title: "Read Rocket tutorial".into(),
        description: "Read https://rocket.rs/guide/quickstart/".into(),
        done: false,
    }])))
}

/// 新しいTODOを作成する
#[post("/todos", data = "<todo>")]
fn new_todo(todo: Json<ToDo>) -> String {
    format!("Accepted post request! {:?}", todo.0)
}

// TODOを取得する
#[get("/todos/<todoid>")]
fn todo_by_id(todoid: u32) -> String {
    let todo = ToDo {
        id: 1,
        title: "Read Rocket tutorial".into(),
        description: "Read https://rocket.rs/guide/quickstart/".into(),
        done: false,
    };
    format!("{:?}", todo)
}
