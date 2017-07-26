#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

fn main() {
    println!("Hello, world!");
}

#[get("/")]
fn hello() -> &'static str {
    "Hi"
}
