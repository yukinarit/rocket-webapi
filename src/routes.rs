use rocket::http::Status;
use rocket::request::Request;
use rocket::response::status::Accepted;
use rocket::response::{Responder, Response};
use rocket_contrib::json::Json;

use crate::models::ToDo;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[derive(Debug, Clone, PartialEq)]
pub struct HogeAccepted<R>(pub R);

/// Sets the status code of the response to 202 Accepted. If the responder is
/// `Some`, it is used to finalize the response.
impl<'r, R: Responder<'r>> Responder<'r> for HogeAccepted<R> {
    fn respond_to(self, req: &Request) -> Result<Response<'r>, Status> {
        let mut build = Response::build();

        build.merge(self.0.respond_to(req)?);

        build.status(Status::Accepted).ok()
    }
}

/// TODOリストを返す
#[get("/todos")]
pub fn todos() -> Accepted<Json<Vec<ToDo>>> {
    Accepted(Some(Json(vec![ToDo {
        id: 1,
        title: "Read Rocket tutorial".into(),
        description: "Read https://rocket.rs/guide/quickstart/".into(),
        done: false,
    }])))
}

/// 新しいTODOを作成する
#[post("/todos", data = "<todo>")]
pub fn new_todo(todo: Json<ToDo>) -> HogeAccepted<Json<ToDo>> {
    HogeAccepted(Json(ToDo {
        id: 1,
        title: "Read Rocket tutorial".into(),
        description: "Read https://rocket.rs/guide/quickstart/".into(),
        done: false,
    }))
}

// TODOを取得する
#[get("/todos/<todoid>")]
pub fn todo_by_id(todoid: u32) -> String {
    let todo = ToDo {
        id: 1,
        title: "Read Rocket tutorial".into(),
        description: "Read https://rocket.rs/guide/quickstart/".into(),
        done: false,
    };
    format!("{:?}", todo)
}
