#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod requests;
mod response;

#[get("/")]
fn index() -> &'static str {
    "Helo World"
}

fn main() {
    println!("Hello World");
    rocket::ignite().mount("/", routes![index]).launch();
}
