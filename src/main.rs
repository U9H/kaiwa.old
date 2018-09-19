#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_attribute)]

extern crate chrono;
extern crate rocket;
extern crate serde;
extern crate dotenv;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket_contrib;
extern crate r2d2;
#[macro_use]
extern crate serde_derive;

mod kaiwa;

use dotenv::dotenv;
use std::env;
use kaiwa::db::DbConn;
use kaiwa::db;

static PORT: &'static str = "3000";

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    rocket::ignite()
        .manage(db::init(&database_url))
        .mount("/", routes![index]).launch();
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
