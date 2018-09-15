#![feature(try_trait)]
extern crate actix_web;
extern crate kaiwa_lib;
use kaiwa_lib::app::error::Error as AppError;
use kaiwa_lib::error::Error;

use actix_web::{server, App, HttpRequest, Result};
static PORT: &'static str = "3000";

fn index(req: &HttpRequest) -> std::result::Result<String, Error> {
    let site = req.match_info().get("site")?;
    Ok(format!("{}", site))
}

fn main() -> Result<(), AppError> {
    let backend = server::new(|| {
        App::new()
            .resource("/", |r| r.f(index))
            .resource("/{site}", |r| r.f(index))
    }).bind(format!("127.0.0.1:{}", PORT))?;
    backend.run();
    Ok(())
}
