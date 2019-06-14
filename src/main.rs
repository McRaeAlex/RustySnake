#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[allow(dead_code)]
mod requests;
#[allow(dead_code)]
mod response;

#[get("/")]
fn index() -> &'static str {
    "Helo World"
}

#[get("/start", format = "json", data = "<Turn>")]
fn start() -> response::Start {
    
}

fn main() {
    println!("Hello World");
    rocket::ignite().mount("/", routes![index]).launch();
}
