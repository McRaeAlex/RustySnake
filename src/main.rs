#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod requests;

#[get("/")]
fn index() -> &'static str {
    "Helo World"
}

fn main() {
    println!("Hello WOrld");
    rocket::ignite().mount("/", routes![index]).launch();
}