#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate chrono;

mod models;
mod routes;

use routes::*;

fn main() {
    rocket::ignite()
        .mount("/", routes![index, todos, new_todo, todo_by_id])
        .launch();
}
