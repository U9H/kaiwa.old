#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_attribute)]

extern crate chrono;
extern crate rocket;
extern crate serde;
#[macro_use]
extern crate serde_derive;

mod kaiwa;

static PORT: &'static str = "3000";

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
