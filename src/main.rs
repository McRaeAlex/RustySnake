#![feature(proc_macro_hygiene, decl_macro)]

// Modules
#[allow(dead_code)]
mod requests;
#[allow(dead_code)]
mod responses;

// External crates
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

// Uses
use rocket_contrib::json::Json;


#[get("/")]
fn index() -> &'static str {
    "Helo World"
}

#[get("/start", format = "json", data="<req>")]
fn start(req: Json<requests::Turn>) -> Json<responses::Start> {
    Json(responses::Start::new(
        "#AA3E39".to_string(),
        responses::HeadType::Pixel,
        responses::TailType::Sharp
        )
    )
}

#[get("/", format = "json", data="<req>")]
fn movement(req: Json<requests::Turn>) -> Json<responses::Move> {
    let mut movement = responses::Move::new(responses::Movement::Right);
    // Logic goes here
    Json(movement)
}

fn main() {
    println!("Hello World");
    rocket::ignite().mount("/", routes![index, start, movement]).launch();
}
