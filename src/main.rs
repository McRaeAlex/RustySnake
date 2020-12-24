#![feature(proc_macro_hygiene, decl_macro)]

// Modules
#[allow(dead_code)]
mod requests;
#[allow(dead_code)]
mod responses;
#[cfg(test)]
mod test;

// External crates
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

// Uses
use rocket::http::Status;
use rocket_contrib::json::Json;

#[get("/")]
fn index() -> Json<responses::Info> {
    Json(responses::Info {
        apiversion: "1".to_string(),
        author: None,
        color: Some("#b7410e".to_string()),
        head: None,
        tail: None,
        version: Some("0".to_string()),
    })
}

#[post("/start")]
fn start() -> Status {
    Status::Ok
}

#[post("/move", data = "<req>")]
fn movement(req: Json<requests::Turn>) -> Json<responses::Move> {
    let movement = responses::Move::new(responses::Movement::Right);
    // Logic goes here
    Json(movement)
}

#[post("/end")]
fn end() -> Status {
    Status::Ok
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index, start, movement, end])
}

fn main() {
    rocket().launch();
}
