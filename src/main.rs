#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate serde_derive;

extern crate rocket;
extern crate rocket_contrib;
extern crate serde;

mod unit;
mod plane;

use rocket_contrib::Json;

fn main() {
    rocket::ignite().mount("/", routes![tel_heading]).launch();
}

// Here we have a sample of a telemetry route. It's not at all complete, of course...
//
// The first problem here is the telemetry struct itself: we need to canonicalize heading into 
// a single kind of measurement, and of course that measurement probably needs to be in degrees 
// or radians as an f32 or f64 or something along those lines--you know, rather than a String.
//
// In addition to transforming this json telemetry packet into usable data, we need to store that 
// data as part of the "airplane's" state.

#[post("/telemetry/heading", data = "<heading>")]
fn tel_heading(heading: Json<unit::HeadingPayload>) -> &'static str {
    "Ok!"
}
