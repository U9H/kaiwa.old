#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_attribute)]
#![feature(custom_derive)]
#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use]
extern crate failure;
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
extern crate percent_encoding;

mod kaiwa;

use kaiwa::controllers as c;
use dotenv::dotenv;
use std::env;
use kaiwa::db;
fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    rocket::ignite()
        .manage(db::init(&database_url))
        .mount("/", routes![
            c::comments::read,
            c::comments::create,
        ])
        .launch();
}

