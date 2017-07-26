#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

fn main() {
    rocket::ignite().mount("/", routes![hello]).launch();
}

#[get("/")]
fn hello() -> &'static str {
    "Hi"
}
